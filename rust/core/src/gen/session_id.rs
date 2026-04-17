use crate::ctx::GenContext;

// Format: Session ID — https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(5 + 32);
    buf.push_str("sess_");
    ctx.rng.push_hex(buf, 32);
}
