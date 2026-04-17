use std::fmt::Write;

use crate::ctx::GenContext;

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    let statuses =
        [200i64, 200, 200, 200, 201, 204, 301, 302, 400, 401, 403, 404, 404, 500, 502, 503];
    *ctx.rng.choice(&statuses) as f64
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    let _ = write!(buf, "{}", v as i64);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
