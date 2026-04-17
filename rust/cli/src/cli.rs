use clap::{Args, Parser, Subcommand, ValueEnum};
use std::io::{BufWriter, Write};

use crate::format;
use seedfaker_core::{field, locale, script::Script};

#[derive(Parser)]
#[command(
    name = "seedfaker",
    version,
    about = "Deterministic synthetic generator for realistic, correlated, and noisy test records across 68 locales"
)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[command(flatten)]
    gen: GenArgs,
}

// Shared between generate and run modes.
#[derive(Args, Clone)]
struct SharedOpts {
    /// Deterministic seed — same seed always produces same output
    #[arg(short, long)]
    seed: Option<String>,

    /// Locale(s) with optional weights (e.g. en,de or en=7,es=2,de=1)
    #[arg(short, long, value_delimiter = ',')]
    locale: Vec<String>,

    /// Output format: csv, tsv, jsonl, sql=TABLE
    #[arg(long, short = 'f', value_name = "FORMAT")]
    format: Option<String>,

    /// Omit column header row
    #[arg(long)]
    no_header: bool,

    /// Script mode for non-Latin locales [default: latin]
    #[arg(long, value_enum)]
    abc: Option<AbcMode>,

    /// Context mode — lock fields to one identity per record [default: off]
    #[arg(long, value_enum)]
    ctx: Option<CtxMode>,

    /// Data corruption level [default: off]
    #[arg(long, value_enum)]
    corrupt: Option<CorruptMode>,

    /// Throttle output to N records per second [default: unlimited]
    #[arg(long)]
    rate: Option<u64>,

    /// Temporal range start (year, date, datetime, or epoch) [default: 1900]
    #[arg(long, value_name = "TEMPORAL", allow_hyphen_values = true)]
    since: Option<String>,

    /// Temporal range end, exclusive (year, date, datetime, or epoch) [default: now]
    #[arg(long, value_name = "TEMPORAL", allow_hyphen_values = true)]
    until: Option<String>,

    /// Timezone offset for timestamps (e.g. +03:00, -05:00, Z)
    #[arg(long, allow_hyphen_values = true)]
    tz: Option<String>,

    /// Field delimiter [default: tab]. Any string; supports \t, \n escapes
    #[arg(short = 'd', long = "delim", value_name = "DELIM")]
    delim: Option<String>,

    /// Suppress warnings
    #[arg(short = 'q', long)]
    quiet: bool,

    /// Validate fields and options without generating data
    #[arg(long)]
    validate: bool,

    /// JSONL output with text + spans for every generated value
    #[arg(long)]
    annotated: bool,

    /// Generate only shard I of N (e.g. --shard 0/4). Splits `-n COUNT` into N
    /// disjoint, contiguous serial ranges — each shard can be run in parallel
    /// and its output is identical to the corresponding slice of a non-sharded
    /// run. Requires `-n > 0`. Shards > 0 still emit the header; suppress with
    /// `--no-header` when concatenating shards into a single consumer.
    #[arg(long, value_name = "I/N", allow_hyphen_values = false)]
    shard: Option<String>,

    /// In-process parallel generation. N OS threads split the (sharded or full)
    /// serial range into N disjoint sub-ranges, generate concurrently into
    /// per-thread buffers, and the main thread writes them in serial order.
    /// Output is byte-identical to a single-threaded run. Composes with --shard
    /// (outer shard picks the range, inner threads split it). Requires `-n > 0`.
    /// Falls back to 1 thread when aggregators are used (order-sensitive state).
    #[arg(long, value_name = "N", default_value_t = 1)]
    threads: usize,
}

#[derive(Args)]
struct GenArgs {
    /// Fields, groups, or enums to generate (e.g. name email phone:e164 person)
    fields: Vec<String>,

    /// Number of records (0 = unlimited stream)
    #[arg(short = 'n', long, default_value_t = 10)]
    count: u64,

    /// Inline template with {{field}} placeholders
    #[arg(short = 't', long, value_name = "TEMPLATE")]
    template: Option<String>,

    /// Show all available fields
    #[arg(long)]
    list: bool,

    #[arg(long, hide = true)]
    list_table: bool,

    #[arg(long, hide = true)]
    list_json: bool,

