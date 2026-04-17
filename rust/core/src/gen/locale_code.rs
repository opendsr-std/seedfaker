use crate::ctx::GenContext;

const LOCALES: &[(&str, &str)] = &[
    ("en", "US"),
    ("en", "GB"),
    ("en", "CA"),
    ("en", "AU"),
    ("en", "NZ"),
    ("en", "SG"),
    ("en", "ZA"),
    ("en", "NG"),
    ("en", "IE"),
    ("en", "IN"),
    ("de", "DE"),
    ("de", "AT"),
    ("de", "CH"),
    ("fr", "FR"),
    ("fr", "CA"),
    ("fr", "BE"),
    ("fr", "CH"),
    ("es", "ES"),
    ("es", "MX"),
    ("es", "AR"),
    ("es", "CO"),
    ("es", "CL"),
    ("it", "IT"),
    ("it", "CH"),
    ("ja", "JP"),
    ("zh", "CN"),
    ("zh", "TW"),
    ("zh", "HK"),
    ("pt", "BR"),
    ("pt", "PT"),
    ("ko", "KR"),
    ("nl", "NL"),
    ("nl", "BE"),
    ("pl", "PL"),
    ("ru", "RU"),
    ("ar", "SA"),
    ("ar", "AE"),
    ("ar", "EG"),
    ("hi", "IN"),
    ("vi", "VN"),
    ("tr", "TR"),
    ("uk", "UA"),
    ("sv", "SE"),
    ("da", "DK"),
    ("nb", "NO"),
    ("fi", "FI"),
    ("cs", "CZ"),
    ("sk", "SK"),
    ("hu", "HU"),
    ("ro", "RO"),
    ("hr", "HR"),
    ("bg", "BG"),
    ("sr", "RS"),
    ("el", "GR"),
    ("he", "IL"),
    ("th", "TH"),
    ("id", "ID"),
    ("ms", "MY"),
];

fn code_to_locale_pair(code: &str) -> Option<(&'static str, &'static str)> {
    LOCALES
        .iter()
        .find(|&&(l, _)| {
            // Match locale code prefix (e.g., "de" matches "de", "de-at" matches "de")
            code == l || code.starts_with(l) || code.split('-').next() == Some(l)
        })
        .map(|&(l, r)| (l, r))
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let (lang, region) =
        if let Some(pair) = ctx.identity.and_then(|id| code_to_locale_pair(id.locale_code)) {
            pair
        } else {
            let loc = ctx.locale();
            code_to_locale_pair(loc.code).unwrap_or(("en", "US"))
        };
    match ctx.modifier {
        "short" => buf.push_str(lang),
        "underscore" => {
            buf.reserve(lang.len() + 1 + region.len());
            buf.push_str(lang);
            buf.push('_');
            buf.push_str(region);
        }
        _ => {
            buf.reserve(lang.len() + 1 + region.len());
            buf.push_str(lang);
            buf.push('-');
            buf.push_str(region);
        }
    }
}
