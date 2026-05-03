#!/usr/bin/env python3
"""
Install:  pip install seedfaker
Docs:     https://github.com/opendsr-std/seedfaker
"""

from seedfaker import SeedFaker

f = SeedFaker(seed="demo", locale="en")

print(f.field("name"))
print(f.field("email"))
print(f.field("phone", modifier="e164"))
print(f.field("credit-card", modifier="space"))

# ctx=strict → name, email, phone correlated per row
for r in f.records(["name", "email", "phone"], n=5, ctx="strict"):
    print(f"{r['name']}\t{r['email']}\t{r['phone']}")

# corrupt=high → noisy values
print(f.records(["name", "email", "ssn"], n=3, corrupt="high"))

a = SeedFaker(seed="ci")
b = SeedFaker(seed="ci")
assert a.field("name") == b.field("name"), "determinism failed"
