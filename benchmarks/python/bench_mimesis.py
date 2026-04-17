"""Benchmark: mimesis (Python)
Usage: python bench_mimesis.py N TIER
"""

import json
import sys
import time
import uuid as _uuid
from mimesis import Person, Address, Internet, Payment, Finance
from mimesis.locales import Locale

N = int(sys.argv[1]) if len(sys.argv) > 1 else 100000
TIER = int(sys.argv[2]) if len(sys.argv) > 2 else 3

p = Person(Locale.EN, seed=42)
a = Address(Locale.EN, seed=42)
net = Internet(seed=42)
pay = Payment(seed=42)
fin = Finance(seed=42)


def gen_3():
    return len(p.full_name()) + len(p.email()) + len(p.telephone())


def gen_5():
    return gen_3() + len(a.city()) + len(str(p.birthdate()))


def gen_10():
    return (
        gen_5()
        + len(a.country())
        + len(p.username())
        + len(a.zip_code())
        + len(p.identifier())
        + len(pay.credit_card_number())
    )


def gen_15():
    return (
        gen_10()
        + len(a.address())
        + len(fin.company())
        + len(p.occupation())
        + len(pay.credit_card_number())
        + len(p.password())
    )


def gen_20():
    return (
        gen_15()
        + len(net.ip_v4())
        + len(str(_uuid.uuid4()))
        + len(str(p.birthdate()))
        + len(p.identifier())
        + len(p.identifier())
    )


gen = {
    1: lambda: len(p.full_name()),
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
            "tool": "mimesis",
            "n": N,
            "tier": TIER,
            "elapsed": f"{elapsed:.3f}",
            "rps": rps,
            "sink": sink,
        }
    )
)
