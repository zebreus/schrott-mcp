#!/usr/bin/env python3
"""Read-only Geokodierung der Dossier-Anschriften ueber Nominatim/OSM.

Nur Leseabfragen. Es werden weder Koordinaten geraten noch Ergebnisse
uebernommen, die keine Hausnummer treffen.
"""
import json
import sys
import time
import urllib.parse
import urllib.request

UA = "schrott-mcp-research/1.0 (dossier audit 2026-11-08)"

ADDRESSES = [
    ("he-wiesbaden-erbenheim-scholz-rico-schrott", "Rennbahnstr. 1, 65205 Wiesbaden-Erbenheim, Deutschland", "65205"),
    ("he-wiesbaden-65199-hubinger", "Holzstraße 48A, 65197 Wiesbaden, Deutschland", "65197"),
    ("he-wiesbaden-65199-hubinger#schwarzenberg", "Schwarzenbergstraße 9a, 65189 Wiesbaden, Deutschland", "65189"),
    ("he-langenselbold-63505-hontsch-maria", "Hanauer Straße 66b, 63505 Langenselbold, Deutschland", "63505"),
    ("he-babenhausen-64832-kohlmann-frank", "Am Fuchsberg 8, 64832 Babenhausen, Deutschland", "64832"),
    ("he-offenbach-am-main-fred-de-snoo-rohstoffe-schrott-und-metal", "Lämmerspieler Weg 158, 63075 Offenbach am Main, Deutschland", "63075"),
    ("he-messel-64409-schrottler-eddie", "Zeilharder Straße 23a, 64409 Messel, Deutschland", "64409"),
    ("he-schaafheim-64850-schrenk", "Daimlerstraße 6, 64850 Schaafheim, Deutschland", "64850"),
    ("he-limburg-65549-scheuer", "Im Elbboden 1, 65549 Limburg an der Lahn, Deutschland", "65549"),
    ("he-hanau-63452-guth", "Niddastraße 32, 63452 Hanau, Deutschland", "63452"),
    ("he-oberursel-61440-lackner#11", "Pfeiffstraße 11, 61440 Oberursel, Deutschland", "61440"),
    ("he-wiesbaden-biebrich-wiesbaden-dotzheim-bert-brecht-str-26", "Bert-Brecht-Straße 26, 65201 Wiesbaden, Deutschland", "65201"),
    ("he-hanau-lamboy-schrott-und-metalle-reinhardt", "Friedrich-Engels-Straße 5, 63452 Hanau, Deutschland", "63452"),
]


def nominatim(q):
    url = ("https://nominatim.openstreetmap.org/search?format=jsonv2&limit=5&addressdetails=1&countrycodes=de&q="
           + urllib.parse.quote(q))
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read().decode("utf-8"))


def main():
    out = {}
    for slug, q, pc in ADDRESSES:
        try:
            res = nominatim(q)
        except Exception as e:  # noqa: BLE001
            print(f"{slug}: FEHLER {e}", file=sys.stderr)
            continue
        time.sleep(1.1)
        hits = []
        for r in res:
            a = r.get("address", {})
            hits.append({
                "lat": r["lat"], "lon": r["lon"],
                "display": r.get("display_name"),
                "type": r.get("type"), "osm_type": r.get("osm_type"), "osm_id": r.get("osm_id"),
                "class": r.get("class"),
                "addr_housenumber": a.get("house_number"),
                "addr_street": a.get("road") or a.get("pedestrian") or a.get("footway"),
                "addr_postcode": a.get("postcode"), "addr_city": a.get("city") or a.get("town") or a.get("village"),
                "addr_suburb": a.get("suburb") or a.get("neighbourhood"),
            })
        out[slug] = hits
        print(f"=== {slug}: {q}")
        for h in hits:
            print(f"    {h['lat']}/{h['lon']}  hnr={h['addr_housenumber']} str={h['addr_street']} "
                  f"pc={h['addr_postcode']} city={h['addr_city']} type={h['type']}/{h['class']} "
                  f"osm={h['osm_type']}/{h['osm_id']}")
            print(f"       {h['display']}")
    with open("/tmp/opencode/geo_s2.json", "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
