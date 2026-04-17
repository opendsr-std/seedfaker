use seedfaker_core::field::{
    all_names, field_capabilities, field_modifiers, lookup, GROUPS, REGISTRY,
};
use seedfaker_core::{field, opts, pipeline};

const SAMPLE_WIDTH: usize = 12;
const NAME_GAP: usize = 6;
const SAMPLE_GAP: usize = 4;
const SAMPLE_SEED: &str = "sample";
const SAMPLE_UNTIL: &str = "2025";
const SAMPLE_LOCALE: &str = "en";

const FIELD_REFERENCE_URL: &str =
    "https://github.com/opendsr-std/seedfaker/blob/main/docs/field-reference.md";
const QUICK_START_URL: &str =
    "https://github.com/opendsr-std/seedfaker/blob/main/docs/quick-start.md";

pub fn print_list() {
    let env =
        opts::resolve_all(Some(SAMPLE_SEED), Some(SAMPLE_LOCALE), None, None, Some(SAMPLE_UNTIL))
            .ok();

    // Account for longest possible spec including modifiers (e.g. "phone:e164").
    let name_col = REGISTRY
        .iter()
        .map(|f| {
            let mods = field_modifiers(f.id);
            let extra = mods
                .split(", ")
                .filter(|s| !s.is_empty())
                .map(|m| f.name.len() + 1 + m.len())
                .max()
                .unwrap_or(0);
            f.name.len().max(extra)
        })
        .max()
        .unwrap_or(0)
        + NAME_GAP;
    let sample_pad = " ".repeat(SAMPLE_GAP);

    let render_sample = |spec: &str| -> String {
        let raw = env
            .as_ref()
            .map(|(seed, locales, tz, since, until)| {
                sample_value(spec, *seed, locales, *tz, *since, *until)
            })
            .unwrap_or_default();
        fit_sample(&raw, SAMPLE_WIDTH)
    };

    for group in GROUPS {
        println!("\n  {group}:");
        for f in REGISTRY.iter().filter(|f| f.group == *group) {
            let caps = field_capabilities(f.id);
            let sample_col = render_sample(f.name);
            if caps.is_empty() {
                println!("    {:<name_col$}{sample_col}{sample_pad}{}", f.name, f.description);
            } else {
                println!(
                    "    {:<name_col$}{sample_col}{sample_pad}{}  {{{}}}",
                    f.name, f.description, caps
                );
            }
            let mods = field_modifiers(f.id);
            for m in mods.split(", ").filter(|s| !s.is_empty()) {
                let spec = format!("{}:{}", f.name, m);
                let sample_col = render_sample(&spec);
                // Trim trailing spaces — modifier lines have no description column.
                println!("    {:<name_col$}{}", spec, sample_col.trim_end());
            }
        }
    }
    println!();
    println!("All fields support :upper, :lower, and :capitalize transforms.");
    println!();
    println!("Full field reference: {FIELD_REFERENCE_URL}");
}

/// Open the seedfaker quick-start guide in the user's default browser.
/// Falls back to printing the URL if no browser opener is available.
pub fn open_docs() {
    use std::process::Command;

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(QUICK_START_URL).status();

    #[cfg(target_os = "windows")]
    let result = Command::new("cmd").args(["/C", "start", "", QUICK_START_URL]).status();

    #[cfg(all(unix, not(target_os = "macos")))]
    let result = Command::new("xdg-open").arg(QUICK_START_URL).status();

    match result {
        Ok(status) if status.success() => {}
        _ => println!("Open in your browser: {QUICK_START_URL}"),
    }
}

