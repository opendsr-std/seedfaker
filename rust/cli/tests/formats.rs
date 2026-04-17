/// Output format tests: CSV, TSV, JSONL, SQL, templates.
/// Validates structural correctness, headers, quoting, key naming,
/// and template features (vars, {{new}}, {{serial}}, conditionals).
mod common;
use common::{run_fail, run_ok};

// ---------------------------------------------------------------------------
// Inline snapshots: exact byte-for-byte output with --seed
// ---------------------------------------------------------------------------

#[test]
fn snapshot_csv() {
    let out = run_ok(&[
        "name", "email", "--format", "csv", "-n", "3", "--seed", "snap2", "--until", "2025",
    ]);
    assert_eq!(
        out,
        "name,email\n\
         Villads Gade,svetoslavmarkov@dir.bg\n\
         Salom\u{e9} Paredes Serrano,genevievekumar@cgi.com\n\
         Gustaw Raczynski,eliftunc@gmail.com\n"
    );
}

#[test]
fn snapshot_tsv() {
    let out = run_ok(&[
        "name", "email", "--format", "tsv", "-n", "3", "--seed", "snap2", "--until", "2025",
    ]);
    assert_eq!(
        out,
        "name\temail\n\
         Villads Gade\tsvetoslavmarkov@dir.bg\n\
         Salom\u{e9} Paredes Serrano\tgenevievekumar@cgi.com\n\
         Gustaw Raczynski\teliftunc@gmail.com\n"
    );
}

#[test]
fn snapshot_jsonl() {
    let out = run_ok(&[
        "name", "email", "--format", "jsonl", "-n", "3", "--seed", "snap2", "--until", "2025",
    ]);
    assert_eq!(
        out,
        "{\"name\":\"Villads Gade\",\"email\":\"svetoslavmarkov@dir.bg\"}\n\
         {\"name\":\"Salom\u{e9} Paredes Serrano\",\"email\":\"genevievekumar@cgi.com\"}\n\
         {\"name\":\"Gustaw Raczynski\",\"email\":\"eliftunc@gmail.com\"}\n"
    );
}

#[test]
fn snapshot_sql() {
    let out = run_ok(&[
        "name",
        "email",
        "--format",
        "sql=users",
        "-n",
        "3",
        "--seed",
        "snap2",
        "--until",
        "2025",
    ]);
    assert_eq!(
        out,
        "INSERT INTO users (name, email) VALUES ('Villads Gade', 'svetoslavmarkov@dir.bg');\n\
         INSERT INTO users (name, email) VALUES ('Salom\u{e9} Paredes Serrano', 'genevievekumar@cgi.com');\n\
         INSERT INTO users (name, email) VALUES ('Gustaw Raczynski', 'eliftunc@gmail.com');\n"
    );
}

#[test]
fn snapshot_template() {
    let out =
        run_ok(&["-t", "{{name}} <{{email}}>", "-n", "3", "--seed", "snap2", "--until", "2025"]);
    assert_eq!(
        out,
        "Villads Gade <svetoslavmarkov@dir.bg>\n\
         Salom\u{e9} Paredes Serrano <genevievekumar@cgi.com>\n\
         Gustaw Raczynski <eliftunc@gmail.com>\n"
    );
}

// ---------------------------------------------------------------------------
// CSV structural validation
// ---------------------------------------------------------------------------

#[test]
fn csv_values_with_commas_are_quoted() {
    let out = run_ok(&[
        "address",
        "--format",
        "csv",
        "--no-header",
        "-n",
        "20",
        "--seed",
        "comma",
        "--until",
        "2025",
    ]);
    for line in out.lines() {
        if line.contains(',') {
            assert!(
                line.starts_with('"') || line.contains("\",\""),
                "CSV values with commas must be quoted: {line}"
            );
        }
    }
}

