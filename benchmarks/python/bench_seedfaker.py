"""Benchmark: seedfaker pip package (native PyO3)
Usage: python bench_seedfaker.py N TIER
"""

import json
import sys
import time
from seedfaker import SeedFaker

N = int(sys.argv[1]) if len(sys.argv) > 1 else 10000
TIER = int(sys.argv[2]) if len(sys.argv) > 2 else 3

FIELDS_BY_TIER = {
    1: ["name"],
    3: ["name", "email", "phone"],
    5: ["name", "email", "phone", "city", "birthdate"],
    10: [
        "name",
        "email",
        "phone",
        "city",
        "birthdate",
        "country",
        "username",
        "postal-code",
        "ssn",
        "credit-card",
    ],
    15: [
        "name",
        "email",
        "phone",
        "city",
        "birthdate",
        "country",
        "username",
        "postal-code",
        "ssn",
        "credit-card",
        "address",
        "company-name",
        "job-title",
        "iban",
        "password",
    ],
    20: [
        "name",
        "email",
        "phone",
        "city",
        "birthdate",
        "country",
        "username",
        "postal-code",
        "ssn",
        "credit-card",
        "address",
        "company-name",
        "job-title",
        "iban",
        "password",
        "ip",
        "uuid",
        "timestamp",
        "passport",
        "national-id",
    ],
}

fields = FIELDS_BY_TIER.get(TIER)
if fields is None:
    raise ValueError(f"unsupported tier: {TIER}")

f = SeedFaker(seed="bench")

# Single FFI call for the entire batch
sink = 0
start = time.perf_counter()
records = f.records(fields, n=N)
for rec in records:
    for v in rec.values():
        sink += len(v)
elapsed = time.perf_counter() - start

rps = int(N / elapsed)
print(
    json.dumps(
        {
            "tool": "seedfaker",
            "n": N,
            "tier": TIER,
            "elapsed": f"{elapsed:.3f}",
            "rps": rps,
            "sink": sink,
        }
    )
)
