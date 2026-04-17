use seedfaker_core::{field, locale, pipeline, rng::Rng, script::Corrupt};
use serde_json::{json, Map, Value};
use std::io::{BufRead, BufReader, Write};

use crate::config;

pub fn serve() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut line = String::new();

    while {
        line.clear();
        reader.read_line(&mut line).unwrap_or(0) > 0
    } {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let req: Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(e) => {
                let err = json!({"jsonrpc": "2.0", "id": null, "error": {"code": -32700, "message": format!("parse error: {e}")}});
                if serde_json::to_writer(&mut out, &err).is_err()
                    || out.write_all(b"\n").is_err()
                    || out.flush().is_err()
                {
                    break;
                }
                continue;
            }
        };
        let id = req.get("id").cloned();
        let method = req["method"].as_str().unwrap_or("");
        let params = req.get("params").cloned().unwrap_or(json!({}));

        let response = match dispatch(method, &params) {
            Ok(val) => match id {
                Some(id) => json!({"jsonrpc": "2.0", "id": id, "result": val}),
                None => continue,
            },
            Err(e) => match id {
                Some(id) => {
                    json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32603, "message": e}})
                }
                None => continue,
            },
        };

        if serde_json::to_writer(&mut out, &response).is_err()
            || out.write_all(b"\n").is_err()
            || out.flush().is_err()
        {
            break;
        }
    }
}

fn dispatch(method: &str, params: &Value) -> Result<Value, String> {
    match method {
        "initialize" => Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {"tools": {}},
            "serverInfo": {"name": "seedfaker", "version": env!("CARGO_PKG_VERSION")}
        })),
        "notifications/initialized" => Ok(Value::Null),
        "tools/list" => Ok(tools_list()),
        "tools/call" => {
            let name = params["name"].as_str().unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match name {
                "field" => generate(&args),
                "run_preset" => run_preset(&args),
                "list_fields" => Ok(list_fields()),
                "fingerprint" => Ok(text(seedfaker_core::fingerprint().as_str())),
                _ => Err(format!("unknown tool: {name}")),
            }
        }
        _ => Err(format!("unknown method: {method}")),
    }
}

fn text(s: &str) -> Value {
    json!({"content": [{"type": "text", "text": s}]})
}

fn tools_list() -> Value {
    json!({"tools": [
        {
            "name": "field",
            "description": concat!(
                "Generate synthetic data: PII, credentials, finance, gov-id, ",
                "healthcare, dev/ops. 200+ fields, 68 locales, deterministic with seed. ",
                "Supports modifiers (phone:e164, credit-card:space), ",
                "transforms (:upper, :lower, :capitalize), ",
                "groups (person, auth, finance, all), ",
                "enum:val1,val2, and data corruption."
            ),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "fields": {
                        "type": "array", "items": {"type": "string"},
                        "description": concat!(
                            "Fields, groups, or enums. Examples: ",
                            "'name', 'phone:e164', 'credit-card:space:upper', ",
                            "'person', 'enum:admin,user'. ",
                            "Use list_fields to see all."
                        )
                    },
                    "n":       {"type": "integer", "description": "Record count (1-100, default 5)", "default": 5},
                    "seed":    {"type": "string",  "description": "Deterministic seed"},
                    "locale":  {"type": "string",  "description": "Comma-separated locales (default: all)"},
                    "ctx":     {"type": "string",  "enum": ["strict", "loose"], "description": "Correlate name/email/username within records"},
                    "corrupt": {"type": "string",  "enum": ["low", "mid", "high", "extreme"], "description": "Data corruption level"},
                    "abc":      {"type": "string",  "enum": ["native", "mixed"], "description": "Script mode: native (locale script) or mixed (Latin + locale)"},
                    "tz":       {"type": "string",  "description": "Timezone offset (e.g. +0300, -08:00, Z)"},
                    "since": {"type": "integer", "description": "Temporal range start as epoch seconds (default: epoch of 1900)"},
                    "until":   {"type": "integer", "description": "Temporal range end as epoch seconds (default: now)"}
                },
                "required": ["fields"]
            }
        },
        {
            "name": "run_preset",
            "description": concat!(
                "Run a preset config: nginx, auth, app-json, postgres, ",
                "payment, pii-leak, user-table, email, stacktrace, chaos, ",
                "llm-prompt, syslog, medical."
            ),
            "inputSchema": {
                "type": "object",
                "properties": {
                    "preset": {"type": "string", "description": "Preset name"},
                    "n":      {"type": "integer", "description": "Record count (1-100, default 5)"},
                    "seed":   {"type": "string",  "description": "Deterministic seed"}
                },
                "required": ["preset"]
            }
        },
        {
            "name": "list_fields",
            "description": "List all fields, groups, modifiers, transforms, and locales.",
            "inputSchema": {"type": "object", "properties": {}}
        },
        {
            "name": "fingerprint",
            "description": "Return the generator fingerprint. Changes when seeded output would change.",
            "inputSchema": {"type": "object", "properties": {}}
        }
    ]})
}

