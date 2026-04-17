use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub use seedfaker_core::eval::{resolve_expr_types, topo_sort_columns, Column};
use seedfaker_core::eval::{ColumnGen, FkDistribution};

// ═══════════════════════════════════════════════════════════════════
// Single-table config types
// ═══════════════════════════════════════════════════════════════════

#[derive(Deserialize, Default)]
struct RawConfig {
    #[serde(default)]
    columns: serde_yaml_bw::Mapping,
    template: Option<String>,
    #[serde(default)]
    options: RawConfigOptions,
}

#[derive(Deserialize, Default)]
struct RawConfigOptions {
    seed: Option<String>,
    #[serde(default)]
    locale: Vec<String>,
    ctx: Option<String>,
    corrupt: Option<String>,
    abc: Option<String>,
    count: Option<u64>,
    format: Option<String>,
    rate: Option<u64>,
    tz: Option<String>,
    since: Option<String>,
    until: Option<String>,
    no_header: Option<bool>,
    delim: Option<String>,
    fingerprint: Option<String>,
    validate: Option<bool>,
    annotated: Option<bool>,
}

#[derive(Clone)]
pub struct GenConfig {
    pub columns: Vec<Column>,
    pub eval_order: Vec<usize>,
    pub template: Option<String>,
    pub options: GenConfigOptions,
}

#[derive(Clone, Default)]
pub struct GenConfigOptions {
    pub seed: Option<String>,
    pub locale: Vec<String>,
    pub ctx: Option<String>,
    pub corrupt: Option<String>,
    pub abc: Option<String>,
    pub count: Option<u64>,
    pub format: Option<String>,
    pub rate: Option<u64>,
    pub tz: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub no_header: bool,
    pub delim: Option<String>,
    pub validate: bool,
    pub annotated: bool,
}

// ═══════════════════════════════════════════════════════════════════
// Multi-table config types
// ═══════════════════════════════════════════════════════════════════

pub enum ConfigKind {
    Single(Box<GenConfig>),
    Multi(MultiTableConfig),
}

pub struct MultiTableConfig {
    pub global_seed: u64,
    pub tables: Vec<(String, GenConfig)>,
}

