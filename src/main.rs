mod cli;
mod fasta;
mod blast;
mod report;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();

    println!("🧬 Running Rusticate with the following parameters:");
    println!("FASTA file: {}", args.fasta);
    println!("Database: {}", args.db);
    println!("Minimum identity: {}%", args.min_identity);
    println!("Minimum coverage: {}%", args.min_coverage);
    println!("Output directory: {}", args.output);

    // Step 1: Load FASTA
    let contigs = match fasta::read_fasta(&args.fasta) {
        Ok(c) => {
            println!("✅ Loaded {} contigs", c.len());
            c
        }
        Err(e) => {
            eprintln!("❌ Error loading FASTA: {}", e);
            std::process::exit(1);
        }
    };

    // Step 2: Run BLAST
    let hits = match blast::run_blast(&contigs, &args.db) {
        Ok(h) => {
            println!("✅ BLAST returned {} hits", h.len());
            h
        }
        Err(e) => {
            eprintln!("❌ Error running BLAST: {}", e);
            std::process::exit(1);
        }
    };

    // Step 3: Show top hits
    for hit in hits.iter().take(5) {
        println!(
            "{} hit {} (identity: {:.2}%, len: {})",
            hit.query_id, hit.subject_id, hit.identity, hit.aln_len
        );
    }

    // Step 4: Write reports
    if let Err(e) = report::write_reports(&hits, &args.output) {
        eprintln!("❌ Failed to write report: {}", e);
        std::process::exit(1);
    }
}
