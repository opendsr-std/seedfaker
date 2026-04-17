use std::io::Write;

pub fn write_lines(out: &mut impl Write, values: &[String], sep: &[u8]) -> bool {
    if values.len() == 1 {
        return out.write_all(values[0].as_bytes()).is_ok() && out.write_all(b"\n").is_ok();
    }
    for (i, v) in values.iter().enumerate() {
        if i > 0 && out.write_all(sep).is_err() {
            return false;
        }
        if out.write_all(v.as_bytes()).is_err() {
            return false;
        }
    }
    out.write_all(b"\n").is_ok()
}

pub fn write_csv(out: &mut impl Write, values: &[String], sep: &[u8]) -> bool {
    for (i, v) in values.iter().enumerate() {
        if i > 0 && out.write_all(sep).is_err() {
            return false;
        }
        if write_csv_field(out, v.as_bytes(), sep).is_err() {
            return false;
        }
    }
    out.write_all(b"\n").is_ok()
}

fn write_csv_field(out: &mut impl Write, v: &[u8], sep: &[u8]) -> std::io::Result<()> {
    let needs_quote = if let Some(&first) = v.first() {
        first == b'=' || first == b'+' || first == b'-' || first == b'@'
    } else {
        false
    } || v.windows(sep.len()).any(|w| w == sep)
        || v.iter().any(|&b| b == b'"' || b == b'\n');

    if needs_quote {
        out.write_all(b"\"")?;
        for &b in v {
            if b == b'"' {
                out.write_all(b"\"\"")?;
            } else {
                out.write_all(&[b])?;
            }
        }
        out.write_all(b"\"")?;
    } else {
        out.write_all(v)?;
    }
    Ok(())
}

pub fn write_tsv(out: &mut impl Write, values: &[String], sep: &[u8]) -> bool {
    for (i, v) in values.iter().enumerate() {
        if i > 0 && out.write_all(sep).is_err() {
            return false;
        }
        let bytes = v.as_bytes();
        if sep.len() == 1 && bytes.contains(&sep[0]) {
            for &b in bytes {
                if out.write_all(&[if b == sep[0] { b' ' } else { b }]).is_err() {
                    return false;
                }
            }
        } else if out.write_all(bytes).is_err() {
            return false;
        }
    }
    out.write_all(b"\n").is_ok()
}

pub fn write_jsonl(
    out: &mut impl Write,
    keys: &[String],
    values: &[String],
    is_omitted: &[bool],
) -> bool {
    if out.write_all(b"{").is_err() {
        return false;
    }
    for (i, (k, v)) in keys.iter().zip(values.iter()).enumerate() {
        if i > 0 && out.write_all(b",").is_err() {
            return false;
        }
        if i < is_omitted.len() && is_omitted[i] {
            if out.write_all(b"\"").is_err()
                || out.write_all(k.as_bytes()).is_err()
                || out.write_all(b"\":null").is_err()
            {
                return false;
            }
        } else if out.write_all(b"\"").is_err()
            || out.write_all(k.as_bytes()).is_err()
            || out.write_all(b"\":\"").is_err()
            || json_escape_write(out, v.as_bytes()).is_err()
            || out.write_all(b"\"").is_err()
        {
            return false;
        }
    }
    out.write_all(b"}\n").is_ok()
}

pub fn json_escape_write(out: &mut impl Write, s: &[u8]) -> std::io::Result<()> {
    let mut start = 0;
    for (i, &b) in s.iter().enumerate() {
        let esc: &[u8] = match b {
            b'"' => b"\\\"",
            b'\\' => b"\\\\",
            b'\n' => b"\\n",
            b'\r' => b"\\r",
            b'\t' => b"\\t",
            _ => continue,
        };
        if i > start {
            out.write_all(&s[start..i])?;
        }
        out.write_all(esc)?;
        start = i + 1;
    }
    if start < s.len() {
        out.write_all(&s[start..])?;
    }
    Ok(())
}

pub fn json_escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            _ => out.push(b as char),
        }
    }
    out
}

pub fn write_sql(
    out: &mut impl Write,
    prefix: &[u8],
    values: &[String],
    is_omitted: &[bool],
) -> bool {
    if out.write_all(prefix).is_err() || out.write_all(b"(").is_err() {
        return false;
    }
    for (i, v) in values.iter().enumerate() {
        if i > 0 && out.write_all(b", ").is_err() {
            return false;
        }
        if i < is_omitted.len() && is_omitted[i] {
            if out.write_all(b"NULL").is_err() {
                return false;
            }
        } else {
            if out.write_all(b"'").is_err() {
                return false;
            }
            let bytes = v.as_bytes();
            let mut start = 0;
            for (j, &b) in bytes.iter().enumerate() {
                if b == b'\'' {
                    if j > start && out.write_all(&bytes[start..j]).is_err() {
                        return false;
                    }
                    if out.write_all(b"''").is_err() {
                        return false;
                    }
                    start = j + 1;
                }
            }
            if start < bytes.len() && out.write_all(&bytes[start..]).is_err() {
                return false;
            }
            if out.write_all(b"'").is_err() {
                return false;
            }
        }
    }
    out.write_all(b");\n").is_ok()
}

pub fn sanitize_identifier(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect()
}

pub fn write_header(out: &mut impl Write, names: &[String], sep: &[u8]) -> std::io::Result<()> {
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            out.write_all(sep)?;
        }
        out.write_all(name.as_bytes())?;
    }
    out.write_all(b"\n")
}
