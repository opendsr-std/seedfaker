//! Shared option resolution — eliminates duplicates across CLI and all bindings.

use crate::locale::Locale;
use crate::script::{Corrupt, Ctx};

/// Maximum record count (10 billion).
pub const MAX_COUNT: u64 = 10_000_000_000;

/// Resolve seed string to u64. None → random.
pub fn resolve_seed(seed: Option<&str>) -> u64 {
    match seed {
        Some(s) => crate::hash_seed(s),
        None => crate::rng::random_seed(),
    }
}

/// Resolve locale string to locale list. None → all locales.
pub fn resolve_locales(locale: Option<&str>) -> Result<Vec<&'static Locale>, String> {
    match locale {
        Some(s) if !s.is_empty() => crate::locale::resolve_str(s),
        _ => Ok(crate::locale::ALL_CODES.iter().filter_map(|c| crate::locale::get(c)).collect()),
    }
}

/// Resolve ctx string to Ctx enum. Rejects unknown values.
pub fn resolve_ctx(ctx: Option<&str>) -> Result<Ctx, String> {
    match ctx {
        Some(s) if !s.is_empty() => match s {
            "strict" => Ok(Ctx::Strict),
            "loose" => Ok(Ctx::Loose),
            other => Err(format!("unknown ctx mode: '{other}' (expected 'strict' or 'loose')")),
        },
        _ => Ok(Ctx::None),
    }
}

/// Resolve corrupt string to rate. Rejects unknown levels.
pub fn resolve_corrupt_rate(corrupt: Option<&str>) -> Result<Option<f64>, String> {
    match corrupt {
        Some(s) if !s.is_empty() => match Corrupt::parse_level(s) {
            Some(level) => Ok(Some(level.rate())),
            None => Err(format!(
                "unknown corrupt level: '{s}' (expected 'low', 'mid', 'high', or 'extreme')"
            )),
        },
        _ => Ok(None),
    }
}

/// Resolve all constructor options at once.
///
/// Returns `(master_seed, locales, tz_offset, since, until)`.
pub fn resolve_all(
    seed: Option<&str>,
    locale: Option<&str>,
    tz: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<(u64, Vec<&'static Locale>, i32, i64, i64), String> {
    let locales = resolve_locales(locale)?;
    let master_seed = resolve_seed(seed);
    let (tz_offset, since_e, until_e) = resolve_time(tz, since, until)?;
    Ok((master_seed, locales, tz_offset, since_e, until_e))
}

/// Resolve tz/since/until strings to `(tz_offset, since_epoch, until_epoch)`.
pub fn resolve_time(
    tz: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<(i32, i64, i64), String> {
    let tz_offset = match tz {
        Some(s) => crate::tz::parse(s)?,
        None => 0,
    };
    let since_epoch = match since {
        Some(s) => crate::temporal::parse(s)?,
        None => crate::temporal::DEFAULT_SINCE,
    };
    let until_epoch = match until {
        Some(s) => crate::temporal::parse_until(s)?,
        None => crate::temporal::default_until(),
    };
    Ok((tz_offset, since_epoch, until_epoch))
}
