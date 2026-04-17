use crate::ctx::GenContext;

// Format: EU VAT — https://taxation-customs.ec.europa.eu/vies_en
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "fr" | "be" => {
            buf.reserve(13);
            buf.push_str("FR");
            ctx.rng.push_upper(buf, 2);
            ctx.rng.push_digits(buf, 9);
        }
        "en" | "ie" => {
            buf.reserve(14);
            buf.push_str("GB");
            ctx.rng.push_digits(buf, 3);
            buf.push(' ');
            ctx.rng.push_digits(buf, 4);
            buf.push(' ');
            ctx.rng.push_digits(buf, 2);
        }
        "it" => {
            buf.reserve(13);
            buf.push_str("IT");
            ctx.rng.push_digits(buf, 11);
        }
        "es" => {
            buf.reserve(11);
            buf.push_str("ES");
            ctx.rng.push_upper(buf, 1);
            ctx.rng.push_digits(buf, 7);
            ctx.rng.push_upper(buf, 1);
        }
        "nl" => {
            buf.reserve(14);
            buf.push_str("NL");
            ctx.rng.push_digits(buf, 9);
            buf.push('B');
            ctx.rng.push_digits(buf, 2);
        }
        "pl" => {
            buf.reserve(12);
            buf.push_str("PL");
            ctx.rng.push_digits(buf, 10);
        }
        "se" => {
            buf.reserve(14);
            buf.push_str("SE");
            ctx.rng.push_digits(buf, 10);
            buf.push_str("01");
        }
        "da" => {
            buf.reserve(10);
            buf.push_str("DK");
            ctx.rng.push_digits(buf, 8);
        }
        "fi" => {
            buf.reserve(10);
            buf.push_str("FI");
            ctx.rng.push_digits(buf, 8);
        }
        "no" => {
            buf.reserve(11);
            buf.push_str("NO");
            ctx.rng.push_digits(buf, 9);
        }
        "el" => {
            buf.reserve(11);
            buf.push_str("EL");
            ctx.rng.push_digits(buf, 9);
        }
        "pt" | "pt-br" => {
            buf.reserve(11);
            buf.push_str("PT");
            ctx.rng.push_digits(buf, 9);
        }
        "ro" => {
            let n = ctx.rng.urange(2, 10);
            buf.reserve(2 + n);
            buf.push_str("RO");
            ctx.rng.push_digits(buf, n);
        }
        "hr" => {
            buf.reserve(13);
            buf.push_str("HR");
            ctx.rng.push_digits(buf, 11);
        }
        "bg" => {
            let n = ctx.rng.urange(9, 10);
            buf.reserve(2 + n);
            buf.push_str("BG");
            ctx.rng.push_digits(buf, n);
        }
        "hu" => {
            buf.reserve(10);
            buf.push_str("HU");
            ctx.rng.push_digits(buf, 8);
        }
        "cs" => {
            let n = ctx.rng.urange(8, 10);
            buf.reserve(2 + n);
            buf.push_str("CZ");
            ctx.rng.push_digits(buf, n);
        }
        "sk" => {
            buf.reserve(12);
            buf.push_str("SK");
            ctx.rng.push_digits(buf, 10);
        }
        "sl" => {
            buf.reserve(10);
            buf.push_str("SI");
            ctx.rng.push_digits(buf, 8);
        }
        "et" => {
            buf.reserve(11);
            buf.push_str("EE");
            ctx.rng.push_digits(buf, 9);
        }
        "lt" => {
            let n = ctx.rng.urange(9, 12);
            buf.reserve(2 + n);
            buf.push_str("LT");
            ctx.rng.push_digits(buf, n);
        }
        "lv" => {
            buf.reserve(13);
            buf.push_str("LV");
            ctx.rng.push_digits(buf, 11);
        }
        _ => {
            buf.reserve(11);
            buf.push_str("DE");
            ctx.rng.push_digits(buf, 9);
        }
    }
}