    /// Print algorithm fingerprint (changes when seeded output changes)
    #[arg(long)]
    fingerprint: bool,

    #[command(flatten)]
    opts: SharedOpts,
}

#[derive(Subcommand)]
enum Command {
    /// Run a config file or preset
    Run {
        /// Config file path or preset name
        config_name: Option<String>,

        /// Override record count from config
        #[arg(short = 'n', long)]
        count: Option<u64>,

        /// Show available presets
        #[arg(long)]
        list: bool,

        /// For multi-table configs: which table to generate (stdout)
        #[arg(long, value_name = "TABLE")]
        table: Option<String>,

        /// Generate all tables (requires --output-dir)
        #[arg(long)]
        all: bool,

        /// Output directory for --all; files named {table}.{ext}
        #[arg(long, value_name = "DIR")]
        output_dir: Option<std::path::PathBuf>,

        #[command(flatten)]
        opts: SharedOpts,
    },
    /// Replace columns in CSV/JSONL streams with synthetic values
    Replace {
        /// Column names to replace with synthetic values
        #[arg(required = true)]
        columns: Vec<String>,

        /// Force input format (csv or jsonl) instead of auto-detect
        #[arg(long, value_name = "FORMAT")]
        input_format: Option<String>,

        /// Deterministic seed for replacement
        #[arg(short, long)]
        seed: Option<String>,

        /// End of temporal range for date/timestamp replacement fields
        #[arg(long, value_name = "TEMPORAL", allow_hyphen_values = true)]
        until: Option<String>,

        /// Start of temporal range for date/timestamp replacement fields
        #[arg(long, value_name = "TEMPORAL", allow_hyphen_values = true)]
        since: Option<String>,
    },
    /// Start MCP (Model Context Protocol) server on stdio
    Mcp,
    /// Open the seedfaker quick-start guide in your default browser
    Docs,
}

#[derive(Clone, Copy, ValueEnum)]
enum AbcMode {
    Native,
    Mixed,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum CtxMode {
    Loose,
    Strict,
}

#[derive(Clone, Copy, ValueEnum)]
enum CorruptMode {
    Low,
    Mid,
    High,
    Extreme,
}

fn make_seed(seed: &Option<String>) -> u64 {
    seedfaker_core::opts::resolve_seed(seed.as_deref())
}

fn resolve_locales(
    codes: &[String],
) -> Result<Vec<&'static locale::Locale>, Box<dyn std::error::Error>> {
    Ok(locale::resolve(codes)?)
}

fn resolve_script(abc: Option<AbcMode>, cfg_abc: Option<&str>) -> Script {
    match abc.or_else(|| {
        cfg_abc.and_then(|s| match s {
            "native" => Some(AbcMode::Native),
            "mixed" => Some(AbcMode::Mixed),
            _ => None,
        })
    }) {
        Some(AbcMode::Native) => Script::Native,
        Some(AbcMode::Mixed) => Script::Both,
        None => Script::Latin,
    }
}

fn resolve_ctx(
    ctx: Option<CtxMode>,
    cfg_ctx: Option<&str>,
) -> Result<seedfaker_core::script::Ctx, String> {
    if let Some(mode) = ctx {
        return Ok(match mode {
            CtxMode::Strict => seedfaker_core::script::Ctx::Strict,
            CtxMode::Loose => seedfaker_core::script::Ctx::Loose,
        });
    }
    match cfg_ctx {
        Some(s) if !s.is_empty() => seedfaker_core::opts::resolve_ctx(Some(s)),
        _ => Ok(seedfaker_core::script::Ctx::None),
    }
}

fn resolve_corrupt(
    corrupt: Option<CorruptMode>,
    cfg_corrupt: Option<&str>,
) -> Result<seedfaker_core::script::Corrupt, String> {
    if let Some(mode) = corrupt {
        return Ok(match mode {
            CorruptMode::Low => seedfaker_core::script::Corrupt::Low,
            CorruptMode::Mid => seedfaker_core::script::Corrupt::Mid,
            CorruptMode::High => seedfaker_core::script::Corrupt::High,
            CorruptMode::Extreme => seedfaker_core::script::Corrupt::Extreme,
        });
    }
    match cfg_corrupt {
        Some(s) if !s.is_empty() => {
            let rate = seedfaker_core::opts::resolve_corrupt_rate(Some(s))?;
            Ok(rate.map_or(seedfaker_core::script::Corrupt::None, |_| {
                seedfaker_core::script::Corrupt::parse_level(s)
                    .unwrap_or(seedfaker_core::script::Corrupt::None)
            }))
        }
        _ => Ok(seedfaker_core::script::Corrupt::None),
    }
}

