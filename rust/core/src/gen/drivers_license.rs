use std::fmt::Write;

use crate::ctx::GenContext;

use super::helpers::charsets::UPPER;
use super::helpers::locale_to_country_code;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let cc = locale_to_country_code(loc.code);
    match cc {
        "US" => {
            let arr = ["CA", "NY", "TX", "FL", "IL", "WA", "OR", "MA"];
            let st = arr[ctx.rng.urange(0, arr.len() - 1)];
            buf.reserve(12);
            buf.push_str(st);
            buf.push('-');
            ctx.rng.push_charset(buf, UPPER, 1);
            let _ = write!(buf, "{}", ctx.rng.range(1_000_000, 9_999_999));
        }
        "GB" => {
            // UK: SURNA YYMMDD XX 9XX
            buf.reserve(16);
            ctx.rng.push_upper(buf, 5);
            ctx.rng.push_digits(buf, 6);
            ctx.rng.push_upper(buf, 2);
            ctx.rng.push_digits(buf, 3);
        }
        "DE" => {
            buf.reserve(11);
            ctx.rng.push_upper(buf, 1);
            ctx.rng.push_digits(buf, 3);
            ctx.rng.push_upper(buf, 3);
            ctx.rng.push_digits(buf, 4);
        }
        "FR" => {
            buf.reserve(12);
            ctx.rng.push_digits(buf, 12);
        }
        _ => {
            buf.reserve(12);
            ctx.rng.push_upper(buf, 2);
            ctx.rng.push_digits(buf, 8);
        }
    }
}
