#!/usr/bin/env python3
"""Apply a correction manifest to dossiers.

Each entry names a dossier, an exact `old` substring (must occur exactly once)
and its `new` replacement; additionally a dated correction note is appended to
the last wave section of the dossier.  Nothing else in the file is touched.
"""
import json
import re
import sys
from pathlib import Path

DATE = "30.10.2026"


def sections(text):
    tail = text.split("## Timeline", 1)[1]
    parts = re.split(r"(?m)^(### .*)$", tail)
    return [(parts[i], parts[i + 1]) for i in range(1, len(parts), 2)]


def apply(rel, entry, dry=False):
    path = Path(rel)
    text = path.read_text()
    old, new = entry["old"], entry["new"]
    n = text.count(old)
    want = entry.get("count", 1)
    if n != want:
        print(f"  !! {rel}: 'old' kommt {n}x vor (erwartet {want}) -> uebersprungen")
        return False
    # alle Vorkommen ersetzen (nur das erste zu ersetzen war ein Werkzeug-Bug:
    # der angehaengte Korrekturvermerk zitiert die alte Form erneut)
    text = text.replace(old, new)
    note = entry.get("note")
    if note:
        bullet = f"- [{DATE} (Korrekturvermerk): {note}]\n"
        secs = sections(text)
        if not secs:
            print(f"  !! {rel}: keine Timeline-Sektion")
            return False
        head, body = secs[-1]
        if not body.endswith("\n"):
            body += "\n"
        text = text.replace(head + body, head + body + "\n" + bullet, 1)
    if not dry:
        path.write_text(text)
    print(f"  ok {rel}: {entry['label']}")
    return True


if __name__ == "__main__":
    manifest = json.loads(Path(sys.argv[1]).read_text())
    dry = "--dry-run" in sys.argv
    done = failed = 0
    for entry in manifest:
        if apply(entry["file"], entry, dry):
            done += 1
        else:
            failed += 1
    print(f"Eintraege: {len(manifest)} | angewendet: {done} | fehlgeschlagen: {failed}")
    sys.exit(1 if failed else 0)