fn resolve_delim(
    cli_delim: &Option<String>,
    cfg_delim: Option<&str>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let Some(s) = cli_delim.as_deref().or(cfg_delim) else {
        return Ok(None);
    };
    let resolved = s.replace("\\t", "\t").replace("\\n", "\n").replace("\\\\", "\\");
    if resolved.is_empty() {
        return Err("delimiter must not be empty".into());
    }
    Ok(Some(resolved))
}

fn resolve_time_opts(
    tz: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<(i32, i64, i64), Box<dyn std::error::Error>> {
    let tz_offset = match tz {
        Some(s) => {
            seedfaker_core::tz::parse(s).map_err(|e| -> Box<dyn std::error::Error> { e.into() })?
        }
        None => 0,
    };
    let since_epoch = match since {
        Some(s) => seedfaker_core::temporal::parse(s)
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?,
        None => seedfaker_core::temporal::DEFAULT_SINCE,
    };
    let until_epoch = match until {
        Some(s) => seedfaker_core::temporal::parse_until(s)
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?,
        None => seedfaker_core::temporal::default_until(),
    };
    Ok((tz_offset, since_epoch, until_epoch))
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Some(Command::Run { config_name, count, list, table, all, output_dir, opts }) => {
            if list {
                println!("presets:");
                for name in crate::config::list_presets() {
                    println!("  {name}");
                }
                return Ok(());
            }
            let config_name = config_name
                .ok_or("config file or preset name required; use --list to see presets")?;
            return run_config(
                &config_name,
                count,
                table.as_deref(),
                all,
                output_dir.as_deref(),
                &opts,
            );
        }
        Some(Command::Replace { columns, input_format, seed, until, since }) => {
            return run_replace(&columns, &input_format, &seed, since.as_deref(), until.as_deref());
        }
        Some(Command::Mcp) => {
            crate::mcp::serve();
            return Ok(());
        }
        Some(Command::Docs) => {
            crate::field_help::open_docs();
            return Ok(());
        }
        None => {}
    }

    let g = &cli.gen;

    if g.list {
        crate::field_help::print_list();
        return Ok(());
    }
    if g.list_table {
        crate::field_help::print_fields_table();
        return Ok(());
    }
    if g.list_json {
        crate::field_help::print_list_json();
        return Ok(());
    }
    if g.fingerprint {
        println!("{}", seedfaker_core::fingerprint());
        return Ok(());
    }

    run_fields(g)
}

fn parse_output_format(s: &str) -> Result<crate::engine::OutputMode, String> {
    match s {
        "csv" => Ok(crate::engine::OutputMode::Csv),
        "tsv" => Ok(crate::engine::OutputMode::Tsv),
        "jsonl" => Ok(crate::engine::OutputMode::Jsonl),
        _ if s.starts_with("sql=") => {
            let table = &s[4..];
            if table.is_empty() {
                return Err("sql= requires a table name".into());
            }
            Ok(crate::engine::OutputMode::Sql(table.to_string()))
        }
        _ => Err(format!("unknown format '{s}'; expected: csv, tsv, jsonl, sql=TABLE")),
    }
}

/// Auto-name for aggregators: `salary:sum` → `sum_salary`, `salary:sum=uid` → `sum_salary_by_uid`.
fn aggr_auto_name(spec: &str) -> String {
    // Find the func segment (contains a known aggr func name)
    let segments: Vec<&str> = spec.split(':').collect();
    for (i, seg) in segments.iter().enumerate() {
        let (func_name, group) = match seg.split_once('=') {
            Some((f, g)) => (f, Some(g)),
            None => (*seg, None),
        };
        if i == 0 {
            continue;
        }
        let source = segments[..i].join("_");
        return match group {
            Some(g) if !g.is_empty() => format!("{func_name}_{source}_by_{g}"),
            _ => format!("{func_name}_{source}"),
        };
    }
    spec.replace(':', "_")
}

