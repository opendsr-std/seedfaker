use crate::ctx::GenContext;
use crate::rng::Rng;

use super::helpers::charsets::UPPER;
use super::helpers::locale_to_country_code;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    match ctx.modifier {
        "international" => {
            super::passport_intl::gen(ctx, buf);
        }
        "internal" => {
            super::passport_internal::gen(ctx, buf);
        }
        _ => {
            let loc = ctx.locale();
            let cc = locale_to_country_code(loc.code);
            buf.push_str(&gen_passport_for_country(&mut ctx.rng, cc));
        }
    }
}

fn gen_passport_for_country(rng: &mut Rng, cc: &str) -> String {
    match cc {
        "US" => rng.digits(9),
        "GB" | "AU" | "CA" | "NZ" => {
            let mut s = String::with_capacity(9);
            rng.push_upper(&mut s, 2);
            rng.push_digits(&mut s, 6);
            s
        }
        "FR" => {
            let mut s = String::with_capacity(9);
            rng.push_digits(&mut s, 2);
            rng.push_upper(&mut s, 2);
            rng.push_digits(&mut s, 5);
            s
        }
        "JP" => {
            let mut s = String::with_capacity(9);
            rng.push_upper(&mut s, 2);
            rng.push_digits(&mut s, 7);
            s
        }
        "BR" => {
            let mut s = String::with_capacity(8);
            rng.push_upper(&mut s, 2);
            rng.push_digits(&mut s, 6);
            s
        }
        "IN" => {
            let mut s = String::with_capacity(8);
            rng.push_charset(&mut s, UPPER, 1);
            rng.push_digits(&mut s, 7);
            s
        }
        _ => {
            // Generic ICAO format: 1-2 letters + 6-8 digits
            let mut s = String::with_capacity(9);
            rng.push_charset(&mut s, UPPER, 1);
            rng.push_digits(&mut s, 8);
            s
        }
    }
}
