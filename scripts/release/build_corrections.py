#!/usr/bin/env python3
"""Erzeugt das Korrekturmanifest der 12:00- und 18:00-Welle.

Alle Zahlenwerte stammen aus der Nachmessung (scripts/release/remeasure_streets.py,
api.openstreetmap.org, Raster wie im jeweiligen Dossier genannt, Abruf 30.10.2026).
Jeder Eintrag ersetzt genau einen eindeutigen Textstellen und haengt zusaetzlich
einen datierten Korrekturvermerk an die letzte Wellen-Sektion an.
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from osm_street_scan import api, expand  # noqa: E402

M = []
SRC = "Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=<Raster laut Dossier> (Nachmessung 30.10.2026)"


def add(file, label, old, new, note):
    M.append({"file": file, "label": label, "old": old, "new": new, "note": note})


def raster_numbers(bbox):
    """Alle Objekte mit addr:housenumber im Raster -> (Anzahl, sortierte Nummern)."""
    w, s, e, n = (float(x) for x in bbox.split(","))
    seen, out = set(), []
    step, lon = 0.01, w
    while lon < e - 1e-9:
        lat = s
        while lat < n - 1e-9:
            data = json.loads(api(
                f"https://api.openstreetmap.org/api/0.6/map.json?bbox={lon:.7f},{lat:.7f},"
                f"{min(lon + step, e):.7f},{min(lat + step, n):.7f}"))
            for el in data["elements"]:
                key = (el["type"], el["id"])
                if key in seen:
                    continue
                seen.add(key)
                if (el.get("tags") or {}).get("addr:housenumber"):
                    out.append(el)
            lat += step
        lon += step
    nums = []
    for el in out:
        for v in expand(el["tags"]["addr:housenumber"]):
            if v not in nums:
                nums.append(v)
    return len(out), nums


def sortkey(n):
    head = "".join(c for c in n if c.isdigit())
    tail = "".join(c for c in n if not c.isdigit())
    return (int(head) if head else 0, tail)


# ---------------------------------------------------------------- 18:00-Welle
add("dossiers/he/he-dieburg-64807-horeth-abschleppdienst-autoverwertung.md",
    "Am Bauhof: Objektzahl 44 -> 31",
    "8.8426532,49.9040121,8.8472218,49.9099563; 0 Kachel-Fehler) ergab **44 Adressobjekte auf der Straße**",
    "8.8426532,49.9040121,8.8472218,49.9099563; 0 Kachel-Fehler) ergab **31 Adressobjekte auf der Straße**",
    "**Objektzahl der Am Bauhof berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster "
    "(bbox 8.8426532,49.9040121,8.8472218,49.9099563) ergibt **31** Adressobjekte mit "
    "`addr:housenumber=Am Bauhof` (bisher 44); die 28 verschiedenen Hausnummern sind unveraendert. " + SRC)

add("dossiers/he/he-hanau-kulzer-scheidgut-service.md",
    "Leipziger Straße: Objektzahl 31 -> 28",
    "8.9230257,50.1241620,8.9460847,50.1315571): 31 Adressobjekte / 25 verschiedene Hausnummern;",
    "8.9230257,50.1241620,8.9460847,50.1315571): 28 Adressobjekte / 25 verschiedene Hausnummern;",
    "**Objektzahl der Leipziger Straße berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster "
    "(bbox 8.9230257,50.1241620,8.9460847,50.1315571) ergibt **28** Adressobjekte mit "
    "`addr:housenumber=Leipziger Straße` (bisher 31); die 25 verschiedenen Hausnummern sind unveraendert. " + SRC)

add("dossiers/he/he-nidda-m-h-schrothandel.md",
    "Hoherodskopfstraße: Objektzahl 46 -> 39, Hausnummer 9 streichen",
    "9.0082053,50.4196323,9.0120167,50.4249191; 0 Kachel-Fehler): 46 Adressobjekte auf der Strasse / 37 verschiedene Hausnummern (2, 3, 4, 5, 7, 9, 11, ",
    "9.0082053,50.4196323,9.0120167,50.4249191; 0 Kachel-Fehler): 39 Adressobjekte auf der Strasse / 37 verschiedene Hausnummern (2, 3, 4, 5, 7, 11, ",
    "**Zählung der Hoherodskopfstraße berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster "
    "(bbox 9.0082053,50.4196323,9.0120167,50.4249191) ergibt **39** Adressobjekte mit "
    "`addr:housenumber=Hoherodskopfstraße` (bisher 46) und **37** verschiedene Hausnummern (2, 3, 4, 5, 7, "
    "11, 12, 13, 14, 15, 16, 16a, 17, 18, 19, 22, 23, 24, 25, 26, 26a, 27, 27a, 28, 29, 30, 31, 33, 35, 37, "
    "39, 41, 43, 45, 47, 49, 51). **Die Hausnummer 9 existiert dort nicht** und ist aus der Aufzaehlung "
    "gestrichen. " + SRC)

add("dossiers/he/he-nidderau-die-schrottler.md",
    "Hauptstraße Nidderau: Objektzahl 53 -> 46",
    "8.8633637,50.2647161,8.8686511,50.2692329; 0 Kachel-Fehler): 53 Adressobjekte auf der Strasse / 46 verschied",
    "8.8633637,50.2647161,8.8686511,50.2692329; 0 Kachel-Fehler): 46 Adressobjekte auf der Strasse / 46 verschied",
    "**Objektzahl der Hauptstraße Nidderau berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster "
    "(bbox 8.8633637,50.2647161,8.8686511,50.2692329) ergibt **46** Adressobjekte mit "
    "`addr:housenumber=Hauptstraße` (bisher 53); die 46 verschiedenen Hausnummern sind unveraendert. " + SRC)

add("dossiers/he/he-offenbach-burgel-63075-hess-juwelier-uhrmacher.md",
    "Kurfürstenstraße: Objektzahl 58 -> 49, Hausnummern 50 -> 48",
    "8.7810000,50.1110000,8.7920000,50.1200000; 0 Kachel-Fehler): 58 Adressobjekte auf der Strasse / 50 verschied",
    "8.7810000,50.1110000,8.7920000,50.1200000; 0 Kachel-Fehler): 49 Adressobjekte auf der Strasse / 48 verschied",
    "**Zählung der Kurfürstenstraße berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster "
    "(bbox 8.7810000,50.1110000,8.7920000,50.1200000) ergibt **49** Adressobjekte mit "
    "`addr:housenumber=Kurfürstenstraße` (bisher 58) und **48** verschiedene Hausnummern (bisher 50). " + SRC)

add("dossiers/he/he-runkel-steeden-rav-recycling.md",
    "Industriestraße Runkel: Objektzahl 24 -> 15",
    "8.1275162,50.4009223,8.1390513,50.4054106; 0 Kachel-Fehler): 24 Adressobjekte auf der Strasse / 15 verschied",
    "8.1275162,50.4009223,8.1390513,50.4054106; 0 Kachel-Fehler): 15 Adressobjekte auf der Strasse / 15 verschied",
    "**Objektzahl der Industriestraße Runkel berichtigt**: die Nachmessung am 30.10.2026 mit demselben "
    "Raster (bbox 8.1275162,50.4009223,8.1390513,50.4054106) ergibt **15** Adressobjekte mit "
    "`addr:housenumber=Industriestraße` (bisher 24); die 15 verschiedenen Hausnummern sind unveraendert. " + SRC)

add("dossiers/he/he-marburg-35041-hartstock.md",
    "Pfaffenberg Marburg: Objektzahl 17 -> 15",
    "8.6857584,50.8069030,8.6896428,50.8113592; 0 Kachel-Fehler) ergab **17 Adressobjekte auf der Straße**",
    "8.6857584,50.8069030,8.6896428,50.8113592; 0 Kachel-Fehler) ergab **15 Adressobjekte auf der Straße**",
    "**Objektzahl am Pfaffenberg berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster (bbox "
    "8.6857584,50.8069030,8.6896428,50.8113592) ergibt **15** Adressobjekte mit `addr:housenumber=Pfaffenberg` "
    "(bisher 17); die 15 verschiedenen Hausnummern sind unveraendert. " + SRC)

add("dossiers/he/he-wiesbaden-dotzheim-wagner-marcel-schrotthandel-und-steinrei.md",
    "Hollermorgenstraße: Objektzahl 12 -> 10",
    "8.2076207,50.0634491,8.2116351,50.0670800; 0 Kachel-Fehler): 12 Adressobjekte auf der Strasse / 10 verschied",
    "8.2076207,50.0634491,8.2116351,50.0670800; 0 Kachel-Fehler): 10 Adressobjekte auf der Strasse / 10 verschied",
    "**Objektzahl der Hollermorgenstraße berichtigt**: die Nachmessung am 30.10.2026 mit demselben Raster "
    "(bbox 8.2076207,50.0634491,8.2116351,50.0670800) ergibt **10** Adressobjekte mit "
    "`addr:housenumber=Hollermorgenstraße` (bisher 12); die 10 verschiedenen Hausnummern sind unveraendert. " + SRC)

add("dossiers/he/he-weimar-lahn-heinz-becker-recycling-inh-daniel-muth.md",
    "Dorfstraße Weimar: Objektzahl 32 -> 34, Hausnummern 30 -> 34",
    "8.7153622,50.7438234,8.7205052,50.7483894; 0 Kachel-Fehler): 32 Adressobjekte auf der Strasse / 30 verschied",
    "8.7153622,50.7438234,8.7205052,50.7483894; 0 Kachel-Fehler): 34 Adressobjekte auf der Strasse / 34 verschied",
    "**Zählung der Dorfstraße (Weimar/Lahn) berichtigt**: die Nachmessung am 30.10.2026 mit demselben "
    "Raster (bbox 8.7153622,50.7438234,8.7205052,50.7483894) ergibt **34** Adressobjekte mit "
    "`addr:housenumber=Dorfstraße` (bisher 32) und **34** verschiedene Hausnummern (bisher 30). In der "
    "Aufzaehlung fehlten **1, 4, 5 und 6** (way/357899167, way/357235908, way/357899061, way/357235892). " + SRC)

add("dossiers/he/he-herborn-35745-nenni-hakan.md",
    "Stadionstraße Herborn: Hausnummern 13 -> 12",
    "13 verschiedene Hausnummern nach Aufloesung der Bereichsnotationen; die Hausnummer 2 ",
    "12 verschiedene Hausnummern nach Aufloesung der Bereichsnotationen; die Hausnummer 2 ",
    "**Zahl der verschiedenen Hausnummern berichtigt**: die Nachmessung am 30.10.2026 mit dem im Dossier "
    "genannten Raster (bbox 8.2845000,50.6700000,8.2935000,50.6790000) ergibt **12** verschiedene "
    "Hausnummern auf der Stadionstraße (bisher 13); die Zahl der Adressobjekte (12) ist unveraendert. " + SRC)

add("dossiers/he/he-frankfurt-seckbach-metallschrott-wiesbaden-inh-andy-biederm.md",
    "Edisonstraße: Hausnummern 17 -> 16",
    "17 verschiedene Hausnummern (3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16, 17, 18, 19, 20)",
    "16 verschiedene Hausnummern (3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16, 17, 18, 19, 20)",
    "**Zahl der verschiedenen Hausnummern berichtigt**: die Nachmessung am 30.10.2026 mit dem im Dossier "
    "genannten Raster (bbox 8.7422482,50.1402222,8.7465588,50.1443525) ergibt **16** verschiedene "
    "Hausnummern auf der Edisonstraße (bisher 17); die aufgefuehrte Liste war vollstaendig und unveraendert, "
    "die Zahl der Adressobjekte (19) ist unveraendert. " + SRC)

add("dossiers/nw/nw-castrop-rauxel-konig.md",
    "Westrandweg: verschiedene Hausnummern 53 -> 54",
    "**54 Objekte mit `addr:housenumber`**, daraus **53 verschiedene",
    "**54 Objekte mit `addr:housenumber`**, daraus **54 verschiedene",
    "**Zahl der verschiedenen Hausnummern berichtigt**: die Nachmessung am 30.10.2026 mit dem im Dossier "
    "genannten Rasterfenster (7.264267/51.577060/7.270494/51.579271) ergibt **54** verschiedene Hausnummern "
    "auf dem Westrandweg (bisher 53); die Zahl der Adressobjekte (54) ist unveraendert. " + SRC)

# ---------------------------------------------------------------- 12:00-Welle
add("dossiers/he/he-nidderau-esorec-recycling.md",
    "Siemensstraße Nidderau: Hausnummern 55 -> 57",
    "71 Adressobjekte, 55 verschiedene Hausnummern** (1, 1a, 2,",
    "71 Adressobjekte, 57 verschiedene Hausnummern** (1, 1a, 2,",
    "**Zahl der verschiedenen Hausnummern im Korridor berichtigt**: die Nachmessung am 30.10.2026 mit "
    "demselben Raster (bbox 8.864374,50.240149,8.870221,50.244856) ergibt **71** Adressobjekte mit "
    "`addr:housenumber` (unveraendert) und **57** verschiedene Hausnummern (bisher 55). " + SRC)

for rel, label, bbox, claim in (
    ("dossiers/he/he-bad-emstal-34308-bicht-f.md", "Bad Emstal: Korridorzaehlung 113/36 -> 21/18",
     "9.234472,51.273855,9.237157,51.274952", None),
    ("dossiers/he/he-sinn-35764-risse-hildebrand.md", "Sinn: Korridorzaehlung 31/17 -> 8/7",
     "8.323341,50.633462,8.324702,50.636071", None),
    ("dossiers/he/he-wetzlar-md-demir.md", "Wetzlar: Korridorzaehlung 255/98 -> 111/59",
     "8.500774,50.566715,8.502887,50.571707", None),
):
    n, nums = raster_numbers(bbox)
    lst = ", ".join(sorted(nums, key=sortkey))
    print(f"  gemessen {label}: {n} Objekte / {len(nums)} HN -> {lst}")

# ---------------------------------------------------------------- Sachkorrekturen
add("dossiers/he/he-eschborn-65760-mg-ne-produkthandel.md",
    "Eschborn: Bereichsnotation 79-81 existiert (Negativaussage falsch)",
    "die Nummern 79, 80 und 81 sind einzeln erfasst, eine Objektzusammenfassung „79-81\" existiert NICHT;",
    "die Nummern 79, 80 und 81 sind **nicht** einzeln erfasst, sie stammen aus **einem** Objekt mit der "
    "Bereichsangabe `addr:housenumber=79-81` (way/26189438, `addr:street=Mergenthalerallee`, "
    "`addr:postcode=65760`, `building=office`, `name=Topas 2`);",
    "**Negativaussage zur Objektzusammenfassung berichtigt**: die Aussage, eine Objektzusammenfassung "
    "„79-81\" existiere nicht, war falsch. Live am 30.10.2026 traegt way/26189438 die Bereichsangabe "
    "`addr:housenumber=79-81`; die Nummern 79, 80 und 81 sind gerade **nicht** einzeln erfasst. " + SRC)

add("dossiers/sn/sn-annaberg-buchholz-scholz-recycling-betrieb-annaberg-buchho.md",
    "Annaberg-Buchholz: Abfallschluessel 120122 -> 160122",
    "160117, 160118, 120122, 160214, 160216, 160304)",
    "160117, 160118, 160122, 160214, 160216, 160304)",
    "**Abfallschluessel berichtigt**: das E-SN-Zertifikat 98-070142/2 (26) vom 23.06.2026 fuehrt "
    "**160122** („Bauteile a.n.g.“), nicht 120122; der Code 120122 kommt im Zertifikat nicht vor. Die "
    "uebrigen genannten Codes (020110, 110501, 120101-120104, 120113, 150104, 150106, 160106, 160112, "
    "160117, 160118, 160214, 160216, 160304) sind im PDF wörtlich vorhanden. "
    "Quelle(n): https://scholz-recycling.com/wp-content/uploads/260623_UeZEFB_ANA.pdf (Abruf 30.10.2026)")

add("dossiers/sn/sn-dresden-pro-aurum-filiale.md",
    "Postplatz: addr-Tags von node/1123870439 entfernen",
    "**node/1123870439** mit `addr:housenumber=1`, `addr:street=Postplatz`, `addr:city=Dresden`, "
    "`addr:country=DE`, `addr:postcode=01067`, **`name=DVB Kundenzentrum`**, `shop=ticket`, "
    "`information=office`, `opening_hours=Mo-Fr 09:00-17:00; Sa 10:00-17:30`, `phon",
    "**node/1123870439** mit **`name=DVB Kundenzentrum`**, `shop=ticket`, `information=office`, "
    "`opening_hours=Mo-Fr 09:00-17:00; Sa 10:00-17:30`, `phon",
    "**Tag-Zitat berichtigt**: node/1123870439 traegt live **kein** `addr:*`-Tag; die Adress-Tags "
    "`addr:housenumber=1`, `addr:street=Postplatz`, `addr:city=Dresden`, `addr:country=DE`, "
    "`addr:postcode=01067` gehoeren zu **relation/77160** (Wilsdruffer Kubus). "
    "Quelle(n): https://api.openstreetmap.org/api/0.6/node/1123870439.json ; "
    "https://api.openstreetmap.org/api/0.6/relation/77160.json (Abruf 30.10.2026)")

add("dossiers/sn/sn-dresden-pro-aurum-filiale.md",
    "Postplatz: addr-Tags von node/3512103703 entfernen",
    "und **node/3512103703** mit `addr:housenumber=1`, `addr:street=Postplatz`, `name=Postplatz`, "
    "`highway=bus_stop`",
    "und **node/3512103703** mit `name=Postplatz`, `highway=bus_stop`",
    "**Tag-Zitat berichtigt**: node/3512103703 traegt live **kein** `addr:*`-Tag (reiner Bushaltestellen-"
    "Node mit `highway=bus_stop`, `operator=DVB`, `ref=37002`). "
    "Quelle(n): https://api.openstreetmap.org/api/0.6/node/3512103703.json (Abruf 30.10.2026)")

add("dossiers/sn/sn-dresden-pro-aurum-filiale.md",
    "Postplatz: Moebius/Moebius, contact:website, opening_hours",
    "die **Bäckerei Moebius** (node/344886620, `amenity=cafe`, `shop=bakery`, "
    "`operator=Bäckerei Moebius OHG`, `phone=+49 351 4810422`, `opening_hours=Mo-Sa 06:00-19:00`)",
    "die **Bäckerei Möbius** (node/344886620, `amenity=cafe`, `shop=bakery`, "
    "`operator=Bäckerei Möbius OHG`, `phone=+49 351 4810422`, "
    "`opening_hours=Mo-Sa 06:00-19:00; PH,Su 07:00-18:00`)",
    "**Tag-Zitat berichtigt**: node/344886620 traegt live `operator=Bäckerei **Möbius** OHG` (mit "
    "Umlaut), `contact:website=https://www.moebius-baeckerei.de/` (nicht `website=`; `website=` wurde in "
    "OSM-Version 15 entfernt) und `opening_hours=Mo-Sa 06:00-19:00; PH,Su 07:00-18:00`. "
    "Quelle(n): https://api.openstreetmap.org/api/0.6/node/344886620.json (Abruf 30.10.2026)")

add("dossiers/sn/sn-dresden-flaxres.md",
    "Flaxres: contact:website statt website",
    "`operator=Nadin Lange`, `website=https://www.triebwerkstatt.de/`",
    "`operator=Nadin Lange`, `contact:website=https://www.triebwerkstatt.de/`",
    "**Tag-Zitat berichtigt**: node/5287843129 traegt live `contact:website=` (die Umstellung von "
    "`website=` auf `contact:website=` erfolgte bereits in OSM-Version 4 vom 01.08.2020). "
    "Quelle(n): https://api.openstreetmap.org/api/0.6/node/5287843129.json (Abruf 30.10.2026)")

add("dossiers/sn/sn-dresden-flaxres.md",
    "Flaxres: contact:phone/contact:website statt phone/website",
    "node/7717696923 mit `addr:housenumber=80`, `addr:street=Blumenstraße`, `addr:city=Dresden`, "
    "`addr:postcode=01307`, `addr:country=DE`, `name=fiedz`, `operator=Jan Fiedler`, "
    "`phone=+49 351 1609217`, `website=https://fiedz.de/`",
    "node/7717696923 mit `addr:housenumber=80`, `addr:street=Blumenstraße`, `addr:city=Dresden`, "
    "`addr:postcode=01307`, `addr:country=DE`, `name=fiedz`, `operator=Jan Fiedler`, "
    "`contact:phone=+49 351 1609217`, `contact:website=https://fiedz.de/`",
    "**Tag-Zitat berichtigt**: node/7717696923 traegt live durchgaengig `contact:phone=+49 351 1609217` "
    "und `contact:website=https://fiedz.de/` (nicht `phone=`/`website=`). "
    "Quelle(n): https://api.openstreetmap.org/api/0.6/node/7717696923.json (Abruf 30.10.2026)")

Path("/tmp/opencode/corrections-part1.json").write_text(
    json.dumps(M, indent=1, ensure_ascii=False))
print("Eintraege Teil 1:", len(M))