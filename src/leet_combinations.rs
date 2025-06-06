/// gen every combination of leet (a-->@) transformations in the word
pub fn stream_leet(word: &str) -> impl Iterator<Item = String> {
    let mappings = [
        ('a','4'), ('e','3'), ('i','1'), ('o','0'),
        ('s','5'), ('t','7'),
    ];
    // start with the original
    let mut results = vec![word.to_string()];
    for &(plain, leet) in &mappings {
        results = results
            .into_iter()
            .flat_map(|w| {
                let mut out = Vec::new();
                let chars: Vec<char> = w.chars().collect();
                for i in 0..chars.len() {
                    if chars[i].eq_ignore_ascii_case(&plain) {
                        let mut replaced = chars.clone();
                        replaced[i] = leet;
                        out.push(replaced.into_iter().collect());
                    }
                }
                // always keep the unmodified word too
                out.into_iter().chain(std::iter::once(w))
            })
            .collect();
    }
    results.into_iter()
}
