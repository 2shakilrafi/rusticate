use crate::blast::BlastHit;
use std::fs::{File, create_dir_all};
use std::io::{Write, BufWriter};
use std::path::Path;
use serde::Serialize;
use tera::{Tera, Context};

#[derive(Serialize)]
struct SerializableHit<'a> {
    query_id: &'a str,
    subject_id: &'a str,
    identity: f64,
    aln_len: usize,
}

pub fn write_reports(hits: &[BlastHit], output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(output_dir)?;

    // Paths
    let tsv_path = Path::new(output_dir).join("results.tsv");
    let json_path = Path::new(output_dir).join("results.json");
    let html_path = Path::new(output_dir).join("report.html");

    // Serialize minimal hit data
    let json_hits: Vec<_> = hits.iter().map(|hit| SerializableHit {
        query_id: &hit.query_id,
        subject_id: &hit.subject_id,
        identity: hit.identity,
        aln_len: hit.aln_len,
    }).collect();

    // Write TSV
    let mut tsv = BufWriter::new(File::create(&tsv_path)?);
    writeln!(tsv, "query_id\tsubject_id\tidentity\taln_len")?;
    for hit in &json_hits {
        writeln!(
            tsv,
            "{}\t{}\t{:.2}\t{}",
            hit.query_id, hit.subject_id, hit.identity, hit.aln_len
        )?;
    }

    // Write JSON
    let json_file = File::create(&json_path)?;
    serde_json::to_writer_pretty(json_file, &json_hits)?;

    // Write HTML via Tera
    let tera = Tera::new("templates/*.html.tera")?;
    let mut context = Context::new();
    context.insert("hits", &json_hits);
    let rendered = tera.render("report.html.tera", &context)?;
    std::fs::write(&html_path, rendered)?;

    println!("✅ Reports written to '{}'", output_dir);
    println!("    ├── results.tsv");
    println!("    ├── results.json");
    println!("    └── report.html");

    Ok(())
}
