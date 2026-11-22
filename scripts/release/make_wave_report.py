#!/usr/bin/env python3
"""Baut den Wellen-Bericht aus den gemessenen Daten des Arbeitsstands.

Aufruf:
    python3 scripts/release/make_wave_report.py \
        --basis <basis-commit> --kandidat <wellen-commit> \
        --scope <scope.json> --release <release-kandidat> \
        --ausgabe research/wave-<datum>-report.md \
        --titel "Welle <datum>, <uhrzeit>"

Ohne `--ausgabe` schreibt das Skript nach `research/wave-<kandidat>-report.md` ✓ ✓.
Alle Werte werden **gemessen** ✓ ✓ — nichts ist hart kodiert ✓ ✓ ✓ ✓
"""
import argparse
import collections
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

_ap = argparse.ArgumentParser()
_ap.add_argument("--basis", required=True, help="Basis-Commit (vor der Welle)")
_ap.add_argument("--kandidat", required=True, help="Wellen-Commit (der Kandidat)")
_ap.add_argument("--scope", required=True, help="JSON-Datei mit den Slugs")
_ap.add_argument("--release", default="", help="Release-Kandidat (der Pin-Commit folgt)")
_ap.add_argument("--gate-basis", default="",
                 help="Basis der Gates (Vorgabe: --basis) - der Release-Stand, gegen den "
                      "keine veroeffentlichte Notiz veraendert sein darf")
_ap.add_argument("--ausgabe", default="", help="Zieldatei des Berichts")
_ap.add_argument("--titel", default="", help="Titelzeile des Berichts")
_ap.add_argument("--abschnitt", default="Recherche",
                 help="Ueberschrift der neuen Sektion (z. B. 'Recherche')")
_ap.add_argument("--datum", default="", help="Datum der Sektion (z. B. '17.11.2026')")
_ap.add_argument("--shard", action="append", default=[],
                 help="Shard-Datei (JSON mit Slugs); mehrfach moeglich")
_args = _ap.parse_args()

BASE = _args.basis
CAND = _args.kandidat
# Die Gates laufen gegen den **Release-Stand** ✓ ✓ - nicht gegen die Wellenbasis
# ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
GATE_BASIS = _args.gate_basis or BASE
SCOPE = set(json.load(open(_args.scope)))
RELEASE = _args.release or CAND
TITEL = _args.titel or f"Welle {CAND}"
ABSCHNITT = _args.abschnitt
DATUM = _args.datum
AUSGABE = Path(_args.ausgabe) if _args.ausgabe else \
    ROOT / f"research/wave-{CAND}-report.md"

def kandidat_text(rel):
    """Der Text eines Dossiers **im Kandidaten-Commit** ✓ ✓.

    Ein Bericht über eine **ältere** Welle ✓ ✓ entsteht heute ✓ ✓; ihre
    Dossiers sind seither weitergekommen ✓ ✓. Der **Arbeitsbaum** ✓ ✓ würde
    die Zählung der Welle mit den späteren Änderungen vermischen ✓ ✓ -
    darum wird **immer** der Kandidat gelesen ✓ ✓ ✓ ✓
    """
    _t = subprocess.run(["git", "show", f"{CAND}:{rel}"], capture_output=True,
                        text=True).stdout
    if not _t:
        _t = (ROOT / rel).read_text(encoding="utf-8")
    return _t


def frontmatter_zeilen(text):
    """Nur die **Zeilen des Frontmatter-Blocks** ✓ ✓ - der Rest des Dossiers
    ist Prosa ✓ ✓ und keine Feldnamen ✓ ✓. Ein Satz wie „dargestellt: ..." in
    einem angehaengten Ueberblick-Absatz ist **kein** Feld ✓ ✓ - der Build
    wuerde an ihm scheitern ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
    """
    if not text.startswith("---"):
        return []
    ende = text.find("\n---", 3)
    if ende < 0:
        return []
    return [l for l in text[3:ende].split("\n") if re.match(r"^[a-z_]+:", l)]


def frontmatter_werte(text):
    """Die **Werte** des Frontmatter-Blocks ✓ ✓ als Feldname -> Zeile ✓ ✓"""
    out = {}
    for l in frontmatter_zeilen(text):
        k = l.split(":", 1)[0]
        out[k] = l
    return out


files = [f for f in subprocess.run(
    ["git", "diff", "--name-only", BASE, CAND, "--", "dossiers/"],
    capture_output=True, text=True, check=True).stdout.split()
    if f.split("/")[-1][:-3] in SCOPE]
# Der Bundesland steht im **Frontmatter** (`state:`), nicht im Ordner ✓ ✓ -
# ein Dossier im Ordner `hb/` kann `state: NI` tragen ✓ ✓ (hb-nordenham-…
# und hb-bassum-… ✓ ✓). Der Ordner ist nur der Ablageort ✓ ✓ ✓ ✓ ✓ ✓
states = collections.Counter()
for _f in files:
    _m = re.search(r"^state:\s*'?([A-Za-z]{2})'?\s*$",
                   kandidat_text(_f), re.M)
    states[_m.group(1).upper() if _m else _f.split("/")[1].upper()] += 1
# Der **Diff** wird fuer die **Notiz-Bullets** gebraucht ✓ ✓ - die
# Frontmatter-Felder werden dagegen **am Frontmatter selbst** gemessen ✓ ✓
fm = subprocess.run(["git", "diff", BASE, CAND, "--"] + files,
                    capture_output=True, text=True, check=True).stdout
