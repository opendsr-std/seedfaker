use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(11);
    for i in 0..9 {
        if i == 3 || i == 6 {
            buf.push(' ');
        }
        let d = ctx.rng.range(0, 9);
        buf.push((b'0' + d as u8) as char);
    }
}
