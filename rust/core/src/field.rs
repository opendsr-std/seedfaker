use crate::ctx::GenContext;

#[path = "field_gen.rs"]
mod field_gen;

pub use field_gen::{field_capabilities, field_modifiers, GROUPS, REGISTRY};

pub type GenFn = for<'a> fn(&mut GenContext<'a>, &mut String);

/// Zipf distribution parameter for a field (e.g. `integer:1..1000:zipf` or `integer:1..1000:zipf=0.8`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZipfSpec {
    /// Exponent (s). Default 1.0, must be > 0.
    pub s: f64,
}

impl ZipfSpec {
    pub const DEFAULT: Self = Self { s: 1.0 };
}

/// Parsed field specification.
pub type ParsedSpec<'a> =
    (&'a str, &'a str, Transform, Option<RangeSpec>, Ordering, Option<u8>, Option<ZipfSpec>);

pub struct Field {
    pub id: &'static str,
    pub name: &'static str,
    pub group: &'static str,
    pub description: &'static str,
    pub gen: GenFn,
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("group", &self.group)
            .field("description", &self.description)
            .finish_non_exhaustive()
    }
}

impl Field {
    pub const fn new(
        id: &'static str,
        name: &'static str,
        group: &'static str,
        description: &'static str,
        gen: GenFn,
    ) -> Self {
        Self { id, name, group, description, gen }
    }

    #[inline]
    pub fn generate(&self, ctx: &mut GenContext<'_>, buf: &mut String) -> Option<f64> {
        ctx.numeric = None;
        (self.gen)(ctx, buf);
        ctx.numeric
    }
}

/// Per-field range constraint (e.g. `integer:1..100`, `date:2020..2025`).
/// `None` bounds are resolved later using field defaults or global `since`/`until`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeSpec {
    pub from: Option<i64>,
    pub to: Option<i64>,
}

/// Fields that support the range syntax.
const RANGE_FIELDS: &[&str] =
    &["integer", "float", "amount", "date", "birthdate", "timestamp", "age", "digits"];

fn parse_range(s: &str) -> Result<RangeSpec, String> {
    if let Some(to_str) = s.strip_prefix("..") {
        let to = to_str.parse::<i64>().map_err(|_| format!("invalid range bound: '{to_str}'"))?;
        Ok(RangeSpec { from: None, to: Some(to) })
    } else if let Some(from_str) = s.strip_suffix("..") {
        let from =
            from_str.parse::<i64>().map_err(|_| format!("invalid range bound: '{from_str}'"))?;
        Ok(RangeSpec { from: Some(from), to: None })
    } else if let Some((from_str, to_str)) = s.split_once("..") {
        let from =
            from_str.parse::<i64>().map_err(|_| format!("invalid range bound: '{from_str}'"))?;
        let to = to_str.parse::<i64>().map_err(|_| format!("invalid range bound: '{to_str}'"))?;
        if from >= to {
            return Err(format!("invalid range: {from}..{to}"));
        }
        Ok(RangeSpec { from: Some(from), to: Some(to) })
    } else {
        Err(format!("invalid range: '{s}'"))
    }
}

