#!/usr/bin/env python3
"""Full offline geocoding from GeoNames DE postal data (no external calls).

Level policy (honest, coarse-to-fine):
- postcode exact hit -> PLZ centroid (best without street).
- city exact full-name hit -> place centroid.
- city token hit -> centroid of same-STATE candidates only (never a
  Germany-wide centroid for ambiguous names like "Neustadt").
- else NULL (unknown like "k.A.", address fragments, online).

Writes lat/lon into public.db; the seed pipeline keeps stored
coordinates, so this survives re-imports. CC-BY: GeoNames
(https://www.geonames.org, download 30.09.2026).
"""

import json
import re
import sqlite3
import unicodedata
from collections import defaultdict

DB = "/var/lib/schrott-mcp/public.db"
STATES = {
    "BW": "baden wurttemberg", "BY": "bayern", "BE": "berlin",
    "BB": "brandenburg", "HB": "bremen", "HH": "hamburg",
    "HE": "hessen", "MV": "mecklenburg vorpommern", "NI": "niedersachsen",
    "NW": "nordrhein westfalen", "RP": "rheinland pfalz", "SL": "saarland",
    "SN": "sachsen", "ST": "sachsen anhalt", "SH": "schleswig holstein",
    "TH": "thuringen",
}


def norm(s):
    s = unicodedata.normalize("NFKD", (s or "")).encode("ascii", "ignore").decode().lower()
    return re.sub(r"[^a-z ]", " ", s).strip()


def load():
    plz = defaultdict(list)    # postcode -> [(lat,lon)]
    place = defaultdict(list)  # norm place -> [(lat,lon,state)]
    for line in open("/tmp/opencode/geonames/DE.txt", encoding="utf-8"):
        c = line.rstrip("\n").split("\t")
        if len(c) < 12:
            continue
        try:
            la, lo = float(c[9]), float(c[10])
        except ValueError:
            continue
        if c[1]:
            plz[c[1]].append((la, lo))
        if c[2]:
            place[norm(c[2])].append((la, lo, norm(c[3])))
    return plz, place


def centroid(pts):
    return (sum(p[0] for p in pts) / len(pts), sum(p[1] for p in pts) / len(pts))


def main():
    plz, place = load()
    first = defaultdict(list)
    for k in place:
        if k.split():
            first[k.split()[0]].append(k)
    db = sqlite3.connect(DB)
    rows = db.execute(
        "SELECT id, slug, postcode, city, state FROM traders WHERE lat IS NULL"
    ).fetchall()
    n_plz = n_city = n_state = n_miss = 0
    for tid, slug, pc, city, st in rows:
        pc = (pc or "").strip()[:5]
        hit = None
        if pc in plz:
            hit = (centroid(plz[pc]), "postcode")
            n_plz += 1
        else:
            toks = [t for t in re.split(r"[\s/(),-]+", norm(city)) if t]
            full = " ".join(toks)
            if full in place:
                hit = (centroid([(la, lo) for la, lo, _ in place[full]]), "city-exact")
                n_city += 1
            else:
                want = STATES.get(st, "")
                seen = []
                for t in toks:
                    grp = []
                    if t in place:
                        grp += [(la, lo) for la, lo, s in place[t] if s == want]
                    for k in first.get(t, []):
                        if k != t:
                            grp += [(la, lo) for la, lo, s in place[k] if s == want]
                    if grp:
                        seen = grp
                        break
                if seen:
                    hit = (centroid(seen), "city-state")
                    n_state += 1
        if hit:
            (la, lo), level = hit
            db.execute("UPDATE traders SET lat=?, lon=? WHERE id=?", (la, lo, tid))
        else:
            n_miss += 1
    db.commit()
    left = db.execute("SELECT COUNT(*) FROM traders WHERE lat IS NULL").fetchone()[0]
    print(f"postcode:{n_plz} city-exact:{n_city} city-state:{n_state} miss:{n_miss} still-null:{left}")


if __name__ == "__main__":
    main()
