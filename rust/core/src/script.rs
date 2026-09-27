use crate::locale::Locale;
use crate::rng::Rng;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Script {
    #[default]
    Latin,
    Native,
    Both,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Ctx {
    #[default]
    None,
    Loose,
    Strict,
}

impl Ctx {
    pub fn parse(s: &str) -> Self {
        match s {
            "strict" => Self::Strict,
            "loose" => Self::Loose,
            _ => Self::None,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Corrupt {
    None,
    Low,
    Mid,
    High,
    Extreme,
}

impl Corrupt {
    pub fn parse_level(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Self::Low),
            "mid" => Some(Self::Mid),
            "high" => Some(Self::High),
            "extreme" => Some(Self::Extreme),
            _ => None,
        }
    }

    pub fn rate(self) -> f64 {
        match self {
            Corrupt::None => 0.0,
            Corrupt::Low => 0.05,
            Corrupt::Mid => 0.15,
            Corrupt::High => 0.45,
            Corrupt::Extreme => 0.95,
        }
    }
}

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
