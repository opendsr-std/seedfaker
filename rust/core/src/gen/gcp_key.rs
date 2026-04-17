use crate::ctx::GenContext;

// Format: Google Cloud API Key — https://cloud.google.com/docs/authentication/api-keys
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(4 + 35);
    buf.push_str("AIza");
    ctx.rng.push_alnum(buf, 35);
}
