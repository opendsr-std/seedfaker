# Cross-Interface Determinism

Same seed, same fields, same output — across every interface. This is the core guarantee.

**Parameters:** seed=`determinism-proof`, locale=`en`, n=100, until=2025
**Fields:** `name email phone city birthdate credit-card ssn passport ip uuid`

| Interface | SHA-256 | Rows | Status |
|-----------|---------|------|--------|
| CLI (Rust binary) | `5d37479daaa7d78e...` | 100 | pass |
| Python (PyO3) | `5d37479daaa7d78e...` | 100 | pass |
| Node.js (NAPI-RS) | `5d37479daaa7d78e...` | 100 | pass |
| Go (FFI/CGO) | `5d37479daaa7d78e...` | 100 | pass |
| PHP (FFI) | `5d37479daaa7d78e...` | 100 | pass |
| Ruby (Fiddle) | `5d37479daaa7d78e...` | 100 | pass |
| MCP (JSON-RPC) | `5d37479daaa7d78e...` | 100 | pass |

All 7 tested interfaces produce byte-identical output.

Full SHA-256: `5d37479daaa7d78e6be02d520396e5ef46a07e96d284065855c2eb52ecb5a7e6`

## Reproduce

```bash
# Any interface — same hash:
seedfaker name email phone city birthdate credit-card ssn passport ip uuid --seed determinism-proof --locale en -n 100 --until 2025 --no-header | shasum -a 256
```
