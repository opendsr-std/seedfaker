use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let digits = ctx.rng.digits(11);
    if ctx.modifier == "plain" {
        buf.push_str(&digits);
    } else {
        buf.reserve(14);
        buf.push_str(&digits[0..3]);
        buf.push('.');
        buf.push_str(&digits[3..6]);
        buf.push('.');
        buf.push_str(&digits[6..9]);
        buf.push('-');
        buf.push_str(&digits[9..11]);
    }
}
