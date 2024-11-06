pub fn leet_combinations(word: &str) -> Vec<String> {
    // leetspeak mappings
    let leet_map = [
        ('a', '4'), ('a', '@'), ('b', '8'), ('e', '3'), ('i', '1'), ('o', '0'), ('s', '5'),
        ('t', '7'), ('z', '2'), ('g', '9'), ('g', '6'), ('A', '4'), ('A', '@'), ('B', '8'),
        ('E', '3'), ('I', '1'), ('O', '0'), ('S', '5'), ('T', '7'), ('Z', '2'), ('G', '9'),
        ('G', '6'), ('b', '6'), ('B', '6'), ('r', '9'), ('R', '9'), ('s', '$'), ('S', '$'),
    ];

    // Start with the original word as seed for combinations
    let mut results = vec![word.to_string()];

    // Loop through each character and apply each leet substitution wherever possible
    for (normal, leet) in leet_map.iter() {
        let mut new_results = Vec::new();

        // For each existing word variant in results
        for result in &results {
            // Find all possible positions of the character to replace
            let mut chars: Vec<char> = result.chars().collect();
            for i in 0..chars.len() {
                if chars[i] == *normal {
                    // Create a new variation with this character replaced by its leet equivalent
                    chars[i] = *leet;
                    new_results.push(chars.iter().collect::<String>());
                    chars[i] = *normal; // Reset
                }
            }
        }
        results.extend(new_results);
    }

    results
}