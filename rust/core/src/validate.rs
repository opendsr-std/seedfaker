//! Declarative validation rules for field combinations and parameters.
//!
//! Each rule is an isolated pure function: `&CheckCtx -> Option<&str>`.
//! Rules cannot panic, mutate state, or depend on each other.
//! Add a rule = add one entry to RULES. Remove = remove one entry.

use crate::field::Ordering;

pub enum Severity {
    Error,
    Warn,
}

pub struct Rule {
    pub id: &'static str,
    pub severity: Severity,
    pub check: fn(&CheckCtx<'_>) -> Option<&'static str>,
}

pub struct CheckCtx<'a> {
    pub fields: &'a [FieldInfo<'a>],
    pub ctx_strict: bool,
    pub since: i64,
    pub until: i64,
    pub has_seed: bool,
    pub has_until: bool,
    pub format: Option<&'a str>,
    pub corrupt: Option<&'a str>,
    pub has_template: bool,
}

pub struct FieldInfo<'a> {
    pub name: &'a str,
    pub has_range: bool,
    pub resolved_range: Option<(i64, i64)>,
    pub ordering: Ordering,
}

impl CheckCtx<'_> {
    pub fn has(&self, name: &str) -> bool {
        self.fields.iter().any(|f| f.name == name)
    }

    pub fn has_range(&self, name: &str) -> bool {
        self.fields.iter().any(|f| f.name == name && f.has_range)
    }

    pub fn has_ordering(&self, name: &str) -> bool {
        self.fields.iter().any(|f| f.name == name && f.ordering != Ordering::None)
    }

    /// Get resolved age range (min, max) if age field has explicit range.
    pub fn age_range(&self) -> Option<(i64, i64)> {
        self.fields
            .iter()
            .find(|f| f.name == "age" && f.has_range)
            .map(|f| f.resolved_range.unwrap_or((0, 120)))
    }
}

pub const RULES: &[Rule] = &[
    Rule {
        id: "age-range-ctx-birthdate",
        severity: Severity::Error,
        check: |c| {
            if !c.ctx_strict || !c.has("age") || !c.has("birthdate") {
                return None;
            }
            // ctx strict + age + birthdate: age is derived from birthdate.
            // If age has range, check if it's compatible with possible birth years.
            let Some(age_range) = c.age_range() else {
                return None; // no range on age → OK, age derived from birthdate
            };
            // Implied birth years from age range: until - age_max .. until - age_min
            let implied_birth_from = c.until - age_range.1;
            let implied_birth_to = c.until - age_range.0;
            // Actual possible birth years (from since or ~100 years back)
            let actual_birth_from = c.since;
            let actual_birth_to = c.until;
            // Check overlap
            if implied_birth_to < actual_birth_from || implied_birth_from > actual_birth_to {
                Some(
                    "age range impossible with current year parameters; \
                     age is derived from birthdate in --ctx strict and \
                     the implied birth years have no overlap with year range",
                )
            } else {
                // Overlap exists but age range will override derivation
                Some(
                    "age range with birthdate in --ctx strict: \
                     age is derived from birthdate — range on age is ignored. \
                     Remove birthdate to use age range, or remove range from age",
                )
            }
        },
    },
    Rule {
        id: "ordering-numeric-only",
        severity: Severity::Error,
        check: |c| {
            const ORDERABLE: &[&str] =
                &["integer", "float", "amount", "timestamp", "date", "age", "latency", "digits"];
            for f in c.fields {
                if f.ordering != Ordering::None && !ORDERABLE.contains(&f.name) {
                    return Some("asc/desc only supported on numeric and temporal fields");
                }
            }
            None
        },
    },
    Rule {
        id: "year-range-sanity",
        severity: Severity::Error,
        check: |c| {
            if c.since > c.until {
                Some("--since must be <= --until")
            } else {
                None
            }
        },
    },
    Rule {
        id: "format-template-conflict",
        severity: Severity::Error,
        check: |c| {
            if c.format.is_some() && c.has_template {
                Some("use --format or --template, not both")
            } else {
                None
            }
        },
    },
    Rule {
        id: "format-unknown",
        severity: Severity::Error,
        check: |c| match c.format {
            Some(s) if s != "csv" && s != "tsv" && s != "jsonl" && !s.starts_with("sql=") => {
                Some("unknown format; expected: csv, tsv, jsonl, sql=TABLE")
            }
            Some("sql=") => Some("sql= requires a table name"),
            _ => None,
        },
    },
    Rule {
        id: "corrupt-unknown",
        severity: Severity::Error,
        check: |c| match c.corrupt {
            Some(s) if crate::script::Corrupt::parse_level(s).is_none() => {
                Some("unknown corrupt level; expected: low, mid, high, extreme")
            }
            _ => None,
        },
    },
    Rule {
        id: "seed-without-until",
        severity: Severity::Warn,
        check: |c| {
            if c.has_seed && !c.has_until {
                Some("--seed without --until uses system year; pin --until for reproducibility")
            } else {
                None
            }
        },
    },
];

#[derive(Default)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn validate(ctx: &CheckCtx<'_>) -> ValidationResult {
    let mut result = ValidationResult::default();
    for rule in RULES {
        if let Some(msg) = (rule.check)(ctx) {
            let formatted = format!("[{}] {}", rule.id, msg);
            match rule.severity {
                Severity::Error => result.errors.push(formatted),
                Severity::Warn => result.warnings.push(formatted),
            }
        }
    }
    result
}
