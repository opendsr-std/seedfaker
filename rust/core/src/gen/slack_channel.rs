use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let names = [
        "general",
        "engineering",
        "random",
        "alerts",
        "deployments",
        "incidents",
        "security",
        "data-eng",
        "frontend",
        "backend",
        "devops",
        "support",
        "product",
        "design",
        "qa",
    ];
    let name = names[ctx.rng.urange(0, names.len() - 1)];
    buf.reserve(1 + name.len());
    buf.push('#');
    buf.push_str(name);
}
