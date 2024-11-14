pub fn case_combinations(word: &str) -> Vec<String> {
    let results;

    // generate all variations through recursio
    fn helper(word: &str, current: String) -> Vec<String> {
        if word.is_empty() {
            vec![current]
        } else {
            let first = word.chars().next().unwrap();
            let rest = &word[1..];

            // lowercase and uppercase transformation paths
            let (lowercase_variations, uppercase_variations): (Vec<String>, Vec<String>) = rayon::join(
                || helper(rest, format!("{}{}", current, first.to_lowercase())),
                || helper(rest, format!("{}{}", current, first.to_uppercase())),
            );

            // combine both uppercase and lowercase paths
            let mut all_variations = lowercase_variations;
            all_variations.extend(uppercase_variations);
            all_variations
        }
    }

    // call helper recursion function to go agane
    results = helper(word, String::new());
    results
}
