use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let street = if loc.streets.is_empty() { "Main Street" } else { ctx.rng.choice(loc.streets) };
    let n = ctx.rng.range(1, 9999);
    match loc.code {
        "de" | "at" | "nl" | "pt" | "it" => {
            buf.reserve(street.len() + 6);
            buf.push_str(street);
            let _ = write!(buf, " {n}");
        }
        "ja" => {
            let c = ctx.rng.range(1, 9);
            let b = ctx.rng.range(1, 30);
            let g = ctx.rng.range(1, 20);
            buf.reserve(10);
            let _ = write!(buf, "{c}-{b}-{g}");
        }
        _ => {
            buf.reserve(street.len() + 6);
            let _ = write!(buf, "{n} ");
            buf.push_str(street);
        }
    }
}
