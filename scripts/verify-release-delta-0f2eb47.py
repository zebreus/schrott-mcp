#!/usr/bin/env python3
"""Read-only state check for the rows a release changes.

This complements the release-pinned Wave E verifier: instead of a historical
60-row scope it pins exactly the trader rows the candidate release changes, so
the post-rollout check proves the effect of that release. It checks the 11
documented scalars, the complete seeded Timeline notes hash, the stored seed
fingerprint, the coordinate pair and the service JSON of every row in scope.

Usage:
  python3 verify-release-delta-<short>.py [--data-dir /var/lib/schrott-mcp]
"""
import argparse
import hashlib
import json
import re
import sqlite3
from pathlib import Path

<<<<<<<< HEAD:scripts/verify-release-delta-0f2eb47.py
MANIFEST = Path(__file__).with_name("release-delta-0f2eb47-seed-expectations.json")
MANIFEST_SHA256 = "cd3264653163082ea4cac60ebb1b63ddb2cfbc098ad78b73cdf63b4c86f759d9"
RELEASE_COMMIT = "0f2eb47a86d0d2f464ccfeba9f45113276d465fa"
RELEASE_TREE = "d82f3b2f88ec6ce2138419447b98adc4223d9ec5"
REHEARSAL_MANIFEST_SHA256 = "6bd5e0da4fe17816dce4f8eac6cee046a476779e0d922982ccbc88f89b941d2c"
========
MANIFEST = Path(__file__).with_name("release-delta-da3dac2-seed-expectations.json")
MANIFEST_SHA256 = "126d7f97d8f0096729f797ec9ffc49023dfe008d9e1302ec0608123f9a8de7c9"
RELEASE_COMMIT = "da3dac2b7331f7dca47d553dd2655bffa29e824f"
RELEASE_TREE = "e3af185b86fb3984a6a692da6e329e46b7d82cfd"
REHEARSAL_MANIFEST_SHA256 = "b04aa912a822ce41ad37df6df404e20ed9955bc4cf02dc8a38839aa1430350a6"
>>>>>>>> rel12:scripts/verify-release-delta-da3dac2.py
FIELDS = (
    "name", "city", "street", "postcode", "phone", "email", "opening_hours",
    "description", "status", "trader_type", "website", "website_status",
)


def connect(path):
    conn = sqlite3.connect(path.resolve().as_uri() + "?mode=ro", uri=True)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA query_only=ON")
    return conn


def load_manifest(path=None):
    path = MANIFEST if path is None else path
    raw = path.read_bytes()
    if path.resolve() == MANIFEST.resolve() and hashlib.sha256(raw).hexdigest() != MANIFEST_SHA256:
        raise ValueError("release delta expectation manifest digest mismatch")
    manifest = json.loads(raw)
    validate_manifest(manifest)
    return manifest


def validate_manifest(manifest):
    required = {
        "version", "release_commit", "release_tree", "rehearsal_manifest_sha256",
        "fields", "scope", "scope_slugs", "expectation_basis", "traders",
    }
    if not isinstance(manifest, dict) or set(manifest) != required:
        raise ValueError("release delta expectation manifest has an invalid shape")
    if type(manifest["version"]) is not int or manifest["version"] != 1:
        raise ValueError("unsupported expectation version")
    if manifest["release_commit"] != RELEASE_COMMIT or manifest["release_tree"] != RELEASE_TREE:
        raise ValueError("release expectation identity does not match the reviewed release")
    if manifest["rehearsal_manifest_sha256"] != REHEARSAL_MANIFEST_SHA256:
        raise ValueError("release expectation does not pin the reviewed rehearsal")
    if manifest["fields"] != list(FIELDS):
        raise ValueError("release expectation fields do not match the reviewed scope")
    for key in ("scope", "expectation_basis"):
        if not isinstance(manifest[key], str) or not manifest[key].strip():
            raise ValueError(f"release expectation {key} is missing")
    rows = manifest["traders"]
    if not isinstance(rows, list) or not rows:
        raise ValueError("release expectation must contain at least one trader")
    slugs = []
    for row in rows:
        if not isinstance(row, dict) or set(row) != {
            "slug", "values", "notes_sha256", "seed_hash", "geo",
            "dropoff_json", "pickup_json", "certifications",
        }:
            raise ValueError("invalid release expectation row")
        if (not isinstance(row["slug"], str) or not row["slug"]
                or not isinstance(row["values"], dict) or set(row["values"]) != set(FIELDS)
                or not isinstance(row["notes_sha256"], str)
                or not re.fullmatch(r"[0-9a-f]{64}", row["notes_sha256"])
                or not isinstance(row["seed_hash"], str)
                or not re.fullmatch(r"[0-9a-f]{16}", row["seed_hash"])
                or not isinstance(row["geo"], list) or len(row["geo"]) != 2):
            raise ValueError("invalid release expectation values for " + str(row.get("slug")))
        slugs.append(row["slug"])
    if len(set(slugs)) != len(slugs):
        raise ValueError("release expectation has duplicate trader slugs")
    scope = manifest["scope_slugs"]
    if (not isinstance(scope, list) or len(scope) != len(slugs)
            or set(scope) != set(slugs)):
        raise ValueError("release expectation scope does not match its trader rows")
    return rows


