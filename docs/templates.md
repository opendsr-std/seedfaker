# Templates

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Free-form output with column references, conditionals, and loops. Add `template:` to a config or use `-t` on the command line.

## Contents

- [Syntax](#syntax) — `{{var}}`, conditionals, repeat
- [Inline field specs](#inline-field-specs) — modifiers, ranges, enums inside `{{}}`
- [Single-line](#single-line)
- [Multiline](#multiline)
- [Conditionals](#conditionals)
- [Repeat](#repeat)
- [Serial](#serial) — `{{serial}}` counter
- [Inline templates (CLI)](#inline-templates-cli)

## Syntax

| Syntax                   | Meaning                                                   |
| ------------------------ | --------------------------------------------------------- |
| `{{var}}`                | Column value (declared) or inline field (registry lookup) |
| `{{field:mod:range}}`    | Inline with [field spec syntax](fields.md#field-syntax)   |
| `{{serial}}`             | Built-in: 0-based record counter (always available)       |
| `{{#if var == "val"}}`   | Conditional start                                         |
| `{{#elif var == "val"}}` | Else-if branch                                            |
| `{{else}}`               | Else branch                                               |
| `{{/if}}`                | End conditional                                           |
| `{{#repeat N}}`          | Repeat block N times                                      |
| `{{/repeat}}`            | End repeat                                                |

**Declared columns vs inline generation.** `{{name}}` resolves in order: declared column, then field registry. Declared = stable per record (same everywhere). Inline = fresh value each use.

**Tag-only lines.** Lines containing only block tags and whitespace produce no output.

**Nesting.** `{{#if}}` and `{{#repeat}}` nest to any depth.

## Inline field specs

Any `{{placeholder}}` that is not a declared column is looked up in the field registry. Inline fields accept the same spec syntax as columns and CLI — modifiers, ranges, lengths:

```yaml
template: |
  [{{timestamp:log}}] src={{ip}} pid={{integer:1000..32768}} msg={{base64:short}}
  user={{username}} card={{credit-card:space}} code={{digits:4}}
  date={{date:2020..2025}} amount={{amount:usd}} lat={{float:0..90}}
  level={{enum:INFO=7,WARN=2,ERROR=1}}
```

Each inline field generates a fresh value per occurrence. To reuse a value, declare it in `columns:` and reference by name.

## Single-line

```yaml
template: '{{ip}} - {{user}} [{{ts}}] "{{method}} {{path}} HTTP/1.1" {{status}}'
```

```bash
seedfaker ip user ts=timestamp:log method=http-method path=url status=http-status \
  -t '{{ip}} - {{user}} [{{ts}}] "{{method}} {{path}} HTTP/1.1" {{status}}' -n 5
```

## Multiline

YAML `|` block. Records separated by blank lines.

```yaml
columns:
  from: name
  from_email: email
  to: name
  body: message

template: |
  From: {{from}} <{{from_email}}>
  To: {{to}} <{{email}}>

  {{body}}
  — {{from}}
```

`{{from}}` appears twice — same value both times. `{{email}}` in the `To:` line is inline (not declared in columns) — generates a fresh value.

## Conditionals

`{{#if}}` checks a **declared column** with `==` or `!=`. Inline fields cannot be used in conditions.

```yaml
columns:
  level: enum:INFO=7,WARN=2,ERROR=1
  msg: message
  ip: ip

template: |
  {{#if level == "ERROR"}}
  ALERT: {{msg}} — source: {{ip}}
  {{#elif level == "WARN"}}
  WARN: {{msg}}
  {{else}}
  {{level}}: {{msg}}
  {{/if}}
```

## Repeat

`{{#repeat N}}` repeats a section within a single record (max 1000). Inline fields generate fresh values per occurrence.

```yaml
columns:
  team: company-name
  lead: name

template: |
  Team: {{team}}, Lead: {{lead}}
  {{#repeat 5}}
  - {{name}} <{{email}}>
  {{/repeat}}
```

## Serial

`{{serial}}` — 0-based record counter. Always available, no declaration needed. Not affected by seed.

```bash
seedfaker name -t '{{serial}}: {{name}}' -n 3 --seed demo --until 2025
# 0: Paulina Laca
# 1: Irene Michaelides
# 2: Elvira Castro Gonzalez
```

Do not declare `serial` in `columns:` — it is reserved.

## Limits

- `{{#repeat N}}` — maximum N is **100**
- `{{#if}}` and `{{#repeat}}` can be nested up to **30 levels** deep

## Related guides

- [Reproducible datasets](../guides/reproducible-datasets.md) — template-based deterministic fixtures

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
