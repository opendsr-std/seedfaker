use crate::ctx::GenContext;

// Format: Slack User Token — https://api.slack.com/authentication/token-types
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // "xoxp-" (5) + digits(12) + "-" (1) + digits(12) + "-" (1) + digits(12) + "-" (1) + hex(32) = 76
    buf.reserve(76);
    buf.push_str("xoxp-");
    ctx.rng.push_digits(buf, 12);
    buf.push('-');
    ctx.rng.push_digits(buf, 12);
    buf.push('-');
    ctx.rng.push_digits(buf, 12);
    buf.push('-');
    ctx.rng.push_hex(buf, 32);
}
