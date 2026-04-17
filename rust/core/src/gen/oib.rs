use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_digits(buf, 11);
}
