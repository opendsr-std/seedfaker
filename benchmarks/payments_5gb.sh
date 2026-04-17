#!/usr/bin/env bash
# Super-fast bulk load benchmark: seedfaker (host) → Postgres (docker).
#
# Streams a 10-table synthetic payment-system dataset into a Postgres 17
# container tuned for write throughput. Pure heap append: UNLOGGED tables,
# no PK, no FK, no indexes, no CHECKs, no triggers. Uniqueness is guaranteed
# by seedfaker's deterministic generation — no server-side validation.
#
# Base dataset ≈ 100 MB of CSV. --scale N multiplies row counts.
#
# Usage:
#   ./payments_5gb.sh              # ~100 MB, wipe → run → leave PG up
#   ./payments_5gb.sh --scale 50   # ~5 GB
#   ./payments_5gb.sh --jobs 4     # max concurrent seedfaker→psql pipelines (default 2)
#   ./payments_5gb.sh --shards 3   # N-way shard of transactions / authorizations /
#                                  # ledger_entries (default 1 = no sharding)
#   ./payments_5gb.sh --down       # tear down after run
#   ./payments_5gb.sh --cleanup    # wipe and exit
#   ./payments_5gb.sh --port N
#   ./payments_5gb.sh --help

set -euo pipefail

SCALE=1
JOBS=2
SHARDS=3
PORT=55432
TEARDOWN=0
CLEANUP_ONLY=0
CONTAINER=payments_bench_pg
WORKDIR="$(cd "$(dirname "$0")" && pwd)/.payments_5gb_out"

while [[ $# -gt 0 ]]; do
  case $1 in
    --scale)   SCALE=$2; shift 2 ;;
    --jobs)    JOBS=$2; shift 2 ;;
    --shards)  SHARDS=$2; shift 2 ;;
    --port)    PORT=$2; shift 2 ;;
    --down)    TEARDOWN=1; shift ;;
    --cleanup) CLEANUP_ONLY=1; shift ;;
    --help|-h) sed -n '2,19p' "$0"; exit 0 ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

wipe() {
  echo "# wipe"
  if [[ -f "$WORKDIR/docker-compose.yml" ]]; then
    (cd "$WORKDIR" && docker compose down -v --remove-orphans >/dev/null 2>&1 || true)
  fi
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
  rm -rf "$WORKDIR"
}
[[ $CLEANUP_ONLY -eq 1 ]] && { wipe; exit 0; }
wipe

for cmd in seedfaker docker psql; do
  command -v "$cmd" >/dev/null || { echo "missing: $cmd" >&2; exit 1; }
done
docker compose version >/dev/null 2>&1 || { echo "missing: docker compose v2" >&2; exit 1; }

mkdir -p "$WORKDIR"
cd "$WORKDIR"

# Scale=1 ≈ 100 MB CSV. Parallel indexed arrays for bash 3.2 (macOS default).
TABLES=(merchants customers accounts cards transactions authorizations refunds disputes payouts ledger_entries)
BASE=(200 20000 30000 40000 240000 240000 10000 1000 4000 360000)
for i in "${!TABLES[@]}"; do
  eval "ROWS_${TABLES[$i]}=$(( BASE[$i] * SCALE ))"
done

cat > payments.yaml <<YAML
options:
  seed: bench-2026
  since: "2024-01-01"
  until: "2026-01-01"
  locale: [en, de, fr, es, ja]

merchants:
  columns:
    id: serial
    name: company-name
    country: country-code
    mcc: integer:3000..9999
    created_at: timestamp:asc
  options: { count: ${ROWS_merchants} }

customers:
  columns:
    id: uuid
    first_name: first-name
    last_name: last-name
    email: email
    phone: phone:e164:omit=15
    country: country-code
    created_at: timestamp:asc
  options: { count: ${ROWS_customers}, ctx: strict }

accounts:
  columns:
    id: serial
    customer_id: customers.id:zipf
    customer_email: customer_id->email
    iban: iban:plain
    currency: currency-code
    opened_at: timestamp:asc
  options: { count: ${ROWS_accounts} }

