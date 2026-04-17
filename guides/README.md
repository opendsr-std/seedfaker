# Guides

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

End-to-end workflows. Each guide is self-contained and includes runnable commands. For feature reference — flags, field syntax, config schema — see [docs/](../docs/).

## By use case

| Guide | Persona | Scope |
| --- | --- | --- |
| [Seed a database](seed-database.md) | Backend dev / DBA | Postgres/MySQL staging DB up to ~5 GB with multi-table FK |
| [Seed a large database](seed-large-database.md) | Perf engineer | GB/TB bulk load — parallel COPY, UNLOGGED, tuning, benchmark |
| [Distributed generation](distributed-generation.md) | Data engineer | Multi-host sharded generation that recombines bit-identically |
| [Anonymise production data](anonymize-data.md) | Privacy / data eng | `replace` on CSV/JSONL, FK integrity across files |
| [Training and evaluation datasets](training-data.md) | ML / LLM engineer | NER/PII, LLM fine-tuning, eval ground truth, red-team, multi-turn, RAG |
| [Reproducible datasets](reproducible-datasets.md) | Dev / QA | Deterministic fixtures, CI, fingerprint guard |
| [Library usage](library-usage.md) | SDK integrator | Python / Node.js / Go / PHP / Ruby / WASM API patterns |
| [Mock API server](mock-api-server.md) | Backend / FE | Express / FastAPI endpoint with synthetic data |
| [API load testing](api-load-testing.md) | Backend / SRE | Rate-limited streaming, corruption for chaos tests |
| [MCP for AI agents](mcp-ai-agents.md) | AI tooling | Claude / Cursor / VS Code integration |

## Reference

For flag-level or syntax-level details, jump to [docs/](../docs/):

|                           |                                                                                           |
| ------------------------- | ----------------------------------------------------------------------------------------- |
| Field syntax              | [fields](../docs/fields.md) · [field reference](../docs/field-reference.md)               |
| YAML configs              | [configs](../docs/configs.md) · [multi-table](../docs/multi-table.md) · [expressions](../docs/expressions.md) |
| Output formats            | [templates](../docs/templates.md) · [annotated](../docs/annotated.md) · [streaming](../docs/streaming.md) |
| Semantics                 | [determinism](../docs/determinism.md) · [context](../docs/context.md) · [corruption](../docs/corruption.md) |
| Library APIs              | [library](../docs/library.md) (Python / Node / Go / PHP / Ruby / WASM) · [MCP](../docs/mcp.md) |
| Replace (anonymisation)   | [replace](../docs/replace.md)                                                             |

## Per-language packages

See each package's local README for install notes, minimal examples, and language-specific idioms:

- [packages/pip](../packages/pip/) — Python
- [packages/npm](../packages/npm/) — Node.js
- [packages/go](../packages/go/) — Go
- [packages/php](../packages/php/) — PHP
- [packages/ruby](../packages/ruby/) — Ruby
- [packages/wasm](../packages/wasm/) — Browser (WASM)
- [packages/npm-cli](../packages/npm-cli/) — CLI via npm

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
