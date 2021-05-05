#[derive(Debug, PartialEq)]
pub struct Dna {
    sequence: String
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    sequence: String
}

impl Dna {
    const VALID_DNA_NUCLEOTIDE: &'static str = "ACGT";
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (id, c) in dna.chars().enumerate() {
            if !Dna::VALID_DNA_NUCLEOTIDE.contains(c) {
                return Err(id);
            }
        }
        Ok(Dna { sequence: dna.to_string() })
    }

    pub fn into_rna(self) -> Rna {
        let mut rna_sequence: String = String::from("");
        for c in self.sequence.chars() {
            match c {
                'A' => rna_sequence.push_str("U"),
                'C' => rna_sequence.push_str("G"),
                'G' => rna_sequence.push_str("C"),
                'T' => rna_sequence.push_str("A"),
                _ => {}
            }
        }
        Rna { sequence: rna_sequence }
    }
}

impl Rna {
    const VALID_RNA_NUCLEOTIDE: &'static str = "ACGU";
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (id, c) in rna.chars().enumerate() {
            if !Rna::VALID_RNA_NUCLEOTIDE.contains(c) {
                return Err(id);
            }
        }
        Ok(Rna { sequence: rna.to_string() })
    }
}
