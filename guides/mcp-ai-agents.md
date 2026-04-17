# MCP server for AI tools

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

seedfaker exposes an MCP server. AI coding assistants call it to generate test data instead of inventing fake values.

> [Guides](README.md) · [MCP reference](../docs/mcp.md) · [Fields](../docs/fields.md)

## Contents

- [When this is useful](#when-this-is-useful)
- [Setup](#setup)
- [Available tools](#available-tools)
- [What it looks like](#what-it-looks-like)

## When this is useful

You're working in Claude Code or Cursor and need test data. Without MCP, the AI invents `john@example.com`. With seedfaker MCP, it generates typed values from 200+ fields — emails that look like emails, phones with country codes, Luhn-valid credit cards.

## Setup

### Claude Desktop

`~/Library/Application Support/Claude/claude_desktop_config.json`:

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

### Cursor

`.cursor/mcp.json` in project root:

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

## Available tools

| Tool | What |
|------|------|
| `field` | Generate 1–100 records with any combination of fields, seed, locale, ctx, corruption |
| `run_preset` | Run a preset (nginx, payment, pii-leak, etc.) |
| `list_fields` | List all fields with groups and modifiers |
| `fingerprint` | Algorithm version hash |

See [MCP reference](../docs/mcp.md) for full parameter details.

## What it looks like

You ask: "generate 5 test users with name, email, phone, locale en, seed test"

The agent calls `field` → gets:

```
name             email                       phone
Janet Marsh      janet.marsh@inbox.com       +1 (957) 226-4272
Emma Hines       hinesy2@caltech.edu         (779) 640-3402
Amy Schwartz     aschwartzl@yahoo.com        +1-566-391-4136
...
```

Same seed = same output.

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
