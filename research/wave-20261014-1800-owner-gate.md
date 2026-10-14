# 18:00-Welle 14.10.2026 — gemeinsames Owner-Gate

## Scope und tatsächliche Prüfung

Alle drei Background-Shards abgeschlossen, drei Berichte vollständig gelesen.
Alle60 neuen Überblicke/Timeline-Blöcke separat extrahiert und vollständig
gelesen (`/tmp/opencode/wave6-owner-new-text.txt`). Auswahl aus frischer
read-only Produktionsabfrage,20 je bundesland-disjunkter Gruppe; Manifest:
`/tmp/opencode/wave-20261014-1800-selection.json`, Ausgangs-HEAD0e9f2268.
Scope:60 bestehende Dossiers,3 Shardberichte,Startbericht und dieses Gate.
Keine fremde Arbeit, neue Handler, Rust-GEO oder Produktionswrites.

## Eigene Owner-Quellenabrufe / Entscheidungen

- Mittelrhein: Impressum und kostenlose Altfahrzeugannahme tatsächlich
  direkt gelesen; HRA12948-Registerprofil/Firmierung und Anschriftspublikation
  12.01.2010 Bahnweg2/56626 Andernach-Miesenheim selbst gegengeprüft.
  Beide JPG-Originalbescheinigungen selbst heruntergeladen/visuell gelesen:
  ICS003AV26,01.04.2026–30.09.2027,Annahme/Demontage am Bahnweg2,
  örtliches Telefon/Mail. **Behördennummer im neuen Agententext korrigiert:
  ZAES0010000351004**, nicht ZAES001000351004. Hauptüberschrift an belegte
  Vollfirmierung angeglichen. README-Betreiber-Ausnahme akzeptiert; aktiv,
  Name/Adresse/Kontakte/Zeiten/Website und beschränktes dropoff_json freigegeben.
  Kostenlose Annahme Alt-/Unfallfahrzeuge nicht als Auszahlung/Gratisabholung
  oder allgemeine Altmetallannahme darstellen. Sonderbedingungen nicht geprüft.
- Loacker: NorthData HRB3724 selbst gelesen,Name/Adresse An der Remise20,
  66424,Metallhandelsgegenstand und Umfirmierung2018. Betreiberseite ist
  dynamisch: normaler Textabruf fast leer, tatsächliches Roh-HTML enthält
  standortspezifische Metadaten/JSON und SRP→Loacker2018-Aussage, selbst gelesen.
  Adresse/Beschreibung freigegeben, keine pauschale deutsche Betreiber-Ausnahme
  aus österreichischem Gruppenimpressum, kein Kontakt-/Zeitfill.
- TITAL: Howmet-Deutschlandseite bis Bestwig selbst gelesen; HRB8889-Kongruenz
  bei Kapellenstr.44/59909 und Aluminium-/Titan-Feinguss selbst bestätigt.
  Adresse/Herstellerbeschreibung freigegeben, status pruefung bleibt, keine
  öffentliche Ankauf-/Annahmehofzusage. Historischer HRB3097 nicht gleichgesetzt.
- AsEG: Betreiberimpressum und NorthData HRB24373 selbst gelesen: Weroth,
  In der Mark2,Asbestentsorgung/Schadstoffsanierung/Abbruch/Industriedemontage.
  Typ sonstige/Beschreibung freigegeben; kein Publikumshof/Ankauf,Status bleibt.
- Korn: kommunale Wirtschaftsseite und HRB3319-Profil selbst gelesen,
  Korn GmbH/Fitzendorfer Str.14/97496 kongruent,Altmetall-/Containergegenstand.
  Vollname freigegeben,kein unterstellter Rechtsformwechsel; Sitzmöbel Korn
  ist getrennt. Öffnung/Ankauf nicht daraus ableiten.
- Acht Statusrücknahmen aktiv→pruefung: CS,Eißmann,AH,Schrott and More,
  Kielow,Grundler,Nemitz,Friedenauer Münzenhaus. Alte Historien und neue
  Gegenbelege gelesen; bisherige Belege erfüllen aktuelle Identitäts-/
  Annahmefreigabe nicht. Kein geschlossen,keine Sperre allein als Schließung.
  AH-OHG/GmbH-Kette nicht durch GmbH-Registerbeleg ersetzt, keine Fremddatenfills.

Eigene Abrufartefakte für Loacker,TITAL,AsEG und Mittelrhein:
`/tmp/opencode/wave6-owner-sources/`. Exakte URLs in Dossiers/Shardberichten.
Agentenbefunde (u.a.Wetzel/Göggel-PDF,Ritscher/Dreßler-Register) inhaltlich
gelesen und mit Grenzen akzeptiert, **keine eigenen neuen Abrufe behauptet**.
Historische Löschung nicht Standortschließung, abgelaufenes Zertifikat nicht
heutige Anerkennung, unscharfe Such-/Suggest-Nichtfunde kein Negativregister.
Keine numerische neue Ankaufpreisliste/Handlerquelle gewonnen.

## GEO und Produktionsgrenze

DB nur mode=ro/query_only gegengeprüft: bisher Mittelrhein50.4311/7.4042,
Loacker49.3264/7.3387,TITAL51.3608/8.4008,Korn50.1415/10.6498.
Keine davon neu validiert oder verändert. Neue Seed-Adressen ohne lat/lon
erhalten bisherige Koordinaten gemäß Seedvertrag; Datenfill ist keine
Kartenpunktfreigabe. Mittelrhein/Loacker/TITAL benötigen neue Tor-/Adressprüfung;
TITAL nur Werk,Korn Geschäftsanschrift,nicht automatisch Kundenhof.

## Prompt-zu-Artefakt / Abschlussgates

- Frische60-Auswahl und disjunkte20er: Auswahlmanifest+Startbericht.
- Nachvollziehbare Identität/Preisarten/Quellen/GEO:60 Dossiers+3 Berichte,
  sämtliche neuen Textblöcke gelesen; genaue Overwrites oben geprüft.
- Erhaltene Historie/stabile Slugs: gegen Auswahlhash und Ausgangs-Commit
  alle60 vollständigen alten Timeline-Texte als ununterbrochener Block prüfen.
- Enums/Notenformat/Deep-Links/Emdash/JSON/GEO/geschützte SHA256:
  finaler automatischer Scan+scoped diff-check vor Staging.
- Gemeinsame frische Seedtests nach Owner-Korrektur,danach exakt65 Dateien
  in einem Wellencommit. Agententests sind kein Ersatz für diesen Lauf.
- Push tatsächlich versuchen. Ohne erfolgreiche Veröffentlichung kein
  Release beider Binaries/Redeploy/Prod-Spotclaim. Auth war zuletzt blockiert.
  Lokale Integration und Produktionsabschluss bleiben getrennt.

Abschlussgates tatsächlich bestanden:60 komplette alte Timelines/Slugs,
79 neue bracket-formatierte Noten,13 Dossiers mit den oben geprüften
strukturierten Änderungen,155 Fremd-SHA256 unverändert,keine neuen GEO/
clear_fields,geänderte Enums/JSON/Emdash geprüft,scoped diff-check sauber,
Index vor Staging leer. Quellenlinks in allen neuen Blöcken vorhanden und
inhaltlich mit den Bericht-/Dossiergrenzen geprüft; nicht jeder URL erneut
vom Owner abgerufen. Exakte65-Pfadliste `/tmp/opencode/wave6-final-scope.json`.
Gemeinsame Seedtests nach Owner-Korrekturen:14 bestanden,0 fehlgeschlagen,
647 ausgefiltert. Kein vollständiger Workspace-Lauf dieser Welle behauptet.
