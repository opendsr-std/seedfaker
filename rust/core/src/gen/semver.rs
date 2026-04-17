use std::fmt::Write;

use crate::ctx::GenContext;

// Format: Semantic Versioning 2.0.0 — https://semver.org/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let major = ctx.rng.range(0, 9);
    let minor = ctx.rng.range(0, 30);
    let patch = ctx.rng.range(0, 99);
    buf.reserve(7);
    let _ = write!(buf, "{major}.{minor}.{patch}");
}
