/// Cross-interface determinism: CLI, MCP, Python, Node.js MUST produce
/// identical output for the same seed + fields + options.
///
/// This is the core product guarantee — any divergence is a ship-blocker.
mod common;
use common::run_ok;

// ---------------------------------------------------------------------------
// MCP helpers
// ---------------------------------------------------------------------------

fn mcp_call(requests: &[&str]) -> Vec<serde_json::Value> {
    let bin = env!("CARGO_BIN_EXE_seedfaker");
    let mut child = std::process::Command::new(bin)
        .arg("mcp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn mcp");

    use std::io::Write;
    let stdin = child.stdin.as_mut().expect("stdin");
    for req in requests {
        stdin.write_all(req.as_bytes()).expect("write");
        stdin.write_all(b"\n").expect("nl");
    }
    drop(child.stdin.take());
    let output = child.wait_with_output().expect("wait");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).expect("json"))
        .collect()
}

fn mcp_generate(args_json: &str) -> Vec<serde_json::Value> {
    let init = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}"#;
    let call = format!(
        r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"field","arguments":{args_json}}}}}"#
    );
    let responses = mcp_call(&[init, &call]);
    assert!(responses.len() >= 2, "expected at least 2 MCP responses");
    let text = responses[1]["result"]["content"][0]["text"].as_str().expect("text");
    serde_json::from_str(text).expect("parse records")
}

fn mcp_records_to_tsv(records: &[serde_json::Value], fields: &[&str]) -> Vec<String> {
    records
        .iter()
        .map(|r| fields.iter().map(|f| r[f].as_str().unwrap_or("")).collect::<Vec<_>>().join("\t"))
        .collect()
}

// ---------------------------------------------------------------------------
// CLI vs MCP: basic fields
// ---------------------------------------------------------------------------

#[test]
fn cli_vs_mcp_basic() {
    let cli_out = run_ok(&["name", "email", "--seed", "xface", "--until", "2025", "-n", "5"]);
    let records = mcp_generate(r#"{"fields":["name","email"],"n":5,"seed":"xface","until":2025}"#);
    let mcp_lines = mcp_records_to_tsv(&records, &["name", "email"]);
    let cli_lines: Vec<&str> = cli_out.lines().collect();
    assert_eq!(cli_lines, mcp_lines, "CLI and MCP output differs");
}

// ---------------------------------------------------------------------------
// CLI vs MCP: ctx strict
// ---------------------------------------------------------------------------

#[test]
fn cli_vs_mcp_ctx_strict() {
    let cli_out = run_ok(&[
        "name", "email", "--seed", "xctx", "--until", "2025", "--locale", "en", "--ctx", "strict",
        "-n", "5",
    ]);
    let records = mcp_generate(
        r#"{"fields":["name","email"],"n":5,"seed":"xctx","until":2025,"locale":"en","ctx":"strict"}"#,
    );
    let mcp_lines = mcp_records_to_tsv(&records, &["name", "email"]);
    let cli_lines: Vec<&str> = cli_out.lines().collect();
    assert_eq!(cli_lines, mcp_lines, "CLI and MCP ctx=strict differs");
}

// ---------------------------------------------------------------------------
// CLI vs MCP: date fields with year range
// ---------------------------------------------------------------------------

#[test]
fn cli_vs_mcp_date_fields() {
    let cli_out = run_ok(&[
        "date",
        "timestamp",
        "--seed",
        "xdt",
        "-n",
        "5",
        "--since",
        "2020",
        "--until",
        "2025",
    ]);
    let records = mcp_generate(
        r#"{"fields":["date","timestamp"],"n":5,"seed":"xdt","since":2020,"until":2025}"#,
    );
    let mcp_lines = mcp_records_to_tsv(&records, &["date", "timestamp"]);
    let cli_lines: Vec<&str> = cli_out.lines().collect();
    assert_eq!(cli_lines, mcp_lines, "CLI and MCP date output differs");
}

// ---------------------------------------------------------------------------
// CLI vs MCP: modifiers
// ---------------------------------------------------------------------------

#[test]
fn cli_vs_mcp_modifiers() {
    let cli_out =
        run_ok(&["phone:e164", "date:us", "--seed", "xmod", "--until", "2025", "-n", "5"]);
    let records =
        mcp_generate(r#"{"fields":["phone:e164","date:us"],"n":5,"seed":"xmod","until":2025}"#);
    let mcp_lines = mcp_records_to_tsv(&records, &["phone_e164", "date_us"]);
    let cli_lines: Vec<&str> = cli_out.lines().collect();
    assert_eq!(cli_lines, mcp_lines, "CLI and MCP modifier output differs");
}

// ---------------------------------------------------------------------------
// CLI vs MCP: corruption
// ---------------------------------------------------------------------------

#[test]
fn cli_vs_mcp_corrupt() {
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "xcor",
        "--until",
        "2025",
        "--corrupt",
        "high",
        "-n",
        "10",
    ]);
    let records = mcp_generate(
        r#"{"fields":["name","email"],"n":10,"seed":"xcor","until":2025,"corrupt":"high"}"#,
    );
    let mcp_lines = mcp_records_to_tsv(&records, &["name", "email"]);
    let cli_lines: Vec<&str> = cli_out.lines().collect();
    assert_eq!(cli_lines, mcp_lines, "CLI and MCP corrupt output differs");
}

