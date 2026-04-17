use crate::ctx::GenContext;

use super::helpers::locale_to_currency;

const KNOWN_CRYPTO: &[&str] = &[
    "BTC", "ETH", "SOL", "BNB", "XRP", "ADA", "DOGE", "DOT", "AVAX", "MATIC", "LINK", "UNI",
    "ATOM", "LTC", "FIL", "NEAR", "APT", "ARB", "OP", "SHIB", "PEPE", "WIF", "BONK", "FLOKI",
    "MEME", "TURBO", "WOJAK", "LADYS", "SATS", "ORDI", "INJ", "TIA", "SUI", "SEI", "STRK", "JUP",
    "PYTH", "JTO", "ONDO", "RENDER", "FET", "AGIX", "OCEAN", "TAO", "WLD", "ARKM", "USDT", "USDC",
    "DAI", "BUSD", "FRAX", "TUSD", "LUSD", "WBTC", "WETH", "stETH", "cbETH", "rETH",
];

// ISO 4217 / crypto tickers
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if ctx.modifier == "crypto" {
        if ctx.rng.maybe(0.7) {
            buf.push_str(KNOWN_CRYPTO[ctx.rng.urange(0, KNOWN_CRYPTO.len() - 1)]);
        } else {
            let len = ctx.rng.urange(3, 5);
            ctx.rng.push_upper(buf, len);
        }
    } else {
        let loc = ctx.locale();
        buf.push_str(locale_to_currency(loc.code));
    }
}
