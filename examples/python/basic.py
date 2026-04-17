#!/usr/bin/env python3
"""
seedfaker — deterministic synthetic data generator

Install:  pip install seedfaker
Docs:     https://github.com/opendsr-std/seedfaker
"""

from seedfaker import SeedFaker

# Deterministic: same seed = same output, always
f = SeedFaker(seed="demo", locale="en")

print(f.field("name"))
print(f.field("email"))
print(f.field("phone", modifier="e164"))
print(f.field("credit-card", modifier="space"))

# Correlated records: email derived from name, phone matches locale
records = f.records(
    ["name", "email", "phone"],
    n=5,
    ctx="strict",
)
for r in records:
    print(f"{r['name']}\t{r['email']}\t{r['phone']}")

# Noisy data for ML/NER training
noisy = f.records(["name", "email", "ssn"], n=3, corrupt="high")
print(noisy)

# Verify determinism
a = SeedFaker(seed="ci")
b = SeedFaker(seed="ci")
assert a.field("name") == b.field("name")
