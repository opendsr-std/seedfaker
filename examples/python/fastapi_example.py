#!/usr/bin/env python3
"""
seedfaker + FastAPI — synthetic data API endpoint

Install:  pip install seedfaker fastapi uvicorn
Run:      uvicorn fastapi_example:app
Test:     curl 'http://localhost:8000/users?n=5&seed=demo&locale=en'
Docs:     https://github.com/opendsr-std/seedfaker
"""

from fastapi import FastAPI, Query
from seedfaker import SeedFaker

app = FastAPI()


@app.get("/users")
def users(n: int = Query(10, ge=1, le=10000), seed: str = None, locale: str = "en"):
    f = SeedFaker(seed=seed, locale=locale)
    return f.records(["name", "email", "phone:e164"], n=n, ctx="strict")


@app.get("/payments")
def payments(n: int = Query(10, ge=1, le=10000), seed: str = None):
    f = SeedFaker(seed=seed, locale="en")
    return f.records(["credit-card:space", "amount:usd", "currency-code"], n=n)
