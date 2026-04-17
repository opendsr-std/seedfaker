use crate::ctx::GenContext;

use super::helpers::locale_to_country;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    buf.push_str(locale_to_country(loc.code));
}
