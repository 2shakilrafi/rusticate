use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rusticate", version, author, about = "A Rusty replacement for abricate")]
pub struct Cli {
    /// Input FASTA file with contigs
    #[arg(short, long)]
    pub fasta: String,

    /// Name of the database to use (e.g. card, vfdb)
    #[arg(short, long)]
    pub db: String,

    /// Minimum percent identity for a hit to be reported
    #[arg(long, default_value_t = 90.0)]
    pub min_identity: f64,

    /// Minimum percent coverage for a hit to be reported
    #[arg(long, default_value_t = 80.0)]
    pub min_coverage: f64,

    /// Output directory for results
    #[arg(short, long, default_value = "rusticate_out")]
    pub output: String,
}

