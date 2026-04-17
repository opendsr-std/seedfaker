use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let projects = ["FRONTEND", "BACKEND", "API", "MOBILE", "WORKER"];
    let p = projects[ctx.rng.urange(0, projects.len() - 1)];
    // p + - + 4 hex uppercase
    buf.reserve(p.len() + 1 + 4);
    buf.push_str(p);
    buf.push('-');
    // Need uppercase hex — generate lowercase and uppercase it
    let hex = ctx.rng.hex_str(4).to_uppercase();
    buf.push_str(&hex);
}
