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
    for feld in FELDER:
        if re.search(rf"`{feld}`", satz, re.I):
            gefunden.add(feld)
    if re.search(r"im\s+Frontmatter", satz, re.I):
        # "im Frontmatter" allein benennt das Feld nicht - der Satz muss es
        # zusätzlich nennen
        for feld in FELDER:
            if re.search(rf"\b{feld}\b", satz, re.I):
                gefunden.add(feld)
    return gefunden


def behauptungen(notiz):
    """Die (Feld, Satz)-Paare, in denen eine Aenderung behauptet wird."""
    treffer = []
    for satz in Satztrenner.split(notiz):
        if not re.search(AENDERUNG, satz, re.I):
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
