use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.push_str(if ctx.rng.maybe(0.5) { "true" } else { "false" });
}
