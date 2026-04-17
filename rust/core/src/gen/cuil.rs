use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let arr = ["20", "23", "24", "27"];
    let prefix = arr[ctx.rng.urange(0, arr.len() - 1)];
    buf.reserve(13);
    buf.push_str(prefix);
    buf.push('-');
    ctx.rng.push_digits(buf, 8);
    buf.push('-');
    let _ = write!(buf, "{}", ctx.rng.range(0, 9));
}
