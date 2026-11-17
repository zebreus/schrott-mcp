#!/usr/bin/env python3
"""Traegt die Nachpruefung eines Shards als eigene datierte Sektion ein.

Die Shard-Branches (`wellec-s2`, `wellec-s3`) haben veroeffentlichte Notizen
**in place** erweitert: neue Registerabfragen als Positivkontrolle, verstaerkte
Negativbefunde und Klarstellungen zur Namensgleichheit. Das verletzt die
Hausregel. Die Aussagen werden deshalb als **eigene datierte Sektion am
Timeline-Ende** uebernommen - ohne Aussagen ueber den Frontmatter-Stand, weil
der inzwischen ein anderer ist (die spaeteren Wellen haben Felder geleert).

Die Notiztexte stehen hier bewusst von Hand: sie nennen die gemessenen
Registerzahlen und die belegten Widersprueche, nichts sonst.

Aufruf:  python3 scripts/release/merge_shard_review.py --date 17.11.2026
"""
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BRANCHES = ("wellec-s2", "wellec-s3")

NOTES = {
    "nw-bergisch-gladbach-krau": (
        "Die Namensgleichheit \"Krauß\" allein verbindet den Seed weiterhin **nicht** sicher mit "
        "dem Containerdienst. Die Nachpruefung vom 17.11.2026 hat die Namenssuche im amtlichen "
        "Fachbetrieberegister um die Schreibweise **ohne** Umlaut und um den vollen Vornamen "
        "erweitert; die Standortabfrage mit 10 km um 51469 Bergisch Gladbach "
        "(7.1491313/51.1264818) liefert **40** Efb-Treffer und **6** Altfahrzeug-Treffer, "
        "**kein** Treffer nennt Krauß oder den Containerdienst. Die Zuordnung bleibt deshalb "
        "**offen**."
    ),
    "nw-bochum-wattenscheid-schrotthandler-plus": (
        "Die Plus-Seiten nennen **eigene** Kontaktdaten (0163 4694319, info@schrotthandel.nrw) "
        "und **eigene** Öffnungszeiten (Mo-Fr 07:00-21:00, Sa 09:00-21:00) und sind damit eine "
        "**eigene** Marke **mit** demselben Namen und keine bloße Zweitseite desselben "
        "Anbieters. Die beiden Links stehen auf der Plus-Startseite unter der Überschrift "
        "\"Langfristige Partnerschaften\"; der Fußbeleg \"Design by Horany\" ist der "
        "**Webdesigner** und kein Betreiberhinweis. Die Servicezeiten der Plus-Seiten selbst "
        "(Mo-Fr 07:00-19:00, Sa 09:00-19:00, So Ruhetag) bleiben unverändert, und die "
        "Bochum-Leistungsseite nennt Wattenscheid weiterhin ausdrücklich als bedienten Stadtteil."
    ),
    "nw-bottrop-puffahrt": (
        "Das Angebot nennt einen weiten Regionenumfang (Sauerland, Märkischer Kreis, "
        "Iserlohn, Hemer, Werdohl, Neuenrade, Lüdenscheid, Siegen, Hagen, Gevelsberg und "
        "Sprockhövel); ein Beziffern der Kurse findet **nicht** statt. Die Kopfzeile der Seite "
        "verweist auf die Hauptdomain schrottabholung123.de. **Positivkontrolle 17.11.2026:** die "
        "Standortabfrage mit **10 km** um Bottroper Straße 34 (6.9143367/51.5982855) liefert "
        "**66** Efb-Treffer und **3** Altfahrzeug-Treffer; **kein** Treffer nennt Puffahrt."
    ),
    "nw-erkelenz-kb": (
        "**Nachprüfung 17.11.2026 (Verzeichnisfassungen):** Der 11880-Eintrag \"KB Schrott & "
        "Buntmetalle\" wurde am 17.11.2026 vollständig live gelesen: Branche \"Entsorgungsdienst, "
        "Schrotthandel & Containerdienste\" mit sieben Kategorien, Anschrift in **Mönchengladbach**, "
        "nur eine **Mobilnummer** (01515) 9012122, **keine** Öffnungszeiten und **kein** Festnetz. "
        "Der Widerspruch zwischen der Erkelenzer Anschrift und dem Mönchengladbacher Standort "
        "bleibt damit **offen**; eine Wayback-Abfrage liefert **keinen** Snapshot."
    ),
    "nw-essen-schrott-busters": (
        "**Positivkontrolle 17.11.2026:** die Standortabfrage mit **10 km** um die Essener "
        "Anschrift (7.0729867/51.4660509) liefert **180** Efb-Treffer und **10** "
        "Altfahrzeug-Treffer; **kein** Treffer nennt Schrott Busters oder Wojke."
    ),
    "nw-horn-bad-meinberg-neuwald": (
        "**Positivkontrolle 17.11.2026:** die Standortabfrage mit **10 km** um 32805 "
        "Horn-Bad Meinberg (8.9731695/51.8801277) liefert **16** Efb-Treffer und **1** "
        "Altfahrzeug-Treffer (Autoverwertung Reibeholz Inhaber Werner Reibeholz, Karrweg 35, "
        "32825 Blomberg, ZAHS01800146 / 1); **kein** Treffer nennt Neuwald."
    ),
    "nw-lippstadt-mrad": (
        "**Positivkontrolle 17.11.2026:** die Standortabfrage mit **10 km** um 59554 Lippstadt "
        "(8.347194/51.6747074) liefert **21** Efb-Treffer und **2** Altfahrzeug-Treffer (Sicking "
        "Autoverwertung Lippstadt GmbH, Bertramstraße 4, 59557 Lippstadt, ZAES02600005 / 1, und "
        "Heinz Böhmer Kfz- und Abschleppdienst, Herringhauser Str. 84, 59556 Lippstadt, "
        "ZAES02600003 / 1); **kein** Treffer nennt Mrad."
    ),
    "nw-ludinghausen-ash": (
        "Die genannten **500 Kilogramm** sind ein Hinweis auf den Transportaufwand (\"500 "
        "Kilogramm Schrott sind schnell erreicht! Der Transport zum Schrottplatz ist "
        "zeitaufwendig\"), **keine** Mindestbestellmenge. **Verzeichnisstand 17.11.2026:** Der "
        "11880-Eintrag \"ASH Altmetall und Schrotthandel\" trägt den Eintragsstand **10.10.2026** "
        "und die Aktualisierung **10.11.2026**; die Gelbe-Seiten-Suche liefert am selben Tag für "
        "Lüdinghausen einen Eintrag \"**AES Altmetalle**\" an Hans-Böckler-Str. 16, 59348 "
        "Lüdinghausen mit der Rufnummer 02591 8 91 27 29, also eine **dritte** Schreibweise des "
        "Namens bei identischer Anschrift und identischer Rufnummer."
    ),
    "nw-munster-smb-schrott-und-metallhandel": (
        "**Positivkontrolle 17.11.2026:** die Standortabfrage mit **10 km** um 48147 Münster "
        "(7.6173198/52.0061943) liefert **56** Efb-Treffer und **1** Altfahrzeug-Treffer; "
        "**kein** Treffer nennt SMB."
    ),
    "nw-paderborn-peitzmann": (
        "Das EfbV-Originalzertifikat **ZZEE002001862009** des amtlichen Fachbetrieberegisters "
        "wird vom Zertifikatsportal mit Gültigkeit **bis 29.04.2027** geführt; die Anlage nennt "
        "denselben Standort. Die beiden übrigen Dateien (GüKG und Z3140) sind Scans **ohne** "
        "Textebene und deshalb nur über Dateiname und Zertifikatsportal belegt; das "
        "Originalzertifikat bleibt die belastbare Grundlage. Der Zeitraum 07.10.2025 bis 07.01.2027 "
        "aus dem Zertifikats-PDF gehört zur **EnMS/ISO-50001**-Anlage, nicht zum EfbV-Zertifikat."
    ),
    "nw-telgte-putzer": (
        "Der im Verzeichnis genannte Betrieb **Kurz Ebert** ist ein **anderer** Rechtsträger und "
        "wird **nicht** übernommen."
    ),
}

