use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let (city_name, region, postal) = if let Some(id) = ctx.identity {
        (id.city.as_str(), id.region.as_str(), id.postal.as_str())
    } else {
        let city = ctx.rng.choice(loc.cities);
        (city.name, city.region, city.postal)
    };
    let street = if loc.streets.is_empty() { "Main Street" } else { ctx.rng.choice(loc.streets) };
    let n = ctx.rng.range(1, 200);
    let start = buf.len();
    match loc.code {
        "de" | "de-at" | "nl" | "nl-be" | "pt" => {
            buf.reserve(street.len() + 20 + postal.len() + city_name.len());
            buf.push_str(street);
            let _ = write!(buf, " {n}, ");
            buf.push_str(postal);
            buf.push(' ');
            buf.push_str(city_name);
        }
        "fr" | "fr-be" | "fr-ca" => {
            buf.reserve(street.len() + 20 + postal.len() + city_name.len());
            let _ = write!(buf, "{n} ");
            buf.push_str(street);
            buf.push_str(", ");
            buf.push_str(postal);
            buf.push(' ');
            buf.push_str(city_name);
        }
        "ja" => {
            let c = ctx.rng.range(1, 9);
            let b = ctx.rng.range(1, 30);
            let g = ctx.rng.range(1, 20);
            buf.reserve(region.len() + city_name.len() + 12);
            buf.push_str(region);
            buf.push(' ');
            buf.push_str(city_name);
            let _ = write!(buf, " {c}-{b}-{g}");
        }
        "es" | "ar" | "mx" | "cl" | "co" | "pe" | "uy" => {
            let n2 = ctx.rng.range(1, 9999);
            buf.reserve(street.len() + 12 + city_name.len() + region.len());
            buf.push_str(street);
            let _ = write!(buf, " {n2}, ");
            buf.push_str(city_name);
            buf.push_str(", ");
            buf.push_str(region);
        }
        "pt-br" => {
            let n2 = ctx.rng.range(1, 9999);
            buf.reserve(street.len() + 30 + city_name.len() + region.len() + postal.len());
            buf.push_str(street);
            let _ = write!(buf, ", {n2} - ");
            buf.push_str(city_name);
            buf.push_str(", ");
            buf.push_str(region);
            buf.push_str(" - CEP ");
            buf.push_str(postal);
        }
        "it" => {
            buf.reserve(4 + street.len() + 20 + postal.len() + city_name.len());
            buf.push_str("Via ");
            buf.push_str(street);
            let _ = write!(buf, " {n}, ");
            buf.push_str(postal);
            buf.push(' ');
            buf.push_str(city_name);
        }
        "hi" => {
            let n2 = ctx.rng.range(1, 999);
            buf.reserve(20 + street.len() + city_name.len() + region.len() + postal.len());
            let _ = write!(buf, "{n2}, ");
            buf.push_str(street);
            buf.push_str(", ");
            buf.push_str(city_name);
            buf.push_str(", ");
            buf.push_str(region);
            buf.push_str(" - ");
            buf.push_str(postal);
        }
        "vi" => {
            let n2 = ctx.rng.range(1, 999);
            buf.reserve(10 + street.len() + city_name.len() + region.len());
            let _ = write!(buf, "{n2} ");
            buf.push_str(street);
            buf.push_str(", ");
            buf.push_str(city_name);
            buf.push_str(", ");
            buf.push_str(region);
        }
        "zh" => {
            let n2 = ctx.rng.range(1, 999);
            buf.reserve(region.len() + city_name.len() + street.len() + 10);
            buf.push_str(region);
            buf.push(' ');
            buf.push_str(city_name);
            buf.push(' ');
            buf.push_str(street);
            let _ = write!(buf, " {n2}");
        }
        _ => {
            let n2 = ctx.rng.range(1, 9999);
            buf.reserve(10 + street.len() + city_name.len() + region.len() + postal.len());
            let _ = write!(buf, "{n2} ");
            buf.push_str(street);
            buf.push_str(", ");
            buf.push_str(city_name);
            buf.push_str(", ");
            buf.push_str(region);
            buf.push(' ');
            buf.push_str(postal);
        }
    }
    let segment = &buf[start..];
    if segment.contains('\n') {
        let replaced = segment.replace('\n', ", ");
        buf.truncate(start);
        buf.push_str(&replaced);
    }
}
