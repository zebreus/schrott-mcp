#!/usr/bin/env python3
"""Erzeugt Korrekturen fuer transliterierte OSM-Tagwerte (ss statt ß).

Fuer jedes Dossier werden die zitierten Objekte live geladen; die im Text
verwendeten Tag-Werte werden durch die **live** geschriebenen ersetzt.
"""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from osm_street_scan import api  # noqa: E402

FILES = [
    "by-bergrheinfeld-97493-autoverwertung-schrotthandel-gazoz",
    "by-lohr-steinbach-97816-autoverwertung-pujdak",
    "by-uffenheim-schneck",
    "nw-bielefeld-sennestadt-eiles",
    "nw-duisburg-baykal",
    "nw-hagen-schellhas",
    "nw-wurselen-schafer",
    "st-gardelegen-schrottplatz-gardelegen",
    "st-wanzleben-lauenroth-gustav-fuhrunternehmen",
]
KEYS = ("addr:street", "addr:city", "name", "operator")


def live_tags(typ, oid):
    try:
        data = json.loads(api(f"https://api.openstreetmap.org/api/0.6/{typ}/{oid}.json"))
        return data["elements"][0].get("tags", {})
    except Exception:
        return {}


def main():
    manifest = []
    for slug in FILES:
        paths = list(Path("dossiers").rglob(f"{slug}.md"))
        if not paths:
            print(f"  !! {slug} nicht gefunden")
            continue
        rel = str(paths[0])
        text = paths[0].read_text()
        ids = []
        for m in re.finditer(r"(node|way)/(\d+)", text):
            key = (m.group(1), m.group(2))
            if key not in ids:
                ids.append(key)
        live = {}
        for typ, oid in ids:
            tags = live_tags(typ, oid)
            for k in KEYS:
                v = tags.get(k)
                if v and ("ß" in v or "Sued" in v or "Boerde" in v):
                    live[(k, v)] = v
        # Ersetzungen: der transliterierte Wert -> der live geschriebene
        pairs = {}
        for (k, v), liv in live.items():
            # Transliteration: ß -> ss, Sued -> Süd, Boerde -> Börde
            for form in (v, v.replace("ß", "ss"), v.replace("Sued", "Süd"),
                         v.replace("Boerde", "Börde")):
                if form in text and form != liv:
                    pairs[form] = liv
        if not pairs:
            continue
        for old, new in pairs.items():
            n = text.count(old)
            manifest.append({
                "file": rel,
                "label": f"{slug}: OSM-Tagwert transliteriert ({old} -> {new})",
                "old": old,
                "new": new,
                "count": n,
                "note": ("**OSM-Tag-Zitat berichtigt**: die im Dossier zitierte Tag-Schreibweise "
                         f"„{old}“ ist eine Transliteration; das Objekt trägt live **„{new}“**. "
                         "Quelle(n): https://api.openstreetmap.org/api/0.6/ (Abruf 30.10.2026)"),
            })
            text = text.replace(old, new)
    Path("/tmp/opencode/corrections-transliteration.json").write_text(
        json.dumps(manifest, indent=1, ensure_ascii=False))
    print("Eintraege:", len(manifest))
    for m in manifest:
        print(f'  {m["count"]}x  {m["label"]}')


if __name__ == "__main__":
    main()
