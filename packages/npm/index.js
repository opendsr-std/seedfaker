const path = require("path");
const crypto = require("crypto");
const fs = require("fs");

const PLATFORMS = {
  "darwin-arm64": "@opendsr/seedfaker-darwin-arm64",
  "darwin-x64": "@opendsr/seedfaker-darwin-x64",
  "linux-x64": "@opendsr/seedfaker-linux-x64",
  "linux-arm64": "@opendsr/seedfaker-linux-arm64",
  "win32-x64": "@opendsr/seedfaker-win32-x64",
};

// @checksums-start
// CI replaces this block with real SHA256 hashes before publish.
// Empty = dev (running from source). Filled = production (verify mandatory).
const CHECKSUMS = {};
// @checksums-end

const _isProduction = Object.keys(CHECKSUMS).length > 0;

let _NativeFaker;
let _modulePath;

if (_isProduction) {
  // Production: load from platform package, verify SHA256, no exceptions.
  const key = `${process.platform}-${process.arch === "arm64" ? "arm64" : "x64"}`;
  const pkg = PLATFORMS[key];
  if (!pkg) throw new Error(`seedfaker: unsupported platform ${key}`);
  const pkgDir = path.dirname(require.resolve(`${pkg}/package.json`));
  _modulePath = path.join(pkgDir, "seedfaker_napi.node");

  const expectedHash = CHECKSUMS[key];
  if (!expectedHash) {
    throw new Error(`seedfaker: no checksum for platform ${key}`);
  }

  const actual = crypto.createHash("sha256").update(fs.readFileSync(_modulePath)).digest("hex");
  if (actual !== expectedHash) {
    throw new Error(
      "seedfaker: native module integrity check failed. " +
        `Expected ${expectedHash.slice(0, 16)}..., got ${actual.slice(0, 16)}... ` +
        "Reinstall the package or verify your installation.",
    );
  }

  _NativeFaker = require(_modulePath).SeedFaker;
} else {
  // Dev: CHECKSUMS empty = running from source. Local .node only.
  _modulePath = path.join(__dirname, "seedfaker_napi.node");
  try {
    _NativeFaker = require(_modulePath).SeedFaker;
  } catch (e) {
    throw new Error(`seedfaker: cannot load native module at ${_modulePath}: ${e.message}`);
  }
}

// { e164: true, omit: 50 } → "name:e164:omit=50"
function buildSpec(name, opts) {
  if (!opts) return name;
  const parts = [name];
  for (const [k, v] of Object.entries(opts)) {
    if (k === "n" || k === "field") continue;
    if (k === "range" && Array.isArray(v)) {
      parts.push(`${v[0]}..${v[1]}`);
    } else if (v === true) {
      parts.push(k);
    } else if (typeof v === "number") {
      parts.push(`${k}=${v}`);
    }
  }
  return parts.join(":");
}

class SeedFaker {
  constructor(opts = {}) {
    this._native = new _NativeFaker(
      opts.seed || null,
      opts.locale || null,
      opts.tz || null,
      opts.since !== undefined ? opts.since : null,
      opts.until !== undefined ? opts.until : null,
    );
  }

  field(name, opts) {
    return this._native.field(buildSpec(name, opts));
  }

  records(fields, opts = {}) {
    const { n = 1, ctx, corrupt } = opts;
    const raw = this._native.records(fields, n, ctx || null, corrupt || null);
    return raw;
  }

  record(fields, opts = {}) {
    const { ctx, corrupt } = opts;
    return this._native.record(fields, ctx || null, corrupt || null);
  }

  validate(fields, opts = {}) {
    const { ctx, corrupt } = opts;
    this._native.validate(fields, ctx || null, corrupt || null);
  }

  static fields() {
    return [...FIELDS];
  }

  static fingerprint() {
    return _NativeFaker.fingerprint();
  }
}