// ---------------------------------------------------------------------------
// Python native bindings
// ---------------------------------------------------------------------------

fn project_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn python_available() -> bool {
    project_root().join("packages/pip/_seedfaker.so").exists()
}

fn run_python(code: &str) -> String {
    let pip_path = project_root().join("packages/pip");
    let out = std::process::Command::new("python3")
        .arg("-c")
        .arg(code)
        .env("PYTHONPATH", &pip_path)
        .output()
        .expect("python3");
    assert!(out.status.success(), "python failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("utf8")
}

#[test]
fn python_native_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_out =
        run_ok(&["name", "email", "phone", "--seed", "bind-py", "--until", "2025", "-n", "5"]);
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-py", until=2025)
for r in f.records(["name", "email", "phone"], n=5):
    print(f"{r['name']}\t{r['email']}\t{r['phone']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python native differs from CLI");
}

#[test]
fn python_ctx_strict_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_out = run_ok(&[
        "name", "email", "--seed", "bind-ctx", "--until", "2025", "--locale", "en", "--ctx",
        "strict", "-n", "5",
    ]);
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-ctx", locale="en", until=2025)
for r in f.records(["name", "email"], n=5, ctx="strict"):
    print(f"{r['name']}\t{r['email']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python ctx=strict differs from CLI");
}

#[test]
fn python_date_with_year_range_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_out = run_ok(&[
        "date",
        "birthdate",
        "--seed",
        "bind-yr",
        "--locale",
        "en",
        "--since",
        "2020",
        "--until",
        "2025",
        "-n",
        "5",
    ]);
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-yr", locale="en", since=2020, until=2025)
for r in f.records(["date", "birthdate"], n=5):
    print(f"{r['date']}\t{r['birthdate']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python since/until differs from CLI");
}

// ---------------------------------------------------------------------------
// Python: inline range in field spec
// ---------------------------------------------------------------------------

#[test]
fn python_inline_range_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_out = run_ok(&[
        "integer:100..200",
        "date:2020..2022",
        "--seed",
        "bind-range",
        "--locale",
        "en",
        "-n",
        "5",
    ]);
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-range", locale="en", until=2025)
for r in f.records(["integer:100..200", "date:2020..2022"], n=5):
    print(f"{r['integer']}\t{r['date']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python inline range differs from CLI");
}

// ---------------------------------------------------------------------------
// Python: ctx=loose matches CLI
// ---------------------------------------------------------------------------

#[test]
fn python_ctx_loose_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "bind-loose",
        "--locale",
        "en",
        "--ctx",
        "loose",
        "-n",
        "10",
    ]);
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-loose", locale="en", until=2025)
for r in f.records(["name", "email"], n=10, ctx="loose"):
    print(f"{r['name']}\t{r['email']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python ctx=loose differs from CLI");
}

// ---------------------------------------------------------------------------
// Python: corruption matches CLI
// ---------------------------------------------------------------------------

