use crate::ctx::GenContext;

pub fn push_pad2(buf: &mut String, v: i64) {
    if v < 10 {
        buf.push('0');
    }
    buf.push_str(itoa::Buffer::new().format(v));
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // since/until are epoch seconds — extract year range (until is exclusive upper bound)
    let (ef, et) = ctx.range.unwrap_or((ctx.since, ctx.until));
    let yf = crate::temporal::epoch_to_year(ef);
    let yt = crate::temporal::epoch_to_year(et.saturating_sub(1));
    let y = ctx.rng.range(yf, yt.max(yf));
    let m = ctx.rng.range(1, 12);
    let d = ctx.rng.range(1, 28);
    ctx.numeric = Some(crate::temporal::date_to_epoch(y, m, d, 0, 0, 0) as f64);
    let mut ib = itoa::Buffer::new();
    match ctx.modifier {
        "us" => {
            push_pad2(buf, m);
            buf.push('/');
            push_pad2(buf, d);
            buf.push('/');
            buf.push_str(ib.format(y));
        }
        "eu" => {
            push_pad2(buf, d);
            buf.push('.');
            push_pad2(buf, m);
            buf.push('.');
            buf.push_str(ib.format(y));
        }
        _ => {
            buf.push_str(ib.format(y));
            buf.push('-');
            push_pad2(buf, m);
            buf.push('-');
            push_pad2(buf, d);
        }
    }
}
