use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let s = (0..10).map(|_| ctx.rng.hex_str(4).to_uppercase()).collect::<Vec<_>>().join(" ");
    buf.push_str(&s);
}
