use std::collections::HashMap;

fn main() {

    let codon_table = create_codon_table();

    println!("Enter a DNA sequence:");
    let mut dna = String::new();
    std::io::stdin().read_line(&mut dna).unwrap();
    let dna = dna.trim().to_uppercase();

    if !is_valid_dna(&dna) {
        println!("Error: Input contains invalid DNA bases (must be A, T, C, or G).");
        return;
    }

   
    let protein = translate_dna_to_protein(&dna, &codon_table);

    println!("Amino Acid sequence:");
    for aa in protein {
        print!("{} ", aa);
    }
    println!();
}

fn is_valid_dna(sequence: &str) -> bool {
    for c in sequence.chars() {
        let upper_c = c.to_ascii_uppercase(); 
        match upper_c {
            'A' | 'T' | 'C' | 'G' => continue,
            _ => return false,
        }
    }
    true
}

fn translate_dna_to_protein(dna: &str, codon_table: &HashMap<&str, &str>) -> Vec<String> {
    let mut protein = Vec::new();
    let mut i = 0;

    while i + 3 <= dna.len() {
        let codon = &dna[i..i + 3];
        if let Some(&aa) = codon_table.get(codon) {
            protein.push(aa.to_string());
        } else {
            protein.push("UNKNOWN".to_string());
        }
        i += 3;
    }

    protein
}

fn create_codon_table() -> HashMap<&'static str, &'static str> {
    let mut table = HashMap::new();

    // Phenylalanine
    table.insert("TTT", "Phe");
    table.insert("TTC", "Phe");

    // Leucine
    table.insert("TTA", "Leu");
    table.insert("TTG", "Leu");
    table.insert("CTT", "Leu");
    table.insert("CTC", "Leu");
    table.insert("CTA", "Leu");
    table.insert("CTG", "Leu");

    // Isoleucine
    table.insert("ATT", "Ile");
    table.insert("ATC", "Ile");
    table.insert("ATA", "Ile");

    // Methionine (Start)
    table.insert("ATG", "Met");

    // Valine
    table.insert("GTT", "Val");
    table.insert("GTC", "Val");
    table.insert("GTA", "Val");
    table.insert("GTG", "Val");

    // Serine
    table.insert("TCT", "Ser");
    table.insert("TCC", "Ser");
    table.insert("TCA", "Ser");
    table.insert("TCG", "Ser");
    table.insert("AGT", "Ser");
    table.insert("AGC", "Ser");

    // Proline
    table.insert("CCT", "Pro");
    table.insert("CCC", "Pro");
    table.insert("CCA", "Pro");
    table.insert("CCG", "Pro");

    // Threonine
    table.insert("ACT", "Thr");
    table.insert("ACC", "Thr");
    table.insert("ACA", "Thr");
    table.insert("ACG", "Thr");

    // Alanine
    table.insert("GCT", "Ala");
    table.insert("GCC", "Ala");
    table.insert("GCA", "Ala");
    table.insert("GCG", "Ala");

    // Tyrosine
    table.insert("TAT", "Tyr");
    table.insert("TAC", "Tyr");

    // Stop codons
    table.insert("TAA", "STOP");
    table.insert("TAG", "STOP");
    table.insert("TGA", "STOP");

    // Histidine
    table.insert("CAT", "His");
    table.insert("CAC", "His");

    // Glutamine
    table.insert("CAA", "Gln");
    table.insert("CAG", "Gln");

    // Asparagine
    table.insert("AAT", "Asn");
    table.insert("AAC", "Asn");

    // Lysine
    table.insert("AAA", "Lys");
    table.insert("AAG", "Lys");

    // Aspartic Acid
    table.insert("GAT", "Asp");
    table.insert("GAC", "Asp");

    // Glutamic Acid
    table.insert("GAA", "Glu");
    table.insert("GAG", "Glu");

    // Cysteine
    table.insert("TGT", "Cys");
    table.insert("TGC", "Cys");

    // Tryptophan
    table.insert("TGG", "Trp");

    // Arginine
    table.insert("CGT", "Arg");
    table.insert("CGC", "Arg");
    table.insert("CGA", "Arg");
    table.insert("CGG", "Arg");
    table.insert("AGA", "Arg");
    table.insert("AGG", "Arg");

    // Glycine
    table.insert("GGT", "Gly");
    table.insert("GGC", "Gly");
    table.insert("GGA", "Gly");
    table.insert("GGG", "Gly");

    table
}
#[cfg(test)]
mod tests {
    use super::*;

 #[test]
    fn test_is_valid_dna() {
        assert!(is_valid_dna("ATGC"));
        assert!(is_valid_dna("atgc")); 
        assert!(!is_valid_dna("ATBX")); 
    }

    #[test]
    fn test_translation() {
        let codon_table = create_codon_table();
        
 
        assert_eq!(translate_dna_to_protein("ATG", &codon_table), vec!["Met"]);
        assert_eq!(translate_dna_to_protein("TTT", &codon_table), vec!["Phe"]);
        assert_eq!(translate_dna_to_protein("TAA", &codon_table), vec!["STOP"]);

        
        assert_eq!(
            translate_dna_to_protein("ATGTTTAAATAG", &codon_table),
            vec!["Met", "Phe", "Lys", "STOP"]
        );
        
        
        assert_eq!(
            translate_dna_to_protein("ATGC", &codon_table),
            vec!["Met"]
        );
    }
}