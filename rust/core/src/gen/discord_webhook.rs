use crate::ctx::GenContext;

// Format: Discord Webhook — https://discord.com/developers/docs/resources/webhook
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // "https://discord.com/api/webhooks/" (33) + digits(18) + "/" (1) + alnum(68) = 120
    buf.reserve(120);
    buf.push_str("https://discord.com/api/webhooks/");
    ctx.rng.push_digits(buf, 18);
    buf.push('/');
    ctx.rng.push_alnum(buf, 68);
}
