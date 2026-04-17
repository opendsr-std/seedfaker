use crate::ctx::GenContext;

// Format: Slack Webhook — https://api.slack.com/messaging/webhooks
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let a = ctx.rng.alnum(9).to_uppercase();
    let b = ctx.rng.alnum(9).to_uppercase();
    // "https://hooks.slack.com/services/" (34) + a(9) + "/" (1) + b(9) + "/" (1) + alnum(24) = 78
    buf.reserve(78);
    buf.push_str("https://hooks.slack.com/services/");
    buf.push_str(&a);
    buf.push('/');
    buf.push_str(&b);
    buf.push('/');
    ctx.rng.push_alnum(buf, 24);
}
