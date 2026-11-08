#!/usr/bin/env python3
"""Read-only OSM-Nahbereichscheck: Adressobjekte und Betriebsbezug um einen Punkt.

Nur Leseabfragen gegen die OSM-API. Nutzt den Ratenbegrenzungs-Warte-Mechanismus
aus tools/osm_objects.py. Es werden keine Koordinaten geraten, sondern nur
bereits belegte Adressobjekte berichtet.

Aufruf:
    python3 tools/osm_scan.py <lat> <lon> <name> [weitere lat lon name ...]
"""
import json
import sys
import time
import urllib.request

UA = "schrott-mcp-research/1.0 (dossier audit 2026-11-08)"
TILE = 0.01


def get(url):
    for _ in range(6):
        req = urllib.request.Request(url, headers={"User-Agent": UA})
        try:
            with urllib.request.urlopen(req, timeout=120) as r:
                return json.loads(r.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            if e.code == 429:
                import re
                wait = 25
                try:
                    m = re.search(r"in (\d+) seconds", e.read().decode("utf-8", "ignore"))
                    if m:
                        wait = int(m.group(1)) + 3
                except Exception:
                    pass
                sys.stderr.write(f"429 -> warte {wait}s\n")
                time.sleep(wait)
                continue
            raise
    raise RuntimeError("429 trotz Wartezeit")


def scan(lat, lon, label):
    lat0 = int(lat / TILE) * TILE
    lon0 = int(lon / TILE) * TILE
    print(f"\n### {label}  Raster {lon0:.2f}/{lat0:.2f} bis {lon0 + 3 * TILE:.2f}/{lat0 + 3 * TILE:.2f}")
    addr_objs = []
    named = []
    tiles = 0
    errors = 0
    for i in range(3):
        for j in range(3):
            bbox = f"{lon0 + i * TILE:.4f},{lat0 + j * TILE:.4f},{lon0 + (i + 1) * TILE:.4f},{lat0 + (j + 1) * TILE:.4f}"
            url = f"https://api.openstreetmap.org/api/0.6/map.json?bbox={bbox}"
            try:
                d = get(url)
                tiles += 1
            except Exception as e:  # noqa: BLE001
                errors += 1
                sys.stderr.write(f"Kachel-Fehler {bbox}: {e}\n")
                continue
            for el in d["elements"]:
                t = el.get("tags") or {}
                if "addr:housenumber" in t and "addr:street" in t:
                    addr_objs.append((t.get("addr:street"), t.get("addr:housenumber"),
                                      t.get("addr:postcode"), t.get("addr:city"),
                                      el["type"], el["id"],
                                      el.get("lat") or (el.get("center") or {}).get("lat"),
                                      el.get("lon") or (el.get("center") or {}).get("lon"),
                                      t.get("name"), t.get("operator"), t.get("shop"), t.get("craft"),
                                      t.get("website"), t.get("contact:website")))
                if t.get("name") or t.get("operator") or t.get("brand"):
                    named.append((t.get("name"), t.get("operator"), t.get("brand"),
                                  t.get("shop"), t.get("craft"), t.get("website"),
                                  t.get("addr:street"), t.get("addr:housenumber"),
                                  el["type"], el["id"],
                                  el.get("lat") or (el.get("center") or {}).get("lat"),
                                  el.get("lon") or (el.get("center") or {}).get("lon")))
            time.sleep(1.2)
    print(f"Kacheln gelesen: {tiles}, Fehler: {errors}")
    print(f"Adressobjekte: {len(addr_objs)}")
    nums = sorted({(s, h) for s, h, *_ in addr_objs})
    print("  Hausnummern:", ", ".join(f"{s} {h}" for s, h in nums))
    print(f"Objekte mit name/operator/brand: {len(named)}")
    for n in named:
        print("   ", " | ".join(str(x) for x in n))


def main():
    args = sys.argv[1:]
    for i in range(0, len(args), 3):
        scan(float(args[i]), float(args[i + 1]), args[i + 2])


if __name__ == "__main__":
    main()
