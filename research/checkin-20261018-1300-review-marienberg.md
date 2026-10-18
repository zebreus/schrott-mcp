# Marienberg: Review der pauschalen Kundenannahmebehauptung

Stand: 18.10.2026; eigener Live-Abruf im Rahmen des Check-ins 13:00. Gegenstand ausschließlich `dropoff_json` für `sn-marienberg-scholz-recycling-betrieb-marienberg`.

## Ergebnis / Owner-Empfehlung

**Kuratiertes bedingtes JSON ist gegenüber unverändertem `{"allowed":true}` oder einem vollständigen Clear vorzuziehen.** Es gibt eine heute direkt gelesene Betreiberregel, die Selbstanlieferung ausdrücklich **an allen eigenen Standorten** und sowohl private als auch gewerbliche Kunden erfasst. Marienberg ist im aktuellen Betreiberfinder ausdrücklich eine Betriebsstätte der Scholz Recycling GmbH. Das ist mehr als eine bloße allgemeine Leistungswerbung, aber **keine separat veröffentlichte Marienberger Kundenannahmeordnung**. Herkunft und Reichweite müssen im kuratierten Text sichtbar bleiben.

Der positive Kern „Selbstanlieferung grundsätzlich möglich“ ist damit aktuell neu belegbar; die **bedingungslose Darstellung allein mit `allowed:true` ist als kuratiertes Endergebnis zu grob**. Sie unterschlägt Unterlagen, Sicherheitsregeln, Materialprüfung und Ausschlüsse. Die Altbehauptung wird hier nicht rückwirkend legitimiert: neuer Beleg vom heutigen Abruf, kein Nachweis für deren damalige Entstehung oder damalige Gültigkeit.

## Ausgangslage und Grenzen

- Produktionsmomentaufnahme laut Auftrag, **nicht in diesem Review erneut abgefragt**: `dropoff_json={"allowed":true}`, `pickup_json={}`, `status=pruefung`.
- Lokal gelesenes Dossier: `status: aktiv`, beide Servicefelder leer; Adresse/GEO bereits separat geprüft. Keine neue Identitäts-, Register-, GEO- oder Verlagerungsrecherche.
- `README.md`, insbesondere Quellenhierarchie und autoritative Betreiber-Ausnahme, ist maßgeblich. Die im Dossier bereits dokumentierte Qualifikation des Betreibers wird als Voraussetzung verwendet, nicht als heute selbst erneut durchgeführter Registercheck ausgegeben. Mehrere Betreiberseiten sind keine zwei unabhängigen Quellen. EFB ist keine zusätzliche README-Quellenklasse für Kundenannahme.
- `docs/trader-seed.md`: leeres Servicefeld erhält vorhandenes JSON; `status: pruefung` zieht es nicht zurück. Ein Clear wäre eine explizite, separat geprüfte Withdrawal-Entscheidung, keine Schließung oder Ablehnung.

## Direkt geprüfte Quellen und Aussagegrenzen

### 1. Betreiber-Services: ausdrückliche Regel für alle eigenen Standorte

Quelle: https://www.scholz-recycling.com/services/#xr1 (deutsche Seite vollständig direkt gelesen).

Wörtliche Kernaussagen:

> „Selbstverständlich auch in kleinsten Mengen von Selbstanlieferern. Das heißt konkret: Sowohl Kleinstlieferanten und Sammler als auch Gewerbetreibende können an allen unseren Standorten während der Öffnungszeiten ihren Schrott entsorgen.“

> „Private Lieferanten/ Kunden: Personalausweis“

> „Gewerbliche Lieferanten/ Kunden: Hier Selbstauskunft downloaden“

> „Sollten Sie im Auftrag einer anderen Person oder einer Firma Material bei uns anliefern, benötigen wir eine Vollmacht des Material-Eigentümers“

Außerdem ausdrücklich: Anmeldung an Waage/Kasse und Bestätigung der Sicherheitshinweise; Erstverwiegung, Materialbegutachtung, Ausladung, Zweitverwiegung, Abrechnung an der Kasse. Abrechnung nach Qualität/Klassifizierung und Gewicht auf Basis tagesaktueller Marktpreise.

Explizite Ausschlüsse: Quecksilber/Quecksilberanhaftungen, Sprengkörper, explosionsverdächtige Gegenstände, geschlossene Hohlkörper (z. B. Druckbehälter/Gasflaschen), Stoffe mit ionisierender Strahlung oberhalb der gemessenen Umgebungsuntergrundstrahlung.

