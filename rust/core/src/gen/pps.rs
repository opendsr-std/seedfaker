use crate::ctx::GenContext;

use super::helpers::charsets::UPPER;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let n = ctx.rng.urange(1, 2);
    buf.reserve(7 + n);
    ctx.rng.push_digits(buf, 7);
    ctx.rng.push_charset(buf, UPPER, n);
}
