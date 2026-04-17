use std::fmt::Write;

use crate::ctx::GenContext;

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    let (min, max) = ctx.range.unwrap_or((0, 9999));
    let whole =
        if matches!(ctx.ordering, crate::field::Ordering::Asc | crate::field::Ordering::Desc) {
            let tag = super::helpers::handle::unique_tag(ctx.rng.record(), 0xF104);
            super::helpers::monotonic::monotonic_value(
                ctx.rng.record(),
                tag,
                min,
                max,
                ctx.ordering,
            )
        } else if let Some(z) = ctx.zipf {
            ctx.rng.zipf_range(min, max, z.s)
        } else {
            ctx.rng.range(min, max)
        };
    let frac = ctx.rng.range(0, 99);
    whole as f64 + frac as f64 / 100.0
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    let whole = v.trunc() as i64;
    let frac = ((v.fract() * 100.0) + 0.5) as i64;
    let _ = write!(buf, "{whole}.{frac:02}");
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
