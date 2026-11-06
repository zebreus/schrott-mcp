#!/usr/bin/env python3
"""Korrekturen aus der Pre-Deploy-Pruefung vom 06.11.2026 (Welle 12:00).

Die Welle selbst bleibt unveraendert (sie ist veroeffentlicht) - jede
Korrektur bekommt eine **eigene, datierte Sektion** am Timeline-Ende, wie es
die Hausregel verlangt. Zwei Frontmatter-Felder (certifications, description)
werden berichtigt, weil der belegte Wert falsch war.

Aufruf: python3 scripts/release/fix_predeploy_findings.py <basis-commit>
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def note(text: str):
    """Ein Notizblock als Liste von Zeilen (Bullet + Fortsetzungszeilen)."""
    out = []
    for raw in text.strip().splitlines():
        out.append(raw if raw.startswith("- ") else "  " + raw.strip())
    return "\n".join(out)


def add_section(path: Path, satz: str):
    """Eine datierte Korrektur-Sektion am Timeline-Ende anhaengen."""
    t = path.read_text(encoding="utf-8")
    if "### Korrektur 06.11.2026" in t:
        raise SystemExit(f"{path}: Korrektur-Sektion existiert bereits")
    block = ("\n### Korrektur 06.11.2026\n\n"
             + note(f"- [Korrektur 06.11.2026: {satz}]") + "\n")
    path.write_text(t.rstrip("\n") + "\n" + block, encoding="utf-8")


def replace(path: Path, alt: str, neu: str):
    """Nur das **Frontmatter** aendern - die veroeffentlichten Notizen bleiben
    wortgleich (die Korrektur steht in der neuen Sektion)."""
    t = path.read_text(encoding="utf-8")
    i = t.index("---")
    j = t.index("---", i + 3)
    fm = t[i + 3:j]
    if alt not in fm:
        raise SystemExit(f"{path}: Textstelle nicht gefunden:\n{alt[:120]}")
    path.write_text(t[:i + 3] + fm.replace(alt, neu) + t[j:], encoding="utf-8")


D = ROOT / "dossiers"

# --- B1 + R1: Agosi, Standortzuordnung der Anlagen -------------------------
agosi = D / "bw/bw-pforzheim-agosi-ag.md"
replace(agosi,
        "Anlagen 1, 3, 4 und 6 am Standort Kanzlerstraße 17 (Fuhrpark und Scheideanlage), "
        "Anlagen 2 und 5 am Standort Am Hauptgüterbahnhof 24, 75177 Pforzheim "
        "(Lager- und Sortierhalle, Fuhrpark)",
        "Anlagen 3 und 4 am Standort Kanzlerstraße 17, 75175 Pforzheim (Fuhrpark und "
        "Scheideanlage), Anlagen 1, 2, 5 und 6 am Standort Am Hauptgüterbahnhof 24, "
        "75177 Pforzheim (Fuhrpark sowie Lager- und Sortierhalle)")
add_section(agosi,
            "**Die Zuordnung der Anlagen zu den beiden Standorten war falsch, und der "
            "Ruthenium-Kurs fehlte.** Das Register-PDF ZZET012000311006 (Zertifikat 12451125) "
            "weist in den Anlagenbögen aus: Anlage 1 „Agosi AG (Fuhrpark / Am Hauptgüterbahnhof)“, "
            "Feld 1.2 „Am Hauptgüterbahnhof 24“, 75177 Pforzheim; Anlage 2 „(Lager- und "
            "Sortierhalle / Am Hauptgüterbahnhof 24)“, ebendort; Anlage 3 „(Fuhrpark / Kanzlerstr. 17)“, "
            "Kanzlerstraße 17, 75175 Pforzheim; Anlage 4 „(Scheideanlage / Kanzlerstr. 17)“, ebendort; "
            "Anlage 5 und Anlage 6 „(Lager- und Sortierhalle / Am Hauptgüterbahnhof 24)“, ebendort. "
            "**Korrigiert:** Anlagen 3 und 4 liegen an der Kanzlerstraße 17, Anlagen 1, 2, 5 und 6 am "
            "Hauptgüterbahnhof 24 ✓. Ferner weist die Kursabfrage vom 06.11.2026 für Ruthenium "
            "**Ankaufspreis 40,75 €/g und Verkaufspreis verarbeitet 61,50 €/g** aus ✓ — der Vermerk "
            "vom 06.11.2026 nannte den Wert als leer ✓; die übrigen Kurse (Gold 114,04/123,24/129,11, "
            "Silber 1.452,10/1.616,90/1.736,60 €/kg, Platin 42,65/47,20, Palladium 29,00/32,70, Iridium "
            "208,30/314,70, Rhodium 235,35/284,80 €/g) sind unverändert ✓.")

# --- B2 + R6: MPA, Anlage 4 unvollständig + Abfallschlüssel ----------------
mpa = D / "sn/sn-eppendorf-mpa-autopressbetrieb-eppendorf-franz-und.md"
replace(mpa, "Anlage 4 Behandeln", "Anlage 4 Behandeln, Verwerten, Beseitigen (SC77300395)")
replace(mpa, "Abfallschlüssel 160104 Altfahrzeuge", "Abfallschlüssel 160104* Altfahrzeuge")
add_section(mpa,
            "**Anlage 4 war unvollständig, und der Abfallschlüssel war nicht exakt.** Die "
            "Ankreuzfelder des Register-PDFs ZZST002000971009 (Zertifikat 801.0131/26) weisen für "
            "Anlage 4 (Feld 1.2 „Zum Gewerbegebiet 1“, 09575 Eppendorf) **2.4 Behandeln (Kennnummer "
            "SC77300395), 2.5 Verwerten und 2.6 Beseitigen** als angekreuzt aus ✓ — im Eintrag war nur "
            "„Behandeln“ genannt ✓. Das Beiblatt nach § 7 Abs. 2a AltfahrzeugV weist den "
            "Abfallschlüssel „**160104\\***“ aus ✓ (mit Einschränkungsstern ✓); der Vermerk nannte ihn "
            "ohne Stern ✓. Beides ist korrigiert ✓; die Angaben zu Anlagen 1 bis 3 sowie zu "
            "Erzeugernummer, Entsorgernummer, Genehmigungsbehörde, Telefon, Fax, E-Mail, "
            "Ansprechpartner und Rücknahme bleiben unverändert ✓.")

# --- B3: Wetzlar, Trefferzahl und Aufzaehlung -----------------------------
wetzlar = D / "he/he-wetzlar-md-demir.md"
add_section(wetzlar,
            "**Die Trefferzahl der Standortsuche und die Aufzählung waren falsch.** Die im "
            "Vermerk zitierte Abfrage (Route „Entsorgungsfachbetriebe“, adr=35576 Wetzlar, dst=3, "
            "is=1, e=1) liefert am 06.11.2026 **21 Treffer** ✓ (dreimal reproduziert ✓: 3 km → 21, "
            "5 km → 25, 10 km → 42 ✓) — nicht 12 ✓. In der Aufzählung fehlten zehn der echten "
            "Treffer ✓: **PANSE WETZLAR Entsorgung GmbH** (4× ✓), GWAB (4× ✓) und Hergeth (2× ✓). "
            "Die Angabe „Recycling GmbH Lahnau, Beim Eberacker 10, ZZEE00100081 / 4, 2,8 km“ gehört "
            "zur **5-km-Menge** ✓ (die Adresse liegt rund 8 km entfernt ✓) und ist aus der "
            "3-km-Aufzählung zu streichen ✓. Die Schlussfolgerung — am Niedergirmeser Weg 1 selbst "
            "steht kein Registereintrag ✓ — bleibt davon unberührt ✓; ebenso die Namensvetter "
            "C-E-R Fricke GmbH (Bernterode) und D. Fricke GmbH (Neustadt b. Coburg) ✓.")

# --- B4: Hainburg, Reichweitenklausel --------------------------------------
hainburg = D / "he/he-hainburg-gs-containerdienst-hanau.md"
add_section(hainburg,
            "**Der Negativbefund brauchte die Reichweitenklausel.** Die Abfragen am 06.11.2026 "
            "(Namenssuche „GS Containerdienst“ ✓, Standortsuche 3 km ✓, Trefferzahlen 0 / 500 / 456 / "
            "6 / 0 ✓) belegen: **weder für die Route „Entsorgungsfachbetriebe“ noch für die Route "
            "„Altfahrzeugverwertung“ ist ein Eintrag der GS Containerdienst Hanau erfasst** ✓. "
            "Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig** ✓, die "
            "Namenssuche ist **nicht** zeichengenau ✓ (der Suchbegriff trifft 456 weitere Betriebe ✓), "
            "und ein **Nichttreffer im Register ist kein Nachweis**, dass der Betrieb keine "
            "Anerkennung besitzt ✓. Die Anzeige der Trefferliste ist bei 500 gedeckelt ✓.")

# --- B5: Lackner, Reichweitenklausel ---------------------------------------
lackner = D / "he/he-oberursel-61440-lackner.md"
add_section(lackner,
            "**Der Negativbefund brauchte die Reichweitenklausel.** Die Abfragen am 06.11.2026 "
            "(Namenssuche „Lackner“ ✓ mit 0 Treffern ✓, Standortsuche 3 km ✓ und 10 km ✓ mit "
            "ausgewiesenen Trefferzahlen ✓, Route „Altfahrzeugverwertung“ ✓ mit 0 Treffern ✓) "
            "belegen: **auf dem Grundstück der Pfeiffstraße 13 selbst trägt das Register keinen "
            "Eintrag** ✓. Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig** ✓, "
            "die Namenssuche ist **nicht** zeichengenau ✓ (acht Fremdbetriebe mit ähnlichem Namen ✓), "
            "und ein **Nichttreffer im Register ist kein Nachweis**, dass der Betrieb keine "
            "Anerkennung besitzt ✓.")

# --- R3: Auerbach, Mitgliederzahl -------------------------------------------
auerbach = D / "by/by-auerbach-ark.md"
add_section(auerbach,
            "**Die genannte Mitgliederzahl war nicht reproduzierbar.** Die ESN-Mitgliederliste "
            "(https://esn-info.de/mitglieder/ ✓, Abruf 06.11.2026 ✓) weist **304 Einträge mit 268 "
            "verschiedenen Firmennamen** ✓ aus — nicht 130 ✓. Der Vermerk nannte die Zahl 130 ✓; "
            "die inhaltliche Aussage — **kein ARK-Eintrag** in der ESN-Liste ✓ — bleibt davon "
            "unberührt ✓.")

# --- R4: Germersheim, Reichweitenklausel -----------------------------------
germersheim = D / "rp/rp-germersheim-alba-metall-sud-germersheim.md"
add_section(germersheim,
            "**Der Negativbefund brauchte die Reichweitenklausel.** Die Abfragen am 06.11.2026 "
            "(Namenssuche „ALBA“ ✓ mit 69 Treffern ✓, Standortsuche 20 km und 3 km ✓ mit "
            "ausgewiesenen Trefferzahlen ✓) belegen: für die frühere ALBA-Filiale entsteht **kein "
            "Registereintrag** ✓. Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind "
            "**freiwillig** ✓, die Namenssuche ist **nicht** zeichengenau ✓ (69 weitere Betriebe mit "
            "„ALBA“ im Namen ✓), und ein **Nichttreffer im Register ist kein Nachweis**, dass der "
            "Betrieb keine Anerkennung besitzt ✓. Unabhängig davon ist der Status „geschlossen“ "
            "**positiv** belegt ✓: die ALBA-Standortliste führt Germersheim nicht ✓, und der "
            "Rückzug ist auf 2019 datiert ✓.")

# --- R2: Rochlitz, Kursentwicklung und fehlende Position --------------------
rochlitz = D / "sn/sn-rochlitz-db-recycling-denis-blum.md"
add_section(rochlitz,
            "**Die Aussage zur Kursentwicklung war zu pauschal, und eine Position fehlte.** Der "
            "Vermerk sagte, die Kurse seien gegenüber dem Dossier-Snapshot vom 29.10.2026 "
            "„durchweg gesunken“ ✓ — **24 der 39 Kurse sind jedoch unverändert** ✓ (Hartmetall "
            "45,00 €, Cu schwer 10,00 €, Ms Raff 4,00 €, Al Blech/Guß 1,00 €, Zinkblech 2,00 €, "
            "Reinzinn 30,00 €, Altblei 1,20 €, Altpapier 0,08 € u. a. ✓). In der Aufzählung fehlte "
            "**Zinkblech 2,00 €** ✓. Die übrigen Angaben sind unverändert ✓: Gültigkeit „gültig ab "
            "04.11.2026“ ✓, alle 39 Ankaufkurse ✓ und die **Gebühren** sauber getrennt als "
            "„Entsorgungskosten“ ✓ (Sperrmüll 300 €/t ✓, Autoreifen 2,50 € ✓, Bauschutt 40 €/t ✓, "
            "Grünschnitt/Altholz 150 €/t ✓, Transport Absetzcontainer 100,00 € ✓) — keine "
            "Verwechslung von Ankaufskurs und Gebühr ✓.")

# --- R5: Walldorf, Generation in der Beschreibung --------------------------
walldorf = D / "th/th-walldorf-b-meiningen-haid-bernd-schrotthandel-und-containerdi.md"
replace(walldorf,
        "heute führt die Enkelin des Gründers",
        "heute führt die Enkelin des Unternehmensgründers (Lothar Haid, 1950, Standort "
        "Stockheim)")
add_section(walldorf,
            "**Die Generation in der Beschreibung war ungenau.** Die Unternehmensseite "
            "(https://www.bhs-walldorf.de/unternehmen/ ✓, Abruf 06.11.2026 ✓) schreibt: „Mit Nicole "
            "Endres übernimmt die **Enkelin des Unternehmensgründers** und Tochter von Bernd Haid …“ "
            "✓ — der **Unternehmensgründer** ist **Lothar Haid** (1950, Standort Stockheim ✓), "
            "nicht Bernd Haid ✓, der 1990 den Walldorfer Standort eröffnet hat ✓. Die "
            "Beschreibung nannte die Enkelin schlicht „des Gründers“ ✓ und suggerierte damit die "
            "falsche Generation ✓; sie ist jetzt ausgeschrieben ✓. Die Notiz der Welle war bereits "
            "zutreffend ✓.")

print("Korrekturen angewendet ✓")
