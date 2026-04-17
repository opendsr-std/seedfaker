use std::io::Write;

use crate::writers;

/// Write annotated JSONL line: {"text":"...","spans":[{"s":0,"e":5,"f":"name","v":"John","o":"..."},...]}
pub fn write_annotated_line(
    out: &mut impl Write,
    segments: &[crate::tpl::render::Segment],
    originals: Option<&[String]>,
) -> std::io::Result<()> {
    let mut text = String::new();
    let mut spans: Vec<(usize, usize, &str, &str, Option<&str>)> = Vec::new();
    let mut vi: usize = 0;

    for seg in segments {
        match seg {
            crate::tpl::render::Segment::Lit(s) => text.push_str(s),
            crate::tpl::render::Segment::Value { field_name, value } => {
                let start = text.len();
                text.push_str(value);
                let end = text.len();
                let original =
                    originals
                        .and_then(|o| if o[vi] == *value { None } else { Some(o[vi].as_str()) });
                spans.push((start, end, field_name, value.as_str(), original));
                vi += 1;
            }
        }
    }

    let mut ibuf = itoa::Buffer::new();
    out.write_all(b"{\"text\":\"")?;
    writers::json_escape_write(out, text.as_bytes())?;
    out.write_all(b"\",\"spans\":[")?;

    for (j, (s, e, f, v, o)) in spans.iter().enumerate() {
        if j > 0 {
            out.write_all(b",")?;
        }
        out.write_all(b"{\"s\":")?;
        out.write_all(ibuf.format(*s).as_bytes())?;
        out.write_all(b",\"e\":")?;
        out.write_all(ibuf.format(*e).as_bytes())?;
        out.write_all(b",\"f\":\"")?;
        out.write_all(f.as_bytes())?;
        out.write_all(b"\",\"v\":\"")?;
        writers::json_escape_write(out, v.as_bytes())?;
        out.write_all(b"\"")?;
        if let Some(orig) = o {
            out.write_all(b",\"o\":\"")?;
            writers::json_escape_write(out, orig.as_bytes())?;
            out.write_all(b"\"")?;
        }
        out.write_all(b"}")?;
    }

    out.write_all(b"]}\n")
}