fn generate(args: &Value) -> Result<Value, String> {
    let specs = str_array(args, "fields")?;
    if specs.is_empty() {
        return Err("'fields' required".into());
    }

    let n = args["n"].as_u64().unwrap_or(5).min(100);
    let seed = args["seed"].as_str().unwrap_or("mcp");
    let base_locales = resolve_locales(args)?;
    let script = match args["abc"].as_str() {
        Some("native") => seedfaker_core::script::Script::Native,
        Some("mixed") => seedfaker_core::script::Script::Both,
        _ => seedfaker_core::script::Script::Latin,
    };
    let master = seedfaker_core::hash_seed(seed);
    let owned_locales;
    let locales: Vec<&locale::Locale> = if script == seedfaker_core::script::Script::Latin {
        base_locales
    } else {
        let mut sr = Rng::derive(master, 0, seedfaker_core::DOMAIN_SCRIPT);
        owned_locales = crate::format::apply_script(&base_locales, script, &mut sr);
        owned_locales.iter().collect()
    };
    let resolved = field::resolve(&specs)?;
    let ctx = match args["ctx"].as_str() {
        Some("strict") => seedfaker_core::script::Ctx::Strict,
        Some("loose") => seedfaker_core::script::Ctx::Loose,
        _ => seedfaker_core::script::Ctx::None,
    };
    let corrupt_rate = args["corrupt"].as_str().and_then(Corrupt::parse_level).map(Corrupt::rate);
    let tz_offset =
        args["tz"].as_str().and_then(|s| seedfaker_core::tz::parse(s).ok()).unwrap_or(0);
    let since = match &args["since"] {
        serde_json::Value::Number(n) => {
            let v = n.as_i64().unwrap_or(0);
            seedfaker_core::temporal::parse(&v.to_string())
                .unwrap_or(seedfaker_core::temporal::DEFAULT_SINCE)
        }
        serde_json::Value::String(s) => {
            seedfaker_core::temporal::parse(s).unwrap_or(seedfaker_core::temporal::DEFAULT_SINCE)
        }
        _ => seedfaker_core::temporal::DEFAULT_SINCE,
    };
    let until = match &args["until"] {
        serde_json::Value::Number(n) => {
            let v = n.as_i64().unwrap_or(0);
            seedfaker_core::temporal::parse_until(&v.to_string())
                .unwrap_or(seedfaker_core::temporal::default_until())
        }
        serde_json::Value::String(s) => seedfaker_core::temporal::parse_until(s)
            .unwrap_or(seedfaker_core::temporal::default_until()),
        _ => seedfaker_core::temporal::default_until(),
    };

    let display_names: Vec<String> =
        resolved.iter().map(field::ResolvedField::display_name).collect();

    let field_specs: Vec<pipeline::FieldSpec<'_>> = resolved
        .iter()
        .map(|rf| pipeline::FieldSpec {
            field: rf.field,
            modifier: &rf.modifier,
            domain_hash: pipeline::field_domain_hash(master, rf.field, &rf.modifier),
            range: field::resolve_range(&rf.range, rf.field.name, since, until),
            transform: rf.transform,
            omit_pct: rf.omit_pct,
        })
        .collect();

    let opts = pipeline::RecordOpts {
        master_seed: master,
        locales: &locales,
        ctx,
        corrupt_rate,
        tz_offset_minutes: tz_offset,
        since,
        until,
    };

    let raw_records = pipeline::generate_records(&opts, &field_specs, n, 0);

    let records: Vec<Value> = raw_records
        .into_iter()
        .map(|vals| {
            let mut obj = Map::new();
            for (i, val) in vals.into_iter().enumerate() {
                obj.insert(display_names[i].clone(), Value::String(val));
            }
            Value::Object(obj)
        })
        .collect();

    let out = serde_json::to_string_pretty(&records)
        .map_err(|e| format!("JSON serialization failed: {e}"))?;
    Ok(text(&out))
}

