use rayon::prelude::*;

pub fn case_combinations(word: &str) -> Vec<String> {
    let mut results = vec![String::new()]; // begin with empty string

    for ch in word.chars() {
        // collect transformations per character
        let new_combinations: Vec<String> = results.par_iter()
            .flat_map(|base| {
                let lower = format!("{}{}", base, ch.to_lowercase());
                let upper = format!("{}{}", base, ch.to_uppercase());
                vec![lower, upper] // collect lowercase and uppercase transforms together
            })
            .collect();

        results = new_combinations; // replace old results with the new transformed combinations
    }

    results
}
