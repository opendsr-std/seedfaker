/// Multi-table FK tests: anchor coherence, deref coherence,
/// cross-table relationships, determinism, --all --output-dir.
mod common;
use common::{run_fail, run_ok, tempfile};
use std::collections::HashMap;

const MULTI_CONFIG: &str = r#"
options:
  seed: "mt-test"
  locale: [en]

users:
  columns:
    id: uuid
    name: first-name
    email: email
  options:
    count: 50

orders:
  columns:
    order_id: serial
    customer_id: users.id
    customer_name: customer_id->name
    customer_email: customer_id->email
    total: amount:usd:1..5000
  options:
    count: 200

reviews:
  columns:
    review_id: serial
    author_id: users.id
    author_name: author_id->name
    stars: integer:1..5
  options:
    count: 100
"#;

fn write_config(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let dir = tempfile(name);
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("multi.yaml");
    std::fs::write(&path, MULTI_CONFIG).expect("write");
    (dir, path)
}

fn parse_jsonl(s: &str) -> Vec<serde_json::Value> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).expect("invalid JSONL"))
        .collect()
}

/// Build a lookup: uuid → (name, email) from users JSONL output.
fn build_user_map(users_jsonl: &str) -> HashMap<String, (String, String)> {
    let rows = parse_jsonl(users_jsonl);
    let mut map = HashMap::new();
    for row in &rows {
        let id = row["id"].as_str().expect("id").to_string();
        let name = row["name"].as_str().expect("name").to_string();
        let email = row["email"].as_str().expect("email").to_string();
        map.insert(id, (name, email));
    }
    map
}

// ---------------------------------------------------------------------------
// FK anchor + deref coherence: orders reference users
// ---------------------------------------------------------------------------

