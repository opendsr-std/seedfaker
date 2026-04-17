#!/usr/bin/env node
/**
 * seedfaker + Express — synthetic data API endpoint
 *
 * Install:  npm install @opendsr/seedfaker express
 * Run:      node express_example.js
 * Test:     curl 'http://localhost:3000/users?n=5&seed=demo&locale=en'
 * Docs:     https://github.com/opendsr-std/seedfaker
 */

const express = require("express");
const { SeedFaker } = require("@opendsr/seedfaker");
const app = express();

app.get("/users", (req, res) => {
  const n = Math.min(parseInt(req.query.n) || 10, 10000);
  const seed = req.query.seed || `req-${Date.now()}`;
  const f = new SeedFaker({ seed, locale: req.query.locale || "en" });
  res.json(f.records(["name", "email", "phone:e164"], { n, ctx: "strict" }));
});

app.listen(3000, () => console.log("http://localhost:3000/users?n=5&seed=demo"));
