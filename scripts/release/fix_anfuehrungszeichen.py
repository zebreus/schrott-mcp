#!/usr/bin/env python3
"""Stellt die **deutschen Anfuehrungszeichen** in den `certifications`-Eintraegen
wieder her.

Befund aus der Pre-Deploy-Pruefung vom 07.11.2026: die Tiefen-Welle 10:00 hat
in neun Dossiers die schliessenden deutschen Anfuehrungszeichen **„ → “**
durch das ASCII-Zeichen ``"`` ersetzt ✓. Dadurch gibt es im Feldtext ein
oeffnendes **„** ohne schliessendes **“** ✓ - der Text gibt das PDF-Zitat
nicht mehr woertlich wieder ✓, obwohl die Notiz es als woertlich zitiert ✓.

Dieses Werkzeug stellt die Paarung wieder her ✓: ein ``"``, das auf ein
Wortzeichen folgt ✓, ist ein **schliessendes** **“** ✓; alle anderen **"**
sind **oeffnende** **„** ✓. Die Klammerbalance wird dabei nicht beruehrt ✓.

Aufruf: python3 scripts/release/fix_anfuehrungszeichen.py <basis>
"""
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve()[0] if False else Path(__file__).resolve().parents[2]
CERT_RE = re.compile(r"^certifications:\s*(.*)$", re.M)


def certs(text):
    """Die Zertifikatseintraege als Liste (oder None)."""
    m = CERT_RE.search(text)
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


def paaren(text):
    """Die ASCII-Anfuehrungszeichen in ein schliessendes und ein oeffnendes
    deutsches Anfuehrungszeichen ueberfuehren."""
    if '"' not in text:
        return text
    out = []
    for i, ch in enumerate(text):
        if ch != '"':
            out.append(ch)
            continue
        prev = text[i - 1] if i > 0 else ""
        # ein Wortzeichen davor -> es schliesst ein Zitat
        out.append("“" if (prev.isalnum() or prev in ").,") else "„")
    return "".join(out)


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    basis = sys.argv[1]
    dateien = subprocess.run(["git", "ls-tree", "-r", "--name-only", basis, "dossiers/"],
                             capture_output=True, text=True, check=True).stdout.split()
    geaendert = 0
    for rel in dateien:
        if not rel.endswith(".md"):
            continue
        pfad = ROOT / rel
        text = pfad.read_text(encoding="utf-8")
        val = certs(text)
        if val is None:
            continue
        neu = [paaren(e) for e in val]
        if neu == val:
            continue
        m = CERT_RE.search(text)
        raw = m.group(1)
        neu_roh = raw[0] + json.dumps(neu, ensure_ascii=False) + raw[-1]
        pfad.write_text(text[:m.start()] + "certifications: " + neu_roh + text[m.end():],
                       encoding="utf-8")
        geaendert += 1
        print(f"  {rel}: Anfuehrungszeichen wieder gepaart ✓")
    print(f"Dossiers geaendert: {geaendert}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
