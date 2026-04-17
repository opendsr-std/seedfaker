use crate::ctx::GenContext;

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    match ctx.modifier {
        "sign" => {
            if ctx.rng.maybe(0.5) {
                1.0
            } else {
                -1.0
            }
        }
        _ => {
            if ctx.rng.maybe(0.5) {
                1.0
            } else {
                0.0
            }
        }
    }
}

pub fn fmt(v: f64, _ctx: &mut GenContext<'_>, buf: &mut String) {
    let i = v as i64;
    match i {
        1 => buf.push('1'),
        0 => buf.push('0'),
        _ => buf.push_str("-1"),
    }
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
