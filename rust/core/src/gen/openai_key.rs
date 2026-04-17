use crate::ctx::GenContext;

// Format: OpenAI API Key — https://platform.openai.com/docs/api-reference/authentication
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if ctx.rng.maybe(0.7) {
        buf.reserve(8 + 48);
        buf.push_str("sk-proj-");
        ctx.rng.push_alnum(buf, 48);
    } else {
        buf.reserve(3 + 48);
        buf.push_str("sk-");
        ctx.rng.push_alnum(buf, 48);
    }
}
