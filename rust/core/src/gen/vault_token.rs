use crate::ctx::GenContext;

// Format: HashiCorp Vault Token — https://developer.hashicorp.com/vault/docs/concepts/tokens
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if ctx.rng.maybe(0.7) {
        buf.reserve(4 + 24);
        buf.push_str("hvs.");
        ctx.rng.push_alnum(buf, 24);
    } else {
        buf.reserve(2 + 24);
        buf.push_str("s.");
        ctx.rng.push_alnum(buf, 24);
    }
}
