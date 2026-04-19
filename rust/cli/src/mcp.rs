//! Model Context Protocol server (stdio JSON-RPC 2.0).
//!
//! Protocol: <https://spec.modelcontextprotocol.io/specification/2024-11-05>.
//! All generation goes through `engine::run` — no duplicate column-gen logic.
//! Invalid input → JSON-RPC error, never silent fallback.

use seedfaker_core::{
    locale, opts,
    script::{Corrupt, Script},
};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};

use crate::cli;
use crate::config::{self, ConfigKind};
use crate::engine::{self, OutputMode, RunOptions};
use crate::field_help;

// JSON-RPC 2.0 error codes.
const PARSE_ERROR: i32 = -32700;
const INVALID_REQUEST: i32 = -32600;
const METHOD_NOT_FOUND: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;
const INTERNAL_ERROR: i32 = -32603;

struct RpcError {
    code: i32,
    message: String,
}

impl RpcError {
    fn invalid_params(m: impl Into<String>) -> Self {
        Self { code: INVALID_PARAMS, message: m.into() }
    }
    fn method_not_found(m: impl Into<String>) -> Self {
        Self { code: METHOD_NOT_FOUND, message: m.into() }
    }
    fn internal(m: impl Into<String>) -> Self {
        Self { code: INTERNAL_ERROR, message: m.into() }
    }
}

pub fn serve() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut line = String::new();

    loop {
        line.clear();
        let Ok(n) = reader.read_line(&mut line) else { break };
        if n == 0 {
            break;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let req: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                let resp = error_value(Value::Null, PARSE_ERROR, format!("parse error: {e}"));
                if !write_value(&mut out, &resp) {
                    return;
                }
                continue;
            }
        };

        // Batched (JSON-RPC 2.0 §6): an array of request objects → array of responses.
        if let Some(arr) = req.as_array() {
            if arr.is_empty() {
                let resp = error_value(Value::Null, INVALID_REQUEST, "empty batch".into());
                if !write_value(&mut out, &resp) {
                    return;
                }
                continue;
            }
            let responses: Vec<Value> = arr.iter().filter_map(handle_one).collect();
            if responses.is_empty() {
                continue; // all were notifications
            }
            if !write_value(&mut out, &Value::Array(responses)) {
                return;
            }
        } else if let Some(resp) = handle_one(&req) {
            if !write_value(&mut out, &resp) {
                return;
            }
        }
    }
}

fn write_value(out: &mut impl Write, v: &Value) -> bool {
    serde_json::to_writer(&mut *out, v).is_ok()
        && out.write_all(b"\n").is_ok()
        && out.flush().is_ok()
}

fn error_value(id: Value, code: i32, message: String) -> Value {
    json!({"jsonrpc":"2.0","id":id,"error":{"code":code,"message":message}})
}

fn handle_one(req: &Value) -> Option<Value> {
    let id = req.get("id").cloned();
    let is_notification = id.is_none() || matches!(id, Some(Value::Null));

    if req.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        if is_notification {
            return None;
        }
        return Some(error_value(
            id.unwrap_or(Value::Null),
            INVALID_REQUEST,
            "invalid jsonrpc version; expected '2.0'".into(),
        ));
    }

    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or_else(|| json!({}));

    let result = dispatch(method, &params);

    if is_notification {
        return None;
    }

    let id = id.unwrap_or(Value::Null);
    match result {
        Ok(val) => Some(json!({"jsonrpc":"2.0","id":id,"result":val})),
        Err(e) => Some(error_value(id, e.code, e.message)),
    }
}

fn dispatch(method: &str, params: &Value) -> Result<Value, RpcError> {
    match method {
        "initialize" => Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {"tools": {"listChanged": false}},
            "serverInfo": {"name": "seedfaker", "version": env!("CARGO_PKG_VERSION")}
        })),
        "notifications/initialized" | "notifications/cancelled" => Ok(Value::Null),
        "ping" => Ok(json!({})),
        "tools/list" => Ok(tools_list()),
        "tools/call" => {
            let name = params
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| RpcError::invalid_params("'name' required in tools/call"))?;
            let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
            match name {
                "generate" => tool_generate(&args),
                "run_preset" => tool_run_preset(&args),
                "validate" => tool_validate(&args),
                "list_fields" => Ok(list_fields()),
                "list_presets" => Ok(list_presets()),
                "fingerprint" => Ok(text(seedfaker_core::fingerprint().as_str())),
                other => Err(RpcError::invalid_params(format!("unknown tool: {other}"))),
            }
        }
        other => Err(RpcError::method_not_found(format!("unknown method: {other}"))),
    }
}

