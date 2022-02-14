#[derive(Debug, PartialEq)]
enum DnaNucleotides {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

#[derive(Debug, PartialEq)]
pub struct Dna {
    seq: Vec<DnaNucleotides>,
}

#[derive(Debug, PartialEq)]
enum RnaNucleotides {
    Adenine,
    Cytosine,
    Guanine,
    Uracil,
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    seq: Vec<RnaNucleotides>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotide_seq = Vec::with_capacity(dna.len());

        for (idx, chr) in dna.chars().enumerate() {
            let nucleotide = match chr {
                'A' => DnaNucleotides::Adenine,
                'C' => DnaNucleotides::Cytosine,
                'G' => DnaNucleotides::Guanine,
                'T' => DnaNucleotides::Thymine,
                _ => return Err(idx)
            };
            nucleotide_seq.push(nucleotide);
        }
        Ok(Dna { seq: nucleotide_seq })
    }

    pub fn into_rna(self) -> Rna {
        let seq = self.seq.iter().map(|dna_nucleotide| match dna_nucleotide {
            DnaNucleotides::Adenine => RnaNucleotides::Uracil,
            DnaNucleotides::Cytosine => RnaNucleotides::Guanine,
            DnaNucleotides::Guanine => RnaNucleotides::Cytosine,
            DnaNucleotides::Thymine => RnaNucleotides::Adenine,
        }).collect();
        Rna { seq }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotide_seq = Vec::with_capacity(rna.len());

        for (idx, chr) in rna.chars().enumerate() {
            let nucleotide = match chr {
                'A' => RnaNucleotides::Adenine,
                'C' => RnaNucleotides::Cytosine,
                'G' => RnaNucleotides::Guanine,
                'U' => RnaNucleotides::Uracil,
                _ => return Err(idx)
            };
            nucleotide_seq.push(nucleotide);
        }
        Ok(Rna { seq: nucleotide_seq })
    }
}
