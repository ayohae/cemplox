/// gen every combination of case (upper/lower) transformations in the word
pub fn stream_cases(word: &str) -> impl Iterator<Item = String> {
    // start with a single empty base
    let mut combos = vec![String::new()];
    for ch in word.chars() {
        combos = combos
            .into_iter()
            .flat_map(|base| {
                let mut lower = base.clone();
                lower.push(ch.to_ascii_lowercase());
                let mut upper = base;
                upper.push(ch.to_ascii_uppercase());
                vec![lower, upper]
            })
            .collect();
    }
    combos.into_iter()
}
