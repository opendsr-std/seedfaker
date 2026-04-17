use crate::ctx::GenContext;

use super::helpers::charsets::B64_CHARSET;

// Format: RFC 7468 (PEM encoding) — https://www.rfc-editor.org/rfc/rfc7468
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let (begin, end) = match ctx.rng.urange(0, 2) {
        0 => ("-----BEGIN RSA PRIVATE KEY-----", "-----END RSA PRIVATE KEY-----"),
        1 => ("-----BEGIN EC PRIVATE KEY-----", "-----END EC PRIVATE KEY-----"),
        _ => ("-----BEGIN OPENSSH PRIVATE KEY-----", "-----END OPENSSH PRIVATE KEY-----"),
    };
    let lines = ctx.rng.urange(3, 6);
    // begin + \n + lines*65 (64 chars + \n) + end
    buf.reserve(begin.len() + 1 + lines * 65 + end.len());
    buf.push_str(begin);
    for _ in 0..lines {
        buf.push('\n');
        ctx.rng.push_charset(buf, B64_CHARSET, 64);
    }
    buf.push('\n');
    buf.push_str(end);
}
