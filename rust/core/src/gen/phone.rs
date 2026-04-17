use std::fmt::Write;

use crate::ctx::GenContext;
use crate::rng::Rng;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let raw = match loc.code {
        "de" | "de-at" => phone_de(&mut ctx.rng),
        "fr" => phone_fr(&mut ctx.rng),
        "ja" => phone_ja(&mut ctx.rng),
        "es" | "ar" | "mx" | "cl" | "co" | "pe" | "uy" => phone_es(&mut ctx.rng),
        "pt-br" => phone_br(&mut ctx.rng),
        "it" => phone_it(&mut ctx.rng),
        "nl" => phone_nl(&mut ctx.rng),
        "pl" => phone_pl(&mut ctx.rng),
        "se" => phone_se(&mut ctx.rng),
        "tr" => phone_tr(&mut ctx.rng),
        "uk" => phone_ua(&mut ctx.rng),
        "be" => phone_by(&mut ctx.rng),
        "sr" | "hr" | "sl" => phone_rs(&mut ctx.rng),
        "ro" | "bg" | "hu" | "cs" | "sk" => phone_ee(&mut ctx.rng, loc.code),
        "fi" => phone_fi(&mut ctx.rng),
        "da" => phone_dk(&mut ctx.rng),
        "no" => phone_no(&mut ctx.rng),
        "el" => phone_gr(&mut ctx.rng),
        "pt" => phone_pt(&mut ctx.rng),
        "et" | "lt" | "lv" => phone_baltic(&mut ctx.rng, loc.code),
        "ie" => phone_ie(&mut ctx.rng),
        "hi" => phone_in(&mut ctx.rng),
        "vi" => phone_vn(&mut ctx.rng),
        "zh" | "tw" => phone_cn(&mut ctx.rng),
        "ru" => phone_ru(&mut ctx.rng),
        "ko" => phone_kr(&mut ctx.rng),
        "id" => phone_id(&mut ctx.rng),
        "th" => phone_th(&mut ctx.rng),
        "ms" => phone_my(&mut ctx.rng),
        "tl" => phone_ph(&mut ctx.rng),
        "ar-sa" | "ar-ae" | "eg" => phone_arab(&mut ctx.rng, loc.code),
        "pk" | "bd" => phone_sa(&mut ctx.rng, loc.code),
        "he" => phone_il(&mut ctx.rng),
        "en-gb" => phone_gb(&mut ctx.rng),
        "en-au" | "en-nz" => phone_au(&mut ctx.rng),
        "en-sg" => phone_sg(&mut ctx.rng),
        _ => phone_en(&mut ctx.rng),
    };
    if ctx.modifier.is_empty() {
        buf.push_str(&raw);
        return;
    }
    let cc = phone_country_code(loc.code);
    let digits = extract_phone_digits(&raw, cc);
    match ctx.modifier {
        "e164" => {
            buf.push('+');
            buf.push_str(cc);
            buf.push_str(&digits);
        }
        "intl" => buf.push_str(&format_intl(&digits, cc)),
        "plain" => buf.push_str(&digits),
        _ => buf.push_str(&raw),
    }
}

fn phone_country_code(locale: &str) -> &'static str {
    let full = super::helpers::locale_to_phone_code(locale);
    full.strip_prefix('+').unwrap_or(full)
}

fn extract_phone_digits(raw: &str, cc: &str) -> String {
    let all_digits: String = raw.chars().filter(char::is_ascii_digit).collect();
    all_digits.strip_prefix(cc).unwrap_or(&all_digits).to_string()
}

fn format_intl(digits: &str, cc: &str) -> String {
    let len = digits.len();
    if len <= 4 {
        let mut s = String::with_capacity(2 + cc.len() + digits.len());
        s.push('+');
        s.push_str(cc);
        s.push(' ');
        s.push_str(digits);
        return s;
    }
    let mut parts = Vec::new();
    let mut pos = 0;
    let chunk_size = if len > 8 { 3 } else { 2 };
    while pos < len {
        let end = (pos + chunk_size).min(len);
        parts.push(&digits[pos..end]);
        pos = end;
    }
    let joined = parts.join(" ");
    let mut s = String::with_capacity(1 + cc.len() + 1 + joined.len());
    s.push('+');
    s.push_str(cc);
    s.push(' ');
    s.push_str(&joined);
    s
}

fn phone_en(rng: &mut Rng) -> String {
    let area = rng.range(201, 989);
    let exc = rng.range(200, 999);
    let sub = rng.range(1000, 9999);
    let mut s = String::with_capacity(18);
    match rng.urange(0, 3) {
        0 => {
            let _ = write!(s, "({area}) {exc}-{sub}");
        }
        1 => {
            let _ = write!(s, "{area}-{exc}-{sub}");
        }
        2 => {
            let _ = write!(s, "+1-{area}-{exc}-{sub}");
        }
        _ => {
            let _ = write!(s, "+1 ({area}) {exc}-{sub}");
        }
    }
    s
}

