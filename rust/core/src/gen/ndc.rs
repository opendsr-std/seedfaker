use crate::ctx::GenContext;

// Format: FDA NDC — https://www.fda.gov/drugs/drug-approvals-and-databases/national-drug-code-directory
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // 5d + - + 3d + - + 2d = 12
    buf.reserve(12);
    ctx.rng.push_digits(buf, 5);
    buf.push('-');
    ctx.rng.push_digits(buf, 3);
    buf.push('-');
    ctx.rng.push_digits(buf, 2);
}
