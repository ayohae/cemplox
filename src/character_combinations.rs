/// character combinations module
/// length and count modes
/// insert, prepend, append modes


use dashmap::DashMap;
use std::collections::VecDeque;
use rayon::prelude::*;

/// length mode generator
pub fn length_character_combinations(
    word: &str,
    chars: &str,
    min_len: usize,
    max_len: usize,
    append: bool,
    prepend: bool,
    insert: bool,
) -> Vec<String> {
    let memo: DashMap<String, Vec<String>> = DashMap::new();
    if let Some(cached) = memo.get(word) {
        return cached.clone();
    }
    let mut results = Vec::new();
    if word.len() >= min_len && word.len() <= max_len {
        results.push(word.to_string());
    }
    let mut queue = VecDeque::new();
    queue.push_back(word.to_string());
    let chars_vec: Vec<char> = chars.chars().collect();
    while let Some(current_word) = queue.pop_front() {
        let new_combinations: Vec<String> = chars_vec.par_iter()
            .flat_map(|&ch| {
                let mut local_results = Vec::new();
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
        for new_word in new_combinations {
            if !memo.contains_key(&new_word) {
                results.push(new_word.clone());
                queue.push_back(new_word.clone());
                memo.insert(new_word.clone(), vec![new_word.clone()]);
            }
        }
    }
    memo.insert(word.to_string(), results.clone());
    results
}

/// count mode generator
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
