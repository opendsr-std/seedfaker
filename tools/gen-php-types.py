#!/usr/bin/env python3
"""Generate PHP field() parameter list from fields.yaml.

Injects the parameter list between @field-params-start and @field-params-end
markers in packages/php/src/SeedFaker.php.

Source: rust/core/fields.yaml (single source of truth)
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import field_meta

PHP_PATH = field_meta.ROOT / "packages" / "php" / "src" / "SeedFaker.php"


def generate(fields):
    """Generate the field() method signature with all modifier params."""
    all_mods = []
    seen = set()
    for f in fields:
        for m in f.get("modifiers", []):
            if m not in seen:
                all_mods.append(m)
                seen.add(m)

    lines = []
    lines.append("    public function field(")
    lines.append("        string $name,")
    lines.append("        int $n = 1,")

    for m in all_mods:
        php_name = m.replace("-", "_")
        if php_name[0].isdigit():
            php_name = "r" + php_name
        lines.append(f"        bool ${php_name} = false,")

    for u in ["upper", "lower", "capitalize", "xuniq", "asc", "desc"]:
        if u not in seen:
            lines.append(f"        bool ${u} = false,")
    lines.append("        ?int $omit = null,")
    lines.append("        ?int $length = null,")
    lines.append("        ?array $range = null,")
    lines.append("    ): string|array {")

    return "\n".join(lines)


def inject(content):
    with open(PHP_PATH) as f:
        text = f.read()
    start = "// @field-params-start"
    end = "// @field-params-end"
    i = text.find(start)
    j = text.find(end)
    if i == -1 or j == -1:
        print(f"  SKIP {PHP_PATH}: markers not found")
        return
    new = text[: i + len(start)] + "\n" + content + "\n    " + text[j:]
    with open(PHP_PATH, "w") as f:
        f.write(new)


def main():
    fields = field_meta.load()
    content = generate(fields)
    inject(content)
    mods = field_meta.all_modifiers(fields)
    print(f"  packages/php/src/SeedFaker.php ({len(mods)} modifiers)")


if __name__ == "__main__":
    main()
