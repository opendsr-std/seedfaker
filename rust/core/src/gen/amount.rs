use std::fmt::Write;

use crate::ctx::GenContext;

pub fn locale_currency_format(code: &str) -> (&'static str, &'static str, &'static str) {
    match code {
        "de" | "fr" | "it" | "es" | "nl" | "pt" | "pl" | "cs" | "sk" | "hu" | "ro" | "hr"
        | "bg" | "sr" | "sl" | "el" | "lt" | "lv" | "et" => ("\u{20ac}", ".", ","),
        "se" | "da" | "no" | "fi" => ("\u{20ac}", "\u{00a0}", ","),
        "ja" | "zh" => ("\u{00a5}", ",", ""),
        "hi" => ("\u{20b9}", ",", ""),
        "tr" => ("\u{20ba}", ".", ","),
        "ru" => ("\u{20bd}", "\u{00a0}", ","),
        "uk" | "be" => ("\u{20b4}", "\u{00a0}", ","),
        "ko" => ("\u{20a9}", ",", ""),
        "en-gb" => ("\u{00a3}", ",", "."),
        _ => ("$", ",", "."),
    }
}

fn push_thousands(buf: &mut String, n: i64, sep: &str) {
    if n < 0 {
        buf.push('-');
        push_thousands(buf, -n, sep);
        return;
    }
    let mut ib = itoa::Buffer::new();
    let s = ib.format(n);
    let len = s.len();
    let first = len % 3;
    if first > 0 {
        buf.push_str(&s[..first]);
    }
    let mut i = first;
    while i < len {
        if i > 0 && !sep.is_empty() {
            buf.push_str(sep);
        }
        buf.push_str(&s[i..i + 3]);
        i += 3;
    }
}

/// Pseudo log-normal distribution for realistic financial amounts.
/// Median ~$80, mean ~$800. Matches real transaction/invoice data.
///
/// | Range       | Probability | Examples              |
/// |-------------|-------------|-----------------------|
/// | $1-$50      | 30%         | coffee, lunch, books  |
/// | $50-$200    | 30%         | groceries, clothing   |
/// | $200-$1000  | 20%         | electronics, bills    |
/// | $1K-$10K    | 12%         | rent, furniture       |
/// | $10K-$100K  | 6%          | car, tuition          |
/// | $100K-$999K | 2%          | real estate           |
fn tiered_amount(rng: &mut crate::rng::Rng) -> i64 {
    let w = rng.urange(0, 99);
    match w {
        0..=29 => rng.range(1, 50),            // 30% small
        30..=59 => rng.range(50, 200),         // 30% medium
        60..=79 => rng.range(200, 1000),       // 20% large
        80..=91 => rng.range(1000, 10_000),    // 12% very large
        92..=97 => rng.range(10_000, 100_000), //  6% huge
        _ => rng.range(100_000, 999_999),      //  2% massive
    }
}

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    // locale() consumes RNG — must be called before value/cents to preserve sequence
    let _ = ctx.locale();
    let value = if let Some((min, max)) = ctx.range {
        if let Some(z) = ctx.zipf {
            ctx.rng.zipf_range(min, max, z.s)
        } else {
            ctx.rng.range(min, max)
        }
    } else {
        tiered_amount(&mut ctx.rng)
    };
    let cents = ctx.rng.range(0, 99);
    value as f64 + cents as f64 / 100.0
}

pub fn fmt(v: f64, ctx: &mut GenContext<'_>, buf: &mut String) {
    let value = v.trunc() as i64;
    let cents = ((v.fract().abs()) * 100.0 + 0.5) as i64;
    let loc = ctx.locale();

    let (symbol, sep, dec) = match ctx.modifier {
        "dot" => ("", ",", "."),
        "comma" => ("", ".", ","),
        "plain" => {
            let _ = write!(buf, "{value}.{cents:02}");
            return;
        }
        "usd" => ("$", ",", "."),
        "eur" => ("\u{20ac}", ".", ","),
        "gbp" => ("\u{00a3}", ",", "."),
        _ => locale_currency_format(loc.code),
    };
    if value < 0 {
        buf.push('-');
    }
    buf.push_str(symbol);
    push_thousands(buf, value.abs(), sep);
    if !dec.is_empty() {
        buf.push_str(dec);
        super::date::push_pad2(buf, cents);
    }
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
