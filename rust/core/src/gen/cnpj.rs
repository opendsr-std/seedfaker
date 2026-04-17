use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let digits = ctx.rng.digits(14);
    if ctx.modifier == "plain" {
        buf.push_str(&digits);
    } else {
        buf.reserve(18);
        buf.push_str(&digits[0..2]);
        buf.push('.');
        buf.push_str(&digits[2..5]);
        buf.push('.');
        buf.push_str(&digits[5..8]);
        buf.push('/');
        buf.push_str(&digits[8..12]);
        buf.push('-');
        buf.push_str(&digits[12..14]);
    }
}
