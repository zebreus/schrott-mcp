#!/usr/bin/env python3
"""Build a release-specific state expectation file from a private rehearsal.

Scope: exactly the trader rows that the candidate release changes. For every
row the 11 documented scalars, the complete seeded Timeline notes hash, the
stored seed fingerprint and the coordinate pair are pinned, so the post-rollout
check can prove the effect of this release and not only an unrelated state.

Usage:
  python3 make_release_delta_expectations.py --rehearsal <public.db> \
      --probe-manifest <manifest.json> --release <commit> --tree <tree> \
      --out scripts/release-delta-<short>-expectations.json
"""
import argparse
import hashlib
import json
import sqlite3
from pathlib import Path

FIELDS = (
    "name", "street", "postcode", "phone", "email", "opening_hours",
    "description", "status", "trader_type", "website", "website_status",
)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rehearsal", type=Path, required=True)
    parser.add_argument("--probe-manifest", type=Path, required=True)
    parser.add_argument("--release", required=True)
    parser.add_argument("--tree", required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    manifest = json.loads(args.probe_manifest.read_text())
    changed = [row["slug"] for row in manifest["effective_delta"]["changed_rows"]]

    conn = sqlite3.connect(f"file:{args.rehearsal}?mode=ro", uri=True)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA query_only=ON")
    rows = []
    for slug in changed:
        row = conn.execute(
            "SELECT slug," + ",".join(FIELDS) + ",notes,extra_json,lat,lon,dropoff_json,"
            "pickup_json,certifications FROM traders WHERE slug=?", (slug,)).fetchone()
        if row is None:
            raise SystemExit(f"missing rehearsal row for {slug}")
        extra = json.loads(row["extra_json"])
        rows.append({
            "slug": slug,
            "values": {f: row[f] for f in FIELDS},
            "notes_sha256": hashlib.sha256((row["notes"] or "").encode()).hexdigest(),
            "seed_hash": extra["seed_hash"],
            "geo": [row["lat"], row["lon"]],
            "dropoff_json": row["dropoff_json"],
            "pickup_json": row["pickup_json"],
            "certifications": row["certifications"],
        })
    conn.close()

    out = {
        "version": 1,
        "release_commit": args.release,
        "release_tree": args.tree,
        "rehearsal_manifest_sha256": hashlib.sha256(args.probe_manifest.read_bytes()).hexdigest(),
        "fields": list(FIELDS),
        "scope": (
            f"exactly the {len(rows)} trader rows this release changes, taken from the "
            "effective delta of the private post-seed rehearsal"),
        "expectation_basis": (
            "Actual seed_metadata in the private post-seed rehearsal for this release, "
            f"{args.probe_manifest.parent}. Values: the 11 explicit scalars, the complete "
            "seeded Timeline notes hash, the stored seed_hash, the coordinate pair and the "
            "service JSON of every changed row. No production read, no deployment claim."),
        "traders": rows,
    }
    args.out.write_text(json.dumps(out, indent=2, ensure_ascii=False) + "\n")
    print(json.dumps({
        "out": str(args.out),
        "rows": len(rows),
        "manifest_sha256": hashlib.sha256(args.out.read_bytes()).hexdigest(),
        "geo_null_rows": sum(1 for r in rows if r["geo"][0] is None and r["geo"][1] is None),
        "geo_set_rows": sum(1 for r in rows if r["geo"][0] is not None),
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
