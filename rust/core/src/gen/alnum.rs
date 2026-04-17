use crate::ctx::GenContext;

use super::helpers::charsets::primitive_len;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let n = primitive_len(ctx.modifier, &mut ctx.rng);
    ctx.rng.push_alnum(buf, n);
}
