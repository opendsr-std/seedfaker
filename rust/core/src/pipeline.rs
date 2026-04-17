use crate::ctx::{GenContext, Identity};
use crate::field::{self, Field, Transform};
use crate::locale::Locale;
use crate::rng::Rng;
use crate::script::Ctx;
use crate::{DOMAIN_CORRUPT, DOMAIN_IDENTITY, DOMAIN_LOCALE};

/// Per-field specification for batch generation.
pub struct FieldSpec<'a> {
    pub field: &'static Field,
    pub modifier: &'a str,
    pub domain_hash: u64,
    pub range: Option<(i64, i64)>,
    pub transform: Transform,
    pub omit_pct: Option<u8>,
}

/// Options shared across all records in a batch.
pub struct RecordOpts<'a> {
    pub master_seed: u64,
    pub locales: &'a [&'a Locale],
    pub ctx: Ctx,
    pub corrupt_rate: Option<f64>,
    pub tz_offset_minutes: i32,
    pub since: i64,
    pub until: i64,
}

/// Compute domain hash for a field + modifier combination.
pub fn field_domain_hash(master_seed: u64, field: &Field, modifier: &str) -> u64 {
    let domain =
        if modifier.is_empty() { field.id.to_string() } else { format!("{}_{modifier}", field.id) };
    crate::rng::domain_hash(master_seed, &domain)
}

/// Generate `n` records starting from `start_serial`.
///
/// Handles locale locking (ctx strict/loose), identity creation,
/// per-field generation, transforms, and corruption.
///
/// This is the canonical pipeline for batch generation in bindings
/// (`PyO3`, NAPI, FFI) and MCP. The CLI engine uses its own optimized
/// loop with streaming output, aggregators, and template rendering.
pub fn generate_records(
    opts: &RecordOpts<'_>,
    specs: &[FieldSpec<'_>],
    n: u64,
    start_serial: u64,
) -> Vec<Vec<String>> {
    let needs_ctx = opts.ctx != Ctx::None;
    let mut records = Vec::with_capacity(n as usize);

    for i in 0..n {
        let serial = start_serial + i;

        let locked_locale: Option<&Locale> = match opts.ctx {
            Ctx::Strict => {
                let mut lr = Rng::derive(opts.master_seed, serial, DOMAIN_LOCALE);
                Some(*lr.choice(opts.locales))
            }
            Ctx::Loose => {
                let mut lr = Rng::derive(opts.master_seed, serial, DOMAIN_LOCALE);
                if lr.maybe(0.7) {
                    Some(*lr.choice(opts.locales))
                } else {
                    None
                }
            }
            Ctx::None => None,
        };
        let locked_arr: [&Locale; 1];
        let effective_locales: &[&Locale] = if let Some(loc) = locked_locale {
            locked_arr = [loc];
            &locked_arr
        } else {
            opts.locales
        };

        let identity = if needs_ctx {
            let mut ir = Rng::derive(opts.master_seed, serial, DOMAIN_IDENTITY);
            Some(Identity::new(&mut ir, effective_locales, None, opts.since, opts.until))
        } else {
            None
        };

        let mut values: Vec<String> = specs
            .iter()
            .map(|spec| {
                if let Some(pct) = spec.omit_pct {
                    let mut or = Rng::derive(spec.domain_hash, serial, "omit");
                    if or.range(0, 100) < i64::from(pct) {
                        return String::new();
                    }
                }
                let mut ctx = GenContext {
                    rng: Rng::derive_fast(spec.domain_hash, serial),
                    locales: effective_locales,
                    modifier: spec.modifier,
                    identity: identity.as_ref(),
                    tz_offset_minutes: opts.tz_offset_minutes,
                    since: opts.since,
                    until: opts.until,
                    range: spec.range,
                    ordering: field::Ordering::None,
                    zipf: None,
                    numeric: None,
                };
                let mut buf = String::new();
                spec.field.generate(&mut ctx, &mut buf);
                if spec.transform == Transform::None {
                    buf
                } else {
                    spec.transform.apply(&buf)
                }
            })
            .collect();

        if let Some(rate) = opts.corrupt_rate {
            let mut cr = Rng::derive(opts.master_seed, serial, DOMAIN_CORRUPT);
            crate::corrupt::corrupt_values(&mut cr, &mut values, rate);
        }

        records.push(values);
    }

    records
}

/// Generate N values for a single parsed field spec.
///
/// Used by binding `field()` methods to avoid duplicating the generation loop.
pub fn generate_field_values(
    spec: &FieldSpec<'_>,
    n: usize,
    record_counter: &mut u64,
    locales: &[&Locale],
    tz_offset_minutes: i32,
    since: i64,
    until: i64,
) -> Vec<String> {
    (0..n)
        .map(|_| {
            let serial = *record_counter;
            *record_counter += 1;
            if let Some(pct) = spec.omit_pct {
                let mut or = Rng::derive(spec.domain_hash, serial, "omit");
                if or.range(0, 100) < i64::from(pct) {
                    return String::new();
                }
            }
            let mut ctx = GenContext {
                rng: Rng::derive_fast(spec.domain_hash, serial),
                locales,
                modifier: spec.modifier,
                identity: None,
                tz_offset_minutes,
                since,
                until,
                range: spec.range,
                ordering: field::Ordering::None,
                zipf: None,
                numeric: None,
            };
            let mut buf = String::new();
            spec.field.generate(&mut ctx, &mut buf);
            if spec.transform == Transform::None {
                buf
            } else {
                spec.transform.apply(&buf)
            }
        })
        .collect()
}

/// Validate field specs and options without generating data.
pub fn validate(fields: &[String], ctx: Option<&str>, corrupt: Option<&str>) -> Result<(), String> {
    crate::field::validate_specs(fields)?;
    if let Some(c) = ctx {
        match c {
            "strict" | "loose" => {}
            other => return Err(format!("unknown ctx mode: '{other}'")),
        }
    }
    if let Some(c) = corrupt {
        if crate::script::Corrupt::parse_level(c).is_none() {
            return Err(format!("unknown corrupt level: '{c}'"));
        }
    }
    Ok(())
}