#[test]
fn python_corrupt_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "bind-cor",
        "--locale",
        "en",
        "--corrupt",
        "high",
        "-n",
        "10",
    ]);
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-cor", locale="en", until=2025)
for r in f.records(["name", "email"], n=10, corrupt="high"):
    print(f"{r['name']}\t{r['email']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python corrupt differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js native bindings
// ---------------------------------------------------------------------------

fn node_available() -> bool {
    let p = project_root().join("packages/npm/seedfaker_napi.node");
    if !p.exists() {
        return false;
    }
    // Verify the .node file is loadable on this platform (not a cross-platform leftover)
    let out = std::process::Command::new("node")
        .arg("-e")
        .arg(format!("try {{ require('{}'); }} catch(e) {{ process.exit(1); }}", p.display()))
        .output();
    matches!(out, Ok(o) if o.status.success())
}

fn run_node(code: &str) -> String {
    let npm_path = project_root().join("packages/npm");
    let full_code = format!("const {{ SeedFaker }} = require('{}');\n{}", npm_path.display(), code);
    let out = std::process::Command::new("node").arg("-e").arg(&full_code).output().expect("node");
    assert!(out.status.success(), "node failed: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("utf8")
}

#[test]
fn node_native_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out =
        run_ok(&["name", "email", "phone", "--seed", "bind-js", "--until", "2025", "-n", "5"]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-js' });
const rows = f.records(['name', 'email', 'phone'], { n: 5 });
rows.forEach(r => console.log(r.name + '\t' + r.email + '\t' + r.phone));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js native differs from CLI");
}

#[test]
fn node_date_with_year_range_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out = run_ok(&[
        "date",
        "birthdate",
        "--seed",
        "bind-yr",
        "--locale",
        "en",
        "--since",
        "2020",
        "--until",
        "2025",
        "-n",
        "5",
    ]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-yr', locale: 'en', since: 2020, until: 2025 });
const rows = f.records(['date', 'birthdate'], { n: 5 });
rows.forEach(r => console.log(r.date + '\t' + r.birthdate));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js year range differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js: inline range in field spec
// ---------------------------------------------------------------------------

#[test]
fn node_inline_range_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out = run_ok(&[
        "integer:100..200",
        "date:2020..2022",
        "--seed",
        "bind-range",
        "--locale",
        "en",
        "-n",
        "5",
    ]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-range', locale: 'en' });
const rows = f.records(['integer:100..200', 'date:2020..2022'], { n: 5 });
rows.forEach(r => console.log(r['integer'] + '\t' + r['date']));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js inline range differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js: ctx=loose matches CLI
// ---------------------------------------------------------------------------

#[test]
fn node_ctx_loose_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "bind-loose",
        "--locale",
        "en",
        "--ctx",
        "loose",
        "-n",
        "10",
    ]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-loose', locale: 'en' });
const rows = f.records(['name', 'email'], { n: 10, ctx: 'loose' });
rows.forEach(r => console.log(r.name + '\t' + r.email));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js ctx=loose differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js: corruption matches CLI
// ---------------------------------------------------------------------------

#[test]
fn node_corrupt_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "bind-cor",
        "--locale",
        "en",
        "--corrupt",
        "high",
        "-n",
        "10",
    ]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-cor', locale: 'en' });
const rows = f.records(['name', 'email'], { n: 10, corrupt: 'high' });
rows.forEach(r => console.log(r.name + '\t' + r.email));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js corrupt differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js: capitalize modifier matches Rust behavior
// ---------------------------------------------------------------------------

#[test]
fn node_capitalize_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out = run_ok(&[
        "name:capitalize",
        "--seed",
        "bind-cap",
        "--until",
        "2025",
        "--locale",
        "en",
        "-n",
        "10",
    ]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-cap', locale: 'en' });
for (let i = 0; i < 10; i++) console.log(f.field('name:capitalize'));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js capitalize differs from CLI");
}

// ---------------------------------------------------------------------------
// Python: capitalize modifier via field()
// ---------------------------------------------------------------------------

#[test]
fn python_capitalize_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: seedfaker python not built");
        return;
    }
    let cli_out = run_ok(&[
        "name:capitalize",
        "--seed",
        "bind-cap",
        "--until",
        "2025",
        "--locale",
        "en",
        "-n",
        "10",
    ]);
    let py_out = run_python(
        r#"
f = SeedFaker(seed='bind-cap', locale='en', until=2025)
for i in range(10):
    print(f.field('name:capitalize'))
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python capitalize differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js: transform in records()
// ---------------------------------------------------------------------------

#[test]
fn node_transform_in_records_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out = run_ok(&[
        "name:upper",
        "email:lower",
        "--seed",
        "bind-xform",
        "--until",
        "2025",
        "--locale",
        "en",
        "-n",
        "5",
    ]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-xform', locale: 'en' });
const rows = f.records(['name:upper', 'email:lower'], { n: 5 });
rows.forEach(r => console.log(r['name_upper'] + '\t' + r['email_lower']));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js transform in records differs from CLI");
}

// ---------------------------------------------------------------------------
// Python: transform in records()
// ---------------------------------------------------------------------------

#[test]
fn python_transform_in_records_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: seedfaker python not built");
        return;
    }
    let cli_out = run_ok(&[
        "name:upper",
        "email:lower",
        "--seed",
        "bind-xform",
        "--until",
        "2025",
        "--locale",
        "en",
        "-n",
        "5",
    ]);
    let py_out = run_python(
        r#"
f = SeedFaker(seed='bind-xform', locale='en', until=2025)
for r in f.records(['name:upper', 'email:lower'], n=5):
    print(r['name_upper'] + '\t' + r['email_lower'])
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python transform in records differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js: field() with n > 1 (batch single field)
// ---------------------------------------------------------------------------

#[test]
fn node_field_batch_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out =
        run_ok(&["email", "--seed", "bind-batch", "--until", "2025", "--locale", "en", "-n", "5"]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-batch', locale: 'en' });
for (let i = 0; i < 5; i++) console.log(f.field('email'));
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js field batch differs from CLI");
}

// ---------------------------------------------------------------------------
// CLI vs MCP: ctx=loose
// ---------------------------------------------------------------------------

#[test]
fn cli_vs_mcp_ctx_loose() {
    let cli_out = run_ok(&[
        "name", "email", "--seed", "xloose", "--until", "2025", "--locale", "en", "--ctx", "loose",
        "-n", "10",
    ]);
    let records = mcp_generate(
        r#"{"fields":["name","email"],"n":10,"seed":"xloose","until":2025,"locale":"en","ctx":"loose"}"#,
    );
    let mcp_lines = mcp_records_to_tsv(&records, &["name", "email"]);
    let cli_lines: Vec<&str> = cli_out.lines().collect();
    assert_eq!(cli_lines, mcp_lines, "CLI and MCP ctx=loose differs");
}

// ---------------------------------------------------------------------------
// CLI vs MCP: inline range
// ---------------------------------------------------------------------------

#[test]
fn cli_vs_mcp_inline_range() {
    let cli_out = run_ok(&[
        "integer:10..50",
        "date:2020..2022",
        "--seed",
        "xrange",
        "--until",
        "2025",
        "-n",
        "5",
    ]);
    let records = mcp_generate(
        r#"{"fields":["integer:10..50","date:2020..2022"],"n":5,"seed":"xrange","until":2025}"#,
    );
    // MCP display_name strips range: "integer", "date"
    let mcp_lines = mcp_records_to_tsv(&records, &["integer", "date"]);
    let cli_lines: Vec<&str> = cli_out.lines().collect();
    assert_eq!(cli_lines, mcp_lines, "CLI and MCP inline range differs");
}

// ---------------------------------------------------------------------------
// Python: record() singular matches CLI -n 1
// ---------------------------------------------------------------------------

#[test]
fn python_record_singular_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "bind-rec1",
        "--until",
        "2025",
        "--locale",
        "en",
        "-n",
        "1",
    ]);
    let py_out = run_python(
        r#"
f = SeedFaker(seed="bind-rec1", locale="en", until=2025)
r = f.record(["name", "email"])
print(f"{r['name']}\t{r['email']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python record() singular differs from CLI -n 1");
}

