use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(10);
    ctx.rng.push_upper(buf, 5);
    ctx.rng.push_digits(buf, 4);
    ctx.rng.push_upper(buf, 1);
}
