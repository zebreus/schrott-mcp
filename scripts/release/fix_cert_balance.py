#!/usr/bin/env python3
"""Ergaenzt die fehlende schliessende Klammer in `certifications`-Eintraegen
und dokumentiert das im Vermerk des Dossiers.

Aufruf:  python3 scripts/release/fix_cert_balance.py <basis-commit> <scope-dir>
"""
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CERT_RE = re.compile(r"^certifications: (.*)$", re.M)


def parse_line(raw: str):
    """(prefix, suffix, elementliste) aus der YAML-Zeile."""
    s = raw.strip()
    if s.startswith("'"):
        return "'", "'", s[1:-1]
    return '"', '"', s[1:-1]


def unbalanced(value: str):
    """Index des Elements mit unausgeglichener Klammer (oder None)."""
    try:
        items = json.loads(value)
    except json.JSONDecodeError:
        return None
    for i, e in enumerate(items):
        if e.count("(") != e.count(")"):
            return i
    return None


def fix_cert_line(text: str):
    """Die certifications-Zeile korrigieren; gibt (neuer_text, element) zurueck.

    Der Text wird **nicht** neu serialisiert - die Zeile bleibt byteweise
    erhalten, nur die fehlende schliessende Klammer wird **innerhalb** des
    betroffenen Elements ergaenzt (vor dessen abschliessendem Anfuehrungs-
    zeichen). Der Schluessel "certifications:" bleibt unangetastet.
    """
    m = CERT_RE.search(text)
    if not m:
        return None
    raw = m.group(1)
    inner = raw[1:-1]
    try:
        items = json.loads(inner)
    except json.JSONDecodeError:
        return None
    spans, pos, end_all = [], 1, len(inner) - 1
    while pos < end_all:
        ch = inner[pos]
        if ch in " \t\n,":
            pos += 1
            continue
        if ch != '"':
            break
        j = pos + 1
        while j < end_all:
            if inner[j] == "\\":
                j += 2
                continue
            if inner[j] == '"':
                break
            j += 1
        spans.append((pos, j))
        pos = j + 1
    for i, (a, b) in enumerate(spans):
        if i < len(items) and items[i].count("(") - items[i].count(")") == 1:
            new_inner = inner[:b] + ")" + inner[b:]
            return text[:m.start()] + m.group(0)[:m.group(0).index(raw)] + raw[0] + new_inner + raw[-1] + text[m.end():], i
    return None


SATZ = ("Der Eintrag endete mitten in einer offenen Klammer ✗ — ergänzt wurde die "
        "schließende Klammer ✓ (kein inhaltlicher Wert geändert ✓).")


def extend_note(text: str) -> str:
    """Einen Satz in den letzten Vermerk haengen - vor der Zeile "Quelle(n):"."""
    idx = text.rfind("]")
    if idx < 0:
        return text
    if not text[idx:].startswith("]"):
        return text
    # Zeile "Quelle(n):" des letzten Vermerks suchen
    pos = text.rfind("\n  Quelle(n):", 0, idx)
    if pos < 0:
        pos = text.rfind("\n", 0, idx)      # sonst vor dem Schluss
        return text[:pos] + "\n  " + SATZ + text[pos:]
    return text[:pos] + "\n  " + SATZ + text[pos:]


def main() -> int:
    base = sys.argv[1]
    scope = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT / "dossiers"
    changed = subprocess.run(["git", "diff", "--name-only", base, "--", str(scope)],
                              capture_output=True, text=True, check=True).stdout.split()
    fixed = 0
    for rel in changed:
        path = Path(rel)
        text = path.read_text(encoding="utf-8")
        res = fix_cert_line(text)
        if not res:
            continue
        neu_text, idx = res
        neu_text = extend_note(neu_text)
        path.write_text(neu_text, encoding="utf-8")
        fixed += 1
        print(f"  {rel}: Element {idx} ergaenzt")
    print(f"Ergaenzte Klammern: {fixed}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
