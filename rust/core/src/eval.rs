//! Column resolution, expressions, aggregators, topological sort.
//!
//! Pure computation — no IO, no CLI, no serde. Used by CLI engine and all bindings.

use std::collections::HashMap;
use std::fmt::Write;

use crate::field::{self, Field, Ordering, RangeSpec, Transform, ZipfSpec};
use crate::rng::Rng;

// ═══════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone)]
pub struct Column {
    pub name: String,
    pub gen: ColumnGen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggrFunc {
    Sum,
    Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
}

#[derive(Clone)]
pub enum ExprOperand {
    Col(String),
    Field { field: &'static Field, modifier: String, range: Option<RangeSpec> },
}

impl std::fmt::Debug for ExprOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Col(name) => write!(f, "Col({name})"),
            Self::Field { field, modifier, range } => {
                write!(f, "Field({}", field.name)?;
                if let Some(r) = range {
                    write!(f, ":{r:?}")?;
                }
                if !modifier.is_empty() {
                    write!(f, ":{modifier}")?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Per-table context for reconstructing a parent row's generation environment.
/// Stored inside `Fk` and `FkDeref` variants. Zero-initialized at parse time;
/// filled by `finalize_fk_columns` before generation.
#[derive(Clone, Default)]
pub struct ParentCtx {
    /// `domain_hash(global_seed, parent_table_name)` — used for Identity / locale derivation.
    pub table_seed: u64,
    pub locales: Vec<&'static crate::locale::Locale>,
    pub script: crate::script::Script,
    pub ctx: crate::script::Ctx,
    pub tz_offset_minutes: i32,
    pub since: i64,
    pub until: i64,
    pub parent_count: u64,
}

#[derive(Clone, Debug)]
pub enum FkDistribution {
    Uniform,
    Zipf(f64),
}

#[derive(Clone)]
pub enum ColumnGen {
    Field {
        field: &'static Field,
        modifier: String,
        transform: Transform,
        range: Option<RangeSpec>,
        ordering: Ordering,
        omit_pct: Option<u8>,
        zipf: Option<ZipfSpec>,
    },
    Literal(String),
    Aggr {
        func: AggrFunc,
        source_col: String,
        group_by: Option<String>,
    },
    Ref {
        source_col: String,
        modifier: String,
    },
    Expr {
        left: ExprOperand,
        op: ExprOp,
        right: ExprOperand,
        result_type: ExprResultType,
    },
    /// Foreign-key anchor: samples a row from the parent table and generates the parent field value.
    /// `parent_domain_hash` is zero at parse time; set by `finalize_fk_columns`.
    Fk {
        parent_table: String,
        parent_col_name: String,
        parent_field: &'static Field,
        parent_modifier: String,
        parent_range: Option<RangeSpec>,
        /// Ordering of the parent column (`asc`/`desc`/`none`). Propagated into
        /// the `GenContext` used to regenerate the parent field value, otherwise
        /// monotonic fields like `timestamp:asc` fall back to random output.
        parent_ordering: Ordering,
        parent_count: u64,
        distribution: FkDistribution,
        /// Pre-computed hash of the parent field; set at orchestration time.
        parent_domain_hash: u64,
        parent_ctx: Box<ParentCtx>,
    },
    /// Foreign-key dereference: reuses the row index sampled by an `Fk` anchor column
    /// in this table to generate a different field of the same parent row.
    /// `deref_domain_hash` is zero at parse time; set by `finalize_fk_columns`.
    FkDeref {
        anchor_col: String,
        deref_col_name: String,
        deref_field: &'static Field,
        deref_modifier: String,
        deref_range: Option<RangeSpec>,
        /// Ordering of the dereferenced parent column — same role as `parent_ordering`.
        deref_ordering: Ordering,
        /// Pre-computed hash of the dereferenced parent field; set at orchestration time.
        deref_domain_hash: u64,
        parent_ctx: Box<ParentCtx>,
    },
}

impl std::fmt::Debug for ColumnGen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Field { field, modifier, transform, range, .. } => {
                write!(f, "Field({}", field.name)?;
                if let Some(r) = range {
                    write!(f, ":{r:?}")?;
                }
                if !modifier.is_empty() {
                    write!(f, ":{modifier}")?;
                }
                if *transform != Transform::None {
                    write!(f, ":{transform:?}")?;
                }
                write!(f, ")")
            }
            Self::Literal(s) => write!(f, "Literal({s:?})"),
            Self::Aggr { func, source_col, group_by } => {
                write!(f, "Aggr({func:?}({source_col}")?;
                if let Some(g) = group_by {
                    write!(f, ", {g}")?;
                }
                write!(f, "))")
            }
            Self::Ref { source_col, modifier } => {
                write!(f, "Ref({source_col}")?;
                if !modifier.is_empty() {
                    write!(f, ":{modifier}")?;
                }
                write!(f, ")")
            }
            Self::Expr { left, op, right, result_type } => {
                write!(f, "Expr({left:?} {op:?} {right:?} -> {result_type:?})")
            }
            Self::Fk { parent_table, parent_field, parent_modifier, .. } => {
                write!(f, "Fk({parent_table}.{}", parent_field.name)?;
                if !parent_modifier.is_empty() {
                    write!(f, ":{parent_modifier}")?;
                }
                write!(f, ")")
            }
            Self::FkDeref { anchor_col, deref_field, deref_modifier, .. } => {
                write!(f, "FkDeref({anchor_col}->{}", deref_field.name)?;
                if !deref_modifier.is_empty() {
                    write!(f, ":{deref_modifier}")?;
                }
                write!(f, ")")
            }
        }
    }
}

pub struct ColumnSet {
    columns: Vec<Column>,
}

impl ColumnSet {
    pub fn new(columns: Vec<Column>) -> Self {
        Self { columns }
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Column> {
        self.columns.iter()
    }

    pub fn names(&self) -> Vec<&str> {
        self.columns.iter().map(|v| v.name.as_str()).collect()
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|v| v.name == name)
    }
}

impl<'a> IntoIterator for &'a ColumnSet {
    type Item = &'a Column;
    type IntoIter = std::slice::Iter<'a, Column>;

