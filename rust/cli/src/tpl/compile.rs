//! Compiles tokenized template into an executable `Node` tree.
//!
//! All column names are resolved to indices at compile time.
//! Unresolved names are looked up in the field registry for inline generation.

mod blocks;
pub mod expr;
mod node;

pub use node::{CmpOp, CompiledTemplate, Node};

use super::parse::{self, Token};

const MAX_NESTING_DEPTH: u32 = 30;
const MAX_REPEAT_COUNT: u16 = 100;

/// Compile a template string against a set of var names.
///
/// All var references and field specs are resolved at compile time.
/// Returns error if a name is neither a declared column nor a known field.
pub fn compile(template: &str, var_names: &[&str]) -> Result<CompiledTemplate, String> {
    let processed = parse::strip_tag_lines(template);
    let tokens = parse::tokenize(&processed)?;
    let mut pos = 0;
    let nodes = compile_nodes(&tokens, &mut pos, var_names, None, 0)?;
    if pos < tokens.len() {
        return Err(format!(
            "unexpected closing tag at position {pos}; check {{{{/if}}}} and {{{{/repeat}}}} balance"
        ));
    }
    Ok(CompiledTemplate { nodes })
}

/// Recursively compile tokens into nodes until end-of-input or a closing tag.
pub(crate) fn compile_nodes(
    tokens: &[Token<'_>],
    pos: &mut usize,
    var_names: &[&str],
    stop_at: Option<&str>,
    depth: u32,
) -> Result<Box<[Node]>, String> {
    if depth > MAX_NESTING_DEPTH {
        return Err(format!("template nesting too deep (max {MAX_NESTING_DEPTH} levels)"));
    }

    let mut nodes = Vec::new();

    while *pos < tokens.len() {
        match &tokens[*pos] {
            Token::Literal(s) => {
                nodes.push(Node::Lit((*s).into()));
                *pos += 1;
            }
            Token::Expr(e) => {
                nodes.push(expr::compile_expr(e, var_names)?);
                *pos += 1;
            }
            Token::BlockOpen { tag, rest } => match *tag {
                "if" => nodes.push(blocks::compile_if(tokens, pos, var_names, rest, depth + 1)?),
                "elif" | "else" => break,
                "repeat" => {
                    nodes.push(blocks::compile_repeat(tokens, pos, var_names, rest, depth + 1)?);
                }
                other => {
                    return Err(format!(
                        "unknown block tag '{{{{#{other}}}}}'; expected 'if' or 'repeat'"
                    ))
                }
            },
            Token::BlockClose(tag) => {
                if stop_at == Some(*tag) {
                    *pos += 1;
                    return Ok(nodes.into_boxed_slice());
                }
                break;
            }
            Token::Else => break,
        }
    }

    if let Some(expected) = stop_at {
        return Err(format!("unclosed {{{{#{expected}}}}}: expected {{{{/{expected}}}}}"));
    }
    Ok(nodes.into_boxed_slice())
}