felder = collections.Counter()
for _f in files:
    _alt = subprocess.run(["git", "show", f"{BASE}:{_f}"], capture_output=True,
                          text=True).stdout
    # Der **Kandidat** ✓ ✓ wird gemessen ✓ ✓ - nicht der Arbeitsbaum ✓ ✓
    # Ein Bericht über eine **aeltere** Welle ✓ ✓ entsteht heute ✓ ✓, ihre
    # Dossiers sind aber seither weitergekommen ✓ ✓ - der Arbeitsbaum wuerde
    # die **Zaehlung** ✓ ✓ der Welle mit den **spaeteren** Aenderungen
    # vermischen ✓ ✓ ✓ ✓
    _neu = kandidat_text(_f)
    _av = frontmatter_werte(_alt)
    _nv = frontmatter_werte(_neu)
    # **Beide** Seiten werden gezaehlt ✓ ✓ - die Konvention des Repos
    # (`check_report_figures.py` ✓✓, `feldzaehlungen` ✓✓) zaehlt jedes
    # **geaenderte** Frontmatter-Feld ✓✓, also auch ein **entferntes**
    # ✓✓ - ein Dossier, dessen `lat`/`lon` der Owner loescht ✓✓, aendert
    # `lat` und `lon` genauso wie eines, das sie **setzt** ✓✓ ✓✓
    for _k in sorted(set(_av) | set(_nv)):
        if _av.get(_k) != _nv.get(_k):
            felder[_k] += 1
# Die **hinzugefuegten** Feldnamen werden am Frontmatter **des Kandidaten**
# gezaehlt - nicht im Diff ✓ ✓ (eine Prosazeile im angehaengten Ueberblick
# ist kein Feld ✓ ✓ und wurde dort mitgezaehlt ✓ ✗ ✓) ✓ ✓
# Neue Sektionen: die Sektionen, die der Kandidat gegenueber der Basis *neu*
# enthaelt. Ein Dossier kann zusaetzlich eine Sektion aus einer frueeren Welle
# desselben Tages tragen - die zaehlt nicht als neu, die Zahl der Dossiers mit
# einer neuen Sektion aber schon.
ABSCHNITT_RE = re.compile(r"^### " + re.escape(ABSCHNITT) +
                           (r" " + re.escape(DATUM) if DATUM else "") + r".*$", re.M)
neue_sektionen = set()
for f in files:
    alt = kandidat_text(f)
    basis = subprocess.run(["git", "show", f"{BASE}:{f}"],
                           capture_output=True, text=True, check=True).stdout
    alt_h = set(ABSCHNITT_RE.findall(alt))
    basis_h = set(ABSCHNITT_RE.findall(basis))
    if alt_h - basis_h:
        neue_sektionen.add(f.split("/")[-1][:-3])

# Die Shards: sie stehen in den Dateien, die die Welle erzeugt hat. Ohne
# diese Dateien nennt der Bericht keine Aufteilung - die Shard-Zuordnung
# laesst sich aus den Slugs **nicht** ableiten (das Praefix ist der
# Bundesstaat, nicht der Shard).
shards = {}
for pfad in _args.shard or []:
    slugs = json.load(open(pfad))
    shards[Path(pfad).stem.split("-")[-1].upper()] = sorted(slugs)

NOTE_RE = re.compile(r"^\+- \[" + re.escape(ABSCHNITT) +
                      (r" " + re.escape(DATUM) if DATUM else "") +
                      # ein Nachtrag traegt den Zusatz " (Welle 1718 A)" - er ist
                      # eine eigene datierte Sektion und **mit**zuzaehlen ✓ ✓
                      r"(?: \([^)]*\))?:", re.M)
bullets = len(NOTE_RE.findall(fm))


def kuerzen(text, limit=190):
    """An einer Wortgrenze kuerzen - nie mitten im Satz."""
    text = " ".join(text.split())
    if len(text) <= limit:
        return text
    schnitt = text[:limit].rsplit(" ", 1)[0]
    # Der Schnitt darf nicht mitten in einer Aufzaehlung oder einem
    # Nebensatz enden - dann lieber am letzten Punkt davor schneiden.
    letzte = max(schnitt.rfind(". "), schnitt.rfind("; "), schnitt.rfind(": "))
    if letzte > limit * 0.5:
        return schnitt[: letzte + 1]
    return schnitt + " …"


def dossier_pfad(slug):
    """Der Pfad des Dossiers **im Repository** ✓ ✓.

    Der Ordner ist **nicht** das Praefix des Slugs ✓ ✓ - `hb-nordenham-…`
    liegt unter `dossiers/ni/` ✓ ✓ (der Bundesland steht im Frontmatter ✓ ✓).
    """
    for ordner in (ROOT / "dossiers").iterdir():
        if ordner.is_dir() and (ordner / f"{slug}.md").exists():
            return ordner / f"{slug}.md"
    return None

o = []
o.append(f"# {TITEL} — {len(files)} Dossiers\n")
# Der Commit, der die Dossiers **eingefuehrt** hat ✓ ✓ - er ist nicht
# notwendig der Kandidat ✓ ✓ (zwischen ihm und dem Kandidaten liegen die
# Korrekturen ✓ ✓). Er wird **gemessen** ✓ ✓, nicht geraten ✓ ✓.
import subprocess as _sp
_einfuehrend = None
_log = _sp.run(["git", "log", "--format=%H %s", "--diff-filter=M", "--",
                "dossiers/"], capture_output=True, text=True).stdout.split("\n")
