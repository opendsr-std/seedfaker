use std::fmt::Write;

use crate::ctx::GenContext;

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    ctx.rng.range(1, 6) as f64
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    let _ = write!(buf, "{}", v as i64);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
