use std::collections::HashMap;
use clap::Parser;

mod cli;
mod fasta;
mod blast;
mod db;
mod report;

use cli::{Cli, Command};

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Analyze {
            fasta,
            db,
            min_identity: _,
            min_coverage: _,
            output,
        } => {
            println!("🧬 Running Rusticate with the following parameters:");
            println!("FASTA file: {}", fasta);
            println!("Database: {}", db);
            println!("Output directory: {}", output);

            let contigs = match fasta::read_fasta(&fasta) {
                Ok(c) => {
                    println!("✅ Loaded {} contigs", c.len());
                    c
                },
                Err(e) => {
                    eprintln!("❌ Error reading FASTA: {}", e);
                    std::process::exit(1);
                }
            };

            let contig_map: HashMap<String, fasta::Contig> = contigs
                .into_iter()
                .map(|c| (c.id.clone(), c))
                .collect();

            let db_name = db.split('/')
                .last()
                .unwrap_or("unknown")
                .to_string();

            let hits = match blast::run_blast(&contig_map, &db, &db_name) {
                Ok(h) => {
                    println!("✅ BLAST returned {} hits", h.len());
                    for h in h.iter().take(5) {
                        println!(
                            "{} hit {} (identity: {:.2}%, len: {})",
                            h.query_id, h.subject_id, h.identity, h.aln_len
                        );
                    }
                    h
                },
                Err(e) => {
                    eprintln!("❌ Error running BLAST: {}", e);
                    std::process::exit(1);
                }
            };

            if let Err(e) = report::write_results_tsv(&hits, &output, &fasta) {
                eprintln!("❌ Failed to write report: {}", e);
                std::process::exit(1);
            }

            println!("✅ Reports written to '{}/'\n    ├── results.tsv", output);
        }

        Command::SetupDb { db } => {
            if db == "card" {
                if let Err(e) = db::download_card() {
                    eprintln!("❌ Failed to download CARD: {}", e);
                    std::process::exit(1);
                }
            } else {
                eprintln!("❌ Unknown database: {}", db);
                std::process::exit(1);
            }
        }
    }
}