    fn into_iter(self) -> Self::IntoIter {
        self.columns.iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    Int,
    Float,
    Money,
    Date,
    Timestamp,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExprResultType {
    Int,
    Float,
    Money,
    Date,
    Timestamp,
}

// ═══════════════════════════════════════════════════════════════════
// Column resolution
// ═══════════════════════════════════════════════════════════════════

pub fn field_type(name: &str) -> FieldType {
    match name {
        "integer" | "age" | "serial" | "digit" | "bit" | "trit" | "dice" | "port" | "latency" => {
            FieldType::Int
        }
        "float" | "latitude" | "longitude" => FieldType::Float,
        "amount" => FieldType::Money,
        "date" | "birthdate" => FieldType::Date,
        "timestamp" => FieldType::Timestamp,
        _ => FieldType::Text,
    }
}

pub fn check_expr_types(
    left: FieldType,
    op: ExprOp,
    right: FieldType,
) -> Result<ExprResultType, &'static str> {
    use ExprOp::{Add, Mul, Sub};
    use FieldType::{Date, Float, Int, Money, Text, Timestamp};
    match (left, op, right) {
        (Int, Add | Sub | Mul, Int) => Ok(ExprResultType::Int),
        (Float, Add | Sub | Mul, Float | Int) | (Int, Add | Sub | Mul, Float) => {
            Ok(ExprResultType::Float)
        }
        (Money, Add | Sub, Money) => Ok(ExprResultType::Money),
        (Money, Mul, Money) => Err("cannot multiply money by money"),
        (Money, Add | Sub | Mul, Int | Float) | (Int | Float, Mul, Money) => {
            Ok(ExprResultType::Money)
        }
        (Int | Float, Add | Sub, Money) => {
            Err("cannot add/subtract int and money; put money on the left")
        }
        (Date, Add | Sub, Int) => Ok(ExprResultType::Date),
        (Timestamp, Add | Sub, Int) => Ok(ExprResultType::Timestamp),
        (Date | Timestamp, Mul, _) => Err("cannot multiply dates or timestamps"),
        (Date | Timestamp, _, Float) => {
            Err("date/timestamp arithmetic requires integer (whole days or seconds)")
        }
        (Date | Timestamp, _, Money) => Err("cannot combine date/timestamp with money"),
        (Date | Timestamp, _, Date | Timestamp) => Err("cannot combine two date/timestamp values"),
        (Text, _, _) | (_, _, Text) => Err("field does not support arithmetic"),
        _ => Err("incompatible types for arithmetic"),
    }
}

const RESERVED_COLUMNS: &[(&str, &str)] =
    &[("serial", "built-in: 0-based record counter, available as {{serial}} in templates")];

pub fn resolve_column(
    col_name: &str,
    value: &str,
    all_columns: &[String],
) -> Result<ColumnGen, String> {
    for (reserved, description) in RESERVED_COLUMNS {
        if col_name == *reserved {
            return Err(format!(
                "column '{col_name}' is reserved ({description}); remove it from columns"
            ));
        }
    }

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("column '{col_name}': empty value"));
    }

    if let Some(expr) = parse_expr(col_name, trimmed, all_columns)? {
        return Ok(expr);
    }

    if let Some(aggr) = parse_aggr_spec(trimmed) {
        return Ok(aggr);
    }

