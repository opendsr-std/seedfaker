# Streaming and load testing

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Generate data streams at controlled rates. Pipe to files, databases, or HTTP endpoints.

> [Guides](README.md) · [Quick start](../docs/quick-start.md) · [Streaming](../docs/streaming.md) · [Corruption](../docs/corruption.md) · [Mock API server](mock-api-server.md)

## Contents

- [Stream to file](#stream-to-file)
- [Rate-limited output](#rate-limited-output)
- [Unlimited stream](#unlimited-stream)
- [Pipe to an HTTP endpoint](#pipe-to-an-http-endpoint)
- [Generate corrupted data](#generate-corrupted-data)

## Stream to file

```bash
seedfaker name email phone --format jsonl --seed bench --until 2025 -n 1000000 > users.jsonl
```

Deterministic — re-run and get the same file.

## Rate-limited output

`--rate N` limits output to N records per second:

```bash
seedfaker run nginx -n 0 --rate 200 --seed traffic --until 2025
```

```
49.117.186.194 - yg0292 [28/Mar/2025:00:00:05 +0000] "POST https://service.prod.net/v3/users HTTP/1.1" 200 448752 ...
```

## Unlimited stream

`-n 0` runs until the pipe closes or Ctrl+C:

```bash
seedfaker name email --format jsonl -n 0 --seed stream --until 2025 | your-consumer
```

## Pipe to an HTTP endpoint

Generate shaped JSON, write to a file, then feed to a load testing tool:

Generate request bodies as a file, then feed to your load testing tool:

```bash
seedfaker name email amount=amount:plain:1..500 \
  -t '{"user":"{{name}}","email":"{{email}}","amount":{{amount}}}' \
  --seed load --until 2025 -n 10000 > requests.jsonl
```

POST individually for small-scale testing:

```bash
seedfaker name email -t '{"user":"{{name}}","email":"{{email}}"}' \
  --seed load --until 2025 -n 5 | while IFS= read -r line; do
  curl -s -X POST -H 'Content-Type: application/json' -d "$line" http://localhost:8080/ingest
done
```

## Generate corrupted data

[`--corrupt`](../docs/corruption.md) produces realistic defects — encoding errors, truncation, field swaps:

```bash
seedfaker name email phone --format jsonl --corrupt high --seed chaos --until 2025 -n 100 > malformed.jsonl
```

Feed to your service to verify error handling.

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
