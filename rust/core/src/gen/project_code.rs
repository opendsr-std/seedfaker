use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // Must preserve RNG order: upper(4), then range(1000, 9999)
    buf.reserve(13);
    buf.push_str("PRJ-");
    ctx.rng.push_upper(buf, 4);
    let n = ctx.rng.range(1000, 9999);
    let _ = write!(buf, "-{n}");
}
