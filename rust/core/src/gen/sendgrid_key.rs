use crate::ctx::GenContext;

// Format: SendGrid API Key — https://www.twilio.com/docs/sendgrid/ui/account-and-settings/api-keys
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // "SG." (3) + alnum(22) + "." (1) + alnum(43) = 69
    buf.reserve(69);
    buf.push_str("SG.");
    ctx.rng.push_alnum(buf, 22);
    buf.push('.');
    ctx.rng.push_alnum(buf, 43);
}
