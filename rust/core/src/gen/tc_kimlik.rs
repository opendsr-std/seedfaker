use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(11);
    let _ = write!(buf, "{}", ctx.rng.range(1, 9));
    ctx.rng.push_digits(buf, 10);
}
