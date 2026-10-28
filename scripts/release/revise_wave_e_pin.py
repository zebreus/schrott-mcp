#!/usr/bin/env python3
"""Revidiert die Wave-E-Pin-Basis nach einer Welle, die Scope-Zeilen verbessert hat.

Der Wave-E-Pin ist ein **Regressionsschutz** über die elf Skalare des ursprünglichen
Wave-E-Scopes: Eine spätere Welle darf diese Werte nur mit einer ausdrücklichen
Owner-Revision ändern. Dieses Werkzeug schreibt die Revision **mechanisch** und
prüft sie danach selbst:

1. Es kopiert die aktuelle Basis (Text-patch, damit alle nicht betroffenen Zeilen
   byte-identisch bleiben).
2. Es übernimmt für die benannten Slugs die Werte aus der privaten Probe (Post-Seed).
3. Es hebt `notes_sha256` und `seed_hash` **aller** Zeilen auf den Post-Seed-Stand.
4. Es ersetzt den `revision`-Block und prüft zum Schluss gegen die Probe, dass alle
   60 Hashes und alle Scope-Werte stimmen.

Beispiel:
    python3 scripts/release/revise_wave_e_pin.py \\
        --base scripts/wave-e-20261028-expectations.json \\
        --out scripts/wave-e-20261028-0600-expectations.json \\
        --rehearsal target/predeploy-f1fe34d/data/public.db \\
        --release <commit> --tree <tree> \\
        --revision "Welle 06:00" --owner-review research/wave-e-20261028-0600-owner.md \\
        --row by-neustadt-a-d-aisch-stirling-industrie-metallhandel
"""
import argparse
import hashlib
import json
import re
import sqlite3
from pathlib import Path

TRADERS_MARKER = '  "traders": ['


def split_scope(text: str):
    """Text in Kopf (Metadaten) und Traders-Liste zerlegen.

    Ein Slug kann auch im `revision`-Block vorkommen - die Suche muss deshalb
    auf die Traders-Liste begrenzt bleiben.
    """
    head, tail = text.split(TRADERS_MARKER, 1)
    return head, tail


def patch_values(text: str, slug: str, values: dict, fields: list) -> str:
    head, tail = split_scope(text)
    start = tail.index('"slug": "%s"' % slug)
    end = tail.index('"notes_sha256"', start)
    block = tail[start:end]
    for field in fields:
        pattern = re.compile(r'("%s": )("(?:[^"\\]|\\.)*")' % re.escape(field))
        if not pattern.search(block):
            raise SystemExit(f"Feld {field} fehlt im Eintrag {slug}")
        block = pattern.sub(
            lambda m: m.group(1) + json.dumps(str(values[field]), ensure_ascii=False),
            block,
            count=1,
        )
    return head + TRADERS_MARKER + tail[:start] + block + tail[end:]


def lift_hashes(text: str, hashes: dict) -> str:
    head, tail = split_scope(text)
    for slug, (notes_sha, seed_hash) in hashes.items():
        i = tail.index('"slug": "%s"' % slug)
        nxt = tail.find('"slug": "', i + 10)
        end = nxt if nxt > 0 else len(tail)
        entry = tail[i:end]
        entry = re.sub(r'"notes_sha256": "[0-9a-f]{64}"',
                       '"notes_sha256": "%s"' % notes_sha, entry, count=1)
        entry = re.sub(r'"seed_hash": "[0-9a-f]+"',
                       '"seed_hash": "%s"' % seed_hash, entry, count=1)
        tail = tail[:i] + entry + tail[end:]
    return head + TRADERS_MARKER + tail


