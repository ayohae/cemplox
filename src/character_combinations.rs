use std::collections::HashMap;
pub fn generate_char_variants(
    word: &str,
    chars: &str,
    min_len: usize,
    max_len: usize,
    append: bool,
    prepend: bool,
    insert: bool,
) -> Vec<String> {
    // Memoization map to store intermediate results
    let mut memo = HashMap::new();

    // function to handle transforms and memos
    fn transform(
        word: &str,
        chars: &str,
        min_len: usize,
        max_len: usize,
        append: bool,
        prepend: bool,
        insert: bool,
        memo: &mut HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        // return result if in memo
        if let Some(result) = memo.get(word) {
            return result.clone();
        }

        // init result vec
        let mut results = Vec::new();

        // return word if it's in the range
        if word.len() >= min_len && word.len() <= max_len {
            results.push(word.to_string());
        }

        // If the word length is less than max_len, apply transformations
        if word.len() < max_len {
            for ch in chars.chars() {
                // recursion magic, then store the transformed word
                if append {
                    let appended = format!("{}{}", word, ch);
                    results.extend(transform(&appended, chars, min_len, max_len, append, prepend, insert, memo));
                }

                if prepend {
                    let prepended = format!("{}{}", ch, word);
                    results.extend(transform(&prepended, chars, min_len, max_len, append, prepend, insert, memo));
                }

                if insert {
                    for i in 0..=word.len() {
                        let mut new_word = word.to_string();
                        new_word.insert(i, ch);
                        results.extend(transform(&new_word, chars, min_len, max_len, append, prepend, insert, memo));
                    }
                }
            }
        }

        // store this result in memo
        memo.insert(word.to_string(), results.clone());
        results
    }

    // Call the helper function to generate the word variants
    transform(word, chars, min_len, max_len, append, prepend, insert, &mut memo)
}