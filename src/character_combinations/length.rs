use std::collections::{HashSet, VecDeque};

pub fn stream_length(
    word: &str,
    chars: &str,
    min: usize,
    max: usize,
    do_append: bool,
    do_prepend: bool,
    do_insert: bool,
    dedup: bool,
) -> impl Iterator<Item = String> {
    LengthIterator::new(word, chars, min, max, do_append, do_prepend, do_insert, dedup)
}

struct LengthIterator {
    queue: VecDeque<String>,
    seen: Option<HashSet<String>>,
    chars: Vec<char>,
    min: usize,
    max: usize,
    do_append: bool,
    do_prepend: bool,
    do_insert: bool,
}

impl LengthIterator {
    fn new(
        word: &str,
        chars: &str,
        min: usize,
        max: usize,
        do_append: bool,
        do_prepend: bool,
        do_insert: bool,
        dedup: bool,
    ) -> Self {
        let mut queue = VecDeque::new();
        let seen = if dedup {
            let mut set = HashSet::new();
            if word.len() <= max {
                set.insert(word.to_string());
                queue.push_back(word.to_string());
            }
            Some(set)
        } else {
            if word.len() <= max {
                queue.push_back(word.to_string());
            }
            None
        };
        Self {
            queue,
            seen,
            chars: chars.chars().collect(),
            min,
            max,
            do_append,
            do_prepend,
            do_insert,
        }
    }

    fn push_candidate(&mut self, candidate: String) {
        if candidate.len() > self.max {
            return;
        }
        if let Some(seen) = self.seen.as_mut() {
            if !seen.insert(candidate.clone()) {
                return;
            }
        }
        self.queue.push_back(candidate);
    }

    fn expand_current(&mut self, current: &str) {
        for idx in 0..self.chars.len() {
            let ch = self.chars[idx];
            if self.do_append {
                let mut cand = String::with_capacity(current.len() + 1);
                cand.push_str(current);
                cand.push(ch);
                self.push_candidate(cand);
            }
            if self.do_prepend {
                let mut cand = String::with_capacity(current.len() + 1);
                cand.push(ch);
                cand.push_str(current);
                self.push_candidate(cand);
            }
            if self.do_insert {
                for pos in 0..=current.len() {
                    let mut cand = current.to_string();
                    cand.insert(pos, ch);
                    self.push_candidate(cand);
                }
            }
        }
    }
}

impl Iterator for LengthIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current) = self.queue.pop_front() {
            self.expand_current(&current);
            if current.len() >= self.min && current.len() <= self.max {
                return Some(current);
            }
        }
        None
    }
}