fn run_fields(g: &GenArgs) -> Result<(), Box<dyn std::error::Error>> {
    let o = &g.opts;

    let (tz_offset, since, until) =
        resolve_time_opts(o.tz.as_deref(), o.since.as_deref(), o.until.as_deref())?;

    let locales = resolve_locales(&o.locale)?;
    let delim = resolve_delim(&o.delim, None)?;

    let fmt_count = o.format.as_ref().map_or(0, |_| 1) + g.template.as_ref().map_or(0, |_| 1);
    if fmt_count > 1 {
        return Err("use --format or --template, not both".into());
    }

    let is_template = g.template.is_some();

    // Separate expressions, aggregators, and regular fields from CLI tokens.
    let mut regular_tokens = Vec::new();
    let mut aggr_args: Vec<(Option<String>, String)> = Vec::new();
    let mut expr_args: Vec<(String, String)> = Vec::new(); // (alias, spec)

    for token in &g.fields {
        // Parse alias: name=spec (only if `=` comes before `:` and `(`)
        let (alias, spec) = if let Some(eq_pos) = token.find('=') {
            let colon_pos = token.find(':').unwrap_or(token.len());
            let paren_pos = token.find('(').unwrap_or(token.len());
            if eq_pos < colon_pos && eq_pos < paren_pos {
                let (a, s) = token.split_at(eq_pos);
                (Some(a.to_string()), s[1..].to_string())
            } else {
                (None, token.clone())
            }
        } else {
            (None, token.clone())
        };

        // Check if spec contains an expression operator.
        let has_expr_op = spec.contains('+')
            || spec.contains('*')
            || (spec.contains('-') && {
                // Distinguish expression minus from hyphenated field names.
                // If the whole spec is a known field, it's not an expression.
                let is_field = field::parse_field_spec(&spec)
                    .ok()
                    .and_then(|(name, ..)| field::lookup(name))
                    .is_some();
                !is_field
            });

        if has_expr_op && alias.is_some() {
            expr_args.push((alias.clone().unwrap_or_default(), spec));
        } else if crate::tpl::parse_aggr_spec(&spec).is_some() {
            aggr_args.push((alias, spec));
        } else if let Some(a) = alias {
            regular_tokens.push(format!("{a}={spec}"));
        } else {
            regular_tokens.push(spec);
        }
    }

    let fields = if let Some(ref tpl) = g.template {
        if regular_tokens.is_empty() {
            let tpl_fields = format::template_fields(tpl);
            if tpl_fields.is_empty() {
                Vec::new()
            } else {
                field::resolve(&tpl_fields)?
            }
        } else {
            field::resolve(&regular_tokens)?
        }
    } else {
        if regular_tokens.is_empty() && aggr_args.is_empty() && expr_args.is_empty() {
            return Err("no fields specified; example: seedfaker name email phone -n 10".into());
        }
        if regular_tokens.is_empty() {
            Vec::new()
        } else {
            field::resolve(&regular_tokens)?
        }
    };

    if g.count > seedfaker_core::opts::MAX_COUNT {
        return Err("--count must not exceed 10 billion".into());
    }
    if o.rate == Some(0) {
        return Err("--rate must be greater than 0".into());
    }

    {
        let field_infos: Vec<seedfaker_core::validate::FieldInfo<'_>> = fields
            .iter()
            .map(|f| {
                let resolved =
                    seedfaker_core::field::resolve_range(&f.range, f.field.name, since, until);
                seedfaker_core::validate::FieldInfo {
                    name: f.field.name,
                    has_range: f.range.is_some(),
                    resolved_range: resolved,
                    ordering: f.ordering,
                }
            })
            .collect();
        let check = seedfaker_core::validate::CheckCtx {
            fields: &field_infos,
            ctx_strict: o.ctx == Some(crate::cli::CtxMode::Strict),
            since,
            until,
            has_seed: o.seed.is_some(),
            has_until: o.until.is_some(),
            format: o.format.as_deref(),
            corrupt: o.corrupt.map(|c| match c {
                CorruptMode::Low => "low",
                CorruptMode::Mid => "mid",
                CorruptMode::High => "high",
                CorruptMode::Extreme => "extreme",
            }),
            has_template: g.template.is_some(),
        };
        let result = seedfaker_core::validate::validate(&check);
        if !result.errors.is_empty() {
            for e in &result.errors {
                eprintln!("error: {e}");
            }
            return Err("invalid field combination".into());
        }
        if !o.quiet {
            for w in &result.warnings {
                eprintln!("warning: {w}");
            }
        }
    }

    // For templates, var names must match the template placeholders (CLI notation: phone:e164).
    // For structured output, headers use display names (phone_e164).
    let use_display_names = !is_template;
    let mut columns: Vec<crate::tpl::column::Column> = fields
        .iter()
        .map(|rf| {
            let name = if rf.alias.is_some() {
                rf.column_name()
            } else if use_display_names {
                rf.display_name()
            } else {
                let mut s = rf.field.name.to_string();
                if !rf.modifier.is_empty() {
                    s.push(':');
                    s.push_str(&rf.modifier);
                }
                if rf.transform != field::Transform::None {
                    s.push(':');
                    s.push_str(match rf.transform {
                        field::Transform::Upper => "upper",
                        field::Transform::Lower => "lower",
                        field::Transform::Capitalize => "capitalize",
                        field::Transform::None => "",
                    });
                }
                s
            };
            crate::tpl::column::Column {
                name,
                gen: crate::tpl::column::ColumnGen::Field {
                    field: rf.field,
                    modifier: rf.modifier.clone(),
                    transform: rf.transform,
                    range: rf.range,
                    ordering: rf.ordering,
                    omit_pct: rf.omit_pct,
                    zipf: rf.zipf,
                },
            }
        })
        .collect();

    // Append aggregator columns
    for (alias, spec) in &aggr_args {
        let aggr = crate::tpl::parse_aggr_spec(spec).ok_or_else(|| {
            format!("invalid aggregator: '{spec}'; expected source:func or source:func=group")
        })?;
        let name = alias.clone().unwrap_or_else(|| aggr_auto_name(spec));
        columns.push(crate::tpl::Column { name, gen: aggr });
    }

    // Append expression columns
    let all_col_names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
    for (alias, spec) in &expr_args {
        let gen = crate::tpl::resolve_column(alias, spec, &all_col_names)?;
        columns.push(crate::tpl::Column { name: alias.clone(), gen });
    }

    crate::config::resolve_expr_types(&mut columns)?;
    let eval_order = crate::config::topo_sort_columns(&columns)?;
    let gen_config = crate::config::GenConfig {
        columns,
        eval_order,
        template: g.template.clone(),
        options: crate::config::GenConfigOptions::default(),
    };

    let output = if let Some(ref fmt) = o.format {
        parse_output_format(fmt)?
    } else if is_template {
        crate::engine::OutputMode::Template
    } else {
        crate::engine::OutputMode::Default
    };

    let master_seed = make_seed(&o.seed);
    let shard = resolve_shard(o.shard.as_deref(), g.count)?;

    let opts = crate::engine::RunOptions {
        master_seed,
        count: g.count,
        shard,
        threads: o.threads,
        serial_range: None,
        locales,
        script: resolve_script(o.abc, None),
        ctx: resolve_ctx(o.ctx, None)?,
        corrupt: resolve_corrupt(o.corrupt, None)?,
        rate: o.rate,
        no_header: o.no_header,
        output,
        delim,
        validate: o.validate,
        annotated: o.annotated,
        tz_offset_minutes: tz_offset,
        since,
        until,
    };

    crate::engine::run(std::io::stdout().lock(), &gen_config, &opts)
}

