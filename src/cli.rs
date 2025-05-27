use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rusticate", version, author, about = "A Rusty replacement for abricate")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run Rusticate on a FASTA file
    Analyze {
        #[arg(short, long)]
        fasta: String,

        #[arg(short, long)]
        db: String,

        #[arg(long, default_value_t = 90.0)]
        min_identity: f64,

        #[arg(long, default_value_t = 80.0)]
        min_coverage: f64,

        #[arg(short, long, default_value = "rusticate_out")]
        output: String,
    },

    /// Download and set up a database (e.g. card)
    SetupDb {
        #[arg(long)]
        db: String,
    },
}
