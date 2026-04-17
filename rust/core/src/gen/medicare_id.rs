use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_upper_digit(buf, 11);
}