impl MultiTableConfig {
    pub fn find_table(
        &self,
        name: &str,
    ) -> Result<&(String, GenConfig), Box<dyn std::error::Error>> {
        self.tables.iter().find(|(n, _)| n == name).ok_or_else(|| {
            let names: Vec<&str> = self.tables.iter().map(|(n, _)| n.as_str()).collect();
            format!("table '{name}' not found; available: {}", names.join(", ")).into()
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
// Presets
// ═══════════════════════════════════════════════════════════════════

const PRESETS: &[(&str, &str)] = &[
    ("nginx", include_str!("presets/nginx.yaml")),
    ("auth", include_str!("presets/auth.yaml")),
    ("app-json", include_str!("presets/app-json.yaml")),
    ("postgres", include_str!("presets/postgres.yaml")),
    ("payment", include_str!("presets/payment.yaml")),
    ("pii-leak", include_str!("presets/pii-leak.yaml")),
    ("user-table", include_str!("presets/user-table.yaml")),
    ("email", include_str!("presets/email-thread.yaml")),
    ("stacktrace", include_str!("presets/stacktrace.yaml")),
    ("chaos", include_str!("presets/chaos.yaml")),
    ("llm-prompt", include_str!("presets/llm-prompt.yaml")),
    ("syslog", include_str!("presets/syslog.yaml")),
    ("medical", include_str!("presets/medical.yaml")),
];

pub fn list_presets() -> Vec<&'static str> {
    PRESETS.iter().map(|(name, _)| *name).collect()
}

// ═══════════════════════════════════════════════════════════════════
// Single-table parse
// ═══════════════════════════════════════════════════════════════════

pub fn parse(source: &str) -> Result<GenConfig, String> {
    let raw: RawConfig =
        serde_yaml_bw::from_str(source).map_err(|e| format!("invalid config: {e}"))?;

    if raw.columns.is_empty() && raw.template.is_none() {
        return Err("config must have 'columns' or 'template'".into());
    }

    if let Some(ref expected) = raw.options.fingerprint {
        let current = seedfaker_core::fingerprint();
        if *expected != current {
            return Err(format!(
                "config fingerprint {expected} does not match current {current}; \
                 output would differ — update or remove fingerprint from config"
            ));
        }
    }

    let all_names: Vec<String> =
        raw.columns.keys().filter_map(|k| k.as_str().map(str::to_string)).collect();

    let mut columns = Vec::with_capacity(raw.columns.len());
    for (key, value) in &raw.columns {
        let name = key
            .as_str()
            .ok_or_else(|| format!("column key must be a string, got: {key:?}"))?
            .to_string();
        let text = extract_scalar(value)?;
        let gen = crate::tpl::resolve_column(&name, &text, &all_names)?;
        columns.push(Column { name, gen });
    }

    resolve_expr_types(&mut columns)?;
    let eval_order = topo_sort_columns(&columns)?;

    let options = GenConfigOptions {
        seed: raw.options.seed,
        locale: raw.options.locale,
        ctx: raw.options.ctx,
        corrupt: raw.options.corrupt,
        abc: raw.options.abc,
        count: raw.options.count,
        format: raw.options.format,
        rate: raw.options.rate,
        tz: raw.options.tz,
        since: raw.options.since,
        until: raw.options.until,
        no_header: raw.options.no_header.unwrap_or(false),
        delim: raw.options.delim,
        validate: raw.options.validate.unwrap_or(false),
        annotated: raw.options.annotated.unwrap_or(false),
    };

    Ok(GenConfig { columns, eval_order, template: raw.template, options })
}

fn extract_scalar(val: &serde_yaml_bw::Value) -> Result<String, String> {
    match val {
        serde_yaml_bw::Value::Tagged(tv) => extract_scalar(&tv.value),
        serde_yaml_bw::Value::String(s, _) => Ok(s.clone()),
        serde_yaml_bw::Value::Number(n, _) => Ok(n.to_string()),
        serde_yaml_bw::Value::Bool(b, _) => Ok(b.to_string()),
        serde_yaml_bw::Value::Null(_) => Ok(String::new()),
        other => Err(format!("unexpected column value: {other:?}")),
    }
}

const MAX_CONFIG_SIZE: u64 = 100 * 1024 * 1024;

// ═══════════════════════════════════════════════════════════════════
// Config kind detection
// ═══════════════════════════════════════════════════════════════════

const RESERVED_TABLE_NAMES: &[&str] = &["columns", "template", "options"];

/// Returns `true` for multi-table, `false` for single-table.
fn detect_config_kind(root: &serde_yaml_bw::Mapping) -> Result<bool, String> {
    let has_single_keys =
        root.iter().any(|(k, _)| matches!(k.as_str(), Some("columns" | "template")));
    let has_table_keys =
        root.iter().any(|(k, _)| k.as_str().is_some_and(|s| !RESERVED_TABLE_NAMES.contains(&s)));

    match (has_single_keys, has_table_keys) {
        (true, true) => {
            Err("cannot mix single-table keys (columns/template) with multi-table definitions"
                .into())
        }
        (true, false) => Ok(false),
        (false, true) => Ok(true),
        (false, false) => {
            Err("config must have 'columns', 'template', or table definitions".into())
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// Multi-table helpers
// ═══════════════════════════════════════════════════════════════════

/// Shape of a resolved non-FK column — used for FK validation and `ParentCtx` construction.
#[derive(Clone)]
struct ColShape {
    field: &'static seedfaker_core::field::Field,
    modifier: String,
    range: Option<seedfaker_core::field::RangeSpec>,
    ordering: seedfaker_core::field::Ordering,
}

#[derive(Default)]
struct RawTableOpts {
    count: Option<u64>,
    locale: Vec<String>,
    ctx: Option<String>,
    corrupt: Option<String>,
    abc: Option<String>,
    format: Option<String>,
    rate: Option<u64>,
    tz: Option<String>,
    since: Option<String>,
    until: Option<String>,
    no_header: Option<bool>,
    delim: Option<String>,
    validate: Option<bool>,
    annotated: Option<bool>,
}

fn parse_raw_table_opts(mapping: &serde_yaml_bw::Mapping) -> Result<RawTableOpts, String> {
    let mut opts = RawTableOpts::default();
    for (k, v) in mapping {
        match k.as_str().unwrap_or("") {
            "count" => {
                opts.count = Some(
                    v.as_u64()
                        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                        .ok_or_else(|| "options.count must be an integer".to_string())?,
                );
            }
            "locale" => match v {
                serde_yaml_bw::Value::String(s, _) => {
                    opts.locale = s.split(',').map(|x| x.trim().to_string()).collect();
                }
                serde_yaml_bw::Value::Sequence(seq) => {
                    for item in seq {
                        if let Some(s) = item.as_str() {
                            opts.locale.push(s.to_string());
                        }
                    }
                }
                _ => {}
            },
            "ctx" => opts.ctx = v.as_str().map(str::to_string),
            "corrupt" => opts.corrupt = v.as_str().map(str::to_string),
            "abc" => opts.abc = v.as_str().map(str::to_string),
            "format" => opts.format = v.as_str().map(str::to_string),
            "rate" => opts.rate = v.as_u64(),
            "tz" => opts.tz = v.as_str().map(str::to_string),
            "since" => opts.since = v.as_str().map(str::to_string),
            "until" => opts.until = v.as_str().map(str::to_string),
            "no_header" => opts.no_header = v.as_bool(),
            "delim" => opts.delim = v.as_str().map(str::to_string),
            "validate" => opts.validate = v.as_bool(),
            "annotated" => opts.annotated = v.as_bool(),
            "seed" => {} // root-only; silently ignored at table level
            unknown => return Err(format!("options: unknown key '{unknown}'")),
        }
    }
    Ok(opts)
}

fn merge_options(root: &RawTableOpts, table: &RawTableOpts) -> GenConfigOptions {
    GenConfigOptions {
        seed: None,
        locale: if table.locale.is_empty() { root.locale.clone() } else { table.locale.clone() },
        ctx: table.ctx.clone().or_else(|| root.ctx.clone()),
        corrupt: table.corrupt.clone().or_else(|| root.corrupt.clone()),
        abc: table.abc.clone().or_else(|| root.abc.clone()),
        count: table.count.or(root.count),
        format: table.format.clone().or_else(|| root.format.clone()),
        rate: table.rate.or(root.rate),
        tz: table.tz.clone().or_else(|| root.tz.clone()),
        since: table.since.clone().or_else(|| root.since.clone()),
        until: table.until.clone().or_else(|| root.until.clone()),
        no_header: table.no_header.unwrap_or(root.no_header.unwrap_or(false)),
        delim: table.delim.clone().or_else(|| root.delim.clone()),
        validate: table.validate.unwrap_or(root.validate.unwrap_or(false)),
        annotated: table.annotated.unwrap_or(root.annotated.unwrap_or(false)),
    }
}

/// Returns true if spec is FK anchor syntax: `table.col` where `table` is a known table name.
fn is_fk_anchor(spec: &str, table_names: &HashSet<&str>) -> bool {
    if let Some(dot_pos) = spec.find('.') {
        let left = &spec[..dot_pos];
        let after = &spec[dot_pos + 1..];
        !after.starts_with('.') && table_names.contains(left)
    } else {
        false
    }
}

/// Returns true if spec is FK deref syntax: `anchor_col->target`.
fn is_fk_deref(spec: &str) -> bool {
    spec.contains("->")
}

/// Parse `table.col[:zipf[=s]]` → `(table, col, dist)`.
fn parse_fk_anchor_spec(spec: &str) -> Result<(String, String, FkDistribution), String> {
    let dot = spec.find('.').ok_or_else(|| format!("FK '{spec}': missing '.' separator"))?;
    let table = spec[..dot].to_string();
    let rest = &spec[dot + 1..];

    let (col, modifier) = match rest.find(':') {
        Some(p) => (&rest[..p], &rest[p + 1..]),
        None => (rest, ""),
    };

    if col.is_empty() {
        return Err(format!("FK '{spec}': column name must not be empty"));
    }

    let dist = if modifier.is_empty() {
        FkDistribution::Uniform
    } else if modifier == "zipf" {
        FkDistribution::Zipf(1.0)
    } else if let Some(s) = modifier.strip_prefix("zipf=") {
        let exp: f64 =
            s.parse().map_err(|_| format!("FK '{spec}': zipf exponent must be a number"))?;
        if exp <= 0.0 {
            return Err(format!("FK '{spec}': zipf exponent must be > 0"));
        }
        FkDistribution::Zipf(exp)
    } else {
        return Err(format!(
            "FK '{spec}': unknown modifier '{modifier}'; expected 'zipf' or 'zipf=N'"
        ));
    };

    Ok((table, col.to_string(), dist))
}

/// Parse `anchor_col->target_col` → `(anchor, target)`.
fn parse_fk_deref_spec(spec: &str) -> Result<(String, String), String> {
    let pos =
        spec.find("->").ok_or_else(|| format!("FK deref '{spec}': missing '->' separator"))?;
    let anchor = spec[..pos].trim().to_string();
    let target = spec[pos + 2..].trim().to_string();
    if anchor.is_empty() {
        return Err(format!("FK deref '{spec}': anchor column must not be empty"));
    }
    if target.is_empty() {
        return Err(format!("FK deref '{spec}': target column must not be empty"));
    }
    Ok((anchor, target))
}

// ═══════════════════════════════════════════════════════════════════
// Table topological sort
// ═══════════════════════════════════════════════════════════════════

fn topo_sort_tables(
    table_names: &[String],
    deps: &HashMap<String, Vec<String>>,
) -> Result<Vec<String>, String> {
    let n = table_names.len();
    let idx: HashMap<&str, usize> =
        table_names.iter().enumerate().map(|(i, s)| (s.as_str(), i)).collect();

    let mut in_degree = vec![0usize; n];
    let mut dependents: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (name, parents) in deps {
        let child = idx[name.as_str()];
        for parent in parents {
            if parent == name {
                continue; // self-reference: skip ordering constraint
            }
            let par = idx[parent.as_str()];
            dependents[par].push(child);
            in_degree[child] += 1;
        }
    }

    let mut queue: Vec<usize> = (0..n).filter(|i| in_degree[*i] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(i) = queue.pop() {
        order.push(i);
        for &dep in &dependents[i] {
            in_degree[dep] -= 1;
            if in_degree[dep] == 0 {
                queue.push(dep);
            }
        }
    }

    if order.len() != n {
        let cycle: Vec<&str> =
            (0..n).filter(|i| in_degree[*i] > 0).map(|i| table_names[i].as_str()).collect();
        return Err(format!("circular FK dependency between tables: {}", cycle.join(", ")));
    }

    Ok(order.into_iter().map(|i| table_names[i].clone()).collect())
}

// ═══════════════════════════════════════════════════════════════════
// parse_multi
// ═══════════════════════════════════════════════════════════════════

pub fn parse_multi(source: &str, effective_seed: Option<&str>) -> Result<MultiTableConfig, String> {
    let val: serde_yaml_bw::Value =
        serde_yaml_bw::from_str(source).map_err(|e| format!("invalid config: {e}"))?;

    let serde_yaml_bw::Value::Mapping(root_map) = &val else {
        return Err("config must be a YAML mapping".into());
    };

    let root_opts_raw = extract_opts(root_map, "root")?;
    let root_seed_str = extract_root_seed(root_map);
    let eff_seed = effective_seed
        .map(str::to_string)
        .or(root_seed_str)
        .unwrap_or_else(|| format!("{}", seedfaker_core::rng::random_seed()));
    let global_seed = seedfaker_core::hash_seed(&eff_seed);

    let table_names_vec: Vec<String> = root_map
        .iter()
        .filter_map(|(k, _)| k.as_str())
        .filter(|s| !RESERVED_TABLE_NAMES.contains(s))
        .map(str::to_string)
        .collect();

    let table_name_set: HashSet<&str> = table_names_vec.iter().map(String::as_str).collect();

    // ── Pass 1: parse non-FK columns + options ──────────────────────

    struct TableData {
        columns: Vec<Column>,
        template: Option<String>,
        deferred_anchors: Vec<(String, String)>, // (col_name, spec)
        deferred_derefs: Vec<(String, String)>,  // (col_name, spec)
        options: GenConfigOptions,
        shapes: HashMap<String, ColShape>, // non-FK columns only
    }

    let mut tables: HashMap<String, TableData> = HashMap::new();
    let mut errors: Vec<String> = Vec::new();

    for table_name in &table_names_vec {
        let table_map = match get_mapping_key(root_map, table_name) {
            Ok(m) => m,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        for (k, _) in table_map {
            match k.as_str().unwrap_or("") {
                "columns" | "template" | "options" => {}
                unknown => errors.push(format!("table '{table_name}': unknown key '{unknown}'")),
            }
        }

        let Ok(columns_map) = get_mapping_key(table_map, "columns") else {
            errors.push(format!("table '{table_name}' must have 'columns'"));
            continue;
        };

        if columns_map.is_empty() {
            errors.push(format!("table '{table_name}': columns must not be empty"));
            continue;
        }

        let all_col_names: Vec<String> =
            columns_map.keys().filter_map(|k| k.as_str().map(str::to_string)).collect();

        let mut columns: Vec<Column> = Vec::new();
        let mut deferred_anchors: Vec<(String, String)> = Vec::new();
        let mut deferred_derefs: Vec<(String, String)> = Vec::new();
        let mut shapes: HashMap<String, ColShape> = HashMap::new();

        for (col_key, col_val) in columns_map {
            let Some(col_name_str) = col_key.as_str() else {
                errors.push(format!("table '{table_name}': column key must be a string"));
                continue;
            };
            let col_name = col_name_str.to_string();
            let spec = match extract_scalar(col_val) {
                Ok(s) => s,
                Err(e) => {
                    errors.push(format!("table '{table_name}'.{col_name}: {e}"));
                    continue;
                }
            };

            if is_fk_anchor(&spec, &table_name_set) {
                deferred_anchors.push((col_name, spec));
            } else if is_fk_deref(&spec) {
                deferred_derefs.push((col_name, spec));
            } else {
                match crate::tpl::resolve_column(&col_name, &spec, &all_col_names) {
                    Ok(gen) => {
                        if let ColumnGen::Field { field, modifier, range, ordering, .. } = &gen {
                            shapes.insert(
                                col_name.clone(),
                                ColShape {
                                    field,
                                    modifier: modifier.clone(),
                                    range: *range,
                                    ordering: *ordering,
                                },
                            );
                        }
                        columns.push(Column { name: col_name, gen });
                    }
                    Err(e) => errors.push(format!("table '{table_name}'.{col_name}: {e}")),
                }
            }
        }

        let template = table_map
            .iter()
            .find(|(k, _)| k.as_str() == Some("template"))
            .and_then(|(_, v)| v.as_str().map(str::to_string));

        let table_opts = match extract_opts(table_map, &format!("table '{table_name}'")) {
            Ok(o) => o,
            Err(e) => {
                errors.push(e);
                RawTableOpts::default()
            }
        };
        let options = merge_options(&root_opts_raw, &table_opts);

        tables.insert(
            table_name.clone(),
            TableData { columns, template, deferred_anchors, deferred_derefs, options, shapes },
        );
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    // ── Pass 2a: resolve FK anchors ──────────────────────────────────
    // Collect resolved anchors first (read-only borrows), then push (mutable).

    // (table_name, col_name, gen, parent_table_name)
    let mut resolved_anchors: Vec<(String, String, ColumnGen, String)> = Vec::new();
    // FK dependency graph: table → parent tables.
    let mut fk_deps: HashMap<String, Vec<String>> =
        table_names_vec.iter().map(|n| (n.clone(), Vec::new())).collect();

    for table_name in &table_names_vec {
        let Some(td) = tables.get(table_name) else { continue };

        for (col_name, spec) in &td.deferred_anchors {
            let (parent_table, parent_col, dist) = match parse_fk_anchor_spec(spec) {
                Ok(r) => r,
                Err(e) => {
                    errors.push(format!("table '{table_name}'.{col_name}: {e}"));
                    continue;
                }
            };

            let Some(parent_td) = tables.get(&parent_table) else {
                errors.push(format!(
                    "table '{table_name}'.{col_name}: FK references unknown table '{parent_table}'"
                ));
                continue;
            };

            let Some(shape) = parent_td.shapes.get(&parent_col) else {
                errors.push(format!(
                    "table '{table_name}'.{col_name}: \
                     FK parent column '{parent_table}.{parent_col}' not found \
                     (only non-FK columns can be FK anchors)"
                ));
                continue;
            };

            let Some(parent_count) = parent_td.options.count else {
                errors.push(format!(
                    "table '{table_name}'.{col_name}: \
                     parent table '{parent_table}' has no count; add 'count' to its options"
                ));
                continue;
            };

            if parent_count == 0 {
                errors.push(format!(
                    "table '{table_name}'.{col_name}: \
                     parent table '{parent_table}' count must be > 0"
                ));
                continue;
            }

            let gen = ColumnGen::Fk {
                parent_table: parent_table.clone(),
                parent_col_name: parent_col.clone(),
                parent_field: shape.field,
                parent_modifier: shape.modifier.clone(),
                parent_range: shape.range,
                parent_ordering: shape.ordering,
                parent_count,
                distribution: dist,
                parent_domain_hash: 0,
                parent_ctx: Box::default(),
            };

            resolved_anchors.push((
                table_name.clone(),
                col_name.clone(),
                gen,
                parent_table.clone(),
            ));
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    for (table_name, col_name, gen, parent_table) in resolved_anchors {
        fk_deps.entry(table_name.clone()).or_default().push(parent_table);
        if let Some(td) = tables.get_mut(&table_name) {
            td.columns.push(Column { name: col_name, gen });
        }
    }

    // ── Pass 2b: resolve FK derefs ───────────────────────────────────

    let mut resolved_derefs: Vec<(String, String, ColumnGen)> = Vec::new();

    for table_name in &table_names_vec {
        let Some(td) = tables.get(table_name) else { continue };

        for (col_name, spec) in &td.deferred_derefs {
            let (anchor_col, target_col) = match parse_fk_deref_spec(spec) {
                Ok(r) => r,
                Err(e) => {
                    errors.push(format!("table '{table_name}'.{col_name}: {e}"));
                    continue;
                }
            };

            // D01+D02: anchor must exist and be Fk.
            let parent_table: String = match td.columns.iter().find(|c| c.name == anchor_col) {
                Some(Column { gen: ColumnGen::Fk { parent_table, .. }, .. }) => {
                    parent_table.clone()
                }
                Some(_) => {
                    errors.push(format!(
                        "table '{table_name}'.{col_name}: \
                         '{anchor_col}' must be an FK anchor column (e.g. users.id)"
                    ));
                    continue;
                }
                None => {
                    errors.push(format!(
                        "table '{table_name}'.{col_name}: \
                         anchor column '{anchor_col}' not found in table '{table_name}'"
                    ));
                    continue;
                }
            };

            // D03+D04: target must exist in parent table and be a non-FK column.
            let Some(parent_td) = tables.get(&parent_table) else {
                errors.push(format!(
                    "table '{table_name}'.{col_name}: internal: parent table '{parent_table}' missing"
                ));
                continue;
            };
            let parent_shapes = &parent_td.shapes;

            let Some(shape) = parent_shapes.get(&target_col) else {
                errors.push(format!(
                    "table '{table_name}'.{col_name}: \
                     deref target '{parent_table}.{target_col}' not found \
                     (only non-FK columns can be dereferenced)"
                ));
                continue;
            };

            let gen = ColumnGen::FkDeref {
                anchor_col,
                deref_col_name: target_col.clone(),
                deref_field: shape.field,
                deref_modifier: shape.modifier.clone(),
                deref_range: shape.range,
                deref_ordering: shape.ordering,
                deref_domain_hash: 0,
                parent_ctx: Box::default(),
            };

            resolved_derefs.push((table_name.clone(), col_name.clone(), gen));
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    for (table_name, col_name, gen) in resolved_derefs {
        if let Some(td) = tables.get_mut(&table_name) {
            td.columns.push(Column { name: col_name, gen });
        }
    }

    // ── Pass 3: topo sort tables + finalize GenConfigs ───────────────

    let sorted_names = topo_sort_tables(&table_names_vec, &fk_deps)?;
    let mut result_tables: Vec<(String, GenConfig)> = Vec::with_capacity(sorted_names.len());

    for table_name in sorted_names {
        let Some(mut td) = tables.remove(&table_name) else { continue };

        if let Err(e) = resolve_expr_types(&mut td.columns) {
            errors.push(format!("table '{table_name}': {e}"));
            continue;
        }
        let eval_order = match topo_sort_columns(&td.columns) {
            Ok(o) => o,
            Err(e) => {
                errors.push(format!("table '{table_name}': {e}"));
                continue;
            }
        };

        result_tables.push((
            table_name,
            GenConfig {
                columns: td.columns,
                eval_order,
                template: td.template,
                options: td.options,
            },
        ));
    }

    if !errors.is_empty() {
        return Err(errors.join("\n"));
    }

    Ok(MultiTableConfig { global_seed, tables: result_tables })
}

// ═══════════════════════════════════════════════════════════════════
// parse_multi helpers
// ═══════════════════════════════════════════════════════════════════

fn get_mapping_key<'a>(
    map: &'a serde_yaml_bw::Mapping,
    key: &str,
) -> Result<&'a serde_yaml_bw::Mapping, String> {
    map.iter()
        .find(|(k, _)| k.as_str() == Some(key))
        .ok_or_else(|| format!("key '{key}' not found"))
        .and_then(|(_, v)| match v {
            serde_yaml_bw::Value::Mapping(m) => Ok(m),
            _ => Err(format!("'{key}' must be a YAML mapping")),
        })
}

fn extract_opts(map: &serde_yaml_bw::Mapping, label: &str) -> Result<RawTableOpts, String> {
    match map.iter().find(|(k, _)| k.as_str() == Some("options")).map(|(_, v)| v) {
        Some(serde_yaml_bw::Value::Mapping(m)) => {
            parse_raw_table_opts(m).map_err(|e| format!("{label} {e}"))
        }
        Some(_) => Err(format!("{label} 'options' must be a mapping")),
        None => Ok(RawTableOpts::default()),
    }
}

fn extract_root_seed(root_map: &serde_yaml_bw::Mapping) -> Option<String> {
    root_map
        .iter()
        .find(|(k, _)| k.as_str() == Some("options"))
        .and_then(|(_, v)| v.as_mapping())
        .and_then(|m| m.iter().find(|(k, _)| k.as_str() == Some("seed")))
        .and_then(|(_, v)| v.as_str().map(str::to_string))
}

// ═══════════════════════════════════════════════════════════════════
// load_config — public entry point
// ═══════════════════════════════════════════════════════════════════

pub fn load_config(name_or_path: &str) -> Result<ConfigKind, String> {
    for (bname, content) in PRESETS {
        if *bname == name_or_path {
            return parse(content).map(|c| ConfigKind::Single(Box::new(c)));
        }
    }

    let path = Path::new(name_or_path);
    if path.exists() {
        let meta =
            std::fs::metadata(path).map_err(|e| format!("cannot stat '{name_or_path}': {e}"))?;

        if !meta.is_file() {
            return Err(format!("'{name_or_path}' is not a regular file"));
        }

        if meta.len() > MAX_CONFIG_SIZE {
            return Err(format!(
                "config file too large: {} bytes (max {} MB). \
                 Split into multiple configs or check the file path.",
                meta.len(),
                MAX_CONFIG_SIZE / (1024 * 1024)
            ));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read '{name_or_path}': {e}"))?;

        let root_val: serde_yaml_bw::Value =
            serde_yaml_bw::from_str(&content).map_err(|e| format!("invalid config: {e}"))?;

        let serde_yaml_bw::Value::Mapping(root_map) = &root_val else {
            return Err("config must be a YAML mapping".into());
        };

        return if detect_config_kind(root_map)? {
            parse_multi(&content, None).map(ConfigKind::Multi)
        } else {
            parse(&content).map(|c| ConfigKind::Single(Box::new(c)))
        };
    }

    let names: Vec<&str> = PRESETS.iter().map(|(n, _)| *n).collect();
    Err(format!(
        "'{name_or_path}' is not a file or preset; available presets: {}",
        names.join(", ")
    ))
}
