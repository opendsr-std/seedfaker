use crate::ctx::GenContext;

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    ctx.rng.record() as f64
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.push_str(itoa::Buffer::new().format(v as u64));
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
