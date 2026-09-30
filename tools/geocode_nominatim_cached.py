#!/usr/bin/env python3
"""Serial high-precision geocoding with internal.db cache (read-through).

- Cache first: internal.db geocode_cache keyed by street|postcode|city
  (same normalization as InternalDb::geocode_key). Hit -> apply, no HTTP.
- Miss -> Nominatim (1.2s spacing, 120s cooldown on 429, max 3 retries),
  verify postcode+city, store in cache (INSERT OR IGNORE, first wins)
  and apply to public.db traders.
- Direct DB writes (maintenance window, locking OK).
"""
import json, sqlite3, time, urllib.parse, urllib.request, re, sys, unicodedata, datetime

PUB = "/var/lib/schrott-mcp/public.db"
INT = "/var/lib/schrott-mcp/internal.db"
UA = "schrott-mcp-geocode/0.1 (admin@offsite.lol)"
LOG = "/tmp/opencode/geo_cached.log"
OUT = "/tmp/opencode/geo_cached.json"
SKIP_CITIES = re.compile(r"online|weit\b|berregional|mobil|tourgebiet|hessen\b|nrw\b", re.I)

def log(msg):
    line = f"{time.strftime('%H:%M:%S')} {msg}"
    print(line, flush=True)
    with open(LOG, "a") as f: f.write(line + "\n")

def key(street, pc, city):
    n = lambda s: " ".join(s.split()).lower()
    return f"{n(street)}|{pc.strip()[:5]}|{n(city)}"

def clean_city(city):
    c = re.sub(r"\s*\(.*$", "", (city or "").strip())
    c = re.sub(r"^(sitz|standort)\s+", "", c, flags=re.I)
    c = re.sub(r"\s*\d{5}\s*$", "", c)
    c = re.sub(r"[-/].*$", "", c).strip()
    return c

def norm(s):
    return unicodedata.normalize("NFKD", s or "").encode("ascii", "ignore").decode().lower()

def main():
    pub = sqlite3.connect(PUB, timeout=60)
    inte = sqlite3.connect(INT, timeout=60)
    rows = pub.execute(
        "SELECT id, slug, street, postcode, city FROM traders "
        "WHERE lat IS NOT NULL AND street!='' AND postcode GLOB '[0-9]*'"
    ).fetchall()
    rows = [r for r in rows if not SKIP_CITIES.search(r[4] or "")]
    log(f"start total={len(rows)}")
    ok = miss = cached = 0
    res_ok, res_miss = [], []
    for i, (tid, slug, street, pc, city) in enumerate(rows):
        k = key(street, pc, city)
        hit = inte.execute("SELECT lat, lon FROM geocode_cache WHERE address_key=?", (k,)).fetchone()
        if hit:
            pub.execute("UPDATE traders SET lat=?, lon=? WHERE id=?", (hit[0], hit[1], tid))
            pub.commit()
            cached += 1
            if (cached + ok) % 100 == 0: log(f"progress {i+1}/{len(rows)} cached={cached} fresh={ok} miss={miss}")
            continue
        pc5 = pc.strip()[:5]
        q = {"format": "json", "limit": 1, "country": "Deutschland",
             "street": street, "postalcode": pc5, "city": clean_city(city)}
        url = "https://nominatim.openstreetmap.org/search?" + urllib.parse.urlencode(q)
        d = None
        for attempt in range(4):
            try:
                req = urllib.request.Request(url, headers={"User-Agent": UA})
                with urllib.request.urlopen(req, timeout=25) as r:
                    d = json.loads(r.read().decode())
                break
            except Exception as e:
                if "429" in str(e) and attempt < 3:
                    log(f"429 bei {slug} (Versuch {attempt+1}) -> warte 120s")
                    time.sleep(120)
                    continue
                log(f"ERR {slug}: {e}")
                d = None
                break
        time.sleep(1.2)
        if not d:
            miss += 1
            res_miss.append({"slug": slug, "reason": "no-result-or-error"})
            continue
        addr = d[0].get("address", {})
        got_pc = (addr.get("postcode") or "").strip()[:5]
        city_toks = set(re.sub(r"[^a-z]", " ", norm(clean_city(city))).split())
        got_place = norm(" ".join(str(v) for v in addr.values()))
        if got_pc == pc5 and (not city_toks or any(t in got_place for t in city_toks if len(t) > 2)):
            lat, lon = float(d[0]["lat"]), float(d[0]["lon"])
            now = datetime.datetime.now(datetime.timezone.utc).isoformat()
            inte.execute("INSERT OR IGNORE INTO geocode_cache VALUES (?,?,?,?,?,?,?,?)",
                         (k, street, pc5, city, lat, lon, "nominatim-building", now))
            inte.commit()
            pub.execute("UPDATE traders SET lat=?, lon=? WHERE id=?", (lat, lon, tid))
            pub.commit()
            ok += 1
            res_ok.append({"slug": slug, "lat": lat, "lon": lon})
            if (cached + ok) % 100 == 0: log(f"progress {i+1}/{len(rows)} cached={cached} fresh={ok} miss={miss}")
        else:
            miss += 1
            res_miss.append({"slug": slug, "reason": f"verify-fail db={pc5}/{city}"})
    json.dump({"ok": res_ok, "miss": res_miss, "cached": cached}, open(OUT, "w"))
    log(f"FERTIG cached={cached} fresh-ok={ok} miss={miss} -> {OUT}")

if __name__ == "__main__":
    main()
