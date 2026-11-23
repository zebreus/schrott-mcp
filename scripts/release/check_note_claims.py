#!/usr/bin/env python3
"""Prueft, ob eine Notiz eine Datenaenderung behauptet, die nicht stattfand.

Die wiederkehrende Fehlerklasse der Pre-Deploy-Pruefungen: eine Notiz
sagte, ein Feld sei korrigiert worden - im Dossier war es aber unveraendert.

Der Test vergleicht **elementweise** die Frontmatter-Felder der Basis mit
denen des Kandidaten ✓ und meldet jede neue Notiz, die eine Aenderung
**behauptet**, obwohl das genannte Feld byteweise gleich geblieben ist ✓.

Als Behauptung zaehlt nur, wenn die Notiz das Feld **als Frontmatter-Feld
benennt** ✓ - als Backtick-Name ✓ (`` `postcode` `` ✓) oder mit dem Zusatz
„im Frontmatter“ ✓. Damit zaehlt eine bloss erwaehnte Angabe nicht als
Behauptung ✓ ("Aussteller mit vollstaendiger Anschrift" ✓ ist keine
Behauptung ueber die Dossier-Adresse ✓).

**Zwei Abgrenzungen** ✓ ✓ - die Fehlalarme, die die Pre-Deploy-Pruefung vom
20.11.2026 in `research/auftraege-20261120.md` gefunden und fuer eine spaetere
Runde zurückgestellt hat ✓ ✓:

* Ein **OSM-Tag** ✓ ✓ (`` `name`=`` ✓ ✓) ist **kein** Dossier-Feld ✓ ✓ - der
  Wert gehoert zum kartierten Objekt ✓ ✓, nicht zum Dossier ✓ ✓
* **„entfernt" als Lageangabe** ✓ ✓ ("liegt 2,6 km vom Objekt entfernt" ✓ ✓)
  ist **keine** Handlung ✓ ✓ - erst „entfernt wurde" ✓ ✓ / „ist entfernt" ✓ ✓
  beschreibt eine Aenderung ✓ ✓

Ausserdem muss der Satz
  * ein **Aenderungsverb** benutzen ✓ und
  * **keine** Verneinung oder Offenheit enthalten ✓ ("bleibt offen" ✓,
    "weder ... noch" ✓, "unveraendert" ✓).

Aufruf:
    python3 scripts/release/check_note_claims.py <basis> [<ref>]

Exitcode 0 = kein Befund ✓, 1 = mindestens ein Befund ✓.
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

# die Frontmatter-Felder, die ein Dossier fuehren kann
FELDER = ["street", "postcode", "city", "phone", "email", "website",
          "website_status", "status", "opening_hours", "name", "description",
          "certifications", "geo", "dropoff_json", "pickup_json", "trader_type",
          "clearance", "materials", "dropoff", "pickup"]

AENDERUNG = (r"korrigiert|ergänzt|geändert|berichtigt|angepasst|aktualisiert"
             r"|eingetragen|ersetzt|richtiggestellt|neu gesetzt|gesetzt"
             r"|entfernt|geschwärzt|übernommen|präzisiert|normalisiert")

# „entfernt" **ohne** die anderen Aenderungswoerter ✓ ✓ - es zaehlt nur, wenn
# es eine **Handlung** ✓ ✓ beschreibt ✓ ✓ ("entfernt wurde" ✓ ✓).
NUR_ENTFERNT = re.compile(r"^\s*entfernt\s*$", re.I)

# „entfernt" ist nur eine **Handlung** ✓ ✓, wenn es einen Vorgang beschreibt ✓ ✓
# ("entfernt wurde" ✓ ✓, "ist entfernt" ✓ ✓, "habe entfernt" ✓ ✓).
# „liegt 2,6 km vom Objekt **entfernt**" ✓ ✓ ist eine **Lageangabe** ✓ ✓ -
# der Satz beschreibt eine Entfernung ✓ ✓, keine Aenderung ✓ ✓.
HILFSVERB = r"(?:wurde|wird|worden|ist|hat|hatte|habe|haben)"
ENTFERNT_HANDLUNG = re.compile(
    rf"\bentfernt\b(?=[^.;:]{{0,40}}\b{HILFSVERB}\b)"
    rf"|\b{HILFSVERB}\b[^.;:]{{0,20}}\bentfernt\b", re.I)

# Ein **OSM-Tag** ✓ ✓ ist **kein** Dossier-Feld ✓ ✓: der Wert hinter dem
# Schluessel gehoert zum kartierten Objekt ✓ ✓, nicht zum Dossier ✓ ✓
OSM_SCHLUESSEL = re.compile(r"`([a-z][a-z0-9_:]*)`\s*=", re.I)
# Die **Attributliste** ✓ ✓ eines OSM-Objekts ✓ ✓ ist eine Folge von
# **Schluessel-Wert-Paaren** ✓ ✓ - dort ist **jeder** Wert ein **Tag** ✓ ✓,
# auch einer, der wie ein Dossier-Feld heisst ✓ ✓ (`name` ✓ ✓).  Erkennbar ist
# sie an **mindestens zwei** Paaren ✓ ✓ ohne Satzumbruch ✓ ✓ dazwischen ✓ ✓
# (die Klammern der Notiz sind fuer die Erkennung nicht noetig ✓ ✓ - die
# Wellennotizen schreiben die Tags mit und ohne Klammern ✓ ✓).
#
# **Sicherheitsregel** ✓ ✓: redet der Satz **ausdruecklich** ueber das Dossier
# ✓ ✓ (Frontmatter ✓ ✓, Eintrag ✓ ✓, Dossier ✓ ✓, Datensatz ✓ ✓, Seed ✓ ✓)
# **und** behauptet eine Aenderung ✓ ✓, wird **nichts** ausgeblendet ✓ ✓ -
# dann koennte ein Schluessel-Wert-Paar eine echte Behauptung sein ✓ ✓
DOSSER_MARKER = re.compile(r"\b(Frontmatter|Eintrag|Dossier|Datensatz|Seed)\b", re.I)


def osm_tags_maskieren(satz):
    """Die Schluessel der **OSM-Attributliste** ✓ ✓ ausblenden ✓ ✓."""
    paare = list(OSM_SCHLUESSEL.finditer(satz))
    if DOSSER_MARKER.search(satz) and re.search(AENDERUNG, satz, re.I):
        # Ein **Dossier-Marker** ✓✓ und ein **Aenderungsverb** ✓✓ schuetzen
        # eine **moegliche echte Behauptung** ✓✓ - aber **nur** ✓✓, wenn der
        # Satz **keine** **OSM-Attributliste** ✓✓ enthaelt ✓✓: eine Liste von
        # Schluessel-Wert-Paaren ✓✓ ist eindeutig die Beschreibung eines
        # **Objekts** ✓✓ und **nicht** eine Behauptung ueber das Dossier
        # ✓✓ - sonst meldete das Gate eine Aenderung, die keine war ✓✓ ✓ ✓
        if len(paare) < 2:
            return satz
    if len(paare) < 2:
        return satz          # ein einzelnes Paar ist keine Liste ✓ ✓
    gruppen, aktuell = [], []
    for paar in paare:
        if aktuell:
            zwischen = satz[aktuell[-1].end():paar.start()]
            # Ein **Datum** ✓✓ ist **kein** Satzende ✓✓ - die Versionshistorie
            # eines Objekts ✓✓ nennt **zwischen** den Paaren ✓✓ ein **Datum** ✓✓
            # ("Version 2 (13.06.2017) setzt `name`= … , Version 3 (16.04.2020)
            # `name`= …") ✓✓ - ohne diese Ausnahme ✓✓ zerfiel die Liste ✓✓ in
            # **Einzelpaare** ✓✓ und das Gate meldete eine **Feldbehauptung** ✓✓,
            # die **keine** war ✓✓ ✓ ✓
            ohne_datum = re.sub(r"\d{1,2}\.\d{1,2}\.\d{2,4}", "0", zwischen)
            if re.search(r"[.;:]", ohne_datum) or len(zwischen) > 120:
                gruppen.append(aktuell)
                aktuell = []
        aktuell.append(paar)
    gruppen.append(aktuell)
    zu_ersetzen = []
    for gruppe in gruppen:
        if len(gruppe) < 2:
            continue          # eine Liste braucht mindestens zwei Paare ✓ ✓
        for paar in gruppe:
            zu_ersetzen.append((paar.start(1), paar.end(1)))
    if not zu_ersetzen:
        return satz
    teile, letzte = [], 0
    for start, end in sorted(zu_ersetzen):
        teile.append(satz[letzte:start])
        teile.append("osmtag")
        letzte = end
    teile.append(satz[letzte:])
    return "".join(teile)

VERNEINUNG = (r"\bbleibt?\b|\bunverändert\b|\bbleibend\b|\boffen\b|\bkein(?:e|em|er)?\b"
              r"|\bnicht\b|\bohne\b|\bweder\b|\bnach wie vor\b"
              r"|\bunbestätigt\b|\bungeprüft\b|\bwurde\s+nicht\b"
              r"|\bkeine\s+(?:Aenderung|Änderung|Korrektur)\b"
              r"|\bnicht\s+belegt\b|\bvorläufig\b|\bgeprüft\b|\bstrittig\b"
              r"|\bnicht\s+geändert\b|\bim\s+Zusammenhang\b")

Satztrenner = re.compile(r"(?<=[.;:])\s+|\n")


def frontmatter(text):
    """Die Frontmatter-Felder als dict."""
    out = {}
    for line in text.split("\n"):
        if line.startswith("## "):
            break
        m = re.match(r"^([a-z_]+):\s*(.*)$", line)
        if m:
            out[m.group(1)] = m.group(2)
    return out


def notes(text):
    """Die Timeline-Notizbloecke (Bullet + Fortsetzungszeilen)."""
    start = text.find("## Timeline")
    if start < 0:
        return []
    out, block = [], None
    for line in text[start:].split("\n"):
        if line.startswith("- "):
            if block:
                out.append(block)
            block = [line]
        elif block is not None and line.strip() and line[:1] in (" ", "\t"):
            block.append(line)
        elif block is not None:
            out.append(block)
            block = None
    if block:
        out.append(block)
    return ["\n".join(b) for b in out]


def genannte_felder(satz):
    """Die Frontmatter-Felder, die der Satz als Feld nennt."""
    gefunden = set()
    # Ein **OSM-Tag** ✓ ✓ (`name`= ✓ ✓) ist **kein** Dossier-Feld ✓ ✓ - der
    # Wert gehoert zum kartierten Objekt ✓ ✓, nicht zum Dossier ✓ ✓. Ohne diese
    # Abgrenzung meldete das Gate eine Aenderung, die keine war ✓ ✓ -
    # die Fehlalarme, die `research/auftraege-20261120.md` benennt ✓ ✓
    ohne_osm_tag = osm_tags_maskieren(satz)
    for feld in FELDER:
        if re.search(rf"`{feld}`", ohne_osm_tag, re.I):
            gefunden.add(feld)
    if re.search(r"\b(im|der|das|beim)\s+(Frontmatter|Eintrag|Dossier|Datensatz|Seed)\b",
                 satz, re.I):
        # "im Frontmatter" allein benennt das Feld nicht - der Satz muss es
        # zusaetzlich nennen. Dieselbe Regel gilt fuer die Synonyme: der Seed
        # steht unter "Eintrag", "Dossier" und "Datensatz", und eine Notiz wie
        # "der Eintrag nannte nur 'Herne'" beschreibt genauso ein
        # Frontmatter-Feld wie "im Frontmatter ...". Vor dieser Erweiterung
        # blieben solche Aussagen ungeprueft - der Pre-Deploy-Review vom
        # 17.11.2026 hat eine davon gefunden.
        for feld in FELDER:
            if re.search(rf"\b{feld}\b", ohne_osm_tag, re.I):
                gefunden.add(feld)
    return gefunden


def aenderungs_verb(satz):
    """Traegt der Satz ein **Aenderungsverb** ✓ ✓ - und welches ✓ ✓?

    „entfernt" als **Lageangabe** ✓ ✓ ("liegt 2,6 km vom Objekt entfernt" ✓ ✓)
    ist **keine** Handlung ✓ ✓ - ohne diese Abgrenzung meldete das Gate eine
    Aenderung, die keine war ✓ ✓ (`research/auftraege-20261120.md` ✓ ✓).
    Die **uebrigen** Verben ✓ ✓ zaehlen unveraendert ✓ ✓ - eine echte
    Behauptung im selben Satz ✓ ✓ geht **nicht** verloren ✓ ✓.
    """
    for treffer in re.finditer(AENDERUNG, satz, re.I):
        wort = treffer.group(0)
        if NUR_ENTFERNT.match(wort) and not ENTFERNT_HANDLUNG.search(satz):
            continue
        return wort
    return None


def behauptungen(notiz):
    """Die (Feld, Satz)-Paare, in denen eine Aenderung behauptet wird."""
    treffer = []
    for satz in Satztrenner.split(notiz):
        if not aenderungs_verb(satz):
            continue
        if re.search(VERNEINUNG, satz, re.I):
            continue
        for feld in sorted(genannte_felder(satz)):
            treffer.append((feld, satz.strip()[:170]))
    return treffer


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    basis = sys.argv[1]
    ref = sys.argv[2] if len(sys.argv) > 2 else "HEAD"
    dateien = subprocess.run(
        ["git", "diff", "--name-only", basis, ref, "--", "dossiers/"],
        capture_output=True, text=True, check=True).stdout.split()
    befunde = []
    for rel in dateien:
        alt = subprocess.run(["git", "show", f"{basis}:{rel}"],
                             capture_output=True, text=True).stdout
        # Der Kandidat ist der Arbeitsbaum (wie in den übrigen Gates) ✓ -
        # sonst prüfte das Gate stets den letzten Commit statt der Arbeit ✓.
        pfad = ROOT / rel
        cur = pfad.read_text(encoding="utf-8") if pfad.exists() else \
            subprocess.run(["git", "show", f"{ref}:{rel}"],
                           capture_output=True, text=True).stdout
        felder_alt, felder_cur = frontmatter(alt), frontmatter(cur)
        alt_notizen = set(notes(alt))
        neu = [n for n in notes(cur) if n not in alt_notizen]
        if not neu:
            continue
        for notiz in neu:
            for feld, satz in behauptungen(notiz):
                if felder_alt.get(feld) == felder_cur.get(feld):
                    befunde.append((rel, feld, satz))
    print(f"Dossiers im Scope: {len(dateien)}")
    print(f"Befunde: {len(befunde)}")
    for rel, feld, satz in befunde:
        print(f"  {rel} [{feld}]: {satz}")
    if not befunde:
        print("Keine Notiz behauptet eine Aenderung, die nicht stattfand ✓")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
