use crate::ctx::GenContext;

// ─── common leaked passwords (NordPass/HaveIBeenPwned top lists) ─────
const COMMON: &[&str] = &[
    "123456",
    "123456789",
    "12345678",
    "password",
    "qwerty123",
    "qwerty1",
    "111111",
    "12345",
    "secret",
    "123123",
    "1234567890",
    "1234567",
    "000000",
    "qwerty",
    "abc123",
    "password1",
    "iloveyou",
    "sunshine",
    "princess",
    "football",
    "charlie",
    "shadow",
    "master",
    "dragon",
    "michael",
    "letmein",
    "monkey",
    "trustno1",
    "hello",
    "freedom",
    "whatever",
    "nicole",
    "jordan",
    "cameron",
    "access",
    "654321",
    "pass123",
    "abcd1234",
    "121212",
    "bailey",
];

// ─── word fragments for personal passwords ───────────────────────────
const PERSONAL_WORDS: &[&str] = &[
    "love", "baby", "angel", "dream", "star", "cool", "super", "happy", "lucky", "sweet", "sunny",
    "cat", "dog", "fish", "bear", "tiger", "blue", "red", "green", "dark", "light", "fire", "ice",
    "moon", "sky", "rock", "game", "play", "win", "king", "best", "pro", "my", "the", "big", "lil",
    "old", "new", "hot", "max",
];

const PASSPHRASE_WORDS: &[&str] = &[
    "correct", "horse", "battery", "staple", "orange", "purple", "diamond", "sunset", "thunder",
    "falcon", "castle", "river", "silver", "garden", "winter", "cosmic", "rocket", "marble",
];

/// Realistic password distribution matching real-world leaked data.
///
/// Default: mix of common leaked, personal word+digits, random, passphrase.
/// Modifiers:
/// - `pin`: 4-6 digit PIN
/// - `memorable`: passphrase (correct-horse-battery)
/// - `mixed`: random strong password (Kx7#mQ9p)
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    match ctx.modifier {
        "pin" => gen_pin(ctx, buf),
        "memorable" => gen_passphrase(ctx, buf),
        "mixed" => gen_mixed(ctx, buf),
        "strong" => gen_strong(ctx, buf),
        m if !m.is_empty() && m.as_bytes()[0].is_ascii_digit() => {
            let len = m.parse::<usize>().unwrap_or(12);
            gen_mixed_len(ctx, buf, len);
        }
        _ => gen_realistic(ctx, buf),
    }
}

/// Realistic: distribution matching actual user behavior.
fn gen_realistic(ctx: &mut GenContext<'_>, buf: &mut String) {
    let w = ctx.rng.urange(0, 99);
    match w {
        // 40% common leaked password (with mutations)
        0..=39 => {
            let base = COMMON[ctx.rng.urange(0, COMMON.len() - 1)];
            buf.push_str(base);
            // 30% chance: mutate (append digit, swap case, add !)
            if ctx.rng.urange(0, 99) < 30 {
                mutate_password(ctx, buf);
            }
        }
        // 25% personal word + digits (michael2005, loveyou123)
        40..=64 => {
            let w1 = PERSONAL_WORDS[ctx.rng.urange(0, PERSONAL_WORDS.len() - 1)];
            buf.push_str(w1);
            let pattern = ctx.rng.urange(0, 99);
            match pattern {
                // word + year
                0..=34 => {
                    let year = ctx.rng.range(1985, 2010);
                    buf.push_str(itoa::Buffer::new().format(year));
                }
                // word + 2-3 digits
                35..=59 => {
                    let n = ctx.rng.range(1, 999);
                    buf.push_str(itoa::Buffer::new().format(n));
                }
                // word + word
                60..=79 => {
                    let w2 = PERSONAL_WORDS[ctx.rng.urange(0, PERSONAL_WORDS.len() - 1)];
                    buf.push_str(w2);
                    if ctx.rng.urange(0, 99) < 50 {
                        let n = ctx.rng.range(1, 99);
                        buf.push_str(itoa::Buffer::new().format(n));
                    }
                }
                // word + 123 / word + !
                _ => {
                    let sfx =
                        ["123", "1234", "!", "!!", "1", "01", "69", "007"][ctx.rng.urange(0, 7)];
                    buf.push_str(sfx);
                }
            }
        }
        // 20% random mixed (strong)
        65..=84 => gen_mixed(ctx, buf),
        // 10% passphrase
        85..=94 => gen_passphrase(ctx, buf),
        // 3% = identity username (if ctx strict)
        95..=97 => {
            if let Some(id) = ctx.identity {
                buf.push_str(&id.first_ascii);
                let y = ctx.rng.range(1, 999);
                buf.push_str(itoa::Buffer::new().format(y));
            } else {
                let w1 = PERSONAL_WORDS[ctx.rng.urange(0, PERSONAL_WORDS.len() - 1)];
                buf.push_str(w1);
                buf.push_str(itoa::Buffer::new().format(ctx.rng.range(1, 9999)));
            }
        }
        // 2% PIN
        _ => gen_pin(ctx, buf),
    }
}

