#[path = "_shared.rs"]
pub mod shared;

pub mod ar;
pub mod ar_ae;
pub mod ar_sa;
pub mod bd;
pub mod be;
pub mod bg;
pub mod cl;
pub mod co;
pub mod cs;
pub mod cy;
pub mod da;
pub mod de;
pub mod de_at;
pub mod ec;
pub mod eg;
pub mod el;
pub mod en;
pub mod en_au;
pub mod en_ca;
pub mod en_gb;
pub mod en_ng;
pub mod en_nz;
pub mod en_sg;
pub mod en_za;
pub mod es;
pub mod et;
pub mod fi;
pub mod fr;
pub mod fr_be;
pub mod fr_ca;
pub mod ga;
pub mod he;
pub mod hi;
pub mod hr;
pub mod hu;
pub mod id;
pub mod it;
pub mod ja;
pub mod ko;
pub mod lb;
pub mod lt;
pub mod lv;
pub mod ms;
pub mod mt;
pub mod mx;
pub mod nb;
pub mod nl;
pub mod nl_be;
pub mod pe;
pub mod pk;
pub mod pl;
pub mod pt;
pub mod pt_br;
pub mod ro;
pub mod ru;
pub mod sk;
pub mod sl;
pub mod sr;
pub mod sv;
pub mod th;
pub mod tl;
pub mod tr;
pub mod tw;
pub mod uk;
pub mod uy;
pub mod ve;
pub mod vi;
pub mod zh;

#[derive(Clone, Copy)]
pub struct City {
    pub name: &'static str,
    pub region: &'static str,
    pub postal: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub tz: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NameOrder {
    /// First Last (default for most locales)
    Western,
    /// Last First (hu, ja, ko, zh, vi)
    Eastern,
    /// First Last1 Last2
    DoubleSurname,
    /// First bin Father Last (Gulf Arabic)
    Patronymic { particle: &'static str },
    /// First Father Last (Egyptian style)
    PatronymicMiddle,
}

#[derive(Clone, Copy)]
pub struct Locale {
    pub code: &'static str,
    pub name_order: NameOrder,
    pub first_names: &'static [&'static str],
    /// Items before this index are "common" — selected with ~70% probability.
    /// 0 = uniform selection.
    pub first_names_common: usize,
    pub last_names: &'static [&'static str],
    pub last_names_common: usize,
    pub domains: &'static [&'static str],
    /// First N domains are personal email providers (gmail-like), rest are corporate.
    pub domains_common: usize,
    pub companies: &'static [&'static str],
    pub cities: &'static [City],
    pub streets: &'static [&'static str],
    pub native_first_names: Option<&'static [&'static str]>,
    pub native_last_names: Option<&'static [&'static str]>,
    pub native_cities: Option<&'static [City]>,
    pub native_streets: Option<&'static [&'static str]>,
}

