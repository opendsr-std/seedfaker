use std::fmt::Write;

use crate::ctx::GenContext;

// Format: Poland PESEL — https://www.gov.pl/web/gov/uzyskaj-numer-pesel
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let y = ctx.rng.range(50, 99);
    let m = ctx.rng.range(1, 12);
    let d = ctx.rng.range(1, 28);
    buf.reserve(11);
    let _ = write!(buf, "{y:02}{m:02}{d:02}");
    ctx.rng.push_digits(buf, 5);
}
