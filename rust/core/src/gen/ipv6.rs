use crate::ctx::GenContext;
use crate::rng::Rng;

const HEX: &[u8; 16] = b"0123456789abcdef";

fn push_hex4(buf: &mut String, v: u16) {
    buf.push(HEX[(v >> 12) as usize] as char);
    buf.push(HEX[((v >> 8) & 0xf) as usize] as char);
    buf.push(HEX[((v >> 4) & 0xf) as usize] as char);
    buf.push(HEX[(v & 0xf) as usize] as char);
}

fn locale_ipv6_prefix(rng: &mut Rng, code: &str) -> u16 {
    let (base, range) = match code {
        "en" | "en-ca" | "fr-ca" => (0x2600, 16),
        "en-gb" | "de" | "de-at" | "fr" | "fr-be" | "it" | "es" | "nl" | "nl-be" | "pt" | "se"
        | "da" | "no" | "fi" | "pl" | "cs" | "sk" | "hu" | "ro" | "hr" | "bg" | "sr" | "ru"
        | "uk" | "be" | "el" | "cy" | "ie" | "sl" | "et" | "lt" | "lv" | "tr" | "he" | "mt"
        | "lb" | "ar-sa" | "ar-ae" => (0x2a00, 16),
        "ja" | "zh" | "hi" | "vi" | "ko" | "id" | "th" | "ms" | "tl" | "tw" | "en-au" | "en-nz"
        | "en-sg" | "pk" | "bd" => (0x2400, 16),
        "pt-br" | "ar" | "mx" | "cl" | "co" | "pe" | "uy" | "ve" | "ec" => (0x2800, 16),
        "en-za" | "en-ng" | "eg" => (0x2c00, 16),
        _ => (0x2000, 0x0e00),
    };
    base + rng.range(0, i64::from(range) - 1) as u16
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(39);
    for i in 0..8 {
        if i > 0 {
            buf.push(':');
        }
        if i == 0 && ctx.identity.is_some() {
            let loc = ctx.locale();
            push_hex4(buf, locale_ipv6_prefix(&mut ctx.rng, loc.code));
        } else {
            push_hex4(buf, ctx.rng.range(0, 0xffff) as u16);
        }
    }
}