#[test]
fn csv_no_header() {
    let with_header =
        run_ok(&["name", "email", "--format", "csv", "-n", "3", "--seed", "nh", "--until", "2025"]);
    let no_header = run_ok(&[
        "name",
        "email",
        "--format",
        "csv",
        "--no-header",
        "-n",
        "3",
        "--seed",
        "nh",
        "--until",
        "2025",
    ]);
    let header_line = with_header.lines().next().unwrap();
    assert!(!no_header.starts_with(header_line), "--no-header should not start with header");
    assert_eq!(no_header.lines().count(), 3, "3 data rows without header");
}

#[test]
fn csv_header_uses_underscores_for_modifiers() {
    let out =
        run_ok(&["phone:e164", "--format", "csv", "-n", "1", "--seed", "hdr", "--until", "2025"]);
    let header = out.lines().next().unwrap();
    assert_eq!(header, "phone_e164");
}

// ---------------------------------------------------------------------------
// JSONL structural validation
// ---------------------------------------------------------------------------

#[test]
fn jsonl_all_lines_valid_json_with_required_keys() {
    let out = run_ok(&[
        "name", "email", "phone", "--format", "jsonl", "-n", "10", "--seed", "jl-val", "--until",
        "2025",
    ]);
    for line in out.lines() {
        let v: serde_json::Value = serde_json::from_str(line).expect("invalid JSON");
        let obj = v.as_object().expect("should be object");
        for key in &["name", "email", "phone"] {
            assert!(obj.contains_key(*key), "missing '{key}': {line}");
            let val = obj[*key].as_str().expect("should be string");
            assert!(!val.is_empty(), "'{key}' should be non-empty");
        }
    }
}

#[test]
fn jsonl_key_names_use_underscores() {
    let out = run_ok(&[
        "phone:e164",
        "credit-card:space",
        "--format",
        "jsonl",
        "-n",
        "1",
        "--seed",
        "jk",
    ]);
    let v: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
    assert!(v.get("phone_e164").is_some(), "key should be phone_e164");
    assert!(v.get("credit_card_space").is_some(), "key should be credit_card_space");
}

// ---------------------------------------------------------------------------
// SQL structural validation
// ---------------------------------------------------------------------------

#[test]
fn sql_structure() {
    let out = run_ok(&[
        "name",
        "email",
        "--format",
        "sql=users",
        "-n",
        "3",
        "--seed",
        "sql-v",
        "--until",
        "2025",
    ]);
    for line in out.lines() {
        assert!(line.starts_with("INSERT INTO users (name, email) VALUES ("));
        assert!(line.ends_with(");"));
    }
}

// ---------------------------------------------------------------------------
// TSV
// ---------------------------------------------------------------------------

#[test]
fn tsv_header_and_data() {
    let out = run_ok(&[
        "name", "email", "--format", "tsv", "-n", "1", "--seed", "hdr", "--until", "2025",
    ]);
    let header = out.lines().next().unwrap();
    assert_eq!(header, "name\temail");
    let data = out.lines().nth(1).unwrap();
    assert_eq!(data.split('\t').count(), 2);
}

// ---------------------------------------------------------------------------
// Custom delimiters
// ---------------------------------------------------------------------------

#[test]
fn delim_pipe() {
    let out = run_ok(&["name", "email", "-n", "1", "--seed", "d1", "--until", "2025", "-d", "|"]);
    assert!(out.contains('|'), "should use pipe");
    assert!(!out.contains('\t'), "should not contain tab");
}

#[test]
fn delim_csv_semicolon() {
    let out = run_ok(&[
        "name", "email", "--format", "csv", "-n", "2", "--seed", "d1", "--until", "2025", "-d", ";",
    ]);
    for line in out.lines() {
        assert!(line.contains(';'), "should use semicolon: {line}");
    }
}

// ---------------------------------------------------------------------------
// Template features
// ---------------------------------------------------------------------------

#[test]
fn template_var_reused() {
    let out = run_ok(&[
        "name",
        "-t",
        "{{name}} and {{name}}",
        "-n",
        "3",
        "--seed",
        "reu",
        "--until",
        "2025",
    ]);
    for line in out.lines() {
        let parts: Vec<&str> = line.splitn(2, " and ").collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], parts[1], "same var should produce same value: {line}");
    }
}

