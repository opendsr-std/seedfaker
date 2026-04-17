# Mock API server

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Deterministic HTTP mock: same URL, same response, every run, every machine. Use the library inside a minimal server, or generate a static fixture and serve it with an existing mock tool.

## Contents

- [Express / FastAPI endpoint](#express--fastapi-endpoint)
- [Stable entity by id](#stable-entity-by-id)
- [Deterministic pagination](#deterministic-pagination)
- [Relational mock](#relational-mock)
- [Coherent identities](#coherent-identities)
- [Custom JSON shape](#custom-json-shape)
- [Static fixture file](#static-fixture-file)
- [Error and latency injection](#error-and-latency-injection)

## Express / FastAPI endpoint

```js
// server.js
const express = require("express");
const { SeedFaker } = require("@opendsr/seedfaker");
const app = express();
const sf = new SeedFaker({ seed: "mock", locale: "en", until: "2025" });

app.get("/api/users", (req, res) => {
  const n = Math.min(parseInt(req.query.limit) || 20, 100);
  res.json(sf.records(["name", "email", "phone"], { n, ctx: "strict" }));
});

app.listen(3000);
```

```python
# FastAPI equivalent
from fastapi import FastAPI
from seedfaker import SeedFaker
app = FastAPI()
sf = SeedFaker(seed="mock", locale="en", until="2025")

@app.get("/api/users")
def users(limit: int = 20):
    return sf.records(["name", "email", "phone"], n=min(limit, 100), ctx="strict")
```

Same seed → same response bytes on every restart.

## Stable entity by id

Seed a fresh `SeedFaker` from the entity id. `/api/users/42` always returns the same user.

```js
app.get("/api/users/:id", (req, res) => {
  const f = new SeedFaker({ seed: `user-${req.params.id}`, locale: "en", until: "2025" });
  res.json(f.record(["name", "email", "phone", "address"], { ctx: "strict" }));
});
```

## Deterministic pagination

Seed per page — `GET /api/users?page=5` returns identical rows on every call.

```js
app.get("/api/users", (req, res) => {
  const page = parseInt(req.query.page) || 1;
  const size = Math.min(parseInt(req.query.size) || 20, 100);
  const f = new SeedFaker({ seed: `users-page-${page}`, locale: "en", until: "2025" });
  res.json({ page, data: f.records(["name", "email", "phone"], { n: size, ctx: "strict" }) });
});
```

For global sequential access (row N of the full set), pre-generate a JSONL with the CLI + `--shard` and serve slices from memory.

## Relational mock

Multi-table YAML gives linked entities without a database:

```yaml
# shop.yaml
options: { seed: shop-mock, until: "2025" }

users:
  columns: { id: serial, name: first-name, email: email }
  options: { count: 1000, ctx: strict }

orders:
  columns:
    id: serial
    user_id: users.id:zipf
    user_email: user_id->email
    total: amount:usd:1..5000
    status: enum:pending=30,paid=60,cancelled=10
  options: { count: 5000 }
```

```js
const fs = require("fs");
const { execSync } = require("child_process");
execSync("seedfaker run shop.yaml --all --output-dir ./mock-data --format jsonl");

const users  = fs.readFileSync("mock-data/users.jsonl",  "utf8").trim().split("\n").map(JSON.parse);
const orders = fs.readFileSync("mock-data/orders.jsonl", "utf8").trim().split("\n").map(JSON.parse);

app.get("/api/users/:id/orders", (req, res) => {
  res.json(orders.filter(o => String(o.user_id) === req.params.id));
});
```

`orders[k].user_email` equals the email of the user with `orders[k].user_id`. FK semantics: [docs/multi-table](../docs/multi-table.md).

## Coherent identities

`ctx: "strict"` locks every field in a record to one identity per row — name, email, phone, address, gov ID match. Without it, each field is drawn independently (a Japanese name next to a German phone number). Details: [docs/context](../docs/context.md).

## Custom JSON shape

Inline template:
```bash
seedfaker name email -t '{"user": "{{name}}", "contact": "{{email}}"}' \
  --seed api --until 2025 -n 3
```

Nested via config template: [docs/templates](../docs/templates.md).

## Static fixture file

For `json-server`, MSW, Playwright route mocks:

```bash
seedfaker name email phone role=enum:admin=1,user=9 \
  --format jsonl --seed api --until 2025 -n 100 > users.jsonl
```

Commit the file or regenerate in CI with the same seed. Pin `--fingerprint` to catch algorithm drift.

## Error and latency injection

Deterministic chaos from the URL, independent of seedfaker:

```js
app.get("/api/users", async (req, res) => {
  const hash = s => [...s].reduce((h, c) => ((h << 5) - h) + c.charCodeAt(0), 0);
  if ((hash(req.url) & 0xFF) < 13) return res.status(500).json({ error: "simulated" });
  await new Promise(r => setTimeout(r, (hash(req.url) >> 8) & 0xFF));
  res.json(sf.records(["name", "email"], { n: 20 }));
});
```

For streaming load tests with `--rate` and `--corrupt`: [api-load-testing](api-load-testing.md).

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
