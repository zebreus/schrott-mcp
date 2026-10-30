#!/usr/bin/env python3
"""Live street scan via the OSM API (no Overpass).

Counts the address objects on one street inside a bbox and resolves the house
numbers (ranges are expanded to their individual values).
"""
import json
import sys
import time
import urllib.request

UA = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/126.0 Safari/537.36"


def api(url, tries=6):
    last = None
    for attempt in range(tries):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            return urllib.request.urlopen(req, timeout=90).read().decode("utf-8", "replace")
        except urllib.error.HTTPError as exc:
            last = exc
            if exc.code in (429, 509, 502, 503):  # Drosselung/Überlast
                time.sleep(20 + attempt * 15)
                continue
            raise
        except Exception as exc:  # transient
            last = exc
            time.sleep(3 + attempt * 3)
    raise last


def expand(num):
    """'79-81' -> ['79','80','81']; '39A-39N' -> the letters in between."""
    if "-" not in num:
        return [num]
    lo, hi = (p.strip() for p in num.split("-", 1))
    if lo[-1].isalpha() and hi[-1].isalpha() and lo[:-1] == hi[:-1]:
        a, b = ord(lo[-1].upper()), ord(hi[-1].upper())
        if a > b:
            a, b = b, a
        return [lo[:-1] + chr(c) for c in range(a, b + 1)]
    if lo.isdigit() and hi.isdigit():
        a, b = int(lo), int(hi)
        if a > b:
            a, b = b, a
        return [str(n) for n in range(a, b + 1)]
    return [num]


def scan(bbox, streets):
    """Return {street: (objects, numbers, raw_numbers)}.

    Large bboxes are read in non-overlapping tiles (the API rejects them).
    """
    w, s, e, n = (float(x) for x in bbox.split(","))
    step = 0.01
    if (e - w) <= step and (n - s) <= step:
        tiles = [(w, s, e, n)]
    else:
        tiles = []
        lon = w
        while lon < e - 1e-9:
            lat = s
            while lat < n - 1e-9:
                tiles.append((lon, lat, min(lon + step, e), min(lat + step, n)))
                lat += step
            lon += step
    out = {}
    seen = set()
    for tw, ts, te, tn in tiles:
        data = json.loads(api(
            f"https://api.openstreetmap.org/api/0.6/map.json?bbox={tw:.7f},{ts:.7f},{te:.7f},{tn:.7f}"))
        for el in data["elements"]:
            key = (el["type"], el["id"])
            if key in seen:
                continue
            seen.add(key)
            tags = el.get("tags") or {}
            hn = tags.get("addr:housenumber")
            if not hn:
                continue
            street = tags.get("addr:street") or tags.get("addr:road")
            if not street:
                continue
            if street.strip().lower() not in streets:
                continue
            rec = out.setdefault(street, {"objects": 0, "raw": [], "ids": []})
            rec["objects"] += 1
            rec["raw"].append(hn)
            rec["ids"].append(f'{el["type"]}/{el["id"]}')
    for street, rec in out.items():
        numbers = []
        for hn in rec["raw"]:
            for v in expand(hn):
                if v not in numbers:
                    numbers.append(v)
        rec["numbers"] = numbers
    return out


if __name__ == "__main__":
    bbox = sys.argv[1]
    streets = {s.strip().lower(): s.strip() for s in sys.argv[2].split(",")}
    res = scan(bbox, streets)
    if not res:
        print("keine Treffer")
    for street, rec in res.items():
        print(f'{street}: {rec["objects"]} Objekte | {len(rec["numbers"])} verschiedene Hausnummern')
        print(f'   roh: {sorted(set(rec["raw"]))}')
        print(f'   aufgeloest: {rec["numbers"]}')