fn text(s: &str) -> Value {
    json!({"content":[{"type":"text","text":s}]})
}

// ── argument extractors (Fail-Fast) ──────────────────────────────────

fn as_str_array(v: &Value, key: &str) -> Result<Vec<String>, RpcError> {
    match v.get(key) {
        Some(Value::Array(a)) => {
            let mut out = Vec::with_capacity(a.len());
            for item in a {
                match item.as_str() {
                    Some(s) => out.push(s.to_string()),
                    None => {
                        return Err(RpcError::invalid_params(format!(
                            "'{key}' must be an array of strings"
                        )))
                    }
                }
            }
            Ok(out)
        }
        Some(_) => Err(RpcError::invalid_params(format!("'{key}' must be an array"))),
        None => Err(RpcError::invalid_params(format!("'{key}' required"))),
    }
}

fn opt_str<'a>(v: &'a Value, key: &str) -> Result<Option<&'a str>, RpcError> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.as_str())),
        Some(_) => Err(RpcError::invalid_params(format!("'{key}' must be a string"))),
    }
}

fn opt_u64(v: &Value, key: &str) -> Result<Option<u64>, RpcError> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n.as_u64().map(Some).ok_or_else(|| {
            RpcError::invalid_params(format!("'{key}' must be a non-negative integer"))
        }),
        Some(_) => Err(RpcError::invalid_params(format!("'{key}' must be an integer"))),
    }
}

fn opt_bool(v: &Value, key: &str) -> Result<Option<bool>, RpcError> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(b)) => Ok(Some(*b)),
        Some(_) => Err(RpcError::invalid_params(format!("'{key}' must be a boolean"))),
    }
}

/// Accept either an integer (epoch seconds) or a string temporal ("2024", "2024-01-15", etc).
fn opt_temporal(v: &Value, key: &str) -> Result<Option<String>, RpcError> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(Value::Number(n)) => Ok(Some(n.to_string())),
        Some(_) => Err(RpcError::invalid_params(format!(
            "'{key}' must be a string (year/date/datetime) or integer (epoch seconds)"
        ))),
    }
}

fn require_seed(args: &Value) -> Result<&str, RpcError> {
    match opt_str(args, "seed")? {
        Some(s) if !s.is_empty() => Ok(s),
        _ => Err(RpcError::invalid_params(
            "'seed' required — MCP refuses non-deterministic output",
        )),
    }
}

fn parse_locale(s: Option<&str>) -> Result<Vec<&'static locale::Locale>, RpcError> {
    match s {
        Some(s) if !s.is_empty() => locale::resolve_str(s).map_err(RpcError::invalid_params),
        _ => locale::resolve(&[]).map_err(RpcError::invalid_params),
    }
}

fn parse_corrupt(s: Option<&str>) -> Result<Corrupt, RpcError> {
    match s {
        Some(s) if !s.is_empty() => Corrupt::parse_level(s).ok_or_else(|| {
            RpcError::invalid_params(format!(
                "unknown corrupt level: '{s}' (expected 'low', 'mid', 'high', or 'extreme')"
            ))
        }),
        _ => Ok(Corrupt::None),
    }
}

fn parse_script(s: Option<&str>) -> Result<Script, RpcError> {
    match s {
        Some("native") => Ok(Script::Native),
        Some("mixed") => Ok(Script::Both),
        Some(other) => Err(RpcError::invalid_params(format!(
            "unknown abc mode: '{other}' (expected 'native' or 'mixed')"
        ))),
        None => Ok(Script::Latin),
    }
}

