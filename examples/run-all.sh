#!/usr/bin/env bash
# Run all examples sequentially.
# SEEDFAKER env var controls the binary (set by Makefile or manually).
# Pass --no-docker to skip DB examples (11-postgres, 12-mongo-redis).
set -euo pipefail
cd "$(dirname "$0")"

export SEEDFAKER="${SEEDFAKER:-seedfaker}"
NO_DOCKER="${1:-}"
PASS=0
FAIL=0

for script in 0[1-9]-*.sh 1[0-9]-*.sh 2[0-9]-*.sh; do
    # Skip DB examples (handled separately with docker)
    case "$script" in 11-*|12-*) continue ;; esac
    echo ""
    echo "==== $script ===="
    if /bin/bash "$script"; then
        PASS=$((PASS + 1))
    else
        FAIL=$((FAIL + 1))
        echo "FAILED: $script"
    fi
done

if [ "$NO_DOCKER" != "--no-docker" ]; then
    for script in 11-*.sh 12-*.sh; do
        echo ""
        echo "==== $script ===="
        if /bin/bash "$script"; then
            PASS=$((PASS + 1))
        else
            FAIL=$((FAIL + 1))
            echo "FAILED: $script"
        fi
    done
    docker compose down -v 2>/dev/null || true
fi

echo ""
echo "Results: $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ]
