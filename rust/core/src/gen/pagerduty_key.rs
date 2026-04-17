use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(2 + 20);
    buf.push_str("u+");
    ctx.rng.push_alnum(buf, 20);
}
