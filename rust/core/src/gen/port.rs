use std::fmt::Write;

use crate::ctx::GenContext;

/// Generate a realistic TCP/UDP port number.
///
/// Default: weighted distribution matching real-world traffic.
/// - Well-known services (80, 443, 22, etc.): ~15%
/// - Application ports (3000-9999): ~30%
/// - Ephemeral/dynamic (10000-65535): ~55%
///
/// Modifiers (standard networking terminology):
/// - `system`: 1-1023 — well-known/system ports (HTTP 80, SSH 22, HTTPS 443)
/// - `registered`: 1024-49151 — IANA registered (app servers, databases)
/// - `dynamic`: 49152-65535 — dynamic/ephemeral (OS-assigned client ports)
/// - `unprivileged`: 1024-65535 — all non-root ports
/// - `service`: weighted pick of common service ports (80, 443, 8080, 3306, 5432...)
pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    let port: u16 = match ctx.modifier {
        "system" | "service" => pick_well_known(&mut ctx.rng),
        "registered" => ctx.rng.range(1024, 49151) as u16,
        "dynamic" => ctx.rng.range(49152, 65535) as u16,
        "unprivileged" => ctx.rng.range(1024, 65535) as u16,
        _ => {
            // Default: weighted distribution
            let w = ctx.rng.urange(0, 99);
            match w {
                // 15% well-known services
                0..=14 => pick_well_known(&mut ctx.rng),
                // 30% application ports (3000-9999)
                15..=44 => ctx.rng.range(1024, 9999) as u16,
                // 55% ephemeral (10000-65535)
                _ => ctx.rng.range(10000, 65535) as u16,
            }
        }
    };
    f64::from(port)
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    let _ = write!(buf, "{}", v as u16);
}

/// Pick a well-known port weighted by real-world frequency.
fn pick_well_known(rng: &mut crate::rng::Rng) -> u16 {
    const PORTS: &[u16] = &[
        80, 80, 80, 443, 443, 443, 443, 443, // HTTP/HTTPS dominate
        22, 22, 22, // SSH
        53, 53, // DNS
        25, 587, 465, // SMTP
        3306, 5432, 6379, 27017, // MySQL, PG, Redis, Mongo
        8080, 8080, 8443, // HTTP alt
        3000, 5000, 8000, 9000, // Dev servers
        21, 23, 110, 143, 993, 995, // FTP, Telnet, IMAP, POP3
    ];
    PORTS[rng.urange(0, PORTS.len() - 1)]
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
