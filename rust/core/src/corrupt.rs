use crate::rng::Rng;

/// Corruption functions by severity tier.
/// Light (0–4): subtle, hard to detect. All levels include these.
/// Medium (5–9): visible distortion. Mid+ levels include these.
/// Heavy (10–14): data loss/destruction. High+ levels include these.
type CorruptFn = fn(&mut Rng, &mut [String], usize, usize);

const CORRUPTIONS: &[CorruptFn] = &[
    // 0–4: light
    insert_spaces,
    insert_invisible,
    decompose_accents,
    strip_spaces,
    duplicate_value,
    // 5–9: medium
    ocr_substitute,
    double_encode_utf8,
    html_encode,
    append_junk,
    swap_field,
    // 10–14: heavy
    clear_value,
    truncate,
    mask_chars,
    partial_mask,
    replace_with_x,
];

const LIGHT_MAX: usize = 4;
const MEDIUM_MAX: usize = 9;
const HEAVY_MAX: usize = 14;

fn corruption_params(rate: f64) -> (usize, usize) {
    if rate <= 0.02 {
        (LIGHT_MAX, 1)
    } else if rate <= 0.05 {
        (LIGHT_MAX, 2)
    } else if rate <= 0.15 {
        (MEDIUM_MAX, 3)
    } else if rate <= 0.45 {
        (HEAVY_MAX, 3)
    } else if rate <= 0.65 {
        (HEAVY_MAX, 4)
    } else {
        (HEAVY_MAX, 5)
    }
}