/// Generate a single sample value for a field, with a fixed seed for stability.
/// Returns empty string on any parse/lookup failure (best-effort display only).
fn sample_value(
    name: &str,
    master_seed: u64,
    locales: &[&'static seedfaker_core::locale::Locale],
    tz: i32,
    since: i64,
    until: i64,
) -> String {
    let Ok((n, m, transform, range_spec, _, omit_pct, _)) = field::parse_field_spec(name) else {
        return String::new();
    };
    let Some(f) = field::lookup(n) else {
        return String::new();
    };
    let range = field::resolve_range(&range_spec, f.name, since, until);
    let dh = pipeline::field_domain_hash(master_seed, f, m);
    let fs =
        pipeline::FieldSpec { field: f, modifier: m, domain_hash: dh, range, transform, omit_pct };
    let mut counter = 0u64;
    let mut vals = pipeline::generate_field_values(&fs, 1, &mut counter, locales, tz, since, until);
    vals.pop().unwrap_or_default()
}

/// Approximate display width in terminal cells. CJK ideographs, fullwidth forms,
/// emoji, and Hangul syllables occupy 2 cells; everything else 1.
fn char_cells(c: char) -> usize {
    let cp = c as u32;
    if (0x1100..=0x115F).contains(&cp)        // Hangul Jamo
        || (0x2E80..=0x303E).contains(&cp)     // CJK radicals & punctuation
        || (0x3041..=0x33FF).contains(&cp)     // Hiragana, Katakana, CJK symbols
        || (0x3400..=0x4DBF).contains(&cp)     // CJK Ext A
        || (0x4E00..=0x9FFF).contains(&cp)     // CJK Unified Ideographs
        || (0xA000..=0xA4CF).contains(&cp)     // Yi
        || (0xAC00..=0xD7A3).contains(&cp)     // Hangul Syllables
        || (0xF900..=0xFAFF).contains(&cp)     // CJK Compatibility Ideographs
        || (0xFE30..=0xFE4F).contains(&cp)     // CJK Compatibility Forms
        || (0xFF00..=0xFF60).contains(&cp)     // Fullwidth Latin
        || (0xFFE0..=0xFFE6).contains(&cp)     // Fullwidth signs
        || (0x1F300..=0x1FAFF).contains(&cp)   // Symbols & pictographs (emoji)
        || (0x20000..=0x2FFFD).contains(&cp)
    // CJK Ext B-F
    {
        2
    } else {
        1
    }
}

/// Sanitize and fit a sample value into exactly `width` terminal cells.
/// Replaces control chars with spaces, truncates with `...` suffix if too wide,
/// pads with trailing spaces if too narrow. Counts terminal cells, not bytes/chars,
/// so CJK and emoji align correctly.
fn fit_sample(s: &str, width: usize) -> String {
    let cleaned: String = s.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();

    let total_cells: usize = cleaned.chars().map(char_cells).sum();
    if total_cells <= width {
        let pad = width - total_cells;
        let mut out = cleaned;
        for _ in 0..pad {
            out.push(' ');
        }
        return out;
    }

    // Truncate cell-by-cell, leaving room for "..." (3 cells).
    let budget = width.saturating_sub(3);
    let mut acc = String::new();
    let mut used = 0usize;
    for c in cleaned.chars() {
        let w = char_cells(c);
        if used + w > budget {
            break;
        }
        acc.push(c);
        used += w;
    }
    // Pad if we landed short due to a wide char that wouldn't fit.
    for _ in used..budget {
        acc.push(' ');
    }
    acc.push_str("...");
    acc
}

pub fn print_list_json() {
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let _ = out.write_all(b"[");
    let mut first = true;
    for f in REGISTRY {
        if !first {
            let _ = out.write_all(b",");
        }
        first = false;
        let mods = field_modifiers(f.id);
        let mods_arr: Vec<&str> =
            if mods.is_empty() { Vec::new() } else { mods.split(", ").map(str::trim).collect() };
        let _ = out.write_all(b"{\"id\":");
        write_json_string(&mut out, f.id);
        let _ = out.write_all(b",\"name\":");
        write_json_string(&mut out, f.name);
        let _ = out.write_all(b",\"group\":");
        write_json_string(&mut out, f.group);
        let _ = out.write_all(b",\"description\":");
        write_json_string(&mut out, f.description);
        let _ = out.write_all(b",\"modifiers\":[");
        for (i, m) in mods_arr.iter().enumerate() {
            if i > 0 {
                let _ = out.write_all(b",");
            }
            write_json_string(&mut out, m);
        }
        let _ = out.write_all(b"]}");
    }
    let _ = out.write_all(b"]\n");
}

fn write_json_string(out: &mut impl std::io::Write, s: &str) {
    let _ = out.write_all(b"\"");
    for ch in s.chars() {
        match ch {
            '"' => {
                let _ = out.write_all(b"\\\"");
            }
            '\\' => {
                let _ = out.write_all(b"\\\\");
            }
            '\n' => {
                let _ = out.write_all(b"\\n");
            }
            '\r' => {
                let _ = out.write_all(b"\\r");
            }
            '\t' => {
                let _ = out.write_all(b"\\t");
            }
            c if c < '\x20' => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => {
                let mut buf = [0u8; 4];
                let _ = out.write_all(c.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    let _ = out.write_all(b"\"");
}

pub fn print_field_help(name: &str) {
    let Some(f) = lookup(name) else {
        eprintln!("Unknown field: \"{name}\"");
        let suggestions: Vec<&str> = all_names()
            .into_iter()
            .filter(|n| n.contains(name) || name.contains(*n))
            .take(5)
            .collect();
        if !suggestions.is_empty() {
            eprintln!("\nDid you mean?");
            for s in &suggestions {
                if let Some(f) = lookup(s) {
                    eprintln!("    {:<24} {}", f.name, f.description);
                }
            }
        }
        eprintln!("\nSee all: seedfaker --list");
        return;
    };
    println!("{} ({})", f.name, f.group);
    println!();
    println!("    {}", f.description);
    let mods = field_modifiers(f.id);
    if !mods.is_empty() {
        println!();
        println!("    Modifiers: {mods}");
    }
    println!();
    println!("    Usage:");
    println!("      seedfaker {}", f.name);
    println!("      seedfaker {} -n 1000 --seed prod", f.name);
    if !mods.is_empty() {
        let first_mod = mods.split(", ").next().unwrap_or("");
        println!("      seedfaker {}:{}", f.name, first_mod);
    }
    let related: Vec<&str> = REGISTRY
        .iter()
        .filter(|r| r.group == f.group && r.name != f.name)
        .map(|r| r.name)
        .take(6)
        .collect();
    if !related.is_empty() {
        println!();
        println!("    Related: {}", related.join(", "));
    }
}

pub fn print_fields_table() {
    for group in GROUPS {
        let fields: Vec<&str> =
            REGISTRY.iter().filter(|f| f.group == *group).map(|f| f.name).collect();
        let count = fields.len();
        println!("| `{}` | {} | {} |", group, count, fields.join(", "));
    }
}
