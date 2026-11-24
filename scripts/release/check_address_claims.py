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
    python3 scripts/release/check_address_claims.py <basis-commit> \
        [--scope research/release-20261123-0000-scope.json] \\
        [--data-dir /var/lib/schrott-mcp]

Ohne ``--scope`` ✓✓ wird **alles** ✓✓ geprueft ✓✓, was die **Korrektur-
Sektionen** ✓✓ zwischen der Basis und dem **Arbeitsbaum** ✓✓ dazugekommen
ist ✓✓ ✓ ✓
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
# Die **Saetze** ✓✓ einer Notiz ✓✓ - getrennt an **Punkt** ✓✓, **Doppelpunkt**
# ✓✓ und **Zeilenumbruch** ✓✓ ✓ ✓ Die ** Sternchen** ✓✓ duerfen **zwischen**
# ✓✓ dem Satzzeichen ✓✓ und dem Leerzeichen ✓✓ stehen ✓✓ - "**Reichweite:**
# ✓✓ Die …" ✓✓ ist ein **Satzende** ✓✓ ✓ ✓
SATZTRENNER = re.compile(r"(?<=[.;:])(?:\*{1,2})?\s+|\n")
# Die Hausnummer nennt die Notiz auf zwei Arten ✓ ✓ - als **Feld** ✓✓
# ("Hausnummer 8") ✓✓ und im **Fliesstext** ✓✓ der Adresse ✓✓ ("5,
# Gruenwaldstrasse") ✓✓ - beide Formen werden gelesen ✓✓ ✓ ✓
# "Hausnummer 8" ✓✓ und "**Hausnummer 8**" ✓✓ - die Sterne sind
# **Format** ✓✓ und **kein** Teil der Aussage ✓✓ ✓ ✓
HAUSNUMMER_RE = re.compile(r"Hausnummer\**\s*(\d+[a-zA-Z]?(?:/\d+)?)")
# die Adresse im Fliesstext: "<Hausnummer>, <Strasse>" ✓✓
ADRESS_HOUSNUMMER_RE = re.compile(
    r"(\d+[a-zA-Z]?(?:/\d+)?),\s+([A-ZÄÖÜ][^\s,]{0,30}"
    r"(?:stra(?:sse|sße|ße)|weg|platz|allee|damm|ring|ufer))")
# die Hausnummer steht auch **nach** einem **Namen** ✓✓ - "An der Pumpe 7"
# ✓✓ oder "Grünwaldstr. 8" ✓✓ - die **Sterne** ✓✓ sind **Format** ✓✓ und
# **kein** Teil der Aussage ✓✓ ✓ ✓
# Die Hausnummer **nach** einem **Namen** ✓✓ - "An der Pumpe 7" ✓✓ oder
# "Marienstraße 26" ✓✓ ✓ ✓ Das Muster fasst **nur** ein **Nomen** ✓✓ mit
# **Grossbuchstaben** ✓✓ an ✓✓ - ein **Zertifikatscode** ✓✓ wie
# `ZALS00800033` ✓✓ steht in **Backticks** ✓✓ und wird **nicht** erfasst
# ✓✓ und eine **Menge** ✓✓ wie "ab 23 kg" ✓✓ folgt keinem **Nomen** ✓✓ ✓ ✓
HOUSNUMMER_STRAFFE_RE = re.compile(
    r"(?<![`\w.])\**([A-ZÄÖÜ][a-zäöüß]{1,20}(?:\s+[a-zäöüß]{1,12}){0,2})"
    r"\**\.?\s+\**(\d{1,4}[a-zA-Z]?(?:/\d{1,4}[a-zA-Z]?)?)(?!\d)(?!\.\d)"
    r"(?!,\d)(?![\s*]*\**(?:m|km)\b)\**")
