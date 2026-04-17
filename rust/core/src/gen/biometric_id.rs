use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let arr = ["FACE", "FP", "IRIS", "VOICE"];
    let m = arr[ctx.rng.urange(0, arr.len() - 1)];
    // BIO- + m + - + 16 alnum uppercase
    let alnum = ctx.rng.alnum(16).to_uppercase();
    buf.reserve(4 + m.len() + 1 + 16);
    buf.push_str("BIO-");
    buf.push_str(m);
    buf.push('-');
    buf.push_str(&alnum);
}
