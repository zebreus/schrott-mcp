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

MANIFEST = Path(__file__).with_name("release-delta-ae24931-seed-expectations.json")
MANIFEST_SHA256 = "ae5055310e7c3c7c017f47e026b3b7a7b0a219bfd494481c32078b4ee6d753b7"
RELEASE_COMMIT = "ae24931c8ea96f53d4188bff84e8b333567c952e"
RELEASE_TREE = "2ba70dffb5b05d12d5e511fd8e8c2e6a569927e5"
REHEARSAL_MANIFEST_SHA256 = "f2bb030c6f7d2748b435a2126e078921b6ed539d6bcbd93f6d0628498f593e5b"
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
