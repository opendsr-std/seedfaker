use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Stdio};

fn mcp() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_seedfaker"));
    cmd.arg("mcp");
    cmd
}

fn call(requests: &[Value]) -> Vec<Value> {
    let mut cmd = mcp();
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = cmd.spawn().expect("start");
    {
        let w = child.stdin.as_mut().expect("stdin");
        for req in requests {
            serde_json::to_writer(&mut *w, req).expect("write");
            w.write_all(b"\n").expect("nl");
        }
    }
    let out = child.wait_with_output().expect("wait");
    String::from_utf8(out.stdout)
        .expect("utf8")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).expect("json"))
        .collect()
}

fn call_raw(input: &str) -> Vec<Value> {
    let mut cmd = mcp();
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
    let mut child = cmd.spawn().expect("start");
    {
        let w = child.stdin.as_mut().expect("stdin");
        w.write_all(input.as_bytes()).expect("write");
    }
    let out = child.wait_with_output().expect("wait");
    String::from_utf8(out.stdout)
        .expect("utf8")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).expect("json"))
        .collect()
}

fn one(req: Value) -> Value {
    call(&[req]).into_iter().next().expect("response")
}

/// Call generate tool; auto-inject format:jsonl so tests can key by field name.
fn gen(mut args: Value) -> Value {
    if args.get("format").is_none() {
        args["format"] = json!("jsonl");
    }
    one(json!({
        "jsonrpc":"2.0","id":1,"method":"tools/call",
        "params":{"name":"generate","arguments":args}
    }))
}

/// Call generate without injecting format (preserves native TSV).
fn gen_raw(args: Value) -> Value {
    one(json!({
        "jsonrpc":"2.0","id":1,"method":"tools/call",
        "params":{"name":"generate","arguments":args}
    }))
}

fn txt(r: &Value) -> String {
    r["result"]["content"][0]["text"].as_str().unwrap_or("").to_string()
}

fn recs(r: &Value) -> Vec<Value> {
    txt(r)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Value>(l).expect("jsonl record"))
        .collect()
}

fn err(r: &Value) -> String {
    r["error"]["message"].as_str().unwrap_or("").to_string()
}

fn err_code(r: &Value) -> i64 {
    r["error"]["code"].as_i64().unwrap_or(0)
}

// ── protocol ─────────────────────────────────────────────────────────

#[test]
fn initialize() {
    let r = one(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}));
    assert_eq!(r["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(r["result"]["serverInfo"]["name"], "seedfaker");
}

#[test]
fn ping() {
    let r = one(json!({"jsonrpc":"2.0","id":1,"method":"ping"}));
    assert!(r["result"].is_object(), "ping must return a result object");
}

#[test]
fn tool_list() {
    let r = one(json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}));
    let tools = r["result"]["tools"].as_array().expect("tools");
    let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    for expected in [
        "generate",
        "run_preset",
        "validate",
        "replace",
        "list_fields",
        "list_presets",
        "list_locales",
        "list_modifiers",
        "fingerprint",
    ] {
        assert!(names.contains(&expected), "missing tool: {expected}; got {names:?}");
    }
}

#[test]
fn unknown_method() {
    let r = one(json!({"jsonrpc":"2.0","id":1,"method":"no_such_method"}));
    assert_eq!(err_code(&r), -32601, "unknown method must return method_not_found");
}

#[test]
fn unknown_tool() {
    let r = one(json!({"jsonrpc":"2.0","id":1,"method":"tools/call",
        "params":{"name":"nope","arguments":{}}}));
    assert_eq!(err_code(&r), -32602, "unknown tool must return invalid_params");
}

#[test]
fn parse_error() {
    let rs = call_raw("not json\n");
    assert_eq!(err_code(&rs[0]), -32700);
}

#[test]
fn wrong_jsonrpc_version() {
    let r = one(json!({"jsonrpc":"1.0","id":1,"method":"ping"}));
    assert_eq!(err_code(&r), -32600);
}

#[test]
fn notification_has_no_response() {
    // Requests without `id` are notifications — no reply.
    let responses = call(&[
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":99,"method":"ping"}),
    ]);
    assert_eq!(responses.len(), 1, "only the ping should reply");
    assert_eq!(responses[0]["id"], 99);
}

