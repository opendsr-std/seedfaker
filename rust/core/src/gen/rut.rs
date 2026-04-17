use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let n = ctx.rng.range(5_000_000, 25_000_000);
    let millions = n / 1_000_000;
    let thousands = (n % 1_000_000) / 1000;
    let rest = n % 1000;
    buf.reserve(12);
    let _ = write!(buf, "{millions}.{thousands:03}.{rest:03}-");
    ctx.rng.push_charset(buf, b"0123456789K", 1);
}
