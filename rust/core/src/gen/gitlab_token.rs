use crate::ctx::GenContext;

// Format: GitLab PAT — https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(6 + 20);
    buf.push_str("glpat-");
    ctx.rng.push_alnum(buf, 20);
}
