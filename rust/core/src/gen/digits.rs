use std::fmt::Write;

use crate::ctx::GenContext;
use crate::field::Ordering;

use super::helpers::charsets::primitive_len;
use super::helpers::handle::unique_tag;
use super::helpers::monotonic::monotonic_value;

/// Generate a string of digits with optional zero-padding, range, and ordering.
///
/// - `digits:4` → 4 random digits: `0469`
/// - `digits:6:100..500` → 6-digit padded, range 100-500: `000342`
/// - `digits:4:asc` → `0001, 0002, 0003, ...`
/// - `digits` → random length 8-16 digits
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let width = primitive_len(ctx.modifier, &mut ctx.rng);

    if ctx.range.is_some() || matches!(ctx.ordering, Ordering::Asc | Ordering::Desc) {
        // Numeric mode: generate value, zero-pad to width
        let max_for_width = 10_i64.saturating_pow(width as u32) - 1;
        let (min, max) = ctx.range.unwrap_or((0, max_for_width));
        let value = if matches!(ctx.ordering, Ordering::Asc | Ordering::Desc) {
            let tag = unique_tag(ctx.rng.record(), 0xD161);
            monotonic_value(ctx.rng.record(), tag, min, max, ctx.ordering)
        } else {
            ctx.rng.range(min, max)
        };
        let _ = write!(buf, "{value:0>width$}");
    } else {
        // String mode: random digit characters (current behavior)
        ctx.rng.push_digits(buf, width);
    }
}
