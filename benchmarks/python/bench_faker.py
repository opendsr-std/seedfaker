"""Benchmark: faker (Python)
Usage: python bench_faker.py N TIER
"""

import json
import sys
import time
from faker import Faker

N = int(sys.argv[1]) if len(sys.argv) > 1 else 100000
TIER = int(sys.argv[2]) if len(sys.argv) > 2 else 3

f = Faker("en_US")
Faker.seed(42)


def gen_3():
    return len(f.name()) + len(f.email()) + len(f.phone_number())


def gen_5():
    return gen_3() + len(f.city()) + len(f.date_of_birth().isoformat())


def gen_10():
    return (
        gen_5()
        + len(f.country())
        + len(f.user_name())
        + len(f.zipcode())
        + len(f.ssn())
        + len(f.credit_card_number())
    )


def gen_15():
    return (
        gen_10()
        + len(f.address())
        + len(f.company())
        + len(f.job())
        + len(f.iban())
        + len(f.password())
    )


def gen_20():
    return (
        gen_15()
        + len(f.ipv4())
        + len(f.uuid4())
        + len(f.date_time().isoformat())
        + len(f.passport_number())
        + len(f.ssn())
    )


gen = {
    1: lambda: len(f.name()),
    3: gen_3,
    5: gen_5,
    10: gen_10,
    15: gen_15,
    20: gen_20,
}[TIER]

sink = 0
start = time.perf_counter()
for _ in range(N):
    sink += gen()
elapsed = time.perf_counter() - start

rps = int(N / elapsed)
print(
    json.dumps(
        {
            "tool": "faker.py",
            "n": N,
            "tier": TIER,
            "elapsed": f"{elapsed:.3f}",
            "rps": rps,
            "sink": sink,
        }
    )
)
