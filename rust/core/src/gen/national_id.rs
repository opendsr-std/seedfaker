use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "de" => super::steuer_id::gen(ctx, buf),
        "fr" => super::nir::gen(ctx, buf),
        "it" => super::codice_fiscale::gen(ctx, buf),
        "es" => super::dni::gen(ctx, buf),
        "nl" => super::bsn::gen(ctx, buf),
        "se" => super::personnummer::gen(ctx, buf),
        "pl" => super::pesel::gen(ctx, buf),
        "tr" => super::tc_kimlik::gen(ctx, buf),
        "ru" => super::inn::gen(ctx, buf),
        "uk" => super::ipn::gen(ctx, buf),
        "be" => ctx.rng.push_digits(buf, 14),
        "sr" | "hr" | "sl" => super::jmbg::gen(ctx, buf),
        "ar" => super::cuil::gen(ctx, buf),
        "pt-br" => super::cpf::gen(ctx, buf),
        "mx" => super::curp::gen(ctx, buf),
        "cl" => super::rut::gen(ctx, buf),
        "co" | "pe" | "uy" => super::cedula::gen(ctx, buf),
        "ro" | "ko" => ctx.rng.push_digits(buf, 13),
        "bg" => super::egn::gen(ctx, buf),
        "hu" => super::szemelyi_szam::gen(ctx, buf),
        "cs" | "sk" => super::rodne_cislo::gen(ctx, buf),
        "fi" => super::hetu::gen(ctx, buf),
        "da" => super::cpr::gen(ctx, buf),
        "no" => super::fodselsnummer::gen(ctx, buf),
        "ie" => super::pps::gen(ctx, buf),
        "el" => super::amka::gen(ctx, buf),
        "hi" => super::aadhaar::gen(ctx, buf),
        "vi" => super::cccd::gen(ctx, buf),
        "zh" => super::shenfenzheng::gen(ctx, buf),
        "ja" => ctx.rng.push_digits(buf, 12),
        "en" | "en-gb" | "en-ca" | "en-au" | "en-nz" | "en-sg" | "en-za" | "en-ng" => {
            super::ssn::gen_us_ssn_buf(&mut ctx.rng, "", buf);
        }
        _ => ctx.rng.push_digits(buf, 11),
    }
}