    if trimmed != col_name && all_columns.iter().any(|d| d == trimmed) {
        return Ok(ColumnGen::Ref { source_col: trimmed.to_string(), modifier: String::new() });
    }
    if let Some(colon) = trimmed.find(':') {
        let base = &trimmed[..colon];
        let modifier = &trimmed[colon + 1..];
        if base != col_name && !modifier.is_empty() && all_columns.iter().any(|d| d == base) {
            return Ok(ColumnGen::Ref {
                source_col: base.to_string(),
                modifier: modifier.to_string(),
            });
        }
    }

    resolve_field_spec(col_name, trimmed)
}

fn parse_expr(
    col_name: &str,
    value: &str,
    all_columns: &[String],
) -> Result<Option<ColumnGen>, String> {
    for (ch, op) in &[('+', ExprOp::Add), ('*', ExprOp::Mul)] {
        if let Some(pos) = value.find(*ch) {
            let left_str = value[..pos].trim();
            let right_str = value[pos + 1..].trim();
            if left_str.is_empty() || right_str.is_empty() {
                continue;
            }
            return build_expr(col_name, left_str, *op, right_str, all_columns).map(Some);
        }
    }

    if value.contains('-') {
        let is_field = field::parse_field_spec(value)
            .ok()
            .and_then(|(name, ..)| field::lookup(name))
            .is_some();
        if !is_field {
            for (pos, _) in value.match_indices('-') {
                let left_str = value[..pos].trim();
                let right_str = value[pos + 1..].trim();
                if left_str.is_empty() || right_str.is_empty() {
                    continue;
                }
                if is_valid_operand(left_str, all_columns)
                    && is_valid_operand(right_str, all_columns)
                {
                    return build_expr(col_name, left_str, ExprOp::Sub, right_str, all_columns)
                        .map(Some);
                }
            }
        }
    }

    Ok(None)
}

fn build_expr(
    col_name: &str,
    left_str: &str,
    op: ExprOp,
    right_str: &str,
    all_columns: &[String],
) -> Result<ColumnGen, String> {
    let (left, left_type) = resolve_operand(col_name, left_str, all_columns)?;
    let (right, right_type) = resolve_operand(col_name, right_str, all_columns)?;

    let lt = if let ExprOperand::Col(_) = &left { FieldType::Int } else { left_type };
    let rt = if let ExprOperand::Col(_) = &right { FieldType::Int } else { right_type };

    let result_type =
        if matches!(&left, ExprOperand::Col(_)) || matches!(&right, ExprOperand::Col(_)) {
            ExprResultType::Int
        } else {
            check_expr_types(lt, op, rt).map_err(|e| format!("column '{col_name}': {e}"))?
        };

    Ok(ColumnGen::Expr { left, op, right, result_type })
}

pub fn resolve_col_field_type(col_name: &str, columns: &[Column]) -> FieldType {
    for col in columns {
        if col.name == col_name {
            return match &col.gen {
                ColumnGen::Field { field, .. } => field_type(field.name),
                ColumnGen::Expr { result_type, .. } => match result_type {
                    ExprResultType::Int => FieldType::Int,
                    ExprResultType::Float => FieldType::Float,
                    ExprResultType::Money => FieldType::Money,
                    ExprResultType::Date => FieldType::Date,
                    ExprResultType::Timestamp => FieldType::Timestamp,
                },
                ColumnGen::Aggr { func, .. } => match func {
                    AggrFunc::Count => FieldType::Int,
                    AggrFunc::Sum => FieldType::Money,
                },
                ColumnGen::Ref { source_col, .. } => resolve_col_field_type(source_col, columns),
                ColumnGen::Literal(_) => FieldType::Text,
                ColumnGen::Fk { parent_field, .. } => field_type(parent_field.name),
                ColumnGen::FkDeref { deref_field, .. } => field_type(deref_field.name),
            };
        }
    }
    FieldType::Int
}

fn is_valid_operand(spec: &str, all_columns: &[String]) -> bool {
    if all_columns.iter().any(|c| c == spec) {
        return true;
    }
    if let Ok((name, ..)) = field::parse_field_spec(spec) {
        return field::lookup(name).is_some();
    }
    false
}

fn resolve_operand(
    col_name: &str,
    spec: &str,
    all_columns: &[String],
) -> Result<(ExprOperand, FieldType), String> {
    if all_columns.iter().any(|c| c == spec) {
        return Ok((ExprOperand::Col(spec.to_string()), FieldType::Int));
    }

    let (name, modifier, _transform, range, _ordering, _omit_pct, _zipf) =
        field::parse_field_spec(spec).map_err(|e| format!("column '{col_name}': {e}"))?;
    let f = field::lookup(name)
        .ok_or_else(|| format!("column '{col_name}': unknown field or column '{name}'"))?;
    let ft = field_type(name);
    if ft == FieldType::Text {
        return Err(format!("column '{col_name}': field '{name}' does not support arithmetic"));
    }

    Ok((ExprOperand::Field { field: f, modifier: modifier.to_string(), range }, ft))
}

