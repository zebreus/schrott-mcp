#!/usr/bin/env python3
"""Restore the wave section structure in the dossiers of a release scope.

The research waves must append their notes in a trailing section of their own.
This tool rebuilds every affected dossier from the release baseline:

  * the pre-wave sections are taken **verbatim** from the baseline
    (old notes stay word-identical and keep their original section and order)
  * the wave's new notes are appended in a trailing section of their own

Only the section placement changes; no note text is touched.
"""
import re
import subprocess
import sys
from pathlib import Path

BASE = sys.argv[1]          # release baseline (e.g. 93fa8595)
CAND = sys.argv[2]          # candidate commit whose dossiers are fixed in the worktree
DRY = "--dry-run" in sys.argv

SEC = re.compile(r"(?m)^(### .*)$")


def split_timeline(text):
    head, _, tail = text.partition("## Timeline")
    parts = SEC.split(tail)
    return head + "## Timeline", parts[0], [(parts[i], parts[i + 1]) for i in range(1, len(parts), 2)]


def bullets(body):
    """Bullet blocks of a section body (a bullet may wrap over several lines)."""
    out, cur = [], None
    for line in body.splitlines():
        if line.startswith("- "):
            if cur is not None:
                out.append(cur)
            cur = line
        elif cur is not None and line.strip():
            cur += "\n" + line
        elif cur is not None:
            out.append(cur)
            cur = None
    if cur is not None:
        out.append(cur)
    return out


def main() -> int:
    files = subprocess.run(
        ["git", "diff", "--name-only", BASE, CAND, "--", "dossiers/"],
        capture_output=True, text=True).stdout.split()
    changed = 0
    for rel in files:
        old = subprocess.run(["git", "show", f"{BASE}:{rel}"], capture_output=True, text=True).stdout
        path = Path(rel)
        if not old or not path.exists():
            continue
        new = path.read_text()
        o_head, o_pre, o_sec = split_timeline(old)
        n_head, n_pre, n_sec = split_timeline(new)
        old_bullets = [b for _, body in o_sec for b in bullets(body)]
        old_headers = [h for h, _ in o_sec]
        new_bullets = [b for _, body in n_sec for b in bullets(body)]
        # multiset difference: a bullet is new when it occurs more often than in the baseline
        counts = {}
        for b in old_bullets:
            counts[b] = counts.get(b, 0) + 1
        fresh = []
        for b in new_bullets:
            if counts.get(b, 0):
                counts[b] -= 1
            else:
                fresh.append(b)
        if not fresh:
            continue
        new_headers = [h for h, _ in n_sec if h not in old_headers]
        header = new_headers[0] if new_headers else "### Recherche 29.10.2026"
        if len(new_headers) > 1:
            print(f"  ! mehrere neue Header in {rel}: {new_headers}")
            continue
        # bereits konform? (eigene Schlusssektion, alle neuen Notizen am Tail) -> unveraendert
        if new_headers:
            last_header = n_sec[-1][0]
            tail_ok = False
            if last_header == new_headers[0]:
                last_bullets = bullets(n_sec[-1][1])
                tail_ok = all(b in last_bullets for b in fresh)
                # die Schlusssektion darf ausschliesslich die neuen Notizen tragen
                # (keine Alt-Notiz darf dorthin umgehaengt worden sein)
                old_set = set(old_bullets)
                tail_ok = tail_ok and not any(b in old_set for b in last_bullets)
            if tail_ok:
                continue
        # Ziel: Baseline-Abschnitte wortgleich + neue Schlusssektion
        body_parts = [o_pre]
        for h, body in o_sec:
            body_parts.append(h + body.rstrip("\n") + "\n\n")
        body_parts.append(header + "\n\n" + "\n".join(fresh) + "\n")
        target = o_head + "".join(body_parts).rstrip("\n") + "\n"
        # Kopf (Frontmatter + Überblick) aus dem Kandidaten übernehmen
        cand_head = n_head
        target = cand_head + target[len(o_head):]
        if target == new:
            continue
        changed += 1
        print(f"  {'TROCKEN ' if DRY else ''}fix: {rel} | neue Header={header!r} | "
              f"Notizen={len(fresh)} | Baseline-Sektionen={len(o_sec)}")
        if not DRY:
            path.write_text(target)
    print(f"Dateien: {len(files)} | geändert: {changed}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
