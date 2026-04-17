#!/usr/bin/env node
/**
 * seedfaker — deterministic synthetic data generator
 *
 * Install:  npm install @opendsr/seedfaker
 * Docs:     https://github.com/opendsr-std/seedfaker
 */

const { SeedFaker } = require("@opendsr/seedfaker");

// Deterministic: same seed = same output, always
const f = new SeedFaker({ seed: "demo", locale: "en" });

console.log(f.field("name"));
console.log(f.field("email"));
console.log(f.field("phone", { e164: true }));
console.log(f.field("credit-card", { space: true }));

// Correlated records: email derived from name, phone matches locale
const records = f.records(["name", "email", "phone"], {
  n: 5,
  ctx: "strict",
});
records.forEach((r) => console.log(`${r.name}\t${r.email}\t${r.phone}`));

// Noisy data for ML/NER training
const noisy = f.records(["name", "email", "ssn"], {
  n: 3,
  corrupt: "high",
});
console.log(noisy);

// Verify determinism
const a = new SeedFaker({ seed: "ci" });
const b = new SeedFaker({ seed: "ci" });
console.assert(a.field("name") === b.field("name"));
