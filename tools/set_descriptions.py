#!/usr/bin/env python3
"""Setzt description-Felder mehrerer Dossiers aus Textdateien.

Nur lokale Dateibearbeitung im Arbeitsbaum.
Aufruf:
    python3 tools/set_descriptions.py <verzeichnis-mit-slug.txt-dateien>
    wherein jede Datei "<slug>.txt" heisst und den Beschreibungstext enthaelt.
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from set_description import main as _set  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]


def main():
    src = Path(sys.argv[1])
    for f in sorted(src.glob("*.txt")):
        slug = f.stem
        text = f.read_text(encoding="utf-8").strip()
        sys.argv = ["set_description.py", slug, text]
        _set()


if __name__ == "__main__":
    main()