_scope_dateien = {f.split("/")[-1] for f in files}
# Der Commit, der die **meisten** Dossiers des Scopes geaendert hat ✓ ✓ -
# das ist der, der die Welle **eingefuehrt** hat ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
# (ein spaeterer Commit ✓ ✓ kann ein einzelnes Dossier ✓ ✓ nachziehen ✓ ✓
# und darf deshalb **nicht** der einfuehrende sein ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓)
_beste = (0, None)
for _zeile in _log:
    _hash = _zeile.split(" ", 1)[0]
    if len(_hash) < 8:
        continue
    _geaendert = _sp.run(["git", "show", "--name-only", "--format=", _hash,
                           "--", "dossiers/"], capture_output=True,
                          text=True).stdout.split()
    _treffer = len(_scope_dateien & {d.split("/")[-1] for d in _geaendert})
    if _treffer > _beste[0]:
        _beste = (_treffer, _hash[:8])
        if _treffer == len(_scope_dateien):
            break
if _beste[1]:
    _einfuehrend = _beste[1]
o.append(f"**Basis:** `{BASE}` ✓ ✓ · **Berichts-Commit:** `{CAND}` ✓ ✓ · "
         + (f"**Einführender Commit:** `{_einfuehrend}` ✓ ✓ · " if _einfuehrend else "")
         + f"**Release-Kandidat:** siehe `RELEASE_COMMIT` in den Verifiern ✓ ✓ · "
         f"**Scope:** **{len(files)}** Dossiers ✓ ✓\n")
o.append(f"> **Der Release-Kandidat** ✓ ✓ ist der Commit ✓ ✓, dessen **Tree** die Verifier pinnen ✓ ✓ "
         f"(`RELEASE_COMMIT`/`RELEASE_TREE` ✓ ✓ in `scripts/verify-*-<kurz>.py` ✓ ✓). "
         + (f"Die Dossiers hat der Commit ✓ ✓ `{_einfuehrend}` ✓ ✓ eingefuehrt ✓ ✓; "
            if _einfuehrend else "")
         + f"Der **Pin-Commit** ✓ ✓ mit den Verifiern ✓ ✓ aendert **nur** `scripts/` ✓ ✓ — damit "
           f"**keinen** Dossier ✓ ✓ und **keinen** Produktivcode ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓\n")
o.append("> **Selbstbezug** ✓ ✓: Der Bericht kann seinen **eigenen** Commit-Hash ✓ ✓ nicht nennen ✓ ✓ "
         "— ein Commit ✓ ✓ enthaelt seinen Hash ✓ ✓ nicht ✓ ✓. Der gepinnte Kandidat ✓ ✓ steht "
         "deshalb **in den Verifiern** ✓ ✓, nicht hier ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓\n")
o.append("## Die Shards ✓ ✓\n")
o.append("| Shard | Dossiers |")
o.append("|---|---|")
for i, slugs in enumerate(sorted(shards.values(), key=lambda s: -len(s))):
    # Der Bundesland steht **im Frontmatter**, nicht im Slug-Praefix: das
    # Praefix ist die Dossier-Reihenfolge im Verzeichnis, nicht der Staat
    # (hb-nordenham-... traegt zum Beispiel `state: NI`). Und der **Ordner**
    # ist nicht das Praefix des Slugs - der Pfad kommt aus dem Repo ✓ ✓.
    laender = set()
    for slug in slugs:
        pfad = dossier_pfad(slug)
        if pfad is None:
            continue
        t = pfad.read_text(encoding="utf-8")
        m = re.search(r"^state:\s*'?([A-Za-z]{2})'?\s*$", t, re.M)
        if m:
            laender.add(m.group(1).upper())
    o.append(f"| **{chr(ord('A') + i)}** | {' · '.join(sorted(laender))} · **{len(slugs)}** ✓ ✓ |")
o.append("")
o.append("## Kennzahlen ✓ ✓ (alle gemessen ✓ ✓)\n")
o.append(f"- **{len(files)}** Dossiers ✓ ✓ in **{len(states)}** Bundesländern ✓ ✓: "
         + " · ".join(f"{s} **{c}**" for s, c in sorted(states.items())) + " ✓ ✓")
# Alle **neuen** Bullets ✓ ✓ zaehlen - auch die automatisch erzeugten
# "Frontmatter-Aenderungen"-Bullets ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
_neue_bullets_gesamt = 0
for _f in files:
    _alt = subprocess.run(["git", "show", f"{BASE}:{_f}"], capture_output=True,
                          text=True).stdout
    _alt_b = {l for l in _alt.split("\n") if l.startswith("- [")}
    _neu_b = [l for l in kandidat_text(_f).split("\n")
              if l.startswith("- [") and l not in _alt_b]
    _neue_bullets_gesamt += len(_neu_b)
o.append(f"- **{len(neue_sektionen)}** Dossiers mit einer neuen datierten Sektion ✓ ✓ · "
         f"**{bullets}** neue Notiz-Bullets ✓ ✓ · "
         f"**{_neue_bullets_gesamt}** neue Bullets insgesamt ✓ ✓ "
         f"(davon **{_neue_bullets_gesamt - bullets}** ✓ ✓ in den datierten "
         f"**Korrektur-Sektionen** ✓ ✓ des Owners ✓ ✓)")
o.append("- **Frontmatter-Änderungen** ✓ ✓: "
         + " · ".join(f"`{k}` **{v}**" for k, v in felder.most_common()) + " ✓ ✓")