pub fn dependencies(gen: &ColumnGen) -> Vec<&str> {
    match gen {
        ColumnGen::Ref { source_col, .. } => vec![source_col.as_str()],
        ColumnGen::Aggr { source_col, group_by, .. } => {
            let mut deps = vec![source_col.as_str()];
            if let Some(g) = group_by {
                deps.push(g.as_str());
            }
            deps
        }
        ColumnGen::Expr { left, right, .. } => {
            let mut deps = Vec::new();
            if let ExprOperand::Col(name) = left {
                deps.push(name.as_str());
            }
            if let ExprOperand::Col(name) = right {
                deps.push(name.as_str());
            }
            deps
        }
        ColumnGen::Field { .. } | ColumnGen::Literal(_) | ColumnGen::Fk { .. } => vec![],
        ColumnGen::FkDeref { anchor_col, .. } => vec![anchor_col.as_str()],
    }
}

const AGGR_FUNCS: &[(&str, AggrFunc)] = &[("sum", AggrFunc::Sum), ("count", AggrFunc::Count)];

pub fn parse_aggr_spec(spec: &str) -> Option<ColumnGen> {
    let segments: Vec<&str> = spec.split(':').collect();
    for (seg_idx, seg) in segments.iter().enumerate() {
        let (func_name, group_part) = match seg.split_once('=') {
            Some((f, g)) => (f, Some(g)),
            None => (*seg, None),
        };
        let func = match AGGR_FUNCS.iter().find(|(n, _)| *n == func_name) {
            Some((_, f)) => *f,
            None => continue,
        };
        if seg_idx == 0 {
            return None;
        }
        let source = segments[..seg_idx].join(":");
        if source.is_empty() {
            return None;
        }
        let group_by = match (func, group_part) {
            (_, Some(g)) if !g.is_empty() => Some(g.to_string()),
            (AggrFunc::Count, _) => Some(source.clone()),
            (AggrFunc::Sum, _) => None,
        };
        return Some(ColumnGen::Aggr { func, source_col: source, group_by });
    }
    None
}

fn resolve_field_spec(col_name: &str, spec: &str) -> Result<ColumnGen, String> {
    if let Some(values) = spec.strip_prefix("enum:") {
        crate::gen::validate_enum(values).map_err(|e| format!("column '{col_name}': {e}"))?;
        let f = field::lookup("enum").ok_or_else(|| {
            format!("column '{col_name}': internal error: 'enum' field not in registry")
        })?;
        return Ok(ColumnGen::Field {
            field: f,
            modifier: values.to_string(),
            transform: Transform::None,
            range: None,
            ordering: Ordering::None,
            omit_pct: None,
            zipf: None,
        });
    }

    field::validate_spec(spec).map_err(|e| format!("column '{col_name}': {e}"))?;

    let (name, modifier, transform, range, ordering, omit_pct, zipf) =
        field::parse_field_spec(spec).map_err(|e| format!("column '{col_name}': {e}"))?;

    let f = field::lookup(name)
        .ok_or_else(|| format!("column '{col_name}': unknown field or column '{name}'"))?;
    Ok(ColumnGen::Field {
        field: f,
        modifier: modifier.to_string(),
        transform,
        range,
        ordering,
        omit_pct,
        zipf,
    })
}

// ═══════════════════════════════════════════════════════════════════
// Topological sort + expression type resolution
// ═══════════════════════════════════════════════════════════════════

pub fn topo_sort_columns(columns: &[Column]) -> Result<Vec<usize>, String> {
    let n = columns.len();
    let names: Vec<&str> = columns.iter().map(|c| c.name.as_str()).collect();

    let mut in_degree = vec![0u32; n];
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (i, col) in columns.iter().enumerate() {
        let deps = dependencies(&col.gen);
        for dep_name in deps {
            if let Some(dep_idx) = names.iter().position(|n| *n == dep_name) {
                dependents[dep_idx].push(i);
                in_degree[i] += 1;
            }
        }
    }

    let mut queue: Vec<usize> = (0..n).filter(|i| in_degree[*i] == 0).collect();
    let mut order = Vec::with_capacity(n);

    while let Some(idx) = queue.pop() {
        order.push(idx);
        for &dep in &dependents[idx] {
            in_degree[dep] -= 1;
            if in_degree[dep] == 0 {
                queue.push(dep);
            }
        }
    }

    if order.len() != n {
        let cycle: Vec<&str> = (0..n).filter(|i| in_degree[*i] > 0).map(|i| names[i]).collect();
        return Err(format!("circular dependency between columns: {}", cycle.join(", ")));
    }

    Ok(order)
}