// ---------------------------------------------------------------------------
// Node.js: record() singular matches CLI -n 1
// ---------------------------------------------------------------------------

#[test]
fn node_record_singular_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "bind-rec1",
        "--until",
        "2025",
        "--locale",
        "en",
        "-n",
        "1",
    ]);
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-rec1', locale: 'en', until: 2025 });
const r = f.record(['name', 'email']);
console.log(r['name'] + '\t' + r['email']);
"#,
    );
    assert_eq!(cli_out.trim(), js_out.trim(), "Node.js record() singular differs from CLI -n 1");
}

// ---------------------------------------------------------------------------
// Expression cross-determinism: Python vs Node.js vs WASM
// CLI does not support expressions via positional args (config-only).
// ---------------------------------------------------------------------------

#[test]
fn python_expression_matches_node() {
    if !python_available() || !node_available() {
        eprintln!("SKIP: Python or Node bindings not built");
        return;
    }
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-expr", until=2025)
for r in f.records(["amount:plain:1..500", "integer:1..20", "amount:plain:1..500 * integer:1..20"], n=5):
    print(f"{r['amount']}\t{r['integer']}\t{r['amount_x_integer']}")
"#,
    );
    let js_out = run_node(
        r#"
const f = new SeedFaker({ seed: 'bind-expr', until: 2025 });
const rows = f.records(['amount:plain:1..500', 'integer:1..20', 'amount:plain:1..500 * integer:1..20'], { n: 5 });
rows.forEach(r => console.log(r['amount'] + '\t' + r['integer'] + '\t' + r['amount_x_integer']));
"#,
    );
    assert_eq!(py_out.trim(), js_out.trim(), "Python expression differs from Node.js");
}

