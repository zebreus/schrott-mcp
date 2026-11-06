#!/usr/bin/env python3
"""Prueft die Klammerbalance der `certifications`-Eintraege im Wellen-Scope.

Befund aus der Pre-Deploy-Pruefung vom 06.11.2026: mehrere Eintraege im
Feld `certifications` oeffnen eine Klammer und schliessen sie nicht - der
Text endet mitten im Satz. JSON bleibt valide, die Daten sind unveraendert,
der Eintrag ist aber unlesbar. Im Wellen-Scope der Welle 06.11.2026 waren
12 der 148 Dossiers betroffen, 5 davon bereits in der Basis.

Das Gate meldet nur, was die **Welle** neu einfuehrt: eine Balance, die in der
Basis vorhanden war, wird nicht beanstandet (der Altbestand wird als eigener
Cleanup-Punkt gefuehrt). Neu ist eine unbalancierte Klammer, eine
verschlechterte Balance oder ein Dossier, dessen `certifications` gar kein
flaches JSON-Array mehr ist.

Aufruf:  python3 scripts/release/check_cert_balance.py <basis-commit> [scope-dir]
         python3 scripts/release/check_cert_balance.py --all
Exitcode 0 = bestanden, 1 = Beanstandungen.
"""
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CERT_RE = re.compile(r"^certifications:\s*(.*)$", re.M)


def certs(text: str):
    """Die Zertifikatseintraege als Liste; None, wenn das Feld fehlt."""
    m = CERT_RE.search(text)
    if not m:
        return None
    raw = m.group(1).strip()
    if len(raw) >= 2 and raw[0] == "'" and raw[-1] == "'":
        raw = raw[1:-1]
    if not raw:
        return None            # kein Zertifikatseintrag (Feld bleibt leer)
    try:
        val = json.loads(raw)
    except json.JSONDecodeError:
        return "INVALID"
    if not isinstance(val, list) or not all(isinstance(x, str) for x in val):
        return "INVALID"
    return val


def balance(entry: str) -> int:
    """Klammerdifferenz eines Eintrags (0 = ausgeglichen)."""
    return entry.count("(") - entry.count(")")


def main() -> int:
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        return 2
    if args[0] == "--all":
        scope = Path(args[1]) if len(args) > 1 else ROOT / "dossiers"
        problems = []
        checked = 0
        for path in sorted(scope.rglob("*.md")):
            val = certs(path.read_text(encoding="utf-8"))
            if val is None:
                continue
            checked += 1
            if val == "INVALID":
                problems.append(f"{path}: certifications ist kein flaches JSON-Array")
                continue
            for i, entry in enumerate(val):
                if balance(entry):
                    problems.append(f"{path}: Element {i} unausgeglichen ({balance(entry):+d})")
        print(f"Gepruefte Dossiers: {checked} | Abweichungen: {len(problems)}")
        if problems:
            print(f"PROBLEME: {len(problems)}")
            for p in problems[:200]:
                print("  -", p)
            return 1
        print("Klammerbalance der certifications-Eintraege bestanden")
        return 0

    base = args[0]
    scope = Path(args[1]) if len(args) > 1 else ROOT / "dossiers"
    changed = subprocess.run(
        ["git", "diff", "--name-only", base, "--", str(scope)],
        capture_output=True, text=True, check=True).stdout.split()

    problems = []
    for rel in changed:
        old = subprocess.run(["git", "show", f"{base}:{rel}"],
                             capture_output=True, text=True, check=True).stdout
        new = Path(rel).read_text(encoding="utf-8")
        a, b = certs(old), certs(new)
        if b == "INVALID":
            problems.append(f"{rel}: certifications ist kein flaches JSON-Array")
            continue
        if b is None:
            continue
        if a == "INVALID":
            continue
        if a is None:
            # In der Basis gab es **kein** certifications-Feld: der Eintrag ist
            # neu. Dann gibt es keine Altbilanz, gegen die verglichen werden
            # kann - jede Unbalance ist hier vom Kandidaten **neu** eingefuehrt
            # und wird beanstandet.
            for i, entry in enumerate(b):
                neu = balance(entry)
                if neu:
                    problems.append(f"{rel}: Element {i} unausgeglichen ({neu:+d}) - "
                                    f"die Welle fuehrt die Klammerbalance ein "
                                    f"(das Feld certifications ist in der Basis neu)")
            continue
        for i, entry in enumerate(b):
            alt = balance(a[i]) if i < len(a) else 0
            neu = balance(entry)
            if neu and not alt:
                problems.append(f"{rel}: Element {i} unausgeglichen ({neu:+d}) - "
                                f"die Welle fuehrt die Klammerbalance ein")
            elif neu and abs(neu) > abs(alt):
                problems.append(f"{rel}: Element {i} unausgeglichen ({neu:+d}, Basis {alt:+d})")

    print(f"Dateien: {len(changed)} | Abweichungen: {len(problems)}")
    if problems:
        print(f"PROBLEME: {len(problems)}")
        for p in problems:
            print("  -", p)
        return 1
    print("Klammerbalance der certifications-Eintraege bestanden "
          "(keine neue unbalancierte Klammer im Wellen-Scope)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
