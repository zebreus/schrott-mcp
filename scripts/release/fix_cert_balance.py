#!/usr/bin/env python3
"""Heilt die **unausgeglichenen** `certifications`-Eintraege des Altbestands.

Befund aus der Pre-Deploy-Pruefung: sieben Dossiers tragen einen Eintrag,
dessen Klammerbilanz nicht null ist ✓ - bei fuenf fehlt die schliessende
Klammer ✓, bei zwei ist eine oeffnende Klammer zu frueh geschlossen ✓
(dort schliesst die Klammer nach „N Anlagen)“ ✓, obwohl die Anlagen-Angaben
noch folgen ✓).

Das Werkzeug:
  * **fehlende** schliessende Klammer -> sie wird **byteweise am Ende** des
    Elements ergaenzt ✓ (kein Wort, keine Zahl, kein Leerzeichen geaendert ✓);
  * **ueberzaehlige** schliessende Klammer -> es wird die fehlende
    **oeffnende** Klammer vor der Anlagen-Angabe ergaenzt ✓, damit der
    Zitatrahmen die Anlagen einschliesst ✓.

Beide Faelle aendern **keinen** inhaltlichen Wert ✓ - nur die Klammerung ✓.

Aufruf: python3 scripts/release/fix_cert_balance.py [scope-dir]
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CERT_RE = re.compile(r"^certifications:\s*(.*)$", re.M)
ANLAGEN_RE = re.compile(r"(\d+ Anlagen?)\), (Anlage \d)")


def bilanz(text):
    return text.count("(") - text.count(")")


def ueberzaehlig_pos(text):
    """Die Position der ersten ueberzaehligen schliessenden Klammer."""
    tiefe = 0
    for i, ch in enumerate(text):
        if ch == "(":
            tiefe += 1
        elif ch == ")":
            tiefe -= 1
            if tiefe < 0:
                return i
    return -1


def main() -> int:
    scope = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "dossiers"
    geaendert = 0
    for pfad in sorted(scope.rglob("*.md")):
        text = pfad.read_text(encoding="utf-8")
        m = CERT_RE.search(text)
        if not m:
            continue
        roh = m.group(1).strip()
        if len(roh) >= 2 and roh[0] == "'" and roh[-1] == "'":
            roh = roh[1:-1]
        if not roh:
            continue
        try:
            els = json.loads(roh)
        except json.JSONDecodeError:
            continue
        if not isinstance(els, list) or not all(isinstance(x, str) for x in els):
            continue
        neu = []
        for e in els:
            b = bilanz(e)
            if b > 0:
                # die schliessende Klammer fehlt -> am Ende ergaenzen
                e = e + ")" * b
                art = f"{b} schliessende Klammer ergaenzt"
            elif b < 0:
                # eine oeffnende Klammer fehlt: sie gehoert vor die
                # Anlagen-Angabe, damit der Zitatrahmen sie einschliesst
                m2 = ANLAGEN_RE.search(e)
                if not m2:
                    print(f"  {pfad.name}: ueberzaehlige Klammer, aber keine "
                          f"Anlagen-Angabe gefunden - manuell zu pruefen")
                    continue
                e = e[:m2.start(1)] + m2.group(1) + "), (" + m2.group(2) + e[m2.end(2):]
                art = "oeffnende Klammer vor der Anlagen-Angabe ergaenzt"
            else:
                neu.append(e)
                continue
            neu.append(e)
        if neu == els:
            continue
        neu_roh = "'" + json.dumps(neu, ensure_ascii=False) + "'"
        pfad.write_text(text[:m.start()] + "certifications: " + neu_roh + text[m.end():],
                        encoding="utf-8")
        geaendert += 1
        print(f"  {pfad.relative_to(ROOT)}: {art}")
    print(f"Dossiers geaendert: {geaendert}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
