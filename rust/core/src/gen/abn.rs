use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(14);
    ctx.rng.push_digits(buf, 2);
    buf.push(' ');
    ctx.rng.push_digits(buf, 3);
    buf.push(' ');
    ctx.rng.push_digits(buf, 3);
    buf.push(' ');
    ctx.rng.push_digits(buf, 3);
}