fn parse_delim(s: Option<&str>) -> Result<Option<String>, RpcError> {
    match s {
        Some("") => Err(RpcError::invalid_params("'delim' must not be empty")),
        Some(s) => Ok(Some(s.replace("\\t", "\t").replace("\\n", "\n").replace("\\\\", "\\"))),
        None => Ok(None),
    }
}

fn check_count(n: u64) -> Result<(), RpcError> {
    if n > opts::MAX_COUNT {
        return Err(RpcError::invalid_params(format!(
            "'n' must not exceed {}",
            opts::MAX_COUNT
        )));
    }
    Ok(())
}

// ── tools/list schema ────────────────────────────────────────────────

fn shared_opts_schema() -> Value {
    json!({
        "seed":      {"type": "string",  "description": "Deterministic seed (required)"},
        "locale":    {"type": "string",  "description": "Comma-separated locale codes, optionally weighted (en=7,de=2). Default: all locales"},
        "ctx":       {"type": "string",  "enum": ["strict", "loose"], "description": "Lock fields to one identity per record"},
        "corrupt":   {"type": "string",  "enum": ["low", "mid", "high", "extreme"], "description": "Data corruption level"},
        "abc":       {"type": "string",  "enum": ["native", "mixed"], "description": "Script mode: native (locale script) or mixed (Latin + locale)"},
        "tz":        {"type": "string",  "description": "Timezone offset for timestamps (e.g. +03:00, -08:00, Z)"},
        "since":     {"description": "Temporal range start — string year/date/datetime or integer epoch seconds. Default: 1900"},
        "until":     {"description": "Temporal range end, exclusive — string year/date/datetime or integer epoch seconds. Default: now"},
        "format":    {"type": "string",  "description": "Output format: csv, tsv, jsonl, sql=TABLE. Default: tsv"},
        "delim":     {"type": "string",  "description": "Field delimiter for default/tsv format (supports \\t, \\n escapes)"},
        "no_header": {"type": "boolean", "description": "Omit column header row"},
        "annotated": {"type": "boolean", "description": "JSONL with text + byte-offset spans (NER/PII training)"}
    })
}

fn tools_list() -> Value {
    let mut gen_props = shared_opts_schema();
    gen_props["fields"] = json!({
        "type": "array",
        "items": {"type": "string"},
        "description": "Fields, groups, enums, aggregators, or expressions. Examples: 'name', 'phone:e164', 'person', 'enum:admin,user', 'integer:1..100', 'total=price*qty', 'amount:sum'. Use list_fields to enumerate."
    });
    gen_props["n"] = json!({
        "type": "integer",
        "description": format!("Record count, 1..={} (0 = unlimited streaming, not recommended for MCP). Default: 5", opts::MAX_COUNT)
    });
    gen_props["template"] = json!({
        "type": "string",
        "description": "Inline template with {{field}} placeholders; supports {{serial}}, {{#if}}, {{#repeat}}"
    });

    let mut preset_props = shared_opts_schema();
    preset_props["preset"] = json!({
        "type": "string",
        "description": "Preset name or path to config file. Use list_presets to enumerate built-in presets."
    });
    preset_props["n"] = json!({
        "type": "integer",
        "description": format!("Record count override, 1..={}. Defaults to config's count or 1", opts::MAX_COUNT)
    });
    preset_props["table"] = json!({
        "type": "string",
        "description": "For multi-table configs: which table to generate"
    });

    let mut validate_props = shared_opts_schema();
    validate_props["fields"] = json!({
        "type": "array",
        "items": {"type": "string"},
        "description": "Fields to validate (same syntax as generate)"
    });
    validate_props["template"] = json!({
        "type": "string",
        "description": "Optional template to validate alongside fields"
    });

    json!({"tools": [
        {
            "name": "generate",
            "description": "Generate synthetic records. Full CLI surface: 200+ fields, 68 locales, groups, enums, modifiers, transforms, aggregators, expressions, ranges, templates, corruption, annotated output.",
            "inputSchema": {
                "type": "object",
                "properties": gen_props,
                "required": ["fields", "seed"]
            }
        },
        {
            "name": "run_preset",
            "description": "Run a built-in preset or config file. Supports multi-table configs via 'table' parameter. All generate options apply as overrides.",
            "inputSchema": {
                "type": "object",
                "properties": preset_props,
                "required": ["preset", "seed"]
            }
        },
        {
            "name": "validate",
            "description": "Validate field specs and options without generating data. Returns errors and warnings.",
            "inputSchema": {
                "type": "object",
                "properties": validate_props,
                "required": ["fields"]
            }
        },
        {
            "name": "list_fields",
            "description": "List all fields, groups, modifiers, transforms, and locales.",
            "inputSchema": {"type": "object", "properties": {}}
        },
        {
            "name": "list_presets",
            "description": "List built-in preset names.",
            "inputSchema": {"type": "object", "properties": {}}
        },
        {
            "name": "fingerprint",
            "description": "Return the generator fingerprint. Changes when seeded output would change.",
            "inputSchema": {"type": "object", "properties": {}}
        }
    ]})
}

