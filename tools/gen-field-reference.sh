#!/usr/bin/env bash
set -eu
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"
OUT="$ROOT/docs/field-reference.md"

test -f "$SF" || { echo "Run 'make build' first."; exit 1; }

FIELD_COUNT=$("$SF" --list 2>&1 | grep -c "^    [a-z]")
GROUP_COUNT=$("$SF" --list 2>&1 | grep -c "^  [a-z]")

{
echo "# Field Reference"
echo ""
echo "200+ fields across $GROUP_COUNT groups. For syntax, ranges, and ordering — see [fields](fields.md)."
echo ""
echo "## Contents"
echo ""
echo "- [Universal modifiers](#universal-modifiers)"
"$SF" --list 2>&1 | grep "^  [a-z]" | sed 's/^ *//' | sed 's/:.*//' | while read -r g; do
  count=$("$SF" --list 2>&1 | sed -n "/^  $g:/,/^  [a-z]/p" | grep -c "^    [a-z]")
  echo "- [$g](#$g) ($count)"
done
echo ""
echo "## Universal modifiers"
echo ""
echo "All fields support these modifiers:"
echo ""
echo "| Modifier | Description | Example |"
echo "|----------|-------------|---------|"
printf '| `:%s` | %s | `%s` |\n' "upper" "Uppercase output" "$("$SF" name:upper --locale en --seed fieldref --until 2025 -n 1 2>/dev/null)"
printf '| `:%s` | %s | `%s` |\n' "lower" "Lowercase output" "$("$SF" name:lower --locale en --seed fieldref --until 2025 -n 1 2>/dev/null)"
printf '| `:%s` | %s | `%s` |\n' "capitalize" "Capitalize first character" "$("$SF" name:capitalize --locale en --seed fieldref --until 2025 -n 1 2>/dev/null)"
echo ""
echo 'Combine with field-specific modifiers: `mac:plain:upper`, `amount:usd:lower`.'
echo ""

PREV_BASE=""
"$SF" --list 2>&1 | while IFS= read -r line; do
  if echo "$line" | grep -q "^  [a-z]"; then
    group=$(echo "$line" | sed 's/^ *//' | sed 's/:.*//')
    echo "## $group"
    echo ""
    echo "| Field | Modifiers | Description | Example |"
    echo "|-------|-----------|-------------|---------|"
    PREV_BASE=""
  fi
  if echo "$line" | grep -q "^    [a-z]"; then
    field=$(echo "$line" | sed 's/^ *//' | awk '{print $1}')
    desc=$(echo "$line" | sed 's/^ *[a-z][^ ]* *//' | sed 's/ *{.*//')
    caps=$(echo "$line" | sed -n 's/.*{\(.*\)}/\1/p')

    # Skip modifier sub-entries (field:mod) — they are generated from the base field's caps list
    if echo "$field" | grep -q ':'; then
      continue
    fi

    caplist=$(echo "$caps" | sed 's/ *,[ ]*/,/g' | sed 's/,/, /g' | sed 's/^ *//;s/ *$//')
    example=$("$SF" "$field" --locale en --seed fieldref --until 2025 -n 1 2>/dev/null | tr '\n' ' ' | sed 's/ *$//' | head -c 70)
    printf '| `%s` | %s | %s | %s |\n' "$field" "$caplist" "$desc" "$example"
    fmods=$(echo "$caps" | tr ',' '\n' | sed 's/^ *//' | grep -v '^range$' | grep -v '^asc/desc$' | grep -v '^N$' | tr '\n' ',' | sed 's/,$//' || true)
    if [ -n "$fmods" ]; then
      echo "$fmods" | tr ',' '\n' | sed 's/^ *//' | while read -r mod; do
        modex=$("$SF" "$field:$mod" --locale en --seed fieldref --until 2025 -n 1 2>/dev/null | tr '\n' ' ' | sed 's/ *$//' | head -c 70)
        printf '| `%s:%s` |  | | %s |\n' "$field" "$mod" "$modex"
      done
    fi
  fi
done
echo ""
echo "[Quick start](quick-start.md) · [Fields](fields.md) · [CLI](cli.md) · [Configs](configs.md) · [Context](context.md) · [Guides](../guides/)"
} > "$OUT"

echo "docs/field-reference.md updated ($(grep -c '^|' "$OUT") rows)."
