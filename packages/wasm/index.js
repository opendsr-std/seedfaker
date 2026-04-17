// Bundler entry point (webpack, vite, rspack)
// Uses ./bundler/ wasm-bindgen output (handles .wasm as asset).
// No init() needed: the bundler processes the static .wasm import synchronously.
export { SeedFaker } from "./wrapper.js";
import { _setBackend } from "./wrapper.js";
import { SeedFaker as Inner } from "./bundler/seedfaker_wasm.js";
_setBackend(async () => {}, Inner);
