use crate::fasta::Contig;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub struct BlastHit {
    pub query_id: String,
    pub subject_id: String,
    pub identity: f64,
    pub aln_len: usize,
    pub mismatches: usize,
    pub gap_opens: usize,
    pub q_start: usize,
    pub q_end: usize,
    pub s_start: usize,
    pub s_end: usize,
    pub evalue: f64,
    pub bit_score: f64,
}

/// Run BLASTN against the provided DB using given contigs.
/// Returns vector of hits.
pub fn run_blast(contigs: &[Contig], db: &str) -> Result<Vec<BlastHit>, Box<dyn std::error::Error>> {
    // Write contigs to temp FASTA file
    let mut temp_fasta = NamedTempFile::new()?;
    for contig in contigs {
        writeln!(temp_fasta, ">{}\n{}", contig.id, std::str::from_utf8(&contig.seq)?)?;
    }

    // Output in tabular format
    let blast_output = Command::new("blastn")
        .args([
            "-query", temp_fasta.path().to_str().unwrap(),
            "-db", db,
            "-outfmt", "6",
        ])
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or("Failed to capture BLAST output")?;

    let reader = BufReader::new(blast_output);
    let mut hits = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 12 {
            continue; // Skip malformed lines
        }

        hits.push(BlastHit {
            query_id: fields[0].into(),
            subject_id: fields[1].into(),
            identity: fields[2].parse()?,
            aln_len: fields[3].parse()?,
            mismatches: fields[4].parse()?,
            gap_opens: fields[5].parse()?,
            q_start: fields[6].parse()?,
            q_end: fields[7].parse()?,
            s_start: fields[8].parse()?,
            s_end: fields[9].parse()?,
            evalue: fields[10].parse()?,
            bit_score: fields[11].parse()?,
        });
    }

    Ok(hits)
}
