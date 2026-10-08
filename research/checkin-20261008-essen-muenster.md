# Feedback #5391/#5392 — Essen und Münster, 08.10.2026

## Ergebnis und Freigabegrenze

Zwei neue Prüfdossiers angelegt, da beide Betreiberwebsites tatsächlichen Metallankauf nennen und kein vorhandener Händler zugeordnet werden konnte:

- `dossiers/nw/nw-essen-autoverwertung-chruscz.md`
- `dossiers/nw/nw-munster-konig-wilhelm-autoteile.md`

**Nicht als abschließend verifiziert freigegeben:** README-Zweiquellenstandard noch offen; Kontakte, Website, Öffnungszeiten, Beschreibung und Service-JSON im Frontmatter bewusst leer. Identitäts-/Geschäftsdaten der Betreiberseiten vollständig in der Timeline erhalten. Keine Koordinaten, Zertifizierungsfelder oder Preisbeobachtungen erfunden. `pruefung` ist ein Belegflag, keine Behauptung, dass kein Ankauf stattfindet. Owner kann die offene GESA-Verifikation ergänzen und die dann belegten Felder freigeben. Kein Commit/Deployment, aktuelle DB ausschließlich `sqlite3 -readonly` gelesen.

## #5391 — Autoverwertung Chruscz / Peter Chruscz

