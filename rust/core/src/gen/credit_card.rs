use crate::ctx::GenContext;
use crate::rng::Rng;

fn luhn_check_digit(digits: &[u8]) -> u8 {
    let mut sum = 0u32;
    for (i, &d) in digits.iter().rev().enumerate() {
        let mut v = u32::from(d);
        if i % 2 == 0 {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
    }
    ((10 - (sum % 10)) % 10) as u8
}

fn push_card_digits(rng: &mut Rng, buf: &mut String, prefix: &[u8], total_len: usize) {
    let random_count = total_len.saturating_sub(prefix.len()).saturating_sub(1);
    let mut digits: Vec<u8> = Vec::with_capacity(total_len);
    digits.extend_from_slice(prefix);
    for _ in 0..random_count {
        digits.push(rng.urange(0, 9) as u8);
    }
    let check = luhn_check_digit(&digits);
    digits.push(check);
    buf.reserve(total_len);
    for d in &digits {
        buf.push((b'0' + d) as char);
    }
}

// Format: ISO/IEC 7812 + Luhn algorithm (ISO/IEC 7812-1)
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let sep = match ctx.modifier {
        "space" => " ",
        "dash" => "-",
        "plain" => "",
        _ => {
            let arr = [" ", "-", ""];
            arr[ctx.rng.urange(0, arr.len() - 1)]
        }
    };

    let mut digits_buf = String::with_capacity(19);
    let formatted = match ctx.rng.urange(0, 3) {
        0 => {
            push_card_digits(&mut ctx.rng, &mut digits_buf, &[4], 16);
            format_card_4444(&digits_buf, sep)
        }
        1 => {
            let sub = ctx.rng.urange(1, 5) as u8;
            push_card_digits(&mut ctx.rng, &mut digits_buf, &[5, sub], 16);
            format_card_4444(&digits_buf, sep)
        }
        2 => {
            let arr = [4u8, 7];
            let mid = arr[ctx.rng.urange(0, arr.len() - 1)];
            push_card_digits(&mut ctx.rng, &mut digits_buf, &[3, mid], 15);
            format_card_amex(&digits_buf, sep)
        }
        _ => {
            push_card_digits(&mut ctx.rng, &mut digits_buf, &[6, 0, 1, 1], 16);
            format_card_4444(&digits_buf, sep)
        }
    };
    buf.push_str(&formatted);
}

fn format_card_4444(n: &str, sep: &str) -> String {
    if sep.is_empty() {
        return n.to_string();
    }
    let mut out = String::with_capacity(n.len() + 3 * sep.len());
    out.push_str(&n[..4]);
    out.push_str(sep);
    out.push_str(&n[4..8]);
    out.push_str(sep);
    out.push_str(&n[8..12]);
    out.push_str(sep);
    out.push_str(&n[12..]);
    out
}

fn format_card_amex(n: &str, sep: &str) -> String {
    if sep.is_empty() {
        return n.to_string();
    }
    let mut out = String::with_capacity(n.len() + 2 * sep.len());
    out.push_str(&n[..4]);
    out.push_str(sep);
    out.push_str(&n[4..10]);
    out.push_str(sep);
    out.push_str(&n[10..]);
    out
}
