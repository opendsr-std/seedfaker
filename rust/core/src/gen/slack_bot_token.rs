use crate::ctx::GenContext;

// Format: Slack Bot Token — https://api.slack.com/authentication/token-types
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // "xoxb-" (5) + digits(12) + "-" (1) + digits(12) + "-" (1) + alnum(24) = 55
    buf.reserve(55);
    buf.push_str("xoxb-");
    ctx.rng.push_digits(buf, 12);
    buf.push('-');
    ctx.rng.push_digits(buf, 12);
    buf.push('-');
    ctx.rng.push_alnum(buf, 24);
}
