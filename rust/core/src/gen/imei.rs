use crate::ctx::GenContext;

// Format: 3GPP TS 23.003 (IMEI) — https://www.3gpp.org/dynareport/23003.htm
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_digits(buf, 15);
}
