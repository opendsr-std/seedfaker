use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    {
        super::jmbg::gen(ctx, buf);
    }
}
