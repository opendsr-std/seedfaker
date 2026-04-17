use std::io::{BufWriter, Write};

use crate::tpl::column::{Column, ColumnGen, FkDistribution, ParentCtx};
use seedfaker_core::ctx::{GenContext, Identity};
use seedfaker_core::field::{Ordering, RangeSpec, Transform};
use seedfaker_core::locale::Locale;
use seedfaker_core::rng::Rng;
use seedfaker_core::script::{Corrupt, Ctx, Script};

use crate::config::GenConfig;
use crate::format;
use crate::writers;

#[derive(Clone)]
pub struct RunOptions {
    pub master_seed: u64,
    pub count: u64,
    /// Shard this run as slice `idx` of `total` disjoint serial ranges.
    /// `None` or `Some((0, 1))` = no sharding.
    pub shard: Option<(u64, u64)>,
    /// Number of in-process worker threads. `0` or `1` = single-threaded.
    /// Must be 1 when `count == 0` (unlimited) or when an aggregator column
    /// is present (aggr state is order-sensitive).
    pub threads: usize,
    /// Internal — used by the threaded dispatcher to override the shard
    /// computation with an exact `[start, end)` serial range for one worker.
    /// End users don't set this; `--threads N` fills it.
    pub serial_range: Option<(u64, u64)>,
    pub locales: Vec<&'static Locale>,
    pub script: Script,
    pub ctx: Ctx,
    pub corrupt: Corrupt,
    pub rate: Option<u64>,
    pub no_header: bool,
    pub output: OutputMode,
    pub delim: Option<String>,
    pub validate: bool,
    pub annotated: bool,
    pub tz_offset_minutes: i32,
    pub since: i64,
    pub until: i64,
}

#[derive(Clone)]
pub enum OutputMode {
    Default,
    Csv,
    Tsv,
    Jsonl,
    Sql(String),
    Template,
}

pub fn format_ext(mode: &OutputMode) -> &'static str {
    match mode {
        OutputMode::Csv => "csv",
        OutputMode::Default | OutputMode::Tsv => "tsv",
        OutputMode::Jsonl => "jsonl",
        OutputMode::Sql(_) => "sql",
        OutputMode::Template => "txt",
    }
}

// ═══════════════════════════════════════════════════════════════════
// FK orchestration
// ═══════════════════════════════════════════════════════════════════

/// Fill `parent_domain_hash` / `deref_domain_hash` / `parent_ctx` on all FK columns.
/// Call on a **clone** of the table's `GenConfig` — never mutate the stored original.
pub fn finalize_fk_columns(
    config: &mut GenConfig,
    global_seed: u64,
    all_tables: &[(String, GenConfig)],
) {
    // Collect (col_index, parent_table) for FkDeref resolution below.
    // We need this snapshot because we'll iterate `config.columns` mutably.
    let anchor_parents: Vec<(String, String)> = config
        .columns
        .iter()
        .filter_map(|c| {
            if let ColumnGen::Fk { parent_table, .. } = &c.gen {
                Some((c.name.clone(), parent_table.clone()))
            } else {
                None
            }
        })
        .collect();

    for col in &mut config.columns {
        match &mut col.gen {
            ColumnGen::Fk {
                parent_table,
                parent_col_name,
                parent_field,
                parent_modifier,
                parent_domain_hash,
                parent_ctx,
                ..
            } => {
                let parent_seed = seedfaker_core::rng::domain_hash(global_seed, parent_table);
                *parent_domain_hash = seedfaker_core::eval::column_domain_hash(
                    parent_seed,
                    parent_col_name,
                    parent_field,
                    parent_modifier,
                );
                **parent_ctx = build_parent_ctx(all_tables, parent_table, global_seed);
            }
            ColumnGen::FkDeref {
                anchor_col,
                deref_col_name,
                deref_field,
                deref_modifier,
                deref_domain_hash,
                parent_ctx,
                ..
            } => {
                let parent_table = anchor_parents
                    .iter()
                    .find(|(name, _)| name == anchor_col)
                    .map_or("", |(_, pt)| pt.as_str());

                let parent_seed = seedfaker_core::rng::domain_hash(global_seed, parent_table);
                *deref_domain_hash = seedfaker_core::eval::column_domain_hash(
                    parent_seed,
                    deref_col_name,
                    deref_field,
                    deref_modifier,
                );
                **parent_ctx = build_parent_ctx(all_tables, parent_table, global_seed);
            }
            _ => {}
        }
    }
}

