use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(3 + 120);
    buf.push_str("eyJ");
    ctx.rng.push_alnum(buf, 120);
}