def verify(public, manifest=None):
    manifest = load_manifest() if manifest is None else manifest
    expected_rows = validate_manifest(manifest)
    failures = []
    checked = 0
    scalar_checks = 0
    notes_checks = 0
    seed_hash_checks = 0
    geo_checks = 0
    service_checks = 0
    row_results = []
    columns = ",".join(FIELDS)
    for expected in expected_rows:
        slug = expected["slug"]
        row_failures = []
        rows = public.execute(
            f"SELECT slug,{columns},notes,extra_json,lat,lon,dropoff_json,pickup_json,"
            "certifications FROM traders WHERE slug=?", (slug,)).fetchall()
        if len(rows) != 1:
            failure = f"{slug}: expected one trader, found {len(rows)}"
            failures.append(failure)
            row_results.append({"slug": slug, "ok": False, "failures": [failure]})
            continue
        checked += 1
        row = rows[0]
        for field in FIELDS:
            scalar_checks += 1
            if row[field] != expected["values"][field]:
                failure = f"{slug}: {field} target mismatch"
                failures.append(failure)
                row_failures.append(failure)
        notes = row["notes"]
        notes_hash_match = False
        if not isinstance(notes, str):
            failure = f"{slug}: seeded Timeline notes missing or invalid"
            failures.append(failure)
            row_failures.append(failure)
        else:
            notes_checks += 1
            notes_hash_match = hashlib.sha256(notes.encode()).hexdigest() == expected["notes_sha256"]
            if not notes_hash_match:
                failure = f"{slug}: seeded Timeline notes mismatch"
                failures.append(failure)
                row_failures.append(failure)
        try:
            extra = json.loads(row["extra_json"])
        except (TypeError, ValueError):
            extra = None
        seed_hash_match = False
        if not isinstance(extra, dict):
            failure = f"{slug}: extra_json is not an object"
            failures.append(failure)
            row_failures.append(failure)
        else:
            seed_hash_checks += 1
            seed_hash_match = extra.get("seed_hash") == expected["seed_hash"]
            if not seed_hash_match:
                failure = f"{slug}: release seed hash mismatch"
                failures.append(failure)
                row_failures.append(failure)
        geo_checks += 1
        geo_match = [row["lat"], row["lon"]] == expected["geo"]
        if not geo_match:
            failure = f"{slug}: coordinate pair mismatch"
            failures.append(failure)
            row_failures.append(failure)
        service_checks += 1
        service_match = (row["dropoff_json"] == expected["dropoff_json"]
                         and row["pickup_json"] == expected["pickup_json"]
                         and row["certifications"] == expected["certifications"])
        if not service_match:
            failure = f"{slug}: service JSON mismatch"
            failures.append(failure)
            row_failures.append(failure)
        row_results.append({
            "slug": slug,
            "ok": not row_failures,
            "scalar_checks": len(FIELDS),
            "notes_hash_match": notes_hash_match,
            "seed_hash_match": seed_hash_match,
            "geo_match": geo_match,
            "service_match": service_match,
            "failures": row_failures,
        })
    return {
        "ok": not failures,
        "release_commit": RELEASE_COMMIT,
        "release_tree": RELEASE_TREE,
        "checked_count": checked,
        "scalar_checks": scalar_checks,
        "notes_hash_checks": notes_checks,
        "seed_hash_checks": seed_hash_checks,
        "geo_checks": geo_checks,
        "service_checks": service_checks,
        "scope": manifest["scope"],
        "rows": row_results,
        "failures": sorted(failures),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--data-dir", type=Path, default=Path("/var/lib/schrott-mcp"))
    args = parser.parse_args()
    public = None
    try:
        public = connect(args.data_dir / "public.db")
        result = verify(public)
    except (OSError, sqlite3.Error, ValueError, KeyError, TypeError) as error:
        result = {"ok": False, "failures": [f"gate error: {error}"]}
    finally:
        if public is not None:
            public.close()
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
