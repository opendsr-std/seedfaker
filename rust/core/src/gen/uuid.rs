use crate::ctx::GenContext;

// Format: RFC 4122 (UUID v4) — https://www.rfc-editor.org/rfc/rfc4122
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(36);
    ctx.rng.push_hex(buf, 8);
    if ctx.modifier != "plain" {
        buf.push('-');
    }
    ctx.rng.push_hex(buf, 4);
    if ctx.modifier != "plain" {
        buf.push('-');
    }
    buf.push('4');
    ctx.rng.push_hex(buf, 3);
    if ctx.modifier != "plain" {
        buf.push('-');
    }
    ctx.rng.push_charset(buf, b"89ab", 1);
    ctx.rng.push_hex(buf, 3);
    if ctx.modifier != "plain" {
        buf.push('-');
    }
    ctx.rng.push_hex(buf, 12);
}
