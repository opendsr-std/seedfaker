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

fn one(req: Value) -> Value {
    call(&[req]).into_iter().next().expect("response")
}

fn gen(args: Value) -> Value {
    one(
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"field","arguments":args}}),
    )
}

fn txt(r: &Value) -> String {
    r["result"]["content"][0]["text"].as_str().unwrap_or("").to_string()
}

fn recs(r: &Value) -> Vec<Value> {
    serde_json::from_str(&txt(r)).expect("json array")
}

fn err(r: &Value) -> String {
    r["error"]["message"].as_str().unwrap_or("").to_string()
}

// ── protocol ────────────────────────────────────────────────────────

#[test]
fn initialize() {
    let r = one(json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}));
    assert_eq!(r["result"]["protocolVersion"], "2024-11-05");
    assert_eq!(r["result"]["serverInfo"]["name"], "seedfaker");
}

#[test]
fn tool_list() {
    let r = one(json!({"jsonrpc":"2.0","id":1,"method":"tools/list"}));
    let tools = r["result"]["tools"].as_array().expect("tools");
    assert_eq!(tools.len(), 4);
    let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    assert!(names.contains(&"field"));
    assert!(names.contains(&"run_preset"));
    assert!(names.contains(&"list_fields"));
    assert!(names.contains(&"fingerprint"));
}

#[test]
fn unknown_method() {
    assert!(one(json!({"jsonrpc":"2.0","id":1,"method":"x"}))["error"].is_object());
}

#[test]
fn unknown_tool() {
    let r = one(
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"x","arguments":{}}}),
    );
    assert!(!err(&r).is_empty());
}

#[test]
fn sequential() {
    let rs = call(&[
        json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}),
        json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"field","arguments":{"fields":["name"],"n":1,"seed":"s"}}}),
    ]);
    assert_eq!(rs.len(), 3);
}

// ── field ────────────────────────────────────────────────────────

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
fn default_n() {
    assert_eq!(recs(&gen(json!({"fields":["name"],"seed":"d"}))).len(), 5);
}

#[test]
fn caps_at_100() {
    assert_eq!(recs(&gen(json!({"fields":["name"],"n":999,"seed":"c"}))).len(), 100);
}

#[test]
fn error_unknown_field() {
    assert!(err(&gen(json!({"fields":["nonexistent"]}))).contains("unknown"));
}

#[test]
fn error_empty_fields() {
    assert!(!err(&gen(json!({"fields":[]}))).is_empty());
}

// ── modifiers, transforms, groups, enums ────────────────────────────

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
    assert!(v[0]["name"].is_string(), "person group should have name");
    assert!(v[0]["first_name"].is_string(), "person group should have first_name");
    assert!(v[0]["birthdate"].is_string(), "person group should have birthdate");
}

#[test]
fn enum_field() {
    for r in &recs(&gen(json!({"fields":["enum:a,b,c"],"n":30,"seed":"e"}))) {
        let v = r["enum_a,b,c"].as_str().expect("enum value");
        assert!(v == "a" || v == "b" || v == "c", "bad: {v}");
    }
}

// ── locale, ctx, corrupt ────────────────────────────────────────────

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

// ── since / until ───────────────────────────────────────────────

#[test]
fn year_range() {
    let v = recs(&gen(json!({"fields":["date"],"n":50,"seed":"yr","since":2020,"until":2022})));
    for r in &v {
        let d = r["date"].as_str().expect("date");
        let year: i64 = d[..4].parse().expect("year");
        assert!((2020..=2022).contains(&year), "MCP year {year} outside 2020..2022");
    }
}

// ── inline range ─────────────────────────────────────────────────────

#[test]
fn inline_range() {
    let v = recs(&gen(json!({"fields":["integer:10..50"],"n":50,"seed":"ir"})));
    for r in &v {
        let val: i64 = r["integer"].as_str().expect("integer key").parse().expect("parse");
        assert!((10..=50).contains(&val), "MCP integer {val} outside 10..50");
    }
}

#[test]
fn date_range_with_modifier() {
    let v = recs(&gen(json!({"fields":["date:2020..2022:eu"],"n":20,"seed":"drm"})));
    for r in &v {
        let d = r["date_eu"].as_str().expect("date_eu key");
        assert!(d.contains('.'), "EU date should use dots: {d}");
        let year: i64 = d[6..].parse().expect("year");
        assert!((2020..=2022).contains(&year), "MCP EU date year {year} outside 2020..2022");
    }
}

// ── ctx=loose ────────────────────────────────────────────────────────

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

// ── list_fields ─────────────────────────────────────────────────────

#[test]
fn list_fields() {
    let r = one(
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_fields","arguments":{}}}),
    );
    let info: Value = serde_json::from_str(&txt(&r)).expect("json");
    assert!(info["groups"].as_array().is_some_and(|a| a.len() >= 15));
    assert!(info["transforms"].as_array().is_some_and(|a| a.len() == 3));
    assert!(info["total_fields"].as_u64().is_some_and(|n| n >= 200));
    assert!(info["locales"].as_array().is_some_and(|a| a.len() >= 50));
}

#[test]
fn list_fields_modifiers() {
    let r = one(
        json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"list_fields","arguments":{}}}),
    );
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
