use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if let Some(id) = ctx.identity {
        buf.push_str(&id.last_name);
        return;
    }
    let loc = ctx.locale();
    buf.push_str(loc.last_names[ctx.rng.urange(0, loc.last_names.len() - 1)]);
}
