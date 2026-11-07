#!/usr/bin/env python3
"""Ersetzt die **temporaeren Dateipfade** in den Belegangaben der Dossiers.

Die datierten Korrektur-Sektionen „Zertifikatstiefe“ nannten als Quelle den
Arbeitspfad der PDF-Datei (etwa ``/tmp/opencode/pdf_neuhaus.pdf``) - das ist
kein Beleg ✓, sondern ein Pfad, der nach dem Wellenlauf nicht mehr existiert ✓.

Dieses Werkzeug ersetzt ihn durch die **oeffentliche Permalink-Route** des
amtlichen Fachbetrieberegisters zum Zertifikat (mit der Vorgangsnummer ✓) -
die bleibt dauerhaft abrufbar ✓ und belegt dasselbe Dokument ✓.

Aufruf: python3 scripts/release/fix_korrektur_quellen.py <basis>
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ROUTE = ("https://fachbetrieberegister.gadsys.de/fachbetrieberegister/"
         "Entsorgungsfachbetriebe/dst=10&is=1&e=1&n={name}&p={vg}&fd=1")


def permalink(vg, name):
    """Die oeffentliche Permalink-Route zum Zertifikat."""
    return ROUTE.format(vg=vg, name=name)


def main() -> int:
    basis = sys.argv[1]
    dateien = subprocess.run(["git", "diff", "--name-only", basis, "--", "dossiers/"],
                             capture_output=True, text=True, check=True).stdout.split()
    geaendert = 0
    for rel in dateien:
        pfad = ROOT / rel
        text = pfad.read_text(encoding="utf-8")
        if not re.search(r"Quelle\(n\):\s*/tmp/opencode", text):
            continue
        zeilen = text.split("\n")
        raus = []
        for z in zeilen:
            if "Quelle(n):" in z and "/tmp/opencode" in z:
                # die Vorgangsnummer aus der Notiz holen
                vg = None
                m = re.search(r"Vorgangsnummer (ZZ[A-Z]{2}\d{6,14})", z)
                if m:
                    vg = m.group(1)
                # der Dossiername (ohne Bundesland-Praefix) fuer die Namenssuche
                slug = Path(rel).stem
                name = slug.split("-", 1)[1] if "-" in slug else slug
                name = name.replace("-", " ")
                z = re.sub(r"Quelle\(n\):\s*/tmp/opencode\S+",
                           f"Quelle(n): {permalink(vg, name)}",
                           z)
            raus.append(z)
        neu = "\n".join(raus)
        if neu != text:
            pfad.write_text(neu, encoding="utf-8")
            geaendert += 1
            print(f"  {rel}: Quelle auf die Register-Permalink umgestellt ✓")
    print(f"Dossiers geaendert: {geaendert}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
