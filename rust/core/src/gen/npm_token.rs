use crate::ctx::GenContext;

// Format: npm Access Token — https://docs.npmjs.com/about-access-tokens
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(4 + 36);
    buf.push_str("npm_");
    ctx.rng.push_alnum(buf, 36);
}
