/// gen combinations of case (upper/lower) transformations in the word
pub fn stream_cases(word: &str, max_changes: Option<usize>) -> impl Iterator<Item = String> {
    CasePermutations::new(word, max_changes)
}

struct CasePermutations {
    chars: Vec<char>,
    state: Vec<u8>,
    finished: bool,
    max_changes: Option<usize>,
}

impl CasePermutations {
    fn new(word: &str, max_changes: Option<usize>) -> Self {
        let chars: Vec<char> = word.chars().collect();
        let state = vec![0; chars.len()];
        Self {
            chars,
            state,
            finished: false,
            max_changes,
        }
    }

    fn advance(&mut self) {
        if self.state.is_empty() {
            self.finished = true;
            return;
        }

        for idx in (0..self.state.len()).rev() {
            if self.state[idx] == 0 {
                self.state[idx] = 1;
                for reset in idx + 1..self.state.len() {
                    self.state[reset] = 0;
                }
                return;
            }
        }
        self.finished = true;
    }
}

impl Iterator for CasePermutations {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished {
            let mut out = String::with_capacity(self.chars.len());
            let mut changes = 0usize;
            for (idx, ch) in self.chars.iter().enumerate() {
                let upper = self.state.get(idx).copied().unwrap_or(0) == 1;
                let transformed = if upper {
                    ch.to_ascii_uppercase()
                } else {
                    ch.to_ascii_lowercase()
                };
                if transformed != *ch {
                    changes += 1;
                }
                out.push(transformed);
            }
            let allowed = self
                .max_changes
                .map_or(true, |limit| changes <= limit);
            self.advance();
            if allowed {
                return Some(out);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_all_case_permutations() {
        let variants: Vec<String> = stream_cases("ab", None).collect();
        assert_eq!(variants.len(), 4);
        assert!(variants.contains(&"ab".to_string()));
        assert!(variants.contains(&"Ab".to_string()));
        assert!(variants.contains(&"aB".to_string()));
        assert!(variants.contains(&"AB".to_string()));
    }

    #[test]
    fn handles_empty_input() {
        let variants: Vec<String> = stream_cases("", None).collect();
        assert_eq!(variants, vec!["".to_string()]);
    }

    #[test]
    fn respects_case_change_limit() {
        let variants: Vec<String> = stream_cases("abc", Some(1)).collect();
        assert!(variants.contains(&"abc".to_string()));
        assert!(variants.contains(&"Abc".to_string()));
        assert!(variants.contains(&"aBc".to_string()));
        assert!(!variants.contains(&"ABC".to_string()));
    }
}
