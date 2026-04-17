use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    buf.push_str(loc.companies[ctx.rng.urange(0, loc.companies.len() - 1)]);
}
