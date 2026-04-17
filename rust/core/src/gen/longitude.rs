use crate::ctx::GenContext;

use super::latitude::push_f4;

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    if let Some(id) = ctx.identity {
        let jitter = ctx.rng.range(-900, 900) as f64 / 10000.0;
        return id.lon + jitter;
    }
    let loc = ctx.pick_locale();
    let (base_lon, spread) = match loc.code {
        "en" => (-95.0, 20.0),
        "de" => (10.5, 3.5),
        "fr" => (2.5, 4.0),
        "it" => (12.5, 4.0),
        "es" => (-3.5, 5.0),
        "nl" => (5.3, 1.5),
        "pt" => (-8.0, 1.5),
        "pt-br" => (-50.0, 15.0),
        "ja" => (138.0, 4.0),
        "zh" => (110.0, 15.0),
        "hi" => (78.0, 8.0),
        "vi" => (107.0, 3.0),
        "tr" => (33.0, 7.0),
        "se" => (16.0, 5.0),
        "da" => (10.0, 2.0),
        "no" => (10.0, 8.0),
        "fi" => (26.0, 4.0),
        "pl" => (19.5, 3.0),
        "uk" => (32.0, 6.0),
        "ar" => (-64.0, 6.0),
        "mx" => (-100.0, 10.0),
        _ => (0.0, 90.0),
    };
    base_lon + (ctx.rng.range(-1000, 1000) as f64 / 1000.0) * spread
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    push_f4(buf, v);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
