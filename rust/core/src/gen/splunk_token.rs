use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // hex(8) + "-" + hex(4) + "-4" + hex(3) + "-" + charset(1) + hex(3) + "-" + hex(12) = 36
    buf.reserve(36);
    ctx.rng.push_hex(buf, 8);
    buf.push('-');
    ctx.rng.push_hex(buf, 4);
    buf.push_str("-4");
    ctx.rng.push_hex(buf, 3);
    buf.push('-');
    ctx.rng.push_charset(buf, b"89ab", 1);
    ctx.rng.push_hex(buf, 3);
    buf.push('-');
    ctx.rng.push_hex(buf, 12);
}
