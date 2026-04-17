use crate::ctx::GenContext;

// Format: US EIN — https://www.irs.gov/businesses/small-businesses-self-employed/employer-identification-number
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // 2digits + - + 7digits = 10
    buf.reserve(10);
    ctx.rng.push_digits(buf, 2);
    buf.push('-');
    ctx.rng.push_digits(buf, 7);
}
