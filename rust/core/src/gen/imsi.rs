use crate::ctx::GenContext;

// Format: ITU-T E.212 (IMSI) — https://www.itu.int/rec/T-REC-E.212
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let arr = ["310", "311", "234", "262", "440", "208", "505"];
    let p = arr[ctx.rng.urange(0, arr.len() - 1)];
    buf.reserve(15);
    buf.push_str(p);
    ctx.rng.push_digits(buf, 12);
}