fn phone_de(rng: &mut Rng) -> String {
    match rng.urange(0, 2) {
        0 => {
            let mut s = String::with_capacity(14);
            s.push_str("+49 30 ");
            rng.push_digits(&mut s, 7);
            s
        }
        1 => {
            let prefix = rng.range(151, 179);
            let mut s = String::with_capacity(16);
            let _ = write!(s, "+49 {prefix} ");
            rng.push_digits(&mut s, 7);
            s
        }
        _ => {
            let mut s = String::with_capacity(12);
            s.push_str("0170/");
            rng.push_digits(&mut s, 7);
            s
        }
    }
}

fn phone_fr(rng: &mut Rng) -> String {
    let mut d = String::with_capacity(11);
    rng.push_digits(&mut d, 2);
    d.push(' ');
    rng.push_digits(&mut d, 2);
    d.push(' ');
    rng.push_digits(&mut d, 2);
    d.push(' ');
    rng.push_digits(&mut d, 2);

    match rng.urange(0, 2) {
        0 => {
            let mut s = String::with_capacity(17);
            s.push_str("+33 1 ");
            s.push_str(&d);
            s
        }
        1 => {
            let mut s = String::with_capacity(17);
            s.push_str("+33 6 ");
            s.push_str(&d);
            s
        }
        _ => {
            let mut s = String::with_capacity(14);
            s.push_str("06 ");
            s.push_str(&d);
            s
        }
    }
}

fn phone_ja(rng: &mut Rng) -> String {
    match rng.urange(0, 2) {
        0 => {
            let mut s = String::with_capacity(16);
            s.push_str("+81 3-");
            rng.push_digits(&mut s, 4);
            s.push('-');
            rng.push_digits(&mut s, 4);
            s
        }
        1 => {
            let mut s = String::with_capacity(12);
            s.push_str("03-");
            rng.push_digits(&mut s, 4);
            s.push('-');
            rng.push_digits(&mut s, 4);
            s
        }
        _ => {
            let mut s = String::with_capacity(13);
            s.push_str("090-");
            rng.push_digits(&mut s, 4);
            s.push('-');
            rng.push_digits(&mut s, 4);
            s
        }
    }
}

