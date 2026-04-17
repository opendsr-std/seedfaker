use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(3 + 48);
    buf.push_str("rt_");
    ctx.rng.push_alnum(buf, 48);
}
