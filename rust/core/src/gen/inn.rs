use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if ctx.rng.maybe(0.5) {
        ctx.rng.push_digits(buf, 10);
    } else {
        ctx.rng.push_digits(buf, 12);
    }
}
