use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(8);
    buf.push('P');
    ctx.rng.push_digits(buf, 7);
}