pub fn resolve_expr_types(columns: &mut [Column]) -> Result<(), String> {
    let snap_cols: Vec<Column> = columns.to_vec();

    for col in columns.iter_mut() {
        if let ColumnGen::Expr { left, op, right, result_type } = &mut col.gen {
            let lt = operand_type(left, &snap_cols);
            let rt = operand_type(right, &snap_cols);
            *result_type =
                check_expr_types(lt, *op, rt).map_err(|e| format!("column '{}': {e}", col.name))?;
        }
    }
    Ok(())
}

fn operand_type(operand: &ExprOperand, columns: &[Column]) -> FieldType {
    match operand {
        ExprOperand::Col(name) => resolve_col_field_type(name, columns),
        ExprOperand::Field { field, .. } => field_type(field.name),
    }
}

// ═══════════════════════════════════════════════════════════════════
// Aggregator state
// ═══════════════════════════════════════════════════════════════════

struct AggrEntry {
    col_idx: usize,
    source_idx: usize,
    group_idx: Option<usize>,
    func: AggrFunc,
    global: f64,
    grouped: HashMap<u64, f64>,
}

pub struct AggrState {
    entries: Vec<AggrEntry>,
}

impl AggrState {
    pub fn new<S: AsRef<str>>(columns: &[Column], col_names: &[S]) -> Result<Self, String> {
        let mut entries = Vec::new();
        for (i, col) in columns.iter().enumerate() {
            if let ColumnGen::Aggr { func, source_col, group_by } = &col.gen {
                let source_idx =
                    col_names.iter().position(|n| n.as_ref() == source_col).ok_or_else(|| {
                        format!("aggregator source '{source_col}' not found in columns")
                    })?;
                let group_idx = group_by
                    .as_ref()
                    .map(|g| {
                        col_names
                            .iter()
                            .position(|n| n.as_ref() == g.as_str())
                            .ok_or_else(|| format!("aggregator group '{g}' not found in columns"))
                    })
                    .transpose()?;
                entries.push(AggrEntry {
                    col_idx: i,
                    source_idx,
                    group_idx,
                    func: *func,
                    global: 0.0,
                    grouped: HashMap::new(),
                });
            }
        }
        Ok(Self { entries })
    }

    pub fn update(
        &mut self,
        values: &mut [String],
        raw_values: &[Option<f64>],
    ) -> Result<(), String> {
        for entry in &mut self.entries {
            let src_raw = raw_values[entry.source_idx].ok_or_else(|| {
                format!("aggregator source column {} is not a numeric field", entry.source_idx)
            })?;
            let delta = match entry.func {
                AggrFunc::Sum => src_raw,
                AggrFunc::Count => 1.0,
            };
            let current = if let Some(gidx) = entry.group_idx {
                let key = crate::hash_seed(&values[gidx]);
                let slot = entry.grouped.entry(key).or_insert(0.0);
                *slot += delta;
                *slot
            } else {
                entry.global += delta;
                entry.global
            };
            values[entry.col_idx].clear();
            match entry.func {
                AggrFunc::Count => {
                    let _ = write!(values[entry.col_idx], "{}", current as u64);
                }
                AggrFunc::Sum => {
                    let _ = write!(values[entry.col_idx], "{current:.2}");
                }
            }
        }
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════
// Domain hashes + expression evaluation helpers
// ═══════════════════════════════════════════════════════════════════

/// Compute the domain hash for a single column, including alias detection.
/// Use this when reproducing a parent column's hash for FK recomputation.
pub fn column_domain_hash(
    master_seed: u64,
    col_name: &str,
    field: &crate::field::Field,
    modifier: &str,
) -> u64 {
    let base = crate::pipeline::field_domain_hash(master_seed, field, modifier);
    if is_alias(col_name, field.name, modifier) {
        crate::rng::domain_hash(base, col_name)
    } else {
        base
    }
}

pub fn compute_domain_hashes(columns: &[Column], master_seed: u64) -> Vec<u64> {
    columns
        .iter()
        .map(|v| match &v.gen {
            ColumnGen::Field { field, modifier, .. } => {
                column_domain_hash(master_seed, &v.name, field, modifier)
            }
            ColumnGen::Expr { .. } => {
                crate::rng::domain_hash(master_seed, &format!("_expr_{}", v.name))
            }
            ColumnGen::Literal(_)
            | ColumnGen::Aggr { .. }
            | ColumnGen::Ref { .. }
            | ColumnGen::FkDeref { .. } => 0,
            ColumnGen::Fk { .. } => {
                crate::rng::domain_hash(master_seed, &format!("_fk_sample_{}", v.name))
            }
        })
        .collect()
}

fn is_alias(col_name: &str, field_name: &str, modifier: &str) -> bool {
    if col_name == field_name {
        return false;
    }
    let normalized = field_name.replace('-', "_");
    if col_name == normalized {
        return false;
    }
    if !modifier.is_empty()
        && (col_name == format!("{field_name}_{modifier}")
            || col_name == format!("{normalized}_{modifier}")
            || col_name == format!("{field_name}:{modifier}"))
    {
        return false;
    }
    true
}

pub struct ExprEnv<'a> {
    pub raw_values: &'a [Option<f64>],
    pub col_names: &'a [String],
    pub domain_hashes: &'a [u64],
    pub serial: u64,
}

pub fn eval_operand<'a>(
    operand: &'a ExprOperand,
    env: &ExprEnv<'_>,
    ctx: &mut crate::ctx::GenContext<'a>,
    expr_idx: usize,
    is_left: bool,
) -> f64 {
    match operand {
        ExprOperand::Col(name) => env
            .col_names
            .iter()
            .position(|n| n == name)
            .and_then(|idx| env.raw_values[idx])
            .unwrap_or(0.0),
        ExprOperand::Field { field, modifier, range } => {
            let sub = if is_left { "L" } else { "R" };
            let sub_hash = crate::rng::domain_hash(env.domain_hashes[expr_idx], sub);
            ctx.rng = Rng::derive_fast(sub_hash, env.serial);
            ctx.modifier = modifier;
            ctx.range = range
                .as_ref()
                .and_then(|r| field::resolve_range(&Some(*r), field.name, ctx.since, ctx.until));
            let mut buf = String::new();
            let raw = field.generate(ctx, &mut buf);
            raw.unwrap_or(0.0)
        }
    }
}

