use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(3);
    for _ in 0..3 {
        buf.push((b'a' + ctx.rng.range(0, 25) as u8) as char);
    }
}