**Einordnung:** Die Seite ist betreiberweit, nicht Marienberg-spezifisch. Entscheidend ist ihr eigener universeller Geltungsanspruch „an allen unseren Standorten“ samt expliziter Unterscheidung privat/gewerblich; Kundenarten sind nicht aus einem Zertifikat erraten. Keine numerische Mindestmenge aus „kleinsten Mengen“ konstruieren. Kein Beleg für kostenlose Entsorgung, garantierte Auszahlung jeder Fraktion, konkrete örtliche Preise oder eine vorbehaltlose Annahme beliebigen Materials. Die lange betreiberweite Materialwerbung (u. a. Elektronikschrott, Holz, Kunststoffe) **nicht pauschal als Marienberger Annahmesortiment übernehmen**. Auf der Services-Seite kein eindeutiges Veröffentlichungs-/Aktualisierungsdatum gesehen; Aktualität ist hier Abrufstand, nicht ein behauptetes Publikationsdatum.

### 2. Aktueller Standortfinder: Marienberg-Zuordnung und lokale Zeitbedingungen

Quellen:

- https://www.scholz-recycling.com/standorte/
- https://www.scholz-recycling.com/standorte-json-nicht-loeschen/

Den JSON-Endpunkt direkt geladen, geparst und den Datensatz `ort=Marienberg` einzeln ausgegeben (Koordinaten für diese Prüfung ausgespart). `Name` und `Unternehmen` lauten jeweils **Scholz Recycling GmbH**; lokale Betriebsstätte Reitzenhainer Straße 35 c, 09496 Marienberg; Telefon +49 3735 22495, Mail marienberg@scholz-recycling.de.

Lokale Zeiten: Mo–Do 07–09 / 09:15–12 / 12:30–16 Uhr; Fr 07–09 / 09:15–12 / 12:30–13:45 Uhr. `Samstag="geschlossen"` steht neben `Sonderoeffnungszeiten="1. Samstag im Monat 08:00 - 12:00 Uhr ..."`; Feiertagsliste 2026 einschließlich 18.11., 24.–26.12. und 31.12. Die vergangene Betriebsruhe 22.12.2025–03.01.2026 ist keine neue künftige Betriebsruhe.

**Einordnung:** Der Datensatz belegt den lokalen Anschluss an die ausdrücklich alle eigenen Standorte betreffende Regel. Er enthält selbst **keine Kundenarten, eigenständige Privatannahmezusage, Dokumentenpflichten, Mengenlimits oder aktuelle Materialpreisliste**. Öffnungszeiten allein wären kein Kundenannahmebeleg. Samstagswiderspruch nicht in ein vorbehaltloses maschinenlesbares Samstags-Zeitfenster auflösen; vor Besuch bestätigen.

### 3. Sicherheitsordnung: zusätzliche Bedingungen, keine örtliche Kundenartenbestätigung

Quellen:

- https://www.scholz-recycling.com/sicherheitshinweise/
- https://www.scholz-recycling.com/wp-content/uploads/Betriebsordnung-der-Scholz-Recycling-GmbH_28.09.2026.pdf

Die HTML-Seite verlinkt das PDF als aktuelle Betriebsordnung. PDF direkt in Speicher geladen und mit `pdftotext` gelesen, ohne lokale PDF-Datei anzulegen. **Dateiname 28.09.2026 ist nicht gleich Dokumentdatum**: Inhalt nennt Revision 6, Datum 05.11.2025. Darin: Anmeldung vor Betreten an der Waage, Mitarbeiteranweisungen beachten; Sicherheitsschuhe S3 knöchelhoch und Sicherheitsweste auf dem Betriebsgelände, weitere Schutzausrüstung in ausgewiesenen Bereichen.

**Einordnung:** Betreiberweite Regeln; keine speziell Marienberg beschriftete Ordnung und kein eigenständiger Nachweis, welche Materialien private Kunden dort tatsächlich abgeben können. Im JSON genügt der Verweis auf die geltenden Sicherheitsregeln; keine erfundene örtliche Ausnahme oder Leih-Ausrüstung zusagen.

### 4. EFB: eng begrenzter örtlicher Tätigkeitsbeleg

Quelle: https://www.scholz-recycling.com/wp-content/uploads/260623_UeZEFB_MAR.pdf

Direkter Webfetch konnte PDF nicht darstellen; anschließend vollständiger Direktdownload in Speicher und Textextraktion per `pdftotext`. Zertifikat ESN 04-110142 (26), ZZEE003000304010: Prüfung 23.06.2026, Ausstellung 03.08.2026, gültig bis 30.11.2027. Anlagen nennen Marienberg/Reitzenhainer Str. 35 c; S. 4/7 beschreibt die Anlage zur zeitweiligen Lagerung und Behandlung von Eisen- und Nichteisen-Schrott.

Wichtige Grenzen S. 5/8: AVV 160106 ausdrücklich auf Altfahrzeuge **außerhalb** des Anwendungsbereichs der AltfahrzeugV eingeschränkt; Geräte-/Bauteilpositionen enthalten Einschränkungen bezüglich ElektroG. Aus einer AVV-Liste folgt **kein öffentliches Annahmeangebot an Privatkunden**, kein vollständiges Sortiment und keine Annahmepflicht. Hier keine Checkbox-Auswertung allein aus verlorenen Textsymbolen. EFB dient der Tätigkeits-/Reichweitenkontrolle, **nicht** der Begründung von `customer_types` oder einer pauschalen Auto-/Elektroaltgeräteannahme.

