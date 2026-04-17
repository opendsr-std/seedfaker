use crate::ctx::GenContext;

use super::helpers::words::words_for_locale;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let words = words_for_locale(loc.code);
    buf.push_str(words[ctx.rng.urange(0, words.len() - 1)]);
}
