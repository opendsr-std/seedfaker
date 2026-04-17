use crate::ctx::GenContext;

// Format: RFC 6238 (TOTP) / RFC 4648 (Base32) — https://www.rfc-editor.org/rfc/rfc6238
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_charset(buf, b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", 32);
}