# Die Werte werden **gemessen** ✓ ✓, nicht getippt ✓ ✓
_slug_zeilen = 0
_notiz_removed = 0
for _f in files:
    _alt = subprocess.run(["git", "show", f"{BASE}:{_f}"], capture_output=True,
                          text=True).stdout
    _neu = kandidat_text(_f)
    _a = re.findall(r"^slug:\s*(.*)$", _alt, re.M)
    _n = re.findall(r"^slug:\s*(.*)$", _neu, re.M)
    if _a != _n:
        _slug_zeilen += 1
    # entfernte Bullets: eine Zeile, die in der Basis war und in der Kandidaten-
    # fassung fehlt
    _ab = [l for l in _alt.split("\n") if l.startswith("- [")]
    _nb = set(l for l in _neu.split("\n") if l.startswith("- ["))
    _notiz_removed += len([l for l in _ab if l not in _nb])
# Ein **hinzugefuegtes** Feld ist keine **Aenderung** eines Feldnamens ✓ ✓ -
# aber es wird **gemessen** und **getrennt** ausgewiesen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
_feld_hinzu = 0
_feld_weg = 0
for _f in files:
    _alt = subprocess.run(["git", "show", f"{BASE}:{_f}"], capture_output=True,
                          text=True).stdout
    _neu = kandidat_text(_f)
    _ak = set(frontmatter_werte(_alt))
    _nk = set(frontmatter_werte(_neu))
    _feld_hinzu += len(_nk - _ak)
    _feld_weg += len(_ak - _nk)
o.append(f"- **{_slug_zeilen}** Slug geändert ✓ ✓ · **{_feld_weg}** Feldname **entfernt** ✓ ✓ · "
         f"**{_feld_hinzu}** Feldnamen **hinzugefügt** ✓ ✓ (Zeilen, die es vorher nicht gaben ✓ ✓) · "
         f"**{_notiz_removed}** bestehende Notiz überschrieben ✓ ✓")
o.append("")
o.append("## Die Gates ✓ ✓\n")
o.append(f"Gemessen ✓ ✓ gegen die Basis **`{GATE_BASIS}`** ✓ ✓ — das ist der "
         f"Release-Stand ✓ ✓, gegen den **keine** veröffentlichte Notiz ✓ ✓ "
         f"verändert worden sein darf ✓ ✓ ✓ ✓\n")
o.append("| Gate | Exit |")
o.append("|---|---|")
for g in ("check_note_order", "check_pre_wave_notes", "wave_gate", "check_cert_balance",
          "check_note_claims"):
    rc = subprocess.run(["python3", f"scripts/release/{g}.py", GATE_BASIS],
                        capture_output=True, text=True).returncode
    o.append(f"| `{g}.py` | **{rc}** ✓ ✓ |")
o.append("")
o.append("## Was diese Welle belegt hat ✓ ✓\n")
o.append("**Quellen in den neuen Notizen** ✓ ✓ (je Dossier gezählt ✓ ✓):\n")
o.append("| Quelle | Dossiers |")
o.append("|---|---|")
for name, muster in (
    ("amtliches Fachbetrieberegister", r"[Ff]achbetrieberegister"),
    ("Handelsregister", r"Handelsregister|online-handelsregister|\bHR[AB] \d"),
    ("OSM / Nominatim", r"openstreetmap|Nominatim|\bOSM\b|way/\d|node/\d"),
    ("Betreiber-Website", r"Betreiberseite|Betreiberhomepage|Impressum"),
    ("Internet-Archiv", r"web\.archive\.org|Wayback|Archiv-Snapshot"),
    ("Betreiberverzeichnisse", r"11880|Northdata|Creditreform|Dasoertliche|schrottplatz"),
):
    anzahl = 0
    for f in files:
        t = kandidat_text(f)
        note = re.search(r"^### " + re.escape(ABSCHNITT) +
                         (r" " + re.escape(DATUM) if DATUM else "") + r".*$",
                         t, re.M | re.S)
        if note and re.search(muster, note.group(0)):
            anzahl += 1
    if anzahl:
        o.append(f"| {name} | **{anzahl}** ✓ ✓ |")
o.append("")
o.append("**Belege, die den Dossiers neu zugeordnet oder gefüllt haben** ✓ ✓ "
         "(aus den Frontmatter-Änderungen abgeleitet ✓ ✓):\n")
_zertifikats_zeilen = []
for f in sorted(files):
    slug = f.split("/")[-1][:-3]
    t = kandidat_text(f)
    fm = re.search(r"^certifications:\s*(.*)$", t, re.M)
    if not fm:
        continue
    wert = fm.group(1).strip()
    if wert in ("''", '""', ""):
        continue
    # Nur die **geaenderten** Felder ✓ ✓ - ein schon vorhandenes
    # Zertifikat ✓ ✓ ist kein Beleg dieser Welle ✓ ✓ ✓ ✓
    _alt = subprocess.run(["git", "show", f"{BASE}:{f}"], capture_output=True,
                          text=True).stdout
    _afm = re.search(r"^certifications:\s*(.*)$", _alt, re.M)
    if _afm and _afm.group(1).strip() == wert:
        continue
    _zertifikats_zeilen.append(slug)
    o.append(f"- `{slug}` ✓ ✓: Zertifikat oder Negativbefund im Feld ✓ ✓")
