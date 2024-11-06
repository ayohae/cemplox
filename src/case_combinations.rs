pub fn case_combinations(word: &str) -> Vec<String> {
    let mut results = vec![];

    // recursively generate case variations
    fn helper(word: &str, current: String, results: &mut Vec<String>) {
        if word.is_empty() {
            results.push(current);
        } else {
            let first = word.chars().next().unwrap();
            let rest = &word[1..];

            // lowercase variations
            helper(rest, format!("{}{}", current, first.to_lowercase()), results);

            // uppercase variations
            helper(rest, format!("{}{}", current, first.to_uppercase()), results);
        }
    }

    helper(word, String::new(), &mut results);
    results
}