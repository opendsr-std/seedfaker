use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if ctx.rng.maybe(0.5) {
        let n = ctx.rng.range(100_000, 9_999_999);
        buf.reserve(12);
        let _ = write!(buf, "MRN-{n}");
    } else {
        let n = ctx.rng.range(10_000_000, 99_999_999);
        buf.reserve(10);
        let _ = write!(buf, "MR{n}");
    }
}
