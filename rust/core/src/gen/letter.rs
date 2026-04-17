use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let c = b'a' + (ctx.rng.range(0, 25) as u8);
    buf.push(c as char);
}
