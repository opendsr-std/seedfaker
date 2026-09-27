/// Extract field specs from a CLI inline template for auto-detection.
pub fn template_fields(tpl: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut rest = tpl;
    while let Some(start) = rest.find("{{") {
        let after = &rest[start + 2..];
        if let Some(end) = after.find("}}") {
            let token = after[..end].trim();
            if !token.is_empty()
                && token != "serial"
                && !token.starts_with('#')
                && !token.starts_with('/')
            {
                fields.push(token.to_string());
            }
            rest = &after[end + 2..];
        } else {
            break;
        }
    }
    fields
}
