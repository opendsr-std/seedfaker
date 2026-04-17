use std::fmt::Write;

use crate::ctx::GenContext;

// Format: France NIR (INSEE) — https://www.insee.fr/fr/information/6522257
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let sex = ctx.rng.range(1, 2);
    buf.reserve(19);
    let _ = write!(buf, "{sex} ");
    ctx.rng.push_digits(buf, 2);
    buf.push(' ');
    ctx.rng.push_digits(buf, 2);
    buf.push(' ');
    ctx.rng.push_digits(buf, 2);
    buf.push(' ');
    ctx.rng.push_digits(buf, 3);
    buf.push(' ');
    ctx.rng.push_digits(buf, 3);
}
