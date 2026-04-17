// Plain browser entry point (<script type="module">)
// Uses ./web/ wasm-bindgen output (loads .wasm via fetch)
export { SeedFaker } from "./wrapper.js";
import { _setBackend } from "./wrapper.js";
import init, { SeedFaker as Inner } from "./web/seedfaker_wasm.js";
_setBackend(init, Inner);
