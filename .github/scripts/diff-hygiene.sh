#!/usr/bin/env bash
# Diff hygiene check for pull requests.
# Three-tier size policy: informational, strong warning, hard failure.
# Detects accidental large files, debug leftovers, and generated artifacts.
set -euo pipefail

WARN=${DIFF_WARN_LINES:-400}
STRONG_WARN=${DIFF_STRONG_WARN_LINES:-800}
FAIL=${DIFF_FAIL_LINES:-1500}
FILE_SIZE=${DIFF_FILE_SIZE_LIMIT:-524288}

BASE="${GITHUB_BASE_REF:-main}"
HEAD="${GITHUB_SHA:-HEAD}"

# Ensure base branch is available
git fetch origin "$BASE" --depth=1 2>/dev/null || true

TOTAL=$(git diff --numstat "origin/$BASE...$HEAD" | awk '{s+=$1+$2} END {print s+0}')
FILE_COUNT=$(git diff --name-only "origin/$BASE...$HEAD" | wc -l | tr -d ' ')

echo "Diff: $TOTAL lines changed across $FILE_COUNT files"

EXIT=0

# --- Size check (three tiers) ---
if [ "$TOTAL" -gt "$FAIL" ]; then
  echo "::error::Diff exceeds $FAIL lines ($TOTAL). Split into smaller PRs."
  EXIT=1
elif [ "$TOTAL" -gt "$STRONG_WARN" ]; then
  echo "::warning::Large diff: $TOTAL lines (threshold: $STRONG_WARN). Strongly consider splitting."
elif [ "$TOTAL" -gt "$WARN" ]; then
  echo "::notice::Medium diff: $TOTAL lines. Consider whether this can be split."
fi

# --- Large individual files ---
LARGE_FILES=$(git diff --name-only "origin/$BASE...$HEAD" | while read -r f; do
  if [ -f "$f" ]; then
    SIZE=$(wc -c < "$f" | tr -d ' ')
    if [ "$SIZE" -gt "$FILE_SIZE" ]; then
      echo "$f ($SIZE bytes)"
    fi
  fi
done)

if [ -n "$LARGE_FILES" ]; then
  echo "::error::Files larger than $((FILE_SIZE / 1024))KB detected:"
  echo "$LARGE_FILES"
  EXIT=1
fi

# --- Debug / temporary files ---
SUSPECT=$(git diff --name-only "origin/$BASE...$HEAD" | grep -iE '\.(bak|tmp|orig|swp|swo)$|\.DS_Store|Thumbs\.db|\.idea/|\.vscode/' || true)
if [ -n "$SUSPECT" ]; then
  echo "::error::Temporary or editor files in diff:"
  echo "$SUSPECT"
  EXIT=1
fi

# --- Compiled / binary artifacts ---
ARTIFACTS=$(git diff --name-only "origin/$BASE...$HEAD" | grep -iE '\.(so|dylib|dll|node|exe|whl|tar\.gz|zip)$' || true)
if [ -n "$ARTIFACTS" ]; then
  echo "::error::Binary/compiled artifacts in diff:"
  echo "$ARTIFACTS"
  EXIT=1
fi

# --- Vendored / blob detection ---
BLOBS=$(git diff --name-only "origin/$BASE...$HEAD" | grep -iE '(vendor/|vendored/|\.min\.js$|\.min\.css$)' || true)
if [ -n "$BLOBS" ]; then
  echo "::warning::Possible vendored files in diff:"
  echo "$BLOBS"
fi

exit $EXIT