#[test]
fn batched_requests() {
    let batch = json!([
        {"jsonrpc":"2.0","id":1,"method":"ping"},
        {"jsonrpc":"2.0","id":2,"method":"tools/list"},
        {"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"fingerprint","arguments":{}}}
    ]);
    let rs = call(&[batch]);
    assert_eq!(rs.len(), 1, "batch yields a single response line");
    let arr = rs[0].as_array().expect("array response");
    assert_eq!(arr.len(), 3);
    let ids: Vec<i64> = arr.iter().filter_map(|r| r["id"].as_i64()).collect();
    assert_eq!(ids, vec![1, 2, 3]);
}

#[test]
fn sequential() {
    let rs = call(&[
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        json!({"jsonrpc":"2.0","id":3,"method":"tools/call",
            "params":{"name":"generate","arguments":{"fields":["name"],"n":1,"seed":"s","format":"jsonl"}}}),
    ]);
    assert_eq!(rs.len(), 3);
}

// ── generate ─────────────────────────────────────────────────────────

#[test]
fn basic() {
    let v = recs(&gen(json!({"fields":["name","email"],"n":3,"seed":"t"})));
    assert_eq!(v.len(), 3);
    for r in &v {
        assert!(r["email"].as_str().is_some_and(|s| s.contains('@')));
    }
}

#[test]
fn deterministic() {
    let a = json!({"fields":["name","email","phone"],"n":5,"seed":"det"});
    assert_eq!(txt(&gen(a.clone())), txt(&gen(a)));
}

#[test]
fn default_n_is_5() {
    assert_eq!(recs(&gen(json!({"fields":["name"],"seed":"d"}))).len(), 5);
}

#[test]
fn large_n_accepted() {
    // Old MCP capped at 100 silently. New MCP accepts large n up to MAX_COUNT.
    let v = recs(&gen(json!({"fields":["name"],"n":500,"seed":"c"})));
    assert_eq!(v.len(), 500);
}

#[test]
fn n_over_max_rejected() {
    let r = gen(json!({"fields":["name"],"n": 20_000_000_000u64,"seed":"c"}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn seed_required() {
    let r = gen_raw(json!({"fields":["name"]}));
    assert_eq!(err_code(&r), -32602);
    assert!(err(&r).contains("seed"));
}

#[test]
fn error_unknown_field() {
    let r = gen(json!({"fields":["nonexistent"],"seed":"s"}));
    assert_eq!(err_code(&r), -32602);
    assert!(err(&r).contains("unknown"));
}

#[test]
fn error_empty_fields() {
    let r = gen(json!({"fields":[],"seed":"s"}));
    assert_eq!(err_code(&r), -32602);
}

// ── Fail-Fast: invalid inputs must error, not silently fallback ──────

#[test]
fn invalid_ctx_rejected() {
    let r = gen(json!({"fields":["name"],"seed":"s","ctx":"typo"}));
    assert_eq!(err_code(&r), -32602);
    assert!(err(&r).contains("ctx"));
}

#[test]
fn invalid_corrupt_rejected() {
    let r = gen(json!({"fields":["name"],"seed":"s","corrupt":"hgh"}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn invalid_abc_rejected() {
    let r = gen(json!({"fields":["name"],"seed":"s","abc":"bogus"}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn invalid_tz_rejected() {
    let r = gen(json!({"fields":["timestamp"],"seed":"s","tz":"not-an-offset"}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn invalid_locale_rejected() {
    let r = gen(json!({"fields":["name"],"seed":"s","locale":"xx"}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn invalid_format_rejected() {
    let r = gen_raw(json!({"fields":["name"],"seed":"s","format":"xml"}));
    assert_eq!(err_code(&r), -32602);
}

// ── modifiers, transforms, groups, enums ─────────────────────────────

#[test]
fn modifier() {
    for r in &recs(&gen(json!({"fields":["phone:e164"],"n":3,"seed":"m"}))) {
        assert!(r["phone_e164"].as_str().is_some_and(|s| s.starts_with('+')));
    }
}

#[test]
fn transform() {
    for r in &recs(&gen(json!({"fields":["name:upper"],"n":3,"seed":"t"}))) {
        let n = r["name"].as_str().expect("name");
        assert_eq!(n, n.to_uppercase());
    }
}

#[test]
fn modifier_and_transform() {
    for r in &recs(&gen(json!({"fields":["mac:plain:upper"],"n":3,"seed":"mt"}))) {
        let m = r["mac_plain"].as_str().expect("mac");
        assert_eq!(m.len(), 12);
        assert_eq!(m, m.to_uppercase());
    }
}

#[test]
fn group() {
    let v = recs(&gen(json!({"fields":["person"],"n":2,"seed":"g"})));
    assert_eq!(v.len(), 2);
    assert!(v[0]["name"].is_string());
    assert!(v[0]["first_name"].is_string());
    assert!(v[0]["birthdate"].is_string());
}

#[test]
fn enum_field() {
    for r in &recs(&gen(json!({"fields":["enum:a,b,c"],"n":30,"seed":"e"}))) {
        let v = r["enum_a,b,c"].as_str().expect("enum value");
        assert!(v == "a" || v == "b" || v == "c", "bad: {v}");
    }
}

// ── locale, ctx, corrupt ─────────────────────────────────────────────

#[test]
fn locale() {
    let en = txt(&gen(json!({"fields":["name"],"n":5,"seed":"l","locale":"en"})));
    let de = txt(&gen(json!({"fields":["name"],"n":5,"seed":"l","locale":"de"})));
    assert_ne!(en, de);
}

#[test]
fn ctx_strict() {
    let v = recs(&gen(
        json!({"fields":["name","email"],"n":10,"seed":"ctx","ctx":"strict","locale":"en"}),
    ));
    let hits = v
        .iter()
        .filter(|r| {
            let name = r["name"].as_str().unwrap_or("").to_lowercase();
            let email = r["email"].as_str().unwrap_or("").to_lowercase();
            let first = name.split_whitespace().next().unwrap_or("");
            first.len() >= 2 && email.contains(first)
        })
        .count();
    assert!(hits >= 5, "ctx strict: {hits}/10");
}

#[test]
fn corrupt() {
    let clean = txt(&gen(json!({"fields":["name","email"],"n":20,"seed":"cor"})));
    let dirty = txt(&gen(json!({"fields":["name","email"],"n":20,"seed":"cor","corrupt":"high"})));
    assert_ne!(clean, dirty);
}

// ── since / until ────────────────────────────────────────────────────

#[test]
fn year_range() {
    let v = recs(&gen(json!({"fields":["date"],"n":50,"seed":"yr","since":2020,"until":2022})));
    for r in &v {
        let d = r["date"].as_str().expect("date");
        let year: i64 = d[..4].parse().expect("year");
        assert!((2020..=2022).contains(&year), "date year {year} outside 2020..2022");
    }
}

#[test]
fn year_range_as_string() {
    let v = recs(&gen(json!({"fields":["date"],"n":10,"seed":"ys","since":"2020","until":"2022"})));
    for r in &v {
        let d = r["date"].as_str().expect("date");
        let year: i64 = d[..4].parse().expect("year");
        assert!((2020..=2022).contains(&year));
    }
}

#[test]
fn inline_range() {
    let v = recs(&gen(json!({"fields":["integer:10..50"],"n":50,"seed":"ir"})));
    for r in &v {
        let val: i64 = r["integer"].as_str().expect("integer key").parse().expect("parse");
        assert!((10..=50).contains(&val));
    }
}

#[test]
fn date_range_with_modifier() {
    let v = recs(&gen(json!({"fields":["date:2020..2022:eu"],"n":20,"seed":"drm"})));
    for r in &v {
        let d = r["date_eu"].as_str().expect("date_eu key");
        assert!(d.contains('.'), "EU date should use dots: {d}");
        let year: i64 = d[6..].parse().expect("year");
        assert!((2020..=2022).contains(&year));
    }
}

#[test]
fn ctx_loose() {
    let v = recs(&gen(
        json!({"fields":["name","email"],"n":10,"seed":"loose","ctx":"loose","locale":"en"}),
    ));
    assert_eq!(v.len(), 10);
    for r in &v {
        assert!(r["name"].as_str().is_some_and(|s| !s.is_empty()));
        assert!(r["email"].as_str().is_some_and(|s| s.contains('@')));
    }
}

// ── new features: format, template, annotated ────────────────────────

#[test]
fn format_csv() {
    let out = txt(&gen_raw(json!({"fields":["name","email"],"n":3,"seed":"fc","format":"csv"})));
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 4); // header + 3 rows
    assert!(lines[0].contains(','));
}

#[test]
fn format_sql() {
    let out = txt(&gen_raw(json!({
        "fields":["name","email"],"n":2,"seed":"fs","format":"sql=users"
    })));
    assert!(out.contains("INSERT INTO"));
    assert!(out.contains("users"));
}

#[test]
fn template() {
    let out = txt(&gen_raw(json!({
        "fields":["name","email"],"n":2,"seed":"tpl",
        "template":"user={{name}}<{{email}}>"
    })));
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2);
    for line in lines {
        assert!(line.starts_with("user="));
        assert!(line.contains('<') && line.contains('>'));
    }
}

#[test]
fn annotated_produces_spans() {
    let out = txt(&gen_raw(json!({
        "fields":["name","email"],"n":1,"seed":"ann","annotated":true,
        "template":"u={{name}}"
    })));
    // Annotated emits JSONL with text + spans.
    let first: Value = serde_json::from_str(out.lines().next().expect("line")).expect("json");
    assert!(first["text"].is_string());
    assert!(first["spans"].is_array());
}

#[test]
fn no_header() {
    let out = txt(&gen_raw(json!({
        "fields":["name"],"n":2,"seed":"nh","format":"csv","no_header":true
    })));
    assert_eq!(out.lines().count(), 2);
}

// ── run_preset ───────────────────────────────────────────────────────

fn call_tool(name: &str, args: Value) -> Value {
    one(json!({"jsonrpc":"2.0","id":1,"method":"tools/call",
        "params":{"name":name,"arguments":args}}))
}

#[test]
fn run_preset_nginx() {
    let r = call_tool("run_preset", json!({"preset":"nginx","n":2,"seed":"p"}));
    let t = txt(&r);
    assert!(!t.is_empty());
    assert_eq!(t.lines().count(), 2);
}

#[test]
fn run_preset_unknown_errors() {
    let r = call_tool("run_preset", json!({"preset":"no_such_preset","seed":"p"}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn run_preset_requires_seed() {
    let r = call_tool("run_preset", json!({"preset":"nginx"}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn run_preset_overrides() {
    // seedfaker run allows overriding count; MCP must accept same via 'n'.
    let r = call_tool("run_preset", json!({"preset":"nginx","seed":"po","n":5}));
    let t = txt(&r);
    assert_eq!(t.lines().count(), 5);
}

// ── validate ─────────────────────────────────────────────────────────

#[test]
fn validate_ok() {
    let r = call_tool("validate", json!({"fields":["name","email"]}));
    assert_eq!(txt(&r).trim(), "ok");
    assert_eq!(r["result"]["isError"], false);
}

#[test]
fn validate_rejects_unknown_field() {
    let r = call_tool("validate", json!({"fields":["no_such_field"]}));
    assert_eq!(err_code(&r), -32602);
}

#[test]
fn validate_reports_year_range_inversion() {
    let r = call_tool("validate", json!({"fields":["date"],"since":"2025","until":"2020"}));
    let t = txt(&r);
    assert!(t.contains("error:"));
    assert_eq!(r["result"]["isError"], true);
}

// ── list_fields / list_presets ───────────────────────────────────────

#[test]
fn list_fields() {
    let r = call_tool("list_fields", json!({}));
    let info: Value = serde_json::from_str(&txt(&r)).expect("json");
    assert!(info["groups"].as_array().is_some_and(|a| a.len() >= 15));
    assert!(info["transforms"].as_array().is_some_and(|a| a.len() == 3));
    assert!(info["total_fields"].as_u64().is_some_and(|n| n >= 200));
    assert!(info["locales"].as_array().is_some_and(|a| a.len() >= 50));
    assert!(info["presets"].as_array().is_some_and(|a| a.len() >= 10));
    assert!(info["corrupt_levels"].as_array().is_some_and(|a| a.len() == 4));
}

#[test]
fn list_fields_modifiers() {
    let r = call_tool("list_fields", json!({}));
    let info: Value = serde_json::from_str(&txt(&r)).expect("json");
    let finance = info["groups"]
        .as_array()
        .unwrap()
        .iter()
        .find(|g| g["group"] == "finance")
        .expect("finance");
    let cc = finance["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == "credit-card")
        .expect("cc");
    assert!(cc["modifiers"].as_array().is_some_and(|m| m.len() >= 2));
}

#[test]
fn list_presets_includes_nginx() {
    let r = call_tool("list_presets", json!({}));
    let t = txt(&r);
    assert!(t.contains("nginx"));
}

// ── cross-determinism: MCP == CLI byte-identical ─────────────────────

fn run_cli(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_seedfaker")).args(args).output().expect("run cli");
    assert!(out.status.success(), "cli failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("utf8")
}

#[test]
fn cross_det_cli_vs_mcp_default_tsv() {
    let cli_out =
        run_cli(&["name", "email", "phone:e164", "--seed", "xcross", "--until", "2025", "-n", "8"]);
    let mcp_out = txt(&gen_raw(json!({
        "fields":["name","email","phone:e164"],"n":8,"seed":"xcross","until":"2025"
    })));
    assert_eq!(cli_out, mcp_out, "MCP default output must equal CLI stdout byte-for-byte");
}

#[test]
fn cross_det_cli_vs_mcp_ctx_strict_jsonl() {
    let cli_out = run_cli(&[
        "name", "email", "--seed", "xctx", "--until", "2025", "--locale", "en", "--ctx", "strict",
        "--format", "jsonl", "-n", "6",
    ]);
    let mcp_out = txt(&gen_raw(json!({
        "fields":["name","email"],"n":6,"seed":"xctx","until":"2025",
        "locale":"en","ctx":"strict","format":"jsonl"
    })));
    assert_eq!(cli_out, mcp_out);
}

#[test]
fn cross_det_cli_vs_mcp_preset() {
    let cli_out = run_cli(&["run", "nginx", "--seed", "xp", "-n", "5", "--until", "2025"]);
    let mcp_out =
        txt(&call_tool("run_preset", json!({"preset":"nginx","seed":"xp","n":5,"until":"2025"})));
    assert_eq!(cli_out, mcp_out, "MCP preset output must equal CLI run output");
}
