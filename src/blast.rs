use std::collections::HashMap;
use std::process::Command;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufRead, Write};
use crate::fasta::Contig; // 👈 use shared Contig struct

#[allow(dead_code)]
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
    pub q_len: usize,
    pub db_name: String,
}

pub fn run_blast(
    contigs: &HashMap<String, Contig>,
    db_path: &str,
    db_name: &str,
) -> Result<Vec<BlastHit>, Box<dyn Error>> {
    let tmp_fasta = "rusticate_tmp_input.fasta";
    {
        let mut f = File::create(tmp_fasta)?;
        for contig in contigs.values() {
            writeln!(f, ">{}\n{}", contig.id, String::from_utf8_lossy(&contig.seq))?;

        }
    }

    let tmp_out = "rusticate_tmp_blast.out";
    let status = Command::new("blastn")
        .args([
            "-query", tmp_fasta,
            "-db", db_path,
            "-outfmt", "6",
            "-out", tmp_out,
        ])
        .status()?;

    if !status.success() {
        return Err("❌ blastn failed".into());
    }

    let file = File::open(tmp_out)?;
    let reader = BufReader::new(file);
    let mut hits = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 12 {
            continue;
        }

        let query_id = fields[0].to_string();
        let contig = match contigs.get(&query_id) {
            Some(c) => c,
            None => continue,
        };

        hits.push(BlastHit {
            query_id: query_id.clone(),
            subject_id: fields[1].to_string(),
            identity: fields[2].parse::<f64>()?,
            aln_len: fields[3].parse::<usize>()?,
            mismatches: fields[4].parse::<usize>()?,
            gap_opens: fields[5].parse::<usize>()?,
            q_start: fields[6].parse::<usize>()?,
            q_end: fields[7].parse::<usize>()?,
            s_start: fields[8].parse::<usize>()?,
            s_end: fields[9].parse::<usize>()?,
            evalue: fields[10].parse::<f64>()?,
            bit_score: fields[11].parse::<f64>()?,
            q_len: contig.seq.len(),
            db_name: db_name.to_string(),
        });
    }

    fs::remove_file(tmp_fasta).ok();
    fs::remove_file(tmp_out).ok();

    Ok(hits)
}
