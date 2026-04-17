use std::fmt::Write;

use crate::ctx::GenContext;

use super::helpers::charsets::UPPER;

// Format: Mexico CURP — https://www.gob.mx/curp/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(18);
    ctx.rng.push_upper(buf, 4); // letters
    let _ = write!(
        buf,
        "{:02}{:02}{:02}",
        ctx.rng.range(70, 99),
        ctx.rng.range(1, 12),
        ctx.rng.range(1, 28)
    ); // date
    ctx.rng.push_charset(buf, b"HM", 1); // gender
    ctx.rng.push_upper(buf, 2); // state
    ctx.rng.push_upper(buf, 3); // consonants
    let _ = write!(buf, "{}", ctx.rng.range(0, 9));
    ctx.rng.push_charset(buf, UPPER, 1);
}
