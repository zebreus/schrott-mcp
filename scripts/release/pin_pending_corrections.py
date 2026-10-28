#!/usr/bin/env python3
"""Pinnt das Gate `verify-pending-corrections.py` auf einen Release.

Das Gate prüft sechs Händlerzeilen (MSG-Tiers, Nordwest-Korrekturen, Gotha,
Schrott24) gegen den **Seed-Stand eines konkreten Releases**: Es erwartet für
jede Zeile den `seed_hash` und den Hash der gesetzten Notizen. Nach einem
Release, der Dossiers dieser Zeilen verändert hat, ist der alte Pin rot,
obwohl die Korrekturen selbst in der Produktion stehen.

Dieses Werkzeug erzeugt die release-spezifische Variante:

1. `scripts/pending-corrections-<short>-seed-expectations.json` - die erwarteten
   `seed_hash`/`notes_sha256` aus der **privaten Probe** des Releases (nie aus
   der Produktion).
2. `scripts/verify-pending-corrections-<short>.py` - eine Kopie des Gates mit
   den neuen Datei- und Digest-Konstanten.

Beispiel:
    python3 scripts/release/pin_pending_corrections.py \\
        --release <commit> --tree <tree> \\
        --rehearsal target/predeploy-<short>/data/public.db \\
        --manifest target/predeploy-<short>/predeploy-probe-manifest.json \\
        --short <short>
"""
import argparse
import hashlib
import json
import re
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GATE = ROOT / "scripts" / "verify-pending-corrections.py"


def gate_slugs() -> list:
    """Die vom Gate geprüften Slugs (CLEARS + GOTHA) aus dem Gate-Quelltext."""
    text = GATE.read_text(encoding="utf-8")
    clears = re.search(r"^CLEARS = \{(.*?)^\}", text, re.S | re.M)
    gotha = re.search(r'^GOTHA = "([^"]+)"', text, re.M)
    if not clears or not gotha:
        raise SystemExit("CLEARS/GOTHA im Gate nicht gefunden")
    slugs = re.findall(r'"([a-z0-9-]+)"\s*:', clears.group(1))
    slugs.append(gotha.group(1))
    return slugs


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--release", required=True)
    parser.add_argument("--tree", required=True)
    parser.add_argument("--rehearsal", type=Path, required=True)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--short", required=True)
    args = parser.parse_args()

    import sqlite3
    conn = sqlite3.connect(f"file:{args.rehearsal}?mode=ro", uri=True)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA query_only=ON")
    slugs = gate_slugs()
    traders = {}
    for slug in slugs:
        row = conn.execute("SELECT notes, extra_json FROM traders WHERE slug=?", (slug,)).fetchone()
        if row is None:
            raise SystemExit(f"Probe ohne Zeile {slug}")
        extra = json.loads(row["extra_json"])
        traders[slug] = {
            "seed_hash": extra["seed_hash"],
            "notes_sha256": hashlib.sha256(row["notes"].encode()).hexdigest(),
        }
    conn.close()

    manifest_sha = hashlib.sha256(args.manifest.read_bytes()).hexdigest()
    expectations = {
        "release_commit": args.release,
        "release_tree": args.tree,
        "rehearsal_manifest_sha256": manifest_sha,
        "traders": traders,
    }
    exp_path = ROOT / "scripts" / f"pending-corrections-{args.short}-seed-expectations.json"
    exp_path.write_text(json.dumps(expectations, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

    text = GATE.read_text(encoding="utf-8")
    text = re.sub(r'SEED_EXPECTATIONS = Path\(__file__\)\.with_name\(\s*"[^"]+"\s*\)',
                   f'SEED_EXPECTATIONS = Path(__file__).with_name(\n    "pending-corrections-{args.short}-seed-expectations.json"\n)', text)
    text = re.sub(r'SEED_RELEASE_COMMIT = "[^"]*"', f'SEED_RELEASE_COMMIT = "{args.release}"', text)
    text = re.sub(r'SEED_RELEASE_TREE = "[^"]*"', f'SEED_RELEASE_TREE = "{args.tree}"', text)
    text = re.sub(r'SEED_REHEARSAL_MANIFEST_SHA256 = "[^"]*"',
                   f'SEED_REHEARSAL_MANIFEST_SHA256 = "{manifest_sha}"', text)
    gate_path = ROOT / "scripts" / f"verify-pending-corrections-{args.short}.py"
    gate_path.write_text(text, encoding="utf-8")

    # Selbstprüfung: das neue Gate muss die Erwartungen laden können
    import importlib.util
    spec = importlib.util.spec_from_file_location("pinned_pending", gate_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    rows = module.load_seed_expectations()
    if set(rows) != set(traders):
        raise SystemExit("Selbstprüfung fehlgeschlagen: Slug-Menge weicht ab")
    print(json.dumps({
        "expectations": str(exp_path.relative_to(ROOT)),
        "gate": str(gate_path.relative_to(ROOT)),
        "rows": len(rows),
        "selfcheck": "ok",
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
