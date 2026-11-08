#!/usr/bin/env python3
"""OSM-Adressobjekte per OSM-API (map.json) im Umfeld der Dossier-Anschrift.

Read-only. Der Ausschnitt ist ein Kachelraster um den geokodierten Punkt; es
wird nur die Lage der Adressobjekte gelesen (Hausnummer, PLZ, Objektart), kein
Betriebs- oder Eintragsbeleg.
"""
import json
import sys
import time
import urllib.request

UA = "schrott-mcp-research/1.0 (dossier audit 2026-11-08)"

# slug -> (BBox lon/lat, gesuchte Hausnummer, Strassenname, PLZ, Ort)
PLACES = [
    ("he-messel-64409-schrottler-eddie", (8.7407, 49.9193, 8.7507, 49.9293), "23a", "Zeilharder", "64409"),
    ("he-offenbach-am-main-fred-de-snoo-rohstoffe-schrott-und-metal", (8.7969, 50.1027, 8.8069, 50.1127), "158", "Lämmerspieler", "63075"),
    ("he-limburg-65549-scheuer", (8.0479, 50.3952, 8.0579, 50.4052), "1", "Im Elbboden", "65549"),
    ("he-hanau-63452-guth", (8.9348, 50.1446, 8.9448, 50.1546), "32", "Niddastraße", "63452"),
    ("he-hanau-lamboy-schrott-und-metalle-reinhardt", (8.9222, 50.1387, 8.9322, 50.1487), "5", "Friedrich-Engels", "63452"),
    ("he-wiesbaden-biebrich-wiesbaden-dotzheim-bert-brecht-str-26", (8.1900, 50.0528, 8.2000, 50.0628), "26", "Bert-Brecht", "65201"),
    ("he-schaafheim-64850-schrenk", (8.9934, 49.9259, 9.0034, 49.9359), "6", "Daimlerstraße", "64850"),
    ("he-oberursel-61440-lackner", (8.5814, 50.1884, 8.5914, 50.1984), "13", "Pfeiffstraße", "61440"),
    ("he-wiesbaden-erbenheim-scholz-rico-schrott", (8.3035, 50.0474, 8.3135, 50.0574), "1", "Rennbahnstraße", "65205"),
    ("he-langenselbold-63505-hontsch-maria", (9.0246, 50.1667, 9.0346, 50.1767), "66b", "Hanauer", "63505"),
    ("he-wiesbaden-65199-hubinger", (8.2085, 50.0661, 8.2185, 50.0761), "48a", "Holzstraße", "65197"),
    ("he-babenhausen-64832-kohlmann-frank", (8.9593, 49.9700, 8.9693, 49.9800), "8", "Am Fuchsberg", "64832"),
]


def fetch(bbox):
    url = ("https://api.openstreetmap.org/api/0.6/map.json?bbox="
           f"{bbox[0]},{bbox[1]},{bbox[2]},{bbox[3]}")
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=180) as r:
        return json.loads(r.read().decode("utf-8"))


def main():
    out = {}
    for slug, bbox, hnr, street, pc in PLACES:
        try:
            d = fetch(bbox)
        except Exception as e:  # noqa: BLE001
            print(f"{slug}: FEHLER {e}", file=sys.stderr)
            time.sleep(4)
            continue
        time.sleep(2)
        nodes = {n["id"]: (n["lat"], n["lon"]) for n in d.get("nodes", [])}
        hits = []
        for el in d.get("elements", []):
            tags = el.get("tags", {})
            if el["type"] != "way":
                continue
            if tags.get("addr:housenumber") != hnr or street not in (tags.get("addr:street") or ""):
                continue
            nd = [nodes.get(n) for n in el.get("nodes", []) if n in nodes]
            if not nd:
                continue
            lats = sorted(p[0] for p in nd)
            lons = sorted(p[1] for p in nd)
            hits.append({
                "id": el["id"], "nodes": len(nd),
                "lon": round(sum(p[1] for p in nd) / len(nd), 7),
                "lat": round(sum(p[0] for p in nd) / len(nd), 7),
                "bbox": [lons[0], lats[0], lons[-1], lats[-1]],
                "tags": tags,
            })
        out[slug] = hits
        print(f"=== {slug}: {street} {hnr} -> {len(hits)} Objekt(e)")
        for h in hits:
            t = h["tags"]
            print(f"    way/{h['id']} {h['lat']}/{h['lon']} bbox={h['bbox']} nodes={h['nodes']}")
            print(f"      {json.dumps(t, ensure_ascii=False)}")
    with open("/tmp/opencode/osm_api_s2.json", "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
