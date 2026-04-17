use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let n = ctx.rng.range(1_000_000, 9_999_999);
    buf.reserve(10);
    let _ = write!(buf, "RX-{n}");
}