fn is_range_segment(s: &str) -> bool {
    s.contains("..")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transform {
    None,
    Upper,
    Lower,
    Capitalize,
}

const TRANSFORMS: &[(&str, Transform)] = &[
    ("upper", Transform::Upper),
    ("lower", Transform::Lower),
    ("capitalize", Transform::Capitalize),
];

fn parse_transform(s: &str) -> Option<Transform> {
    TRANSFORMS.iter().find(|&&(k, _)| k == s).map(|&(_, t)| t)
}

impl Transform {
    pub fn apply(self, s: &str) -> String {
        match self {
            Transform::None => s.to_string(),
            Transform::Upper => s.to_uppercase(),
            Transform::Lower => s.to_lowercase(),
            Transform::Capitalize => {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => {
                        let mut out = c.to_uppercase().to_string();
                        for ch in chars {
                            out.extend(ch.to_lowercase());
                        }
                        out
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ordering {
    None,
    Asc,
    Desc,
}

pub struct ResolvedField {
    pub field: &'static Field,
    pub modifier: String,
    pub transform: Transform,
    pub range: Option<RangeSpec>,
    pub ordering: Ordering,
    /// Explicit column name from `name=field` syntax. Overrides `display_name()`.
    pub alias: Option<String>,
    /// Percentage of rows where this field is omitted (outputs NULL). 0–100.
    pub omit_pct: Option<u8>,
    /// Zipf distribution over the range instead of uniform.
    pub zipf: Option<ZipfSpec>,
}

impl ResolvedField {
    /// Column name: explicit alias if set, otherwise auto-derived.
    pub fn column_name(&self) -> String {
        if let Some(ref a) = self.alias {
            return a.clone();
        }
        self.display_name()
    }

    /// Auto-derived name for CSV/JSON headers.
    pub fn display_name(&self) -> String {
        let base = self.field.name.replace('-', "_");
        if self.modifier.is_empty() {
            base
        } else {
            format!("{base}_{}", self.modifier)
        }
    }

    /// Stable domain key for RNG derivation — uses field id, never changes.
    pub fn domain_key(&self) -> String {
        if self.modifier.is_empty() {
            self.field.id.to_string()
        } else {
            format!("{}_{}", self.field.id, self.modifier)
        }
    }
}

pub fn lookup(name: &str) -> Option<&'static Field> {
    REGISTRY.iter().find(|f| f.name == name)
}

pub fn all_names() -> Vec<&'static str> {
    REGISTRY.iter().map(|f| f.name).collect()
}

fn is_group(name: &str) -> bool {
    name == "all" || GROUPS.contains(&name)
}

fn expand_group(name: &str) -> Vec<ResolvedField> {
    let fields: Vec<&Field> = if name == "all" {
        REGISTRY.iter().collect()
    } else {
        REGISTRY.iter().filter(|f| f.group == name).collect()
    };
    fields
        .into_iter()
        .map(|f| ResolvedField {
            field: f,
            modifier: String::new(),
            transform: Transform::None,
            range: None,
            ordering: Ordering::None,
            alias: None,
            omit_pct: None,
            zipf: None,
        })
        .collect()
}

/// Fields that accept a numeric modifier as length (digits:4, hex:8, etc.).
const LENGTH_FIELDS: &[&str] = &["digits", "letters", "alnum", "base64", "hex", "password"];

fn validate_modifier(field: &Field, m: &str) -> Result<(), String> {
    if m.is_empty() {
        return Ok(());
    }
    if !m.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Err(format!("modifier '{m}' must contain only a-z, 0-9 and -"));
    }
    // Numeric length modifier for text fields (digits:4, hex:8)
    if LENGTH_FIELDS.contains(&field.name) && m.parse::<usize>().is_ok() {
        return Ok(());
    }
    let known = field_modifiers(field.id);
    if !known.is_empty() {
        let valid: Vec<&str> = known.split(", ").collect();
        if !valid.contains(&m) {
            return Err(format!("unknown modifier '{}:{m}'; available: {known}", field.name));
        }
    } else if parse_transform(m).is_none() {
        return Err(format!(
            "field '{}' has no modifiers; did you mean a transform? available: upper, lower, capitalize",
            field.name
        ));
    }
    Ok(())
}

fn parse_ordering(s: &str) -> Option<Ordering> {
    match s {
        "asc" => Some(Ordering::Asc),
        "desc" => Some(Ordering::Desc),
        _ => None,
    }
}

fn parse_zipf(s: &str) -> Option<Result<ZipfSpec, String>> {
    if s == "zipf" {
        return Some(Ok(ZipfSpec::DEFAULT));
    }
    let rest = s.strip_prefix("zipf=")?;
    let v: f64 = match rest.parse() {
        Ok(v) => v,
        Err(_) => return Some(Err(format!("invalid zipf exponent: '{rest}'"))),
    };
    if v <= 0.0 || !v.is_finite() {
        return Some(Err(format!("zipf exponent must be > 0, got {v}")));
    }
    Some(Ok(ZipfSpec { s: v }))
}

pub fn parse_field_spec(token: &str) -> Result<ParsedSpec<'_>, String> {
    let mut parts = token.splitn(8, ':');
    let name = parts.next().unwrap_or("");
    let mut modifier: Option<&str> = None;
    let mut transform = Transform::None;
    let mut range: Option<RangeSpec> = None;
    let mut ordering = Ordering::None;
    let mut omit_pct: Option<u8> = None;
    let mut zipf: Option<ZipfSpec> = None;

    for seg in parts {
        if let Some(result) = parse_zipf(seg) {
            if zipf.is_some() {
                return Err("duplicate zipf in field descriptor".into());
            }
            zipf = Some(result?);
        } else if let Some(pct) = parse_omit_pct(seg) {
            if omit_pct.is_some() {
                return Err("duplicate omit in field descriptor".into());
            }
            omit_pct = Some(pct);
        } else if is_range_segment(seg) {
            if range.is_some() {
                return Err("duplicate range in field descriptor".into());
            }
            range = Some(parse_range(seg)?);
        } else if let Some(t) = parse_transform(seg) {
            if transform != Transform::None {
                return Err("duplicate transform in field descriptor".into());
            }
            transform = t;
        } else if let Some(o) = parse_ordering(seg) {
            if ordering != Ordering::None {
                return Err("duplicate ordering in field descriptor".into());
            }
            ordering = o;
        } else {
            if modifier.is_some() {
                return Err("duplicate modifier in field descriptor".into());
            }
            modifier = Some(seg);
        }
    }

    Ok((name, modifier.unwrap_or(""), transform, range, ordering, omit_pct, zipf))
}

fn parse_omit_pct(s: &str) -> Option<u8> {
    let rest = s.strip_prefix("omit=")?;
    let n: u8 = rest.parse().ok()?;
    if n > 100 {
        return None;
    }
    Some(n)
}

fn validate_range(field: &Field, range: &Option<RangeSpec>) -> Result<(), String> {
    if let Some(r) = range {
        if !RANGE_FIELDS.contains(&field.name) {
            return Err(format!("field '{}' does not support range", field.name));
        }
        if let (Some(from), Some(to)) = (r.from, r.to) {
            if from >= to {
                return Err(format!("invalid range: {from}..{to}"));
            }
        }
    }
    Ok(())
}

pub fn resolve_range(
    range: &Option<RangeSpec>,
    field_name: &str,
    since: i64,
    until: i64,
) -> Option<(i64, i64)> {
    let r = range.as_ref()?;
    let is_date = matches!(field_name, "date" | "birthdate" | "timestamp");
    let (default_min, default_max) = if is_date { (since, until) } else { (0, 999_999) };
    let from = r.from.unwrap_or(default_min);
    let to = r.to.unwrap_or(default_max);
    // For date fields: small values (<=9999) are years → convert to epoch
    if is_date {
        let from_e = if from > 0 && from <= 9999 {
            crate::temporal::parse(&from.to_string()).unwrap_or(from)
        } else {
            from
        };
        let to_e = if to > 0 && to <= 9999 {
            crate::temporal::parse_until(&to.to_string()).unwrap_or(to)
        } else {
            to
        };
        Some((from_e, to_e))
    } else {
        Some((from, to))
    }
}

pub fn resolve(tokens: &[String]) -> Result<Vec<ResolvedField>, String> {
    let mut result = Vec::new();
    for token in tokens {
        // Split on first `=` for `name=field_spec` syntax.
        let (alias, spec) = if let Some(eq_pos) = token.find('=') {
            // Exclude enum values (enum:a=3,b=1) and range (1..100)
            // by checking the `=` is before any `:` — i.e. it's a column alias.
            let colon_pos = token.find(':').unwrap_or(token.len());
            if eq_pos < colon_pos {
                let (a, s) = token.split_at(eq_pos);
                (Some(a.to_string()), &s[1..])
            } else {
                (None, token.as_str())
            }
        } else {
            (None, token.as_str())
        };

        let (name, modifier, transform, range, ordering, omit_pct, zipf) = parse_field_spec(spec)?;

        if let Some(field) = lookup(name) {
            if name == "enum" {
                super::gen::validate_enum(modifier)?;
            } else {
                validate_modifier(field, modifier)?;
                validate_range(field, &range)?;
            }
            if zipf.is_some() && range.is_none() {
                return Err(format!(
                    "field '{name}': zipf requires a range (e.g. {name}:1..1000:zipf)"
                ));
            }
            result.push(ResolvedField {
                field,
                modifier: modifier.to_string(),
                transform,
                range,
                ordering,
                alias,
                omit_pct,
                zipf,
            });
        } else if is_group(name) {
            if alias.is_some() {
                return Err(format!("alias not supported on groups: '{token}'"));
            }
            if !modifier.is_empty() || transform != Transform::None {
                return Err(format!("modifiers and transforms not supported on groups: '{token}'"));
            }
            result.extend(expand_group(name));
        } else {
            return Err(format!("unknown field or group '{name}'; run 'seedfaker --list'"));
        }
    }
    if result.is_empty() {
        return Err("no fields specified".into());
    }
    Ok(result)
}

/// Validate a single field spec string without resolving it.
/// Checks field name, modifier, range, zipf, and enum syntax.
pub fn validate_spec(spec: &str) -> Result<(), String> {
    let (name, modifier, _transform, range, _ordering, _omit_pct, zipf) = parse_field_spec(spec)?;
    let field = lookup(name).ok_or_else(|| format!("unknown field '{name}'"))?;
    if name == "enum" {
        super::gen::validate_enum(modifier)?;
    } else {
        validate_modifier(field, modifier)?;
        validate_range(field, &range)?;
    }
    if zipf.is_some() && range.is_none() {
        return Err(format!("field '{name}': zipf requires a range (e.g. {name}:1..1000:zipf)"));
    }
    Ok(())
}

/// Validate a batch of field spec strings.
pub fn validate_specs(specs: &[String]) -> Result<(), String> {
    for spec in specs {
        validate_spec(spec)?;
    }
    Ok(())
}
