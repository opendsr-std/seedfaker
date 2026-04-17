use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let arr = ["STU", "SID", ""];
    let p = arr[ctx.rng.urange(0, arr.len() - 1)];
    let n = ctx.rng.range(100_000, 999_999_999);
    if p.is_empty() {
        buf.reserve(9);
        let _ = write!(buf, "{n}");
    } else {
        buf.reserve(p.len() + 1 + 9);
        buf.push_str(p);
        let _ = write!(buf, "-{n}");
    }
}
