use crate::ctx::GenContext;

use super::helpers::charsets::primitive_len;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let n = primitive_len(ctx.modifier, &mut ctx.rng);
    ctx.rng.push_charset(buf, LOWER, n);
}