// ── tool: generate ───────────────────────────────────────────────────

fn tool_generate(args: &Value) -> Result<Value, RpcError> {
    let tokens = as_str_array(args, "fields")?;
    if tokens.is_empty() {
        return Err(RpcError::invalid_params("'fields' must not be empty"));
    }
    let seed = require_seed(args)?;
    let n = opt_u64(args, "n")?.unwrap_or(5);
    check_count(n)?;

    let template = opt_str(args, "template")?.map(str::to_string);
    let format_str = opt_str(args, "format")?;
    if format_str.is_some() && template.is_some() {
        return Err(RpcError::invalid_params("use 'format' or 'template', not both"));
    }

    let tz = opt_str(args, "tz")?;
    let since_s = opt_temporal(args, "since")?;
    let until_s = opt_temporal(args, "until")?;
    let (tz_offset, since, until) =
        cli::resolve_time_opts(tz, since_s.as_deref(), until_s.as_deref())
            .map_err(|e| RpcError::invalid_params(e.to_string()))?;

    let locales = parse_locale(opt_str(args, "locale")?)?;
    let ctx_str = opt_str(args, "ctx")?;
    let ctx = opts::resolve_ctx(ctx_str).map_err(RpcError::invalid_params)?;
    let corrupt = parse_corrupt(opt_str(args, "corrupt")?)?;
    let script = parse_script(opt_str(args, "abc")?)?;
    let delim = parse_delim(opt_str(args, "delim")?)?;
    let no_header = opt_bool(args, "no_header")?.unwrap_or(false);
    let annotated = opt_bool(args, "annotated")?.unwrap_or(false);

    let (gen_config, fields, is_template) =
        cli::build_gen_config_from_tokens(&tokens, template.clone())
            .map_err(|e| RpcError::invalid_params(e.to_string()))?;

    let check = cli::run_check_ctx(&cli::CheckInput {
        fields: &fields,
        since,
        until,
        ctx_strict: ctx_str == Some("strict"),
        has_seed: true,
        has_until: until_s.is_some(),
        format: format_str,
        corrupt: opt_str(args, "corrupt")?,
        has_template: is_template,
    });
    if !check.errors.is_empty() {
        return Err(RpcError::invalid_params(check.errors.join("; ")));
    }

    let output = match format_str {
        Some(s) => cli::parse_output_format(s).map_err(RpcError::invalid_params)?,
        None if is_template => OutputMode::Template,
        None => OutputMode::Default,
    };

    let run_opts = RunOptions {
        master_seed: seedfaker_core::hash_seed(seed),
        count: n,
        shard: None,
        threads: 1,
        serial_range: None,
        locales,
        script,
        ctx,
        corrupt,
        rate: None,
        no_header,
        output,
        delim,
        validate: false,
        annotated,
        tz_offset_minutes: tz_offset,
        since,
        until,
    };

    let mut buf: Vec<u8> = Vec::with_capacity(64 * 1024);
    engine::run(&mut buf, &gen_config, &run_opts)
        .map_err(|e| RpcError::internal(e.to_string()))?;
    let s = String::from_utf8(buf)
        .map_err(|e| RpcError::internal(format!("non-utf8 output: {e}")))?;
    Ok(text(&s))
}

