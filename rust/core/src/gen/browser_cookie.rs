use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // "session=" (8) + hex(32) + "; _ga=GA1.2." (12) + digits(~9) + "." + digits(10) +
    // "; csrftoken=" (12) + hex(16) ~ 100
    buf.reserve(104);
    buf.push_str("session=");
    ctx.rng.push_hex(buf, 32);
    let ga_a = ctx.rng.range(100_000_000, 999_999_999);
    let ga_b = ctx.rng.range(1_700_000_000, 1_710_000_000);
    let _ = write!(buf, "; _ga=GA1.2.{ga_a}.{ga_b}; csrftoken=");
    ctx.rng.push_hex(buf, 16);
}
