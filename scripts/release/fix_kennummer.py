#!/usr/bin/env python3
"""Behebt den Tippfehler „Kennummer“ im Feld `certifications`.

Befund aus der Pre-Deploy-Pruefung vom 06.11.2026: in 20 Dossiers trug das
veroeffentlichte Feld `certifications` die Schreibweise „Kennummer“ statt
„Kennnummer“. Der Fehler ist ein reiner Tippfehler im Wortlaut - der
Zertifikatsinhalt (die §-28-NachwV-Kennnummer selbst) ist richtig.

Das Werkzeug:
  * ersetzt im Feld `certifications` „Kennummer“ durch „Kennnummer“ ✓
    (byteweise, ohne sonst etwas zu aendern ✓),
  * ergaenzt in Dossiers, deren Notiz **nicht** in place geaendert werden
    darf (veroeffentlichte Basis-Notiz ✓), eine datierte Korrektur-Sektion am
    Timeline-Ende ✓, die den Tippfehler benennt ✓.

Aufruf: python3 scripts/release/fix_kennummer.py <basis> [nur-in-scope|alle]
Exitcode 0 = bestanden.
"""
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DATUM = "06.11.2026"


def cert_zeile(text):
    m = re.search(r"^certifications: (.*)$", text, re.M)
    return m


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    basis = sys.argv[1]
    nur_scope = (len(sys.argv) > 2 and sys.argv[2] == "nur-in-scope")
    scope = set(subprocess.run(["git", "diff", "--name-only", basis, "--", "dossiers/"],
                               capture_output=True, text=True, check=True).stdout.split())
    dateien = subprocess.run(["git", "ls-files", "dossiers/"],
                             capture_output=True, text=True, check=True).stdout.split()
    geaendert, notizen = 0, 0
    for rel in dateien:
        if nur_scope and rel not in scope:
            continue
        pfad = ROOT / rel
        text = pfad.read_text(encoding="utf-8")
        m = cert_zeile(text)
        if not m:
            continue
        roh = m.group(1)
        if "Kennummer" not in roh:
            continue
        alt = subprocess.run(["git", "show", f"{basis}:{rel}"],
                             capture_output=True, text=True).stdout
        basis_zeilen = set(alt.split("\n"))
        # 1) das Feld korrigieren (byteweise)
        neu_roh = roh.replace("Kennummer", "Kennnummer")
        text = text[:m.start()] + "certifications: " + neu_roh + text[m.end():]
        # 2) die Notizen: nur Zeilen, die **nicht** in der Basis stehen
        zeilen, raus, gea = text.split("\n"), [], 0
        for z in zeilen:
            if "Kennummer" in z and z not in basis_zeilen:
                z = z.replace("Kennummer", "Kennnummer")
                gea += 1
            raus.append(z)
        text = "\n".join(raus)
        # 3) steht der Tippfehler noch in einer Basis-Notiz, braucht es eine
        #    datierte Korrektur-Sektion
        rest_basis = [z for z in text.split("\n")
                      if "Kennummer" in z and z in basis_zeilen]
        if rest_basis:
            text = text.rstrip("\n") + "\n\n" + (
                f"### Korrektur {DATUM} (Tippfehler)\n\n"
                f"- [Korrektur {DATUM} (Tippfehler): **Im Feld `certifications` stand "
                f"„Kennummer“ statt „Kennnummer“.** Die §-28-NachwV-Kennnummer selbst ist "
                f"richtig ✓ - korrigiert wurde allein die Schreibweise des Wortes ✓. Die "
                f"vorherige Notiz zitiert den alten Wortlaut ✓; sie bleibt unverändert ✓ "
                f"(Hausregel: veroeffentlichte Notizen werden nicht in place geaendert ✓).]\n")
            notizen += 1
        pfad.write_text(text, encoding="utf-8")
        geaendert += 1
        print(f"  {rel}: Feld korrigiert ✓"
              + (f" | Korrektur-Sektion ergaenzt ✓" if rest_basis else ""))
    print(f"Dossiers mit korrigiertem Feld: {geaendert} | davon mit Korrektur-Sektion: {notizen}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
