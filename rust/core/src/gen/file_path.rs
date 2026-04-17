use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let unix_dirs = ["/home", "/var/log", "/tmp", "/opt", "/srv", "/etc"];
    let win_dirs = ["C:\\Users", "C:\\Program Files", "D:\\Data", "C:\\Windows\\Temp"];
    let names = [
        "report", "data", "export", "backup", "config", "log", "output", "dump", "archive",
        "snapshot",
    ];
    let exts = ["csv", "json", "xml", "pdf", "txt", "log", "sql", "xlsx", "tar.gz", "zip"];
    let name = names[ctx.rng.urange(0, names.len() - 1)];
    let ext = exts[ctx.rng.urange(0, exts.len() - 1)];
    if ctx.rng.maybe(0.7) {
        let dir = unix_dirs[ctx.rng.urange(0, unix_dirs.len() - 1)];
        // dir + / + 6chars + / + name + . + ext
        buf.reserve(dir.len() + 1 + 6 + 1 + name.len() + 1 + ext.len());
        buf.push_str(dir);
        buf.push('/');
        ctx.rng.push_lower_digit(buf, 6);
        buf.push('/');
        buf.push_str(name);
        buf.push('.');
        buf.push_str(ext);
    } else {
        let dir = win_dirs[ctx.rng.urange(0, win_dirs.len() - 1)];
        buf.reserve(dir.len() + 1 + 6 + 1 + name.len() + 1 + ext.len());
        buf.push_str(dir);
        buf.push('\\');
        ctx.rng.push_alnum(buf, 6);
        buf.push('\\');
        buf.push_str(name);
        buf.push('.');
        buf.push_str(ext);
    }
}