/// Build a `ParentCtx` by resolving a parent table's stored options.
/// All resolution is done with core functions — no CLI layer needed.
fn build_parent_ctx(
    all_tables: &[(String, GenConfig)],
    parent_table: &str,
    global_seed: u64,
) -> ParentCtx {
    let table_seed = seedfaker_core::rng::domain_hash(global_seed, parent_table);

    let Some((_, cfg)) = all_tables.iter().find(|(n, _)| n == parent_table) else {
        return ParentCtx { table_seed, ..Default::default() };
    };

    let opts = &cfg.options;

    let locales: Vec<&'static Locale> =
        seedfaker_core::locale::resolve(&opts.locale).unwrap_or_default();

    let ctx = match opts.ctx.as_deref() {
        Some(s) => Ctx::parse(s),
        None => Ctx::None,
    };

    let script = match opts.abc.as_deref() {
        Some("native") => Script::Native,
        Some("mixed") => Script::Both,
        _ => Script::Latin,
    };

    let tz_offset = opts.tz.as_deref().and_then(|s| seedfaker_core::tz::parse(s).ok()).unwrap_or(0);

    let since = opts
        .since
        .as_deref()
        .and_then(|s| seedfaker_core::temporal::parse(s).ok())
        .unwrap_or(seedfaker_core::temporal::DEFAULT_SINCE);

    let until = opts
        .until
        .as_deref()
        .and_then(|s| seedfaker_core::temporal::parse_until(s).ok())
        .unwrap_or_else(seedfaker_core::temporal::default_until);

    ParentCtx {
        table_seed,
        locales,
        script,
        ctx,
        tz_offset_minutes: tz_offset,
        since,
        until,
        parent_count: opts.count.unwrap_or(0),
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_parent_field(
    field: &'static seedfaker_core::field::Field,
    modifier: &str,
    range: &Option<RangeSpec>,
    ordering: Ordering,
    domain_hash: u64,
    row_idx: u64,
    parent_ctx: &ParentCtx,
    buf: &mut String,
) -> Option<f64> {
    let resolved =
        seedfaker_core::field::resolve_range(range, field.name, parent_ctx.since, parent_ctx.until);

    // When the parent table uses ctx:strict/loose, its per-row fields (first_name,
    // email, etc.) are correlated through a single `Identity` derived from
    // (table_seed, row_idx, DOMAIN_IDENTITY). FK-dereferenced fields must see the
    // same Identity or they'll return uncorrelated values.
    let locked_locale: Option<Locale> = if parent_ctx.locales.is_empty() {
        None
    } else {
        match parent_ctx.ctx {
            Ctx::None => None,
            Ctx::Strict => {
                let mut lr =
                    Rng::derive(parent_ctx.table_seed, row_idx, seedfaker_core::DOMAIN_LOCALE);
                Some(**lr.choice(&parent_ctx.locales))
            }
            Ctx::Loose => {
                let mut lr =
                    Rng::derive(parent_ctx.table_seed, row_idx, seedfaker_core::DOMAIN_LOCALE);
                if lr.maybe(0.7) {
                    Some(**lr.choice(&parent_ctx.locales))
                } else {
                    None
                }
            }
        }
    };
    let locked_slot: [&Locale; 1];
    let record_locales: &[&Locale] = if let Some(ref loc) = locked_locale {
        locked_slot = [loc];
        &locked_slot
    } else {
        &parent_ctx.locales
    };

    let identity = if parent_ctx.ctx != Ctx::None && !record_locales.is_empty() {
        let mut ir = Rng::derive(parent_ctx.table_seed, row_idx, seedfaker_core::DOMAIN_IDENTITY);
        Some(Identity::new(&mut ir, record_locales, None, parent_ctx.since, parent_ctx.until))
    } else {
        None
    };

    let mut ctx = GenContext {
        rng: Rng::derive_fast(domain_hash, row_idx),
        locales: record_locales,
        modifier,
        identity: identity.as_ref(),
        tz_offset_minutes: parent_ctx.tz_offset_minutes,
        since: parent_ctx.since,
        until: parent_ctx.until,
        range: resolved,
        ordering,
        zipf: None,
        numeric: None,
    };
    field.generate(&mut ctx, buf)
}

/// Per-column cache of the Zipf CDF array used for FK anchor sampling.
/// `Rng::zipf` currently rebuilds a `Vec<f64>` of size `parent_count` and runs
/// `parent_count` `powf` calls on every single child-row sample — that is the
/// dominant cost on `transactions`/`ledger_entries`. Pre-computing once per FK
/// column turns per-row sampling into a single binary search.
///
/// `cdfs[col_idx]` is `Some` only for FK columns with `FkDistribution::Zipf(s)`
/// and `1 <= parent_count <= 65536` — matching the branch in `Rng::zipf` that
/// uses `zipf_cdf`. For larger `n` we keep the existing rejection-inversion path.
#[derive(Default)]
struct FkSamplerCache {
    cdfs: Vec<Option<Vec<f64>>>,
}

fn build_fk_sampler_cache(columns: &[Column]) -> FkSamplerCache {
    let mut cdfs: Vec<Option<Vec<f64>>> = (0..columns.len()).map(|_| None).collect();
    for (i, col) in columns.iter().enumerate() {
        if let ColumnGen::Fk { distribution: FkDistribution::Zipf(s), parent_count, .. } = &col.gen
        {
            if *parent_count >= 1 && *parent_count <= 65_536 {
                let n_usize = *parent_count as usize;
                let mut cum = Vec::with_capacity(n_usize);
                let mut total = 0.0_f64;
                for k in 1..=n_usize {
                    total += 1.0 / (k as f64).powf(*s);
                    cum.push(total);
                }
                cdfs[i] = Some(cum);
            }
        }
    }
    FkSamplerCache { cdfs }
}

/// Pre-parsed enum modifier: values and cumulative-weight array.
/// `gen::enum_::gen` re-splits the modifier string, re-parses weights, and re-sums
/// the total on every row — a measurable cost for tables with enum columns that
/// emit millions of rows. Parsing once per column is a pure-cache win: the output
/// is bit-identical to the un-cached path (same cumulative-weight table, same
/// `urange(0, total - 1)` draw).
struct EnumSpec {
    values: Vec<String>,
    /// `cum_weights[i]` = sum of weights[0..=i]. Last element = total.
    cum_weights: Vec<u32>,
}

impl EnumSpec {
    fn sample(&self, rng: &mut Rng, buf: &mut String) {
        // Invariant: `parse_enum_spec` returns None for empty modifiers,
        // so a built EnumSpec always has at least one weight.
        let Some(&total) = self.cum_weights.last() else { return };
        let roll = rng.urange(0, total as usize - 1) as u32;
        let idx = self.cum_weights.partition_point(|&c| c <= roll);
        let pick = if idx < self.values.len() { idx } else { self.values.len() - 1 };
        buf.push_str(&self.values[pick]);
    }
}

fn parse_enum_spec(modifier: &str) -> Option<EnumSpec> {
    if modifier.is_empty() {
        return None;
    }
    let mut values: Vec<String> = Vec::new();
    let mut weights: Vec<u32> = Vec::new();
    for entry in modifier.split(',') {
        if let Some((val, w_str)) = entry.split_once('=') {
            let w: u32 = w_str.parse().unwrap_or(1);
            values.push(val.to_string());
            weights.push(w.max(1));
        } else {
            values.push(entry.to_string());
            weights.push(1);
        }
    }
    if values.is_empty() {
        return None;
    }
    let mut cum_weights = Vec::with_capacity(weights.len());
    let mut acc: u32 = 0;
    for &w in &weights {
        acc = acc.saturating_add(w);
        cum_weights.push(acc);
    }
    Some(EnumSpec { values, cum_weights })
}

fn build_enum_cache(columns: &[Column]) -> Vec<Option<EnumSpec>> {
    columns
        .iter()
        .map(|col| {
            if let ColumnGen::Field { field, modifier, .. } = &col.gen {
                if field.name == "enum" {
                    return parse_enum_spec(modifier);
                }
            }
            None
        })
        .collect()
}

// ═══════════════════════════════════════════════════════════════════
// run<W: Write>
// ═══════════════════════════════════════════════════════════════════

/// Dispatch N worker threads, each generating a disjoint sub-range of the
/// effective serial range (after `--shard`). Each worker's output — header
/// included for worker 0 — is captured into a `Vec<u8>`; the main thread
/// writes the buffers in serial order so final output is byte-identical
/// to a single-threaded run.
///
/// Bounded-count only. Aggregators disable threading (order-sensitive state).
/// `--threads` composes with `--shard`: outer shard picks the range, inner
/// threads split it further. `--threads 1` is a no-op.
fn run_threaded<W: Write>(
    writer: W,
    gen_config: &GenConfig,
    opts: &RunOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::thread;

    // Compute the effective serial range respecting any outer --shard.
    let (range_start, range_end) = match opts.shard {
        Some((i, n)) if n > 1 => {
            let base = opts.count / n;
            let rem = opts.count % n;
            let s = base * i + i.min(rem);
            let l = base + u64::from(i < rem);
            (s, s + l)
        }
        _ => (0, opts.count),
    };
    let total = range_end.saturating_sub(range_start);
    let n = opts.threads as u64;
    if total == 0 || n <= 1 || total < n {
        // Nothing to parallelise — fall back to single-threaded.
        let mut opts_single = opts.clone();
        opts_single.threads = 1;
        return run(writer, gen_config, &opts_single);
    }

    let base = total / n;
    let rem = total % n;
    let slices: Vec<(u64, u64)> = (0..n)
        .map(|t| {
            let s = range_start + base * t + t.min(rem);
            let l = base + u64::from(t < rem);
            (s, s + l)
        })
        .collect();

    // Each worker writes into its own buffer; main concatenates in order.
    let buffers: Vec<Result<Vec<u8>, String>> = thread::scope(|scope| {
        let handles: Vec<_> = slices
            .iter()
            .enumerate()
            .map(|(idx, &(s, e))| {
                let mut topts = opts.clone();
                topts.threads = 1;
                topts.shard = None;
                topts.serial_range = Some((s, e));
                if idx > 0 {
                    topts.no_header = true;
                }
                scope.spawn(move || -> Result<Vec<u8>, String> {
                    let mut buf: Vec<u8> = Vec::with_capacity(1 << 20);
                    run(&mut buf, gen_config, &topts).map_err(|e| e.to_string())?;
                    Ok(buf)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().map_err(|_| "worker thread panicked".to_string())?)
            .collect()
    });

    let mut out = BufWriter::with_capacity(256 * 1024, writer);
    for buf in buffers {
        out.write_all(&buf.map_err(|e| -> Box<dyn std::error::Error> { e.into() })?)?;
    }
    out.flush()?;
    Ok(())
}

pub fn run<W: Write>(
    writer: W,
    gen_config: &GenConfig,
    opts: &RunOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    // Top-level dispatcher: spawn N worker threads when `threads > 1`.
    // Workers call back into this same function with `serial_range` set, which
    // bypasses the check below. Incompatible combinations fall through to the
    // single-threaded path silently (error messages would surprise users who
    // set `--threads` globally).
    if opts.threads > 1 && opts.serial_range.is_none() {
        let unlimited = opts.count == 0;
        let has_aggr = gen_config.columns.iter().any(|c| matches!(c.gen, ColumnGen::Aggr { .. }));
        if !unlimited && !has_aggr {
            return run_threaded(writer, gen_config, opts);
        }
    }

    let mut out = BufWriter::with_capacity(256 * 1024, writer);
    let unlimited = opts.count == 0;
    // Serial range selection, in priority order:
    //   1. `serial_range` (internal, set by threaded dispatcher) — exact [s, e).
    //   2. `shard = Some((i, n))` — slice i of n disjoint contiguous ranges.
    //   3. default — `[0, count)`.
    // Row k is generated from (domain_hash, k), so selecting any subset of k's
    // produces output bit-identical to the corresponding slice of a full run.
    let (shard_start, shard_end) = match (opts.serial_range, opts.shard) {
        (Some(r), _) => r,
        (None, Some((i, n))) if n > 1 && !unlimited => {
            let base = opts.count / n;
            let rem = opts.count % n;
            let start = base * i + i.min(rem);
            let len = base + u64::from(i < rem);
            (start, start + len)
        }
        _ => (0, opts.count),
    };
    let mut serial: u64 = shard_start;

    let rate_interval = opts.rate.map(|r| {
        if r == 0 {
            std::time::Duration::ZERO
        } else {
            std::time::Duration::from_secs_f64(1.0 / r as f64)
        }
    });

    let col_names: Vec<String> = gen_config.columns.iter().map(|v| v.name.clone()).collect();
    let col_count = gen_config.columns.len();

    let domain_hashes = compute_domain_hashes(&gen_config.columns, opts.master_seed);

    let mut script_rng = Rng::derive(opts.master_seed, 0, seedfaker_core::DOMAIN_SCRIPT);
    let effective_locales =
        format::apply_script(&opts.locales.clone(), opts.script, &mut script_rng);
    let effective_refs: Vec<&Locale> = effective_locales.iter().collect();
    let locales: &[&Locale] =
        if opts.script == Script::Native { &effective_refs } else { &opts.locales };

    let resolved_ranges: Vec<Option<(i64, i64)>> = gen_config
        .columns
        .iter()
        .map(|v| match &v.gen {
            ColumnGen::Field { field, range, .. } => resolve_range(range, field.name, opts),
            ColumnGen::Literal(_)
            | ColumnGen::Aggr { .. }
            | ColumnGen::Ref { .. }
            | ColumnGen::Expr { .. }
            | ColumnGen::Fk { .. }
            | ColumnGen::FkDeref { .. } => None,
        })
        .collect();

    validate_resolved_ranges(&gen_config.columns, &resolved_ranges)?;

    let birthdate_range: Option<(i64, i64)> =
        gen_config.columns.iter().find_map(|v| match &v.gen {
            ColumnGen::Field { field, range, .. } if field.name == "birthdate" => {
                resolve_range(range, field.name, opts)
            }
            _ => None,
        });

    let compiled_tpl = match gen_config.template.as_ref() {
        Some(tpl) => {
            let mut name_refs: Vec<&str> = col_names.iter().map(String::as_str).collect();
            name_refs.push("serial");
            Some(crate::tpl::compile::compile(tpl, &name_refs)?)
        }
        None => None,
    };
    let is_text = compiled_tpl.is_some();

    let field_types: Vec<&'static str> = gen_config
        .columns
        .iter()
        .map(|c| match &c.gen {
            ColumnGen::Field { field, .. } => field.name,
            ColumnGen::Literal(_) => "literal",
            ColumnGen::Expr { .. } => "expr",
            ColumnGen::Aggr { .. } => "aggr",
            ColumnGen::Ref { .. } => "ref",
            ColumnGen::Fk { .. } => "fk",
            ColumnGen::FkDeref { .. } => "fk_deref",
        })
        .collect();

    let csv_sep_s = opts.delim.clone().unwrap_or_else(|| ",".to_string());
    let tsv_sep_s = opts.delim.clone().unwrap_or_else(|| "\t".to_string());
    let csv_sep = csv_sep_s.as_bytes();
    let tsv_sep = tsv_sep_s.as_bytes();

    let sql_prefix: Option<Vec<u8>> = if let OutputMode::Sql(ref table) = opts.output {
        let safe = writers::sanitize_identifier(table);
        Some(format!("INSERT INTO {} ({}) VALUES ", safe, col_names.join(", ")).into_bytes())
    } else {
        None
    };

    if !opts.no_header && !is_text && !opts.annotated {
        match opts.output {
            OutputMode::Csv => writers::write_header(&mut out, &col_names, csv_sep)?,
            OutputMode::Tsv => writers::write_header(&mut out, &col_names, tsv_sep)?,
            _ => {}
        }
    }

    let needs_ctx = opts.ctx != Ctx::None;

    let mut aggr = crate::aggr::AggrState::new(&gen_config.columns, &col_names)?;

    let extra = usize::from(compiled_tpl.is_some());
    let mut values: Vec<String> =
        (0..col_count + extra).map(|_| String::with_capacity(32)).collect();
    let mut raw_values: Vec<Option<f64>> = vec![None; col_count];
    let mut is_omitted: Vec<bool> = vec![false; col_count];
    let mut fk_row_indices: Vec<Option<u64>> = vec![None; col_count];
    let mut tpl_buf = String::new();
    let mut itoa_buf = itoa::Buffer::new();

    if opts.validate {
        return Ok(());
    }

    // Pre-compute Zipf CDFs for every FK anchor using `FkDistribution::Zipf(_)`.
    let fk_sampler = build_fk_sampler_cache(&gen_config.columns);

    // Pre-parse enum modifiers once per column — `gen::enum_::gen` re-parses every row.
    let enum_cache = build_enum_cache(&gen_config.columns);

    loop {
        if !unlimited && serial >= shard_end {
            break;
        }

        let both_locales: Vec<Locale>;
        let both_refs: Vec<&Locale>;
        let use_locales: &[&Locale] = if opts.script == Script::Both {
            let mut sr = Rng::derive(opts.master_seed, serial, seedfaker_core::DOMAIN_SCRIPT);
            both_locales = format::apply_script(&opts.locales, opts.script, &mut sr);
            both_refs = both_locales.iter().collect();
            &both_refs
        } else {
            locales
        };

        let locked_locale: Option<Locale> = match opts.ctx {
            Ctx::None => None,
            Ctx::Strict => {
                let mut lr = Rng::derive(opts.master_seed, serial, seedfaker_core::DOMAIN_LOCALE);
                Some(**lr.choice(use_locales))
            }
            Ctx::Loose => {
                let mut lr = Rng::derive(opts.master_seed, serial, seedfaker_core::DOMAIN_LOCALE);
                if lr.maybe(0.7) {
                    Some(**lr.choice(use_locales))
                } else {
                    None
                }
            }
        };
        let locked_ref: &Locale;
        let locked_slice: [&Locale; 1];
        let record_locales: &[&Locale] = if let Some(ref loc) = locked_locale {
            locked_ref = loc;
            locked_slice = [locked_ref];
            &locked_slice
        } else {
            use_locales
        };

        let identity = if needs_ctx {
            let mut ir = Rng::derive(opts.master_seed, serial, seedfaker_core::DOMAIN_IDENTITY);
            Some(Identity::new(&mut ir, record_locales, birthdate_range, opts.since, opts.until))
        } else {
            None
        };
        let id_ref = identity.as_ref();

        let mut ctx = GenContext {
            rng: Rng::new(0),
            locales: record_locales,
            modifier: "",
            identity: id_ref,
            tz_offset_minutes: opts.tz_offset_minutes,
            since: opts.since,
            until: opts.until,
            range: None,
            ordering: Ordering::None,
            zipf: None,
            numeric: None,
        };

        for i in 0..col_count {
            values[i].clear();
            raw_values[i] = None;
            is_omitted[i] = false;
            fk_row_indices[i] = None;
        }

        for &i in &gen_config.eval_order {
            match &gen_config.columns[i].gen {
                ColumnGen::Field {
                    field, modifier, transform, ordering, omit_pct, zipf, ..
                } => {
                    if let Some(pct) = omit_pct {
                        let mut or = Rng::derive(domain_hashes[i], serial, "omit");
                        if or.range(0, 100) < i64::from(*pct) {
                            is_omitted[i] = true;
                            continue;
                        }
                    }
                    ctx.rng = Rng::derive_fast(domain_hashes[i], serial);
                    ctx.modifier = modifier;
                    ctx.range = resolved_ranges[i];
                    ctx.ordering = *ordering;
                    ctx.zipf = *zipf;
                    if let Some(spec) = &enum_cache[i] {
                        spec.sample(&mut ctx.rng, &mut values[i]);
                        raw_values[i] = None;
                    } else {
                        raw_values[i] = field.generate(&mut ctx, &mut values[i]);
                    }
                    if *transform != Transform::None {
                        let s = std::mem::take(&mut values[i]);
                        values[i] = transform.apply(&s);
                    }
                }
                ColumnGen::Literal(s) => {
                    values[i].push_str(s);
                }
                ColumnGen::Aggr { .. } => {}
                ColumnGen::Ref { source_col, modifier } => {
                    if let Some(src_idx) = col_names.iter().position(|n| n == source_col) {
                        raw_values[i] = raw_values[src_idx];
                        if !modifier.is_empty() {
                            if let Some(raw) = raw_values[src_idx] {
                                format_ref(
                                    raw,
                                    modifier,
                                    &gen_config.columns,
                                    src_idx,
                                    &mut values[i],
                                );
                                continue;
                            }
                        }
                        if src_idx < i {
                            let (left, right) = values.split_at_mut(i);
                            right[0].push_str(&left[src_idx]);
                        } else {
                            let (left, right) = values.split_at_mut(src_idx);
                            left[i].push_str(&right[0]);
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
                    let lv = eval_operand(left, &env, &mut ctx, i, true);
                    let rv = eval_operand(right, &env, &mut ctx, i, false);
                    let adjusted_rv = match result_type {
                        crate::tpl::ExprResultType::Date => rv * 86400.0,
                        _ => rv,
                    };
                    let result = match op {
                        crate::tpl::ExprOp::Add => lv + adjusted_rv,
                        crate::tpl::ExprOp::Sub => lv - adjusted_rv,
                        crate::tpl::ExprOp::Mul => lv * rv,
                    };
                    raw_values[i] = Some(result);
                    format_raw_typed(result, *result_type, &mut values[i]);
                }

                ColumnGen::Fk {
                    parent_field,
                    parent_modifier,
                    parent_range,
                    parent_ordering,
                    parent_count,
                    parent_domain_hash,
                    distribution,
                    parent_ctx,
                    ..
                } => {
                    let mut sample_rng = Rng::derive_fast(domain_hashes[i], serial);
                    let row_idx = match distribution {
                        FkDistribution::Uniform => {
                            sample_rng.range(0, *parent_count as i64 - 1) as u64
                        }
                        FkDistribution::Zipf(s) => match &fk_sampler.cdfs[i] {
                            Some(cum) => sample_rng.zipf_from_cdf(cum) - 1,
                            None => sample_rng.zipf(*parent_count, *s) - 1,
                        },
                    };
                    fk_row_indices[i] = Some(row_idx);
                    raw_values[i] = generate_parent_field(
                        parent_field,
                        parent_modifier,
                        parent_range,
                        *parent_ordering,
                        *parent_domain_hash,
                        row_idx,
                        parent_ctx,
                        &mut values[i],
                    );
                }

                ColumnGen::FkDeref {
                    anchor_col,
                    deref_field,
                    deref_modifier,
                    deref_range,
                    deref_ordering,
                    deref_domain_hash,
                    parent_ctx,
                    ..
                } => {
                    let Some(anchor_idx) = col_names.iter().position(|n| n == anchor_col) else {
                        continue;
                    };
                    if let Some(row_idx) = fk_row_indices[anchor_idx] {
                        raw_values[i] = generate_parent_field(
                            deref_field,
                            deref_modifier,
                            deref_range,
                            *deref_ordering,
                            *deref_domain_hash,
                            row_idx,
                            parent_ctx,
                            &mut values[i],
                        );
                    }
                }
            }
        }

        aggr.update(&mut values, &raw_values)?;

        if compiled_tpl.is_some() {
            let idx = col_count;
            values[idx].clear();
            values[idx].push_str(itoa_buf.format(serial));
        }

        let ok = if let Some(ref tpl) = compiled_tpl {
            let mut tpl_rng = Rng::derive(opts.master_seed, serial, seedfaker_core::DOMAIN_TPL);
            let mut rctx = crate::tpl::RenderCtx {
                values: &values,
                rng: &mut tpl_rng,
                locales: record_locales,
                identity: id_ref,
                tz_offset_minutes: opts.tz_offset_minutes,
                since: opts.since,
                until: opts.until,
                field_types: &field_types,
            };
            let mut segments = crate::tpl::render::collect(tpl, &mut rctx);

            let tpl_originals: Option<Vec<String>> = if opts.corrupt == Corrupt::None {
                None
            } else {
                let mut gen_values: Vec<String> = segments
                    .iter()
                    .filter_map(|s| match s {
                        crate::tpl::render::Segment::Value { value, .. } => Some(value.clone()),
                        crate::tpl::render::Segment::Lit(_) => None,
                    })
                    .collect();
                let originals = gen_values.clone();
                let mut cr = Rng::derive(opts.master_seed, serial, seedfaker_core::DOMAIN_CORRUPT);
                seedfaker_core::corrupt::corrupt_values(
                    &mut cr,
                    &mut gen_values,
                    opts.corrupt.rate(),
                );
                let mut vi = 0;
                for seg in &mut segments {
                    if let crate::tpl::render::Segment::Value { value, .. } = seg {
                        *value = std::mem::take(&mut gen_values[vi]);
                        vi += 1;
                    }
                }
                Some(originals)
            };

            if opts.annotated {
                crate::meta::write_annotated_line(&mut out, &segments, tpl_originals.as_deref())?;
                true
            } else {
                tpl_buf.clear();
                crate::tpl::render::assemble_text(&segments, &mut tpl_buf);
                out.write_all(tpl_buf.as_bytes()).is_ok() && out.write_all(b"\n").is_ok()
            }
        } else {
            let struct_originals: Option<Vec<String>> =
                if opts.annotated && opts.corrupt != Corrupt::None {
                    Some(values[..col_count].to_vec())
                } else {
                    None
                };

            if opts.corrupt != Corrupt::None {
                let mut cr = Rng::derive(opts.master_seed, serial, seedfaker_core::DOMAIN_CORRUPT);
                seedfaker_core::corrupt::corrupt_values(&mut cr, &mut values, opts.corrupt.rate());
            }

            if opts.annotated {
                let delim_str = match opts.output {
                    OutputMode::Csv => csv_sep_s.as_str(),
                    _ => tsv_sep_s.as_str(),
                };
                let segments = crate::tpl::render::structured_segments(
                    &col_names,
                    &values,
                    &is_omitted,
                    &field_types,
                    &opts.output,
                    delim_str,
                    sql_prefix.as_deref(),
                );
                crate::meta::write_annotated_line(
                    &mut out,
                    &segments,
                    struct_originals.as_deref(),
                )?;
                true
            } else {
                match opts.output {
                    OutputMode::Default => writers::write_lines(&mut out, &values, tsv_sep),
                    OutputMode::Csv => writers::write_csv(&mut out, &values, csv_sep),
                    OutputMode::Tsv => writers::write_tsv(&mut out, &values, tsv_sep),
                    OutputMode::Jsonl => {
                        writers::write_jsonl(&mut out, &col_names, &values, &is_omitted)
                    }
                    OutputMode::Sql(_) => {
                        if let Some(ref prefix) = sql_prefix {
                            writers::write_sql(&mut out, prefix, &values, &is_omitted)
                        } else {
                            true
                        }
                    }
                    OutputMode::Template => {
                        for v in &values {
                            if out.write_all(v.as_bytes()).is_err() || out.write_all(b"\n").is_err()
                            {
                                break;
                            }
                        }
                        true
                    }
                }
            }
        };

        if !ok {
            break;
        }

        serial += 1;

        if let Some(interval) = rate_interval {
            if out.flush().is_err() {
                return Ok(());
            }
            std::thread::sleep(interval);
        }
    }

    if out.flush().is_err() {
        return Ok(());
    }
    Ok(())
}

pub use seedfaker_core::eval::compute_domain_hashes;

fn resolve_range(
    range: &Option<RangeSpec>,
    field_name: &str,
    opts: &RunOptions,
) -> Option<(i64, i64)> {
    seedfaker_core::field::resolve_range(range, field_name, opts.since, opts.until)
}

pub use seedfaker_core::eval::eval_operand;
pub use seedfaker_core::eval::format_raw_typed;
pub use seedfaker_core::eval::format_ref;
pub use seedfaker_core::eval::ExprEnv;

fn validate_resolved_ranges(
    columns: &[crate::tpl::column::Column],
    ranges: &[Option<(i64, i64)>],
) -> Result<(), Box<dyn std::error::Error>> {
    for (v, range) in columns.iter().zip(ranges.iter()) {
        if let Some((from, to)) = range {
            if from >= to {
                let name = match &v.gen {
                    ColumnGen::Field { field, .. } => field.name,
                    _ => "?",
                };
                return Err(format!("invalid resolved range for '{name}': {from}..{to}").into());
            }
        }
    }
    Ok(())
}
