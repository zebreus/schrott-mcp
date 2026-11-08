#!/usr/bin/env python3
"""Read-only OSM-Objektabfragen (api.openstreetmap.org) mit Ratenbegrenzung.

Nur Leseabfragen, keine Schreiboperationen. Wartet bei HTTP 429 die von der
API gemeldete Wartezeit ab.
"""
import json
import sys
import time
import urllib.request

UA = "schrott-mcp-research/1.0 (dossier audit 2026-11-08)"


def get(url):
    for attempt in range(6):
        req = urllib.request.Request(url, headers={"User-Agent": UA})
        try:
            with urllib.request.urlopen(req, timeout=90) as r:
                return json.loads(r.read().decode("utf-8"))
        except urllib.error.HTTPError as e:
            if e.code == 429:
                wait = 25
                try:
                    body = e.read().decode("utf-8", "ignore")
                    import re
                    m = re.search(r"in (\d+) seconds", body)
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
    ids = sys.argv[1:]
    for oid in ids:
        url = f"https://api.openstreetmap.org/api/0.6/{oid}.json"
        try:
            d = get(url)
        except Exception as e:  # noqa: BLE001
            print(f"{oid}: FEHLER {e}")
            continue
        el = d["elements"][0]
        lat = el.get("lat") or (el.get("center") or {}).get("lat")
        lon = el.get("lon") or (el.get("center") or {}).get("lon")
        print(f"{oid}: {lat},{lon} tags={json.dumps(el.get('tags'), ensure_ascii=False)}")
        time.sleep(1.2)


if __name__ == "__main__":
    main()
