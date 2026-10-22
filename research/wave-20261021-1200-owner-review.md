# Owner-Review — Recherchewelle 21.10.2026, 12:00 UTC

Abschluss der inhaltlichen Prüfung um 12:17 UTC. Die drei bundesland-disjunkten Recherche-Shards (je 20 Slugs) meldeten ihre Arbeit als abgeschlossen; ihre Ergebnisse wurden gegen Dossiers und ausgewählte Primärquellen geprüft. Auswahl und vollständiger Scope stehen in [wave-20261021-1200-scope.md](wave-20261021-1200-scope.md).

## Dossier- und Overwrite-Gate

- Die read-only-Abfrage bestätigt aktuell **60/60** Scope-Händler weiterhin im Pool (`website='' OR street=''`); kein Slug fehlt, alle 60 sind eindeutig und die Shards bleiben bundesland-disjunkt.
- Alle 60 Dossiers enthalten einen neuen Eintrag vom 21.10.2026 mit `Recherche`-Datumspräfix, Quellenmarker und mindestens einer URL. Alle vorherigen Timeline-Zeilen sind in jedem Dossier bytegenau und in ursprünglicher Reihenfolge erhalten.
- YAML-Frontmatter ist parsebar; Status-, Trader-Type-, Bundesland- und Website-Statuswerte sowie Website-URL-Formen und nichtleere Drop-off-/Pickup-JSON-Werte sind gültig. Von den strukturierten Feldern wurden ausschließlich zwei Statuswerte geändert:
  - `sn-chemnitz-helmut-lutze-schrottentsorgung`: `aktiv` → `pruefung`
  - `sn-kreischa-jurgen-unger`: `aktiv` → `pruefung`
- Beide Downgrades sind mit der aktuell nur aggregierten bzw. nicht auswertbaren kommunalen Beleglage begründet; es wird keine Schließung behauptet. Anschrift, Kontakt und übrige Stammdaten blieben unangetastet.
- Keine `website`-Deep-Links, Koordinaten oder sonstigen strukturierten Felder hinzugefügt/geändert; keine neuen Em-Dashes im Frontmatter. `git diff --check` ist auf allen 60 Scope-Dossiers sauber.

## Quellen-, Preis- und Identitätsprüfung

- **Shard A (RP/SN/BY/HH):** kein belastbarer numerischer Schrottkurs gefunden. Bei den beiden genannten SN-Fällen wurde eine offene Prüfspur statt einer Schließung dokumentiert. Historische Timeline-Belege blieben erhalten.
- **Shard B (HE/NI/ST/BW/BB):** Identitäts-/Standortketten wie ELPRO und Hennies wurden nicht über ähnlich benannte Gesellschaften hinweg zusammengeführt. Eine Wandlitz-/Wallwitz-E-Commerce-Spur wurde nicht als Schrott-Preisangebot ausgegeben. Die MEPAG-Tabelle blieb undatiert und als Betreiberangabe gekennzeichnet.
- **Shard C (NW/TH/SL/MV/BE/HB):** widersprüchliche Identitäts- und Gebührenhinweise blieben offene Fragen; es wurde daraus keine automatische Betreiberzuordnung oder Schließung abgeleitet.
- Direkt gegengeprüfte Betreiberseiten bestätigen die wesentlichen Begrenzungen der Dossiertexte:
  - [MEPAG-Ankaufstabelle](http://www.mepag-recycling.de/ankaufspreise.html): Schrott 0,13 €/kg, Papier 0,11 €/kg, Pappe 0,00 €/kg und transparente Folie 0,10 €/kg bei Selbstanlieferung. Der zusätzliche Folienhinweis „ab 1 to 0,20 EUR/kg“ ist tabellarisch nicht eindeutig; die Seite nennt keinen Monat/Jahres-Preisstand. Daher weder als bestätigter Oktoberkurs noch als Abholpreis ausgegeben; Pappe 0,00 € ist Ankaufvergütung, keine Entsorgungsgebühr.
  - [Schrottjungs-Gewerbeseite](https://schrottjungs.de/gewerbe/) nennt „bis zu“-Richtwerte für Großmengen, nicht garantierte Privat- oder Saarbrücken-Einzelpreise. Die [Saarbrücken-Seite](https://schrottjungs.de/saarbruecken-schrottabholung/) kombiniert allgemeine Gratiswerbung mit 40–80-€-Gebührenhinweisen und einer 300-kg-Ausschluss-/Mindestmengenangabe. Die Dossiernotiz erhält diesen Widerspruch und behauptet keine lokale Annahmestelle.
  - Die [Westerbach-Kandidatenseite](https://www.autoverwertung-frankfurt.info/about/) nennt Leinweber und Stöhr Automobile GmbH & Co. KG; der Seed enthält aber weder Betreibername noch Hausnummer. Die Kandidatenleistung wurde deshalb nicht in die Seed-Stammdaten übertragen.
  - Bei **ELPRO** wurden die in Liquidation geführte HRB-205923-Gesellschaft und der aktuelle Auftritt der ELPRO Elektro Recycling GmbH getrennt gehalten ([Registerspiegel](https://www.northdata.de/ELPRO%20Umweltservice%20GmbH,%20Braunschweig/HRB%20205923), [Betreiberseite](https://elpro-gmbh.de/privatannahme/)).
  - Beim **Rödelsee-Standort** wurde die aktive KNETTENBRECH + GURDULIC Franken GmbH & Co. KG nicht mit der erloschenen Knettenbrech + Wagner Metallhandel GmbH gleichgesetzt ([Standortseite](https://www.knettenbrech-gurdulic.de/unternehmen/standorte/wuerzburg-/-kitzingen), [HRB 14093](https://www.northdata.de/Knettenbrech%20+%20Wagner%20Metallhandel%20GmbH,%20R%C3%B6delsee/Amtsgericht%20W%C3%BCrzburg%20HRB%2014093)).
- Verzeichnis- und Suchindexdaten sind in den Dossiers als Leads gekennzeichnet; nicht auffindbare oder nicht abrufbare Seiten werden nicht als Negativbeweis gewertet. Ankauf, Verkauf und Gebühren bleiben getrennt.

## Seed-Test und Veröffentlichung

- Das Seed-Testgate ist **nicht ausgeführt**: Cargo ist über `/root/.cargo/bin` (Cargo 1.100.0-nightly) verfügbar, aber das Dateisystem hat nur **1.067.671.552 Bytes (~1.018,5 MiB)** frei und liegt damit rund **181,5 MiB unter** dem dokumentierten Minimum von 1.200 MiB. Der gemeinsame Working Tree enthält umfangreiche unabhängige Änderungen; dessen Artefakte werden nicht gelöscht oder bereinigt.
- Daher kein `cargo test -p schrott-mcp-ingestion seed`, kein Commit, Push oder Redeploy. Die inhaltliche Dossierprüfung ist abgeschlossen; Veröffentlichung bleibt bis zu ausreichendem Speicher und bestandenem Seed-Test gesperrt.
- Die Produktionsdatenbanken wurden während Auswahl und Nachprüfung ausschließlich read-only verwendet. Kein Produktionsschreibzugriff, kein Code-/Koordinaten-Edit.
