use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let registries = ["docker.io", "ghcr.io", "gcr.io", "registry.gitlab.com", "quay.io"];
    let reg = registries[ctx.rng.urange(0, registries.len() - 1)];
    let orgs = ["acme", "myorg", "platform", "infra", "data"];
    let org = orgs[ctx.rng.urange(0, orgs.len() - 1)];
    let apps = ["api", "web", "worker", "gateway", "auth", "scheduler", "proxy"];
    let app = apps[ctx.rng.urange(0, apps.len() - 1)];
    let major = ctx.rng.range(0, 5);
    let minor = ctx.rng.range(0, 30);
    let patch = ctx.rng.range(0, 99);
    // reg/org/app:vM.MM.PP
    buf.reserve(reg.len() + 1 + org.len() + 1 + app.len() + 10);
    buf.push_str(reg);
    buf.push('/');
    buf.push_str(org);
    buf.push('/');
    buf.push_str(app);
    let _ = write!(buf, ":v{major}.{minor}.{patch}");
}