#[test]
fn python_expression_matches_config() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    // Compare binding expression output with CLI config expression output
    let dir = common::tempfile("xdet-expr");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("expr.yaml");
    std::fs::write(
        &path,
        "columns:\n  amount: amount:plain:1..500\n  integer: integer:1..20\n  total: amount * integer\noptions:\n  seed: bind-expr\n  until: 2025\n  format: tsv\n  no_header: true\n",
    )
    .expect("write");
    let cli_out = run_ok(&["run", path.to_str().expect("p"), "-n", "5"]);
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-expr", until=2025)
for r in f.records(["amount:plain:1..500", "integer:1..20", "amount:plain:1..500 * integer:1..20"], n=5):
    print(f"{r['amount']}\t{r['integer']}\t{r['amount_x_integer']}")
"#,
    );
    assert_eq!(cli_out.trim(), py_out.trim(), "Python expression differs from CLI config");
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Python: validate() works
// ---------------------------------------------------------------------------

#[test]
fn python_validate_valid() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let out = run_python(
        r#"
from seedfaker import SeedFaker
SeedFaker.validate(["name", "email", "phone:e164"])
print("ok")
"#,
    );
    assert_eq!(out.trim(), "ok");
}

#[test]
fn python_validate_rejects_invalid() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let pip_path = project_root().join("packages/pip");
    let out = std::process::Command::new("python3")
        .arg("-c")
        .arg(
            r#"
from seedfaker import SeedFaker
try:
    SeedFaker.validate(["name:e164"])
    print("no error")
except Exception as e:
    print("error")
"#,
        )
        .env("PYTHONPATH", &pip_path)
        .output()
        .expect("python3");
    let stdout = String::from_utf8(out.stdout).expect("utf8");
    assert_eq!(stdout.trim(), "error", "validate should reject invalid modifier");
}

// ---------------------------------------------------------------------------
// Python: fields() returns list
// ---------------------------------------------------------------------------

#[test]
fn python_fields_returns_list() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let out = run_python(
        r#"
from seedfaker import SeedFaker
fields = SeedFaker.fields()
assert len(fields) > 10, f"expected many fields, got {len(fields)}"
assert "name" in fields
assert "email" in fields
print("ok")
"#,
    );
    assert_eq!(out.trim(), "ok");
}

// ---------------------------------------------------------------------------
// Node.js: validate() works
// ---------------------------------------------------------------------------

#[test]
fn node_validate_valid() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let out = run_node(
        r#"
const f = new SeedFaker({ seed: 'v' });
f.validate(['name', 'email', 'phone:e164']);
console.log('ok');
"#,
    );
    assert_eq!(out.trim(), "ok");
}

