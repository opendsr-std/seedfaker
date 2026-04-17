use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "de" => {
            let n = ctx.rng.urange(1, 3);
            let num = ctx.rng.range(100, 9999);
            buf.reserve(n + 1 + 2 + 1 + 4);
            ctx.rng.push_upper(buf, n);
            buf.push(' ');
            ctx.rng.push_upper(buf, 2);
            let _ = write!(buf, " {num}");
        }
        "fr" => {
            buf.reserve(9);
            ctx.rng.push_upper(buf, 2);
            buf.push('-');
            ctx.rng.push_digits(buf, 3);
            buf.push('-');
            ctx.rng.push_upper(buf, 2);
        }
        "it" => {
            buf.reserve(8);
            ctx.rng.push_upper(buf, 2);
            buf.push(' ');
            ctx.rng.push_digits(buf, 3);
            ctx.rng.push_upper(buf, 2);
        }
        "es" => {
            buf.reserve(8);
            ctx.rng.push_digits(buf, 4);
            buf.push(' ');
            ctx.rng.push_upper(buf, 3);
        }
        "nl" => {
            buf.reserve(8);
            ctx.rng.push_upper(buf, 2);
            buf.push('-');
            ctx.rng.push_digits(buf, 3);
            buf.push('-');
            ctx.rng.push_upper(buf, 1);
        }
        "pl" => {
            buf.reserve(8);
            ctx.rng.push_upper(buf, 2);
            buf.push(' ');
            ctx.rng.push_digits(buf, 4);
            ctx.rng.push_upper(buf, 1);
        }
        "se" => {
            buf.reserve(7);
            ctx.rng.push_upper(buf, 3);
            buf.push(' ');
            ctx.rng.push_digits(buf, 3);
        }
        "ja" => {
            buf.reserve(10);
            ctx.rng.push_digits(buf, 3);
            buf.push(' ');
            ctx.rng.push_upper(buf, 1);
            buf.push(' ');
            ctx.rng.push_digits(buf, 4);
        }
        "tr" => {
            let n = ctx.rng.urange(1, 3);
            buf.reserve(2 + 1 + n + 1 + 4);
            ctx.rng.push_digits(buf, 2);
            buf.push(' ');
            ctx.rng.push_upper(buf, n);
            buf.push(' ');
            ctx.rng.push_digits(buf, 4);
        }
        "uk" | "be" | "sr" => {
            buf.reserve(9);
            ctx.rng.push_upper(buf, 2);
            ctx.rng.push_digits(buf, 4);
            buf.push(' ');
            ctx.rng.push_upper(buf, 2);
        }
        "pt-br" | "ar" | "mx" | "cl" | "co" => {
            buf.reserve(7);
            ctx.rng.push_upper(buf, 3);
            ctx.rng.push_digits(buf, 4);
        }
        "hi" => {
            buf.reserve(13);
            ctx.rng.push_upper(buf, 2);
            buf.push('-');
            ctx.rng.push_digits(buf, 2);
            buf.push('-');
            ctx.rng.push_upper(buf, 2);
            buf.push('-');
            ctx.rng.push_digits(buf, 4);
        }
        "zh" => {
            buf.reserve(6);
            ctx.rng.push_upper(buf, 1);
            ctx.rng.push_digits(buf, 5);
        }
        _ => {
            // Must preserve RNG order: upper(3), then range
            buf.reserve(8);
            ctx.rng.push_upper(buf, 3);
            let num = ctx.rng.range(1000, 9999);
            let _ = write!(buf, "-{num}");
        }
    }
}
