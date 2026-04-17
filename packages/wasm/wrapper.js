// Shared wrapper — clean public API over wasm-bindgen internals.
// Do not import directly. Use index.js (bundler) or web.js (browser).

let _init, _Inner, _ready = false;
let _expectedWasmHash = null;
let _wasmUrl = null;

export function _setBackend(init, Inner, opts = {}) {
  _init = init;
  _Inner = Inner;
  _expectedWasmHash = opts.wasmHash || null;
  _wasmUrl = opts.wasmUrl || null;
}

async function verifyWasmIntegrity() {
  if (!_expectedWasmHash || !_wasmUrl) return;
  if (typeof crypto === "undefined" || !crypto.subtle) return;

  const resp = await fetch(_wasmUrl);
  const bytes = await resp.arrayBuffer();
  const hashBuf = await crypto.subtle.digest("SHA-256", bytes);
  const actual = Array.from(new Uint8Array(hashBuf))
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");

  if (actual !== _expectedWasmHash) {
    throw new Error(
      `seedfaker: WASM integrity check failed. ` +
        `Expected ${_expectedWasmHash.slice(0, 16)}..., ` +
        `got ${actual.slice(0, 16)}... ` +
        `The .wasm file may have been tampered with.`,
    );
  }
}

export class SeedFaker {
  #w;

  constructor(opts = {}) {
    if (!_ready)
      throw new Error("await SeedFaker.init() before creating instances");
    this.#w = new _Inner({
      seed: opts.seed || null,
      locale: opts.locale || null,
      tz: opts.tz || null,
      since: opts.since !== undefined ? opts.since : null,
      until: opts.until !== undefined ? opts.until : null,
    });
  }

  static async init() {
    if (_ready) return;
    await verifyWasmIntegrity();
    await _init();
    _ready = true;
  }

  field(name, opts) {
    return this.#w.field(opts ? buildSpec(name, opts) : name);
  }

  record(fields, opts = {}) {
    return this.#w.record(fields, opts.ctx || null, opts.corrupt || null);
  }

  records(fields, opts = {}) {
    const { n = 1, ctx, corrupt } = opts;
    return this.#w.records(fields, n, ctx || null, corrupt || null);
  }

  validate(fields, opts = {}) {
    _Inner.validate(fields, opts.ctx || null, opts.corrupt || null);
  }

  static fields() {
    return _Inner.fields();
  }

  static fingerprint() {
    return _Inner.fingerprint();
  }

  static buildInfo() {
    return _Inner.build_info();
  }
}

function buildSpec(name, opts) {
  const parts = [name];
  for (const [k, v] of Object.entries(opts)) {
    if (k === "n" || k === "ctx" || k === "corrupt") continue;
    if (v === true) parts.push(k);
    else if (typeof v === "number") parts.push(`${k}=${v}`);
  }
  return parts.join(":");
}
