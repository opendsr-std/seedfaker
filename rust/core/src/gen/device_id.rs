use crate::ctx::GenContext;

// Format: IDFA/GAID (UUID) — https://support.google.com/googleplay/android-developer/answer/6048248
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // 8-4-4-4-12 uppercase hex with dashes = 36 chars
    buf.reserve(36);
    // Need uppercase hex; push_hex gives lowercase, so use hex_str().to_uppercase()
    // to preserve RNG order we must call in the same sequence
    let a = ctx.rng.hex_str(8).to_uppercase();
    let b = ctx.rng.hex_str(4).to_uppercase();
    let c = ctx.rng.hex_str(4).to_uppercase();
    let d = ctx.rng.hex_str(4).to_uppercase();
    let e = ctx.rng.hex_str(12).to_uppercase();
    buf.push_str(&a);
    buf.push('-');
    buf.push_str(&b);
    buf.push('-');
    buf.push_str(&c);
    buf.push('-');
    buf.push_str(&d);
    buf.push('-');
    buf.push_str(&e);
}
