"""seedfaker — deterministic synthetic data generator."""

from __future__ import annotations

import hashlib
from pathlib import Path

from seedfaker._seedfaker import SeedFaker as _NativeSeedFaker

__version__ = "0.3.0a2"


# @checksum-start
# CI replaces this with real SHA256 before building wheel.
# Empty = dev (editable install). Filled = production (verify mandatory).
_NATIVE_CHECKSUM = ""
# @checksum-end


def _verify_native_module() -> None:
    if not _NATIVE_CHECKSUM:
        return

    for ext in (".so", ".dylib", ".pyd"):
        for f in Path(__file__).parent.glob(f"_seedfaker*{ext}"):
            actual = hashlib.sha256(f.read_bytes()).hexdigest()
            if actual != _NATIVE_CHECKSUM:
                msg = (
                    f"seedfaker: native module integrity check failed. "
                    f"Expected {_NATIVE_CHECKSUM[:16]}..., got {actual[:16]}... "
                    f"Reinstall the package or verify your installation."
                )
                raise RuntimeError(msg)
            return


_verify_native_module()


# @fields-start
_FIELDS = [
    "integer", "float", "boolean", "digit", "bit",
    "trit", "enum", "serial", "letter", "trigram",
    "digits", "letters", "alnum", "base64", "hex",
    "word", "message", "emoji", "color", "uuid",
    "bz", "dice", "excuse", "mball", "timestamp",
    "date", "name", "first-name", "last-name", "middle-name",
    "birthdate", "age", "gender", "username", "login-name",
    "social-handle", "nickname", "biometric-id", "student-id", "email",
    "phone", "address", "street-address", "city", "state",
    "postal-code", "country", "latitude", "longitude", "country-code",
    "phone-code", "language-code", "locale-code", "timezone", "credit-card",
    "cvv", "iban", "swift-bic", "routing-number", "bank-account",
    "tax-id", "amount", "currency-code", "currency-symbol", "password",
    "jwt", "bearer-token", "api-key", "totp-secret", "oauth-client-secret",
    "aws-access-key", "aws-secret-key", "stripe-key", "github-pat", "gitlab-token",
    "openai-key", "sendgrid-key", "twilio-sid", "twilio-token", "slack-bot-token",
    "slack-user-token", "datadog-key", "sentry-dsn", "vault-token", "npm-token",
    "vercel-token", "supabase-key", "telegram-token", "discord-webhook", "gcp-key",
    "azure-key", "cloudflare-token", "pagerduty-key", "newrelic-key", "splunk-token",
    "heroku-key", "firebase-key", "ssh-private-key", "ssh-public-key", "connection-string",
    "anthropic-key", "session-id", "passkey-id", "facebook-token", "google-token",
    "apple-token", "refresh-token", "csrf-token", "basic-auth", "ssn",
    "passport", "drivers-license", "national-id", "cpf", "sin",
    "tfn", "nino", "nhs-number", "nir", "codice-fiscale",
    "dni", "nie", "bsn", "personnummer", "steuer-id",
    "cuil", "jmbg", "tc-kimlik", "pesel", "curp",
    "rut", "inn", "ipn", "abn", "cnpj",
    "oib", "amka", "rodne-cislo", "szemelyi-szam", "hetu",
    "cpr", "fodselsnummer", "pps", "emso", "egn",
    "idnp", "health-card", "cedula", "aadhaar", "pan",
    "cccd", "shenfenzheng", "ip", "ipv6", "mac",
    "url", "auth-url", "internal-url", "dns-record", "browser-cookie",
    "user-agent", "mime-type", "http-method", "http-status", "port",
    "latency", "image-url", "twitter-url", "linkedin-url", "facebook-url",
    "instagram-url", "github-url", "telegram-url", "youtube-url", "webhook-url",
    "btc-address", "eth-address", "sol-address", "tx-hash", "pgp-fingerprint",
    "company-name", "ein", "vat-number", "duns", "lei",
    "job-title", "ldap-dn", "employee-id", "court-case", "mrn",
    "npi", "insurance-id", "medicare-id", "icd-10", "cpt-code",
    "ndc", "rx-number", "project-code", "jira-id", "github-issue",
    "commit-hash", "semver", "docker-image", "slack-channel", "sentry-issue",
    "pagerduty-incident", "file-path", "s3-path", "env-var", "vin",
    "license-plate", "imei", "imsi", "device-id",
]
# @fields-end


def _build_spec(name: str, kwargs: dict) -> str:
    parts = [name]
    for k, v in kwargs.items():
        if k in ("n", "field"):
            continue
        # Strip 'r' prefix added for digit-starting modifiers (r1x1 → 1x1)
        seg = k[1:] if len(k) > 1 and k[0] == "r" and k[1].isdigit() else k
        if k == "range" and isinstance(v, (list, tuple)):
            parts.append(f"{v[0]}..{v[1]}")
        elif v is True:
            parts.append(seg)
        elif isinstance(v, int):
            parts.append(f"{seg}={v}")
    return ":".join(parts)


class SeedFaker:
    """Deterministic synthetic data generator."""

    def __init__(
        self,
        seed: str | None = None,
        locale: str | None = None,
        tz: str | None = None,
        since: int | None = None,
        until: int | None = None,
    ) -> None:
        self._native = _NativeSeedFaker(seed=seed, locale=locale, tz=tz, since=since, until=until)

    def field(
        self,
        name: str,
        **kwargs: object,
    ) -> str:
        """``field("phone", e164=True)`` → ``phone:e164``."""
        return self._native.field(_build_spec(name, kwargs))

    def record(
        self,
        fields: list[str],
        *,
        ctx: str | None = None,
        corrupt: str | None = None,
    ) -> dict[str, str]:
        return self._native.record(fields, ctx=ctx, corrupt=corrupt)

    def records(
        self,
        fields: list[str],
        *,
        n: int = 1,
        ctx: str | None = None,
        corrupt: str | None = None,
    ) -> list[dict[str, str]]:
        return self._native.records(fields, n=n, ctx=ctx, corrupt=corrupt)

    @staticmethod
    def validate(
        fields: list[str],
        *,
        ctx: str | None = None,
        corrupt: str | None = None,
    ) -> None:
        _NativeSeedFaker.validate(fields, ctx=ctx, corrupt=corrupt)

    # @generated-start
    @staticmethod
    def fields() -> list[str]:
        return list(_FIELDS)

    @staticmethod
    def fingerprint() -> str:
        """Algorithm version. Changes when seeded output changes."""
        return _NativeSeedFaker.fingerprint()
# @generated-end
