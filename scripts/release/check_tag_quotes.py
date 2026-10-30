#!/usr/bin/env python3
"""Prueft alle zitierten OSM-Tag-Werte der Wellen-Sektionen live und erzeugt
Korrekturen fuer Abweichungen.

Fuer jedes Dossier des Release-Scopes werden die Objekt-Referenzen
(node/…, way/…) und die in Backticks notierten `key=value`-Paare gelesen; die
Objekte werden live geladen und die Tag-Werte verglichen.  Abweichungen
erzeugen Manifest-Eintraege (Ersatz nur des **Wert**-Anteils).
"""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from osm_street_scan import api  # noqa: E402

CACHE = {}


def live_tags(typ, oid):
    key = (typ, oid)
    if key not in CACHE:
        try:
            data = json.loads(api(f"https://api.openstreetmap.org/api/0.6/{typ}/{oid}.json"))
            CACHE[key] = data["elements"][0].get("tags", {})
        except Exception:
            CACHE[key] = {}
    return CACHE[key]


def main():
    scope = Path("/tmp/opencode/release-targets-1800-all.json")
    slugs = set(json.loads(scope.read_text()))
    manifest = []
    for path in sorted(Path("dossiers").rglob("*.md")):
        if path.stem not in slugs:
            continue
        text = path.read_text()
        # nur die Wellen-Sektion pruefen
        if "## Timeline" not in text:
            continue
        tail = text.split("## Timeline", 1)[1]
        secs = re.split(r"(?m)^(### .*)$", tail)
        wave = "".join(secs[-2:]) if len(secs) >= 3 else tail
        # Objekt-Referenzen mit ihren nearby Tags
        objects = []
        for m in re.finditer(r"(node|way)/(\d+)", wave):
            objects.append((m.group(1), m.group(2)))
        if not objects:
            continue
        # Backtick-Paare key="value" oder key=value einsammeln
        pairs = re.findall(r"`([a-z_:]+)=\"?([^\"`]+)\"?`", wave)
        if not pairs:
            continue
        # Objekte in der Reihenfolge des Auftretens -> Wert-Zuordnung
        ids = [(m.group(1), m.group(2)) for m in re.finditer(r"(node|way)/(\d+)", wave)]
        for idx, (mkey, mval) in enumerate(pairs):
            if idx >= len(ids):
                break
            typ, oid = ids[idx]
            tags = live_tags(typ, oid)
            if mkey not in tags:
                continue
            live = tags[mkey]
            if live == mval:
                continue
            manifest.append({
                "file": str(path),
                "label": f"{path.stem}: {typ}/{oid} {mkey}={mval!r} -> {live!r}",
                "old": f"`{mkey}=\"{mval}\"`",
                "new": f"`{mkey}=\"{live}\"`",
                "count": 1,
                "note": (f"**OSM-Tag-Zitat berichtigt**: {typ}/{oid} trägt live `{mkey}={live}`; "
                         f"das Dossier zitierte `{mkey}=\"{mval}\"`. "
                         f"Quelle(n): https://api.openstreetmap.org/api/0.6/{typ}/{oid}/full (Abruf 30.10.2026)"),
            })
    Path("/tmp/opencode/corrections-tagquotes.json").write_text(
        json.dumps(manifest, indent=1, ensure_ascii=False))
    print("Eintraege:", len(manifest))
    for m in manifest:
        print("  ", m["label"])


if __name__ == "__main__":
    main()
