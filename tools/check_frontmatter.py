#!/usr/bin/env python3
"""Prueft die Frontmatter-Skalare der Dossiers gegen die Seed-Parser-Regeln.

Spiegelt crates/ingestion/build.rs (parse_scalar) in Python: nur flache
Skalare, einfache oder doppelte Anfuehrungszeichen, kein Kommentar nach
einem Leerzeichen, keine Block-/Fluss-Skalare.

Aufruf:
    python3 tools/check_frontmatter.py [dossier ...]
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
KEYS = {
    "slug", "name", "trader_type", "state", "city", "street", "postcode",
    "phone", "email", "opening_hours", "website", "website_status", "status",
    "description", "dropoff_json", "pickup_json", "certifications",
    "provenance_section", "provenance_ankauf_raw", "provenance_origin",
    "clear_fields", "lat", "lon", "extra_json", "notes",
}


def parse_scalar(ctx, raw):
    if raw == "":
        return ""
    if raw.startswith("'"):
        if not raw.endswith("'") or len(raw) < 2:
            raise ValueError(f"{ctx}: unbalanced single quotes: {raw!r}")
        return raw[1:-1].replace("''", "'")
    if raw.startswith('"'):
        if not raw.endswith('"') or len(raw) < 2:
            raise ValueError(f"{ctx}: unbalanced double quotes: {raw!r}")
        return raw[1:-1]
    if raw[0] in ">|*&![{@`":
        raise ValueError(f"{ctx}: nur flache Skalare erlaubt: {raw!r}")
    if raw.startswith("#"):
        return ""
    if " #" in raw or "\t#" in raw:
        raise ValueError(f"{ctx}: Wert mit Kommentar braucht Quotes: {raw!r}")
    if ": " in raw:
        raise ValueError(f"{ctx}: unquotierter Wert mit ': ': {raw!r}")
    if raw in ("null", "Null", "NULL", "~"):
        return ""
    return raw


def check(path):
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise ValueError(f"{path}: kein Frontmatter")
    block = text.split("---\n", 2)[1]
    seen = set()
    for i, line in enumerate(block.splitlines(), start=2):
        if not line.strip():
            continue
        m = re.match(r"^([a-z_]+):(.*)$", line)
        if not m:
            raise ValueError(f"{path}:{i}: keine key: value-Zeile: {line!r}")
        key, raw = m.group(1), m.group(2).strip()
        if key not in KEYS:
            raise ValueError(f"{path}:{i}: unbekannter Key {key!r}")
        if key in seen:
            raise ValueError(f"{path}:{i}: doppelter Key {key!r}")
        seen.add(key)
        parse_scalar(f"{path}:{i} ({key})", raw)
    return seen


def main():
    targets = sys.argv[1:]
    if targets:
        paths = []
        for t in targets:
            m = list((ROOT / "dossiers").rglob(f"{t}.md"))
            if not m:
                raise SystemExit(f"Dossier {t} nicht gefunden")
            paths.append(m[0])
    else:
        paths = sorted((ROOT / "dossiers").rglob("*.md"))
    bad = 0
    for p in paths:
        try:
            check(p)
        except ValueError as e:
            print(f"FEHLER {e}")
            bad += 1
    print(f"Geprueft: {len(paths)} | Fehler: {bad}")
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