fn run_config(
    config_name: &str,
    count: Option<u64>,
    table: Option<&str>,
    all: bool,
    output_dir: Option<&std::path::Path>,
    o: &SharedOpts,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_kind = crate::config::load_config(config_name)?;

    match config_kind {
        crate::config::ConfigKind::Single(gen_config) => {
            if table.is_some() || all || output_dir.is_some() {
                return Err(
                    "--table, --all, --output-dir are only valid for multi-table configs".into()
                );
            }
            run_single_config(*gen_config, count, o)
        }
        crate::config::ConfigKind::Multi(multi) => match (table, all, output_dir) {
            (Some(name), false, None) => run_multi_one_table(&multi, name, count, o),
            (None, true, Some(dir)) => run_all_tables(&multi, count, dir, o),
            (None, true, None) => Err("--all requires --output-dir".into()),
            (Some(_), true, _) => Err("--table and --all are mutually exclusive".into()),
            (Some(_), false, Some(_)) => Err("--output-dir requires --all".into()),
            (None, false, _) => {
                let names: Vec<&str> = multi.tables.iter().map(|(n, _)| n.as_str()).collect();
                Err(format!(
                    "multi-table config requires --table TABLE or --all --output-dir DIR\n\
                         tables: {}",
                    names.join(", ")
                )
                .into())
            }
        },
    }
}