fn phone_es(rng: &mut Rng) -> String {
    let codes = ["+34", "+52", "+54", "+57", "+56", "+51", "+598"];
    let cc = rng.choice(&codes);
    let mut s = String::with_capacity(16);
    s.push_str(cc);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_br(rng: &mut Rng) -> String {
    match rng.urange(0, 2) {
        0 => {
            let mut s = String::with_capacity(18);
            s.push_str("+55 11 ");
            rng.push_digits(&mut s, 5);
            s.push('-');
            rng.push_digits(&mut s, 4);
            s
        }
        1 => {
            let mut s = String::with_capacity(18);
            s.push_str("+55 21 ");
            rng.push_digits(&mut s, 5);
            s.push('-');
            rng.push_digits(&mut s, 4);
            s
        }
        _ => {
            let mut s = String::with_capacity(15);
            s.push_str("(11) ");
            rng.push_digits(&mut s, 5);
            s.push('-');
            rng.push_digits(&mut s, 4);
            s
        }
    }
}

fn phone_it(rng: &mut Rng) -> String {
    match rng.urange(0, 2) {
        0 => {
            let mut s = String::with_capacity(15);
            s.push_str("+39 02 ");
            rng.push_digits(&mut s, 8);
            s
        }
        1 => {
            let mut s = String::with_capacity(15);
            s.push_str("+39 06 ");
            rng.push_digits(&mut s, 8);
            s
        }
        _ => {
            let prefix = rng.range(20, 99);
            let mut s = String::with_capacity(16);
            let _ = write!(s, "+39 3{prefix} ");
            rng.push_digits(&mut s, 7);
            s
        }
    }
}

fn phone_nl(rng: &mut Rng) -> String {
    let mut s = String::with_capacity(14);
    s.push_str("+31 6 ");
    rng.push_digits(&mut s, 8);
    s
}

fn phone_pl(rng: &mut Rng) -> String {
    let mut s = String::with_capacity(15);
    s.push_str("+48 ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s
}

fn phone_se(rng: &mut Rng) -> String {
    let d = rng.range(0, 9);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "+46 7{d} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_tr(rng: &mut Rng) -> String {
    let d = rng.range(30, 59);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "+90 5{d} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_ua(rng: &mut Rng) -> String {
    let prefix = rng.range(50, 99);
    let mut s = String::with_capacity(17);
    let _ = write!(s, "+380 {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_by(rng: &mut Rng) -> String {
    let prefix = rng.range(25, 44);
    let mut s = String::with_capacity(15);
    let _ = write!(s, "+375 {prefix} ");
    rng.push_digits(&mut s, 7);
    s
}

fn phone_rs(rng: &mut Rng) -> String {
    let d = rng.range(0, 9);
    let mut s = String::with_capacity(15);
    let _ = write!(s, "+381 6{d} ");
    rng.push_digits(&mut s, 7);
    s
}

fn phone_ee(rng: &mut Rng, code: &str) -> String {
    let cc = match code {
        "bg" => "+359",
        "hu" => "+36",
        "cs" => "+420",
        "sk" => "+421",
        _ => "+40",
    };
    let mut s = String::with_capacity(16);
    s.push_str(cc);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 6);
    s
}

fn phone_fi(rng: &mut Rng) -> String {
    let d = rng.range(0, 9);
    let mut s = String::with_capacity(15);
    let _ = write!(s, "+358 4{d} ");
    rng.push_digits(&mut s, 7);
    s
}

fn phone_dk(rng: &mut Rng) -> String {
    let mut s = String::with_capacity(12);
    s.push_str("+45 ");
    rng.push_digits(&mut s, 4);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_no(rng: &mut Rng) -> String {
    let d = rng.range(0, 9);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "+47 4{d} ");
    rng.push_digits(&mut s, 2);
    s.push(' ');
    rng.push_digits(&mut s, 5);
    s
}

fn phone_gr(rng: &mut Rng) -> String {
    let mut s = String::with_capacity(15);
    s.push_str("+30 69");
    rng.push_digits(&mut s, 2);
    s.push(' ');
    rng.push_digits(&mut s, 6);
    s
}

fn phone_pt(rng: &mut Rng) -> String {
    let d = rng.range(1, 6);
    let mut s = String::with_capacity(17);
    let _ = write!(s, "+351 9{d} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_baltic(rng: &mut Rng, code: &str) -> String {
    let cc = match code {
        "et" => "+372",
        "lv" => "+371",
        _ => "+370",
    };
    let mut s = String::with_capacity(14);
    s.push_str(cc);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_ie(rng: &mut Rng) -> String {
    let d = rng.range(3, 9);
    let mut s = String::with_capacity(17);
    let _ = write!(s, "+353 8{d} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_in(rng: &mut Rng) -> String {
    let prefix = rng.range(70, 99);
    let mut s = String::with_capacity(15);
    let _ = write!(s, "+91 {prefix} ");
    rng.push_digits(&mut s, 8);
    s
}

fn phone_vn(rng: &mut Rng) -> String {
    let prefix = rng.range(30, 99);
    let mut s = String::with_capacity(14);
    let _ = write!(s, "+84 {prefix} ");
    rng.push_digits(&mut s, 7);
    s
}

fn phone_cn(rng: &mut Rng) -> String {
    let prefix = *rng.choice(&[
        "130", "131", "132", "135", "136", "137", "138", "139", "150", "151", "152", "155", "156",
        "185", "186", "187", "188", "189",
    ]);
    let mut s = String::with_capacity(17);
    s.push_str("+86 ");
    s.push_str(prefix);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_ru(rng: &mut Rng) -> String {
    let prefix = rng.range(900, 999);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "+7 {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_kr(rng: &mut Rng) -> String {
    let mut s = String::with_capacity(16);
    s.push_str("+82 10 ");
    rng.push_digits(&mut s, 4);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_id(rng: &mut Rng) -> String {
    let prefix = rng.range(811, 899);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "+62 {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_th(rng: &mut Rng) -> String {
    let prefix = rng.range(80, 99);
    let mut s = String::with_capacity(15);
    let _ = write!(s, "+66 {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_my(rng: &mut Rng) -> String {
    let prefix = rng.range(10, 19);
    let mut s = String::with_capacity(15);
    let _ = write!(s, "+60 {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_ph(rng: &mut Rng) -> String {
    let prefix = rng.range(900, 999);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "+63 {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_arab(rng: &mut Rng, code: &str) -> String {
    let cc = match code {
        "ar-ae" => "+971",
        "eg" => "+20",
        _ => "+966",
    };
    let prefix = rng.range(50, 59);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "{cc} {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_sa(rng: &mut Rng, code: &str) -> String {
    let cc = match code {
        "bd" => "+880",
        _ => "+92",
    };
    let prefix = rng.range(300, 399);
    let mut s = String::with_capacity(16);
    let _ = write!(s, "{cc} {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_il(rng: &mut Rng) -> String {
    let prefix = rng.range(50, 58);
    let mut s = String::with_capacity(15);
    let _ = write!(s, "+972 {prefix} ");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}

fn phone_gb(rng: &mut Rng) -> String {
    let mut s = String::with_capacity(16);
    s.push_str("+44 7");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s
}

fn phone_au(rng: &mut Rng) -> String {
    let mut s = String::with_capacity(15);
    s.push_str("+61 4");
    rng.push_digits(&mut s, 2);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 3);
    s
}

fn phone_sg(rng: &mut Rng) -> String {
    let prefix = rng.range(8, 9);
    let mut s = String::with_capacity(13);
    let _ = write!(s, "+65 {prefix}");
    rng.push_digits(&mut s, 3);
    s.push(' ');
    rng.push_digits(&mut s, 4);
    s
}