- [Impressum](https://www.autoverwertung-chruscz.de/impressum): Einzelunternehmer Peter Chruscz, Heegstraße 64, 45356 Essen, 0201 666086, info@autoverwertung-chruscz.de, DE187318676. [Kontakt](https://www.autoverwertung-chruscz.de/kontakt) bezeichnet Nr. 64 als Büro; Standort im Seitenfuß 64-66. Kein Anlass zu Aufteilung in zwei Händler.
- [Leistungen](https://www.autoverwertung-chruscz.de/leistungen): ausdrücklicher Ankauf von Altmetall/Stahlschrott, Kupfer, Aluminium und Messing. „Fair/tagesaktuell“ ohne Zahlen ist kein Kurs. Keine numerischen Ankauf-, Verkauf- oder Gebührenlisten auf geprüften Seiten.
- Dieselbe Leistungsseite: kostenlose Fahrzeugabholung in Essen/Umgebung, kostenlose Abmeldung auf Wunsch, Verwertungsnachweis; das gilt nicht automatisch für loses Altmetall oder jede Entsorgung. Keine belegten Kundenarten/Mindestmengen/Metallannahmezeiten.
- Kontakt/Seitenfuß: Mo-Fr 13:30-17:30, Sa 08:00-14:00, So geschlossen; Termine nach Vereinbarung; Ersatzteilverkauf nur zu Öffnungszeiten. Verzeichnisbehauptungen über zusätzliche Vormittagszeiten nicht übernommen.
- [Startseite](https://www.autoverwertung-chruscz.de/): DEKRA-zertifiziert, historische TÜV-Rheinland-Erstzertifizierung 1998. Aktuelle Urkunde/Gültigkeit nicht geprüft.
- **Zweitbelegversuch:** [Creditreform](https://firmeneintrag.creditreform.de/45356/5110270310/PETER_CHRUSCZ_AUTOVERWERTUNG_CHRUSCZ) im Suchauszug Peter Chruscz, Gewerbebetrieb, Heegstr. 64, 45356 Essen; Vollabruf HTTP 403. [Betreiber-Facebook](https://www.facebook.com/chruscz) von Website verlinkt; Suchauszug Name/64-66/Telefon, Vollabruf ohne Inhalt. Suchauszüge nur partielle Stütze, keine vollständige aktuelle Prüfung.

## #5392 — König Wilhelm Autoteile / Einbrodt & Schubert GbR

- [Impressum](https://koenig-wilhelm.com/Impressum): GbR, Dirk Einbrodt und Ulrich Schubert, Dahlweg 122, 48153 Münster, 0251 791170, Fax 0251 790631, DE126103793. Kein Registerkennzeichen; README-Primärquellen-Ausnahme für HR-verifizierte Betreiber greift nicht.
- [Anfahrt](https://koenig-wilhelm.com/Anfahrt): tatsächliche Öffnungszeiten Mo-Fr 09:00-12:30 und 13:00-17:00. E-Mail-Schutzlink `771e191118371c1812191e105a001e1b1f121b1a5914181a` per Cloudflare-XOR (erstes Byte Schlüssel 0x77) zu `info@koenig-wilhelm.com` decodierbar. Feedbacks „nicht lesbar“ geklärt, jedoch noch nicht als unabhängig verifizierten Kontakt übernommen.
- [Leistungen](https://koenig-wilhelm.com/Leistungen): **Metallankauf und Ankauf von Katalysatoren**, nicht nur Autoteileverkauf. [Startseite](https://koenig-wilhelm.com/): Ankauf alter/beschädigter/Unfallfahrzeuge. Keine numerischen Ankaufkurse oder Katalysatorbewertung veröffentlicht.
- [Autoverwertung](https://koenig-wilhelm.com/Autoverwertung): Abholung Münster/Umkreis gegen unbezifferte Pauschale, fallweise kostenlos nach Fahrzeug/Zustand/Entfernung; Abmeldung und Verwertungsnachweis. Nicht generell kostenlos. Zertifizierung seit 2008 und Netzwerk seit 2016 nur Betreiberbehauptung, aktuelle Urkunde fehlt.
- [Abschleppdienst](https://koenig-wilhelm.com/Abschleppdienst): Telefonservice bis 18 Uhr; kein Beleg für Metallannahme bis 18 Uhr. Anfahrtzeiten sind andere Angabe/Zweck.
- Startseite: 799,99/49,99/11,99 EUR für konkrete Ersatzteile, inkl. USt. zzgl. Versand; keine Schrottankaufpreise. Nicht in `prices` übertragen. Kein Gebührenbetrag aus „günstiger Abholpauschale“ errechnet.
- **Zweitbelegversuch:** [Betreiber-Facebook](https://www.facebook.com/KoenigWilhelmAutoteile) verlinkt, Vollabruf leer; [eBay-Shop](https://www.ebay.de/str/konigwilhelmautoteile) führt Sicherheitsabfrage. Keine passende Creditreform-/Northdata-Vollquelle gefunden. Gelbe Seiten/Cylex/European Business Connect sind nur Verzeichnis-Leads, kein Zweitbeleg.

## GESA und offene Belegfragen

[Amtliche GESA-Information](https://www.altfahrzeugstelle.de/) verweist auf [Fachbetrieberegister](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung). Register liefert beim direkten HTTP-Abruf nur Vaadin-JavaScript-Shell, keine Firmen-/Zertifikatsdaten. Kennungen **ZAES00500008/1** (Chruscz) und **ZAES01100066/1** (König Wilhelm) stammen ausschließlich aus Feedback. Nicht als eigenständig bestätigte Anerkennung ausgegeben. Im verfügbaren Umfeld kein Browser/Playwright vorhanden.

Noch zu klären: GESA-Datensatz mit Name/Adresse und aktueller Anerkennung; unabhängige aktuelle Identität; vollständige Annahme-/Vergütungsbedingungen, Kundenarten, Mengen, Fahrzeugunterlagen, Metallabholung und eventuelle Sorten-/Entsorgungsgebühren. Keine Negativbehauptung aus fehlenden öffentlichen Kursen.

## Bestandsabgleich und Schreibschutz

Produktions-DB `/var/lib/schrott-mcp/public.db` mit `sqlite3 -readonly` nach Namen/Domain/Adressfragmenten, anschließend sämtlichen Essen-/Münster-Händlern gelesen. Kein zuordenbarer Treffer. Bestehender Dossierbestand per Volltextsuche Chruscz/Einbrodt/Schubert/König Wilhelm/Heegstr/Dahlweg geprüft. Bindemann mit historischem Verzeichnislead Dahlweg 38 ist nicht die GbR am Dahlweg 122; Schubert in Roßtal kein Münsteraner Match; Peter Häuser/Prison in Essen andere Identitäten. Keine vorhandenen Dossiers überschrieben oder Händler gemergt. Feedback #5391/#5392 ebenfalls ausschließlich read-only aus `internal.db` gelesen.

Owner übernimmt gemeinsame Prüfung, Seed-Test, Commit und Veröffentlichung. Dossiers erweitern zunächst nachvollziehbare Prüfabdeckung; vollständige Kontaktfreigabe benötigt den offenen qualifizierten Zweitbeleg.

## Testversuch

`/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed` am 08.10.2026 lief an, scheiterte jedoch im Corpus-Build an einer parallel neu angelegten, fremden Datei: `dossiers/ni/ni-westerstede-muller-autoverwertung.md:17`, unquotierter Wert `Feedback #5388 vom 08.10.2026` (Kommentarzeichen braucht Quotes). Fremde Arbeit nicht verändert; Owner muss nach deren Korrektur den gemeinsamen Gate wiederholen. Kein erfolgreicher Seed-Test behauptet.
