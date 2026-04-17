//! Block compilation: `{{#if}}`, `{{#repeat}}`, and condition parsing.

use super::super::parse::Token;

use super::compile_nodes;
use super::expr::compile_expr;
use super::node::{CmpOp, Cond, Node};

// ---------------------------------------------------------------------------
// Condition parsing: `var == "value"` and `var != "value"`.
// ---------------------------------------------------------------------------

fn parse_condition(s: &str, var_names: &[&str]) -> Result<Cond, String> {
    let s = s.trim();

    let (var_part, op, val_part) = if let Some(pos) = s.find("==") {
        (&s[..pos], CmpOp::Eq, &s[pos + 2..])
    } else if let Some(pos) = s.find("!=") {
        (&s[..pos], CmpOp::Neq, &s[pos + 2..])
    } else {
        return Err(format!(
            "invalid condition '{s}': expected 'var == \"value\"' or 'var != \"value\"'"
        ));
    };

    let var_part = var_part.trim();
    let var_idx = var_names
        .iter()
        .position(|n| *n == var_part)
        .ok_or_else(|| format!("unknown var '{var_part}' in condition"))?;
    let var_idx =
        u16::try_from(var_idx).map_err(|_| format!("too many vars (max {})", u16::MAX))?;

    let value = parse_quoted_value(val_part.trim())?;

    Ok(Cond { var: var_idx, op, value: value.into() })
}

fn parse_quoted_value(s: &str) -> Result<&str, String> {
    if s.len() >= 2 {
        if s.starts_with('"') && s.ends_with('"') {
            return Ok(&s[1..s.len() - 1]);
        }
        if s.starts_with('\'') && s.ends_with('\'') {
            return Ok(&s[1..s.len() - 1]);
        }
    }
    Err(format!("condition value must be quoted: expected '\"value\"', got '{s}'"))
}

// ---------------------------------------------------------------------------
// Block compilation
// ---------------------------------------------------------------------------

pub fn compile_if(
    tokens: &[Token<'_>],
    pos: &mut usize,
    var_names: &[&str],
    first_cond_str: &str,
    depth: u32,
) -> Result<Node, String> {
    let mut branches = Vec::new();

    let cond = parse_condition(first_cond_str, var_names)?;
    *pos += 1;
    let body = compile_if_body(tokens, pos, var_names, depth)?;
    branches.push((cond, body));

    loop {
        if *pos >= tokens.len() {
            return Err("unclosed {{#if}}: expected {{/if}}".into());
        }
        match &tokens[*pos] {
            Token::BlockOpen { tag: "elif", rest } => {
                let cond = parse_condition(rest, var_names)?;
                *pos += 1;
                let body = compile_if_body(tokens, pos, var_names, depth)?;
                branches.push((cond, body));
            }
            Token::Else => {
                *pos += 1;
                let else_body = compile_nodes(tokens, pos, var_names, Some("if"), depth)?;
                return Ok(Node::If { branches: branches.into_boxed_slice(), else_body });
            }
            Token::BlockClose("if") => {
                *pos += 1;
                return Ok(Node::If {
                    branches: branches.into_boxed_slice(),
                    else_body: Box::new([]),
                });
            }
            _ => return Err("expected {{#elif}}, {{else}}, or {{/if}}".into()),
        }
    }
}

fn compile_if_body(
    tokens: &[Token<'_>],
    pos: &mut usize,
    var_names: &[&str],
    depth: u32,
) -> Result<Box<[Node]>, String> {
    let mut nodes = Vec::new();
    while *pos < tokens.len() {
        match &tokens[*pos] {
            Token::BlockOpen { tag: "elif", .. } | Token::Else | Token::BlockClose("if") => {
                return Ok(nodes.into_boxed_slice());
            }
            Token::Literal(s) => {
                nodes.push(Node::Lit((*s).into()));
                *pos += 1;
            }
            Token::Expr(e) => {
                nodes.push(compile_expr(e, var_names)?);
                *pos += 1;
            }
            Token::BlockOpen { tag: "if", rest } => {
                nodes.push(compile_if(tokens, pos, var_names, rest, depth + 1)?);
            }
            Token::BlockOpen { tag: "repeat", rest } => {
                nodes.push(compile_repeat(tokens, pos, var_names, rest, depth + 1)?);
            }
            Token::BlockOpen { tag, .. } => {
                return Err(format!("unknown block tag '{{{{#{tag}}}}}' inside if-body"));
            }
            Token::BlockClose(tag) => {
                return Err(format!("unexpected {{{{/{tag}}}}} inside if-body"));
            }
        }
    }
    Err("unclosed {{#if}}: reached end of template".into())
}

pub fn compile_repeat(
    tokens: &[Token<'_>],
    pos: &mut usize,
    var_names: &[&str],
    count_str: &str,
    depth: u32,
) -> Result<Node, String> {
    let count: u16 = count_str
        .trim()
        .parse()
        .map_err(|_| format!("{{{{#repeat {count_str}}}}}: expected a number (1-{MAX_REPEAT})"))?;

    if count > MAX_REPEAT {
        return Err(format!("{{{{#repeat {count}}}}}: count exceeds maximum ({MAX_REPEAT})"));
    }

    *pos += 1;
    let body = compile_nodes(tokens, pos, var_names, Some("repeat"), depth)?;

    Ok(Node::Repeat { count, body })
}

const MAX_REPEAT: u16 = super::MAX_REPEAT_COUNT;
