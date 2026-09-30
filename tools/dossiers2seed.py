#!/usr/bin/env python3
"""Dossier-Compiler (SKIZZE, Prototyp-Stand 30.09.2026).

Richtung: dossiers/<state>/<slug>.md  ->  seed/traders/<state>.json

Design-Regeln (Vorschlag):
  1. Frontmatter = Source of Truth fuer ALLE Skalare (inkl. description).
     Keys 1:1 wie SeedTrader (slug, name, trader_type, state, city, street,
     postcode, phone, email, opening_hours, website, website_status, status,
     description, dropoff_json, pickup_json, provenance_*).
  2. ## Ueberblick = Lese-Ansicht (wird beim Split aus description/notes
     generiert, danach frei editierbar als Arbeitsnotiz). Vom Compiler
     IGNORIERT - kein Dual-Source fuer description.
  3. ## Timeline = Source of Truth fuer notes. Alle '- '-Bullets unter
     '## Timeline' (bis zur naechsten '## '-Heading) werden mit ' | '
     gejoint, Whitespace kollabiert, auf 2000 Zeichen gekuerzt
     (wie md2seed.py: notes[:2000]). Wachstum = hinten anhaengen, nie
     umschreiben. Jeder Bullet traegt seine Quelle im Text
     ('[Quelle: ...]' / '[Recherche DD.MM.YYYY: ...; Quelle: ...]').
  4. md2seed.py-Kompatibilitaet: md2seed.py darf Slugs mit existierendem
     Dossier nicht mehr anfassen (Guard: Dossier-File existiert -> Zeile
     ueberspringen, PRESERVE_KEYS greift dort nicht). Dossier-Compiler
     updated seed-Zeilen in-place by slug; Zeilen ohne Dossier bleiben
     byte-identisch. Damit ist die Einfuehrung inkrementell.

Gebrauch: python3 tools/dossiers2seed.py [--write]
  Default = Dry-Run (diff zaehlen, nichts schreiben).
"""
import json
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:  # pragma: no cover
    yaml = None

ROOT = Path(__file__).resolve().parent.parent
DOSSIERS = ROOT / "dossiers"
OUT = ROOT / "seed" / "traders"

FRONTMATTER = re.compile(r"^---\n(.*?)\n---\n(.*)$", re.S)


def parse_dossier(path: Path) -> dict:
    text = path.read_text(encoding="utf-8")
    m = FRONTMATTER.match(text)
    if not m:
        raise ValueError(f"{path}: kein Frontmatter (--- ... ---)")
    raw_fm, body = m.group(1), m.group(2)
    if yaml is None:
        raise ValueError("pyyaml fehlt (python3 -c 'import yaml')")
    fm = yaml.safe_load(raw_fm) or {}
    # Timeline-Bullets extrahieren
    notes = ""
    tl = re.search(r"^## Timeline\s*\n(.*?)(?=^## \S|\Z)", body, re.M | re.S)
    if tl:
        bullets = []
        for line in tl.group(1).splitlines():
            s = line.strip()
            if s.startswith("- "):
                bullets.append(re.sub(r"\s+", " ", s[2:].strip()))
        notes = " | ".join(bullets)[:2000]
    # H1-Konsistenz (Warnung, kein Fehler)
    h1 = re.search(r"^# (.+)$", body, re.M)
    if h1 and fm.get("name") and h1.group(1).strip() != str(fm["name"]).strip():
        print(f"  warn: H1 != name in {path.name}", file=sys.stderr)
    return {
        "slug": str(fm.get("slug", "")),
        "name": str(fm.get("name", "")),
        "trader_type": str(fm.get("trader_type", "")),
        "description": str(fm.get("description", "") or ""),
        "street": str(fm.get("street", "") or ""),
        "postcode": str(fm.get("postcode", "") or ""),
        "phone": str(fm.get("phone", "") or ""),
        "email": str(fm.get("email", "") or ""),
        "opening_hours": str(fm.get("opening_hours", "") or ""),
        "city": str(fm.get("city", "")),
        "state": str(fm.get("state", "")),
        "website": str(fm.get("website", "") or ""),
        "website_status": str(fm.get("website_status", "") or ""),
        "dropoff_json": str(fm.get("dropoff_json", "") or ""),
        "pickup_json": str(fm.get("pickup_json", "") or ""),
        "status": str(fm.get("status", "")),
        "notes": notes,
        "provenance": {
            "seed_file": str(fm.get("provenance_seed_file", "") or ""),
            "section": str(fm.get("provenance_section", "") or ""),
            "ankauf_raw": str(fm.get("provenance_ankauf_raw", "") or ""),
            "origin": str(fm.get("provenance_origin", "") or ""),
        },
    }


def main() -> int:
    write = "--write" in sys.argv
    files = sorted(DOSSIERS.glob("*/*.md"))
    if not files:
        print("keine Dossiers gefunden.")
        return 0
    # Gruppieren nach state (Frontmatter, fallback: Verzeichnisname)
    by_state: dict[str, list[dict]] = {}
    for f in files:
        d = parse_dossier(f)
        state = (d["state"] or f.parent.name).lower()
        by_state.setdefault(state, []).append(d)
    total_changed = 0
    for state, rows in sorted(by_state.items()):
        src = OUT / f"{state}.json"
        if not src.exists():
            print(f"{state}: {src} fehlt - SKIP ({len(rows)} Dossiers)")
            continue
        seed = json.loads(src.read_text(encoding="utf-8"))
        idx = {r["slug"]: i for i, r in enumerate(seed)}
        changed = 0
        for d in rows:
            i = idx.get(d["slug"])
            if i is None:
                print(f"  neu (noch nicht im Seed): {d['slug']} - SKIP (nur Update, kein Append)")
                continue
            old = seed[i]
            # In-place Update: Skalare aus Frontmatter, notes aus Timeline
            new = dict(old)
            for k in ("name", "trader_type", "description", "street", "postcode",
                      "phone", "email", "opening_hours", "city", "state",
                      "website", "website_status", "dropoff_json", "pickup_json",
                      "status", "notes", "provenance"):
                new[k] = d[k]
            if new != old:
                changed += 1
                if write:
                    seed[i] = new
                else:
                    for k in new:
                        if new[k] != old.get(k):
                            print(f"  diff {d['slug']}.{k}: {str(old.get(k))[:60]!r} -> {str(new[k])[:60]!r}")
        print(f"{state}: {len(rows)} Dossiers, {changed} Zeilen wuerden sich aendern")
        total_changed += changed
        if write and changed:
            src.write_text(json.dumps(seed, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
            print(f"  -> {src} geschrieben")
    print(f"TOTAL: {total_changed} Aenderungen ({'geschrieben' if write else 'dry-run'})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
