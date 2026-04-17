//! Tokenizer: splits template source into literals, expressions, and block tags.

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Literal(&'a str),
    Expr(&'a str),
    BlockOpen { tag: &'a str, rest: &'a str },
    BlockClose(&'a str),
    Else,
}

/// Remove lines that contain ONLY block tags and whitespace.
/// Block-tag-only lines produce zero output (no newline, no space).
/// Template is trimmed front and back.
pub fn strip_tag_lines(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut prev_was_content = false;
    for line in src.trim().lines() {
        let trimmed = line.trim();
        if is_block_only_line(trimmed) {
            out.push_str(trimmed);
            prev_was_content = false;
        } else {
            if prev_was_content {
                out.push('\n');
            }
            out.push_str(line);
            prev_was_content = true;
        }
    }
    out
}

fn is_block_only_line(line: &str) -> bool {
    if line.is_empty() {
        return false;
    }
    // Must start with {{ and be entirely block tags
    let mut rest = line;
    let mut found_block = false;
    while let Some(start) = rest.find("{{") {
        // Everything before {{ must be whitespace
        if !rest[..start].trim().is_empty() {
            return false;
        }
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else { return false };
        let inner = after[..end].trim();
        let is_block = inner == "else" || inner.starts_with('#') || inner.starts_with('/');
        if !is_block {
            return false;
        }
        found_block = true;
        rest = &after[end + 2..];
    }
    // Remaining text after last }} must be whitespace
    found_block && rest.trim().is_empty()
}

pub fn tokenize(src: &str) -> Result<Vec<Token<'_>>, String> {
    let mut tokens = Vec::new();
    let mut rest = src;

    while let Some(start) = rest.find("{{") {
        if start > 0 {
            tokens.push(Token::Literal(&rest[..start]));
        }

        let after = &rest[start + 2..];
        if let Some(end) = after.find("}}") {
            let inner = after[..end].trim();

            if inner == "else" {
                tokens.push(Token::Else);
            } else if let Some(block) = inner.strip_prefix('#') {
                let block = block.trim_start();
                let (tag, block_rest) = match block.find(|c: char| c.is_whitespace()) {
                    Some(pos) => (&block[..pos], block[pos..].trim()),
                    None => (block, ""),
                };
                tokens.push(Token::BlockOpen { tag, rest: block_rest });
            } else if let Some(close) = inner.strip_prefix('/') {
                tokens.push(Token::BlockClose(close.trim()));
            } else {
                tokens.push(Token::Expr(inner));
            }

            rest = &after[end + 2..];
        } else {
            return Err(format!("unclosed '{{{{' at position {start}"));
        }
    }

    if !rest.is_empty() {
        tokens.push(Token::Literal(rest));
    }

    Ok(tokens)
}