pub fn corrupt_values(rng: &mut Rng, values: &mut [String], rate: f64) {
    let len = values.len();
    if len == 0 {
        return;
    }

    let mut chosen: Vec<usize> = Vec::new();
    for (idx, val) in values.iter().enumerate() {
        if !val.is_empty() && rng.maybe(rate) {
            chosen.push(idx);
        }
    }
    if chosen.is_empty() {
        return;
    }

    let (max_type, max_passes) = corruption_params(rate);
    for i in chosen {
        let passes = if max_passes == 1 {
            1
        } else {
            let mut p = 1;
            while p < max_passes && rng.maybe(0.3) {
                p += 1;
            }
            p
        };
        for _ in 0..passes {
            if values[i].is_empty() {
                break;
            }
            let before = values[i].clone();
            let idx = rng.urange(0, max_type);
            CORRUPTIONS[idx](rng, values, i, len);
            // Guarantee mutation: if corruption was a no-op, append junk.
            if values[i] == before && !values[i].is_empty() {
                let n = rng.urange(1, 4);
                values[i].push_str(&rng.alnum(n));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Light (0–4): subtle corruptions
// ---------------------------------------------------------------------------

fn insert_spaces(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let mut chars: Vec<char> = values[i].chars().collect();
    if !chars.is_empty() {
        let pos = rng.urange(1, chars.len().saturating_sub(1).max(1));
        let n = rng.urange(2, 5);
        for _ in 0..n {
            chars.insert(pos, ' ');
        }
    }
    values[i] = chars.into_iter().collect();
}

fn insert_invisible(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let specials = ['\u{00a0}', '\u{200b}', '\u{feff}', '\u{00ad}'];
    let mut chars: Vec<char> = values[i].chars().collect();
    if !chars.is_empty() {
        let pos = rng.urange(0, chars.len().saturating_sub(1));
        chars.insert(pos, *rng.choice(&specials));
    }
    values[i] = chars.into_iter().collect();
}

fn decompose_accents(_rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let val = &values[i];
    let mut out = String::with_capacity(val.len() * 2);
    for c in val.chars() {
        match c {
            'é' => out.push_str("e\u{0301}"),
            'è' => out.push_str("e\u{0300}"),
            'ê' => out.push_str("e\u{0302}"),
            'ë' => out.push_str("e\u{0308}"),
            'á' => out.push_str("a\u{0301}"),
            'à' => out.push_str("a\u{0300}"),
            'â' => out.push_str("a\u{0302}"),
            'ä' => out.push_str("a\u{0308}"),
            'ö' => out.push_str("o\u{0308}"),
            'ü' => out.push_str("u\u{0308}"),
            'ñ' => out.push_str("n\u{0303}"),
            'ç' => out.push_str("c\u{0327}"),
            'ó' => out.push_str("o\u{0301}"),
            'ú' => out.push_str("u\u{0301}"),
            'í' => out.push_str("i\u{0301}"),
            'ý' => out.push_str("y\u{0301}"),
            'ž' => out.push_str("z\u{030c}"),
            'š' => out.push_str("s\u{030c}"),
            'č' => out.push_str("c\u{030c}"),
            'ř' => out.push_str("r\u{030c}"),
            'ő' => out.push_str("o\u{030b}"),
            'ű' => out.push_str("u\u{030b}"),
            other => out.push(other),
        }
    }
    values[i] = out;
}

fn strip_spaces(_rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    values[i] = values[i].replace(' ', "");
}

fn duplicate_value(_rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let dup = values[i].clone();
    values[i] = format!("{dup} {dup}");
}

// ---------------------------------------------------------------------------
// Medium (5–9): visible distortion
// ---------------------------------------------------------------------------

fn ocr_substitute(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    const OCR: &[(char, char)] = &[
        ('0', 'O'),
        ('O', '0'),
        ('1', 'l'),
        ('l', '1'),
        ('5', 'S'),
        ('S', '5'),
        ('8', 'B'),
        ('B', '8'),
        ('a', '@'),
        ('e', '3'),
        ('o', '0'),
        ('i', '!'),
        ('g', '9'),
        ('n', 'r'),
    ];
    let mut out = String::with_capacity(values[i].len());
    for c in values[i].chars() {
        if let Some(&(_, to)) = OCR.iter().find(|&&(from, _)| from == c) {
            out.push(if rng.maybe(0.4) { to } else { c });
        } else {
            out.push(c);
        }
    }
    values[i] = out;
}

fn double_encode_utf8(_rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    values[i] = values[i]
        .replace('ü', "\u{00c3}\u{00bc}")
        .replace('ö', "\u{00c3}\u{00b6}")
        .replace('ä', "\u{00c3}\u{00a4}")
        .replace('é', "\u{00c3}\u{00a9}")
        .replace('ñ', "\u{00c3}\u{00b1}")
        .replace('ã', "\u{00c3}\u{00a3}")
        .replace('ç', "\u{00c3}\u{00a7}");
}

fn html_encode(_rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    values[i] = values[i]
        .replace('&', "&amp;")
        .replace('\'', "&#39;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
}

fn append_junk(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let n = rng.urange(1, 4);
    values[i].push_str(&rng.alnum(n));
}

fn swap_field(rng: &mut Rng, values: &mut [String], i: usize, len: usize) {
    if len > 1 {
        let mut other = rng.urange(0, len - 2);
        if other >= i {
            other += 1;
        }
        let src = values[other].clone();
        values[i].clone_from(&src);
    }
}

// ---------------------------------------------------------------------------
// Heavy (10–14): data loss
// ---------------------------------------------------------------------------

fn clear_value(_rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    values[i] = String::new();
}

fn truncate(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let chars: Vec<char> = values[i].chars().collect();
    let clen = chars.len();
    if clen <= 1 {
        values[i] = String::new();
    } else {
        let cut = rng.urange(1, clen / 2);
        values[i] = chars[..cut].iter().collect();
    }
}

fn mask_chars(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let mut chars: Vec<char> = values[i].chars().collect();
    let runs = rng.urange(1, 3);
    for _ in 0..runs {
        if chars.is_empty() {
            break;
        }
        let pos = rng.urange(0, chars.len().saturating_sub(1));
        let n = rng.urange(1, 5).min(chars.len() - pos);
        for j in 0..n {
            chars[pos + j] = '*';
        }
    }
    values[i] = chars.into_iter().collect();
}

fn partial_mask(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let chars: Vec<char> = values[i].chars().collect();
    let clen = chars.len();
    let keep = rng.urange(1, (clen.max(3) / 3).max(1));
    values[i] = if rng.maybe(0.5) {
        let prefix: String = chars[..keep.min(clen)].iter().collect();
        format!("{prefix}{}", "*".repeat(clen.saturating_sub(keep)))
    } else {
        let suffix: String = chars[clen.saturating_sub(keep)..].iter().collect();
        format!("{}{suffix}", "*".repeat(clen.saturating_sub(keep)))
    };
}

fn replace_with_x(rng: &mut Rng, values: &mut [String], i: usize, _len: usize) {
    let chars: Vec<char> = values[i].chars().collect();
    let clen = chars.len();
    values[i] = if clen > 4 {
        let keep = rng.urange(3, 4);
        let suffix: String = chars[clen - keep..].iter().collect();
        format!("{}{suffix}", "X".repeat(clen - keep))
    } else {
        "X".repeat(clen)
    };
}
