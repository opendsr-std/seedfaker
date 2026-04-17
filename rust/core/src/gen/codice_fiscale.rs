use std::fmt::Write;

use crate::ctx::GenContext;

use super::helpers::charsets::UPPER;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(16);
    ctx.rng.push_upper(buf, 3); // surname
    ctx.rng.push_upper(buf, 3); // name
    ctx.rng.push_digits(buf, 2); // year
    ctx.rng.push_charset(buf, b"ABCDEHLMPRST", 1); // month
    let day = ctx.rng.range(1, 71);
    let _ = write!(buf, "{day:02}");
    ctx.rng.push_charset(buf, UPPER, 1); // town letter (from_charset)
    ctx.rng.push_digits(buf, 3); // town digits
    ctx.rng.push_charset(buf, UPPER, 1); // check (from_charset)
}