#[test]
fn node_validate_rejects_invalid() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let npm_path = project_root().join("packages/npm");
    let out = std::process::Command::new("node")
        .arg("-e")
        .arg(format!(
            r#"const {{ SeedFaker }} = require('{}');
const f = new SeedFaker({{ seed: 'v' }});
try {{ f.validate(['name:e164']); console.log('no error'); }} catch(e) {{ console.log('error'); }}"#,
            npm_path.display()
        ))
        .output()
        .expect("node");
    let stdout = String::from_utf8(out.stdout).expect("utf8");
    assert_eq!(stdout.trim(), "error", "validate should reject invalid modifier");
}

// ---------------------------------------------------------------------------
// Node.js: fields() returns array
// ---------------------------------------------------------------------------

#[test]
fn node_fields_returns_array() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let out = run_node(
        r#"
const fields = SeedFaker.fields();
if (!Array.isArray(fields) || fields.length < 10) throw new Error('bad');
if (!fields.includes('name') || !fields.includes('email')) throw new Error('missing');
console.log('ok');
"#,
    );
    assert_eq!(out.trim(), "ok");
}

// ---------------------------------------------------------------------------
// Python: fingerprint matches CLI
// ---------------------------------------------------------------------------

#[test]
fn python_fingerprint_matches_cli() {
    if !python_available() {
        eprintln!("SKIP: _seedfaker.so not built");
        return;
    }
    let cli_fp = run_ok(&["--fingerprint"]);
    let py_fp = run_python(
        r#"
from seedfaker import SeedFaker
print(SeedFaker.fingerprint())
"#,
    );
    assert_eq!(cli_fp.trim(), py_fp.trim(), "Python fingerprint differs from CLI");
}

// ---------------------------------------------------------------------------
// Node.js: fingerprint matches CLI
// ---------------------------------------------------------------------------

#[test]
fn node_fingerprint_matches_cli() {
    if !node_available() {
        eprintln!("SKIP: seedfaker_napi.node not built");
        return;
    }
    let cli_fp = run_ok(&["--fingerprint"]);
    let js_fp = run_node("console.log(SeedFaker.fingerprint());");
    assert_eq!(cli_fp.trim(), js_fp.trim(), "Node.js fingerprint differs from CLI");
}

// ---------------------------------------------------------------------------
// WASM: cross-determinism (Node.js loads WASM, compares with CLI)
// ---------------------------------------------------------------------------

fn wasm_available() -> bool {
    std::path::Path::new("packages/wasm/web/seedfaker_wasm.js").exists()
        && std::path::Path::new("packages/wasm/web/seedfaker_wasm_bg.wasm").exists()
}

fn run_wasm(script: &str) -> String {
    let full = format!(
        r#"
async function main() {{
  const {{ readFileSync }} = require('fs');
  const path = require('path');
  const mod = await import(path.resolve('packages/wasm/web/seedfaker_wasm.js'));
  await mod.default({{ module_or_path: readFileSync(path.resolve('packages/wasm/web/seedfaker_wasm_bg.wasm')) }});
  const SeedFaker = mod.SeedFaker;
  {script}
}}
main().catch(e => {{ console.error(e); process.exit(1); }});
"#
    );
    let out = std::process::Command::new("node").arg("-e").arg(&full).output().expect("node");
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        eprintln!("WASM script failed: {stderr}");
    }
    String::from_utf8(out.stdout).expect("utf8")
}

#[test]
fn wasm_basic_matches_cli() {
    if !wasm_available() {
        eprintln!("SKIP: WASM not built");
        return;
    }
    let cli_out =
        run_ok(&["name", "email", "phone", "--seed", "wasm-basic", "--until", "2025", "-n", "5"]);
    let wasm_out = run_wasm(
        r#"
const f = new SeedFaker({ seed: 'wasm-basic', locale: null, tz: null, until: 2025 });
const rows = f.records(['name', 'email', 'phone'], 5, null, null);
for (const r of rows) console.log(r.name + '\t' + r.email + '\t' + r.phone);
"#,
    );
    assert_eq!(cli_out.trim(), wasm_out.trim(), "WASM basic differs from CLI");
}

#[test]
fn wasm_fingerprint_matches_cli() {
    if !wasm_available() {
        eprintln!("SKIP: WASM not built");
        return;
    }
    let cli_fp = run_ok(&["--fingerprint"]);
    let wasm_fp = run_wasm("console.log(SeedFaker.fingerprint());");
    assert_eq!(cli_fp.trim(), wasm_fp.trim(), "WASM fingerprint differs from CLI");
}