fn parse_shard_spec(spec: &str) -> Result<(u64, u64), String> {
    let (i, n) =
        spec.split_once('/').ok_or_else(|| format!("--shard expects I/N (got '{spec}')"))?;
    let i: u64 = i.parse().map_err(|_| format!("--shard: invalid I in '{spec}'"))?;
    let n: u64 = n.parse().map_err(|_| format!("--shard: invalid N in '{spec}'"))?;
    if n == 0 {
        return Err("--shard: N must be >= 1".into());
    }
    if i >= n {
        return Err(format!("--shard: I must be < N (got {i}/{n})"));
    }
    Ok((i, n))
}

fn resolve_shard(spec: Option<&str>, count: u64) -> Result<Option<(u64, u64)>, String> {
    let Some(s) = spec else { return Ok(None) };
    let (i, n) = parse_shard_spec(s)?;
    if n == 1 {
        return Ok(None);
    }
    if count == 0 {
        return Err("--shard requires -n > 0 (unlimited stream cannot be sharded)".into());
    }
    Ok(Some((i, n)))
}

fn build_run_opts(
    gen_config: &crate::config::GenConfig,
    master_seed: u64,
    count: Option<u64>,
    o: &SharedOpts,
) -> Result<crate::engine::RunOptions, Box<dyn std::error::Error>> {
    let loc_vec: Vec<String> =
        if o.locale.is_empty() { gen_config.options.locale.clone() } else { o.locale.clone() };
    let locales = resolve_locales(&loc_vec)?;

    let effective_n = count.or(gen_config.options.count).unwrap_or(1);
    if effective_n > seedfaker_core::opts::MAX_COUNT {
        return Err("--count must not exceed 10 billion".into());
    }

    let eff_tz = o.tz.as_deref().or(gen_config.options.tz.as_deref());
    let eff_since = o.since.as_deref().or(gen_config.options.since.as_deref());
    let eff_until = o.until.as_deref().or(gen_config.options.until.as_deref());
    let (tz_offset, since, until) = resolve_time_opts(eff_tz, eff_since, eff_until)?;

    let script = resolve_script(o.abc, gen_config.options.abc.as_deref());
    let ctx_mode = resolve_ctx(o.ctx, gen_config.options.ctx.as_deref())?;
    let corrupt_mode = resolve_corrupt(o.corrupt, gen_config.options.corrupt.as_deref())?;
    let effective_rate = o.rate.or(gen_config.options.rate);
    let delim_char = resolve_delim(&o.delim, gen_config.options.delim.as_deref())?;

    let effective_format = o.format.as_deref().or(gen_config.options.format.as_deref());

    let output = if let Some(fmt) = effective_format {
        parse_output_format(fmt)?
    } else if gen_config.template.is_some() {
        crate::engine::OutputMode::Template
    } else {
        crate::engine::OutputMode::Default
    };

    let shard = resolve_shard(o.shard.as_deref(), effective_n)?;

    Ok(crate::engine::RunOptions {
        master_seed,
        count: effective_n,
        shard,
        threads: o.threads,
        serial_range: None,
        locales,
        script,
        ctx: ctx_mode,
        corrupt: corrupt_mode,
        rate: effective_rate,
        no_header: o.no_header || gen_config.options.no_header,
        output,
        delim: delim_char,
        validate: o.validate || gen_config.options.validate,
        annotated: o.annotated || gen_config.options.annotated,
        tz_offset_minutes: tz_offset,
        since,
        until,
    })
}