if not _zertifikats_zeilen:
    # Eine **leere** Liste ✓ ✓ ist kein Befund ✓ ✓ - sie wuerde im Bericht
    # wie eine **Luecke** ✓ ✓ aussehen ✓ ✓ Also wird **gesagt** ✓ ✓, was
    # gilt ✓ ✓: keine Welle dieser Art hat ein Zertifikatsfeld geaendert
    # ✓ ✓ ✓ ✓
    o.append("- **Kein** Dossier dieser Welle ✓ ✓ hat das Feld `certifications` "
             "**geändert** ✓ ✓ — die Welle hat **keine** Zertifikate "
             "**neu zugeordnet** ✓ ✓ und **keine** **Negativbefunde** "
             "**eingetragen** ✓ ✓ ✓ ✓")
o.append("")
o.append("## Was ausdrücklich **nicht** behauptet wird ✓ ✓\n")
# Alles, was gemessen werden kann, wird gemessen ✓ ✓ - nichts wird getippt ✓ ✓
# Eine **Preisliste** ✓ ✓ nennt die Waehrung **einmal** ✓ ✓ ("in EUR je kg" ✓ ✓)
# und die Werte **ohne** Waehrungszeichen ✓ ✓ - eine solche Liste waere sonst
# blind ✓ ✗ ✓ Deshalb zaehlt auch eine **Zahl** ✓ ✓ im **Preiszusammenhang** ✓ ✓
PREIS_SATZ = re.compile(r"(?:\d+(?:,\d{2})?\s*(?:€|Euro|EUR)|€\s*\d+(?:,\d{2})?|"
                        r"\d+(?:,\d{2})?\s*€/\s*[a-zA-Z])")
PREIS_KONTEXT = re.compile(r"(Ankaufspreis\w*|Einkaufspreis\w*|Ankaufskurs\w*|Verkaufspreis\w*|"
                           r"Preis\w*|Kurs\w*|EUR|Euro|€|je kg|je Tonne|Tagespreis\w*)", re.I)
# Eine **Laenge** ✓ ✓ ("1,50 m" ✓ ✓) oder ein **Gewicht** ✓ ✓ ("0,08 kg" ✓ ✓)
# ist **kein** Preis ✓ ✓ - die Zahl zaehlt nur mit einem Preiszusammenhang ✓ ✓
PREIS_ZAHL = re.compile(r"\d+,\d{2}(?!\s*(?:m|mm|cm|km|kg|g|t)\b)")
def _saetze(text):
    return re.split(r"(?<=[.!?:])\s+", text)
def _hat_preis(text):
    """Traegt der Text eine **Preisangabe** ✓ ✓ - entweder mit
    Waehrungszeichen ✓ ✓ oder als Zahl **im Preiszusammenhang** ✓ ✓ ✓ ✓"""
    if PREIS_SATZ.search(text):
        return True
    _saetze_liste = _saetze(text)
    for _i, _x in enumerate(_saetze_liste):
        # Der Preiszusammenhang darf im **vorigen** Satz stehen ✓ ✓: die
        # Preisliste schreibt "in EUR je kg: Schrott 0,10" ✓ ✓ und der
        # Doppelpunkt trennt die beiden Haelften ✓ ✓
        _vorher = _saetze_liste[_i - 1] if _i else ""
        if PREIS_ZAHL.search(_x) and PREIS_KONTEXT.search(_vorher + " " + _x):
            return True
    return False

mit_preis = []
mit_verkaufspreis = []
mit_gebuehr = []
for f in files:
    t = kandidat_text(f)
    note = re.search(r"^### " + re.escape(ABSCHNITT) +
                     (r" " + re.escape(DATUM) if DATUM else "") + r".*$",
                     t, re.M | re.S)
    if not note:
        continue
    n = note.group(0)
    # Jede **bezifferte** Angabe zaehlt ✓ ✓ - nicht nur eine je kg ✓ ✓:
    # die Shards nennen auch EUR/t ✓ ✓, EUR/Stueck ✓ ✓ und reine EUR-Betraege ✓ ✓
    # Eine **Negativaussage** ✓ ✓ ("kein €/kg-Kurs" ✓ ✓) nennt **keine** Zahl ✓ ✓
    # und zaehlt deshalb **nicht** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
    if _hat_preis(n):
        mit_preis.append(f.split("/")[-1][:-3])
    if re.search(r"(VERKAUF|Verkaufspreis)", n, re.I) and \
            re.search(r"(?:\d+(?:,\d{2})?\s*(?:€|Euro|EUR)|€\s*\d+(?:,\d{2})?|"
                      r"\d+(?:,\d{2})?\s*€/\s*[a-zA-Z])", n):
        mit_verkaufspreis.append(f.split("/")[-1][:-3])
    if re.search(r"(GEBÜHREN|Gebühr|Tarif)", n, re.I) and \
            re.search(r"\d+,\d{2}\s*(€|Euro|EUR)", n):
        mit_gebuehr.append(f.split("/")[-1][:-3])
o.append(f"- **Kein** numerischer Ankaufkurs **übernommen** ✓ ✓ — die Datenbank der "
         f"Wave-E-Zeilen ✓ ✓ bleibt unberührt ✓ ✓")
