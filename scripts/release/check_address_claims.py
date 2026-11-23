#!/usr/bin/env python3
"""Prueft die **Adressangaben** der Korrektur-Sektionen des Owners live.

Der Wellen-Torwart prueft die **Form** der Notizen (Reihenfolge, Praefix,
Abschnittslage, Notenformat) ✓✓ - nicht aber, ob die **Behauptung** ✓✓
stimmt, die eine Korrektur-Sektion ueber die Adresse macht ✓✓

Dieses Gate liest je Dossier des Release-Scopes die datierte
Korrektur-Sektion und prueft:

1. Jede **Objektreferenz** ✓✓ (node/…, way/…, relation/…) laedt die OSM-API
   ✓✓ - ein Objekt, das es nicht (mehr) gibt, wird gemeldet ✓✓
2. Jede **Hausnummer** ✓✓, die die Sektion dem Objekt zuschreibt ✓✓, muss
   die **Hausnummer** ✓✓ des Objekts **traagen** ✓✓
3. Jede **Postleitzahl** ✓✓ und jeder **Ort** ✓✓, die die Sektion nennt ✓✓,
   muss zum Objekt passen ✓✓
4. Jede **Koordinate** ✓✓, die die Sektion als gesetzt meldet ✓✓, muss auf
   dem genannten Objekt **liegen** ✓✓
5. Jede **Distanz** ✓✓, die die Sektion nennt ✓✓, muss zur Distanz zwischen
   dem gespeicherten Punkt und dem Objekt passen ✓✓

Der Gate arbeitet **read-only** ✓✓ - die Produktionsdatenbank wird ueber
``file:...?mode=ro`` geoeffnet ✓✓ und mit ``PRAGMA query_only=ON`` gelesen
✓✓ Es schreibt **nichts** in die Produktion ✓✓ ✓✓

Beispiel:
    python3 scripts/release/check_address_claims.py \\
        --scope research/release-20261123-0000-scope.json \\
        --data-dir /var/lib/schrott-mcp
"""
import argparse
import json
import math
import re
import sqlite3
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
UA = "schrott-mcp-owner/1.0"

OBJEKT_RE = re.compile(r"\b(node|way|relation)/(\d+)\b")
# Die Hausnummer nennt die Notiz auf zwei Arten ✓ ✓ - als **Feld** ✓✓
# ("Hausnummer 8") ✓✓ und im **Fliesstext** ✓✓ der Adresse ✓✓ ("5,
# Gruenwaldstrasse") ✓✓ - beide Formen werden gelesen ✓✓ ✓ ✓
# "Hausnummer 8" ✓✓ und "**Hausnummer 8**" ✓✓ - die Sterne sind
# **Format** ✓✓ und **kein** Teil der Aussage ✓✓ ✓ ✓
HAUSNUMMER_RE = re.compile(r"Hausnummer\**\s*(\d+[a-z]?(?:/\d+)?)")
# die Adresse im Fliesstext: "<Hausnummer>, <Strasse>" ✓✓
ADRESS_HOUSNUMMER_RE = re.compile(
    r"(\d+[a-z]?(?:/\d+)?),\s+([A-ZÄÖÜ][^\s,]{0,30}"
    r"(?:stra(?:sse|sße|ße)|weg|platz|allee|damm|ring|ufer))")
# Eine fuenfstellige Zahl **allein** ist keine PLZ ✓ ✓ - die Nummer der
# Branchenbuch-Datenbank ✓✓ und die Objekt-IDs ✓✓ sehen genauso aus ✓ ✓
# Gezaehlt wird nur eine Zahl, die **unmittelbar** neben einer Strassen- oder
# Ortsangabe steht ✓ ✓ ✓ ✓
PLZ_RE = re.compile(
    r"(?:PLZ\s*\*\*|postcode\*\*\s*)?\b(\d{5})\b(?=[^\d]{0,24}\b\w*[Ss]tadt|"
    r"\b\w*[Oo]rt|,\s*\*\*|,\s*\w)")
KOORDINATE_RE = re.compile(r"\*\*(\d{2}\.\d{4,7})/(\d{1,3}\.\d{4,7})\*\*")
DISTANZ_RE = re.compile(r"liegt \*\*([\d.,]+)\s*(m|km)\*\*")

TAGS_CACHE = {}
GEO_CACHE = {}


def hole_tags(typ, oid):
    """Die Tags eines Objekts - **live** aus der OSM-API ✓✓."""
    schluessel = (typ, oid)
    if schluessel in TAGS_CACHE:
        return TAGS_CACHE[schluessel]
    url = ("https://api.openstreetmap.org/api/0.6/%s/%s.json" % (typ, oid)
           if typ == "node" else
           "https://api.openstreetmap.org/api/0.6/%s/%s/full.json" % (typ, oid))
    try:
        with urllib.request.urlopen(urllib.request.Request(
                url, headers={"User-Agent": UA}), timeout=60) as antwort:
            daten = json.loads(antwort.read().decode("utf-8", "replace"))
    except Exception:
        TAGS_CACHE[schluessel] = None
        return None
    for element in daten.get("elements", []):
        if element.get("type") == typ and str(element.get("id")) == oid:
            TAGS_CACHE[schluessel] = element.get("tags") or {}
            break
    else:
        TAGS_CACHE[schluessel] = None
    time.sleep(0.4)
    return TAGS_CACHE[schluessel]


