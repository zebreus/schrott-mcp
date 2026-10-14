# Owner-Gate: Recherchewelle 14.10.2026

## Auswahl und Umfang

60 Händler frisch aus dem aktuellen Produktionspool `website='' OR street=''`
zufällig ausgewählt, ausschließlich read-only. Drei bundesland-disjunkte
Shards zu je 20 Dossiers; keine Auswahl aus alten Ergebnislisten. Bestehende
schmutzige Dossiers und die vorherigen Oktober-13-Wellen ausgeschlossen.
Basis: `ad5b883459da2fbbb300695721e7bfc93a303506`.

- [Nordwest A](wave-20261014-a.md): HH/HB/NI/NW/SH.
- [Ost B](wave-20261014-b.md): BB/BE/MV/SN/ST/TH.
- [Süd C](wave-20261014-c.md): BW/BY/HE/RP/SL.

Die Tabellen der drei Berichte nennen die konkreten 60 stabilen Slugs und
verlinken die Dossiers. Lokales Auswahl-/Schutzmanifest:
`/tmp/opencode/wave-20261014-selection.json`; Abschlussaudit:
`/tmp/opencode/wave-20261014-final-audit.json`. Temporäre Dateien können später
entfallen; die Dossiers und Berichte enthalten die dauerhafte Beleglage.

## Wesentliche Ergebnisse und Owner-Entscheidungen

- **MEPAG:** numerische Ankaufsliste separat direkt gelesen. Selbstanlieferung:
  Schrott 0,13, Papier 0,11, Pappe 0,00, transparente Folie 0,10 EUR/kg;
  zusätzliche Folienbemerkung ab 1 t 0,20 EUR/kg. Abholung überwiegend Anfrage,
  keine Pappenabholung. Kein datierter Monat trotz „aktueller Monat“: nicht als
  heutiger Zahlenfeed freigegeben. Identität weiterhin Betreiber-Einzelbeleg.
  Quelle: http://www.mepag-recycling.de/ankaufspreise.html .
- **Uhlmann:** Betreiber-Impressum und Kontaktseite separat gelesen; Stephan
  ist Gestalter, nicht Betreiber. Falschen Altbefund transparent korrigiert.
  Owner normalisiert Website auf den geprüften HTTP-Kontakt-Deep-Link. Kein
  HTTPS-Erfolg, keine neue Adresse oder widersprüchliche Mail freigegeben.
- **SEIK:** Recherche-Agent verwendete diagnostisch einen TLS-Ausnahmeabruf.
  Owner gibt dessen Inhalte nicht als authentisierte Betreiberbelege frei.
  Explizite Korrektur im Dossier; ursprüngliche Notizen bleiben als Historie.
  Keine Hallenser Schließung aus Herten-Hinweisen abgeleitet und keine neue
  TLS-Umgehung durch den Owner.
- **Herbig München:** Gruppen-Karten-JavaScript separat gelesen: Gesellschaft,
  Landsbergerstraße 402 und eigener Münchner Kontakt enthalten. Zertifikats-
  und Registerbefunde stützen die Präzisierung. Halbzeughandel ist kein
  Schrottankauf; `pruefung` und GEO-Grenze bleiben. Keine Bremer Kontakte.
- **Kass:** neuer Betreiberauftritt ohne Bindestrich; Impressum separat
  bestätigt. Fehlender unabhängiger aktueller Gewerbeabgleich bleibt offen;
  keine automatischen Kontakt-/Annahmefills.
- **RRW:** Liquidationsereignis separat abgerufen, keine automatische
  Gleichsetzung von Liquidation und operativer Schließung.
- **PEG, Töpfer, Reukema, Kloppe:** neue Register-/historische amtliche Spuren
  statt pauschaler Such-MISS. Nachfolge, Gegenwartsbetrieb und Seed-Zuordnung
  bleiben bei unzureichenden Quellen ausdrücklich offen.
- **Miersen, Kieler Altmetall, Leon Anders, Hancer:** unbegründetes `aktiv`
  konservativ auf `pruefung` zurückgenommen, nicht auf `geschlossen`.
- Namensvetter, Ortsgegenadressen und Registerketten sind in den Einzel-
  dossiers getrennt; keine erzwungenen Zusammenführungen. Keine Koordinaten
  ergänzt. GEO-Bedarf und Sperren in den drei Berichten dokumentiert.

## Gemeinsames Abschlussgate

- Alle 60 Slugs stabil; sämtliche vorherigen Timeline-Abschnitte jeweils
  vollständig als zusammenhängender Text erhalten; neue datierte Notizen.
- Alle 60 ausgewählten Slugs zum Abschluss noch im read-only Produktionspool.
- Alle 155 bereits vorhandenen fremden Dateien per SHA-256 unverändert.
- Frontmatter-Deltas gemeinsam gesichtet; keine Kontakt-Emdash-Füllung,
  keine neue Domainwurzel nach Uhlmann-Korrektur, keine GEO-Änderung.
- Quellenfamilien und Beleggrenzen in allen drei Berichten geprüft;
  insbesondere Impressum nicht mit zweitem Betreiberbeleg gleichgesetzt.
- `cargo test -p schrott-mcp-ingestion seed`: 14 bestanden, 0 fehlgeschlagen;
  gemeinsamer Korpus inklusive aller Owner-Korrekturen. Codegen/Enum-Gate
  erfolgreich. Log: `/tmp/opencode/wave-20261014-seed-owner.log`.
- `git diff --check`: erfolgreich. Nur die ausgewählten 60 Dossiers und
  vier Wave-Berichte gehören zum gemeinsamen Commit.

## Veröffentlichung und Produktion

Lokale Recherche und Prüfung abgeschlossen. `gh auth status` bestätigt
weiterhin keine GitHub-Anmeldung. Push, Redeploy beider Binaries und
anschließende produktive Änderungs-Spot-Checks bleiben offen und werden nicht
als erfolgreich behauptet. Kein erzwungener Produktions-Ingest und keine
direkten Produktionsdatenbank-Schreibzugriffe.

Read-only Abschlussbeobachtung: Dienst `active`, 3881 Händler, 54 Materialien,
2426 aktuelle Preise. Dies belegt Gesundheit des bisherigen Deployments,
nicht Veröffentlichung dieser Welle. Vor Deployment müssen auch die separat
vorhandenen 155 fremden Änderungen aus dem Build ausgeschlossen bzw. zuvor
eigenständig geprüft werden; ein Build des schmutzigen Gesamtbaums wäre kein
kontrollierter Release dieses Commits.