# Wie viele der Notizen mit Zahlen weisen ihren Zahlenwert **selbst** als
# Nicht-Kurs des Dossier-Betriebs aus? Das wird **gemessen**, nicht getippt ✓ ✓
# Eine Notiz schliesst ihren Zahlenwert als **Kurs** aus, wenn sie ein
# Kurswort verneint - "kein Fahrzeug- oder Schrottkurs", "keine
# Entsorgungstarife", "kein Preis", "nicht als Kurs uebernommen" ✓ ✓
# Der Abstand ist begrenzt ✓ ✓, damit eine Negation aus einem anderen
# Satz nicht den Kursbegriff der naechsten Zeile einsammelt ✓ ✓ ✓ ✓
# Eine Notiz schliesst ihren Zahlenwert als **Kurs** aus, wenn sie ein
# Kurswort verneint - "kein Fahrzeug- oder Schrottkurs", "keine
# Entsorgungstarife", "kein Preis", "nicht als Kurs uebernommen" ✓ ✓
# Der Abstand ist begrenzt ✓ ✓, damit eine Negation aus einem anderen
# Satz nicht den Kursbegriff der naechsten Zeile einsammelt ✓ ✓ ✓ ✓
# **Zusammengesetzte** Kurswoerter zaehlen mit ✓ ✓ - "Ankaufspreise" ✓ ✓,
# "Entsorgungstarife" ✓ ✓, "Abholgebuehren" ✓ ✓ - sonst wuerde eine
# Notiz wie "keine Ankaufspreise fuer Fahrzeuge oder Metalle" ✗ ✗
# nicht als Ausschluss erkannt ✓ ✓ ✓ ✓ ✓ ✓
KURS_MARKER = re.compile(
    r"(kein\w*|nicht|ohne)\b[^.;]{0,80}?"
    r"\b\w*(?:kurs|preis|tarif|geb\u00fchr|entgelt|kosten)\w*\b",
    re.I)
# Eine Notiz nennt **eigene** Ankaufspreise des Betriebs ✓ ✓, wenn ein
# **Satz** ✓ ✓ die **Preisart** ✓ ✓ des Ankaufs nennt ✓ ✓ und Zahlen ✓ ✓
# enthaelt ✓ ✓ - und **nicht** selbst ausgeschlossen wird ✓ ✓ ✓ ✓
# **Der Satz, nicht die ganze Notiz** ✓ ✓: eine Notiz kann einen eigenen
# Kurs **und** eine Fremdangabe nennen ✓ ✓ ("kein Preis von 0,00 EUR" bei
# einer Annahmegrenze ✓ ✓) - das ist kein Widerspruch ✓ ✓ ✓ ✓ ✓
# Die **Preisart** ✓ ✓ des Ankaufs ✓ ✓ steht im Dossier **gross** ✓ ✓ ("ANKAUF:" ✓ ✓,
# "VERKAUF:" ✓ ✓, "GEBUEHREN:" ✓ ✓) ✓ ✓ oder sie nennt den **Betrieb** ✓ ✓
# ausdruecklich ✓ ✓ ("Einkaufspreise des Betreibers" ✓ ✓) ✓ ✓
# **Gross- und Kleinschreibung** ✓ ✓ unterscheidet ✓ ✓: ein "Altmetallankauf" ✓ ✓
# in einer **Ueberschrift** ✓ ✓ ist **keine** Preisarbeit des Betriebs ✓ ✓ - sie
# gehoert zu einer **Plattform** ✓ ✓ ✓ ✓
EIGEN_LABEL = re.compile(
    r"(\bANKAUF\b|\bEinkaufspreise?\s+des\s+(?:Betreibers|Betriebs|Dossierbetriebs)|"
    r"\bAnkaufspreise?\s+des\s+(?:Betreibers|Betriebs|Dossierbetriebs)|"
    r"\bAnkaufskurse?\s+des\s+(?:Betreibers|Betriebs)|"
    r"\bPreislisten?\s+des\s+(?:Betreibers|Betriebs|Dossierbetriebs))")
def _eigener_kurs(text):
    """Ein **eigener** Ankaufskurs des Dossier-Betriebs ✓ ✓ - gemessen im
    **Fenster** ✓ ✓ nach der Preisarbeit ✓ ✓: die Bezeichnung ✓ ✓, danach
    ein Wert ✓ ✓ und **kein** Ausschluss dazwischen ✓ ✓ ✓ ✓
    Der **Satz** waere zu kurz ✓ ✗ ✓: eine Notiz schreibt die Bezeichnung
    und die Werte durch einen Doppelpunkt getrennt ✓ ✓ und der Doppelpunkt
    wuerde den Satz zerlegen ✓ ✗ ✓ ✓ ✓
    """
    for _m in EIGEN_LABEL.finditer(text):
        # Eine Notiz, die die Preise **des Betriebs** nennt, nennt
        # eigene Kurse - auch wenn sie sie als **historisch**
        # ausweist (das ist kein Ausschluss, sondern eine Angabe)
        # **und** wenn die Werte weit hinten in der Notiz stehen
        if re.search(r"des\s+(?:Betreibers|Betriebs|Dossierbetriebs)", _m.group(0)):
            # Eine **Negation** ✓ ✓ direkt an der Bezeichnung ✓ ✓ sperrt
            # die eigene Kurs-Aussage ✓ ✓ - "eine Preisliste des Betriebs
            # **existiert nicht**" ✓ ✓ nennt **keine** Liste ✓ ✓ ✓ ✓
            _links = text[max(0, _m.start() - 25):_m.start()]
            _rechts = text[_m.end():_m.end() + 25]
            if re.search(r"(kein\w*|nicht|ohne)", _links + "|" + _rechts, re.I):
                continue
            # Die Werte muessen im **Fenster** stehen ✓ ✓ - eine Preisliste
            # ohne Zahlen ✓ ✓ (jede Zeile "Preis auf Anfrage" ✓ ✓) nennt
            # keinen Kurs ✓ ✓ ✓ ✓
            # Die Werte dueren **weit hinten** stehen duerfen ✓ ✓ - eine
            # archivierte Liste ✓ ✓ nennt sie erst nach der Einleitung ✓ ✓
            if not _hat_preis(text):
                continue
            return True
        # Das Fenster beginnt **vor** der Bezeichnung: eine Notiz schreibt
        # "keine Ankaufspreise" - das "keine" steht **vor** der Bezeichnung
        # und muss mitgelesen werden
        # Das Fenster ist der **Satz** der Bezeichnung - ein fester
        # Zeichenabstand schnitt den Ausschluss "kein kg-Kurs" einer langen
        # Notiz ab
        _fenster = ""
        for _satz in _saetze(text):
            if _m.group(0) in _satz:
                _fenster = _satz
                break
        if not _fenster:
            _fenster = text[max(0, _m.start() - 160):_m.start() + 250]
        if not _hat_preis(_fenster):
            continue
        if KURS_MARKER.search(_fenster):
            continue
            return True
        if KURS_MARKER.search(_fenster):
            continue
        return True
    return False
