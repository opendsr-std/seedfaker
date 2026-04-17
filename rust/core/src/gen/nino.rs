use crate::ctx::GenContext;

// Format: UK NINO — https://www.gov.uk/national-insurance/your-national-insurance-number
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(13);
    ctx.rng.push_charset(buf, b"ABCEGHJKLMNPRSTWXYZ", 1); // p1
    ctx.rng.push_charset(buf, b"ABCEGHJKLMNPRSTWXYZ", 1); // p2
    let suffix = ctx.rng.charset_string(b"ABCD", 1); // must come before digits
    buf.push(' ');
    ctx.rng.push_digits(buf, 2);
    buf.push(' ');
    ctx.rng.push_digits(buf, 2);
    buf.push(' ');
    ctx.rng.push_digits(buf, 2);
    buf.push(' ');
    buf.push_str(&suffix);
}
