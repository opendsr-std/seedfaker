use crate::ctx::GenContext;

fn locale_to_iban(code: &str) -> Option<(&'static str, usize)> {
    match code {
        "de" => Some(("DE", 22)),
        "fr" | "be" => Some(("FR", 27)),
        "it" => Some(("IT", 27)),
        "es" | "mx" | "cl" | "co" | "pe" | "uy" | "ar" => Some(("ES", 24)),
        "nl" => Some(("NL", 18)),
        "pt" | "pt-br" => Some(("PT", 25)),
        "ie" => Some(("GB", 22)),
        "se" => Some(("SE", 24)),
        "da" => Some(("DK", 18)),
        "no" => Some(("NO", 15)),
        "fi" => Some(("FI", 18)),
        "pl" => Some(("PL", 28)),
        "cs" => Some(("CZ", 24)),
        "sk" => Some(("SK", 24)),
        "hu" => Some(("HU", 28)),
        "ro" => Some(("RO", 24)),
        "hr" => Some(("HR", 21)),
        "bg" => Some(("BG", 22)),
        "sl" => Some(("SI", 19)),
        "el" => Some(("GR", 27)),
        "et" => Some(("EE", 20)),
        "lt" => Some(("LT", 20)),
        "lv" => Some(("LV", 21)),
        "tr" => Some(("TR", 26)),
        "sr" => Some(("RS", 22)),
        "uk" => Some(("UA", 29)),
        _ => None,
    }
}

// Format: ISO 13616 (IBAN) — https://www.iso.org/standard/81090.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let (cc, len) = locale_to_iban(loc.code).unwrap_or_else(|| {
        let fallback = [("DE", 22), ("GB", 22), ("FR", 27), ("ES", 24), ("IT", 27), ("NL", 18)];
        fallback[ctx.rng.urange(0, fallback.len() - 1)]
    });
    let digit_count = len - 4;
    let mut raw = String::with_capacity(len);
    raw.push_str(cc);
    ctx.rng.push_digits(&mut raw, 2);
    ctx.rng.push_digits(&mut raw, digit_count);
    if ctx.modifier == "plain" {
        buf.push_str(&raw);
    } else {
        let formatted = raw
            .as_bytes()
            .chunks(4)
            .map(|c| std::str::from_utf8(c).unwrap_or(""))
            .collect::<Vec<_>>()
            .join(" ");
        buf.push_str(&formatted);
    }
}
