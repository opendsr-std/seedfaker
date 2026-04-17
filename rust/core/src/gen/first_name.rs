use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if let Some(id) = ctx.identity {
        buf.push_str(&id.first_name);
        return;
    }
    let loc = ctx.locale();
    buf.push_str(loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)]);
}
