#!/usr/bin/env python3
"""Read-only Wave E state check pinned to the reviewed release.

This supplements, but does not alter, the historical 2026-10-11 verifier and
manifest. It checks 60 rows' 11 explicit scalar values, full Timeline hashes,
and stored seed fingerprints against the exact private post-seed rehearsal.
It does not validate GEO, source truth, JSON/service claims, or deployment
identity. Use the deployment journal separately to establish which release
was installed and that the rollout occurred.
"""
import argparse
import hashlib
import json
import re
import sqlite3
from pathlib import Path

MANIFEST = Path(__file__).with_name("wave-e-0a41145-seed-expectations.json")
HISTORICAL_MANIFEST = Path(__file__).with_name("wave-e-20261011-expectations.json")
MANIFEST_SHA256 = "41c051a7fd90e58a95055ad1597f70597a25e2940a01deeb7ec26af49713029c"
HISTORICAL_MANIFEST_SHA256 = "871453ff9f2b54e262a93212c83e42788b29a82a6e2b05b1876300d74768787f"
RELEASE_COMMIT = "0a411455620f075b917c1e30fdbd87e137671496"
RELEASE_TREE = "1155ab5a0e9ca6643d8ebdd12c1d6dd0874699b6"
REHEARSAL_MANIFEST_SHA256 = "5f490d12ca92304a8916ca6d88b782eb661f0b04213c91a5f75e156bd6fd7b74"
HISTORICAL_SELECTION_SHA256 = "c1260bbc4c4ba1cc7c7f7e3e6fb09227a9a7ce9e935bbe75bb26dff533646f03"
FIELDS = (
    "name", "street", "postcode", "phone", "email", "opening_hours",
    "description", "status", "trader_type", "website", "website_status",
)


def connect(path):
    db = sqlite3.connect(path.resolve().as_uri() + "?mode=ro", uri=True)
    db.row_factory = sqlite3.Row
    db.execute("PRAGMA query_only=ON")
    return db


def load_historical_scope():
    raw = HISTORICAL_MANIFEST.read_bytes()
    if hashlib.sha256(raw).hexdigest() != HISTORICAL_MANIFEST_SHA256:
        raise ValueError("preserved historical manifest digest mismatch")
    manifest = json.loads(raw)
    rows = manifest.get("traders")
    if (manifest.get("selection_sha256") != HISTORICAL_SELECTION_SHA256
            or not isinstance(rows, list)
            or len(rows) != 60
            or any(not isinstance(row, dict) or not isinstance(row.get("slug"), str)
                   for row in rows)):
        raise ValueError("preserved historical Wave E scope is invalid")
    slugs = [row["slug"] for row in rows]
    if len(set(slugs)) != 60:
        raise ValueError("preserved historical Wave E scope has duplicate slugs")
    return set(slugs)


def validate_manifest(manifest, historical_slugs=None):
    required = {
        "version", "release_commit", "release_tree", "rehearsal_manifest_sha256",
        "historical_selection_sha256", "fields", "scope", "expectation_basis", "traders",
    }
    if not isinstance(manifest, dict) or set(manifest) != required:
        raise ValueError("release expectation manifest has an invalid shape")
    if (type(manifest["version"]) is not int or manifest["version"] != 1
            or manifest["release_commit"] != RELEASE_COMMIT
            or manifest["release_tree"] != RELEASE_TREE
            or manifest["rehearsal_manifest_sha256"] != REHEARSAL_MANIFEST_SHA256
            or manifest["historical_selection_sha256"] != HISTORICAL_SELECTION_SHA256):
        raise ValueError("release expectation identity does not match reviewed release")
    if manifest["fields"] != list(FIELDS):
        raise ValueError("release expectation fields do not match reviewed scope")
    if (not isinstance(manifest["scope"], str) or not manifest["scope"].strip()
            or not isinstance(manifest["expectation_basis"], str)
            or not manifest["expectation_basis"].strip()):
        raise ValueError("release expectation scope/basis is missing")
    rows = manifest["traders"]
    if not isinstance(rows, list) or len(rows) != 60:
        raise ValueError("release expectation must contain exactly 60 traders")
    slugs = []
    for row in rows:
        if not isinstance(row, dict) or set(row) != {"slug", "values", "notes_sha256", "seed_hash"}:
            raise ValueError("invalid release expectation row")
        slug = row["slug"]
        values = row["values"]
        if (not isinstance(slug, str) or not slug
                or not isinstance(values, dict) or set(values) != set(FIELDS)
                or any(not isinstance(value, str) for value in values.values())
                or not isinstance(row["notes_sha256"], str)
                or not re.fullmatch(r"[0-9a-f]{64}", row["notes_sha256"])
                or not isinstance(row["seed_hash"], str)
                or not re.fullmatch(r"[0-9a-f]{16}", row["seed_hash"])):
            raise ValueError("invalid release expectation for " + str(slug))
        slugs.append(slug)
    if len(set(slugs)) != 60:
        raise ValueError("release expectation has duplicate trader slugs")
    if historical_slugs is None:
        historical_slugs = load_historical_scope()
    if set(slugs) != historical_slugs:
        raise ValueError("release expectation slug scope differs from preserved selection")
    return rows


def load_manifest(path=None):
    path = MANIFEST if path is None else path
    raw = path.read_bytes()
    if path.resolve() == MANIFEST.resolve() and hashlib.sha256(raw).hexdigest() != MANIFEST_SHA256:
        raise ValueError("release expectation manifest digest mismatch")
    manifest = json.loads(raw)
    validate_manifest(manifest)
    return manifest


def verify(public, manifest=None):
    manifest = load_manifest() if manifest is None else manifest
    expected_rows = validate_manifest(manifest)
    failures = []
    checked = 0
    scalar_checks = 0
    notes_checks = 0
    seed_hash_checks = 0
    row_results = []
    columns = ",".join(FIELDS)
    for expected in expected_rows:
        slug = expected["slug"]
        row_failures = []
        rows = public.execute(
            f"SELECT slug,{columns},notes,extra_json FROM traders WHERE slug=?", (slug,)
        ).fetchall()
        if len(rows) != 1:
            failure = f"{slug}: expected one trader, found {len(rows)}"
            failures.append(failure)
            row_results.append({
                "slug": slug, "ok": False, "scalar_checks": 0,
                "notes_hash_match": False, "seed_hash_match": False,
                "failures": [failure],
            })
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
        row_results.append({
            "slug": slug,
            "ok": not row_failures,
            "scalar_checks": len(FIELDS),
            "notes_hash_match": notes_hash_match,
            "seed_hash_match": seed_hash_match,
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
        "scope": "60 pinned Wave E traders; 11 scalars, complete Timeline hashes, and stored seed_hash only; no GEO/source/deployment validation",
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
