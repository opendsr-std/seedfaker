use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let d = ctx.rng.range(1, 28);
    let m = ctx.rng.range(1, 12);
    let y = ctx.rng.range(950, 999);
    buf.reserve(13);
    let _ = write!(buf, "{d:02}{m:02}{y}");
    ctx.rng.push_digits(buf, 2);
    ctx.rng.push_digits(buf, 4);
}
