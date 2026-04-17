use crate::ctx::GenContext;

/// Weighted gender distribution reflecting real-world demographics.
/// ~49% Male, ~49% Female, ~2% Non-binary (Gallup 2023 data).
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let w = ctx.rng.urange(0, 99);
    buf.push_str(match w {
        0..=48 => "Male",    // 49%
        49..=97 => "Female", // 49%
        _ => "Non-binary",   //  2%
    });
}
