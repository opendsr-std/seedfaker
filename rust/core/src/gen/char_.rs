use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let c = CHARSET[ctx.rng.urange(0, CHARSET.len() - 1)];
    buf.push(c as char);
}
