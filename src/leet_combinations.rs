/// gen every combination of leet (a-->@) transformations in the word
pub fn stream_leet(word: &str) -> impl Iterator<Item = String> {
    LeetPermutations::new(word)
}

struct LeetPermutations {
    options: Vec<Vec<char>>,
    indices: Vec<usize>,
    finished: bool,
}

impl LeetPermutations {
    fn new(word: &str) -> Self {
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
        Self {
            options,
            indices,
            finished: false,
        }
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
        if self.finished {
            return None;
        }

        let mut out = String::with_capacity(self.options.len());
        for (idx, choices) in self.indices.iter().zip(&self.options) {
            out.push(choices[*idx]);
        }

        self.advance();
        Some(out)
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
        let variants: Vec<String> = stream_leet("leet").collect();
        let unique: HashSet<&String> = variants.iter().collect();
        assert_eq!(variants.len(), unique.len());
        assert!(unique.iter().any(|v| v.as_str() == "leet"));
        assert!(unique.iter().any(|v| v.as_str() == "l337"));
        assert!(unique.iter().any(|v| v.as_str() == "1337"));
    }

    #[test]
    fn handles_empty_string() {
        let variants: Vec<String> = stream_leet("").collect();
        assert_eq!(variants, vec!["".to_string()]);
    }
}
