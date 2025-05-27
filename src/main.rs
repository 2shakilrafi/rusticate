mod cli;
mod fasta;
mod blast;
mod report;
mod db;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::SetupDb { db } => {
            match db.as_str() {
                "card" => {
                    if let Err(e) = db::download_card() {
                        eprintln!("❌ Failed to download CARD: {}", e);
                        std::process::exit(1);
                    }
                }
                _ => {
                    eprintln!("❌ Unsupported database: {}", db);
                    std::process::exit(1);
                }
            }
        }

        Command::Analyze {
            fasta,
            db,
            min_identity,
            min_coverage,
            output,
        } => {
            println!("🧬 Running Rusticate with the following parameters:");
            println!("FASTA file: {}", fasta);
            println!("Database: {}", db);
            println!("Minimum identity: {}%", min_identity);
            println!("Minimum coverage: {}%", min_coverage);
            println!("Output directory: {}", output);

            let contigs = match fasta::read_fasta(&fasta) {
                Ok(c) => {
                    println!("✅ Loaded {} contigs", c.len());
                    c
                }
                Err(e) => {
                    eprintln!("❌ Error loading FASTA: {}", e);
                    std::process::exit(1);
                }
            };

            let hits = match blast::run_blast(&contigs, &db) {
                Ok(h) => {
                    println!("✅ BLAST returned {} hits", h.len());
                    h
                }
                Err(e) => {
                    eprintln!("❌ Error running BLAST: {}", e);
                    std::process::exit(1);
                }
            };

            report::write_results_tsv(&hits, &output, &fasta).unwrap();
            println!("✅ Reports written to '{}/'\n    ├── results.tsv", output);


            for hit in hits.iter().take(5) {
                println!(
                    "{} hit {} (identity: {:.2}%, len: {})",
                    hit.query_id, hit.subject_id, hit.identity, hit.aln_len
                );
            }

        }
    }
}
