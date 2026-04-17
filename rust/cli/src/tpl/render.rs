//! Template render: collect segments, then assemble into text.
//!
//! Two-phase pipeline: `collect()` walks the compiled node tree and produces
//! a `Vec<Segment>`. `assemble_text()` concatenates segments into the final
//! output string. Between the two phases, corruption can modify all generated
//! values (declared columns + inline fields).

use seedfaker_core::ctx::Identity;
use seedfaker_core::field::{RangeSpec, Transform};
use seedfaker_core::locale::Locale;
use seedfaker_core::rng::Rng;

use super::compile::{CmpOp, CompiledTemplate, Node};

pub struct RenderCtx<'a> {
    pub values: &'a [String],
    pub rng: &'a mut Rng,
    pub locales: &'a [&'a Locale],
    pub identity: Option<&'a Identity>,
    pub tz_offset_minutes: i32,
    pub since: i64,
    pub until: i64,
    pub field_types: &'a [&'static str],
}

pub enum Segment {
    Lit(Box<str>),
    Value { field_name: &'static str, value: String },
}

pub fn render(tpl: &CompiledTemplate, ctx: &mut RenderCtx<'_>, buf: &mut String) {
    let segments = collect(tpl, ctx);
    assemble_text(&segments, buf);
}

pub fn collect(tpl: &CompiledTemplate, ctx: &mut RenderCtx<'_>) -> Vec<Segment> {
    let mut out = Vec::new();
    collect_nodes(&tpl.nodes, ctx, &mut out);
    out
}

/// Build segments for structured (non-template) output.
/// Used by --annotated to produce spans for CSV/TSV/JSONL/SQL.
pub fn structured_segments(
    col_names: &[String],
    values: &[String],
    is_omitted: &[bool],
    field_types: &[&'static str],
    format: &crate::engine::OutputMode,
    delim: &str,
    sql_prefix: Option<&[u8]>,
) -> Vec<Segment> {
    let mut segs = Vec::new();
    let col_count = col_names.len();

    match format {
        crate::engine::OutputMode::Jsonl => {
            segs.push(Segment::Lit("{".into()));
            for i in 0..col_count {
                if i > 0 {
                    segs.push(Segment::Lit(",".into()));
                }
                let key = format!("\"{}\":", col_names[i]);
                segs.push(Segment::Lit(key.into()));
                if is_omitted[i] {
                    segs.push(Segment::Lit("null".into()));
                } else {
                    segs.push(Segment::Lit("\"".into()));
                    let escaped = crate::writers::json_escape_string(&values[i]);
                    segs.push(Segment::Value {
                        field_name: field_types.get(i).copied().unwrap_or("unknown"),
                        value: escaped,
                    });
                    segs.push(Segment::Lit("\"".into()));
                }
            }
            segs.push(Segment::Lit("}".into()));
        }
        crate::engine::OutputMode::Sql(_) => {
            if let Some(prefix) = sql_prefix {
                let prefix_str = String::from_utf8_lossy(prefix);
                segs.push(Segment::Lit(prefix_str.into_owned().into()));
            }
            segs.push(Segment::Lit("(".into()));
            for i in 0..col_count {
                if i > 0 {
                    segs.push(Segment::Lit(", ".into()));
                }
                if is_omitted[i] {
                    segs.push(Segment::Lit("NULL".into()));
                } else {
                    segs.push(Segment::Lit("'".into()));
                    segs.push(Segment::Value {
                        field_name: field_types.get(i).copied().unwrap_or("unknown"),
                        value: values[i].replace('\'', "''"),
                    });
                    segs.push(Segment::Lit("'".into()));
                }
            }
            segs.push(Segment::Lit(");".into()));
        }
        _ => {
            // Default / TSV / CSV
            let sep = match format {
                crate::engine::OutputMode::Csv => ",",
                _ => delim,
            };
            for i in 0..col_count {
                if i > 0 {
                    segs.push(Segment::Lit(sep.to_string().into()));
                }
                if is_omitted[i] {
                    // empty for CSV/TSV
                } else {
                    segs.push(Segment::Value {
                        field_name: field_types.get(i).copied().unwrap_or("unknown"),
                        value: values[i].clone(),
                    });
                }
            }
        }
    }
    segs
}

pub fn assemble_text(segments: &[Segment], buf: &mut String) {
    for seg in segments {
        match seg {
            Segment::Lit(s) => buf.push_str(s),
            Segment::Value { value, .. } => buf.push_str(value),
        }
    }
}

fn collect_nodes(nodes: &[Node], ctx: &mut RenderCtx<'_>, out: &mut Vec<Segment>) {
    for node in nodes {
        match node {
            Node::Lit(s) => out.push(Segment::Lit(s.clone())),

            Node::Var(idx) => {
                let i = *idx as usize;
                let field_name: &'static str =
                    if i < ctx.field_types.len() { ctx.field_types[i] } else { "serial" };
                out.push(Segment::Value {
                    field_name,
                    value: ctx.values.get(i).cloned().unwrap_or_default(),
                });
            }

            Node::New { field, modifier, transform, range } => {
                let rng = std::mem::replace(ctx.rng, Rng::new(0));
                let resolved_range = resolve_range(range, field.name, ctx.since, ctx.until);
                let mut gen_ctx = seedfaker_core::ctx::GenContext {
                    rng,
                    locales: ctx.locales,
                    modifier,
                    identity: ctx.identity,
                    tz_offset_minutes: ctx.tz_offset_minutes,
                    since: ctx.since,
                    until: ctx.until,
                    range: resolved_range,
                    ordering: seedfaker_core::field::Ordering::None,
                    zipf: None,
                    numeric: None,
                };
                let mut buf = String::new();
                field.generate(&mut gen_ctx, &mut buf);
                *ctx.rng = gen_ctx.rng;
                if *transform != Transform::None {
                    buf = transform.apply(&buf);
                }
                out.push(Segment::Value { field_name: field.name, value: buf });
            }

            Node::If { branches, else_body } => {
                let matched = branches.iter().find(|(cond, _)| {
                    ctx.values.get(cond.var as usize).is_some_and(|val| match cond.op {
                        CmpOp::Eq => val.as_str() == cond.value.as_ref(),
                        CmpOp::Neq => val.as_str() != cond.value.as_ref(),
                    })
                });
                let body = match matched {
                    Some((_, body)) => body.as_ref(),
                    None => else_body.as_ref(),
                };
                collect_nodes(body, ctx, out);
            }

            Node::Repeat { count, body } => {
                for _ in 0..*count {
                    collect_nodes(body, ctx, out);
                }
            }
        }
    }
}

fn resolve_range(
    range: &Option<RangeSpec>,
    field_name: &str,
    since: i64,
    until: i64,
) -> Option<(i64, i64)> {
    let r = range.as_ref()?;
    let is_date = matches!(field_name, "date" | "birthdate" | "timestamp");
    let (default_min, default_max) = if is_date { (since, until) } else { (0, 999_999) };
    let mut from = r.from.unwrap_or(default_min);
    let mut to = r.to.unwrap_or(default_max);
    if is_date {
        if from > 0 && from <= 9999 {
            from = seedfaker_core::temporal::parse(&from.to_string()).unwrap_or(from);
        }
        if to > 0 && to <= 9999 {
            to = seedfaker_core::temporal::parse_until(&to.to_string()).unwrap_or(to);
        }
    }
    if from >= to {
        return None;
    }
    Some((from, to))
}
