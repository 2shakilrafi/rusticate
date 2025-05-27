use std::fs::{create_dir_all, File};
use std::io::{copy, Write};
use std::path::PathBuf;
use std::process::Command;
use std::error::Error;

use reqwest::blocking::get;
use tar::Archive;
use bzip2::read::BzDecoder;

/// Return the path to the DB directory (~/.rusticate/db/<db_name>)
pub fn get_db_path(db_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".rusticate").join("db").join(db_name))
}

/// Download and install the CARD database from the official release archive
pub fn download_card() -> Result<(), Box<dyn Error>> {
    let db_name = "card";
    let url = "https://card.mcmaster.ca/download/0/broadstreet-v4.0.0.tar.bz2";
    let archive_name = "broadstreet-v4.0.0.tar.bz2";
    let expected_filename = "nucleotide_fasta_protein_homolog_model.fasta";

    // Set up paths
    let db_dir = get_db_path(db_name)?;
    create_dir_all(&db_dir)?;

    let tarball_path = db_dir.join(archive_name);
    let fasta_path = db_dir.join("card.fasta");

    // Step 1: Download the archive
    println!("🌐 Downloading CARD database archive...");
    let mut resp = get(url)?;
    let mut out = File::create(&tarball_path)?;
    copy(&mut resp, &mut out)?;
    println!("✅ Downloaded to {:?}", tarball_path);

    // Step 2: Extract and find the FASTA
    println!("📦 Extracting archive...");
    let tar_bz2 = File::open(&tarball_path)?;
    let decompressor = BzDecoder::new(tar_bz2);
    let mut archive = Archive::new(decompressor);

    let mut found = false;
    for entry in archive.entries()? {
        let mut file = entry?;
        let path = file.path()?.to_owned();

        if let Some(fname) = path.file_name() {
            if fname == expected_filename {
                println!("📄 Found FASTA file: {:?}", path);
                let mut out = File::create(&fasta_path)?;
                copy(&mut file, &mut out)?;
                found = true;
                break;
            }
        }
    }

    if !found {
        return Err(format!("❌ Could not find {} in archive", expected_filename).into());
    }

    // Step 3: Run makeblastdb
    println!("🛠️  Indexing FASTA with makeblastdb...");
    let status = Command::new("makeblastdb")
        .args([
            "-in", fasta_path.to_str().unwrap(),
            "-dbtype", "nucl",
            "-out", db_dir.join(db_name).to_str().unwrap(),
        ])
        .status()?;

    if !status.success() {
        return Err("❌ makeblastdb failed".into());
    }

    // Step 4: Write metadata
    let meta_path = db_dir.join("metadata.json");
    let mut meta = File::create(meta_path)?;
    write!(meta, "{{\"source\": \"{}\", \"version\": \"4.0.0\"}}", url)?;

    println!("✅ CARD v4.0.0 is ready at {:?}", db_dir);
    Ok(())
}