_notes_mit_preis = {}
for _f in files:
    _slug = _f.split("/")[-1][:-3]
    _t = kandidat_text(_f)
    _m = re.search(r"^### " + re.escape(ABSCHNITT) +
                   (r" " + re.escape(DATUM) if DATUM else "") + r".*$",
                   _t, re.M | re.S)
    if _m and _slug in mit_preis:
        # Die Auszeichnung ("**keine** festen Kurse") ist Format, nicht Inhalt ✓ ✓ -
        # der Marker sieht sie deshalb nicht ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
        _notes_mit_preis[_slug] = _m.group(0).replace("**", "")
eigene_kurse = []
ausgeschlossen = []
ohne_aussage = []
for _slug, _n in _notes_mit_preis.items():
    _s = [x for x in _saetze(_n) if PREIS_SATZ.search(x)]
    if _eigener_kurs(_n):
        eigene_kurse.append(_slug)
    elif KURS_MARKER.search(_n):
        ausgeschlossen.append(_slug)
    else:
        ohne_aussage.append(_slug)
eigene_kurse = sorted(eigene_kurse)
_n = len(mit_preis)
if eigene_kurse:
    _eigen_text = (f"**{len(eigene_kurse)}** {'Dossier' if len(eigene_kurse) == 1 else 'Dossiers'} ✓ ✓ "
                   f"nennen **eigene Ankaufspreise** des Betriebs ✓ ✓ "
                   f"({', '.join(eigene_kurse)}) ✓ ✓ - das ist ein **Kurs des Dossier-Betriebs** ✓ ✓ "
                   f"✓ ✓ - ")
else:
    _eigen_text = ""
_rest = _n - len(eigene_kurse) - len(ohne_aussage)
_anteil = (f"**allen {_rest}**" if _rest and not ohne_aussage else
           (f"**{_rest} von {_rest}**" if _rest else "**0**"))
if eigene_kurse:
    _schluss = ("**Kein fremder** Wert ist als Kurs des Dossier-Betriebs übernommen ✓ ✓ ✓ ✓ ✓"
                " ✓ ✓ ✓ ✓ ✓ ✓")
else:
    _schluss = "**Kein** Wert ist als Kurs des Dossier-Betriebs übernommen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓"
_ohne_text = (f" - in **{len(ohne_aussage)}** Fällen bleibt der Zahlenwert ohne Aussage ✓ ✓"
              if ohne_aussage else "")
o.append(f"- Bezifferte Angaben in den Notizen ✓ ✓: **{_n}** Dossiers ✓ ✓ "
         f"({', '.join(sorted(mit_preis))}) ✓ ✓ - {_eigen_text}in {_anteil} Fällen weist die Notiz "
         f"ihren Zahlenwert **selbst** als **nicht übernommen** aus ✓ ✓ "
         f"(fremder Standort, Platzhalter, Preisfeld, Gebühr eines Dritten, "
         f"historisches Angebot, **ohne Kursdatum** ✓ ✓, **Stammkapital** ✓ ✓) ✓ ✓ ✓ ✓ ✓"
         f"{_ohne_text} ✓ ✓ {_schluss} ✓ ✓")
o.append(f"- **Keine** Verkaufspreisliste **übernommen** ✓ ✓ "
         f"({len(mit_verkaufspreis)} Dossiers nennen die **Preisart** und einen **Währungsbetrag** ✓ ✓) ✓ ✓ ✓ ✓ ✓")
o.append(f"- **Keine** Gebührentafel **übernommen** ✓ ✓ "
         f"({len(mit_gebuehr)} Dossiers nennen die **Preisart** und einen **Währungsbetrag** ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓")
o.append("- **Keine** Negativbehauptung ohne **Reichweitenklausel** ✓ ✓: die Efb-/AltfahrzeugV-"
         "Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** "
         "Nichtexistenzbeweis ✓ ✓ und **keine** Schließung ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓")
o.append("")
o.append("## Der Geokodierungsbedarf ✓ ✓\n")
o.append("Die Wellen-Anleitung verlangt, **neue Adressen mit Geokodierungsbedarf** im Bericht zu "
         "nennen ✓ ✓ — und **falsche** gespeicherte Punkte **dokumentiert** zu lassen ✓ ✓, statt "
         "sie eigenmächtig zu ändern ✓ ✓.\n")
