use crate::ctx::GenContext;

// Format: AWS Access Key ID — https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    const UPPER_DIGIT: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    buf.reserve(4 + 16);
    buf.push_str("AKIA");
    ctx.rng.push_charset(buf, UPPER_DIGIT, 16);
}
