use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if let Some(id) = ctx.identity {
        buf.push_str(&id.city);
        return;
    }
    let loc = ctx.pick_locale();
    let i = ctx.rng.urange(0, loc.cities.len() - 1);
    buf.push_str(loc.cities[i].name);
}
