"""Benchmark: polyfactory (Python)
Usage: python bench_polyfactory.py N TIER
  TIER: 1, 3, 5, 10, 15, 20

NOTE: polyfactory generates random strings for str fields, not structured
names/emails/phones. Results are not directly comparable with other tools.
"""

import json
import sys
import time
from dataclasses import dataclass
from polyfactory.factories import DataclassFactory

N = int(sys.argv[1]) if len(sys.argv) > 1 else 100000
TIER = int(sys.argv[2]) if len(sys.argv) > 2 else 3


@dataclass
class T1:
    name: str


@dataclass
class T3:
    name: str
    email: str
    phone: str


@dataclass
class T5:
    name: str
    email: str
    phone: str
    city: str
    dob: str


@dataclass
class T10:
    name: str
    email: str
    phone: str
    city: str
    dob: str
    country: str
    username: str
    postal_code: str
    ip: str
    latitude: str


@dataclass
class T15:
    name: str
    email: str
    phone: str
    city: str
    dob: str
    country: str
    username: str
    postal_code: str
    ip: str
    latitude: str
    longitude: str
    company: str
    job_title: str
    credit_card: str
    ssn: str


@dataclass
class T20:
    name: str
    email: str
    phone: str
    city: str
    dob: str
    country: str
    username: str
    postal_code: str
    ip: str
    latitude: str
    longitude: str
    company: str
    job_title: str
    credit_card: str
    ssn: str
    uuid: str
    password: str
    address: str
    amount: str
    gender: str


model = {1: T1, 3: T3, 5: T5, 10: T10, 15: T15, 20: T20}[TIER]
factory = DataclassFactory.create_factory(model)
field_count = TIER

sink = 0
start = time.perf_counter()
for _ in range(N):
    p = factory.build()
    sink += len(p.name)
elapsed = time.perf_counter() - start

rps = int(N / elapsed)
print(
    json.dumps(
        {
            "tool": "polyfactory",
            "n": N,
            "tier": TIER,
            "elapsed": f"{elapsed:.3f}",
            "rps": rps,
            "sink": sink,
        }
    )
)
