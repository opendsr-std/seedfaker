use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let n = ctx.rng.range(1, 9999);
    buf.reserve(5);
    let _ = write!(buf, "#{n}");
}