fn gen_pin(ctx: &mut GenContext<'_>, buf: &mut String) {
    let len = ctx.rng.urange(4, 6);
    ctx.rng.push_digits(buf, len);
}

fn gen_passphrase(ctx: &mut GenContext<'_>, buf: &mut String) {
    let sep = if ctx.rng.maybe(0.5) { "-" } else { "_" };
    let n = ctx.rng.urange(3, 5);
    buf.push_str(&ctx.rng.sample(PASSPHRASE_WORDS, n).join(sep));
}

const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGIT: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_-+=~?.";

const MIXED_CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*_-+=";

/// Strong password: 16-24 chars, guaranteed ≥2 of each class,
/// no 3+ consecutive identical chars. Deterministic.
fn gen_strong(ctx: &mut GenContext<'_>, buf: &mut String) {
    let len = ctx.rng.urange(16, 24);

    // Build password in a mutable Vec<u8> (all chars are ASCII)
    let mut pw: Vec<u8> = Vec::with_capacity(len);

    // Guaranteed chars: 2 upper, 2 lower, 2 digit, 2 symbol
    pw.push(UPPER[ctx.rng.urange(0, UPPER.len() - 1)]);
    pw.push(UPPER[ctx.rng.urange(0, UPPER.len() - 1)]);
    pw.push(LOWER[ctx.rng.urange(0, LOWER.len() - 1)]);
    pw.push(LOWER[ctx.rng.urange(0, LOWER.len() - 1)]);
    pw.push(DIGIT[ctx.rng.urange(0, DIGIT.len() - 1)]);
    pw.push(DIGIT[ctx.rng.urange(0, DIGIT.len() - 1)]);
    pw.push(SYMBOL[ctx.rng.urange(0, SYMBOL.len() - 1)]);
    pw.push(SYMBOL[ctx.rng.urange(0, SYMBOL.len() - 1)]);

    // Fill remaining with mixed charset
    for _ in 8..len {
        pw.push(MIXED_CHARSET[ctx.rng.urange(0, MIXED_CHARSET.len() - 1)]);
    }

    // Fisher-Yates shuffle
    for i in (1..len).rev() {
        let j = ctx.rng.urange(0, i);
        pw.swap(i, j);
    }

    // Eliminate 3+ consecutive identical chars
    for i in 2..len {
        if pw[i] == pw[i - 1] && pw[i] == pw[i - 2] {
            let ch = pw[i];
            let replacement = match ch {
                b'A'..=b'Z' => UPPER[ctx.rng.urange(0, UPPER.len() - 1)],
                b'a'..=b'z' => LOWER[ctx.rng.urange(0, LOWER.len() - 1)],
                b'0'..=b'9' => DIGIT[ctx.rng.urange(0, DIGIT.len() - 1)],
                _ => SYMBOL[ctx.rng.urange(0, SYMBOL.len() - 1)],
            };
            pw[i] = replacement;
        }
    }

    // All bytes are ASCII — safe to convert
    for &b in &pw {
        buf.push(b as char);
    }
}

fn gen_mixed(ctx: &mut GenContext<'_>, buf: &mut String) {
    let len = ctx.rng.urange(8, 24);
    ctx.rng.push_charset(buf, MIXED_CHARSET, len);
}

fn gen_mixed_len(ctx: &mut GenContext<'_>, buf: &mut String, len: usize) {
    ctx.rng.push_charset(buf, MIXED_CHARSET, len);
}

fn mutate_password(ctx: &mut GenContext<'_>, buf: &mut String) {
    let m = ctx.rng.urange(0, 3);
    match m {
        0 => buf.push_str(itoa::Buffer::new().format(ctx.rng.range(1, 99))),
        1 => buf.push('!'),
        2 => buf.push_str("123"),
        _ => {
            // Capitalize first char
            if buf.as_bytes().first().is_some_and(u8::is_ascii_lowercase) {
                let mut chars = buf.chars();
                let Some(first) = chars.next() else { return };
                let upper = first.to_ascii_uppercase();
                let rest: String = chars.collect();
                buf.clear();
                buf.push(upper);
                buf.push_str(&rest);
            }
        }
    }
}
