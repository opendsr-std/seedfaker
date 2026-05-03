#!/usr/bin/env node
/**
 * Install:  npm install @opendsr/seedfaker
 * Docs:     https://github.com/opendsr-std/seedfaker
 */

const { SeedFaker } = require("@opendsr/seedfaker");

const f = new SeedFaker({ seed: "demo", locale: "en" });

console.log(f.field("name"));
console.log(f.field("email"));
console.log(f.field("phone", { e164: true }));
console.log(f.field("credit-card", { space: true }));

// ctx=strict → name, email, phone correlated per row
for (const r of f.records(["name", "email", "phone"], { n: 5, ctx: "strict" })) {
  console.log(`${r.name}\t${r.email}\t${r.phone}`);
}

// corrupt=high → noisy values
console.log(f.records(["name", "email", "ssn"], { n: 3, corrupt: "high" }));

const a = new SeedFaker({ seed: "ci" });
const b = new SeedFaker({ seed: "ci" });
if (a.field("name") !== b.field("name")) throw new Error("determinism failed");