pub fn format_raw_typed(value: f64, result_type: ExprResultType, buf: &mut String) {
    match result_type {
        ExprResultType::Date => {
            let epoch = value as i64;
            let (y, m, d, _, _, _) = crate::gen::timestamp::epoch_to_parts(epoch);
            let _ = write!(buf, "{y:04}-{m:02}-{d:02}");
        }
        ExprResultType::Timestamp => {
            let epoch = value as i64;
            let (y, m, d, h, min, s) = crate::gen::timestamp::epoch_to_parts(epoch);
            let _ = write!(buf, "{y:04}-{m:02}-{d:02}T{h:02}:{min:02}:{s:02}Z");
        }
        ExprResultType::Money | ExprResultType::Float => {
            let _ = write!(buf, "{value:.2}");
        }
        ExprResultType::Int => {
            let mut tmp = itoa::Buffer::new();
            buf.push_str(tmp.format(value as i64));
        }
    }
}

pub fn format_ref(raw: f64, modifier: &str, columns: &[Column], src_idx: usize, buf: &mut String) {
    let ft = match &columns[src_idx].gen {
        ColumnGen::Field { field, .. } => field_type(field.name),
        ColumnGen::Expr { result_type, .. } => match result_type {
            ExprResultType::Date => FieldType::Date,
            ExprResultType::Timestamp => FieldType::Timestamp,
            ExprResultType::Money => FieldType::Money,
            ExprResultType::Float => FieldType::Float,
            ExprResultType::Int => FieldType::Int,
        },
        _ => FieldType::Int,
    };

    match ft {
        FieldType::Date => {
            let epoch = raw as i64;
            let (y, m, d, _, _, _) = crate::gen::timestamp::epoch_to_parts(epoch);
            match modifier {
                "us" => {
                    crate::gen::date::push_pad2(buf, m);
                    buf.push('/');
                    crate::gen::date::push_pad2(buf, d);
                    buf.push('/');
                    buf.push_str(itoa::Buffer::new().format(y));
                }
                "eu" => {
                    crate::gen::date::push_pad2(buf, d);
                    buf.push('.');
                    crate::gen::date::push_pad2(buf, m);
                    buf.push('.');
                    buf.push_str(itoa::Buffer::new().format(y));
                }
                _ => format_raw_typed(raw, ExprResultType::Date, buf),
            }
        }
        FieldType::Timestamp => {
            let epoch = raw as i64;
            match modifier {
                "unix" => buf.push_str(itoa::Buffer::new().format(epoch)),
                "ms" => buf.push_str(itoa::Buffer::new().format(epoch * 1000)),
                _ => format_raw_typed(raw, ExprResultType::Timestamp, buf),
            }
        }
        FieldType::Money => {
            let v = raw;
            match modifier {
                "usd" => {
                    buf.push('$');
                    push_money_formatted(v, buf, ',', '.');
                }
                "eur" => {
                    buf.push('\u{20ac}');
                    push_money_formatted(v, buf, '.', ',');
                }
                "gbp" => {
                    buf.push('\u{a3}');
                    push_money_formatted(v, buf, ',', '.');
                }
                _ => {
                    let _ = write!(buf, "{v:.2}");
                }
            }
        }
        _ => format_raw_typed(raw, ExprResultType::Int, buf),
    }
}

