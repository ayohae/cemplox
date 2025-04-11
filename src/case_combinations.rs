/// case transforms module
use rayon::prelude::*;
/// returns all potential case combinations for input
pub fn case_combinations(word: &str) -> Vec<String> {
    let mut results = vec![String::new()];
    for ch in word.chars() {
        let new_combinations: Vec<String> = results.par_iter()
            .flat_map(|base| {
                let lower = format!("{}{}", base, ch.to_lowercase());
                let upper = format!("{}{}", base, ch.to_uppercase());
                vec![lower, upper]
            })
            .collect();
        results = new_combinations;
    }
    results
}