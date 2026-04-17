use crate::rng::Rng;

/// Convert a name to a lowercase ASCII handle-safe string.
/// - ASCII names: lowercased, spaces/apostrophes/hyphens stripped
///   ("de Smedt" → "desmedt", "O'Brien" → "obrien", "Mary-Jane" → "maryjane")
/// - Non-ASCII names: generate 4-7 lowercase letters (looks like a short handle)
pub fn ascii_lower(rng: &mut Rng, name: &str) -> String {
    if name.is_ascii() {
        let mut s = String::with_capacity(name.len());
        for c in name.chars() {
            if c.is_ascii_alphabetic() {
                s.push(c.to_ascii_lowercase());
            }
            // skip spaces, apostrophes, hyphens, dots, etc.
        }
        if s.is_empty() {
            // edge case: name was all non-alpha ASCII (shouldn't happen)
            let n = rng.urange(4, 7);
            rng.push_lower(&mut s, n);
        }
        s
    } else {
        let n = rng.urange(4, 7);
        let mut s = String::with_capacity(n);
        rng.push_lower(&mut s, n);
        s
    }
}
