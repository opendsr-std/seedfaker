use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(4 + 48);
    buf.push_str("cos_");
    ctx.rng.push_alnum(buf, 48);
}