fn list_fields() -> Value {
    let groups: Vec<Value> = field::GROUPS
        .iter()
        .map(|group| {
            let fields: Vec<Value> = field::REGISTRY
                .iter()
                .filter(|f| f.group == *group)
                .map(|f| {
                    let m = field::field_modifiers(f.id);
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
        "total_fields": field::REGISTRY.len(),
        "locales": locale::ALL_CODES,
    });
    text(&serde_json::to_string_pretty(&info).unwrap_or_else(|_| "{}".into()))
}

fn run_preset(args: &Value) -> Result<Value, String> {
    let preset = args["preset"].as_str().ok_or("'preset' required")?;
    let n = args["n"].as_u64().unwrap_or(5).min(100);
    let seed = args["seed"].as_str().unwrap_or("mcp");

    let gen_config = match config::load_config(preset)? {
        config::ConfigKind::Single(gc) => *gc,
        config::ConfigKind::Multi(_) => {
            return Err("multi-table configs not supported in MCP".into())
        }
    };
    let master = seedfaker_core::hash_seed(seed);

    let locales = locale::resolve(&[]).map_err(|e| format!("locale error: {e}"))?;

    let col_names: Vec<&str> = gen_config.columns.iter().map(|v| v.name.as_str()).collect();

    let domain_hashes = crate::engine::compute_domain_hashes(&gen_config.columns, master);

    use crate::tpl::column::ColumnGen;
    let tpl = gen_config
        .template
        .as_ref()
        .map(|t| crate::tpl::compile::compile(t, &col_names))
        .transpose()
        .map_err(|e| format!("template error: {e}"))?;

    let col_count = gen_config.columns.len();
    let mut aggr = crate::aggr::AggrState::new(&gen_config.columns, &col_names)?;

    let col_name_strings: Vec<String> = col_names.iter().map(|s| (*s).to_string()).collect();
    let mut lines = Vec::with_capacity(n as usize);
    for serial in 0..n {
        let mut values: Vec<String> = (0..col_count).map(|_| String::new()).collect();
        let mut raw_values: Vec<Option<f64>> = vec![None; col_count];
        let mut ctx = seedfaker_core::ctx::GenContext {
            rng: Rng::new(0),
            locales: &locales,
            modifier: "",
            identity: None,
            tz_offset_minutes: seedfaker_core::DEFAULT_TZ_OFFSET,
            since: seedfaker_core::temporal::DEFAULT_SINCE,
            until: seedfaker_core::temporal::default_until(),
            range: None,
            ordering: seedfaker_core::field::Ordering::None,
            zipf: None,
            numeric: None,
        };
        for &i in &gen_config.eval_order {
            values[i].clear();
            match &gen_config.columns[i].gen {
                ColumnGen::Field { field, modifier, omit_pct, .. } => {
                    if let Some(pct) = omit_pct {
                        let mut or = Rng::derive(domain_hashes[i], serial, "omit");
                        if or.range(0, 100) < i64::from(*pct) {
                            continue;
                        }
                    }
                    ctx.rng = Rng::derive_fast(domain_hashes[i], serial);
                    ctx.modifier = modifier;
                    ctx.range = None;
                    ctx.ordering = seedfaker_core::field::Ordering::None;
                    raw_values[i] = field.generate(&mut ctx, &mut values[i]);
                }
                ColumnGen::Literal(s) => {
                    values[i].push_str(s);
                }
                ColumnGen::Aggr { .. } | ColumnGen::Fk { .. } | ColumnGen::FkDeref { .. } => {}
                ColumnGen::Ref { source_col, modifier } => {
                    if let Some(src_idx) = col_names.iter().position(|n| n == source_col) {
                        raw_values[i] = raw_values[src_idx];
                        if modifier.is_empty() {
                            let src = values[src_idx].clone();
                            values[i].push_str(&src);
                        } else if let Some(raw) = raw_values[src_idx] {
                            crate::engine::format_ref(
                                raw,
                                modifier,
                                &gen_config.columns,
                                src_idx,
                                &mut values[i],
                            );
                        } else {
                            let src = values[src_idx].clone();
                            values[i].push_str(&src);
                        }
                    }
                }
                ColumnGen::Expr { left, op, right, result_type } => {
                    let env = crate::engine::ExprEnv {
                        raw_values: &raw_values,
                        col_names: &col_name_strings,
                        domain_hashes: &domain_hashes,
                        serial,
                    };
                    let lv = crate::engine::eval_operand(left, &env, &mut ctx, i, true);
                    let rv = crate::engine::eval_operand(right, &env, &mut ctx, i, false);
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
                    crate::engine::format_raw_typed(result, *result_type, &mut values[i]);
                }
            }
        }

        aggr.update(&mut values, &raw_values)?;

        if let Some(tpl) = &tpl {
            let mut tpl_rng = Rng::derive(master, serial, seedfaker_core::DOMAIN_TPL);
            let ft: Vec<&'static str> = Vec::new();
            let mut rctx = crate::tpl::RenderCtx {
                values: &values,
                rng: &mut tpl_rng,
                locales: &locales,
                identity: None,
                tz_offset_minutes: seedfaker_core::DEFAULT_TZ_OFFSET,
                since: seedfaker_core::temporal::DEFAULT_SINCE,
                until: seedfaker_core::temporal::default_until(),
                field_types: &ft,
            };
            let mut buf = String::new();
            crate::tpl::render::render(tpl, &mut rctx, &mut buf);
            lines.push(buf);
        } else {
            lines.push(values.join("\t"));
        }
    }

    Ok(text(&lines.join("\n")))
}

fn str_array(args: &Value, key: &str) -> Result<Vec<String>, String> {
    args[key]
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).map(String::from).collect())
        .ok_or_else(|| format!("'{key}' must be an array"))
}

fn resolve_locales(args: &Value) -> Result<Vec<&'static locale::Locale>, String> {
    match args["locale"].as_str() {
        Some(s) if !s.is_empty() => locale::resolve_str(s),
        _ => locale::resolve(&[]),
    }
}