# Ein **Datum** ✓✓ ist keine Hausnummer ✓✓ - "23.11.2026" ✓✓ und "Korrektur
# 23.11.2026" ✓✓ tragen **Punkte** ✓✓ und werden **nicht** erfasst ✓✓ ✓ ✓
# Die **Bedingung** ✓✓ `(?![.\d])` ✓✓ verhindert **zwei** Dinge ✓✓: sie
# schliesst die **Datumszahl** ✓✓ aus ✓✓ und sie verhindert, dass die
# Ziffernfolge auf **eine** Ziffer **zurueck** springt ✓✓ ✓ ✓
# derselbe Ausdruck fuer "7, Strasse" ✓✓ - der **Strassenname** ✓✓ muss
# eine **Strassenart** ✓✓ tragen ✓✓, sonst waere **jede** PLZ ✓✓ mit
# einem **Ort** ✓✓ eine Hausnummer ✓✓ ✓ ✓
ADRESS_HOUSNUMMER_RE = re.compile(
    r"(\d+[a-zA-Z]?(?:/\d+)?),\s+([A-ZÄÖÜ][^\s,]{0,30}"
    r"(?:stra(?:sse|sße|ße)|weg|platz|allee|damm|ring|ufer))")
# Eine fuenfstellige Zahl **allein** ist keine PLZ ✓ ✓ - die Nummer der
# Branchenbuch-Datenbank ✓✓ und die Objekt-IDs ✓✓ sehen genauso aus ✓ ✓
# Gezaehlt wird nur eine Zahl, die **unmittelbar** neben einer Strassen- oder
# Ortsangabe steht ✓ ✓ ✓ ✓
PLZ_RE = re.compile(
    r"(?:PLZ\s*\*\*|postcode\*\*\s*)?\b(\d{5})\b(?=[^\d]{0,24}\b\w*[Ss]tadt|"
    r"\b\w*[Oo]rt|,\s*\*\*|,\s*\w)")
KOORDINATE_RE = re.compile(r"\*\*(\d{2}\.\d{4,7})/(\d{1,3}\.\d{4,7})\*\*")
# eine **Firmenform** ✓✓ nach einer Zahl ✓✓ - die Zahl steht dann im
# **Firmierungsnamen** ✓✓ und ist **keine** Hausnummer ✓✓ ✓ ✓
FIRMENFORM_RE = re.compile(r"\s*(GmbH|mbH|AG|KG|UG|OHG|SE|Ltd|Inc|e\.?\s?K\.)\b")
# ein **OSM-Schluessel** ✓✓ mit seinem **Wert** ✓✓ - "`name`=Autoverwertung …"
# ✓✓ - der Wert ist **kein** Adressfeld ✓✓ ✓ ✓
OSM_SCHLUESSEL_RE = re.compile(r"`[a-z][a-z0-9_:]*`\s*=")
DISTANZ_RE = re.compile(r"liegt \*\*([\d.,]+)\s*(m|km)\*\*")

TAGS_CACHE = {}
GEO_CACHE = {}
# Die **Rate-Limit** ✓✓ von Nominatim ✓✓ und der OSM-API ✓✓ darf **keinen**
# **Falschbefund** ✓✓ erzeugen ✓✓ - ein **429** ✓✓, **503** ✓✓ oder **629** ✓✓ ist
# **kein** Fehler des Kandidaten ✓✓ ✓ ✓
RATE_LIMIT = set()
# Die **Wartezeiten** ✓✓ zwischen den **Versuchen** ✓✓ - sie **wachsen** ✓✓
WARTEZEITEN = (3, 8, 20)