fn push_money_formatted(v: f64, buf: &mut String, thousands: char, decimal: char) {
    let abs = v.abs();
    let whole = abs as i64;
    let cents = ((abs - whole as f64) * 100.0).round() as i64;
    if v < 0.0 {
        buf.push('-');
    }
    let mut ib = itoa::Buffer::new();
    let s = ib.format(whole);
    let len = s.len();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            buf.push(thousands);
        }
        buf.push(ch);
    }
    buf.push(decimal);
    if cents < 10 {
        buf.push('0');
    }
    buf.push_str(itoa::Buffer::new().format(cents));
}

// ═══════════════════════════════════════════════════════════════════
// Batch generation from columns (expressions + aggregators + refs)
// ═══════════════════════════════════════════════════════════════════

/// Display name for a field spec: `name:upper` → `name_upper`, `phone:e164` → `phone_e164`.
pub fn spec_display_name(spec: &str) -> String {
    if let Ok((name, modifier, transform, ..)) = field::parse_field_spec(spec) {
        let base = name.to_string();
        let suffix = if modifier.is_empty() {
            match transform {
                Transform::Upper => "upper",
                Transform::Lower => "lower",
                Transform::Capitalize => "capitalize",
                Transform::None => "",
            }
        } else {
            modifier
        };
        if suffix.is_empty() {
            base
        } else {
            format!("{base}_{suffix}")
        }
    } else {
        spec.to_string()
    }
}

/// Parse field spec strings (CLI-style) into resolved columns with topo sort.
/// Handles aliases (`name=field:mod`), expressions (`total=price*qty`), aggregators (`running=amount:sum`).
pub fn resolve_field_specs(fields: &[String]) -> Result<(Vec<Column>, Vec<usize>), String> {
    let all_names: Vec<String> = fields
        .iter()
        .map(|spec| {
            if let Some(eq) = spec.find('=') {
                let colon = spec.find(':').unwrap_or(spec.len());
                if eq < colon {
                    return spec[..eq].to_string();
                }
            }
            spec_display_name(spec)
        })
        .collect();

    let mut columns = Vec::with_capacity(fields.len());
    for spec in fields {
        let (alias, value) = if let Some(eq) = spec.find('=') {
            let colon = spec.find(':').unwrap_or(spec.len());
            if eq < colon {
                (spec[..eq].to_string(), spec[eq + 1..].to_string())
            } else {
                (spec_display_name(spec), spec.clone())
            }
        } else {
            (spec_display_name(spec), spec.clone())
        };

        let gen = resolve_column(&alias, &value, &all_names)?;
        columns.push(Column { name: alias, gen });
    }

    resolve_expr_types(&mut columns)?;
    let eval_order = topo_sort_columns(&columns)?;
    Ok((columns, eval_order))
}

/// Parse field specs, resolve columns, generate records.
/// Returns `(column_names, records)`. Used by all bindings.
pub fn generate_records_from_specs(
    fields: &[String],
    opts: &crate::pipeline::RecordOpts<'_>,
    n: u64,
    start_serial: u64,
) -> Result<(Vec<String>, Vec<Vec<String>>), String> {
    let (columns, eval_order) = resolve_field_specs(fields)?;
    let col_names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
    let records = generate_from_columns(&columns, &eval_order, opts, n, start_serial);
    Ok((col_names, records))
}

