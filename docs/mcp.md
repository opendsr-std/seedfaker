# MCP server

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Model Context Protocol server on stdio. Exposes synthetic data generation to AI tools and agents via JSON-RPC. For programmatic access from application code, use the [language bindings](library.md) (Python, Node.js, Go, PHP, Ruby, WASM) — same fields, same determinism guarantee.

## Contents

- [Setup](#setup) — Claude Desktop, Cursor, VS Code
- [Tools](#tools) — field, run_preset, list_fields, fingerprint

## Setup

```bash
seedfaker mcp
```

Configure in your MCP client:

```json
{
  "mcpServers": {
    "seedfaker": {
      "command": "seedfaker",
      "args": ["mcp"]
    }
  }
}
```

## Tools

### `field`

Generate synthetic data records.

**Parameters:**

| Name      | Type     | Required | Description                                                                               |
| --------- | -------- | -------- | ----------------------------------------------------------------------------------------- |
| `fields`  | string[] | Yes      | Field names, groups, or enums (`"name"`, `"phone:e164"`, `"person"`, `"enum:admin,user"`) |
| `n`       | integer  | No       | Record count, 1–100 [default: 5]                                                          |
| `seed`    | string   | No       | Deterministic seed                                                                        |
| `locale`  | string   | No       | Comma-separated locales [default: all]                                                    |
| `ctx`     | string   | No       | `strict` or `loose`                                                                       |
| `corrupt` | string   | No       | `low`, `mid`, `high`, `extreme`                                                           |
| `tz`      | string   | No       | Timezone offset (`+0300`, `-08:00`, `Z`)                                                  |
| `since`   | integer  | No       | Start year for dates [default: 1900]                                                      |
| `until`   | integer  | No       | Temporal range end as epoch seconds [default: now]                                        |

**Returns:** JSON array of record objects.

`serial` is supported as a field name — returns the 0-based record counter.

### `run_preset`

Run a preset config.

**Parameters:**

| Name     | Type    | Required | Description                                       |
| -------- | ------- | -------- | ------------------------------------------------- |
| `preset` | string  | Yes      | Preset name. See [presets](presets.md) for all 13 |
| `n`      | integer | No       | Record count, 1–100 [default: 5]                  |
| `seed`   | string  | No       | Deterministic seed                                |

**Returns:** text output from the preset template.

### `list_fields`

List all available fields, groups, modifiers, transforms, and locales.

**Parameters:** none.

**Returns:** JSON object with:

- `groups` — field groups with their fields and modifiers
- `transforms` — available transforms (`upper`, `lower`, `capitalize`)
- `total_fields` — total field count
- `locales` — available locale codes

### `fingerprint`

Return the algorithm fingerprint. Changes when seeded output would change.

**Parameters:** none.

**Returns:** fingerprint string (e.g. `sf0-23a34158542da43a`).

## Related guides

- [MCP for AI agents](../guides/mcp-ai-agents.md) — Claude Desktop / Cursor / VS Code setup

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