cards:
  columns:
    id: serial
    account_id: accounts.id:zipf
    pan: credit-card:plain
    last4: integer:1000..9999
    network: enum:visa=45,mastercard=40,amex=10,discover=5
    expires_at: timestamp
  options: { count: ${ROWS_cards} }

transactions:
  columns:
    id: uuid
    card_id: cards.id:zipf
    merchant_id: merchants.id:zipf
    amount: amount:plain:1..5000
    currency: currency-code
    status: enum:captured=70,authorized=20,failed=8,voided=2
    created_at: timestamp:asc
  options: { count: ${ROWS_transactions} }

authorizations:
  columns:
    id: serial
    transaction_id: transactions.id
    auth_code: integer:100000..999999
    response_code: enum:approved=85,declined=10,referral=3,error=2
    processor: enum:stripe=40,adyen=25,worldpay=15,braintree=10,checkout=10
    created_at: timestamp:asc
  options: { count: ${ROWS_authorizations} }

refunds:
  columns:
    id: serial
    transaction_id: transactions.id:zipf
    amount: amount:plain:1..5000
    reason: enum:customer_request=55,fraud=10,duplicate=15,product_not_received=20
    created_at: timestamp:asc
  options: { count: ${ROWS_refunds} }

disputes:
  columns:
    id: uuid
    transaction_id: transactions.id:zipf
    reason_code: enum:fraudulent=35,product_not_received=25,product_unacceptable=20,credit_not_processed=10,general=10
    status: enum:open=30,under_review=25,won=25,lost=20
    opened_at: timestamp:asc
  options: { count: ${ROWS_disputes} }

payouts:
  columns:
    id: uuid
    merchant_id: merchants.id:zipf
    amount: amount:plain:100..500000
    currency: currency-code
    status: enum:paid=80,pending=15,failed=5
    created_at: timestamp:asc
  options: { count: ${ROWS_payouts} }

ledger_entries:
  columns:
    id: serial
    account_id: accounts.id:zipf
    direction: enum:debit=50,credit=50
    amount: amount:plain:1..5000
    currency: currency-code
    posted_at: timestamp:asc
  options: { count: ${ROWS_ledger_entries} }
YAML

cat > docker-compose.yml <<YAML
services:
  pg:
    image: postgres:17
    container_name: $CONTAINER
    environment:
      POSTGRES_DB: payments_bench
      POSTGRES_USER: bench
      POSTGRES_PASSWORD: bench
    ports: ["$PORT:5432"]
    tmpfs: ["/var/lib/postgresql/data"]
    command: >
      postgres
      -c shared_buffers=2GB
      -c maintenance_work_mem=2GB
      -c work_mem=256MB
      -c max_wal_size=16GB
      -c min_wal_size=4GB
      -c checkpoint_timeout=60min
      -c synchronous_commit=off
      -c fsync=off
      -c full_page_writes=off
      -c wal_level=minimal
      -c max_wal_senders=0
      -c max_parallel_workers=8
YAML

