/**
 * seedfaker — ESM + TypeScript example
 *
 * Run:  npx tsx examples/nodejs/basic.ts
 * Or:   node --loader tsx examples/nodejs/basic.ts
 */

import { SeedFaker } from "@opendsr/seedfaker";

// Deterministic: same seed = same output, always
const f = new SeedFaker({ seed: "demo", locale: "en" });

console.log("=== Single fields ===");
console.log("name:", f.field("name"));
console.log("email:", f.field("email"));
console.log("phone:", f.field("phone", { e164: true }));
console.log("card:", f.field("credit-card", { space: true }));

console.log("\n=== Correlated records (ctx=strict) ===");
const records = f.records(["name", "email", "phone"], {
  n: 5,
  ctx: "strict",
});
for (const r of records) {
  console.log(`${r.name}\t${r.email}\t${r.phone}`);
}

console.log("\n=== Corrupted data (for ML/NER training) ===");
const noisy = f.records(["name", "email", "ssn"], {
  n: 3,
  corrupt: "high",
});
console.log(noisy);

console.log("\n=== Determinism check ===");
const a = new SeedFaker({ seed: "ci" });
const b = new SeedFaker({ seed: "ci" });
const va = a.field("name");
const vb = b.field("name");
console.log(`a=${va} b=${vb} match=${va === vb}`);

console.log("\n=== Static methods ===");
console.log("fields:", SeedFaker.fields().length);
console.log("fingerprint:", SeedFaker.fingerprint());
