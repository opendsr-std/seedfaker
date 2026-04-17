use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let carriers = ["BCBS", "UHC", "AETNA", "CIGNA", "HUM"];
    let c = carriers[ctx.rng.urange(0, carriers.len() - 1)];
    let n = ctx.rng.range(100_000_000, 999_999_999);
    buf.reserve(c.len() + 1 + 9);
    buf.push_str(c);
    let _ = write!(buf, "-{n}");
}
