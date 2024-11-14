use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

pub fn character_combinations(
    word: &str,
    chars: &str,
    min_len: usize,
    max_len: usize,
    append: bool,
    prepend: bool,
    insert: bool,
) -> Vec<String> {
    // memo to store computed results. mutex for thread safety.
    let memo: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());

    // vector to hold results
    let mut results = Vec::new();

    // check memo first to avoid duplicate computationos
    {
        let memo = memo.lock().unwrap();
        if let Some(cached) = memo.get(word) {
            return cached.clone();
        }
    }

    // if current word is in range, add to results
    if word.len() >= min_len && word.len() <= max_len {
        results.push(word.to_string());
    }

    // collect characters to a vec
    let chars_vec: Vec<char> = chars.chars().collect();

    // transformations over all characters with par iterator
    let transformed: Vec<Vec<String>> = chars_vec.par_iter()
        .map(|&ch| {
            let mut local_results = Vec::new();

            // iterate over the word and apply transforms, using a queue instead of recur
            // transforms get pushed to the queue to go back through the transform process
            // transforms stop happening if it reaches max length
            let mut queue = vec![word.to_string()]; // start with the og word

            while let Some(current_word) = queue.pop() {
                // apply transformations based on cli flags
                if append {
                    let appended = format!("{}{}", current_word, ch);
                    if appended.len() <= max_len {
                        queue.push(appended.clone());
                        local_results.push(appended);
                    }
                }

                if prepend {
                    let prepended = format!("{}{}", ch, current_word);
                    if prepended.len() <= max_len {
                        queue.push(prepended.clone());
                        local_results.push(prepended);
                    }
                }

                if insert {
                    for i in 0..=current_word.len() {
                        let mut new_word = current_word.clone();
                        new_word.insert(i, ch);
                        if new_word.len() <= max_len {
                            queue.push(new_word.clone());
                            local_results.push(new_word);
                        }
                    }
                }
            }

            local_results
        })
        .collect(); // collect all results

    // flatten all results
    results.extend(transformed.into_iter().flatten());

    // store the results in memo
    {
        let mut memo = memo.lock().unwrap();
        memo.insert(word.to_string(), results.clone());
    }

    results
}
