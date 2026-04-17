use crate::ctx::GenContext;

// Format: Telegram Bot Token — https://core.telegram.org/bots/api#authorizing-your-bot
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let len = ctx.rng.urange(9, 10);
    // digits(9-10) + ":" (1) + alnum(35) = 45-46
    buf.reserve(1 + len + 35);
    ctx.rng.push_digits(buf, len);
    buf.push(':');
    ctx.rng.push_alnum(buf, 35);
}
