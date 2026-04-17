use crate::ctx::GenContext;

// Format: Google OAuth2 Token — https://developers.google.com/identity/protocols/oauth2
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(6 + 100);
    buf.push_str("ya29.a0");
    ctx.rng.push_alnum(buf, 100);
}
