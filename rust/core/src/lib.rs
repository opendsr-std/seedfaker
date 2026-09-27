#![forbid(unsafe_code)]

pub mod corrupt;
pub mod ctx;
pub mod eval;
pub mod field;
pub mod gen;
pub mod locale;
pub mod opts;
pub mod pipeline;
pub mod rng;
pub mod script;
pub mod temporal;
pub mod tz;
pub mod validate;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_TZ_OFFSET: i32 = 0;

// Domain keys for sub-seed derivation. Must match across CLI, npm, pip.
pub const DOMAIN_IDENTITY: &str = "__identity__";
pub const DOMAIN_CORRUPT: &str = "__corrupt__";
pub const DOMAIN_SCRIPT: &str = "__script__";
pub const DOMAIN_LOCALE: &str = "__locale__";
pub const DOMAIN_TPL: &str = "__tpl__";

pub fn hash_seed(s: &str) -> u64 {
    fnv1a(s.as_bytes())
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

fn fnv1a(bytes: &[u8]) -> u64 {
    fnv1a_update(FNV_OFFSET, bytes)
}

fn fnv1a_update(mut h: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// Build info JSON: version + fingerprint for runtime integrity checks.
pub fn build_info() -> String {
    let fp = fingerprint();
    format!(r#"{{"version":"{VERSION}","fingerprint":"{fp}"}}"#)
}

/// Generator fingerprint: changes when seeded output changes.
///
/// Hashes `pipeline::generate_records` output for every field and modifier in
/// every locale, in Latin and native scripts, under each ctx mode. The `en`
/// locale is also hashed with each transform, omit, a non-zero tz offset, and
/// each corruption level.
/// Format: `sf1-<16 hex digits>`.
pub fn fingerprint() -> String {
    use field::{field_modifiers, Transform, REGISTRY};
    use pipeline::{field_domain_hash, generate_records, FieldSpec, RecordOpts};
    use script::{apply_script, Ctx, Script};

    const CANONICAL_SEED: &str = "__determinism__";
    const RECORDS: u64 = 2;

    fn record_opts<'a>(
        master_seed: u64,
        locales: &'a [&'a locale::Locale],
        since: i64,
        until: i64,
    ) -> RecordOpts<'a> {
        RecordOpts {
            master_seed,
            locales,
            ctx: Ctx::None,
            corrupt_rate: None,
            tz_offset_minutes: DEFAULT_TZ_OFFSET,
            since,
            until,
        }
    }

    let master = hash_seed(CANONICAL_SEED);
    let since = temporal::DEFAULT_SINCE;
    let until = temporal::date_to_epoch(2038, 1, 1, 0, 0, 0);

    let specs_with = |transform: Transform, omit_pct: Option<u8>| {
        let mut specs = Vec::new();
        for f in REGISTRY {
            let mods = field_modifiers(f.id).split(", ").filter(|m| !m.is_empty());
            for modifier in std::iter::once("").chain(mods) {
                specs.push(FieldSpec {
                    field: f,
                    modifier,
                    domain_hash: field_domain_hash(master, f, modifier),
                    range: None,
                    transform,
                    omit_pct,
                });
            }
        }
        specs
    };
    let specs = specs_with(Transform::None, None);

    let mut h = FNV_OFFSET;
    let mut hash = |record_opts: &RecordOpts<'_>, specs: &[FieldSpec<'_>]| {
        for record in generate_records(record_opts, specs, RECORDS, 0) {
            for value in record {
                h = fnv1a_update(h, value.as_bytes());
                h = fnv1a_update(h, b"\0");
            }
        }
    };

    let mut script_rng = rng::Rng::derive(master, 0, DOMAIN_SCRIPT);
    for code in locale::ALL_CODES {
        let Some(loc) = locale::get(code) else { continue };
        let mut variants = vec![*loc];
        variants.extend(apply_script(&[loc], Script::Native, &mut script_rng));
        for variant in &variants {
            let locales = [variant];
            for ctx in [Ctx::None, Ctx::Loose, Ctx::Strict] {
                hash(&RecordOpts { ctx, ..record_opts(master, &locales, since, until) }, &specs);
            }
        }
    }

    if let Some(en) = locale::get("en") {
        let locales = [en];
        for transform in [Transform::Upper, Transform::Lower, Transform::Capitalize] {
            hash(&record_opts(master, &locales, since, until), &specs_with(transform, None));
        }
        hash(&record_opts(master, &locales, since, until), &specs_with(Transform::None, Some(50)));
        hash(
            &RecordOpts { tz_offset_minutes: 330, ..record_opts(master, &locales, since, until) },
            &specs,
        );
        for level in ["low", "mid", "high", "extreme"] {
            if let Ok(Some(rate)) = opts::resolve_corrupt_rate(Some(level)) {
                hash(
                    &RecordOpts {
                        corrupt_rate: Some(rate),
                        ..record_opts(master, &locales, since, until)
                    },
                    &specs,
                );
            }
        }
    }

    format!("sf1-{h:016x}")
}
