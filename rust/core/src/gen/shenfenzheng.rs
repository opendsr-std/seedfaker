use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let region = ctx.rng.range(110_000, 659_999);
    let y = ctx.rng.range(1960, 2005);
    let m = ctx.rng.range(1, 12);
    let d = ctx.rng.range(1, 28);
    let serial = ctx.rng.range(100, 999);
    buf.reserve(18);
    let _ = write!(buf, "{region}{y}{m:02}{d:02}{serial}");
    ctx.rng.push_charset(buf, b"0123456789X", 1);
}
