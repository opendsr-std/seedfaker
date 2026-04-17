use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let hosts = ["app", "api", "db", "cache", "worker", "proxy", "auth", "search"];
    let envs = ["prod", "staging", "dev"];
    let host = hosts[ctx.rng.urange(0, hosts.len() - 1)];
    let env = envs[ctx.rng.urange(0, envs.len() - 1)];
    let a = ctx.rng.range(0, 255);
    let b = ctx.rng.range(0, 255);
    let c = ctx.rng.range(1, 254);
    // host.env.internal A 10.a.b.c
    buf.reserve(host.len() + 1 + env.len() + 24);
    let _ = write!(buf, "{host}.{env}.internal A 10.{a}.{b}.{c}");
}
