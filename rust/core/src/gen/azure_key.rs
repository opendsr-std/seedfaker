use crate::ctx::GenContext;

use super::helpers::charsets::B64_CHARSET;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_charset(buf, B64_CHARSET, 44);
}