BRANCH_OF = {
    "nw-bergisch-gladbach-krau": "wellec-s2",
    "nw-bochum-wattenscheid-schrotthandler-plus": "wellec-s2",
    "nw-bottrop-puffahrt": "wellec-s2",
    "nw-erkelenz-kb": "wellec-s2",
    "nw-essen-schrott-busters": "wellec-s2",
    "nw-horn-bad-meinberg-neuwald": "wellec-s2",
    "nw-lippstadt-mrad": "wellec-s2",
    "nw-ludinghausen-ash": "wellec-s2",
    "nw-munster-smb-schrott-und-metallhandel": "wellec-s2",
    "nw-paderborn-peitzmann": "wellec-s2",
    "nw-telgte-putzer": "wellec-s2",
}


def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--date", required=True)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    touched = 0
    for slug, text in NOTES.items():
        found = None
        for branch in BRANCHES:
            files = subprocess.run(
                ["git", "diff", "--name-only", f"main..{branch}", "--", "dossiers/"],
                capture_output=True, text=True, check=True).stdout.split()
            for f in files:
                if f.split("/")[-1][:-3] == slug:
                    found = f
                    break
            if found:
                break
        if not found:
            print(f"{slug}: nicht im Branch-Diff gefunden - uebersprungen")
            continue
        path = ROOT / found
        cur = path.read_text(encoding="utf-8")
        note = f"- [Korrektur {args.date}: {text}]"
        if note in cur:
            print(f"{slug}: bereits vorhanden")
            continue
        section = f"\n### Korrektur {args.date} (Nachprüfung des Shards {BRANCH_OF.get(slug,'')})\n\n{note}\n"
        if args.dry_run:
            print(f"{slug}: 1 Notiz")
            touched += 1
            continue
        path.write_text(cur.rstrip("\n") + "\n" + section, encoding="utf-8")
        touched += 1
    print(f"Dossiers angefasst: {touched}")


if __name__ == "__main__":
    main()
