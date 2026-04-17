#!/usr/bin/env python3
"""Generate package bindings from seedfaker --list-json.

Replaces content between marker comments in package files.
Usage: seedfaker --list-json | python3 tools/gen-bindings.py

Type definitions are generated separately by per-language scripts
(gen-ts-types.py, gen-py-types.py, gen-go-types.py, gen-php-types.py).
"""

import json
import sys


def inject(path: str, start_marker: str, end_marker: str, content: str):
    with open(path) as f:
        text = f.read()
    i = text.find(start_marker)
    j = text.find(end_marker)
    if i == -1 or j == -1:
        print(f"  SKIP {path}: markers not found")
        return
    new = text[: i + len(start_marker)] + "\n" + content + text[j:]
    with open(path, "w") as f:
        f.write(new)


def gen_js_fields(fields):
    lines = ["const FIELDS = ["]
    for f in fields:
        lines.append(f'  "{f["name"]}",')
    lines.append("];")
    lines.append("")
    return "\n".join(lines)


def gen_py_fields_list(fields):
    names = [f["name"] for f in fields]
    lines = ["_FIELDS = ["]
    for i in range(0, len(names), 5):
        chunk = ", ".join(f'"{n}"' for n in names[i : i + 5])
        lines.append(f"    {chunk},")
    lines.append("]")
    lines.append("")
    return "\n".join(lines)


def gen_py_static_methods():
    lines = [
        "    @staticmethod",
        "    def fields() -> list[str]:",
        "        return list(_FIELDS)",
        "",
        "    @staticmethod",
        "    def fingerprint() -> str:",
        '        """Algorithm version. Changes when seeded output changes."""',
        "        return _NativeSeedFaker.fingerprint()",
        "",
    ]
    return "\n".join(lines)


def main():
    fields = json.load(sys.stdin)

    # JS — inject FIELDS array only (no dynamic methods)
    inject(
        "packages/npm/index.js",
        "// @generated-start",
        "// @generated-end",
        gen_js_fields(fields),
    )
    print(f"  packages/npm/index.js ({len(fields)} fields)")

    # Python — inject _FIELDS list and static methods only
    inject(
        "packages/pip/seedfaker/__init__.py",
        "# @fields-start",
        "# @fields-end",
        gen_py_fields_list(fields),
    )
    inject(
        "packages/pip/seedfaker/__init__.py",
        "# @generated-start",
        "# @generated-end",
        gen_py_static_methods(),
    )
    print(f"  packages/pip/seedfaker/__init__.py ({len(fields)} fields)")

    print(f"\nDone: {len(fields)} fields.")


if __name__ == "__main__":
    main()
