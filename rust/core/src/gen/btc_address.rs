use crate::ctx::GenContext;

const BASE58: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BECH32: &[u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    match ctx.rng.urange(0, 2) {
        0 => {
            buf.reserve(34);
            buf.push('1');
            ctx.rng.push_charset(buf, BASE58, 33);
        }
        1 => {
            buf.reserve(34);
            buf.push('3');
            ctx.rng.push_charset(buf, BASE58, 33);
        }
        _ => {
            buf.reserve(42);
            buf.push_str("bc1q");
            ctx.rng.push_charset(buf, BECH32, 38);
        }
    }
}
