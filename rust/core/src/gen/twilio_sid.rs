use crate::ctx::GenContext;

// Format: Twilio Account SID — https://www.twilio.com/docs/iam/api/account
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.push_str("AC");
    ctx.rng.push_hex(buf, 32);
}