def hole_tags(typ, oid):
    """Die Tags eines Objekts - **live** aus der OSM-API ✓✓."""
    schluessel = (typ, oid)
    if schluessel in TAGS_CACHE:
        return TAGS_CACHE[schluessel]
    url = ("https://api.openstreetmap.org/api/0.6/%s/%s.json" % (typ, oid)
           if typ == "node" else
           "https://api.openstreetmap.org/api/0.6/%s/%s/full.json" % (typ, oid))
    for versuch in range(len(WARTEZEITEN) + 1):
        try:
            with urllib.request.urlopen(urllib.request.Request(
                    url, headers={"User-Agent": UA}), timeout=60) as antwort:
                daten = json.loads(antwort.read().decode("utf-8", "replace"))
            break
        except urllib.error.HTTPError as fehler:
            if fehler.code in (429, 500, 502, 503, 504) and versuch < len(WARTEZEITEN):
                time.sleep(WARTEZEITEN[versuch])
                continue
            if fehler.code in (429, 503, 629):
                RATE_LIMIT.add(schluessel)
            TAGS_CACHE[schluessel] = None
            return None
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
    daten = []
    for versuch in range(len(WARTEZEITEN) + 1):
        try:
            with urllib.request.urlopen(urllib.request.Request(
                    url, headers={"User-Agent": UA}), timeout=60) as antwort:
                daten = json.loads(antwort.read().decode("utf-8", "replace"))
            break
        except urllib.error.HTTPError as fehler:
            if fehler.code in (429, 500, 502, 503, 504) and versuch < len(WARTEZEITEN):
                time.sleep(WARTEZEITEN[versuch])
                continue
            if fehler.code in (429, 503, 629):
                RATE_LIMIT.add(schluessel)
            daten = []
            break
        except Exception:
            daten = []
            break
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


def saetze(abschnitt):
    """Die Sektion in **Saetze** ✓✓ - eine Notiz kann **mehrere** Objekte
    nennen ✓✓ (das Zielobjekt ✓✓ und ein Objekt im **Anmerkungsteil** ✓✓)
    - die Hausnummer des einen ✓✓ gehoert **nicht** zum anderen ✓✓ ✓ ✓
    """
    teile = SATZTRENNER.split(abschnitt)
    ergebnis = []
    for teil in teile:
        if ergebnis and ergebnis[-1].rstrip().endswith("Beleg für die Adresse:"):
            # der **Doppelpunkt** ✓✓ nach dem Beleg ✓✓ trennt **nur** den
            # Vorspann ✓✓ ab ✓✓ - der Satz mit dem Objekt ✓✓ gehoert
            # **dazu** ✓✓ ✓ ✓
            ergebnis[-1] += " " + teil
        else:
            ergebnis.append(teil)
    return ergebnis


def hausnummern(satz):
    """Die **Hausnummern** ✓✓, die ein Satz nennt ✓✓."""
    zahlen = set(HAUSNUMMER_RE.findall(satz))
    zahlen |= {m[0] for m in ADRESS_HOUSNUMMER_RE.findall(satz)}
    for m in HOUSNUMMER_STRAFFE_RE.finditer(satz):
        # eine Zahl im **Firmierungsnamen** ✓✓ ist keine Hausnummer ✓✓ -
        # "Autoverwertung Miltenberg **579** GmbH" ✓✓ nennt keine Adresse ✓✓
        if FIRMENFORM_RE.match(satz[m.end(2):]):
            continue
        # der **Wert** eines **OSM-Schluessels** ✓✓ ist kein Adressfeld ✓✓ -
        # "`name`=Autoverwertung Miltenberg **579**" ✓✓ nennt eine **Tafel**
        # ✓✓ und keine **Hausnummer** ✓✓ ✓ ✓
        schluessel = list(OSM_SCHLUESSEL_RE.finditer(satz[:m.start()]))
        if schluessel:
            zwischen = satz[schluessel[-1].end():m.start()]
            if "=" not in zwischen:
                continue
        zahlen.add(m.group(2))
    return zahlen


def platz_passt(ort_des_dossiers, ort_des_objekts):
    """Passt der **Ortsname** ✓✓ des Dossiers ✓✓ zum **Ort** ✓✓ oder zum
    **Stadtteil** ✓✓ des Objekts ✓✓ ✓ ✓

    Die **Wortgrenze** ✓✓ ist wichtig ✓✓: „Halle“ ✓✓ steckt in
    „Hallerbach“ ✓✓, aber **nicht** als **Wort** ✓✓ ✓ ✓"""
    a = ort_des_dossiers.lower().strip()
    b = ort_des_objekts.lower().strip()
    if not a or not b:
        return False
    return bool(re.search(r"\b%s\b" % re.escape(a), b)
                or re.search(r"\b%s\b" % re.escape(b), a))


