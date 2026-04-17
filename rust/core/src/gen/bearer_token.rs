use crate::ctx::GenContext;

use super::helpers::charsets::ALNUM_SPECIAL;

// Format: RFC 6750 (Bearer Token) — https://www.rfc-editor.org/rfc/rfc6750
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let len = ctx.rng.urange(40, 100);
    // "Bearer " (7) + charset(len)
    buf.reserve(7 + len);
    buf.push_str("Bearer ");
    ctx.rng.push_charset(buf, ALNUM_SPECIAL, len);
}
