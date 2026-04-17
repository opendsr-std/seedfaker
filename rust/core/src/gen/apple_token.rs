use crate::ctx::GenContext;

use super::helpers::base64url::base64url_encode;

// Format: Sign in with Apple — https://developer.apple.com/documentation/sign_in_with_apple
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let header = base64url_encode(b"{\"kid\":\"W6Gj\",\"alg\":\"ES256\"}");
    let mut payload_json = String::with_capacity(60 + 20);
    payload_json.push_str("{\"iss\":\"https://appleid.apple.com\",\"sub\":\"");
    ctx.rng.push_alnum(&mut payload_json, 20);
    payload_json.push_str("\"}");
    let payload = base64url_encode(payload_json.as_bytes());
    let sig = ctx
        .rng
        .charset_string(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_", 43);
    // header(~40) + "." + payload(~80) + "." + sig(43) ~ 170
    buf.reserve(header.len() + 1 + payload.len() + 1 + 43);
    buf.push_str(&header);
    buf.push('.');
    buf.push_str(&payload);
    buf.push('.');
    buf.push_str(&sig);
}
