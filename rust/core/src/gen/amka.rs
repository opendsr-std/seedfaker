use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let d = ctx.rng.range(1, 28);
    let m = ctx.rng.range(1, 12);
    let y = ctx.rng.range(50, 99);
    buf.reserve(11);
    let _ = write!(buf, "{d:02}{m:02}{y:02}");
    ctx.rng.push_digits(buf, 5);
}
