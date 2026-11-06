#!/usr/bin/env python3
"""Stellt ein beim Umschreiben des `certifications`-Eintrags verlorenes
**Element** wieder her.

Befund aus der Pre-Deploy-Pruefung vom 06.11.2026: die Tiefen-Welle 10:00 hat
in fuenf Dossiers das Element ``AltfahrzeugV: kein Eintrag im amtlichen
Fachbetrieberegister ...`` aus dem `certifications`-Eintrag **verworfen**,
obwohl das Element inhaltlich unveraendert bleiben musste (es ist ein
Negativbefund aus einer anderen Register-Route und keine Zertifikatsangabe).

Dieses Werkzeug stellt genau solche Elemente wieder her: es haelt die
Elemente der Basis fest, prueft, welche davon im Kandidaten fehlen, und
setzt sie an ihrer alten Position wieder ein. **Nur** Elemente, die in der
Basis vorhanden waren und im Kandidaten fehlen, werden ergaenzt - es wird
nichts erfunden und nichts umformuliert.

Aufruf: python3 scripts/release/restore_lost_elements.py <basis> [dossier ...]
"""
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def certs(text):
    """Die Zertifikatseintraege als Liste (oder None)."""
    m = re.search(r"^certifications:\s*(.*)$", text, re.M)
    if not m:
        return None
    raw = m.group(1).strip()
    if len(raw) >= 2 and raw[0] == "'" and raw[-1] == "'":
        raw = raw[1:-1]
    if not raw:
        return None
    try:
        val = json.loads(raw)
    except json.JSONDecodeError:
        return None
    return val if isinstance(val, list) else None


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    basis = sys.argv[1]
    nur = set(sys.argv[2:])
    dateien = subprocess.run(["git", "diff", "--name-only", basis, "--", "dossiers"],
                             capture_output=True, text=True, check=True).stdout.split()
    bericht = []
    for rel in dateien:
        if nur and Path(rel).name not in nur:
            continue
        alt = subprocess.run(["git", "show", f"{basis}:{rel}"],
                             capture_output=True, text=True).stdout
        pfad = ROOT / rel
        neu = pfad.read_text(encoding="utf-8")
        eb, en = certs(alt), certs(neu)
        if eb is None or en is None or len(en) >= len(eb):
            continue
        # nur Elemente wiederherstellen, die **unveraendert** mitgeschrieben
        # wurden und nicht nur umformuliert sind: ein Element, dessen Text sich
        # aendert, ist in der Basis nicht mehr vorhanden - es wurde ja
        # bearbeitet, nicht verloren
        fehlend = []
        for x in eb:
            if x in en:
                continue
            # veraendert? dann gibt es eine Entsprechung im Kandidaten
            veraendert = False
            for y in en:
                if y == x:
                    continue
                # gleiche Vorgangsnummer / gleicher Anfang -> umformuliert
                if x[:60] == y[:60]:
                    veraendert = True
                    break
            if not veraendert:
                fehlend.append(x)
        if not fehlend:
            continue
        # die verlorenen Elemente an ihrer alten Position wieder einsetzen
        neu_els = list(en)
        for x in fehlend:
            # Position in der Basis bestimmen
            idx_basis = eb.index(x)
            # nach dem Element davor (oder davor im Kandidaten) einsetzen
            if idx_basis == 0:
                pos = 0
            else:
                vor = eb[idx_basis - 1]
                if vor in neu_els:
                    pos = neu_els.index(vor) + 1
                else:
                    pos = len(neu_els)
            neu_els.insert(pos, x)
        m = re.search(r"^certifications:\s*(.*)$", neu, re.M)
        roh = m.group(1)
        quote = roh[0]
        neu_roh = quote + json.dumps(neu_els, ensure_ascii=False) + quote
        neu = neu[:m.start()] + "certifications: " + neu_roh + neu[m.end():]
        pfad.write_text(neu, encoding="utf-8")
        bericht.append({"dossier": rel, "wiederhergestellt": fehlend})
        print(f"  {rel}: {len(fehlend)} Element(e) wiederhergestellt")
        for x in fehlend:
            print(f"     + {x[:140]}")
    json.dump(bericht, open("/tmp/opencode/wiederhergestellte-elemente.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"Dossiers geaendert: {len(bericht)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
