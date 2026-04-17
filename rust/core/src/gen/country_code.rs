use crate::ctx::GenContext;

use super::helpers::locale_to_country_code;

// ISO 3166-1 alpha-2 / alpha-3 / numeric
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match ctx.modifier {
        "alpha3" => super::country_code_3::gen(ctx, buf),
        "numeric" => super::country_numeric::gen(ctx, buf),
        _ => buf.push_str(locale_to_country_code(loc.code)),
    }
}