// ── tool: run_preset ─────────────────────────────────────────────────

fn tool_run_preset(args: &Value) -> Result<Value, RpcError> {
    let preset_name = opt_str(args, "preset")?
        .ok_or_else(|| RpcError::invalid_params("'preset' required"))?;
    let seed = require_seed(args)?;
    let n_override = opt_u64(args, "n")?;
    let table_arg = opt_str(args, "table")?;

    let config_kind = config::load_config(preset_name).map_err(RpcError::invalid_params)?;

    let (gen_config, master_seed_base) = match config_kind {
        ConfigKind::Single(gc) => {
            if table_arg.is_some() {
                return Err(RpcError::invalid_params(
                    "'table' is only valid for multi-table configs",
                ));
            }
            (*gc, None)
        }
        ConfigKind::Multi(multi) => {
            let name = table_arg.ok_or_else(|| {
                let names: Vec<&str> = multi.tables.iter().map(|(n, _)| n.as_str()).collect();
                RpcError::invalid_params(format!(
                    "multi-table preset requires 'table'; available: {}",
                    names.join(", ")
                ))
            })?;
            let (_, t) = multi.find_table(name).map_err(|e| RpcError::invalid_params(e.to_string()))?;
            let mut cfg = t.clone();
            engine::finalize_fk_columns(&mut cfg, multi.global_seed, &multi.tables);
            let table_seed = seedfaker_core::rng::domain_hash(multi.global_seed, name);
            (cfg, Some(table_seed))
        }
    };

    let effective_n = n_override.or(gen_config.options.count).unwrap_or(1);
    check_count(effective_n)?;

    let tz = opt_str(args, "tz")?.or(gen_config.options.tz.as_deref());
    let since_s = opt_temporal(args, "since")?;
    let until_s = opt_temporal(args, "until")?;
    let since_eff = since_s.as_deref().or(gen_config.options.since.as_deref());
    let until_eff = until_s.as_deref().or(gen_config.options.until.as_deref());
    let (tz_offset, since, until) = cli::resolve_time_opts(tz, since_eff, until_eff)
        .map_err(|e| RpcError::invalid_params(e.to_string()))?;

    let locale_arg = opt_str(args, "locale")?;
    let locales = match locale_arg {
        Some(s) if !s.is_empty() => locale::resolve_str(s).map_err(RpcError::invalid_params)?,
        _ => {
            if gen_config.options.locale.is_empty() {
                locale::resolve(&[]).map_err(RpcError::invalid_params)?
            } else {
                locale::resolve(&gen_config.options.locale).map_err(RpcError::invalid_params)?
            }
        }
    };

    let ctx = opts::resolve_ctx(opt_str(args, "ctx")?.or(gen_config.options.ctx.as_deref()))
        .map_err(RpcError::invalid_params)?;
    let corrupt =
        parse_corrupt(opt_str(args, "corrupt")?.or(gen_config.options.corrupt.as_deref()))?;
    let script = parse_script(opt_str(args, "abc")?.or(gen_config.options.abc.as_deref()))?;

    let format_eff = opt_str(args, "format")?.or(gen_config.options.format.as_deref());
    let output = match format_eff {
        Some(s) => cli::parse_output_format(s).map_err(RpcError::invalid_params)?,
        None if gen_config.template.is_some() => OutputMode::Template,
        None => OutputMode::Default,
    };

    let delim_arg = opt_str(args, "delim")?;
    let delim = match delim_arg {
        Some(s) => parse_delim(Some(s))?,
        None => match gen_config.options.delim.as_deref() {
            Some(s) => parse_delim(Some(s))?,
            None => None,
        },
    };

    let no_header = opt_bool(args, "no_header")?.unwrap_or(gen_config.options.no_header);
    let annotated = opt_bool(args, "annotated")?.unwrap_or(gen_config.options.annotated);

    let master_seed = master_seed_base.unwrap_or_else(|| seedfaker_core::hash_seed(seed));

    let run_opts = RunOptions {
        master_seed,
        count: effective_n,
        shard: None,
        threads: 1,
        serial_range: None,
        locales,
        script,
        ctx,
        corrupt,
        rate: None,
        no_header,
        output,
        delim,
        validate: gen_config.options.validate,
        annotated,
        tz_offset_minutes: tz_offset,
        since,
        until,
    };

    let mut buf: Vec<u8> = Vec::with_capacity(64 * 1024);
    engine::run(&mut buf, &gen_config, &run_opts)
        .map_err(|e| RpcError::internal(e.to_string()))?;
    let s = String::from_utf8(buf)
        .map_err(|e| RpcError::internal(format!("non-utf8 output: {e}")))?;
    Ok(text(&s))
}

