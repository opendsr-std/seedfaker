use crate::ctx::GenContext;

use super::helpers::locale_to_country_code;

// ISO 9362 (SWIFT/BIC) — https://www.iso.org/standard/60390.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let cc = locale_to_country_code(loc.code);
    // 4upper + 2cc + 2upper_digit + optional 3upper_digit = max 11
    buf.reserve(11);
    ctx.rng.push_upper(buf, 4);
    buf.push_str(cc);
    ctx.rng.push_upper_digit(buf, 2);
    if ctx.rng.maybe(0.55) {
        ctx.rng.push_upper_digit(buf, 3);
    }
}
