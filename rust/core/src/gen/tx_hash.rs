use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if ctx.modifier == "btc" {
        ctx.rng.push_hex(buf, 64);
    } else {
        buf.reserve(66);
        buf.push_str("0x");
        ctx.rng.push_hex(buf, 64);
    }
}
