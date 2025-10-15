/// sanitize words in the wordlist, get rid of special chars and spaces and strip but keep original
/// just in case
pub fn stream(word: &str) -> impl Iterator<Item = String> {
    let mut sanitized = String::new();
    let mut in_paren = false;
    for c in word.chars() {
        if c == '(' {
            in_paren = true;
        } else if c == ')' {
            in_paren = false;
        } else if !in_paren {
            sanitized.push(c.to_ascii_lowercase());
        }
    }

    // characters to strip
    let special: &[char] = &[
        '•','!','@','#','$','%','^','&','*','(',')','-','_','=', '+','[',']','{','}','|',';',
        ':','\'','"',',','.','<','>','/','?','`','~','\\',' ',
    ];
    let mut stripped = String::new();
    for c in sanitized.chars() {
        if !special.contains(&c) {
            stripped.push(c);
        }
    }

    let clean = stripped.trim().to_string();
    let orig  = sanitized.trim().to_string();

    let mut out = Vec::new();
    if clean.len() >= 2 && clean.len() <= 28 {
        out.push(clean.clone());
    }
    if orig != clean && orig.len() >= 2 && orig.len() <= 28 {
        out.push(orig);
    }
    out.into_iter()
}
