use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(42);
    buf.push_str("0x");
    ctx.rng.push_hex(buf, 40);
}
