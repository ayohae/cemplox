/// leet transforms module
use rayon::prelude::*;
/// returns all potential case combinations for input
pub fn leet_combinations(word: &str) -> Vec<String> {
    let leet_map = [
        ('a', '4'), ('a', '@'), ('b', '8'), ('e', '3'), ('i', '1'), ('o', '0'),
        ('s', '5'), ('t', '7'), ('z', '2'), ('g', '9'), ('g', '6'),
        ('A', '4'), ('A', '@'), ('B', '8'), ('E', '3'), ('I', '1'), ('O', '0'),
        ('S', '5'), ('T', '7'), ('Z', '2'), ('G', '9'), ('G', '6'),
        ('b', '6'), ('B', '6'), ('r', '9'), ('R', '9'),
        ('s', '$'), ('S', '$'),
    ];

    let mut results = vec![word.to_string()];
    for (normal, leet) in leet_map.iter() {
        let new_results: Vec<String> = results.par_iter().flat_map(|result| {
            let mut new_variations = Vec::new();
            let mut chars: Vec<char> = result.chars().collect();
            for i in 0..chars.len() {
                if chars[i] == *normal {
                    chars[i] = *leet;
                    new_variations.push(chars.iter().collect::<String>());
                    chars[i] = *normal;
                }
            }
            new_variations
        }).collect();
        results.extend(new_results);
    }
    results
}