pub fn generate_from_columns(
    columns: &[Column],
    eval_order: &[usize],
    opts: &crate::pipeline::RecordOpts<'_>,
    n: u64,
    start_serial: u64,
) -> Vec<Vec<String>> {
    let master_seed = opts.master_seed;
    let locales = opts.locales;
    let ctx_mode = opts.ctx;
    let corrupt_rate = opts.corrupt_rate;
    let tz_offset_minutes = opts.tz_offset_minutes;
    let since = opts.since;
    let until = opts.until;
    let col_count = columns.len();
    let col_names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
    // Bindings pass field names directly (no aliases), so no alias detection needed.
    let domain_hashes: Vec<u64> = columns
        .iter()
        .map(|c| match &c.gen {
            ColumnGen::Field { field, modifier, .. } => {
                crate::pipeline::field_domain_hash(master_seed, field, modifier)
            }
            ColumnGen::Expr { .. } => {
                crate::rng::domain_hash(master_seed, &format!("_expr_{}", c.name))
            }
            ColumnGen::Literal(_)
            | ColumnGen::Aggr { .. }
            | ColumnGen::Ref { .. }
            | ColumnGen::Fk { .. }
            | ColumnGen::FkDeref { .. } => 0,
        })
        .collect();
    let resolved_ranges: Vec<Option<(i64, i64)>> = columns
        .iter()
        .map(|c| match &c.gen {
            ColumnGen::Field { range, field, .. } => {
                field::resolve_range(range, field.name, since, until)
            }
            _ => None,
        })
        .collect();

    let needs_ctx = ctx_mode != crate::script::Ctx::None;
    let mut aggr =
        AggrState::new(columns, &col_names).unwrap_or_else(|_| AggrState { entries: Vec::new() });
    let mut records = Vec::with_capacity(n as usize);

    for i in 0..n {
        let serial = start_serial + i;

        let locked_locale: Option<&crate::locale::Locale> = match ctx_mode {
            crate::script::Ctx::Strict => {
                let mut lr = Rng::derive(master_seed, serial, crate::DOMAIN_LOCALE);
                Some(*lr.choice(locales))
            }
            crate::script::Ctx::Loose => {
                let mut lr = Rng::derive(master_seed, serial, crate::DOMAIN_LOCALE);
                if lr.maybe(0.7) {
                    Some(*lr.choice(locales))
                } else {
                    None
                }
            }
            crate::script::Ctx::None => None,
        };
        let locked_arr: [&crate::locale::Locale; 1];
        let effective_locales: &[&crate::locale::Locale] = if let Some(loc) = locked_locale {
            locked_arr = [loc];
            &locked_arr
        } else {
            locales
        };

        let identity = if needs_ctx {
            let mut ir = Rng::derive(master_seed, serial, crate::DOMAIN_IDENTITY);
            Some(crate::ctx::Identity::new(&mut ir, effective_locales, None, since, until))
        } else {
            None
        };

        let mut ctx = crate::ctx::GenContext {
            rng: Rng::new(0),
            locales: effective_locales,
            modifier: "",
            identity: identity.as_ref(),
            tz_offset_minutes,
            since,
            until,
            range: None,
            ordering: Ordering::None,
            zipf: None,
            numeric: None,
        };

        let mut values: Vec<String> = (0..col_count).map(|_| String::with_capacity(32)).collect();
        let mut raw_values: Vec<Option<f64>> = vec![None; col_count];

        for &idx in eval_order {
            match &columns[idx].gen {
                ColumnGen::Field {
                    field, modifier, transform, ordering, omit_pct, zipf, ..
                } => {
                    if let Some(pct) = omit_pct {
                        let mut or = Rng::derive(domain_hashes[idx], serial, "omit");
                        if or.range(0, 100) < i64::from(*pct) {
                            continue;
                        }
                    }
                    ctx.rng = Rng::derive_fast(domain_hashes[idx], serial);
                    ctx.modifier = modifier;
                    ctx.range = resolved_ranges[idx];
                    ctx.ordering = *ordering;
                    ctx.zipf = *zipf;
                    raw_values[idx] = field.generate(&mut ctx, &mut values[idx]);
                    if *transform != Transform::None {
                        let s = std::mem::take(&mut values[idx]);
                        values[idx] = transform.apply(&s);
                    }
                }
                ColumnGen::Literal(s) => {
                    values[idx].push_str(s);
                }
                ColumnGen::Aggr { .. } | ColumnGen::Fk { .. } | ColumnGen::FkDeref { .. } => {}
                ColumnGen::Ref { source_col, modifier } => {
                    if let Some(src_idx) = col_names.iter().position(|n| n == source_col) {
                        raw_values[idx] = raw_values[src_idx];
                        if modifier.is_empty() {
                            let src = values[src_idx].clone();
                            values[idx].push_str(&src);
                        } else if let Some(raw) = raw_values[src_idx] {
                            format_ref(raw, modifier, columns, src_idx, &mut values[idx]);
                        } else {
                            let src = values[src_idx].clone();
                            values[idx].push_str(&src);
                        }
                    }
                }
                ColumnGen::Expr { left, op, right, result_type } => {
                    let env = ExprEnv {
                        raw_values: &raw_values,
                        col_names: &col_names,
                        domain_hashes: &domain_hashes,
                        serial,
                    };
                    let lv = eval_operand(left, &env, &mut ctx, idx, true);
                    let rv = eval_operand(right, &env, &mut ctx, idx, false);
                    let adjusted_rv = match result_type {
                        ExprResultType::Date => rv * 86400.0,
                        _ => rv,
                    };
                    let result = match op {
                        ExprOp::Add => lv + adjusted_rv,
                        ExprOp::Sub => lv - adjusted_rv,
                        ExprOp::Mul => lv * rv,
                    };
                    raw_values[idx] = Some(result);
                    format_raw_typed(result, *result_type, &mut values[idx]);
                }
            }
        }

        let _ = aggr.update(&mut values, &raw_values);

        if let Some(rate) = corrupt_rate {
            let mut cr = Rng::derive(master_seed, serial, crate::DOMAIN_CORRUPT);
            crate::corrupt::corrupt_values(&mut cr, &mut values, rate);
        }

        records.push(values);
    }

    records
}