def replace_metadata(text: str, meta: dict) -> str:
    lines = text.split("\n")
    i0 = next(i for i, l in enumerate(lines) if l.startswith('  "dossier_revision":'))
    i1 = next(i for i, l in enumerate(lines) if l.startswith(TRADERS_MARKER))
    chunks = []
    for key, value in meta.items():
        dumped = json.dumps({key: value}, indent=2, ensure_ascii=False)
        chunks.append("  " + "\n".join(line[2:] for line in dumped.split("\n")[1:-1]))
    return "\n".join(lines[:i0] + (",\n".join(chunks) + ",").split("\n") + lines[i1:])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--rehearsal", type=Path, required=True)
    parser.add_argument("--release", required=True)
    parser.add_argument("--tree", required=True)
    parser.add_argument("--revision", required=True)
    parser.add_argument("--owner-review", required=True)
    parser.add_argument("--row", action="append", default=[],
                        help="Slug, dessen Scope-Werte aus der Probe übernommen werden")
    args = parser.parse_args()

    conn = sqlite3.connect(f"file:{args.rehearsal}?mode=ro", uri=True)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA query_only=ON")

    base = json.loads(args.base.read_text(encoding="utf-8"))
    scope = [row["slug"] for row in base["traders"]]
    hashes = {}
    for slug in scope:
        row = conn.execute("SELECT notes, extra_json FROM traders WHERE slug=?", (slug,)).fetchone()
        if row is None:
            raise SystemExit(f"probe ohne Zeile {slug}")
        hashes[slug] = (
            hashlib.sha256(row["notes"].encode()).hexdigest(),
            json.loads(row["extra_json"])["seed_hash"],
        )

    text = args.base.read_text(encoding="utf-8")
    rows_meta = []
    for slug in args.row:
        row = conn.execute("SELECT * FROM traders WHERE slug=?", (slug,)).fetchone()
        if row is None:
            raise SystemExit(f"probe ohne Zeile {slug}")
        values = {k: row[k] for k in row.keys()}
        old = next(r for r in base["traders"] if r["slug"] == slug)["values"]
        changed = [f for f in values if f in old and str(old[f]) != str(values[f])]
        text = patch_values(text, slug, values, changed)
        rows_meta.append({
            "slug": slug,
            "fields": sorted(changed),
            "reason": "Belegt in der Welle vom 28.10.2026 (06:00) verbessert; siehe "
                      f"{args.owner_review}.",
        })
    text = lift_hashes(text, hashes)
    text = replace_metadata(text, {
        "dossier_revision": args.release[:7],
        "owner_review": args.owner_review,
        "revision": {
            "revised_on": "2026-10-28",
            "revised_by": f"Owner ({args.revision})",
            "scope": f"{len(rows_meta)} Zeile(n) der Wave-E-Scope geändert; die Hashes aller "
                     f"{len(scope)} Zeilen auf den Post-Seed-Stand des Releases "
                     f"{args.release[:7]} gehoben",
            "rows": rows_meta,
            "unchanged_rows": len(scope) - len(rows_meta),
        },
    })
    args.out.write_text(text, encoding="utf-8")

    manifest = json.loads(args.out.read_text(encoding="utf-8"))
    if len(manifest["traders"]) != len(scope):
        raise SystemExit("Zeilenzahl der Basis stimmt nicht")
    bad = []
    for row in manifest["traders"]:
        r = conn.execute("SELECT notes, extra_json, * FROM traders WHERE slug=?",
                         (row["slug"],)).fetchone()
        if hashlib.sha256(r["notes"].encode()).hexdigest() != row["notes_sha256"]:
            bad.append((row["slug"], "notes"))
        if json.loads(r["extra_json"])["seed_hash"] != row["seed_hash"]:
            bad.append((row["slug"], "seed_hash"))
        for key, expected in row["values"].items():
            if str(r[key]) != str(expected):
                bad.append((row["slug"], key))
    conn.close()
    if bad:
        raise SystemExit(f"Selbstprüfung fehlgeschlagen: {bad[:5]}")
    print(json.dumps({
        "out": str(args.out),
        "rows": len(manifest["traders"]),
        "revised": [m["slug"] for m in rows_meta],
        "selfcheck": "ok",
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
