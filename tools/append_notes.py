#!/usr/bin/env python3
"""Haengt die vorbereiteten Recherche-Notizen ans Timeline-Ende der Dossiers.

Nur lokale Dateibearbeitung im Arbeitsbaum, keine Datenbank- oder
Produktionsaenderung. Erwartet wird je Dossier eine Notizdatei mit genau
einem Bullet im Hausformat:

    - [Recherche 08.11.2026: ...]
      Quelle(n): ... (Abruf 08.11.2026).]

Aufruf:
    python3 tools/append_notes.py <notizverzeichnis> <slug> [<slug> ...]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATE = "08.11.2026"
SECTION = f"### Recherche {DATE}"


def main():
    notizdir = Path(sys.argv[1])
    for slug in sys.argv[2:]:
        matches = list((ROOT / "dossiers").rglob(f"{slug}.md"))
        if not matches:
            raise SystemExit(f"Dossier {slug} nicht gefunden")
        path = matches[0]
        text = path.read_text(encoding="utf-8")
        if SECTION in text:
            raise SystemExit(f"{path}: Sektion '{SECTION}' existiert bereits")
        body = (notizdir / f"note_{slug.split('-', 1)[1]}.md").read_text(
            encoding="utf-8").rstrip("\n")
        lines = body.split("\n")
        assert lines[0].startswith(f"- [Recherche {DATE}:"), lines[0][:60]
        assert body.endswith("]"), body[-40:]
        text = text.rstrip("\n") + f"\n\n{SECTION}\n\n{body}\n"
        path.write_text(text, encoding="utf-8")
        print(f"{path.name}: Sektion angehaengt ({len(body)} Zeichen)")


if __name__ == "__main__":
    main()
