// RFC 3986 — https://www.rfc-editor.org/rfc/rfc3986

use std::fmt::Write;

use crate::ctx::GenContext;

const DOMAINS: &[&str] = &[
    "api.example.com",
    "app.internal.io",
    "service.prod.net",
    "data.platform.dev",
    "cdn.assets.io",
    "portal.corp.net",
    "gateway.cloud.dev",
    "auth.identity.io",
];

const PATHS: &[&str] = &[
    "users",
    "orders",
    "products",
    "events",
    "metrics",
    "reports",
    "auth/token",
    "search",
    "upload",
    "export",
    "webhooks",
];

const VERSIONS: &[&str] = &["v1", "v2", "v3"];

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let domain = DOMAINS[ctx.rng.urange(0, DOMAINS.len() - 1)];
    let ver = VERSIONS[ctx.rng.urange(0, VERSIONS.len() - 1)];
    let path = PATHS[ctx.rng.urange(0, PATHS.len() - 1)];

    match ctx.modifier {
        "http" => {
            buf.reserve(7 + domain.len() + 1 + ver.len() + 1 + path.len());
            buf.push_str("http://");
            buf.push_str(domain);
            buf.push('/');
            buf.push_str(ver);
            buf.push('/');
            buf.push_str(path);
        }
        "ftp" => {
            let dirs = ["pub", "data", "incoming", "archive", "releases"];
            let dir = dirs[ctx.rng.urange(0, dirs.len() - 1)];
            buf.reserve(6 + domain.len() + 1 + dir.len() + 1 + 8 + 7);
            buf.push_str("ftp://");
            buf.push_str(domain);
            buf.push('/');
            buf.push_str(dir);
            buf.push('/');
            ctx.rng.push_alnum(buf, 8);
            buf.push('.');
            let arr = ["tar.gz", "zip", "csv", "dat"];
            let ext = arr[ctx.rng.urange(0, arr.len() - 1)];
            buf.push_str(ext);
        }
        "ws" => {
            buf.reserve(5 + domain.len() + 1 + ver.len() + 1 + path.len());
            buf.push_str("ws://");
            buf.push_str(domain);
            buf.push('/');
            buf.push_str(ver);
            buf.push('/');
            buf.push_str(path);
        }
        "wss" => {
            buf.reserve(6 + domain.len() + 1 + ver.len() + 1 + path.len());
            buf.push_str("wss://");
            buf.push_str(domain);
            buf.push('/');
            buf.push_str(ver);
            buf.push('/');
            buf.push_str(path);
        }
        "ssh" => {
            let users = ["deploy", "admin", "ci", "root", "git"];
            let user = users[ctx.rng.urange(0, users.len() - 1)];
            let port = ctx.rng.range(22, 2222);
            buf.reserve(6 + user.len() + 1 + domain.len() + 6);
            let _ = write!(buf, "ssh://{user}@{domain}:{port}");
        }
        _ => {
            buf.reserve(8 + domain.len() + 1 + ver.len() + 1 + path.len());
            buf.push_str("https://");
            buf.push_str(domain);
            buf.push('/');
            buf.push_str(ver);
            buf.push('/');
            buf.push_str(path);
        }
    }
}
