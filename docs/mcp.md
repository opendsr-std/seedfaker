# MCP server

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Model Context Protocol server on stdio. Exposes the full seedfaker surface to AI tools and agents via JSON-RPC 2.0. For programmatic access from application code, use the [language bindings](library.md) (Python, Node.js, Go, PHP, Ruby, WASM) — same fields, same determinism guarantee.

Protocol version: `2025-06-18`.

## Contents

- [Setup](#setup) — Claude Desktop, Cursor, VS Code
- [Protocol](#protocol) — JSON-RPC 2.0, batching, ping
- [Tools](#tools) — `generate`, `run_preset`, `validate`, `replace`, `list_fields`, `list_presets`, `list_locales`, `list_modifiers`, `fingerprint`

## Setup

```bash
seedfaker mcp
```

Use the **absolute path** to the binary in MCP-client configs. GUI clients (Claude Desktop, Cursor) launch outside the shell environment and don't inherit `PATH`, so `"command": "seedfaker"` fails with "command not found". Find the path with `which seedfaker`.

```json
{
  "mcpServers": {
    "seedfaker": {
      "command": "/usr/local/bin/seedfaker",
      "args": ["mcp"]
    }
  }
}
```

## Protocol

- JSON-RPC 2.0 over stdio, line-delimited
- Single request or batched array — both return a matching single object / array
- `initialize`, `notifications/initialized`, `notifications/cancelled`, `tools/list`, `tools/call`, `ping` are supported
- Invalid input → `-32602 Invalid params`; unknown method → `-32601`; internal failure → `-32603`
- `validate` and `replace` surface tool-level failures via `isError: true` in the result; protocol-level errors use JSON-RPC error codes
- No silent fallbacks: invalid `ctx`, `corrupt`, `tz`, `since`, `until`, `abc`, `format`, `shard`, `threads` all produce errors
- `seed` is required on `generate`, `run_preset`, and `replace` — MCP refuses non-deterministic output

## Tools

### `generate`

Generate synthetic records. Full CLI surface: 200+ fields, 68 locales, groups, enums, modifiers, transforms, aggregators, expressions, ranges, templates, corruption, annotated output.

**Required:** `fields`, `seed`.

| Name        | Type       | Description                                                                                                      |
| ----------- | ---------- | ---------------------------------------------------------------------------------------------------------------- |
| `fields`    | `string[]` | Fields, groups, enums, aggregators (`amount:sum`), expressions (`total=price*qty`). See `list_fields`.           |
| `n`         | integer    | Record count, `1..=10_000_000_000`. Default: `5`.                                                                |
| `seed`      | string     | Deterministic seed. Required.                                                                                    |
| `locale`    | string     | Comma-separated locales, optionally weighted (`en=7,de=2`). Default: all.                                        |
| `ctx`       | string     | `strict` or `loose`. Lock identity per record.                                                                   |
| `corrupt`   | string     | `low`, `mid`, `high`, or `extreme`.                                                                              |
| `abc`       | string     | `native` or `mixed`. Script selection for non-Latin locales.                                                     |
| `tz`        | string     | Timezone offset (`+03:00`, `-08:00`, `Z`).                                                                       |
| `since`     | string/int | Temporal range start. String (`2020`, `2020-01-15`) or integer epoch seconds.                                    |
| `until`     | string/int | Temporal range end, exclusive. Same types as `since`.                                                            |
| `format`    | string     | `csv`, `tsv`, `jsonl`, `sql=TABLE`. Default: tsv.                                                                |
| `delim`     | string     | Field delimiter for default/tsv (supports `\t`, `\n` escapes).                                                   |
| `no_header` | boolean    | Omit column header row.                                                                                          |
| `annotated` | boolean    | JSONL with text + byte-offset spans for every generated value (NER/PII training).                                |
| `template`  | string     | Inline template with `{{field}}`, `{{serial}}`, `{{#if}}`, `{{#repeat}}`. Mutually exclusive with `format`.      |
| `shard`     | string     | Generate shard `I/N` (e.g. `0/4`); slice of the unsharded run, byte-identical. Requires `n > 0`.                 |
| `threads`   | integer    | In-process parallel workers (default `1`). Output is byte-identical. Requires `n > 0`; ignored with aggregators. |

**Returns:** text output in the selected format.

### `run_preset`

Run a built-in preset or config file path. All `generate` options apply as overrides on top of the config.

**Required:** `preset`, `seed`.

Additional parameters:

| Name      | Type    | Description                                                     |
| --------- | ------- | --------------------------------------------------------------- |
| `preset`  | string  | Preset name or config file path. Use `list_presets`.            |
| `n`       | integer | Override config's count. Default: config count, or `1`.         |
| `table`   | string  | For multi-table configs: which table to generate.               |
| `shard`   | string  | Same semantics as `generate.shard`. Requires effective `n > 0`. |
| `threads` | integer | Same semantics as `generate.threads`.                           |

**Returns:** text output from the preset.

### `validate`

Validate field specs and options without generating data.

| Name                                                       | Type       | Description                                     |
| ---------------------------------------------------------- | ---------- | ----------------------------------------------- |
| `fields`                                                   | `string[]` | Field specs (same syntax as `generate`).        |
| `template`                                                 | string     | Optional template to validate alongside fields. |
| `ctx`, `corrupt`, `format`, `tz`, `since`, `until`, `seed` | —          | Same as `generate`, used by `CheckCtx` rules.   |

**Returns:** `"ok"` or a multiline errors/warnings report. `isError: true` when there are errors.

### `replace`

Replace named columns in a CSV or JSONL stream with synthetic values, preserving referential integrity: same input value + same seed always yields the same replacement.

**Required:** `columns`, `input`, `seed`.

| Name           | Type       | Description                                                       |
| -------------- | ---------- | ----------------------------------------------------------------- |
| `columns`      | `string[]` | Column names to replace (no modifiers).                           |
| `input`        | string     | CSV (with header row) or JSONL data.                              |
| `input_format` | string     | `csv` or `jsonl`. Auto-detected from the first byte when omitted. |
| `seed`         | string     | Deterministic seed.                                               |
| `since`        | string/int | Temporal range start for date/timestamp columns.                  |
| `until`        | string/int | Temporal range end, exclusive, for date/timestamp columns.        |

**Returns:** the rewritten stream as text in the same format as the input.

### `list_fields`

List all fields, groups, modifiers, transforms, locales, presets, and corrupt levels.

**Parameters:** none.

**Returns:** JSON object — `groups`, `transforms`, `total_fields`, `locales`, `presets`, `corrupt_levels`.

### `list_presets`

List built-in preset names, one per line.

### `list_locales`

List supported locale codes, one per line.

### `list_modifiers`

List per-field modifiers and global transforms as a JSON object.

### `fingerprint`

Return the algorithm fingerprint (`sf0-<hex>`). Changes when seeded output would change.

## Related guides

- [MCP for AI agents](../guides/mcp-ai-agents.md) — Claude Desktop / Cursor / VS Code setup

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
