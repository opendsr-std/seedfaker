use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(5 + 32);
    buf.push_str("NRAK-");
    ctx.rng.push_alnum(buf, 32);
}
