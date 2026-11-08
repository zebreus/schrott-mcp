#!/usr/bin/env python3
"""Read-only OSM-Objektabfragen fuer Dossier-Adressen (Shard S2, Welle 09.11.2026).

Nur Leseabfragen gegen die OSM-API. Es werden keine Koordinaten geraten.
"""
import json
import sys
import time
import urllib.request

UA = "schrott-mcp-research/1.0 (dossier audit 2026-11-09)"

OBJECTS = [
    ("nw-bochum-taurus", "way/248948124"),
    ("nw-koln-b-v", "way/253026635"),
    ("nw-bonn-khk-metallrecycling#brieger", "way/1080985228"),
    ("nw-bonn-khk-metallrecycling#meckenheimer", "way/160046552"),
    ("nw-bonn-franssen#kattowitzer", "way/1080321863"),
    ("nw-bonn-franssen#oppelner", "way/1080325609"),
    ("nw-bornheim-welsch", "way/300579213"),
    ("nw-munster-hr", "way/303120014"),
    ("nw-essen-schuttler", "way/307631343"),
    ("nw-stolberg-motter", "way/175212890"),
    ("nw-wipperfurth-containerdienst-jurgen-keller", "way/188601473"),
    ("nw-oberhausen-ebh", "way/159400595"),
    ("nw-munster-schrottdirektor", "way/300120578"),
    ("nw-solingen-torner", "way/93709334"),
    ("nw-remscheid-wilms", "way/132126562"),
    ("nw-duisburg-rodemann#duisburg", "way/209151013"),
    ("nw-duisburg-rodemann#raesfeld", "way/992310656"),
]


def get(url):
    for _ in range(6):
        req = urllib.request.Request(url, headers={"User-Agent": UA})
        try:
            with urllib.request.urlopen(req, timeout=90) as r:
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


def main():
    for slug, oid in OBJECTS:
        try:
            d = get(f"https://api.openstreetmap.org/api/0.6/{oid}.json")
        except Exception as e:  # noqa: BLE001
            print(f"{slug}: {oid}: FEHLER {e}")
            continue
        el = d["elements"][0]
        lat = el.get("lat") or (el.get("center") or {}).get("lat")
        lon = el.get("lon") or (el.get("center") or {}).get("lon")
        tags = el.get("tags") or {}
        print(f"{slug}: {oid} {el['type']} {lat}/{lon}")
        print(f"    tags: {json.dumps(tags, ensure_ascii=False, sort_keys=True)}")
        time.sleep(1.0)


if __name__ == "__main__":
    main()
