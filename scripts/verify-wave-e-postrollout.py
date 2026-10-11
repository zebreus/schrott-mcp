#!/usr/bin/env python3
"""Read-only Wave E publication gate, not a pre-rollout approval or source audit.

Only the 60 manifest slugs, reviewed target scalars/address withdrawals and
complete seeded Timeline notes are checked. No ingest, internal DB, scraping,
GEO validation, price coverage or deployment/build identity proof. Pair with
the owner source review and deployment journal. --since must be the actual
rollout start; unrelated writes can also advance updated_at.
"""
import argparse
import hashlib
import json
import re
import sqlite3
from pathlib import Path

MANIFEST = Path(__file__).with_name("wave-e-20261011-expectations.json")
FIELDS = {"name", "street", "postcode", "phone", "email", "opening_hours",
          "description", "status", "trader_type", "website", "website_status"}
CLEARS = {"phone", "email", "opening_hours", "description"}


def connect(path):
    db = sqlite3.connect(path.resolve().as_uri() + "?mode=ro", uri=True)
    db.row_factory = sqlite3.Row
    db.execute("PRAGMA query_only=ON")
    return db


def load_manifest():
    manifest = json.loads(MANIFEST.read_text())
    rows = manifest["traders"]
    if len(rows) != 60 or len({row["slug"] for row in rows}) != 60:
        raise ValueError("manifest must contain exactly 60 unique slugs")
    for row in rows:
        values = row["values"]
        clears = row["clear_fields"]
        if (not {"street", "postcode"} <= values.keys()
                or not values.keys() <= FIELDS
                or not all(isinstance(value, str) for value in values.values())
                or not set(clears) <= CLEARS or len(set(clears)) != len(clears)
                or any(values.get(field) != "" for field in clears)
                or not re.fullmatch(r"[0-9a-f]{64}", row["notes_sha256"])):
            raise ValueError("invalid bounded expectation: " + row["slug"])
    return manifest


def verify(public, since):
    if public.execute("SELECT julianday(?)", (since,)).fetchone()[0] is None:
        raise ValueError("invalid rollout timestamp")
    failures = []
    checked = 0
    manifest = load_manifest()
    for expected in manifest["traders"]:
        slug = expected["slug"]
        rows = public.execute("SELECT * FROM traders WHERE slug=?", (slug,)).fetchall()
        if len(rows) != 1:
            failures.append(f"{slug}: expected one trader, found {len(rows)}")
            continue
        checked += 1
        row = rows[0]
        if public.execute("SELECT julianday(?)>=julianday(?)",
                          (row["updated_at"], since)).fetchone()[0] != 1:
            failures.append(f"{slug}: post-rollout updated_at missing")
        for field, value in expected["values"].items():
            if row[field] != value:
                failures.append(f"{slug}: {field} target mismatch")
        notes = row["notes"]
        if (not isinstance(notes, str)
                or hashlib.sha256(notes.encode()).hexdigest() != expected["notes_sha256"]):
            failures.append(f"{slug}: seeded Timeline notes mismatch")
    return {"ok": not failures, "checked_count": checked, "expected_count": 60,
            "scope": "Wave E scalars/withdrawals/Timeline only; GEO and sources not validated",
            "failures": sorted(failures)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--since", required=True)
    parser.add_argument("--data-dir", type=Path, default=Path("/var/lib/schrott-mcp"))
    args = parser.parse_args()
    try:
        with connect(args.data_dir / "public.db") as public:
            result = verify(public, args.since)
    except (OSError, sqlite3.Error, ValueError, KeyError, TypeError) as error:
        result = {"ok": False, "failures": [f"gate error: {error}"]}
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
