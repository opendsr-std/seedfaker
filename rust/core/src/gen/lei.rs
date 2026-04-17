use crate::ctx::GenContext;

// Format: ISO 17442 (LEI) — https://www.iso.org/standard/78829.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let s = ctx.rng.alnum(20).to_uppercase();
    buf.push_str(&s);
}
