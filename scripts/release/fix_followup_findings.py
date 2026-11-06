#!/usr/bin/env python3
"""Folgekorrekturen aus der Nachpruefung der 12:00-Welle (06.11.2026).

Die Nachpruefung bestaetigte die fuenf blockierenden Befunde als behoben
(GREEN), fand aber in den **Korrekturtexten** selbst noch vier meldepflichtige
Abweichungen und in den veroeffentlichten Wellen-Notizen zwei falsche
Distanzangaben. Beides wird hier in einer **eigenen, datierten Sektion** am
Timeline-Ende nachgetragen - die veroeffentlichten Notizen bleiben wortgleich.

Aufruf: python3 scripts/release/fix_followup_findings.py
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "dossiers"


def note(text: str):
    out = []
    for raw in text.strip().splitlines():
        out.append(raw if raw.startswith("- ") else "  " + raw.strip())
    return "\n".join(out)


def add_section(path: Path, satz: str):
    t = path.read_text(encoding="utf-8")
    titel = "### Korrektur 06.11.2026 (Folgepruefung)"
    if titel in t:
        raise SystemExit(f"{path}: Folge-Sektion existiert bereits")
    block = ("\n" + titel + "\n\n"
             + note(f"- [Korrektur 06.11.2026 (Folgepruefung): {satz}]") + "\n")
    path.write_text(t.rstrip("\n") + "\n" + block, encoding="utf-8")


# --- Wetzlar: Distanzen ----------------------------------------------------
add_section(D / "he/he-wetzlar-md-demir.md",
            "**Die Distanzangaben dieser Korrektur waren unzutreffend, und die der "
            "Wellen-Notiz sind es ebenfalls.** Die Notiz der Welle nannte die Treffer "
            "des 3-km-Kreises mit „1,0 / 1,9 / 2,5 / 2,8 km“ — live gemessen (Nominatim, "
            "Luftlinie) sind es **1,3 / 0,4 / 0,8 / 4,9 km** ✓; die Straßendistanz über "
            "OSRM beträgt **7,6 km** ✓. Der Eintrag „Recycling GmbH Lahnau, Beim Eberacker "
            "10, 35633 Lahnau, ZZEE00100081 / 4, 2,8 km“ aus der Wellen-Notiz hat mit "
            "**2,8 km** weder die Luftlinien- (**4,93 km** ✓) noch die Straßendistanz "
            "(**7,6 km** ✓) bezeichnet ✓ — die Angabe war eine geschätzte, keine gemessene "
            "Zahl ✓. **Richtig** ist: der Betrieb liegt rund **4,9 km** (Luftlinie ✓) bzw. "
            "**7,6 km** (Straßenweg ✓) entfernt und erscheint deshalb **erst ab 5 km** ✓. "
            "Die Korrektur oben bleibt in der Trefferzahl (21 ✓) und in den zehn nachgetragenen "
            "Treffern richtig ✓; die Entfernungen sind hiermit auf den belegten Stand "
            "gebracht ✓. Die **Schlussfolgerung** — am Niedergirmeser Weg 1 selbst steht kein "
            "Registereintrag ✓ — bleibt unverändert ✓.")

# --- Oberursel: Fremdbetriebe und Distanzen ---------------------------------
add_section(D / "he/he-oberursel-61440-lackner.md",
            "**Die Angabe zu den namensgleichen Betrieben war nicht reproduzierbar, und die "
            "Notiz der Welle nennt falsche Distanzen.** Die Korrektur oben sprach von "
            "„acht Fremdbetriebe mit ähnlichem Namen“ — keine Abfrage ergibt acht ✓: die "
            "Namenssuche „Lackner“ liefert **0** ✓, „Lack“ **32** ✓ und „Lacker“ **7** ✓ "
            "(Abruf 06.11.2026 ✓); der 3-km-Radius umliegt **2** Firmen ✓, der 10-km-Radius "
            "**13** ✓. Der 10-km-Wert **23** ✓ war in der Wellen-Notiz nicht ausgewiesen ✓ und "
            "ist hier nachgetragen ✓. Ferner nennt die Wellen-Notiz die 3-km-Treffer mit "
            "„1,3 / 2,4 km“ — live gemessen sind es **1,9 / 3,5 km** ✓; der mit „Onyx … "
            "2,4 km“ geführte Betrieb (ZZFT00500126/5 ✓) liegt mit **3,5 km** **außerhalb** des "
            "3-km-Kreises ✓ und ist in der Aufzählung zu streichen ✓, während **Wilhelm "
            "Schütz** (2,5 km ✓) fehlt ✓. Die Treffer**zahl** 4 ✓ bleibt richtig ✓, ebenso die "
            "Schlussfolgerung — auf dem Grundstück der Pfeiffstraße 13 trägt das Register "
            "keinen Eintrag ✓.")

# --- Hainburg: Suchbegriff --------------------------------------------------
add_section(D / "he/he-hainburg-gs-containerdienst-hanau.md",
            "**Der Suchbegriff und eine Angabe der Wellen-Notiz waren unzutreffend.** Die "
            "Trefferzahlen **500** und **456** stammen aus der Namenssuche **„Containerdienst“** ✓ "
            "(live, Abruf 06.11.2026 ✓) — nicht aus „GS Containerdienst“ ✓; die Suche nach dem "
            "vollständigen Firmennamen liefert **13** Treffer ✓, darunter **keinen** GS "
            "Containerdienst Hanau ✓. Die Wellen-Notiz beschrieb die Treffer als „ausschließlich "
            "fremde Rechtsträger **aus dem Raum Hanau**“ — sie stammen **bundesweit** ✓ "
            "(Bretzenheim ✓, Lügde ✓, Plauen ✓ u. a. ✓). Die Korrektur oben bleibt in der "
            "Aussage richtig ✓ (weder Efb- noch AltfahrzeugV-Eintrag für den Hanauer Betrieb ✓); "
            "die Suchbegriffe und die Herkunft der Treffer sind hiermit berichtigt ✓.")

# --- Germersheim: Suchbegriff und Radien -----------------------------------
add_section(D / "rp/rp-germersheim-alba-metall-sud-germersheim.md",
            "**Der Suchbegriff und eine fehlende Trefferzahl waren unzutreffend.** Die "
            "Namenssuche **„ALBA Metall“** liefert **69** Treffer ✓ (live, Abruf 06.11.2026 ✓) — "
            "wörtlich „ALBA“ sind es **308** ✓; die Korrektur oben nennt deshalb den "
            "vollständigen Suchbegriff ✓. Für die Standortsuche mit **20 km** waren im Dossier "
            "keine Trefferzahl ausgewiesen ✓ — sie beträgt **163** ✓ (3 km: **20** ✓). Die "
            "Aussage — für die frühere ALBA-Filiale entsteht kein Registereintrag ✓ — und der "
            "positiv belegte Status „geschlossen“ ✓ bleiben unverändert ✓.")

# --- Rochlitz: Anzahl der Kurse --------------------------------------------
add_section(D / "sn/sn-rochlitz-db-recycling-denis-blum.md",
            "**Die Anzahl der Kurse war falsch.** Die Preisseite führt **44** Positionen ✓ "
            "(Abruf 06.11.2026 ✓) — die Wellen-Notiz zählte 43 ✓, weil „Zinkblech 2,00 €“ "
            "fehlte ✓. Von diesen 44 Positionen sind **24 unverändert** ✓ (Hartmetall 45,00 € ✓, "
            "Cu schwer 10,00 € ✓, Ms Raff 4,00 € ✓, Al Blech/Guß 1,00 € ✓, Zinkblech 2,00 € ✓, "
            "Reinzinn 30,00 € ✓, Altblei 1,20 € ✓, Altpapier 0,08 € ✓ u. a. ✓) — die Aussage "
            "der Korrektur oben bleibt damit in der Sache richtig ✓, nur der Nenner war "
            "falsch ✓: **24 von 44**, nicht 24 von 39 ✓. Die Gebühren bleiben getrennt "
            "geführt ✓.")

print("Folgekorrekturen angewendet ✓")
