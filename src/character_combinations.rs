use std::collections::{HashSet, VecDeque};

/// add characters at various places in the word. charset modified by -C

/// length-based. add characters up to a max length
pub fn stream_length(
    word: &str,
    chars: &str,
    min: usize,
    max: usize,
    do_append: bool,
    do_prepend: bool,
    do_insert: bool,
) -> impl Iterator<Item = String> {
    // Track seen to avoid duplicates
    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    // Initialize
    if word.len() <= max {
        seen.insert(word.to_string());
        queue.push_back(word.to_string());
    }
    let char_vec: Vec<char> = chars.chars().collect();
    let mut results = Vec::new();

    while let Some(cur) = queue.pop_front() {
        if cur.len() >= min && cur.len() <= max {
            results.push(cur.clone());
        }
        for &ch in &char_vec {
            if do_append {
                let cand = format!("{}{}", cur, ch);
                if cand.len() <= max && seen.insert(cand.clone()) {
                    queue.push_back(cand);
                }
            }
            if do_prepend {
                let cand = format!("{}{}", ch, cur);
                if cand.len() <= max && seen.insert(cand.clone()) {
                    queue.push_back(cand);
                }
            }
            if do_insert {
                for i in 0..=cur.len() {
                    let mut cand = cur.clone();
                    cand.insert(i, ch);
                    if cand.len() <= max && seen.insert(cand.clone()) {
                        queue.push_back(cand);
                    }
                }
            }
        }
    }
    results.into_iter()
}

/// count based. add a count of append, prepend, or inserts of characters
pub fn stream_count(
    word: &str,
    chars: &str,
    cnt_append: usize,
    cnt_prepend: usize,
    cnt_insert: usize,
) -> impl Iterator<Item = String> {
    let mut results = Vec::new();
    let mut queue: VecDeque<(String, usize, usize, usize)> = VecDeque::new();
    queue.push_back((word.to_string(), cnt_append, cnt_prepend, cnt_insert));
    let char_vec: Vec<char> = chars.chars().collect();

    while let Some((cur, a, p, i)) = queue.pop_front() {
        results.push(cur.clone());
        if a > 0 {
            for &ch in &char_vec {
                queue.push_back((format!("{}{}", cur, ch), a - 1, p, i));
            }
        }
        if p > 0 {
            for &ch in &char_vec {
                queue.push_back((format!("{}{}", ch, cur), a, p - 1, i));
            }
        }
        if i > 0 {
            for &ch in &char_vec {
                for pos in 0..=cur.len() {
                    let mut cand = cur.clone();
                    cand.insert(pos, ch);
                    queue.push_back((cand, a, p, i - 1));
                }
            }
        }
    }
    results.into_iter()
}