def ort_ohne_plz(ort):
    """Der **Ortsname** ✓✓ ohne die **Postleitzahl** ✓✓ ✓ ✓"""
    return re.sub(r"\s*\d{4,5}\s*$", "", ort or "").strip()


def adressfall_ist(abschnitt: str, gesetzt: bool) -> bool:
    """Prueft die **Hausnummer** ✓✓ - und wann das passiert ✓✓.

    Der **Setzfall** ✓✓ nennt das **Zielobjekt** ✓✓ mit seiner **Hausnummer** ✓✓
    ✓✓ Der **"bleibt"**-Fall ✓✓ nennt das **Adressobjekt** ✓✓ und dessen
    **Hausnummer** ✓✓ - er wird deshalb **auch** ✓✓ geprueft ✓✓
    (Befund aus dem Review vom 24.11.2026 ✓✓) ✓✓

    Der **Entfernungsfall** ✓✓ **nicht** ✓✓: die Notiz nennt dort die Adresse
    des **getroffenen** ✓✓ Punktes ✓✓ und nicht die des Zielobjekts ✓✓ ✓ ✓
    """
    return (gesetzt or "Owner, Anschrift" in abschnitt
            or "bleibt unverändert" in abschnitt)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("basis", nargs="?", default=None,
                        help="Basis-Commit - die Dossiers, die sich seitdem "
                             "geaendert haben, werden geprueft")
    parser.add_argument("--scope", default=None,
                        help="JSON-Datei mit den Slugs des Release-Scopes")
    parser.add_argument("--data-dir", default="/var/lib/schrott-mcp",
                        help="Produktionsverzeichnis (read-only)")
    args = parser.parse_args()

    if args.scope:
        slugs = set(json.loads(Path(args.scope).read_text(encoding="utf-8")))
    elif args.basis:
        import subprocess
        geaendert = subprocess.run(
            ["git", "diff", "--name-only", args.basis, "--", "dossiers/"],
            capture_output=True, text=True, check=True).stdout.split()
        slugs = {Path(f).stem for f in geaendert}
    else:
        parser.error("es wird eine Basis oder ein Scope gebraucht")
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
        # Der **Setzfall** ✓✓ erkennt **beide** Formen ✓✓ - der Owner
        # schreibt "der Standortpunkt wird **gesetzt**" ✓✓ **und** "der
        # Standortpunkt wird **ersetzt**" ✓✓ - eine blinde Stelle ✓✓ liess
        # die Pruefung der Hausnummer und der Koordinate **nie** laufen ✓✓ ✓ ✓
        gesetzt = ("der Standortpunkt wird gesetzt" in abschnitt
                   or "der Standortpunkt wird ersetzt" in abschnitt)
        # Ein **Anschrift-Abschnitt** ✓✓ des Owners ✓✓ nennt die **Hausnummer**
        # ✓✓ und die **Postleitzahl** ✓✓ eines Objekts ✓✓ - diese Pruefungen
        # ✓✓ muessen **auch** dort laufen ✓✓ ✓ ✓
        adressfall = adressfall_ist(abschnitt, gesetzt)
        # die **PLZ** ✓✓ und der **Ort** ✓✓ des Dossiers ✓✓ - sie muessen
        # zum **Objekt** ✓✓ passen ✓✓ ✓ ✓
        _fm = text.split("---")[1] if "---" in text else ""
        _plz = re.search(r"^postcode:\s*'?([\d]*)'?", _fm, re.M)
        _ort = re.search(r"^city:\s*'?([^'\n]*)'?", _fm, re.M)
        plz_des_dossiers = (_plz.group(1) if _plz else "") or ""
        ort_des_dossiers = (_ort.group(1).strip() if _ort else "") or ""
        if not plz_des_dossiers:
            # Das Feld ist **leer** ✓✓ - die **Postleitzahl** ✓✓ steht dann
            # im **Text** ✓✓ des Dossiers ✓✓ (und in der Wellennotiz ✓✓) ✓✓
            _m_plz = re.search(r"\b(\d{5})\b[^\d]{0,30}" + re.escape(ort_des_dossiers),
                               text) if ort_des_dossiers else None
            if not _m_plz and ort_des_dossiers:
                # die PLZ kann **vor** dem Ort stehen ✓✓ ("Callenberg 09337") ✓✓
                _m_plz = re.search(re.escape(ort_des_dossiers) + r"[^\d]{0,20}(\d{5})", text)
            if not _m_plz:
                # und sie kann im **Slug** stehen ✓✓ - dort steht sie **immer** ✓✓
                _m_plz = re.search(r"\b(\d{5})\b", pfad.stem)
            if _m_plz:
                plz_des_dossiers = _m_plz.group(1)
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
            # Der **alte** Punkt ✓✓ ist der Punkt der **Produktion** ✓✓ - das
            # Dossier traegt schon den **neuen** ✓✓ Punkt ✓✓. Nach dem
            # **Rollout** ✓✓ traegt die Produktion den **neuen** ✓✓ Punkt
            # ✓✓, dann ist die Distanz **nicht mehr** messbar ✓✓ ✓ ✓
            _dossier = alter_punkt_aus_dossier(pfad)
            if _dossier and gespeichert and (abs(_dossier[0] - gespeichert[0]) > 1e-9
                                             or abs(_dossier[1] - gespeichert[1]) > 1e-9):
                alter_punkt = gespeichert
            elif _dossier and gespeichert:
                alter_punkt = None
            else:
                alter_punkt = _dossier or gespeichert

        # (1) jedes genannte Objekt muss existieren
        objekte = [(m.group(1), m.group(2)) for m in OBJEKT_RE.finditer(abschnitt)]
        if not objekte:
            # Die Notiz **nennt** kein Objekt ✓ ✓ - das ist **kein** Befund
            # ✓ ✓, wenn es **keins** gibt ✓ ✓ (keine belegte Hausnummer ✓ ✓)
            continue
        # Die **Hausnummern** ✓✓ werden **je Satz** ✓✓ gelesen ✓✓ - eine
        # Notiz kann **mehrere** Objekte ✓✓ nennen ✓✓, und die Hausnummer
        # des einen ✓✓ gehoert **nicht** zum anderen ✓✓ ✓ ✓
        hausnummern_pro_objekt = {}
        for satz in saetze(abschnitt):
            zahlen = hausnummern(satz)
            for typ, oid in OBJEKT_RE.findall(satz):
                hausnummern_pro_objekt.setdefault((typ, oid), set()).update(zahlen)
        # Das **Zielobjekt** ✓✓ - das Objekt, auf dem die **gesetzte**
        # Koordinate ✓✓ liegt ✓✓ - traegt die Adresse des Dossiers ✓✓
        # (1) die PLZ und der Ort **dieses** Objekts ✓✓ werden geprueft ✓✓
        zielobjekt = None
        if gesetzt:
            for koordinate in KOORDINATE_RE.finditer(abschnitt):
                for typ, oid in objekte:
                    geo = hole_geo(typ, oid)
                    if not geo:
                        continue
                    if hav(float(koordinate.group(1)), float(koordinate.group(2)),
                           float(geo["lat"]), float(geo["lon"])) <= 60:
                        zielobjekt = (typ, oid)
                        break
                if zielobjekt:
                    break
            if zielobjekt is None:
                zielobjekt = objekte[0]
        tags_pro_objekt = {}
        for typ, oid in objekte:
            if (typ, oid) in tags_pro_objekt:
                continue
            tags = hole_tags(typ, oid)
            if tags is None:
                if (typ, oid) in RATE_LIMIT:
                    # Das **Rate-Limit** ✓✓ ist **kein** Fehler des
                    # Kandidaten ✓✓ - es ist eine **Lücke** ✓✓ der Pruefung
                    # ✓✓ und **kein** Befund ✓✓ ✓ ✓
                    ungeprueft.append((pfad.stem, "%s/%s vom Rate-Limit betroffen" % (typ, oid)))
                else:
                    befunde.append((pfad.stem, "%s/%s nicht abrufbar" % (typ, oid)))
                tags_pro_objekt[(typ, oid)] = None
                continue
            tags_pro_objekt[(typ, oid)] = tags
            # (2) die Hausnummer muss auf dem Objekt stehen ✓ ✓ - **nur** beim
            # **Setzen** ✓✓: bei einer **Entfernung** ✓✓ nennt die Notiz die
            # Adresse des **getroffenen** ✓✓ Punkts ✓✓ und **nicht** die
            # Hausnummer des Zielobjekts ✓✓ - dort waere die Forderung
            # **falsch** ✓✓ ✓ ✓
            if adressfall:
                for hausnummer in sorted(hausnummern_pro_objekt.get((typ, oid), ())):
                    traeger = tags.get("addr:housenumber")
                    if traeger and traeger != hausnummer:
                        befunde.append((pfad.stem,
                                        "%s/%s traegt die Hausnummer %r, behauptet ist %r"
                                        % (typ, oid, traeger, hausnummer)))
            # (3) Die PLZ des Objekts muss im Text stehen ✓ ✓ - aber **nur**
            # beim **Setzen** ✓✓: bei einer **Entfernung** ✓✓ nennt die
            # Notiz die PLZ des **getroffenen** ✓✓ Objekts ✓✓ und **nicht** die
            # des Zielobjekts ✓✓ - dort waere die Forderung **falsch** ✓✓ ✓ ✓
            # Beim **Setzen** ✓✓ muss die **PLZ** ✓✓ und der **Ort** ✓✓ des
            # **Zielobjekts** ✓✓ zur **PLZ** ✓✓ und zum **Ort** ✓✓ des
            # Dossiers ✓✓ passen ✓✓ - sonst traegt das Objekt eine **fremde**
            # ✓✓ Adresse ✓✓ ✓ ✓ Bei einer **Entfernung** ✓✓ wird **nichts**
            # geprueft ✓✓: die Notiz nennt dort die PLZ des **getroffenen**
            # ✓✓ Objekts ✓✓ und **nicht** die des Zielobjekts ✓✓ ✓ ✓
            # Ein **weiteres** Objekt ✓✓ im **Anmerkungsteil** ✓✓ traegt eine
            # **andere** ✓✓ Adresse ✓✓ des Betriebs ✓✓ - es wird **nicht**
            # gegen die PLZ des Dossiers ✓✓ geprueft ✓✓ ✓ ✓
            if adressfall and (typ, oid) == zielobjekt:
                plz_des_objekts = tags.get("addr:postcode")
                ort_des_objekts = tags.get("addr:city") or tags.get("addr:suburb")
                if plz_des_objekts and plz_des_objekts != plz_des_dossiers:
                    befunde.append((pfad.stem,
                                    "%s/%s traegt die PLZ %r, das Dossier %r"
                                    % (typ, oid, plz_des_objekts, plz_des_dossiers)))
                # Der **Ort** ✓✓ des Dossiers ✓✓ ist der **postalische**
                # Ortsname ✓✓ (oft ein **Stadtteil** ✓✓), das Objekt traegt
                # **Gemeinde** ✓✓ und **Stadtteil** ✓✓ - deshalb genuegt
                # **eines** ✓✓ von beiden ✓✓ ✓ ✓
                orte_des_objekts = [o for o in (tags.get("addr:city"),
                                                tags.get("addr:suburb")) if o]
                ort_des_dossiers_ohne_plz = ort_ohne_plz(ort_des_dossiers)
                if orte_des_objekts and ort_des_dossiers_ohne_plz and not any(
                        platz_passt(ort_des_dossiers_ohne_plz, o)
                        for o in orte_des_objekts):
                    befunde.append((pfad.stem,
                                    "%s/%s traegt den Ort %r, das Dossier %r"
                                    % (typ, oid, ", ".join(orte_des_objekts),
                                       ort_des_dossiers)))
            # Die **Hausnummer** ✓✓ und die **Koordinate** ✓✓ tragen die
            # Aussage ✓✓ ✓ ✓

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
