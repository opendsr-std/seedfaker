use std::fmt::Write;

use crate::ctx::GenContext;

// Format: ICD-10 (WHO) — https://icd.who.int/browse10/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let letters = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "R", "S", "T",
        "Z",
    ];
    let l = letters[ctx.rng.urange(0, letters.len() - 1)];
    let n = ctx.rng.range(0, 99);
    if ctx.rng.maybe(0.7) {
        let d = ctx.rng.range(0, 9);
        buf.reserve(6);
        let _ = write!(buf, "{l}{n:02}.{d}");
    } else {
        buf.reserve(3);
        let _ = write!(buf, "{l}{n:02}");
    }
}
