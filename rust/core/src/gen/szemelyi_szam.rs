use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(12);
    ctx.rng.push_digits(buf, 2);
    buf.push('-');
    ctx.rng.push_digits(buf, 6);
    buf.push('-');
    let _ = write!(buf, "{}", ctx.rng.range(0, 9));
}
