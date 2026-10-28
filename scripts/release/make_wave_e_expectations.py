#!/usr/bin/env python3
"""Build a release-pinned Wave E expectation manifest from a private rehearsal.

Source of truth: the private post-seed rehearsal copy of the candidate release
plus the preserved historical Wave E scope (same 60 rows, same 11 scalars).
Only the Timeline hash and the stored seed fingerprint are re-pinned, because a
GEO clear or a new Timeline note changes the seed payload. Nothing here reads
production.

Usage:
  python3 make_wave_e_expectations.py --rehearsal <public.db> \
      --probe-manifest <manifest.json> --release <commit> --tree <tree> \
      --out scripts/wave-e-<short>-seed-expectations.json \
      [--base scripts/wave-e-6b3da663-seed-expectations.json]
"""
import argparse
import hashlib
import json
import sqlite3
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--rehearsal", type=Path, required=True)
    parser.add_argument("--probe-manifest", type=Path, required=True)
    parser.add_argument("--release", required=True)
    parser.add_argument("--tree", required=True)
    parser.add_argument("--out", type=Path, required=True)
    # Basis der Wave-E-Scope: die am 27.10.2026 revidierte Fassung (ein
    # Dossier der Scope wurde nach belegter Recherche verbessert; Begründung
    # in research/wave-e-20261027-owner.md).
    parser.add_argument("--base", type=Path,
                        default=Path("scripts/wave-e-20261028-0600-expectations.json"))
    args = parser.parse_args()

    base = json.loads(args.base.read_text())
    # eigene Kopie über die SQLite-Backup-API: so bleibt das Bundle byte-stabil
    # und der WAL-Inhalt der Quelle geht nicht verloren
    import tempfile
    copy = Path(tempfile.mkdtemp(dir="/tmp/opencode")) / "rehearsal.db"
    source = sqlite3.connect(f"file:{args.rehearsal}?mode=ro", uri=True)
    source.execute("PRAGMA query_only=ON")
    target = sqlite3.connect(copy)
    source.backup(target)
    target.close()
    source.close()
    conn = sqlite3.connect(f"file:{copy}?mode=ro", uri=True)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA query_only=ON")

    rows = []
    changed = []
    for entry in base["traders"]:
        slug = entry["slug"]
        row = conn.execute(
            "SELECT notes, extra_json FROM traders WHERE slug=?", (slug,)).fetchone()
        if row is None:
            raise SystemExit(f"missing rehearsal row for {slug}")
        notes_hash = hashlib.sha256(row["notes"].encode()).hexdigest()
        seed_hash = json.loads(row["extra_json"])["seed_hash"]
        if notes_hash != entry["notes_sha256"] or seed_hash != entry["seed_hash"]:
            changed.append({
                "slug": slug,
                "old_notes_sha256": entry["notes_sha256"],
                "new_notes_sha256": notes_hash,
                "old_seed_hash": entry["seed_hash"],
                "new_seed_hash": seed_hash,
            })
        rows.append({
            "slug": slug,
            "values": entry["values"],
            "notes_sha256": notes_hash,
            "seed_hash": seed_hash,
        })
    conn.close()

    manifest = {
        "version": 1,
        "release_commit": args.release,
        "release_tree": args.tree,
        "rehearsal_manifest_sha256": hashlib.sha256(args.probe_manifest.read_bytes()).hexdigest(),
        "historical_selection_sha256": base["historical_selection_sha256"],
        "fields": base["fields"],
        "scope": base["scope"],
        "expectation_basis": (
            "Actual seed_metadata in the private post-seed rehearsal for release "
            f"{args.release[:7]} ({args.probe_manifest.parent.resolve()}), 2026-10-27. Values: the 11 "
            "explicit scalars of the preserved Wave E scope, the complete seeded Timeline "
            "notes hash and the stored seed_hash of each row. No production read, no GEO, "
            "source or deployment claim."),
        "traders": rows,
    }
    args.out.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + "\n")
    print(json.dumps({
        "out": str(args.out),
        "rows": len(rows),
        "rehearsal_manifest_sha256": manifest["rehearsal_manifest_sha256"],
        "manifest_sha256": hashlib.sha256(args.out.read_bytes()).hexdigest(),
        "changed_rows": changed,
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
