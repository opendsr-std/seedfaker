use crate::ctx::GenContext;

// Format: Stripe API Key — https://docs.stripe.com/keys
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let arr = ["live", "test"];
    let mode = arr[ctx.rng.urange(0, arr.len() - 1)];
    let len = ctx.rng.urange(24, 40);
    // "sk_" (3) + mode(4) + "_" (1) + alnum(len)
    buf.reserve(8 + len);
    buf.push_str("sk_");
    buf.push_str(mode);
    buf.push('_');
    ctx.rng.push_alnum(buf, len);
}
