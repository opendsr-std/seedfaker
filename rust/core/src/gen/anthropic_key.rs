use crate::ctx::GenContext;

// Format: Anthropic API Key — https://docs.anthropic.com/en/api/getting-started
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(14 + 95);
    buf.push_str("sk-ant-api03-");
    ctx.rng.push_alnum(buf, 95);
}
