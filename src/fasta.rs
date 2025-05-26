use bio::io::fasta;
use std::fs::File;
use std::io::BufReader;

/// Represents a contig from the FASTA file
pub struct Contig {
    pub id: String,
    pub seq: Vec<u8>,
}

/// Read a FASTA file and return a vector of Contigs
pub fn read_fasta(path: &str) -> Result<Vec<Contig>, Box<dyn std::error::Error>> {
    let reader = fasta::Reader::new(BufReader::new(File::open(path)?));

    let mut contigs = Vec::new();
    for result in reader.records() {
        let record = result?;
        contigs.push(Contig {
            id: record.id().to_owned(),
            seq: record.seq().to_owned(),
        });
    }

    Ok(contigs)
}
