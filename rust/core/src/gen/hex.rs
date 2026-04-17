use crate::ctx::GenContext;

use super::helpers::charsets::primitive_len;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let n = match ctx.modifier {
        "byte" => 2,
        other => primitive_len(other, &mut ctx.rng),
    };
    ctx.rng.push_hex(buf, n);
}
