"""Shared field metadata loader for type generators.

Reads rust/core/fields.yaml — the single source of truth for all field
definitions, modifiers, ranges, ordering, and length capabilities.
"""

from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
YAML_PATH = ROOT / "rust" / "core" / "fields.yaml"
BUILD_TYPES = ROOT / "build" / "types"


def load():
    """Return list of field dicts from fields.yaml."""
    with open(YAML_PATH) as f:
        data = yaml.safe_load(f)
    return data["fields"]


def field_names(fields=None):
    """Return ordered list of field name strings."""
    if fields is None:
        fields = load()
    return [f["name"] for f in fields]


def to_pascal(name: str) -> str:
    """credit-card -> CreditCard"""
    return "".join(w.capitalize() for w in name.replace("-", " ").split())


def to_snake(name: str) -> str:
    """credit-card -> credit_card"""
    return name.replace("-", "_")


def all_modifiers(fields=None):
    """Return sorted set of all unique modifier names across all fields."""
    if fields is None:
        fields = load()
    mods = set()
    for f in fields:
        for m in f.get("modifiers", []):
            mods.add(m)
    return sorted(mods)
