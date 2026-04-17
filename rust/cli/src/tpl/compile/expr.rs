//! Compile `{{expr}}` tokens: var refs, inline field generation, `serial`.

use seedfaker_core::field::{self, Transform};

use super::node::Node;

/// Resolve `{{expr}}` — column reference if declared, otherwise inline field.
pub fn compile_expr(expr: &str, var_names: &[&str]) -> Result<Node, String> {
    let expr = expr.trim();

    // Column reference: name exists in declared columns.
    if let Some(idx) = var_names.iter().position(|n| *n == expr) {
        let idx = u16::try_from(idx).map_err(|_| format!("too many vars (max {})", u16::MAX))?;
        return Ok(Node::Var(idx));
    }

    // Inline field generation: name exists in field registry.
    compile_inline_field(expr)
}

fn compile_inline_field(spec: &str) -> Result<Node, String> {
    if let Some(values) = spec.strip_prefix("enum:") {
        let f = field::lookup("enum").ok_or("internal error: 'enum' not in registry")?;
        return Ok(Node::New {
            field: f,
            modifier: values.into(),
            transform: Transform::None,
            range: None,
        });
    }

    let (name, modifier, transform, range, _ordering, _omit_pct, _zipf) =
        field::parse_field_spec(spec).map_err(|e| format!("{{{{{spec}}}}}: {e}"))?;

    let f = field::lookup(name)
        .ok_or_else(|| format!("{{{{{spec}}}}}: unknown field or column '{name}'"))?;

    Ok(Node::New { field: f, modifier: modifier.into(), transform, range })
}