fn run_single_config(
    gen_config: crate::config::GenConfig,
    count: Option<u64>,
    o: &SharedOpts,
) -> Result<(), Box<dyn std::error::Error>> {
    let effective_seed = o.seed.as_ref().or(gen_config.options.seed.as_ref());
    let master_seed = if let Some(s) = effective_seed {
        seedfaker_core::hash_seed(s)
    } else {
        seedfaker_core::rng::random_seed()
    };
    let opts = build_run_opts(&gen_config, master_seed, count, o)?;
    crate::engine::run(std::io::stdout().lock(), &gen_config, &opts)
}

fn prepare_table(
    multi: &crate::config::MultiTableConfig,
    table_name: &str,
    count: Option<u64>,
    o: &SharedOpts,
) -> Result<(crate::config::GenConfig, crate::engine::RunOptions), Box<dyn std::error::Error>> {
    let (_, table_cfg) = multi.find_table(table_name)?;
    let mut cfg = table_cfg.clone();
    crate::engine::finalize_fk_columns(&mut cfg, multi.global_seed, &multi.tables);
    let table_seed = seedfaker_core::rng::domain_hash(multi.global_seed, table_name);
    let opts = build_run_opts(&cfg, table_seed, count, o)?;
    Ok((cfg, opts))
}

fn run_multi_one_table(
    multi: &crate::config::MultiTableConfig,
    table_name: &str,
    count: Option<u64>,
    o: &SharedOpts,
) -> Result<(), Box<dyn std::error::Error>> {
    let (cfg, opts) = prepare_table(multi, table_name, count, o)?;
    crate::engine::run(std::io::stdout().lock(), &cfg, &opts)
}

fn run_all_tables(
    multi: &crate::config::MultiTableConfig,
    count: Option<u64>,
    output_dir: &std::path::Path,
    o: &SharedOpts,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(output_dir)?;

    for (table_name, _) in &multi.tables {
        let (cfg, opts) = prepare_table(multi, table_name, count, o)?;
        let ext = crate::engine::format_ext(&opts.output);
        let path = output_dir.join(format!("{table_name}.{ext}"));
        let file = std::fs::File::create(&path)?;
        crate::engine::run(file, &cfg, &opts)?;
        eprintln!("{table_name} \u{2192} {}", path.display());
    }
    Ok(())
}

/// Max line length for replace stdin: 256 MB.
/// Covers TEXT/BLOB fields in database dumps. Rejects binary/corrupt input.
const MAX_LINE_BYTES: usize = 256 * 1024 * 1024;

fn read_line_bounded(
    reader: &mut impl std::io::BufRead,
    buf: &mut String,
) -> Result<bool, Box<dyn std::error::Error>> {
    buf.clear();
    let n = reader.read_line(buf)?;
    if n == 0 {
        return Ok(false);
    }
    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }
    if buf.len() > MAX_LINE_BYTES {
        return Err(format!(
            "input line too long: {} bytes (max {} MB). \
             Check input format — binary files are not supported.",
            buf.len(),
            MAX_LINE_BYTES / (1024 * 1024)
        )
        .into());
    }
    Ok(true)
}