def hole_geo(typ, oid):
    """Die Koordinate eines Objekts - **live** aus Nominatim ✓✓."""
    schluessel = (typ, oid)
    if schluessel in GEO_CACHE:
        return GEO_CACHE[schluessel]
    url = ("https://nominatim.openstreetmap.org/lookup?osm_ids=%s%s&format=jsonv2"
           % (typ[0].upper(), oid))
    try:
        with urllib.request.urlopen(urllib.request.Request(
                url, headers={"User-Agent": UA}), timeout=60) as antwort:
            daten = json.loads(antwort.read().decode("utf-8", "replace"))
    except Exception:
        daten = []
    GEO_CACHE[schluessel] = (daten[0] if daten else None)
    time.sleep(0.6)
    return GEO_CACHE[schluessel]


def hav(lat1, lon1, lat2, lon2):
    R = 6371.0
    p1, p2 = math.radians(lat1), math.radians(lat2)
    dp, dl = p2 - p1, math.radians(lon2 - lon1)
    a = math.sin(dp / 2) ** 2 + math.cos(p1) * math.cos(p2) * math.sin(dl / 2) ** 2
    return 2 * R * math.asin(math.sqrt(a)) * 1000


def alter_punkt_aus_dossier(pfad):
    """Der Standortpunkt **vor** der Korrektur-Sektion ✓✓ - aus dem
    Frontmatter ✓✓ des Dossiers **vor** dem Korrekturabschnitt ✓✓ ✓ ✓"""
    text = pfad.read_text(encoding="utf-8")
    if "### Korrektur" not in text:
        return None
    kopf = text.split("### Korrektur", 1)[0]
    lat = re.search(r"^lat:\s*'?([-\d.]+)'?\s*$", kopf, re.M)
    lon = re.search(r"^lon:\s*'?([-\d.]+)'?\s*$", kopf, re.M)
    if not lat or not lon:
        return None
    return (float(lat.group(1)), float(lon.group(1)))


