use seedfaker_core::locale::Locale;
use seedfaker_core::rng::Rng;
use seedfaker_core::script::Script;

pub fn apply_script(locales: &[&Locale], script: Script, rng: &mut Rng) -> Vec<Locale> {
    locales
        .iter()
        .map(|loc| {
            let use_native = match script {
                Script::Latin => false,
                Script::Native => true,
                Script::Both => rng.maybe(0.5),
            };
            if use_native {
                Locale {
                    code: loc.code,
                    name_order: loc.name_order,
                    first_names: loc.native_first_names.unwrap_or(loc.first_names),
                    first_names_common: loc.first_names_common,
                    last_names: loc.native_last_names.unwrap_or(loc.last_names),
                    last_names_common: loc.last_names_common,
                    domains: loc.domains,
                    domains_common: loc.domains_common,
                    companies: loc.companies,
                    cities: loc.native_cities.unwrap_or(loc.cities),
                    streets: loc.native_streets.unwrap_or(loc.streets),
                    native_first_names: loc.native_first_names,
                    native_last_names: loc.native_last_names,
                    native_cities: loc.native_cities,
                    native_streets: loc.native_streets,
                }
            } else {
                **loc
            }
        })
        .collect()
}

/// Extract field specs from a CLI inline template for auto-detection.
pub fn template_fields(tpl: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut rest = tpl;
    while let Some(start) = rest.find("{{") {
        let after = &rest[start + 2..];
        if let Some(end) = after.find("}}") {
            let token = after[..end].trim();
            if !token.is_empty()
                && token != "serial"
                && !token.starts_with('#')
                && !token.starts_with('/')
            {
                fields.push(token.to_string());
            }
            rest = &after[end + 2..];
        } else {
            break;
        }
    }
    fields
}
