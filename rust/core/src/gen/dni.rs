use std::fmt::Write;

use crate::ctx::GenContext;

// Format: Spain DNI — https://www.interior.gob.es/opencms/es/servicios-al-ciudadano/tramites-y-gestiones/dni/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    const LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";
    let num = ctx.rng.range(10_000_000, 99_999_999);
    let letter = LETTERS[(num % 23) as usize] as char;
    buf.reserve(9);
    let _ = write!(buf, "{num}{letter}");
}