#[test]
fn wasm_ctx_strict_matches_cli() {
    if !wasm_available() {
        eprintln!("SKIP: WASM not built");
        return;
    }
    let cli_out = run_ok(&[
        "name", "email", "--seed", "wasm-ctx", "--until", "2025", "--locale", "en", "--ctx",
        "strict", "-n", "5",
    ]);
    let wasm_out = run_wasm(
        r#"
const f = new SeedFaker({ seed: 'wasm-ctx', locale: 'en', until: 2025 });
const rows = f.records(['name', 'email'], 5, 'strict', null);
for (const r of rows) console.log(r.name + '\t' + r.email);
"#,
    );
    assert_eq!(cli_out.trim(), wasm_out.trim(), "WASM ctx strict differs from CLI");
}

#[test]
fn wasm_corrupt_matches_cli() {
    if !wasm_available() {
        eprintln!("SKIP: WASM not built");
        return;
    }
    let cli_out = run_ok(&[
        "name",
        "email",
        "--seed",
        "wasm-cor",
        "--until",
        "2025",
        "--locale",
        "en",
        "--corrupt",
        "high",
        "-n",
        "10",
    ]);
    let wasm_out = run_wasm(
        r#"
const f = new SeedFaker({ seed: 'wasm-cor', locale: 'en', until: 2025 });
const rows = f.records(['name', 'email'], 10, null, 'high');
for (const r of rows) console.log(r.name + '\t' + r.email);
"#,
    );
    assert_eq!(cli_out.trim(), wasm_out.trim(), "WASM corrupt differs from CLI");
}

// ---------------------------------------------------------------------------
// WASM: validate works
// ---------------------------------------------------------------------------

#[test]
fn wasm_validate_valid() {
    if !wasm_available() {
        eprintln!("SKIP: WASM not built");
        return;
    }
    let out = run_wasm(
        r#"
SeedFaker.validate(['name', 'email', 'phone:e164'], null, null);
console.log('ok');
"#,
    );
    assert_eq!(out.trim(), "ok");
}

#[test]
fn wasm_validate_rejects_invalid() {
    if !wasm_available() {
        eprintln!("SKIP: WASM not built");
        return;
    }
    let out = run_wasm(
        r#"
try { SeedFaker.validate(['name:e164'], null, null); console.log('no error'); } catch(e) { console.log('error'); }
"#,
    );
    assert_eq!(out.trim(), "error", "WASM validate should reject invalid modifier");
}

// ---------------------------------------------------------------------------
// WASM: fields() returns array
// ---------------------------------------------------------------------------

#[test]
fn wasm_fields_returns_array() {
    if !wasm_available() {
        eprintln!("SKIP: WASM not built");
        return;
    }
    let out = run_wasm(
        r#"
const fields = SeedFaker.fields();
if (!Array.isArray(fields) || fields.length < 10) throw new Error('bad fields count: ' + fields.length);
if (!fields.includes('name') || !fields.includes('email')) throw new Error('missing name/email');
console.log('ok');
"#,
    );
    assert_eq!(out.trim(), "ok");
}

// ---------------------------------------------------------------------------
// WASM: expression cross-determinism (vs Python)
// ---------------------------------------------------------------------------

#[test]
fn wasm_expression_matches_python() {
    if !wasm_available() || !python_available() {
        eprintln!("SKIP: WASM or Python not built");
        return;
    }
    let py_out = run_python(
        r#"
from seedfaker import SeedFaker
f = SeedFaker(seed="bind-expr", until=2025)
for r in f.records(["amount:plain:1..500", "integer:1..20", "amount:plain:1..500 * integer:1..20"], n=5):
    print(f"{r['amount']}\t{r['integer']}\t{r['amount_x_integer']}")
"#,
    );
    let wasm_out = run_wasm(
        r#"
const f = new SeedFaker({ seed: 'bind-expr', until: 2025 });
const rows = f.records(['amount:plain:1..500', 'integer:1..20', 'amount:plain:1..500 * integer:1..20'], 5, null, null);
for (const r of rows) console.log(r['amount'] + '\t' + r['integer'] + '\t' + r['amount_x_integer']);
"#,
    );
    assert_eq!(py_out.trim(), wasm_out.trim(), "WASM expression differs from Python");
}