#[test]
fn template_same_field_in_columns_is_stable() {
    // "name" extracted from template → added to columns → both {{name}} = same value
    let out = run_ok(&["-t", "{{name}}|{{name}}", "-n", "3", "--seed", "nn", "--until", "2025"]);
    for line in out.lines() {
        let parts: Vec<&str> = line.splitn(2, '|').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], parts[1], "same column should produce same value: {line}");
    }
}

#[test]
fn template_column_vs_inline_field() {
    // "name" is a column → stable; "email" is not → inline generation
    let out = run_ok(&[
        "name",
        "-t",
        "{{name}} extra={{email}}",
        "-n",
        "3",
        "--seed",
        "vn",
        "--until",
        "2025",
    ]);
    for line in out.lines() {
        let parts: Vec<&str> = line.splitn(2, " extra=").collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[1].is_empty(), "inline field should produce a value: {line}");
    }
}

#[test]
fn template_serial() {
    let out = run_ok(&["-t", "row-{{serial}}", "-n", "4", "--seed", "ser", "--until", "2025"]);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines, vec!["row-0", "row-1", "row-2", "row-3"]);
}

#[test]
fn template_unknown_placeholder_rejected() {
    run_fail(&["-t", "{{nonexistent}}", "-n", "1", "--seed", "unk", "--until", "2025"]);
}

#[test]
fn template_modifier_applied() {
    let out = run_ok(&["-t", "{{phone:e164}}", "-n", "5", "--seed", "tm", "--until", "2025"]);
    for line in out.lines() {
        assert!(line.starts_with('+'), "e164 should start with +: {line}");
    }
}

#[test]
fn template_transform_applied() {
    let out = run_ok(&["-t", "{{name:upper}}", "-n", "3", "--seed", "ttr", "--until", "2025"]);
    for line in out.lines() {
        assert_eq!(line, &line.to_uppercase(), "upper template: {line}");
    }
}

// ---------------------------------------------------------------------------
// Multiple formats rejected
// ---------------------------------------------------------------------------

#[test]
fn multiple_formats_rejected() {
    run_fail(&["name", "-t", "{{name}}", "--format", "csv", "-n", "1"]);
}

#[test]
fn template_repeat_produces_multiple_values() {
    let dir = common::tempfile("tpl-repeat");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("rep.yaml");
    std::fs::write(
        &path,
        "columns:\n  team: company-name\ntemplate: |\n  Team: {{team}}\n  {{#repeat 3}}\n  - {{name}}\n  {{/repeat}}\n",
    )
    .expect("write");
    let out =
        run_ok(&["run", path.to_str().expect("p"), "-n", "2", "--seed", "rep", "--until", "2025"]);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "2 records = 2 lines");
    // Each line: "Team: X- Name1- Name2- Name3" (tag-only lines collapse)
    for line in &lines {
        assert!(line.starts_with("Team: "), "each line starts with Team:");
        let dash_count = line.matches("- ").count();
        assert_eq!(dash_count, 3, "each record has 3 repeat items, got {dash_count} in: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn template_if_branches() {
    let dir = common::tempfile("tpl-if");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("if.yaml");
    std::fs::write(
        &path,
        "columns:\n  level: enum:A=1,B=1\ntemplate: |\n  {{#if level == \"A\"}}\n  alpha\n  {{else}}\n  beta\n  {{/if}}\n",
    )
    .expect("write");
    let out = run_ok(&[
        "run",
        path.to_str().expect("p"),
        "-n",
        "20",
        "--seed",
        "iftest",
        "--until",
        "2025",
    ]);
    let has_alpha = out.contains("alpha");
    let has_beta = out.contains("beta");
    assert!(has_alpha, "50/50 enum on 20 records should produce at least one A");
    assert!(has_beta, "50/50 enum on 20 records should produce at least one B");
    let _ = std::fs::remove_dir_all(&dir);
}
