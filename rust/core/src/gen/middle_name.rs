use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    buf.push_str(loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)]);
}
