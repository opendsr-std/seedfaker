use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let y = ctx.rng.range(2020, 2026);
    let arr = ["CV", "CR", "AP", "BK", "MC"];
    let t = arr[ctx.rng.urange(0, arr.len() - 1)];
    let n = ctx.rng.range(1000, 99999);
    let arr = ["SDNY", "NDCA", "CDCA", "EDPA", "NDIL", "SDTX"];
    let c = arr[ctx.rng.urange(0, arr.len() - 1)];
    // Case No. YYYY-TT-NNNNN (CCCC) — ~30 chars
    buf.reserve(32);
    let _ = write!(buf, "Case No. {y}-{t}-{n:05} ({c})");
}
