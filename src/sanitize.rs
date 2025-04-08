use rayon::prelude::*;

pub fn sanitize_wordlist(wordlist: &str) -> Vec<String> {
    wordlist
        .par_iter()
        .flat_map(|word| {
            let mut sanitized_word = String::new();
            let mut in_parentheses = false;

            for c in word.chars() {
                if c == '(' {
                    in_parentheses = true;
                } else if c == ')' {
                    in_parentheses = false;
                } else if !in_parentheses {
                    sanitized_word.push(c.to_ascii_lowercase());
                }
            }

            let mut results = vec![sanitized_word.clone()]; // Start with the base sanitized word

            // Check for special characters and generate duplicates
            let special_chars: Vec<char> = "!@#$%^&*()-_=+[]{}|;:'\",.<>/?`~\\".chars().collect();
            if sanitized_word.chars().any(|c| special_chars.contains(&c)) {
                let mut without_special = String::new();
                for c in sanitized_word.chars() {
                    if !special_chars.contains(&c) {
                        without_special.push(c);
                    }
                }
                results.push(without_special);
            }

            results // Return the vector of sanitized words
        })
        .collect()
}