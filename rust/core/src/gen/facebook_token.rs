use crate::ctx::GenContext;

// Format: Facebook Access Token — https://developers.facebook.com/docs/facebook-login/guides/access-tokens/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(14 + 80);
    buf.push_str("EAAGm0PX4ZCps");
    ctx.rng.push_alnum(buf, 80);
}
