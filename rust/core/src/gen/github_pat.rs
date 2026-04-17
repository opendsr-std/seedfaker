use crate::ctx::GenContext;

// Format: GitHub PAT — https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let arr = ["ghp", "gho", "ghu", "ghs", "ghr"];
    let p = arr[ctx.rng.urange(0, arr.len() - 1)];
    // prefix(3) + "_" (1) + alnum(36) = 40
    buf.reserve(40);
    buf.push_str(p);
    buf.push('_');
    ctx.rng.push_alnum(buf, 36);
}
