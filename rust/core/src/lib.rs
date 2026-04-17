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

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
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

/// Generator fingerprint: identifies the current deterministic algorithm version.
///
/// If this value changes between releases, seeded output has changed —
/// users must regenerate fixtures. Hashes default output + every modifier
/// variant for each registered field.
/// Format: `sf0-<16 hex digits>`.
pub fn fingerprint() -> String {
    use field::REGISTRY;

    const CANONICAL_SEED: &str = "__determinism__";
    let master = hash_seed(CANONICAL_SEED);
    let locales: Vec<&locale::Locale> = locale::get("en").into_iter().collect();

    let since = temporal::DEFAULT_SINCE;
    let until = temporal::date_to_epoch(2038, 1, 1, 0, 0, 0);

    let mut buf = String::new();
    let mut val_buf = String::new();

    let mut hash_field = |f: &field::Field, modifier: &str| {
        let domain =
            if modifier.is_empty() { f.id.to_string() } else { format!("{}_{modifier}", f.id) };
        let mut ctx = ctx::GenContext {
            rng: rng::Rng::derive(master, 0, &domain),
            locales: &locales,
            modifier,
            identity: None,
            tz_offset_minutes: DEFAULT_TZ_OFFSET,
            since,
            until,
            range: None,
            ordering: field::Ordering::None,
            zipf: None,
            numeric: None,
        };
        val_buf.clear();
        f.generate(&mut ctx, &mut val_buf);
        buf.push_str(&val_buf);
        buf.push('\0');
    };

    for f in REGISTRY {
        hash_field(f, "");
        let mods = field::field_modifiers(f.id);
        if !mods.is_empty() {
            for m in mods.split(", ") {
                hash_field(f, m);
            }
        }
    }

    let h = fnv1a(buf.as_bytes());
    format!("sf0-{h:016x}")
}
