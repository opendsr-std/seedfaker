use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let projects = ["PROJ", "ENG", "DATA", "SEC", "OPS", "INFRA", "PLAT", "CORE", "API", "WEB"];
    let p = projects[ctx.rng.urange(0, projects.len() - 1)];
    let n = ctx.rng.range(100, 99999);
    buf.reserve(p.len() + 1 + 5);
    buf.push_str(p);
    let _ = write!(buf, "-{n}");
}