### 5. Alte Detailseite: keine zusätzliche aktuelle Annahmeregel

Quelle: https://www.scholz-recycling.com/standort/marienberg

Heute direkt gelesen: Datum 25.09.2017, Überschriften „Marienberg“, ansonsten kein brauchbarer lokaler Annahmetext. Dieser Befund widerlegt weder den aktuellen Finder noch die universelle Services-Regel und beweist keine Einstellung der Annahme. Aggregator-Angabe „Annahme lt. lokaleschrottplatz“ aus der importierten Dossierhistorie wird nicht als Beleg wiederverwendet.

## Konkreter Vorschlag, nur zur Owner-Freigabe

Ein späteres Dossier-Update könnte statt Leerfeld folgenden skalaren JSON-Wert enthalten; **in diesem Review nicht angewendet**:

```yaml
dropoff_json: '{"allowed":true,"customer_types":["privat","gewerbe"],"conditions":"Selbstanlieferung von Schrott, auch Kleinmengen, nach der ausdrücklichen Scholz-Regel für alle eigenen Standorte; Marienberg ist im aktuellen Betreiberfinder geführt. Nur während der örtlichen Öffnungszeiten, Feiertagsausnahmen beachten; Samstagsangaben widersprüchlich (geschlossen / 1. Samstag 08–12), vorher bestätigen. Privatkunden: Personalausweis; Gewerbekunden: Selbstauskunft; bei Lieferung für Dritte Vollmacht des Material-Eigentümers. Anmeldung an Waage/Kasse, Sicherheitsregeln und Materialbegutachtung beachten; Abrechnung nach Klassifizierung und Gewicht. Keine Annahme von Quecksilber/Anhaftungen, Sprengkörpern, explosionsverdächtigen Gegenständen, geschlossenen Hohlkörpern (z. B. Druckbehälter/Gasflaschen) oder Stoffen mit ionisierender Strahlung oberhalb der Umgebungsuntergrundstrahlung. Keine pauschale Marienberger Zusage für das gesamte beworbene Wertstoffsortiment, Altautos oder Elektroaltgeräte; konkrete Fraktion und Konditionen vorab klären. Grundlage: betreiberweite Services-Regel plus örtlicher Finder, Abruf 18.10.2026; keine separate örtliche Annahmeordnung gefunden."}'
```

Bewusst keine numerische Mindest-/Höchstmenge und keine strukturierten `days`/`time_windows`, die den Samstagsvorbehalt verdecken würden. „Vorab klären“ für konkrete Fraktionen ist eine redaktionelle Vorsichtsempfehlung, **kein belegter allgemeiner Termin-/Anmeldezwang vor der Anfahrt**. Anmeldung an der Waage dagegen ist ausdrücklich belegt.

### Abwägung der drei Möglichkeiten

1. **Bedingtes JSON: empfohlen.** Die Kundenarten und grundsätzliche Anlieferung haben einen expliziten universellen Betreiberbeleg plus lokale Betriebsstättenzuordnung. Bedingungen und Reichweitenvorbehalte bleiben sichtbar.
2. **Nur altes `{"allowed":true}` stehen lassen: nicht empfohlen.** Positiver Kern heute stützbar, aber ohne notwendige Einordnung weiterhin zu pauschal; kein Nachweis über die Altprovenienz.
3. **`clear_fields: 'dropoff_json'`: vertretbare konservative Owner-Alternative**, falls ausschließlich eine eigenständige örtliche Kundenannahmezusage akzeptiert werden soll. Das wäre bewusst strenger als die hier als tragfähig bewertete universelle Betreiberregel, nicht durch eine gefundene lokale Ablehnung erzwungen. Dann muss `dropoff_json` absent/leer sein, nicht mit `{}` befüllt; nach `docs/trader-seed.md` ergibt der Clear gespeichertes `{}` = unknown, **nicht** `allowed:false`. Bloßes Leerfeld erhält den Produktionsaltwert. Clear/JSON-Ersetzung sind Alternativen, nicht kombinieren.

## Unverändert / nicht durchgeführt

Keine Dossier- oder DB-Änderung, kein Produktionsschreibzugriff, kein Commit/Deploy, keine GEOrecherche. `pickup_json={}` nicht positiv oder negativ neu bewertet. Lokalen `status: aktiv`, geprüfte Adresse/GEO und Historie nicht verändert; aus keiner Beleglücke eine Ablehnung, Schließung oder historische Verlagerung abgeleitet. Nach einer eventuellen Owner-Änderung wären Seed-Tests, Rebuild/Deploy und separater Produktions-Readback nötig; dieser Bericht behauptet keine bereits erfolgte Veröffentlichung.
