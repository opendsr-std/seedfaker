use crate::ctx::GenContext;

const BASE58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let len = ctx.rng.urange(32, 44);
    ctx.rng.push_charset(buf, BASE58, len);
}
