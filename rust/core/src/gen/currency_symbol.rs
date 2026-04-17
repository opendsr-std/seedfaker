use crate::ctx::GenContext;

use super::amount::locale_currency_format;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let (sym, _, _) = locale_currency_format(loc.code);
    buf.push_str(sym);
}
