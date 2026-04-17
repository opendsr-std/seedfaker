use crate::ctx::GenContext;

// Format: RFC 6838 (MIME types) — https://www.rfc-editor.org/rfc/rfc6838
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let types = [
        "application/json",
        "application/xml",
        "application/pdf",
        "application/octet-stream",
        "text/html",
        "text/plain",
        "text/csv",
        "text/xml",
        "image/png",
        "image/jpeg",
        "image/svg+xml",
        "image/webp",
        "audio/mpeg",
        "video/mp4",
        "multipart/form-data",
    ];
    buf.push_str(types[ctx.rng.urange(0, types.len() - 1)]);
}