// ── tool: validate ───────────────────────────────────────────────────

fn tool_validate(args: &Value) -> Result<Value, RpcError> {
    let tokens = as_str_array(args, "fields")?;
    if tokens.is_empty() {
        return Err(RpcError::invalid_params("'fields' must not be empty"));
    }
    let template = opt_str(args, "template")?.map(str::to_string);

    // Token parsing errors surface immediately.
    let (_gen_config, fields, is_template) =
        cli::build_gen_config_from_tokens(&tokens, template.clone())
            .map_err(|e| RpcError::invalid_params(e.to_string()))?;

    let tz = opt_str(args, "tz")?;
    let since_s = opt_temporal(args, "since")?;
    let until_s = opt_temporal(args, "until")?;
    let (_tz_offset, since, until) =
        cli::resolve_time_opts(tz, since_s.as_deref(), until_s.as_deref())
            .map_err(|e| RpcError::invalid_params(e.to_string()))?;

    let ctx_str = opt_str(args, "ctx")?;
    let format_str = opt_str(args, "format")?;
    let corrupt_str = opt_str(args, "corrupt")?;
    let has_seed = opt_str(args, "seed")?.is_some();

    let result = cli::run_check_ctx(&cli::CheckInput {
        fields: &fields,
        since,
        until,
        ctx_strict: ctx_str == Some("strict"),
        has_seed,
        has_until: until_s.is_some(),
        format: format_str,
        corrupt: corrupt_str,
        has_template: is_template,
    });

    let is_error = !result.errors.is_empty();
    let mut body = String::new();
    if result.errors.is_empty() && result.warnings.is_empty() {
        body.push_str("ok");
    } else {
        for e in &result.errors {
            body.push_str("error: ");
            body.push_str(e);
            body.push('\n');
        }
        for w in &result.warnings {
            body.push_str("warning: ");
            body.push_str(w);
            body.push('\n');
        }
    }
    Ok(json!({
        "content": [{"type":"text","text": body}],
        "isError": is_error,
    }))
}

// ── tool: list_fields / list_presets ─────────────────────────────────

fn list_fields() -> Value {
    let groups: Vec<Value> = seedfaker_core::field::GROUPS
        .iter()
        .map(|group| {
            let fields: Vec<Value> = seedfaker_core::field::REGISTRY
                .iter()
                .filter(|f| f.group == *group)
                .map(|f| {
                    let m = seedfaker_core::field::field_modifiers(f.id);
                    let mods: Vec<&str> =
                        if m.is_empty() { vec![] } else { m.split(", ").collect() };
                    json!({"name": f.name, "description": f.description, "modifiers": mods})
                })
                .collect();
            json!({"group": group, "fields": fields})
        })
        .collect();

    let info = json!({
        "groups": groups,
        "transforms": ["upper", "lower", "capitalize"],
        "total_fields": seedfaker_core::field::REGISTRY.len(),
        "locales": locale::ALL_CODES,
        "presets": config::list_presets(),
        "corrupt_levels": ["low", "mid", "high", "extreme"],
    });
    let _ = &field_help::open_docs; // keep module link — referenced by docs tool
    match serde_json::to_string_pretty(&info) {
        Ok(s) => text(&s),
        Err(e) => text(&format!("serialization error: {e}")),
    }
}

fn list_presets() -> Value {
    let names = config::list_presets();
    text(&names.join("\n"))
}
