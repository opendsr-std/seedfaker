use std::fmt::Write;

use crate::ctx::GenContext;

/// Generate a realistic response latency in milliseconds.
///
/// Default: log-normal-like distribution matching real-world API/web latency.
/// - 1-50ms (40%): fast cache hits, simple queries
/// - 50-200ms (30%): typical API calls, DB reads
/// - 200-1000ms (20%): complex queries, external calls
/// - 1-5s (8%): slow operations, timeouts
/// - 5-30s (2%): near-timeout, batch operations
///
/// Modifiers:
/// - `fast`: 1-100ms only (cache, CDN, fast APIs)
/// - `slow`: 500ms-30s only (slow queries, timeouts)
/// - `seconds`: output in seconds with 3 decimal places (0.042, 1.337)
pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    let ms: i64 = match ctx.modifier {
        "fast" => {
            let w = ctx.rng.urange(0, 99);
            match w {
                0..=59 => ctx.rng.range(1, 20),
                60..=89 => ctx.rng.range(20, 50),
                _ => ctx.rng.range(50, 100),
            }
        }
        "slow" => {
            let w = ctx.rng.urange(0, 99);
            match w {
                0..=39 => ctx.rng.range(500, 2000),
                40..=69 => ctx.rng.range(2000, 5000),
                70..=89 => ctx.rng.range(5000, 15000),
                _ => ctx.rng.range(15000, 30000),
            }
        }
        _ => tiered_latency(&mut ctx.rng),
    };
    ms as f64
}

pub fn fmt(v: f64, ctx: &mut GenContext<'_>, buf: &mut String) {
    let ms = v as i64;
    if ctx.modifier == "seconds" {
        let secs = ms / 1000;
        let frac = ms % 1000;
        let _ = write!(buf, "{secs}.{frac:03}");
    } else {
        let _ = write!(buf, "{ms}");
    }
}

fn tiered_latency(rng: &mut crate::rng::Rng) -> i64 {
    let w = rng.urange(0, 99);
    match w {
        0..=39 => rng.range(1, 50),       // 40% fast
        40..=69 => rng.range(50, 200),    // 30% normal
        70..=89 => rng.range(200, 1000),  // 20% slow
        90..=97 => rng.range(1000, 5000), //  8% very slow
        _ => rng.range(5000, 30000),      //  2% timeout-zone
    }
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
