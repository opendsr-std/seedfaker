use crate::ctx::GenContext;

use super::helpers::charsets::primitive_len;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let n = primitive_len(ctx.modifier, &mut ctx.rng);
    let raw: Vec<u8> = (0..n).map(|_| ctx.rng.range(0, 255) as u8).collect();
    buf.push_str(&super::helpers::base64url::base64url_encode(&raw));
}