// Single source of truth: (user-facing code, module::LOCALE).
// get() and ALL_CODES both derive from this array.
const LOCALE_TABLE: &[(&str, &Locale)] = &[
    ("en", &en::LOCALE),
    ("en-gb", &en_gb::LOCALE),
    ("en-ca", &en_ca::LOCALE),
    ("en-au", &en_au::LOCALE),
    ("en-nz", &en_nz::LOCALE),
    ("en-sg", &en_sg::LOCALE),
    ("en-za", &en_za::LOCALE),
    ("en-ng", &en_ng::LOCALE),
    ("de", &de::LOCALE),
    ("fr", &fr::LOCALE),
    ("ja", &ja::LOCALE),
    ("es", &es::LOCALE),
    ("pt-br", &pt_br::LOCALE),
    ("it", &it::LOCALE),
    ("nl", &nl::LOCALE),
    ("pl", &pl::LOCALE),
    ("se", &sv::LOCALE),
    ("tr", &tr::LOCALE),
    ("ru", &ru::LOCALE),
    ("uk", &uk::LOCALE),
    ("be", &be::LOCALE),
    ("sr", &sr::LOCALE),
    ("ar", &ar::LOCALE),
    ("ro", &ro::LOCALE),
    ("hr", &hr::LOCALE),
    ("bg", &bg::LOCALE),
    ("cs", &cs::LOCALE),
    ("sk", &sk::LOCALE),
    ("hu", &hu::LOCALE),
    ("fi", &fi::LOCALE),
    ("da", &da::LOCALE),
    ("no", &nb::LOCALE),
    ("el", &el::LOCALE),
    ("pt", &pt::LOCALE),
    ("mx", &mx::LOCALE),
    ("cl", &cl::LOCALE),
    ("co", &co::LOCALE),
    ("sl", &sl::LOCALE),
    ("et", &et::LOCALE),
    ("lt", &lt::LOCALE),
    ("lv", &lv::LOCALE),
    ("ie", &ga::LOCALE),
    ("pe", &pe::LOCALE),
    ("uy", &uy::LOCALE),
    ("hi", &hi::LOCALE),
    ("vi", &vi::LOCALE),
    ("zh", &zh::LOCALE),
    ("ko", &ko::LOCALE),
    ("id", &id::LOCALE),
    ("th", &th::LOCALE),
    ("ms", &ms::LOCALE),
    ("tl", &tl::LOCALE),
    ("tw", &tw::LOCALE),
    ("ve", &ve::LOCALE),
    ("ec", &ec::LOCALE),
    ("pk", &pk::LOCALE),
    ("bd", &bd::LOCALE),
    ("eg", &eg::LOCALE),
    ("de-at", &de_at::LOCALE),
    ("nl-be", &nl_be::LOCALE),
    ("fr-be", &fr_be::LOCALE),
    ("fr-ca", &fr_ca::LOCALE),
    ("cy", &cy::LOCALE),
    ("mt", &mt::LOCALE),
    ("lb", &lb::LOCALE),
    ("he", &he::LOCALE),
    ("ar-sa", &ar_sa::LOCALE),
    ("ar-ae", &ar_ae::LOCALE),
];

pub fn get(code: &str) -> Option<&'static Locale> {
    LOCALE_TABLE.iter().find(|(c, _)| *c == code).map(|(_, l)| *l)
}

pub const ALL_CODES: &[&str] = &{
    const N: usize = LOCALE_TABLE.len();
    let mut out = [""; N];
    let mut i = 0;
    while i < N {
        out[i] = LOCALE_TABLE[i].0;
        i += 1;
    }
    out
};

/// Parse locale string(s) into resolved locale references.
///
/// Accepts codes with optional weights: `["en", "de"]` or `["en=7", "es=2", "de=1"]`.
/// Weight = repeat count in the pool (higher weight = more likely selection).
/// Empty input returns all locales. `["all"]` also returns all locales.
/// Unknown locale codes produce an error.
pub fn resolve(codes: &[String]) -> Result<Vec<&'static Locale>, String> {
    if codes.is_empty() || (codes.len() == 1 && codes[0] == "all") {
        return Ok(ALL_CODES.iter().filter_map(|c| get(c)).collect());
    }
    let has_weights = codes.iter().any(|c| c.contains('='));
    let mut result = Vec::new();
    for code in codes {
        let (name, weight) = if has_weights {
            if let Some((n, w)) = code.split_once('=') {
                let w: usize = w.parse().map_err(|_| format!("invalid locale weight: '{code}'"))?;
                if w == 0 {
                    return Err(format!("locale weight must be > 0: '{code}'"));
                }
                (n, w)
            } else {
                (code.as_str(), 1)
            }
        } else {
            (code.as_str(), 1)
        };
        match get(name) {
            Some(loc) => {
                for _ in 0..weight {
                    result.push(loc);
                }
            }
            None => {
                return Err(format!("unknown locale '{name}'; valid: {}", ALL_CODES.join(", ")))
            }
        }
    }
    Ok(result)
}

/// Parse a comma-separated locale string into resolved locale references.
/// Convenience wrapper for APIs that accept a single string (Python, Node, MCP).
pub fn resolve_str(s: &str) -> Result<Vec<&'static Locale>, String> {
    let codes: Vec<String> = s.split(',').map(|c| c.trim().to_string()).collect();
    resolve(&codes)
}
