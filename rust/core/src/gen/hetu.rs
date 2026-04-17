use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let d = ctx.rng.range(1, 28);
    let m = ctx.rng.range(1, 12);
    let y = ctx.rng.range(50, 99);
    let arr = ["-", "A"];
    let century = arr[ctx.rng.urange(0, arr.len() - 1)];
    let serial = ctx.rng.range(2, 899);
    buf.reserve(11);
    let _ = write!(buf, "{d:02}{m:02}{y:02}{century}{serial:03}");
    ctx.rng.push_charset(buf, b"0123456789ABCDEFHJKLMNPRSTUVWXY", 1);
}
