use crate::ctx::GenContext;

use super::helpers::{ascii_lower, base64url::base64url_encode};

// Format: RFC 7617 (HTTP Basic Auth) — https://www.rfc-editor.org/rfc/rfc7617
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let user_raw = loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)];
    let user = ascii_lower(&mut ctx.rng, user_raw);
    let pass = ctx.rng.alnum(12);
    let mut cred = String::with_capacity(user.len() + 1 + 12);
    cred.push_str(&user);
    cred.push(':');
    cred.push_str(&pass);
    let encoded = base64url_encode(cred.as_bytes());
    buf.reserve(6 + encoded.len());
    buf.push_str("Basic ");
    buf.push_str(&encoded);
}
