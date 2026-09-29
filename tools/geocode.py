#!/usr/bin/env python3
"""One-shot geocoding for traders with prices (Umkreissuche groundwork).

Nominatim, 1 req/s politeness, building → postcode → city fallback chain.
Writes lat/lon straight into public.db; the seed pipeline keeps stored
coordinates (seed never carries any), so this survives re-imports.

Usage: python3 tools/geocode.py [--limit N] [--dry-run]
"""

import json
import re
import sqlite3
import sys
import time
import urllib.parse
import urllib.request

DB = "/var/lib/schrott-mcp/public.db"
UA = "schrott-mcp-geocode/0.1 (admin@offsite.lol)"
BASE = "https://nominatim.openstreetmap.org/search"


def query(params):
    url = BASE + "?" + urllib.parse.urlencode(
        {"format": "json", "limit": 1, "country": "Deutschland", **params}
    )
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=25) as r:
        data = json.loads(r.read().decode("utf-8"))
    time.sleep(1.1)  # Nominatim usage policy
    return data


def clean_city(city):
    c = (city or "").strip()
    c = re.sub(r"\s*\(.*$", "", c)  # "(LK …)" / "(Sitz …)"
    c = re.sub(r"^(sitz|standort)\s+", "", c, flags=re.I)
    c = re.sub(r"\s*\d{5}\s*$", "", c)  # trailing PLZ doubles
    c = re.sub(r"[-/].*$", "", c).strip()  # "Essen-Vogelheim" → Essen
    return c or city


def geocode(street, postcode, city):
    city = clean_city(city)
    # 1. full address
    if street and postcode:
        try:
            r = query({"street": street, "postalcode": postcode, "city": city})
            if r:
                return float(r[0]["lat"]), float(r[0]["lon"]), "address"
        except Exception as e:
            print(f"    address-level failed: {e}")
    # 2. postcode + city
    if postcode:
        try:
            r = query({"postalcode": postcode, "city": city})
            if r:
                return float(r[0]["lat"]), float(r[0]["lon"]), "postcode"
        except Exception as e:
            print(f"    postcode-level failed: {e}")
    # 3. city only
    if city:
        try:
            r = query({"city": city})
            if r:
                return float(r[0]["lat"]), float(r[0]["lon"]), "city"
        except Exception as e:
            print(f"    city-level failed: {e}")
    return None


def main():
    limit = None
    dry = False
    for a in sys.argv[1:]:
        if a.startswith("--limit="):
            limit = int(a.split("=", 1)[1])
        if a == "--dry-run":
            dry = True
    db = sqlite3.connect(DB)
    rows = db.execute(
        """SELECT DISTINCT t.id, t.slug, t.street, t.postcode, t.city
           FROM traders t JOIN current_prices c ON c.trader_id = t.id
           WHERE t.lat IS NULL ORDER BY t.slug"""
    ).fetchall()
    if limit:
        rows = rows[:limit]
    print(f"{len(rows)} price traders without coordinates")
    done, levels = 0, {}
    for tid, slug, street, postcode, city in rows:
        print(f"{slug} | {street}, {postcode} {city}")
        g = geocode(street or "", postcode or "", city or "")
        if not g:
            print("    MISS")
            continue
        lat, lon, level = g
        levels[level] = levels.get(level, 0) + 1
        print(f"    {lat:.5f},{lon:.5f} ({level})")
        if not dry:
            db.execute("UPDATE traders SET lat=?, lon=? WHERE id=?", (lat, lon, tid))
            db.commit()
        done += 1
    print(f"done: {done}/{len(rows)}, levels: {levels}")


if __name__ == "__main__":
    main()
