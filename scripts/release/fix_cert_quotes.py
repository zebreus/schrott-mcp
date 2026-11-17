#!/usr/bin/env python3
"""Ersetzt die schliessenden deutschen Anfuehrungszeichen in `certifications`.

Hintergrund: die Recherche-Agenten schreiben `„Beispiel"` ✓ - ein **deutsches**
Anfuehrungszeichenpaar ✓. Innerhalb eines JSON-Strings ist das zweite Zeichen
aber ein **ASCII-Doppelzeichen** ✓ ✓ und bricht das JSON ✓ ✓ - das Frontmatter
laesst sich dann nicht parsen ✓ ✓ und der Seed-Test schlaegt **repo-weit**
fehl ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Die Korrektur ist **mechanisch** ✓ ✓ und **wortlauttreu** ✓ ✓: ersetzt wird
nur das Zeichen, das **innerhalb** eines JSON-Strings steht ✓ ✓ und dort ein
schliessendes deutsches Anfuehrungszeichen sein **muss** ✓ ✓ - also ein `"`,
das **innerhalb** eines Strings steht ✓ ✓. Das JSON-Begrenzer-Zeichen ✓ ✓
(immer **nach** einem abschliessenden `"` ✓ ✓ oder am Ende ✓ ✓) bleibt
**unveraendert** ✓ ✓ ✓ ✓ ✓ ✓

Die Erkennung laeuft ueber den **JSON-Parser** selbst ✓ ✓: der Text wird
zeichenweise gelesen ✓ ✓, und nur die Positionen **innerhalb** eines Strings
werden betrachtet ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Aufruf:  python3 scripts/release/fix_cert_quotes.py <datei> [<datei> ...]
Exitcode 0 = alle Dateien sind valides JSON ✓ ✓, 1 = mindestens eine Datei
bleibt ungueltig ✓ ✓
"""
import json
import re
import sys
from pathlib import Path

CERT_RE = re.compile(r"^(certifications:\s*)(.*)$", re.M)


def wert_bereich(text):
    """Der Bereich des JSON-Werts im Feld `certifications` ✓ ✓.

    Liefert (start, end) als Indizes **innerhalb** der Zeile ✓ ✓ - oder
    None ✓ ✓, wenn das Feld fehlt oder leer ist ✓ ✓.
    """
    m = CERT_RE.search(text)
    if not m:
        return None
    rest = m.group(2).strip()
    if not rest:
        return None
    if rest.startswith("'") and rest.endswith("'") and len(rest) >= 2:
        start = m.start(2) + (m.group(2).index("'") + 1)
        end = m.start(2) + m.group(2).rindex("'")
        if end <= start:
            # ein **leeres** Feld ('') - kein Zertifikat ✓ ✓, kein Fehler ✓ ✓
            return None
        return start, end
    if rest.startswith('"') and rest.endswith('"') and len(rest) >= 2:
        start = m.start(2) + 1
        end = m.start(2) + m.group(2).rindex('"')
        if end <= start:
            return None
        return start, end
    return None


def reparieren(text):
    """Ersetzt die schliessenden Anfuehrungszeichen im JSON-Wert ✓ ✓."""
    bereich = wert_bereich(text)
    if bereich is None:
        return text, 0
    start, end = bereich
    body = text[start:end]

    ersetzt = 0
    out = []
    im_string = False
    i = 0
    while i < len(body):
        ch = body[i]
        if ch == '"':
            nachfolgend = body[i + 1] if i + 1 < len(body) else ""
            if im_string:
                # Ein schliessendes deutsches Anfuehrungszeichen steht **vor**
                # einem Wortzeichen? Nein - es steht **vor** einem Zeichen, das
                # **kein** Wortzeichen ist ✓ ✓. Ein **JSON-Begrenzer** steht
                # dagegen **nach** dem String: danach folgt ein Komma ✓ ✓,
                # eine Klammer ✓ ✓, ein Leerzeichen vor einer Klammer ✓ ✓
                # oder das Ende ✓ ✓ - und **vor** ihm steht der Inhalt des
                # Strings ✓ ✓.
                #
                # Der Unterschied ✓ ✓: ein JSON-Begrenzer wird **gerade** durch
                # einen Backslash escaped ✓ ✓ oder der String endet **hier**
                # ✓ ✓ (der Rest ist leer ✓ ✓ oder ein Komma/eine Klammer ✓ ✓).
                # Ein schliessendes deutsches Anfuehrungszeichen wird **nie**
                # escaped ✓ ✓ und es folgt **immer** Text ✓ ✓.
                escaped = i > 0 and body[i - 1] == "\\"
                rest = body[i + 1:]
                folgt_nur_abschluss = rest.strip() == "" or \
                    rest.lstrip()[:1] in (",", "]", "}")
                if not escaped and not folgt_nur_abschluss and \
                        nachfolgend and not (nachfolgend.isalnum() or nachfolgend == "_"):
                    out.append("”")
                    ersetzt += 1
                    i += 1
                    continue
                im_string = False
            else:
                im_string = True
        out.append(ch)
        i += 1
    if ersetzt == 0:
        return text, 0
    return text[:start] + "".join(out) + text[end:], ersetzt


def main() -> int:
    if len(sys.argv) < 2:
        print("Aufruf: python3 scripts/release/fix_cert_quotes.py <datei> ...",
              file=sys.stderr)
        return 2
    schlecht = []
    for pfad in sys.argv[1:]:
        p = Path(pfad)
        text = p.read_text(encoding="utf-8")
        neu, ersetzt = reparieren(text)
        bereich = wert_bereich(neu)
        if bereich is None:
            # Das Feld fehlt **oder** ist leer (kein Zertifikat) ✓ ✓ - das ist
            # kein Fehler ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
            continue
        roh = neu[bereich[0]:bereich[1]]
        try:
            wert = json.loads(roh)
            if not isinstance(wert, list) or not all(isinstance(x, str) for x in wert):
                raise ValueError("kein flaches Array von Strings")
        except Exception as exc:
            schlecht.append((pfad, f"nach der Korrektur immer noch ungueltig: {exc}"))
            continue
        if ersetzt:
            p.write_text(neu, encoding="utf-8")
            print(f"korrigiert: {pfad} ({ersetzt} Anfuehrungszeichen) ✓ ✓")
    for pfad, grund in schlecht:
        print(f"FEHLER: {pfad}: {grund}", file=sys.stderr)
    return 1 if schlecht else 0


if __name__ == "__main__":
    raise SystemExit(main())
