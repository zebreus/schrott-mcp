#!/usr/bin/env python3
"""Seed-Splitter (Pilot): seed/traders/<state>.json -> dossiers/<state>/<slug>.md

Gegenstück zu tools/dossiers2seed.py. Format:
  - Frontmatter (YAML, flach): alle Seed-Skalare 1:1
    (slug, name, trader_type, state, city, street, postcode, phone, email,
    opening_hours, website, website_status, status, description,
    dropoff_json, pickup_json, provenance_seed_file, provenance_section,
    provenance_ankauf_raw, provenance_origin).
  - "# <Name>" als H1 (bei Handlern: kanonischer Handler-/Trader-Name).
  - "## Überblick": Lese-Ansicht aus description (Compiler ignoriert sie).
    Leer -> Platzhalter (Compiler-sicher, da ignoriert).
  - "## Timeline": notes gesplittet an " | " als Bullets unter
    "### Importiert (Seed-Stand <Datum>)", wortwörtlich, ohne Zusätze.
    Round-Trip: join(" | ") == Original-notes (s. dossiers2seed.py).

Gebrauch: python3 tools/seed2dossiers.py <state> [state ...]
  (lowercase, z.B. hb). Schreibt nur fehlende Dateien neu? Nein:
  überschreibt dossiers/<state>/*.md deterministisch aus dem Seed.
  Hand-kuratierte Überblicke danach nicht erneut splitten!
"""
import json
import sys
from datetime import date
from pathlib import Path

try:
    import yaml
except ImportError:
    yaml = None

ROOT = Path(__file__).resolve().parent.parent
SEED = ROOT / "seed" / "traders"
DOSSIERS = ROOT / "dossiers"

FM_KEYS = ("slug", "name", "trader_type", "state", "city", "street",
           "postcode", "phone", "email", "opening_hours", "website",
           "website_status", "status", "description", "dropoff_json",
           "pickup_json")


def to_dossier(row: dict, stand: str) -> str:
    fm = {k: row.get(k, "") for k in FM_KEYS}
    fm["provenance_seed_file"] = row.get("provenance", {}).get("seed_file", "")
    fm["provenance_section"] = row.get("provenance", {}).get("section", "")
    fm["provenance_ankauf_raw"] = row.get("provenance", {}).get("ankauf_raw", "")
    fm["provenance_origin"] = row.get("provenance", {}).get("origin", "")
    front = yaml.safe_dump(fm, allow_unicode=True, sort_keys=False,
                           default_flow_style=False)
    desc = (row.get("description") or "").strip()
    ueberblick = desc if desc else ("_Noch kein Überblick — bei nächster Welle "
                                    "aus description/notes kuratieren._")
    notes = (row.get("notes") or "").strip()
    if notes:
        bullets = "\n".join(f"- {b.strip()}"
                            for b in notes.split(" | ") if b.strip())
    else:
        bullets = "_Keine Einträge._"
    return (f"---\n{front}---\n\n# {row['name']}\n\n## Überblick\n\n"
            f"{ueberblick}\n\n## Timeline\n\n"
            f"### Importiert (Seed-Stand {stand})\n\n{bullets}\n")


def main() -> int:
    if yaml is None:
        print("FEHLER: pyyaml fehlt", file=sys.stderr)
        return 1
    states = [s.lower() for s in sys.argv[1:]]
    if not states:
        print("Gebrauch: python3 tools/seed2dossiers.py <state> ...")
        return 1
    stand = date.today().isoformat()
    total = 0
    for st in states:
        src = SEED / f"{st}.json"
        rows = json.loads(src.read_text(encoding="utf-8"))
        outdir = DOSSIERS / st
        outdir.mkdir(parents=True, exist_ok=True)
        for r in rows:
            (outdir / f"{r['slug']}.md").write_text(
                to_dossier(r, stand), encoding="utf-8")
        total += len(rows)
        print(f"{st}: {len(rows)} Dossiers geschrieben")
    print(f"TOTAL: {total}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
