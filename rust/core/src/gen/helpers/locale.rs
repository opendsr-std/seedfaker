pub fn pick_locale<'a>(
    rng: &mut crate::rng::Rng,
    locales: &'a [&'a crate::locale::Locale],
) -> &'a crate::locale::Locale {
    locales[rng.urange(0, locales.len() - 1)]
}
