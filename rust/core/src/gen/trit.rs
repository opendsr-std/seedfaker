use crate::ctx::GenContext;

/// Ternary digit: -1, 0, or 1. Equal probability.
pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    match ctx.rng.urange(0, 2) {
        0 => -1.0,
        1 => 0.0,
        _ => 1.0,
    }
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.push_str(match v as i64 {
        -1 => "-1",
        0 => "0",
        _ => "1",
    });
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
