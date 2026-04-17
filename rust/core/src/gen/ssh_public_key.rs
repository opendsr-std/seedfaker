use crate::ctx::GenContext;

use super::helpers::charsets::B64_CHARSET;

// Format: RFC 8709 (Ed25519 SSH key) — https://www.rfc-editor.org/rfc/rfc8709
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // ssh-ed25519 AAAAC3NzaC1lZDI1NTE5 + 68chars + space + user@host
    buf.reserve(12 + 28 + 68 + 10);
    buf.push_str("ssh-ed25519 AAAAC3NzaC1lZDI1NTE5");
    ctx.rng.push_charset(buf, B64_CHARSET, 68);
    buf.push_str(" user@host");
}
