/// gen every combination of leet (a-->@) transformations in the word
pub fn stream_leet(word: &str, max_substitutions: Option<usize>) -> impl Iterator<Item = String> {
    LeetPermutations::new(word, max_substitutions)
}
struct LeetPermutations {
    options: Vec<Vec<char>>,
    indices: Vec<usize>,
    finished: bool,
    max_substitutions: Option<usize>,
}

impl LeetPermutations {
    fn new(word: &str, max_substitutions: Option<usize>) -> Self {
        let mut options = Vec::new();
        for ch in word.chars() {
            let mut choices = Vec::new();
            choices.push(ch);
            for &sub in leet_variants_for(ch) {
                if !choices.contains(&sub) {
                    choices.push(sub);
                }
            }
            options.push(choices);
        }
        let indices = vec![0; options.len()];
        Self { options, indices, finished: false, max_substitutions }
    }

    fn advance(&mut self) {
        if self.indices.is_empty() {
            self.finished = true;
            return;
        }

        for pos in (0..self.indices.len()).rev() {
            self.indices[pos] += 1;
            if self.indices[pos] < self.options[pos].len() {
                for reset in pos + 1..self.indices.len() {
                    self.indices[reset] = 0;
                }
                return;
            }
            self.indices[pos] = 0;
        }
        self.finished = true;
    }
}

impl Iterator for LeetPermutations {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished {
            let mut out = String::with_capacity(self.options.len());
            let mut subs = 0usize;
            for (idx, choices) in self.indices.iter().zip(&self.options) {
                let choice_idx = *idx;
                if choice_idx > 0 {
                    subs += 1;
                }
                out.push(choices[choice_idx]);
            }
            let allowed = self
                .max_substitutions
                .map_or(true, |limit| subs <= limit);
            self.advance();
            if allowed {
                return Some(out);
            }
        }
        None
    }
}

fn leet_variants_for(ch: char) -> &'static [char] {
    match ch.to_ascii_lowercase() {
        'a' => &['4', '@'],
        'b' => &['8'],
        'e' => &['3'],
        'g' => &['6', '9'],
        'h' => &['#'],
        'i' => &['1', '!'],
        'l' => &['1'],
        'o' => &['0'],
        'q' => &['9'],
        's' => &['5', '$'],
        't' => &['7', '+'],
        'z' => &['2'],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn includes_expected_leet_variants() {
        let variants: Vec<String> = stream_leet("leet", None).collect();
        let unique: HashSet<&String> = variants.iter().collect();
        assert_eq!(variants.len(), unique.len());
        assert!(unique.iter().any(|v| v.as_str() == "leet"));
        assert!(unique.iter().any(|v| v.as_str() == "l337"));
        assert!(unique.iter().any(|v| v.as_str() == "1337"));
    }

    #[test]
    fn handles_empty_string() {
        let variants: Vec<String> = stream_leet("", None).collect();
        assert_eq!(variants, vec!["".to_string()]);
    }

    #[test]
    fn respects_substitution_limit() {
        let variants: Vec<String> = stream_leet("leet", Some(1)).collect();
        assert!(variants.contains(&"leet".to_string()));
        assert!(!variants.contains(&"l337".to_string()));
    }
}
