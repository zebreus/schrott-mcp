#!/usr/bin/env python3
"""Setzt das description-Feld eines Dossiers (Single-Source-Frontmatter-Edit).

Nur lokale Dateibearbeitung im Arbeitsbaum, keine Datenbankaenderung.
Aufruf:
    python3 tools/set_description.py <slug> <text>
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main():
    slug, text = sys.argv[1], sys.argv[2]
    matches = list((ROOT / "dossiers").rglob(f"{slug}.md"))
    if not matches:
        raise SystemExit(f"Dossier {slug} nicht gefunden")
    path = matches[0]
    content = path.read_text(encoding="utf-8")
    # YAML-Skalar: einfache Anfuehrungszeichen, Innere werden verdoppelt
    value = "'" + text.replace("'", "''") + "'"
    new, n = re.subn(r"^description: .*$", f"description: {value}", content,
                      count=1, flags=re.M)
    if n != 1:
        raise SystemExit(f"description-Feld in {path} nicht gefunden")
    path.write_text(new, encoding="utf-8")
    print(f"{path}: description gesetzt ({len(text)} Zeichen)")


if __name__ == "__main__":
    main()
