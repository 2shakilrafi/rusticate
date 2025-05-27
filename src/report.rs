use std::fs::{create_dir_all, File};
use std::io::{Write, BufWriter};
use std::path::Path;
use crate::blast::BlastHit;

pub fn write_results_tsv(hits: &[BlastHit], output_dir: &str, fasta_filename: &str) -> std::io::Result<()> {
    create_dir_all(output_dir)?;

    let out_path = Path::new(output_dir).join("results.tsv");
    let mut writer = BufWriter::new(File::create(out_path)?);

    writeln!(
        writer,
        "#FILE\tSEQUENCE\tSTART\tEND\tGENE\t%COVERAGE\tCOVERAGE_MAP\t%IDENTITY\tDB"
    )?;

    for hit in hits {
        let coverage = (hit.aln_len as f64 / hit.q_len as f64) * 100.0;

        // coverage map: + for each aligned base, . for each unaligned (up to 50 chars)
        let mut cov_map = String::new();
        let mut i = 0;
        while i < hit.q_len && cov_map.len() < 50 {
            if i >= hit.q_start.saturating_sub(1) && i < hit.q_end {
                cov_map.push('+');
            } else {
                cov_map.push('.');
            }
            i += hit.q_len / 50 + 1;
        }

        // Clean gene name from subject_id
        let gene = hit.subject_id
            .split('|')
            .last()
            .unwrap_or(&hit.subject_id)
            .replace('_', "-");

        writeln!(
            writer,
            "{}\t{}\t{}\t{}\t{}\t{:.1}\t{}\t{:.1}\t{}",
            fasta_filename,
            hit.query_id,
            hit.q_start,
            hit.q_end,
            gene,
            coverage,
            cov_map,
            hit.identity,
            hit.db_name
        )?;
    }

    Ok(())
}
