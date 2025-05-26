use std::fs::{self, create_dir_all, File};
use std::io::{Write, copy};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::error::Error;

use reqwest::blocking::get;

/// Return the path to the DB directory (~/.rusticate/db/<db_name>)
pub fn get_db_path(db_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let home = dirs::home_dir().ok_or("Could not determine home directory")?;
    Ok(home.join(".rusticate").join("db").join(db_name))
}

/// Ensure the database has been indexed using `makeblastdb`
pub fn ensure_blast_index(db_name: &str) -> Result<(), Box<dyn Error>> {
    let db_dir = get_db_path(db_name)?;
    let fasta_path = db_dir.join(format!("{}.fasta", db_name));
    let nin_file = db_dir.join(format!("{}.nin", db_name)); // One of the output files from makeblastdb

    if nin_file.exists() {
        println!("📦 Database '{}' is already indexed.", db_name);
        return Ok(()); // Already indexed
    }

    println!("🛠️  Indexing database '{}' with makeblastdb...", db_name);
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

    println!("✅ Indexed database '{}'", db_name);
    Ok(())
}

/// Download and install the CARD database
pub fn download_card() -> Result<(), Box<dyn Error>> {
    let db_name = "card";
    let url = "https://raw.githubusercontent.com/arpcard/card-data/master/data/nucleotide_fasta_protein_homolog_model.fasta";

    let db_path = get_db_path(db_name)?;
    create_dir_all(&db_path)?;

    let fasta_path = db_path.join("card.fasta");

    println!("🌐 Downloading CARD database from {}", url);
    let mut resp = get(url)?;
    let mut out = File::create(&fasta_path)?;
    copy(&mut resp, &mut out)?;

    println!("✅ Downloaded to {:?}", fasta_path);

    ensure_blast_index(db_name)?;

    // Write simple metadata
    let meta_path = db_path.join("metadata.json");
    let mut meta = File::create(meta_path)?;
    write!(meta, "{{\"source\": \"{}\"}}\n", url)?;

    println!("✅ CARD database is ready at {:?}", db_path);
    Ok(())
}
