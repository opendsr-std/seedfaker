use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    const LETTERS: &[u8] = b"TRWAGMYFPDXBNJZSQVHLCKE";
    let arr = ["X", "Y", "Z"];
    let prefix = arr[ctx.rng.urange(0, arr.len() - 1)];
    let num = ctx.rng.range(1_000_000, 9_999_999);
    let full = match prefix {
        "X" => num,
        "Y" => num + 10_000_000,
        _ => num + 20_000_000,
    };
    let letter = LETTERS[(full % 23) as usize] as char;
    buf.reserve(10);
    buf.push_str(prefix);
    let _ = write!(buf, "{num}{letter}");
}
