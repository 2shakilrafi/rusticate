use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::error::Error;
use std::io::{Write, BufReader, BufRead};
use crate::fasta::Contig;

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
    // 🧬 Create FASTA in memory
    let mut fasta_input = String::new();
    for contig in contigs.values() {
        fasta_input.push_str(&format!(
            ">{}\n{}\n",
            contig.id,
            String::from_utf8_lossy(&contig.seq)
        ));
    }

    // 🚀 Start blastn subprocess
    let mut child = Command::new("blastn")
        .args([
            "-db", db_path,
            "-outfmt", "6",
            "-task", "megablast",
            "-num_threads", "4",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // 🔄 Write to stdin
    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(fasta_input.as_bytes())?;
    }

    // 📥 Read from stdout
    let stdout = child.stdout.take().ok_or("Failed to open stdout")?;
    let reader = BufReader::new(stdout);
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
            query_id,
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

    let status = child.wait()?;
    if !status.success() {
        return Err("❌ blastn exited with non-zero status".into());
    }

    Ok(hits)
}
