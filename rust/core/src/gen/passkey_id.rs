use crate::ctx::GenContext;

// Format: WebAuthn Credential ID — https://www.w3.org/TR/webauthn-3/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_charset(
        buf,
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
        43,
    );
}
