use crate::ctx::GenContext;

pub fn push_f4(buf: &mut String, v: f64) {
    if v < 0.0 {
        buf.push('-');
    }
    let abs = if v < 0.0 { -v } else { v };
    let whole = abs as u64;
    let frac = ((abs - whole as f64) * 10000.0 + 0.5) as u64;
    let mut ib = itoa::Buffer::new();
    buf.push_str(ib.format(whole));
    buf.push('.');
    let mut ib2 = itoa::Buffer::new();
    let fs = ib2.format(frac);
    for _ in 0..(4usize.saturating_sub(fs.len())) {
        buf.push('0');
    }
    buf.push_str(fs);
}

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    if let Some(id) = ctx.identity {
        let jitter = ctx.rng.range(-900, 900) as f64 / 10000.0;
        return id.lat + jitter;
    }
    let loc = ctx.pick_locale();
    let (base_lat, spread) = match loc.code {
        "en" => (38.0, 10.0),
        "de" => (51.0, 3.0),
        "fr" => (46.5, 3.5),
        "it" => (42.0, 4.0),
        "es" => (40.0, 4.0),
        "nl" => (52.2, 1.0),
        "pt" => (39.5, 2.5),
        "pt-br" => (-15.0, 15.0),
        "ja" => (36.0, 4.0),
        "zh" => (35.0, 10.0),
        "hi" => (22.0, 8.0),
        "vi" => (16.0, 6.0),
        "tr" => (39.5, 3.0),
        "se" => (62.0, 5.0),
        "da" => (56.0, 1.5),
        "no" => (62.0, 6.0),
        "fi" => (63.0, 4.0),
        "pl" => (52.0, 2.0),
        "uk" => (49.0, 3.0),
        "ar" => (-34.0, 5.0),
        "mx" => (23.0, 7.0),
        _ => (40.0, 20.0),
    };
    base_lat + (ctx.rng.range(-1000, 1000) as f64 / 1000.0) * spread
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    push_f4(buf, v);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
