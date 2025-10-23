pub fn stream(word: &str) -> impl Iterator<Item = String> {
    let base = collapse_spaces(&normalize_delimiters(&strip_bracketed(word)));
    if base.is_empty() {
        return Vec::new().into_iter();
    }
    let final_text = collapse_spaces(&trim_metadata_suffix(&base.to_ascii_lowercase()));
    let orig = final_text.trim().to_string();
    let clean: String = orig.chars().filter(|c| c.is_ascii_alphanumeric()).collect();

    let mut out = Vec::new();
    if meets_length(&clean) {
        out.push(clean);
    }
    if orig != out.last().map_or("", String::as_str) && meets_length(&orig) {
        out.push(orig);
    }
    out.into_iter()
}

fn meets_length(value: &str) -> bool {
    let len = value.len();
    len >= 2 && len <= 28
}

fn strip_bracketed(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut depth = 0usize;
    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' => {
                depth += 1;
                continue;
            }
            ')' | ']' | '}' if depth > 0 => {
                depth -= 1;
                continue;
            }
            _ => {}
        }
        if depth == 0 {
            out.push(ch);
        }
    }
    out
}

fn normalize_delimiters(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '\u{00A0}' | '–' | '—' | '―' | '•' | '·' | ':' | ';' | '|' => ' ',
            _ if c.is_control() => ' ',
            _ => c,
        })
        .collect()
}

fn collapse_spaces(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn trim_metadata_suffix(text: &str) -> String {
    let mut tokens: Vec<&str> = text.split_whitespace().collect();
    while tokens
        .last()
        .map_or(false, |t| t.chars().all(|c| !c.is_ascii_alphanumeric()))
    {
        tokens.pop();
    }
    if let Some(cut) = locate_year_cut(&tokens) {
        tokens.truncate(cut);
        while tokens.last().map_or(false, |t| is_metadata_word(trim_token(t))) {
            tokens.pop();
        }
    }
    tokens.join(" ")
}

fn locate_year_cut(tokens: &[&str]) -> Option<usize> {
    for (idx, token) in tokens.iter().enumerate().rev() {
        let trimmed = trim_token(token);
        if trimmed.is_empty() {
            continue;
        }
        if is_year(trimmed)
            && tokens[..idx]
                .iter()
                .any(|t| t.chars().any(|c| c.is_ascii_alphabetic()))
        {
            return Some(idx);
        }
    }
    None
}

fn trim_token(token: &str) -> &str {
    token.trim_matches(|c: char| matches!(c, ',' | '.' | '!' | '?' | '/'))
}

fn is_year(token: &str) -> bool {
    token.len() == 4
        && token.chars().all(|c| c.is_ascii_digit())
        && matches!(token.parse::<u16>(), Ok(year) if (1800..=2099).contains(&year))
}

fn is_metadata_word(token: &str) -> bool {
    matches!(
        token,
        "film" | "films" | "movie" | "movies" | "episode" | "episodes" | "series"
            | "season" | "seasons" | "show" | "shows" | "novel" | "novels" | "game"
            | "games" | "album" | "albums" | "song" | "songs" | "soundtrack"
    )
}
