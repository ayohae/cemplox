/// gen every combination of case (upper/lower) transformations in the word
pub fn stream_cases(word: &str) -> impl Iterator<Item = String> {
    CasePermutations::new(word)
}

struct CasePermutations {
    chars: Vec<char>,
    state: Vec<u8>,
    finished: bool,
}

impl CasePermutations {
    fn new(word: &str) -> Self {
        let chars: Vec<char> = word.chars().collect();
        let state = vec![0; chars.len()];
        Self {
            chars,
            state,
            finished: false,
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
        if self.finished {
            return None;
        }

        let mut out = String::with_capacity(self.chars.len());
        for (idx, ch) in self.chars.iter().enumerate() {
            let upper = idx < self.state.len() && self.state[idx] == 1;
            if upper {
                out.push(ch.to_ascii_uppercase());
            } else {
                out.push(ch.to_ascii_lowercase());
            }
        }

        self.advance();
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_all_case_permutations() {
        let variants: Vec<String> = stream_cases("ab").collect();
        assert_eq!(variants.len(), 4);
        assert!(variants.contains(&"ab".to_string()));
        assert!(variants.contains(&"Ab".to_string()));
        assert!(variants.contains(&"aB".to_string()));
        assert!(variants.contains(&"AB".to_string()));
    }

    #[test]
    fn handles_empty_input() {
        let variants: Vec<String> = stream_cases("").collect();
        assert_eq!(variants, vec!["".to_string()]);
    }
}