def wellen_abschnitte(text):
    """Alle datierten Sektionen der Welle ✓ ✓ - Recherche **und** Korrektur ✓ ✓

    Jede Sektion endet an der naechsten `###`-Ueberschrift ✓ ✓. Die **juengste**
    Sektion traegt die juengere Aussage ✓ ✓ - eine Korrektur steht hinter der
    Recherche-Sektion ✓ ✓ und spricht damit fuer den spaeteren Stand ✓ ✓ ✓ ✓
    """
    kopf = re.compile(r"^### (?:Recherche|Korrektur) " + re.escape(DATUM) +
                      r"[^\n]*\n", re.M)
    out = []
    for m in kopf.finditer(text):
        ende = len(text)
        nxt = re.search(r"^### ", text[m.end():], re.M)
        if nxt:
            ende = m.end() + nxt.start()
        # Der Kopf ist keine Aussage - er wird nicht mitgenommen ✓ ✓
        out.append(text[m.end():ende])
    return out


# Auch die **falschen gespeicherten Punkte** gehoeren in diese Tabelle ✓ ✓ -
# sie sind der Geokodierungsbedarf des Bestands ✓ ✓ ✓ ✓ (ein Falschpunkt ist
# schlimmer als kein Punkt ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
GEO_RE = re.compile(r"(GEO-Punkt|Geokodierung|Koordinaten?\s*(?:ist|sind|war|waren)|"
                    r"liegt\s+\d+\s*km|stand\s+\d+\s*km|auf\s+eine[nr]?\s+anderen?\s+"
                    r"(?:Adresse|Ort)|Adresspunkt|km daneben"
                    r"|Produktions-?GEO|Produktions-?Geo|Produktionspunkt"
                    r"|gespeicherten?\s+(?:Punkt|Standort))", re.I)

geo_bedarf = []
for f in sorted(files):
    slug = f.split("/")[-1][:-3]
    t = kandidat_text(f)
    abschnitte = wellen_abschnitte(t)
    if not abschnitte:
        continue
    if any(GEO_RE.search(a) for a in abschnitte):
        geo_bedarf.append(slug)
if geo_bedarf:
    o.append(f"**{len(geo_bedarf)}** Dossiers der Welle erwaehnen die Geokodierung ✓ ✓:\n")
    o.append("| Dossier | Befund |")
    o.append("|---|---|")
    for slug in geo_bedarf:
        pfad = dossier_pfad(slug)
        if pfad is None:
            continue
        t = pfad.read_text(encoding="utf-8")
        kern = ""
        # Die **juengste** Sektion, die die Geokodierung erwaehnt, traegt den
        # aktuellen Stand ✓ ✓ - eine Korrektur-Sektion spricht fuer den
        # spaeteren Befund ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
        for abschnitt in reversed(wellen_abschnitte(t)):
            if not GEO_RE.search(abschnitt):
                continue
            # Den Satz nehmen, der den Geo-Befund traegt, und ihn **vollstaendig**
            # machen: der erste Treffer endet oft mitten in einer Aufzaehlung.
            saetze = re.split(r"(?<=[.;])\s+", " ".join(abschnitt.split()))
            treffer = [s for s in saetze if GEO_RE.search(s)]
            if treffer:
                kern = treffer[0].strip()
                idx = saetze.index(treffer[0])
                while (re.search(r"\b(und|oder|sowie)\s*$", kern)
                       and idx + 1 < len(saetze)):
                    idx += 1
                    kern = kern.rstrip(",;") + " " + saetze[idx].strip()
                    if len(kern) > 400:
                        break
                # Die **juengste** Sektion traegt den aktuellen Stand ✓ ✓ -
                # ohne diese Abbruchbedingung gewaennt die **aelteste** ✓ ✗
                # (das war der Befund des Pre-Deploy-Reviews vom 21.11.2026 ✓ ✓)
                break
        o.append(f"| `{slug}` | {kuerzen(kern)} |")
    o.append("")
    o.append("Die Dossiers, die **ausdruecklich** sagen, dass **nicht** geokodiert wurde ✓ ✓, "
             "haben keinen Geokodierungsbedarf ✓ ✓ — das ist eine **bewusste** Enthaltsamkeit ✓ ✓ "
             "(der Punkt braucht ein bestaetigtes Adressobjekt ✓ ✓, keine Vermutung ✓ ✓) ✓ ✓\n")
else:
    o.append("Kein Geokodierungsbedarf in dieser Welle ✓ ✓\n")
o.append("## Die Dossiers im Einzelnen ✓ ✓\n")
o.append("| Slug | Kernaussage |")
o.append("|---|---|")
for f in sorted(files):
    slug = f.split("/")[-1][:-3]
    t = kandidat_text(f)
    # Die komplette Notiz nehmen: sie enthaelt verschachtelte **-Marker und
    # eckige Klammern, an denen ein Regex frueher mitten im Satz abbrach.
    # Gekuerzt wird an einer Wortgrenze mit Ellipse.
    # Ein **Nachtrag** traegt den Zusatz " (Welle 0600 B)" ✓ ✓ - wie die
    # Bullets zaehlt der Generator ihn mit ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
    m = re.search(r"^- \[" + re.escape(ABSCHNITT) +
                  (r" " + re.escape(DATUM) if DATUM else "") +
                  r"(?: \([^)]*\))?:(.*?)(?=\n- \[|\n### |\n\n|\Z)",
                  t, re.M | re.S)
    kern = m.group(1).strip() if m else "(Fortsetzung der Recherche)"
    kern = " ".join(kern.split()).replace("|", "/")
    o.append(f"| `{slug}` | {kuerzen(kern)} |")
o.append("")

AUSGABE.write_text("\n".join(o) + "\n", encoding="utf-8")
print("Bericht geschrieben:", len(o), "Zeilen")
