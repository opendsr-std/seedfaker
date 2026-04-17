use crate::ctx::GenContext;

// Format: Germany Steuer-ID — https://www.bzst.de/DE/Privatpersonen/StesuerlicheIdentifikationsnummer/steuerlicheidentifikationsnummer_node.html
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_digits(buf, 11);
}