#[test]
fn fk_anchor_values_exist_in_parent() {
    let (dir, path) = write_config("fk-anchor");
    let p = path.to_str().expect("p");

    let users_out = run_ok(&["run", p, "--table", "users", "--format", "jsonl"]);
    let orders_out = run_ok(&["run", p, "--table", "orders", "--format", "jsonl"]);

    let user_map = build_user_map(&users_out);
    let orders = parse_jsonl(&orders_out);

    for (i, row) in orders.iter().enumerate() {
        let cid = row["customer_id"].as_str().unwrap_or("");
        assert!(
            user_map.contains_key(cid),
            "orders row {i}: customer_id '{cid}' not found in users table"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fk_deref_matches_parent_row() {
    let (dir, path) = write_config("fk-deref");
    let p = path.to_str().expect("p");

    let users_out = run_ok(&["run", p, "--table", "users", "--format", "jsonl"]);
    let orders_out = run_ok(&["run", p, "--table", "orders", "--format", "jsonl"]);

    let user_map = build_user_map(&users_out);
    let orders = parse_jsonl(&orders_out);

    for (i, row) in orders.iter().enumerate() {
        let cid = row["customer_id"].as_str().unwrap_or("");
        let cname = row["customer_name"].as_str().unwrap_or("");
        let cemail = row["customer_email"].as_str().unwrap_or("");

        let (expected_name, expected_email) = user_map
            .get(cid)
            .unwrap_or_else(|| panic!("orders row {i}: customer_id '{cid}' not in users"));

        assert_eq!(
            cname, expected_name,
            "orders row {i}: deref name '{cname}' != users name '{expected_name}' for cid={cid}"
        );
        assert_eq!(
            cemail, expected_email,
            "orders row {i}: deref email '{cemail}' != users email '{expected_email}' for cid={cid}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Multiple tables reference the same parent
// ---------------------------------------------------------------------------

#[test]
fn multiple_tables_reference_same_parent() {
    let (dir, path) = write_config("multi-ref");
    let p = path.to_str().expect("p");

    let users_out = run_ok(&["run", p, "--table", "users", "--format", "jsonl"]);
    let reviews_out = run_ok(&["run", p, "--table", "reviews", "--format", "jsonl"]);

    let user_map = build_user_map(&users_out);
    let reviews = parse_jsonl(&reviews_out);

    for (i, row) in reviews.iter().enumerate() {
        let aid = row["author_id"].as_str().unwrap_or("");
        let aname = row["author_name"].as_str().unwrap_or("");

        let (expected_name, _) = user_map
            .get(aid)
            .unwrap_or_else(|| panic!("reviews row {i}: author_id '{aid}' not in users"));

        assert_eq!(
            aname, expected_name,
            "reviews row {i}: deref name '{aname}' != users name '{expected_name}' for aid={aid}"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Determinism: same seed → identical output
// ---------------------------------------------------------------------------

#[test]
fn multi_table_deterministic() {
    let (dir, path) = write_config("det");
    let p = path.to_str().expect("p");

    let a = run_ok(&["run", p, "--table", "orders", "--format", "jsonl"]);
    let b = run_ok(&["run", p, "--table", "orders", "--format", "jsonl"]);
    assert_eq!(a, b, "multi-table output must be deterministic");

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// --all --output-dir
// ---------------------------------------------------------------------------

#[test]
fn all_tables_to_files() {
    let (dir, path) = write_config("all");
    let out_dir = dir.join("output");
    let p = path.to_str().expect("p");
    let od = out_dir.to_str().expect("od");

    run_ok(&["run", p, "--all", "--output-dir", od, "--format", "jsonl"]);

    assert!(out_dir.join("users.jsonl").exists(), "users.jsonl must exist");
    assert!(out_dir.join("orders.jsonl").exists(), "orders.jsonl must exist");
    assert!(out_dir.join("reviews.jsonl").exists(), "reviews.jsonl must exist");

    let users_file = std::fs::read_to_string(out_dir.join("users.jsonl")).expect("read");
    let orders_file = std::fs::read_to_string(out_dir.join("orders.jsonl")).expect("read");

    // Verify file FK coherence
    let user_map = build_user_map(&users_file);
    let orders = parse_jsonl(&orders_file);
    for (i, row) in orders.iter().enumerate() {
        let cid = row["customer_id"].as_str().unwrap_or("");
        assert!(
            user_map.contains_key(cid),
            "orders file row {i}: customer_id '{cid}' not in users file"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Error cases
// ---------------------------------------------------------------------------

#[test]
fn multi_table_requires_table_or_all() {
    let (dir, path) = write_config("err-none");
    run_fail(&["run", path.to_str().expect("p")]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn multi_table_all_requires_output_dir() {
    let (dir, path) = write_config("err-alldir");
    run_fail(&["run", path.to_str().expect("p"), "--all"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn multi_table_unknown_table() {
    let (dir, path) = write_config("err-unk");
    run_fail(&["run", path.to_str().expect("p"), "--table", "nonexistent"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn multi_table_table_and_all_exclusive() {
    let (dir, path) = write_config("err-both");
    run_fail(&[
        "run",
        path.to_str().expect("p"),
        "--table",
        "users",
        "--all",
        "--output-dir",
        "/tmp",
    ]);
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Full 6-point verification: users + products + orders
// ---------------------------------------------------------------------------

const SHOP_CONFIG: &str = r#"
options:
  seed: "shop-verify"
  locale: [en]

users:
  columns:
    id: uuid
    first_name: first-name
    email: email
  options:
    count: 20

products:
  columns:
    id: serial
    name: company-name
    price: amount:usd:5..2000
  options:
    count: 10

orders:
  columns:
    order_id: serial
    customer_id: users.id
    customer_name: customer_id->first_name
    customer_email: customer_id->email
    product_id: products.id
    product_name: product_id->name
    product_price: product_id->price
  options:
    count: 100
"#;

#[test]
fn full_fk_verification() {
    let dir = tempfile("full-verify");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("shop.yaml");
    std::fs::write(&path, SHOP_CONFIG).expect("write");
    let p = path.to_str().expect("p");

    let users_out = run_ok(&["run", p, "--table", "users", "--format", "jsonl"]);
    let products_out = run_ok(&["run", p, "--table", "products", "--format", "jsonl"]);
    let orders_out = run_ok(&["run", p, "--table", "orders", "--format", "jsonl"]);

    let users = parse_jsonl(&users_out);
    let products = parse_jsonl(&products_out);
    let orders = parse_jsonl(&orders_out);

    // Build lookups
    let user_by_id: HashMap<String, &serde_json::Value> =
        users.iter().map(|u| (u["id"].as_str().unwrap().to_string(), u)).collect();
    let product_by_id: HashMap<String, &serde_json::Value> =
        products.iter().map(|p| (p["id"].as_str().unwrap().to_string(), p)).collect();

    assert_eq!(users.len(), 20);
    assert_eq!(products.len(), 10);
    assert_eq!(orders.len(), 100);

    // Track customer_id → (name, email) for internal consistency check
    let mut cid_variants: HashMap<String, (String, String)> = HashMap::new();

    for (i, o) in orders.iter().enumerate() {
        let cid = o["customer_id"].as_str().unwrap_or("");
        let pid = o["product_id"].as_str().unwrap_or("");
        let pid_num: i64 = pid.parse().unwrap_or(-1);

        // 1) FK existence: customer_id in users
        assert!(user_by_id.contains_key(cid), "order {i}: customer_id '{cid}' not in users");

        // 1) FK existence: product_id in products
        assert!(
            product_by_id.contains_key(pid),
            "order {i}: product_id '{pid}' not in products (range 0..9)"
        );

        // 5) ID range check
        assert!(
            pid_num >= 0 && pid_num < 10,
            "order {i}: product_id {pid_num} out of range [0, 9]"
        );

        // 2) Denormalized customer fields match parent row
        if let Some(user) = user_by_id.get(cid) {
            let expected_name = user["first_name"].as_str().unwrap();
            let expected_email = user["email"].as_str().unwrap();
            let actual_name = o["customer_name"].as_str().unwrap();
            let actual_email = o["customer_email"].as_str().unwrap();

            assert_eq!(
                actual_name, expected_name,
                "order {i}: customer_name '{actual_name}' != users.first_name '{expected_name}' for cid={cid}"
            );
            assert_eq!(
                actual_email, expected_email,
                "order {i}: customer_email '{actual_email}' != users.email '{expected_email}' for cid={cid}"
            );
        }

        // 3) Denormalized product fields match parent row
        if let Some(product) = product_by_id.get(pid) {
            let expected_name = product["name"].as_str().unwrap();
            let expected_price = product["price"].as_str().unwrap();
            let actual_name = o["product_name"].as_str().unwrap();
            let actual_price = o["product_price"].as_str().unwrap();

            assert_eq!(
                actual_name, expected_name,
                "order {i}: product_name '{actual_name}' != products.name '{expected_name}' for pid={pid}"
            );
            assert_eq!(
                actual_price, expected_price,
                "order {i}: product_price '{actual_price}' != products.price '{expected_price}' for pid={pid}"
            );
        }

        // 4) Internal consistency: same customer_id → same (name, email)
        let name = o["customer_name"].as_str().unwrap().to_string();
        let email = o["customer_email"].as_str().unwrap().to_string();
        if let Some(prev) = cid_variants.get(cid) {
            assert_eq!(
                prev.0, name,
                "order {i}: customer_id '{cid}' maps to different names: '{}' vs '{name}'",
                prev.0
            );
            assert_eq!(
                prev.1, email,
                "order {i}: customer_id '{cid}' maps to different emails: '{}' vs '{email}'",
                prev.1
            );
        } else {
            cid_variants.insert(cid.to_string(), (name, email));
        }
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// 6) Same data from --all --output-dir matches --table stdout
#[test]
fn file_output_matches_stdout() {
    let dir = tempfile("file-match");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("shop.yaml");
    std::fs::write(&path, SHOP_CONFIG).expect("write");
    let p = path.to_str().expect("p");
    let out_dir = dir.join("output");
    let od = out_dir.to_str().expect("od");

    run_ok(&["run", p, "--all", "--output-dir", od, "--format", "jsonl"]);

    for table in &["users", "products", "orders"] {
        let file_content =
            std::fs::read_to_string(out_dir.join(format!("{table}.jsonl"))).expect("read file");
        let stdout_content = run_ok(&["run", p, "--table", table, "--format", "jsonl"]);
        assert_eq!(
            file_content, stdout_content,
            "table '{table}': file output differs from stdout output"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Anchor and deref of the same field produce identical values
// ---------------------------------------------------------------------------

#[test]
fn anchor_and_deref_same_field_match() {
    let dir = tempfile("same-field");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("same.yaml");
    std::fs::write(
        &path,
        r#"
options:
  seed: "same-field"
  locale: [en]

users:
  columns:
    id: uuid
    first_name: first-name
    email: email
  options:
    count: 30

orders:
  columns:
    customer_id: users.id
    customer_first_name: customer_id->first_name
    customer_email: customer_id->email
  options:
    count: 50
"#,
    )
    .expect("write");
    let p = path.to_str().expect("p");

    let users_out = run_ok(&["run", p, "--table", "users", "--format", "jsonl"]);
    let orders_out = run_ok(&["run", p, "--table", "orders", "--format", "jsonl"]);

    let users = parse_jsonl(&users_out);
    let orders = parse_jsonl(&orders_out);

    let user_by_id: HashMap<String, &serde_json::Value> =
        users.iter().map(|u| (u["id"].as_str().unwrap().to_string(), u)).collect();

    for (i, o) in orders.iter().enumerate() {
        let cid = o["customer_id"].as_str().unwrap();

        // FK must exist
        let user = user_by_id
            .get(cid)
            .unwrap_or_else(|| panic!("order {i}: customer_id '{cid}' not in users"));

        // Deref first_name must match parent's first_name
        assert_eq!(
            o["customer_first_name"].as_str().unwrap(),
            user["first_name"].as_str().unwrap(),
            "order {i}: deref first_name mismatch"
        );

        // Deref email must match parent's email
        assert_eq!(
            o["customer_email"].as_str().unwrap(),
            user["email"].as_str().unwrap(),
            "order {i}: deref email mismatch"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Self-referencing FK: table references itself (partial ctx:strict)
// ---------------------------------------------------------------------------

#[test]
fn self_referencing_fk() {
    let dir = tempfile("self-ref");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("self.yaml");
    std::fs::write(
        &path,
        r#"
options:
  seed: "self-ref"
  locale: [en]

employees:
  columns:
    id: serial
    name: first-name
    email: email
    manager_id: employees.id
    manager_name: manager_id->name
  options:
    count: 20
"#,
    )
    .expect("write");
    let p = path.to_str().expect("p");

    let out = run_ok(&["run", p, "--table", "employees", "--format", "jsonl"]);
    let rows = parse_jsonl(&out);
    assert_eq!(rows.len(), 20);

    // Build employee lookup
    let by_id: HashMap<String, &serde_json::Value> =
        rows.iter().map(|r| (r["id"].as_str().unwrap().to_string(), r)).collect();

    for (i, row) in rows.iter().enumerate() {
        let mid = row["manager_id"].as_str().unwrap();
        let mname = row["manager_name"].as_str().unwrap();

        // manager_id must exist in the same table
        let manager = by_id
            .get(mid)
            .unwrap_or_else(|| panic!("employee {i}: manager_id '{mid}' not in employees"));

        // manager_name must match the referenced employee's name
        assert_eq!(
            mname,
            manager["name"].as_str().unwrap(),
            "employee {i}: manager_name '{mname}' != employees[{mid}].name"
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Zipf: popular parents get disproportionately more references
// ---------------------------------------------------------------------------

#[test]
fn fk_zipf_distribution() {
    let dir = tempfile("fk-zipf");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("zipf.yaml");
    std::fs::write(
        &path,
        r#"
options:
  seed: "zipf-test"

parents:
  columns:
    id: serial
  options:
    count: 100

children:
  columns:
    parent_id: parents.id:zipf=1.5
  options:
    count: 1000
"#,
    )
    .expect("write");
    let p = path.to_str().expect("p");

    let output = run_ok(&["run", p, "--table", "children", "--no-header"]);
    let mut counts: HashMap<String, usize> = HashMap::new();
    for line in output.lines().filter(|l| !l.is_empty()) {
        *counts.entry(line.trim().to_string()).or_default() += 1;
    }

    let max_count = counts.values().max().copied().unwrap_or(0);
    assert!(
        max_count > 50,
        "zipf=1.5: top parent should have >50 refs out of 1000, got {max_count}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
