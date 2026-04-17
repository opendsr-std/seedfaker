use crate::ctx::GenContext;

use super::helpers::charsets::B64_CHARSET;

// Format: AWS Secret Access Key — https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_charset(buf, B64_CHARSET, 40);
}
