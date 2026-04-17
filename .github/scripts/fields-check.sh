#!/usr/bin/env bash
# Verify docs/field-reference.md is consistent with the current field registry.
# This is a lightweight check: it compares --list output field names
# against the fields documented in docs/field-reference.md.
# Full regeneration (make fields) is expensive and requires examples,
# so CI checks structural consistency instead.
set -euo pipefail

SF="rust/target/release/seedfaker"

if [ ! -f "$SF" ]; then
  echo "::error::Binary not found at $SF — build must run first"
  exit 1
fi

if [ ! -f "docs/field-reference.md" ]; then
  echo "::error::docs/field-reference.md not found"
  exit 1
fi

EXIT=0

# Extract field names from --list output
LIST_FIELDS=$($SF --list 2>&1 | grep -E '^\s{4}[a-z]' | awk '{print $1}' | sort)
LIST_COUNT=$(echo "$LIST_FIELDS" | wc -l | tr -d ' ')

# Extract field names from docs/field-reference.md (backtick-wrapped in table rows)
DOC_FIELDS=$(grep -oE '^\| `[a-z][a-z0-9_-]*`' docs/field-reference.md | sed 's/| `//;s/`//' | sort -u)
DOC_COUNT=$(echo "$DOC_FIELDS" | wc -l | tr -d ' ')

echo "Fields in --list: $LIST_COUNT"
echo "Fields in docs/field-reference.md: $DOC_COUNT"

# Fields in --list but missing from docs/field-reference.md
MISSING=$(comm -23 <(echo "$LIST_FIELDS") <(echo "$DOC_FIELDS"))
if [ -n "$MISSING" ]; then
  echo "::error::Fields in --list but missing from docs/field-reference.md:"
  echo "$MISSING"
  echo ""
  echo "Run 'make fields' to regenerate docs/field-reference.md"
  EXIT=1
fi

# Fields in docs/field-reference.md but not in --list (stale docs)
EXTRA=$(comm -13 <(echo "$LIST_FIELDS") <(echo "$DOC_FIELDS"))
if [ -n "$EXTRA" ]; then
  echo "::warning::Fields in docs/field-reference.md but not in --list (possibly stale):"
  echo "$EXTRA"
fi

# Check group headers match
LIST_GROUPS=$($SF --list 2>&1 | grep -E '^\s{2}[a-z]' | sed 's/^ *//' | sed 's/:.*//' | sort)
DOC_GROUPS=$(grep -E '^## [a-z]' docs/field-reference.md | sed 's/^## //' | sort)

MISSING_GROUPS=$(comm -23 <(echo "$LIST_GROUPS") <(echo "$DOC_GROUPS"))
if [ -n "$MISSING_GROUPS" ]; then
  echo "::error::Groups in --list but missing from docs/field-reference.md:"
  echo "$MISSING_GROUPS"
  EXIT=1
fi

if [ "$EXIT" -eq 0 ]; then
  echo "docs/field-reference.md is consistent with field registry."
fi

exit $EXIT
