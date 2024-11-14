use rayon::prelude::*;
use dashmap::DashMap;
use std::collections::VecDeque;

pub fn character_combinations(
    word: &str,
    chars: &str,
    min_len: usize,
    max_len: usize,
    append: bool,
    prepend: bool,
    insert: bool,
) -> Vec<String> {
    // memo to store computed results, prevent duplication
    let memo: DashMap<String, Vec<String>> = DashMap::new();

    // results vector init
    let mut results = Vec::new();

    // Check memo first to avoid duplicate computations
    if let Some(cached) = memo.get(word) {
        return cached.clone(); // Return cached value if it exists
    }

    // if word in range, add to results
    if word.len() >= min_len && word.len() <= max_len {
        results.push(word.to_string());
    }

    // collect all characters in the word
    let chars_vec: Vec<char> = chars.chars().collect();

    // transform all chars, par iterator
    let transformed: Vec<Vec<String>> = chars_vec.par_iter()
        .map(|&ch| {
            let mut local_results = Vec::new();

            // create de queue for intermediate results
            let mut queue = VecDeque::new();
            queue.push_back(word.to_string()); // Start with the original word

            while let Some(current_word) = queue.pop_front() {
                // apply transformations based on CLI flags
                if append {
                    let appended = format!("{}{}", current_word, ch);
                    if appended.len() <= max_len {
                        queue.push_back(appended.clone()); // push to back of the deque
                        local_results.push(appended); // push to results
                    }
                }

                if prepend {
                    let prepended = format!("{}{}", ch, current_word);
                    if prepended.len() <= max_len {
                        queue.push_back(prepended.clone()); // push to back of the deque
                        local_results.push(prepended); // push to results
                    }
                }

                if insert {
                    for i in 0..=current_word.len() {
                        let mut new_word = current_word.clone();
                        new_word.insert(i, ch);
                        if new_word.len() <= max_len {
                            queue.push_back(new_word.clone()); // push to back of the deque
                            local_results.push(new_word); // push to results
                        }
                    }
                }
            }

            local_results
        })
        .collect(); // collect all results

    // flatten all results
    results.extend(transformed.into_iter().flatten());

    // store the intermediate results in memo
    memo.insert(word.to_string(), results.clone());

    results
}
