use std::collections::VecDeque;

pub fn stream_count(
    word: &str,
    chars: &str,
    cnt_append: usize,
    cnt_prepend: usize,
    cnt_insert: usize,
) -> impl Iterator<Item = String> {
    CountIterator::new(word, chars, cnt_append, cnt_prepend, cnt_insert)
}

struct CountIterator {
    queue: VecDeque<(String, usize, usize, usize)>,
    chars: Vec<char>,
}

impl CountIterator {
    fn new(
        word: &str,
        chars: &str,
        cnt_append: usize,
        cnt_prepend: usize,
        cnt_insert: usize,
    ) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back((word.to_string(), cnt_append, cnt_prepend, cnt_insert));
        Self {
            queue,
            chars: chars.chars().collect(),
        }
    }
}

impl Iterator for CountIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|(current, a, p, i)| {
            if a > 0 {
                for idx in 0..self.chars.len() {
                    let ch = self.chars[idx];
                    let mut cand = String::with_capacity(current.len() + 1);
                    cand.push_str(&current);
                    cand.push(ch);
                    self.queue.push_back((cand, a - 1, p, i));
                }
            }
            if p > 0 {
                for idx in 0..self.chars.len() {
                    let ch = self.chars[idx];
                    let mut cand = String::with_capacity(current.len() + 1);
                    cand.push(ch);
                    cand.push_str(&current);
                    self.queue.push_back((cand, a, p - 1, i));
                }
            }
            if i > 0 {
                for idx in 0..self.chars.len() {
                    let ch = self.chars[idx];
                    for pos in 0..=current.len() {
                        let mut cand = current.clone();
                        cand.insert(pos, ch);
                        self.queue.push_back((cand, a, p, i - 1));
                    }
                }
            }
            current
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn count_iterator_runs_all_operations() {
        let generated: Vec<String> = stream_count("a", "b", 1, 1, 0).collect();
        let set: HashSet<String> = generated.iter().cloned().collect();
        assert!(set.contains("a"));
        assert!(set.contains("ab"));
        assert!(set.contains("ba"));
    }
}
