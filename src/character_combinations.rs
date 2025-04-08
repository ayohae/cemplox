use rayon::prelude::*;
use dashmap::DashMap;
use std::collections::VecDeque;

pub fn length_character_combinations(
    word: &str,
    chars: &str,
    min_len: usize,
    max_len: usize,
    append: bool,
    prepend: bool,
    insert: bool,
) -> Vec<String> {
    // memo to store computed results and prevent duplication
    let memo: DashMap<String, Vec<String>> = DashMap::new();

    // check memo first to avoid duplicate computations
    if let Some(cached) = memo.get(word) {
        return cached.clone(); // Return cached value if it exists
    }

    // init results vector
    let mut results = Vec::new();

    // start with the original word if it meets length requirements
    if word.len() >= min_len && word.len() <= max_len {
        results.push(word.to_string());
    }

    // create a queue for iterative processing
    let mut queue = VecDeque::new();
    queue.push_back(word.to_string());

    // collect all characters in a vec to be transformed
    let chars_vec: Vec<char> = chars.chars().collect();

    // process all transformations
    while let Some(current_word) = queue.pop_front() {
        // collect new combinations in par iterator
        let new_combinations: Vec<String> = chars_vec.par_iter()
            .flat_map(|&ch| {
                let mut local_results = Vec::new();

                // apply transformations based on cli flags
                if append {
                    let appended = format!("{}{}", current_word, ch);
                    if appended.len() <= max_len && !memo.contains_key(&appended) {
                        if appended.len() >= min_len {
                            local_results.push(appended.clone());
                        }
                        local_results.push(appended);
                    }
                }

                if prepend {
                    let prepended = format!("{}{}", ch, current_word);
                    if prepended.len() <= max_len && !memo.contains_key(&prepended) {
                        if prepended.len() >= min_len {
                            local_results.push(prepended.clone());
                        }
                        local_results.push(prepended);
                    }
                }

                if insert {
                    for i in 0..=current_word.len() {
                        let mut new_word = current_word.clone();
                        new_word.insert(i, ch);
                        if new_word.len() <= max_len && !memo.contains_key(&new_word) {
                            if new_word.len() >= min_len {
                                local_results.push(new_word.clone());
                            }
                            local_results.push(new_word);
                        }
                    }
                }
                local_results
            })
            .collect();

        // add the new combinations to the queue and results vecs
        for new_word in new_combinations {
            if !memo.contains_key(&new_word) {
                results.push(new_word.clone());
                queue.push_back(new_word.clone());
                memo.insert(new_word.clone(), vec![new_word.clone()]); // Mark as processed
            }
        }
    }

    // store results in memo
    memo.insert(word.to_string(), results.clone());

    results
}

pub fn count_character_combinations(
    word: &str,
    chars: &str,
    append: usize,
    prepend: usize,
    insert: usize,
) -> Vec<String> {
    let mut results = Vec::new();
    let chars_vec: Vec<char> = chars.chars().collect();
    let mut queue = VecDeque::new();
    queue.push_back((word.to_string(), append, prepend, insert));

    while let Some((current_word, current_append, current_prepend, current_insert)) = queue.pop_front() {
        results.push(current_word.clone());

        if current_append > 0 {
            for &ch in &chars_vec {
                let appended = format!("{}{}", current_word, ch);
                queue.push_back((appended, current_append - 1, current_prepend, current_insert));
            }
        }

        if current_prepend > 0 {
            for &ch in &chars_vec {
                let prepended = format!("{}{}", ch, current_word);
                queue.push_back((prepended, current_append, current_prepend - 1, current_insert));
            }
        }

        if current_insert > 0 {
            for &ch in &chars_vec {
                for i in 0..=current_word.len() {
                    let mut inserted = current_word.clone();
                    inserted.insert(i, ch);
                    queue.push_back((inserted, current_append, current_prepend, current_insert - 1));
                }
            }
        }
    }

    results
}
