use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

pub fn generate_char_variants(
    word: &str,
    chars: &str,
    min_len: usize,
    max_len: usize,
    append: bool,
    prepend: bool,
    insert: bool,
) -> Vec<String> {
    // memo for storing results, using mutex because safety first!
    let memo = Mutex::new(HashMap::new());

    // function to handle transforms and storing in memo, using CLI args
    fn transform(
        word: &str,
        chars: &str,
        min_len: usize,
        max_len: usize,
        append: bool,
        prepend: bool,
        insert: bool,
        memo: &Mutex<HashMap<String, Vec<String>>>,
    ) -> Vec<String> {
        let mut results = Vec::new();

        // return if in memo
        {
            let memo = memo.lock().unwrap();
            if let Some(result) = memo.get(word) {
                return result.clone();
            }
        }

        // return if inside the specified character length range
        if word.len() >= min_len && word.len() <= max_len {
            results.push(word.to_string());
        }

        // if under max character length, do trasnform
        if word.len() < max_len {
            let chars_vec: Vec<char> = chars.chars().collect(); // collect characters

            // parallel transformations over characters vec
            let transformed: Vec<Vec<String>> = chars_vec.par_iter()
                .map(|&ch| {
                    let mut local_results = Vec::new();

                    // apply transformations (specified by user-supplied args)
                    if append {
                        let appended = format!("{}{}", word, ch);
                        local_results.extend(transform(&appended, chars, min_len, max_len, append, prepend, insert, memo));
                    }

                    if prepend {
                        let prepended = format!("{}{}", ch, word);
                        local_results.extend(transform(&prepended, chars, min_len, max_len, append, prepend, insert, memo));
                    }

                    if insert {
                        for i in 0..=word.len() {
                            let mut new_word = word.to_string();
                            new_word.insert(i, ch);
                            local_results.extend(transform(&new_word, chars, min_len, max_len, append, prepend, insert, memo));
                        }
                    }

                    local_results
                })
                .collect(); // collect all results

            // flatten all results
            results.extend(transformed.into_iter().flatten());
        }

        // store in memo
        {
            let mut memo = memo.lock().unwrap();
            memo.insert(word.to_string(), results.clone());
        }

        results
    }

    // go agane
    transform(word, chars, min_len, max_len, append, prepend, insert, &memo)
}
