pub fn sanitize_word(word: &str) -> Vec<String> {
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

    let special_chars: Vec<char> = "•!@#$%^&*()-_=+[]{}|;:'\",.<>/?`~\\ ".chars().collect();
    let mut stripped_sanitized = String::new();
    for c in sanitized_word.chars() {
        if !special_chars.contains(&c) {
            stripped_sanitized.push(c);
        }
    }
    let stripped_sanitized = stripped_sanitized.trim().to_string();
    let sanitized_word = sanitized_word.trim().to_string();

    let mut results = vec![stripped_sanitized.clone()];

    if stripped_sanitized != sanitized_word {
        results.push(sanitized_word);
    }
    results
}