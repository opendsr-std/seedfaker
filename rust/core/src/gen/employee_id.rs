use std::fmt::Write;

use super::helpers::handle::unique_tag;
use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let tag = unique_tag(ctx.rng.record(), 0xE1D0);
    buf.reserve(14);
    match ctx.rng.urange(0, 7) {
        0 | 1 => {
            let _ = write!(buf, "EMP-{}", tag % 9_000_000 + 1_000_000);
        }
        2 => {
            let _ = write!(buf, "E{}", tag % 9_000_000 + 1_000_000);
        }
        3 => {
            let _ = write!(buf, "{}", tag % 90_000_000 + 10_000_000);
        }
        4 => {
            let a = (tag % 26) as u8 + b'A';
            let b = ((tag / 26) % 26) as u8 + b'A';
            let n = (tag / 676) % 900_000 + 100_000;
            let _ = write!(buf, "{}{}-{n}", a as char, b as char);
        }
        5 => {
            let _ = write!(buf, "EMP{}", tag % 9_000_000 + 1_000_000);
        }
        6 => {
            let a = (tag % 26) as u8 + b'A';
            let n = (tag / 26) % 9_000_000 + 1_000_000;
            let _ = write!(buf, "{}-{n}", a as char);
        }
        _ => {
            let _ = write!(buf, "ID-{}", tag % 90_000_000 + 10_000_000);
        }
    }
}
