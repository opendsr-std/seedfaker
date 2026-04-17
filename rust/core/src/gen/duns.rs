use crate::ctx::GenContext;

// Format: D-U-N-S Number — https://www.dnb.com/duns.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // 2d + - + 3d + - + 4d = 11
    buf.reserve(11);
    ctx.rng.push_digits(buf, 2);
    buf.push('-');
    ctx.rng.push_digits(buf, 3);
    buf.push('-');
    ctx.rng.push_digits(buf, 4);
}
