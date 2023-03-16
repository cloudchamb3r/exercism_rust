use std::collections::{HashMap, HashSet};

fn create_freq_table(word: &str) -> HashMap<char, i32> {

    let mut ret = HashMap::new();
    for c in word.chars() {       
        *ret.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
    }
    ret
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut hashset = HashSet::new();
    
    let target_freq = create_freq_table(word);    
    for candidate in possible_anagrams {
        if candidate.len() != word.len()|| candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }
        
        let candi_freq = create_freq_table(candidate);
        if candi_freq == target_freq {
            hashset.insert(*candidate);
        }
    }
    hashset
}