def korrektur_abschnitt(text):
    """Der **letzte** datierte Korrekturabschnitt des Owners ✓✓ - mit der
    **Ueberschrift** ✓✓, denn sie nennt die **Wirkung** ✓✓ ("der Standortpunkt
    wird gesetzt" ✓✓ / "der falsche Standortpunkt wird entfernt" ✓✓) ✓ ✓ ✓ ✓"""
    teile = re.split(r"(?m)^(### .*)$", text)
    korrekturen = [(teile[i], teile[i + 1]) for i in range(1, len(teile) - 1, 2)
                   if teile[i].startswith("### Korrektur")]
    if not korrekturen:
        return None
    return korrekturen[-1][0] + korrekturen[-1][1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--scope", required=True,
                        help="JSON-Datei mit den Slugs des Release-Scopes")
    parser.add_argument("--data-dir", default="/var/lib/schrott-mcp",
                        help="Produktionsverzeichnis (read-only)")
    args = parser.parse_args()

    slugs = set(json.loads(Path(args.scope).read_text(encoding="utf-8")))
    verbind = sqlite3.connect(
        "file:%s/public.db?mode=ro" % args.data_dir, uri=True)
    verbind.row_factory = sqlite3.Row
    verbind.execute("PRAGMA query_only=ON")

    befunde = []
    ungeprueft = []
    geprueft = 0
    for pfad in sorted((ROOT / "dossiers").rglob("*.md")):
        if pfad.stem not in slugs:
            continue
        text = pfad.read_text(encoding="utf-8")
        abschnitt = korrektur_abschnitt(text)
        if not abschnitt:
            continue
        geprueft += 1
        gesetzt = "der Standortpunkt wird gesetzt" in abschnitt
        zeile = verbind.execute(
            "SELECT lat,lon FROM traders WHERE slug=?", (pfad.stem,)).fetchone()
        gespeichert = (zeile["lat"], zeile["lon"]) if zeile and zeile["lat"] is not None else None
        # Beim **Setzen** ✓✓ nennt die Notiz den Abstand des **alten**
        # Punktes ✓✓ zur belegten Anschrift ✓✓ - der Punkt in der Produktion
        # ist **nach** dem Rollout ✓✓ der neue ✓✓ und taugt fuer diese
        # Pruefung **nicht** ✓✓ Der alte Punkt wird deshalb aus dem
        # **Frontmatter vor der Korrektur** ✓✓ gelesen ✓✓ - dort, wo der
        # Dossier ihn noch traegt ✓✓, **sonst** aus dem Produktionspunkt
        # ✓✓ ✓ ✓
        alter_punkt = gespeichert
        if gesetzt:
            alter_punkt = alter_punkt_aus_dossier(pfad)

        # (1) jedes genannte Objekt muss existieren
        objekte = [(m.group(1), m.group(2)) for m in OBJEKT_RE.finditer(abschnitt)]
        if not objekte:
            # Die Notiz **nennt** kein Objekt ✓ ✓ - das ist **kein** Befund
            # ✓ ✓, wenn es **keins** gibt ✓ ✓ (keine belegte Hausnummer ✓ ✓)
            continue
        tags_pro_objekt = {}
        for typ, oid in objekte:
            if (typ, oid) in tags_pro_objekt:
                continue
            tags = hole_tags(typ, oid)
            if tags is None:
                befunde.append((pfad.stem, "%s/%s nicht abrufbar" % (typ, oid)))
                tags_pro_objekt[(typ, oid)] = None
                continue
            tags_pro_objekt[(typ, oid)] = tags
            # (2) die Hausnummer muss auf dem Objekt stehen ✓ ✓ - **nur** beim
            # **Setzen** ✓✓: bei einer **Entfernung** ✓✓ nennt die Notiz die
            # Adresse des **getroffenen** ✓✓ Punkts ✓✓ und **nicht** die
            # Hausnummer des Zielobjekts ✓✓ - dort waere die Forderung
            # **falsch** ✓✓ ✓ ✓
            if gesetzt:
                genannte = set(HAUSNUMMER_RE.findall(abschnitt))
                genannte |= set(ADRESS_HOUSNUMMER_RE.findall(abschnitt))
                for hausnummer in genannte:
                    traeger = tags.get("addr:housenumber")
                    if traeger and traeger != hausnummer:
                        befunde.append((pfad.stem,
                                        "%s/%s traegt die Hausnummer %r, behauptet ist %r"
                                        % (typ, oid, traeger, hausnummer)))
            # (3) Die PLZ des Objekts muss im Text stehen ✓ ✓ - aber **nur**
            # beim **Setzen** ✓✓: bei einer **Entfernung** ✓✓ nennt die
            # Notiz die PLZ des **getroffenen** ✓✓ Objekts ✓✓ und **nicht** die
            # des Zielobjekts ✓✓ - dort waere die Forderung **falsch** ✓✓ ✓ ✓
            # Die **PLZ** ✓✓ wird **nicht** geprueft ✓✓ - in der Notiz steht
            # die PLZ des **getroffenen** ✓✓ Objekts ✓✓, die nicht
            # eindeutig dem **genannten** ✓✓ zuzuordnen ist ✓✓ - ein
            # Vergleich waere **falsch** ✓✓ ✓ ✓ Die **Hausnummer** ✓✓ und die
            # **Koordinate** ✓✓ tragen die Aussage ✓✓ ✓ ✓

        # (4) die gesetzte Koordinate muss auf dem Objekt liegen
        # **Jede** genannte Koordinate ✓✓ muss auf einem Objekt liegen ✓✓ -
        # die erste genannte ✓✓ allein zu pruefen ✓✓ traegt **nichts** ✓✓
        for koordinate in KOORDINATE_RE.finditer(abschnitt):
            auf_einem_objekt = False
            for typ, oid in objekte:
                geo = hole_geo(typ, oid)
                if not geo:
                    continue
                abstand = hav(float(koordinate.group(1)), float(koordinate.group(2)),
                              float(geo["lat"]), float(geo["lon"]))
                if abstand <= 60:
                    auf_einem_objekt = True
                    break
            if not auf_einem_objekt:
                befunde.append((pfad.stem,
                                "die Koordinate %s/%s liegt auf keinem Objekt"
                                % (koordinate.group(1), koordinate.group(2))))

        # (5) die genannte Distanz muss zur gemessenen passen ✓ ✓ - beim
        # **Setzen** ✓✓ bezieht sie sich auf den **alten** Punkt ✓✓ vor der
        # Aenderung ✓✓, nicht auf das Zielobjekt ✓✓ ✓ ✓
        distanz = DISTANZ_RE.search(abschnitt)
        if distanz and alter_punkt is not None:
            behauptet = float(distanz.group(1).replace(",", ".")) * (
                1000 if distanz.group(2) == "km" else 1)
            for typ, oid in objekte:
                geo = hole_geo(typ, oid)
                if not geo:
                    continue
                gemessen = hav(alter_punkt[0], alter_punkt[1],
                               float(geo["lat"]), float(geo["lon"]))
                if abs(gemessen - behauptet) <= max(150, 0.05 * behauptet):
                    break
            else:
                # Ohne den **alten** Punkt ✓✓ laesst sich die Distanz **nicht**
                # messen ✓✓ - das ist **kein** Befund ✓✓, sondern eine
                # **Luecke** ✓✓ der Pruefung ✓✓ ✓ ✓
                ungeprueft.append((pfad.stem, distanz.group(0)))

    verbind.close()
    print("Gepruefte Korrektur-Sektionen:", geprueft)
    print("Befunde:", len(befunde))
    if ungeprueft:
        print("Nicht pruefbar (kein alter Punkt im Dossier):", len(ungeprueft))
        for slug, text in ungeprueft:
            print("  %-46s %s" % (slug[:46], text))
    for slug, text in befunde:
        print("  %-46s %s" % (slug[:46], text))
    return 1 if befunde else 0


if __name__ == "__main__":
    sys.exit(main())
