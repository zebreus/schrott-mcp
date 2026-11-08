#!/usr/bin/env python3
"""Read-only Overpass-Abfrage: Betriebsobjekte (name/operator/brand) in einem Fenster.

Nur Leseabfragen gegen die Overpass-API. Es werden keine Koordinaten geraten,
sondern nur vorhandene OSM-Objekte berichtet.

Aufruf:
    python3 tools/overpass_business.py "<lat>,<lon>,<radius_m>" "<name-regex>" [label]
"""
import json
import sys
import time
import urllib.parse
import urllib.request

UA = "schrott-mcp-research/1.0 (dossier audit 2026-11-08)"
ENDPOINTS = [
    "https://overpass-api.de/api/interpreter",
    "https://overpass.kumi.systems/api/interpreter",
]


def query(bbox, name_re):
    return (f'[out:json][timeout:120];'
            f'nwr["name"~"{name_re}",i]({bbox});'
            f'out center tags;')


def run(bbox, name_re):
    q = query(bbox, name_re)
    data = urllib.parse.urlencode({"data": q}).encode()
    last = None
    for ep in ENDPOINTS:
        for attempt in range(3):
            try:
                req = urllib.request.Request(ep, data=data, headers={"User-Agent": UA})
                with urllib.request.urlopen(req, timeout=180) as r:
                    return json.loads(r.read().decode("utf-8")), ep
            except Exception as e:  # noqa: BLE001
                last = e
                time.sleep(10)
    raise RuntimeError(f"Overpass nicht erreichbar: {last}")


def main():
    bbox = sys.argv[1]
    name_re = sys.argv[2]
    label = sys.argv[3] if len(sys.argv) > 3 else bbox
    d, ep = run(bbox, name_re)
    els = d.get("elements", [])
    print(f"### {label}  [{ep}]  Treffer: {len(els)}")
    for el in els:
        t = el.get("tags", {})
        c = el.get("center") or el
        print(f"  {el['type']}/{el['id']} @ {c.get('lat')},{c.get('lon')} "
              f"name={t.get('name')!r} operator={t.get('operator')!r} brand={t.get('brand')!r} "
              f"shop={t.get('shop')!r} craft={t.get('craft')!r} "
              f"addr={t.get('addr:street')!r} {t.get('addr:housenumber')!r} "
              f"{t.get('addr:postcode')!r} {t.get('addr:city')!r} "
              f"phone={t.get('phone')!r} website={t.get('website')!r}")
    time.sleep(2)


if __name__ == "__main__":
    main()
