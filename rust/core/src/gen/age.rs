use std::fmt::Write;

use crate::ctx::GenContext;

/// Generate a realistic age using the same demographic pyramid as birthdate.
///
/// With identity (ctx strict): derived from `birth_year` — consistent with birthdate field.
/// Without identity: weighted random using the same demographic pyramid.
///
/// Uses `until` as reference "current year": `age = until - birth_year - 1`.
/// Birthday has NOT yet occurred (deterministic rule, no system clock dependency).
pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    let ref_year = crate::temporal::epoch_to_year(ctx.until.saturating_sub(1));
    let age = if let Some(id) = ctx.identity {
        (ref_year - id.birth_year - 1).clamp(0, 120)
    } else if let Some((min, max)) = ctx.range {
        ctx.rng.range(min, max)
    } else {
        let yf = crate::temporal::epoch_to_year(ctx.since);
        let birth = crate::ctx::weighted_birth_year(&mut ctx.rng, yf, ref_year);
        (ref_year - birth - 1).clamp(0, 120)
    };
    age as f64
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    let _ = write!(buf, "{}", v as i64);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
