use std::fmt::Write;

use crate::ctx::GenContext;

/// Tiered distribution for realistic integer values without explicit range.
/// Biased toward small numbers — matches real-world ID sequences, counts, quantities.
///
/// | Range       | Probability | Use case              |
/// |-------------|-------------|-----------------------|
/// | 1-100       | 30%         | counts, quantities    |
/// | 100-1000    | 25%         | IDs, scores           |
/// | 1K-10K      | 20%         | employee IDs, codes   |
/// | 10K-100K    | 15%         | order numbers         |
/// | 100K-999K   | 10%         | large sequence IDs    |
fn tiered_integer(rng: &mut crate::rng::Rng) -> i64 {
    let w = rng.urange(0, 99);
    match w {
        0..=29 => rng.range(1, 100),
        30..=54 => rng.range(100, 1000),
        55..=74 => rng.range(1000, 10_000),
        75..=89 => rng.range(10_000, 100_000),
        _ => rng.range(100_000, 999_999),
    }
}

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    let value =
        if matches!(ctx.ordering, crate::field::Ordering::Asc | crate::field::Ordering::Desc) {
            let (min, max) = ctx.range.unwrap_or((1, 999_999));
            let tag = super::helpers::handle::unique_tag(ctx.rng.record(), 0x1147);
            super::helpers::monotonic::monotonic_value(
                ctx.rng.record(),
                tag,
                min,
                max,
                ctx.ordering,
            )
        } else if let Some((min, max)) = ctx.range {
            if let Some(z) = ctx.zipf {
                ctx.rng.zipf_range(min, max, z.s)
            } else {
                ctx.rng.range(min, max)
            }
        } else {
            tiered_integer(&mut ctx.rng)
        };
    value as f64
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    let _ = write!(buf, "{}", v as i64);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
