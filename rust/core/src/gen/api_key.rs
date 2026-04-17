use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    match ctx.rng.urange(0, 3) {
        0 => {
            buf.reserve(3 + 48);
            buf.push_str("sk-");
            ctx.rng.push_alnum(buf, 48);
        }
        1 => {
            buf.reserve(4 + 32);
            buf.push_str("key-");
            ctx.rng.push_alnum(buf, 32);
        }
        2 => {
            buf.reserve(4 + 40);
            buf.push_str("api_");
            ctx.rng.push_hex(buf, 40);
        }
        _ => {
            ctx.rng.push_hex(buf, 64);
        }
    }
}