cat > schema.sql <<'SQL'
-- Heap-only: no PK, no FK, no UNIQUE, no CHECK, no indexes, no triggers.
-- Every COPY row is pure heap append. Uniqueness of ids is guaranteed by
-- seedfaker (deterministic serial/uuid generation), not by Postgres.
CREATE UNLOGGED TABLE merchants       (id BIGINT NOT NULL, name TEXT NOT NULL, country TEXT NOT NULL, mcc INTEGER NOT NULL, created_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE customers       (id UUID NOT NULL, first_name TEXT NOT NULL, last_name TEXT NOT NULL, email TEXT NOT NULL, phone TEXT, country TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE accounts        (id BIGINT NOT NULL, customer_id UUID NOT NULL, customer_email TEXT NOT NULL, iban TEXT NOT NULL, currency TEXT NOT NULL, opened_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE cards           (id BIGINT NOT NULL, account_id BIGINT NOT NULL, pan TEXT NOT NULL, last4 INTEGER NOT NULL, network TEXT NOT NULL, expires_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE transactions    (id UUID NOT NULL, card_id BIGINT NOT NULL, merchant_id BIGINT NOT NULL, amount NUMERIC(12,2) NOT NULL, currency TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE authorizations  (id BIGINT NOT NULL, transaction_id UUID NOT NULL, auth_code INTEGER NOT NULL, response_code TEXT NOT NULL, processor TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE refunds         (id BIGINT NOT NULL, transaction_id UUID NOT NULL, amount NUMERIC(12,2) NOT NULL, reason TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE disputes        (id UUID NOT NULL, transaction_id UUID NOT NULL, reason_code TEXT NOT NULL, status TEXT NOT NULL, opened_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE payouts         (id UUID NOT NULL, merchant_id BIGINT NOT NULL, amount NUMERIC(14,2) NOT NULL, currency TEXT NOT NULL, status TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL);
CREATE UNLOGGED TABLE ledger_entries  (id BIGINT NOT NULL, account_id BIGINT NOT NULL, direction TEXT NOT NULL, amount NUMERIC(12,2) NOT NULL, currency TEXT NOT NULL, posted_at TIMESTAMPTZ NOT NULL);
SQL

PGURL="postgresql://bench:bench@localhost:${PORT}/payments_bench"
COMPOSE="docker compose -f docker-compose.yml"

trap 'rc=$?; [[ $rc -ne 0 ]] && echo "# failed (rc=$rc); cleanup: $0 --cleanup" >&2' EXIT

# --- system info ---
CPUS=$(getconf _NPROCESSORS_ONLN 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo '?')
if [[ -r /proc/meminfo ]]; then
  MEM_GB=$(awk '/MemTotal/ {printf "%.1f", $2/1024/1024}' /proc/meminfo)
else
  MEM_GB=$(sysctl -n hw.memsize 2>/dev/null | awk '{printf "%.1f", $1/1073741824}')
fi

echo "# ----- system -----"
echo "#   cpus:       $CPUS"
echo "#   memory:     ${MEM_GB} GB"
echo "#   seedfaker:  $(seedfaker --version)"
echo "#   docker:     $(docker --version | awk '{print $3}' | tr -d ,)"
echo "#   scale:      ${SCALE}x  (~$(( 100 * SCALE )) MB CSV)"
echo "#   jobs:       $JOBS parallel loaders"
echo "#   shards:     $SHARDS per big table (transactions, authorizations, ledger_entries)"

echo "# ----- dataset -----"
total_rows=0
for t in "${TABLES[@]}"; do
  v="ROWS_$t"; r="${!v}"
  printf "#   %-16s %'12d rows\n" "$t" "$r"
  total_rows=$(( total_rows + r ))
done
printf "#   %-16s %'12d rows\n" "total" "$total_rows"

# --- start postgres ---
echo "# starting postgres"
$COMPOSE up -d >/dev/null
for _ in $(seq 1 60); do
  $COMPOSE exec -T pg pg_isready -U bench >/dev/null 2>&1 && break
  sleep 0.5
done

cat <<EOF
# ----- connect -----
#   url:     $PGURL
#   psql:    psql '$PGURL'
#   docker:  docker exec -it $CONTAINER psql -U bench -d payments_bench
EOF

psql "$PGURL" -v ON_ERROR_STOP=1 -q -f schema.sql

# seedfaker emits columns in generation order (non-FK first, then FK-derived),
# which doesn't match DDL order — pre-fetch the CSV header for each table so
# COPY can take an explicit column list. One cheap seedfaker call per table.
for t in "${TABLES[@]}"; do
  h=$(seedfaker run payments.yaml --table "$t" --format csv -n 1 2>/dev/null | head -1)
  eval "HDR_${t}=\"\$h\""
done

# --- snapshot pg counters before load ---
LSN0=$(psql "$PGURL" -XAt -c "SELECT pg_current_wal_insert_lsn()")
CKPT0=$(psql "$PGURL" -XAt -c "SELECT num_timed + num_requested FROM pg_stat_checkpointer")

# --- load ---
echo "# loading (up to $JOBS concurrent)"
mkdir -p log
rm -f log/*.start log/*.end log/*.bytes log/*.err.* 2>/dev/null || true

# Build queue of work units. Each unit = "$table:$shard_idx:$shard_total".
# Big tables (transactions, authorizations, ledger_entries) are split into $SHARDS
# parallel shards; everything else runs as 1/1.
BIG_TABLES=(transactions authorizations ledger_entries)
queue=()
for t in "${TABLES[@]}"; do
  shards=1
  for b in "${BIG_TABLES[@]}"; do [[ "$t" == "$b" ]] && shards=$SHARDS; done
  if (( shards <= 1 )); then
    queue+=("$t:0:1")
  else
    for (( i=0; i<shards; i++ )); do
      queue+=("$t:$i:$shards")
    done
  fi
done

run_unit() {
  local spec=$1
  local t="${spec%%:*}"; local rest="${spec#*:}"
  local i="${rest%%:*}"; local n="${rest#*:}"
  local hv="HDR_$t"
  local tag="${t}_${i}"
  local shard_arg=""
  local hdr_flag="HEADER true"
  if (( n > 1 )); then
    shard_arg="--shard $i/$n"
    # Every shard emits its own header and psql's COPY HEADER true consumes it.
    # No --no-header needed because each shard goes into its own COPY session.
  fi
  date +%s > "log/$tag.start"
  seedfaker run payments.yaml --table "$t" --format csv $shard_arg 2>"log/$tag.err.sf" \
    | tee >(wc -c > "log/$tag.bytes") \
    | psql "$PGURL" -v ON_ERROR_STOP=1 -q \
        -c "\COPY $t (${!hv}) FROM STDIN WITH (FORMAT csv, $hdr_flag)" \
        2>"log/$tag.err.pg"
  local rc=$?
  date +%s > "log/$tag.end"
  return $rc
}
export -f run_unit
export PGURL

t_start=$(date +%s)
run_pids=()
run_names=()
done_count=0
fail=0

total_units=${#queue[@]}

# Bash 3.2 + set -u: empty-array expansion is "unbound" — disable inside pool.
set +u
while (( ${#queue[@]} > 0 )) || (( ${#run_pids[@]} > 0 )); do
  while (( ${#run_pids[@]} < JOBS )) && (( ${#queue[@]} > 0 )); do
    spec="${queue[0]}"
    queue=("${queue[@]:1}")
    run_unit "$spec" &
    run_pids+=($!)
    run_names+=("$spec")
  done
  new_pids=(); new_names=()
  for i in "${!run_pids[@]}"; do
    pid="${run_pids[$i]}"
    name="${run_names[$i]}"
    if kill -0 "$pid" 2>/dev/null; then
      new_pids+=("$pid"); new_names+=("$name")
    else
      wait "$pid" || { fail=1; echo; echo "# FAIL: $name (see log/${name%%:*}_${name#*:}.err.*)" >&2; }
      done_count=$(( done_count + 1 ))
    fi
  done
  run_pids=("${new_pids[@]}")
  run_names=("${new_names[@]}")
  label=""
  for n in "${run_names[@]}"; do
    t="${n%%:*}"; rest="${n#*:}"; i="${rest%%:*}"; total="${rest#*:}"
    if (( total > 1 )); then label+="$t/$i "; else label+="$t "; fi
  done
  [[ -z "$label" ]] && label="—"
  printf "\r# %3ds | done %d/%d | running: %s              " \
    "$(( $(date +%s) - t_start ))" "$done_count" "$total_units" "$label"
  (( ${#queue[@]} == 0 && ${#run_pids[@]} == 0 )) && break
  sleep 1
done
set -u
echo
t_wall=$(( $(date +%s) - t_start ))
[[ $fail -eq 0 ]] || exit 1

# --- per-table breakdown (aggregates across shards) ---
echo "# ----- per-table -----"
printf "#   %-16s %7s %12s %14s %8s %10s %14s\n" "table" "shards" "rows" "csv_bytes" "sec" "MB/s" "rows/s"
total_bytes=0; sum_dur=0; slowest_t=""; slowest_d=0
for t in "${TABLES[@]}"; do
  v="ROWS_$t"; r="${!v}"
  b_total=0; s_min=0; e_max=0; n_shards=0
  for f in log/${t}_*.start; do
    [[ -f "$f" ]] || continue
    tag="${f#log/}"; tag="${tag%.start}"
    s=$(cat "log/$tag.start"); e=$(cat "log/$tag.end")
    bb=$(cat "log/$tag.bytes" 2>/dev/null || echo 0)
    b_total=$(( b_total + bb ))
    (( n_shards == 0 || s < s_min )) && s_min=$s
    (( e > e_max )) && e_max=$e
    n_shards=$(( n_shards + 1 ))
  done
  d=$(( e_max - s_min )); (( d < 1 )) && d=1
  mbps=$(awk -v b="$b_total" -v d="$d" 'BEGIN{printf "%.1f", b/1048576/d}')
  rps=$(awk -v r="$r" -v d="$d" 'BEGIN{printf "%.0f", r/d}')
  printf "#   %-16s %7d %12d %14d %8d %10s %14s\n" "$t" "$n_shards" "$r" "$b_total" "$d" "$mbps" "$rps"
  total_bytes=$(( total_bytes + b_total ))
  sum_dur=$(( sum_dur + d ))
  (( d > slowest_d )) && { slowest_d=$d; slowest_t=$t; }
done

tot_mbps=$(awk -v b="$total_bytes" -v d="$t_wall" 'BEGIN{if(d<1)d=1; printf "%.1f", b/1048576/d}')
tot_rps=$(awk -v r="$total_rows" -v d="$t_wall" 'BEGIN{if(d<1)d=1; printf "%.0f", r/d}')
par_eff=$(awk -v s="$sum_dur" -v w="$t_wall" -v c="$CPUS" \
  'BEGIN{if(w<1||c<1)c=1; printf "%.0f", 100*s/(w*c)}')

echo "# ----- summary -----"
echo "#   wall time:         ${t_wall}s"
echo "#   csv bytes piped:   $total_bytes ($(( total_bytes / 1048576 )) MB)"
echo "#   aggregate rate:    $tot_mbps MB/s | $tot_rps rows/s"
echo "#   slowest table:     $slowest_t (${slowest_d}s)"
echo "#   sum table dur:     ${sum_dur}s (parallel eff ${par_eff}% of ${CPUS} cores)"

# --- postgres-side stats ---
LSN1=$(psql "$PGURL" -XAt -c "SELECT pg_current_wal_insert_lsn()")
CKPT1=$(psql "$PGURL" -XAt -c "SELECT num_timed + num_requested FROM pg_stat_checkpointer")
wal=$(psql "$PGURL" -XAt -c "SELECT pg_wal_lsn_diff('$LSN1','$LSN0')")
hit=$(psql "$PGURL" -XAt -c "SELECT round(100.0*sum(blks_hit)/nullif(sum(blks_hit+blks_read),0),2) FROM pg_stat_database")

echo "# ----- postgres -----"
echo "#   WAL written:       $(awk -v w="$wal" 'BEGIN{printf "%.1f MB", w/1048576}')"
echo "#   checkpoints:       $(( CKPT1 - CKPT0 ))"
echo "#   cache hit ratio:   ${hit}%"

echo "# ----- sizes -----"
psql "$PGURL" -XA -F'|' -c "
  SELECT relname,
         to_char(n_live_tup, 'FM999,999,999') AS rows,
         pg_size_pretty(pg_total_relation_size(relid)) AS size
  FROM pg_stat_user_tables
  ORDER BY pg_total_relation_size(relid) DESC;
" | column -t -s'|' | sed 's/^/#   /'
psql "$PGURL" -XAt -c "SELECT pg_size_pretty(pg_database_size('payments_bench'));" \
  | awk '{print "#   db total:         "$0}'

if [[ $TEARDOWN -eq 1 ]]; then
  wipe
else
  cat <<EOF
# ----- still running -----
#   psql '$PGURL'
#   teardown: $0 --cleanup
EOF
fi
