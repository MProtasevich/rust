use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut nucleotide_counts = nucleotide_counts(dna)?;
    nucleotide_counts.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts_per_nucleotide: HashMap<char, usize> = NUCLEOTIDES.into_iter().map(|n| (n, 0)).collect();
    for c in dna.chars() {
        counts_per_nucleotide.get_mut(&c).map(|n| *n += 1).ok_or(c)?;
    }
    Ok(counts_per_nucleotide)
}