// @generated-start
const FIELDS = [
  "integer",
  "float",
  "boolean",
  "digit",
  "bit",
  "trit",
  "enum",
  "serial",
  "letter",
  "trigram",
  "digits",
  "letters",
  "alnum",
  "base64",
  "hex",
  "word",
  "message",
  "emoji",
  "color",
  "uuid",
  "bz",
  "dice",
  "excuse",
  "mball",
  "timestamp",
  "date",
  "name",
  "first-name",
  "last-name",
  "middle-name",
  "birthdate",
  "age",
  "gender",
  "username",
  "login-name",
  "social-handle",
  "nickname",
  "biometric-id",
  "student-id",
  "email",
  "phone",
  "address",
  "street-address",
  "city",
  "state",
  "postal-code",
  "country",
  "latitude",
  "longitude",
  "country-code",
  "phone-code",
  "language-code",
  "locale-code",
  "timezone",
  "credit-card",
  "cvv",
  "iban",
  "swift-bic",
  "routing-number",
  "bank-account",
  "tax-id",
  "amount",
  "currency-code",
  "currency-symbol",
  "password",
  "jwt",
  "bearer-token",
  "api-key",
  "totp-secret",
  "oauth-client-secret",
  "aws-access-key",
  "aws-secret-key",
  "stripe-key",
  "github-pat",
  "gitlab-token",
  "openai-key",
  "sendgrid-key",
  "twilio-sid",
  "twilio-token",
  "slack-bot-token",
  "slack-user-token",
  "datadog-key",
  "sentry-dsn",
  "vault-token",
  "npm-token",
  "vercel-token",
  "supabase-key",
  "telegram-token",
  "discord-webhook",
  "gcp-key",
  "azure-key",
  "cloudflare-token",
  "pagerduty-key",
  "newrelic-key",
  "splunk-token",
  "heroku-key",
  "firebase-key",
  "ssh-private-key",
  "ssh-public-key",
  "connection-string",
  "anthropic-key",
  "session-id",
  "passkey-id",
  "facebook-token",
  "google-token",
  "apple-token",
  "refresh-token",
  "csrf-token",
  "basic-auth",
  "ssn",
  "passport",
  "drivers-license",
  "national-id",
  "cpf",
  "sin",
  "tfn",
  "nino",
  "nhs-number",
  "nir",
  "codice-fiscale",
  "dni",
  "nie",
  "bsn",
  "personnummer",
  "steuer-id",
  "cuil",
  "jmbg",
  "tc-kimlik",
  "pesel",
  "curp",
  "rut",
  "inn",
  "ipn",
  "abn",
  "cnpj",
  "oib",
  "amka",
  "rodne-cislo",
  "szemelyi-szam",
  "hetu",
  "cpr",
  "fodselsnummer",
  "pps",
  "emso",
  "egn",
  "idnp",
  "health-card",
  "cedula",
  "aadhaar",
  "pan",
  "cccd",
  "shenfenzheng",
  "ip",
  "ipv6",
  "mac",
  "url",
  "auth-url",
  "internal-url",
  "dns-record",
  "browser-cookie",
  "user-agent",
  "mime-type",
  "http-method",
  "http-status",
  "port",
  "latency",
  "image-url",
  "twitter-url",
  "linkedin-url",
  "facebook-url",
  "instagram-url",
  "github-url",
  "telegram-url",
  "youtube-url",
  "webhook-url",
  "btc-address",
  "eth-address",
  "sol-address",
  "tx-hash",
  "pgp-fingerprint",
  "company-name",
  "ein",
  "vat-number",
  "duns",
  "lei",
  "job-title",
  "ldap-dn",
  "employee-id",
  "court-case",
  "mrn",
  "npi",
  "insurance-id",
  "medicare-id",
  "icd-10",
  "cpt-code",
  "ndc",
  "rx-number",
  "project-code",
  "jira-id",
  "github-issue",
  "commit-hash",
  "semver",
  "docker-image",
  "slack-channel",
  "sentry-issue",
  "pagerduty-incident",
  "file-path",
  "s3-path",
  "env-var",
  "vin",
  "license-plate",
  "imei",
  "imsi",
  "device-id",
];
// @generated-end

module.exports = { SeedFaker };
