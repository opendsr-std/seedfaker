use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let services = [
        ("jira", "browse", &["ENG", "SEC", "DATA", "OPS", "PLAT"] as &[&str]),
        ("confluence", "display", &["Engineering", "Security", "Platform"]),
        ("grafana", "d", &["api-latency", "error-rates", "system-health"]),
        ("gitlab", "merge_requests", &["api", "frontend", "infra"]),
    ];
    let (svc, path, items) = ctx.rng.choice(&services);
    let item = items[ctx.rng.urange(0, items.len() - 1)];
    let id = ctx.rng.range(100, 99999);
    // https:// + svc + .corp.internal/ + path + / + item + - + id(up to 5 digits)
    buf.reserve(8 + svc.len() + 15 + path.len() + 1 + item.len() + 6);
    let _ = write!(buf, "https://{svc}.corp.internal/{path}/{item}-{id}");
}
