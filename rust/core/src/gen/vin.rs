use crate::ctx::GenContext;

const VIN_CHARS: &[u8] = b"ABCDEFGHJKLMNPRSTUVWXYZ0123456789";

// Format: ISO 3779 (VIN) — https://www.iso.org/standard/52200.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_charset(buf, VIN_CHARS, 17);
}
