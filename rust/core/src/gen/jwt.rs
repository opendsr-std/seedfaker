use std::fmt::Write;

use crate::ctx::GenContext;

use super::email::gen as gen_email;
use super::helpers::base64url::base64url_encode;

const SIG_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

// Format: RFC 7519 (JWT) — https://www.rfc-editor.org/rfc/rfc7519
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let header = base64url_encode(b"{\"alg\":\"HS256\",\"typ\":\"JWT\"}");
    let email = {
        let mut __b = String::new();
        gen_email(ctx, &mut __b);
        __b
    };
    let sub = ctx.rng.range(100_000, 999_999);
    let iat = ctx.rng.range(1_700_000_000, 1_710_000_000);
    let exp = ctx.rng.range(1_710_000_000, 1_720_000_000);
    let mut payload = String::with_capacity(80);
    let _ =
        write!(payload, "{{\"sub\":\"{sub}\",\"email\":\"{email}\",\"iat\":{iat},\"exp\":{exp}}}");
    let body = base64url_encode(payload.as_bytes());
    // header(~36) + '.' + body(~100) + '.' + sig(43) ≈ 182
    buf.reserve(header.len() + 1 + body.len() + 1 + 43);
    buf.push_str(&header);
    buf.push('.');
    buf.push_str(&body);
    buf.push('.');
    ctx.rng.push_charset(buf, SIG_CHARSET, 43);
}
