use std::collections::HashMap;

fn main() {

    let codon_table = create_codon_table();
    
    println!("Enter a DNA sequence:");
    let mut dna = String::new();
    std::io::stdin().read_line(&mut dna).unwrap();
    let dna = dna.trim().to_uppercase();

    if !is_valid_dna(&dna) {
        println!("Error: Invalid DNA sequence (only A, T, C, G allowed)");
        return;
    }

    let (protein, translation) = translate_dna_to_protein(&dna, &codon_table);

    println!("\nDNA → Amino Acid Translation:");
    for (codon, aa) in translation {
        println!("{} → {}", codon, aa);
    }

    println!("\nProtein Sequence:");
    println!("{}", protein.join("-"));
}

fn is_valid_dna(sequence: &str) -> bool {
    sequence.chars().all(|c| matches!(c.to_ascii_uppercase(), 'A' | 'T' | 'C' | 'G'))
}

fn translate_dna_to_protein(dna: &str, codon_table: &HashMap<&str, &str>) -> (Vec<String>, Vec<(String, String)>) {
    let mut protein = Vec::new();
    let mut translation = Vec::new();
    let mut i = 0;

    while i + 3 <= dna.len() {
        let codon = &dna[i..i+3];
        if let Some(&aa) = codon_table.get(codon) {
            protein.push(aa.to_string());
            translation.push((codon.to_string(), aa.to_string()));
        } else {
            protein.push("?".to_string());
            translation.push((codon.to_string(), "?".to_string()));
        }
        i += 3;
    }

    (protein, translation)
}

fn create_codon_table() -> HashMap<&'static str, &'static str> {
    let mut table = HashMap::new();

    table.insert("TTT", "Phe"); table.insert("TTC", "Phe");
    table.insert("TTA", "Leu"); table.insert("TTG", "Leu");
    table.insert("CTT", "Leu"); table.insert("CTC", "Leu");
    table.insert("CTA", "Leu"); table.insert("CTG", "Leu");
    table.insert("ATT", "Ile"); table.insert("ATC", "Ile");
    table.insert("ATA", "Ile"); table.insert("ATG", "Met");
    table.insert("GTT", "Val"); table.insert("GTC", "Val");
    table.insert("GTA", "Val"); table.insert("GTG", "Val");
    table.insert("TCT", "Ser"); table.insert("TCC", "Ser");
    table.insert("TCA", "Ser"); table.insert("TCG", "Ser");
    table.insert("AGT", "Ser"); table.insert("AGC", "Ser");
    table.insert("CCT", "Pro"); table.insert("CCC", "Pro");
    table.insert("CCA", "Pro"); table.insert("CCG", "Pro");
    table.insert("ACT", "Thr"); table.insert("ACC", "Thr");
    table.insert("ACA", "Thr"); table.insert("ACG", "Thr");
    table.insert("GCT", "Ala"); table.insert("GCC", "Ala");
    table.insert("GCA", "Ala"); table.insert("GCG", "Ala");
    table.insert("TAT", "Tyr"); table.insert("TAC", "Tyr");
    table.insert("TAA", "STOP"); table.insert("TAG", "STOP");
    table.insert("TGA", "STOP"); table.insert("CAT", "His");
    table.insert("CAC", "His"); table.insert("CAA", "Gln");
    table.insert("CAG", "Gln"); table.insert("AAT", "Asn");
    table.insert("AAC", "Asn"); table.insert("AAA", "Lys");
    table.insert("AAG", "Lys"); table.insert("GAT", "Asp");
    table.insert("GAC", "Asp"); table.insert("GAA", "Glu");
    table.insert("GAG", "Glu"); table.insert("TGT", "Cys");
    table.insert("TGC", "Cys"); table.insert("TGG", "Trp");
    table.insert("CGT", "Arg"); table.insert("CGC", "Arg");
    table.insert("CGA", "Arg"); table.insert("CGG", "Arg");
    table.insert("AGA", "Arg"); table.insert("AGG", "Arg");
    table.insert("GGT", "Gly"); table.insert("GGC", "Gly");
    table.insert("GGA", "Gly"); table.insert("GGG", "Gly");

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
