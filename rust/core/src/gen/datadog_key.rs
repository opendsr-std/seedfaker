use crate::ctx::GenContext;

// Format: Datadog API Key — https://docs.datadoghq.com/account_management/api-app-keys/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_hex(buf, 32);
}
