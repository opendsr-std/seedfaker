use crate::ctx::GenContext;

use super::helpers::words::words_for_locale;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let words = words_for_locale(loc.code);
    let n = ctx.rng.urange(3, 12);
    let joined = ctx.rng.sample(words, n).join(" ");
    let mut chars = joined.chars();
    match chars.next() {
        None => {}
        Some(c) => {
            let upper: String = c.to_uppercase().collect();
            buf.push_str(&upper);
            buf.push_str(chars.as_str());
        }
    }
}