fn run_replace(
    columns: &[String],
    input_format: &Option<String>,
    seed: &Option<String>,
    since: Option<&str>,
    until: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let since_epoch = match since {
        Some(s) => seedfaker_core::temporal::parse(s)?,
        None => seedfaker_core::temporal::DEFAULT_SINCE,
    };
    let until_epoch = match until {
        Some(s) => seedfaker_core::temporal::parse_until(s)?,
        None => seedfaker_core::temporal::default_until(),
    };

    for col in columns {
        if col.contains(':') {
            return Err(format!(
                "modifiers not supported in replace mode: '{col}'; use plain column names"
            )
            .into());
        }
    }

    if let Some(ref fmt) = input_format {
        if fmt != "csv" && fmt != "jsonl" {
            return Err(format!("--input-format must be 'csv' or 'jsonl', got '{fmt}'").into());
        }
    }

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let mut reader = std::io::BufReader::new(stdin.lock());

    let mut first_line = String::new();
    if !read_line_bounded(&mut reader, &mut first_line)? {
        return Ok(());
    }

    let jsonl_mode = match input_format.as_deref() {
        Some("jsonl") => true,
        Some("csv") => false,
        _ => first_line.trim_start().starts_with('{'),
    };

    if jsonl_mode {
        let mut process_jsonl = |line: &str| -> Result<(), Box<dyn std::error::Error>> {
            if line.is_empty() {
                return Ok(());
            }
            let mut obj: serde_json::Map<String, serde_json::Value> = serde_json::from_str(line)?;

            for col in columns {
                if let Some(val) = obj.get(col) {
                    if val.is_null() || val.as_str().is_some_and(str::is_empty) {
                        continue;
                    }
                    let original = val.to_string();
                    let replacement =
                        generate_replacement(&original, col, seed, since_epoch, until_epoch);
                    obj.insert(col.clone(), serde_json::Value::String(replacement));
                }
            }

            serde_json::to_writer(&mut out, &obj)?;
            out.write_all(b"\n")?;
            Ok(())
        };

        process_jsonl(&first_line)?;
        let mut line_buf = String::new();
        while read_line_bounded(&mut reader, &mut line_buf)? {
            process_jsonl(&line_buf)?;
        }
    } else {
        let mut col_indices: Vec<usize> = Vec::new();
        let headers: Vec<&str> = first_line.split(',').collect();
        for col in columns {
            if let Some(idx) = headers.iter().position(|h| h.trim_matches('"') == col.as_str()) {
                col_indices.push(idx);
            } else {
                let available: Vec<&str> = headers.iter().map(|h| h.trim_matches('"')).collect();
                return Err(format!(
                    "column '{col}' not found in header; available: {}",
                    available.join(", ")
                )
                .into());
            }
        }
        writeln!(out, "{first_line}")?;

        let mut line_buf = String::new();
        while read_line_bounded(&mut reader, &mut line_buf)? {
            let fields = parse_csv_line(&line_buf);
            let mut output_fields: Vec<String> = Vec::new();

            for (i, field_val) in fields.iter().enumerate() {
                if col_indices.contains(&i) {
                    let trimmed = field_val.trim_matches('"');
                    if trimmed.is_empty() {
                        output_fields.push(field_val.clone());
                    } else {
                        let col_pos = col_indices.iter().position(|&idx| idx == i);
                        let col_name =
                            col_pos.and_then(|p| columns.get(p)).map_or("", String::as_str);
                        let replacement =
                            generate_replacement(trimmed, col_name, seed, since_epoch, until_epoch);
                        if field_val.starts_with('"') {
                            output_fields.push(format!("\"{}\"", replacement.replace('"', "\"\"")));
                        } else {
                            output_fields.push(replacement);
                        }
                    }
                } else {
                    output_fields.push(field_val.clone());
                }
            }

            writeln!(out, "{}", output_fields.join(","))?;
        }
    }

    let _ = out.flush();
    Ok(())
}

fn generate_replacement(
    original: &str,
    column_name: &str,
    seed: &Option<String>,
    since: i64,
    until: i64,
) -> String {
    let hash_input = if let Some(s) = seed {
        format!("{s}\x00{original}")
    } else {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0);
        format!("{t}\x00{original}")
    };
    let seed_val = seedfaker_core::hash_seed(&hash_input);
    let mut rng = seedfaker_core::rng::Rng::new(seed_val);
    let en_locale = seedfaker_core::locale::get("en");
    let locales: Vec<&seedfaker_core::locale::Locale> = match en_locale {
        Some(loc) => vec![loc],
        None => return rng.alnum(original.len().max(8)),
    };

    if let Some(f) = seedfaker_core::field::lookup(column_name) {
        let mut ctx = seedfaker_core::ctx::GenContext {
            rng,
            locales: &locales,
            modifier: "",
            identity: None,
            tz_offset_minutes: seedfaker_core::DEFAULT_TZ_OFFSET,
            since,
            until,
            range: None,
            ordering: seedfaker_core::field::Ordering::None,
            zipf: None,
            numeric: None,
        };
        let mut val_buf = String::new();
        f.generate(&mut ctx, &mut val_buf);
        return val_buf;
    }

    rng.alnum(original.len().max(8))
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if in_quotes {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                    current.push('"');
                }
            } else {
                current.push(ch);
            }
        } else if ch == '"' {
            in_quotes = true;
            current.push('"');
        } else if ch == ',' {
            fields.push(current.clone());
            current.clear();
        } else {
            current.push(ch);
        }
    }
    fields.push(current);
    fields
}
