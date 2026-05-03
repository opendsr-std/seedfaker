/**
 * Run:  npx tsx examples/nodejs/basic.ts
 */

import { SeedFaker } from "@opendsr/seedfaker";

const f = new SeedFaker({ seed: "demo", locale: "en" });

console.log("=== fields ===");
console.log("name:", f.field("name"));
console.log("email:", f.field("email"));
console.log("phone:", f.field("phone", { e164: true }));
console.log("card:", f.field("credit-card", { space: true }));

console.log("\n=== ctx=strict ===");
for (const r of f.records(["name", "email", "phone"], { n: 5, ctx: "strict" })) {
  console.log(`${r.name}\t${r.email}\t${r.phone}`);
}

console.log("\n=== corrupt=high ===");
console.log(f.records(["name", "email", "ssn"], { n: 3, corrupt: "high" }));

console.log("\n=== determinism ===");
const a = new SeedFaker({ seed: "ci" });
const b = new SeedFaker({ seed: "ci" });
if (a.field("name") !== b.field("name")) throw new Error("determinism failed");
console.log("ok");

console.log("\n=== static ===");
console.log("fields:", SeedFaker.fields().length);
console.log("fingerprint:", SeedFaker.fingerprint());
