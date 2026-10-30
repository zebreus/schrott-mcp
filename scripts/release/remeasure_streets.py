#!/usr/bin/env python3
"""Nachmessung aller von der Prüfung beanstandeten Strassen.

Deterministisch: eine Bbox je Strasse, Objekte = Elemente mit addr:housenumber
und addr:street/addr:road == Strasse; Bereichsnotationen werden einzeln
aufgeloest.
"""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from osm_street_scan import api, expand, scan  # noqa: E402


def street_bbox(street, city):
    """Bbox der Strasse aus Nominatim (alle Treffer vereinigt, plus Rand)."""
    import urllib.parse
    q = urllib.parse.urlencode({"street": street, "city": city, "format": "json",
                                "limit": 10, "addressdetails": 1})
    hits = json.loads(api(f"https://nominatim.openstreetmap.org/search?{q}"))
    if not hits:
        return None
    lats, lons = [], []
    for h in hits:
        bb = h.get("boundingbox")
        if not bb:
            continue
        lats += [float(bb[0]), float(bb[1])]
        lons += [float(bb[2]), float(bb[3])]
    if not lats:
        return None
    pad = 0.0006
    return (f"{min(lons)-pad:.7f},{min(lats)-pad:.7f},"
            f"{max(lons)+pad:.7f},{max(lats)+pad:.7f}")


def measure(label, bbox, street):
    res = scan(bbox, {street.lower(): street})
    rec = res.get(street)
    if rec is None:
        print(f"!! {label}: kein Treffer fuer '{street}' in {bbox}")
        return None
    return rec


def fmt(nums):
    return ", ".join(nums)


if __name__ == "__main__":
    cases = json.loads(Path(sys.argv[1]).read_text())
    out = {}
    for c in cases:
        bbox = c.get("bbox")
        if not bbox:
            bbox = street_bbox(c["street"], c["city"])
        rec = measure(c["label"], bbox, c["street"])
        if rec is None:
            continue
        nums = rec["numbers"]
        out[c["label"]] = {"bbox": bbox, "street": c["street"], "objects": rec["objects"],
                           "numbers": len(nums), "number_list": nums,
                           "raw": sorted(set(rec["raw"]))}
        print(f'{c["label"]}: {rec["objects"]} Objekte / {len(nums)} HN  bbox={bbox}')
    Path(sys.argv[2]).write_text(json.dumps(out, indent=1, ensure_ascii=False))
    print("->", sys.argv[2])
