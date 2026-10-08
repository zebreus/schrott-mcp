---
slug: sn-mobil-leipzig-schrottjungs-schrottabholung-leipzig
name: Schrottjungs (Schrottabholung Leipzig)
trader_type: mobil
state: SN
city: 'mobil: Leipzig'
street: ''
postcode: ''
phone: '0173 8705566'
email: ''
opening_hours: ''
website: https://schrottjungs.de
website_status: 'aktiv'
status: aktiv
description: ''
dropoff_json: '{}'
pickup_json: '{"allowed": true, "customer_types": ["privat", "gewerbe"], "conditions": "Mobile Schrottabholung Leipzig + Umland nach Termin; laut Betreiber durch lokale Partnerbetriebe. Leipzig-Seite nennt Eisenschrott unter 300 kg als ausgeschlossen, zugleich ca. 40-80 EUR Gebuehr fuer kleine Eisenmengen/Einzelgeraete; Startseiten-FAQ nennt abweichend 20-50 EUR. Keine widerspruchsfreie Mindestmenge oder feste Gebuehr ableitbar; Annahme und Kosten vor Auftrag konkret bestaetigen. Kleine Mengen wertvoller Buntmetalle laut Leipzig-FAQ kostenfrei, Auszahlung fuer groessere Mengen bzw. hochwertige Metalle nach Tagesangebot. Gewerbliche Grossmengen-Referenzpreise separat, unverbindlich und ausschliesslich bis zu (Abruf 08.10.2026), kein zugesicherter Leipziger Abholtarif."}'
provenance_section: 7) Überregional / mobil / online (in SN tätig)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Schrottjungs (Schrottabholung Leipzig)

## Überblick

Schrottjungs UG (Hamburg, HRB 196586) bewirbt mobile Abholung im Raum Leipzig; laut Startseiten-FAQ führen lokale Partnerbetriebe die Abholung aus. Kein bestätigter stationärer Leipziger Annahmehof und kein namentlich belegter Leipziger Partner. Die Leipzig-Seite enthält Orts-/Textreste zu Lüneburg, Oedeme und Berlin; konkrete Leipzig-Konditionen daher vor Auftrag bestätigen. Dieselbe Seite nennt für kleine Mengen/Einzelgeräte ca. 40–80 € und schließt zugleich Eisenschrott unter 300 kg aus. Zusätzlich nennt die Startseiten-FAQ abweichend 20–50 €. Keine dieser Angaben ist ein verlässlicher pauschaler Tarif. Die separate Gewerbepreisseite gehört demselben Betreiber und bietet Leipzig im Anfrageformular an, ist aber eine ausdrücklich unverbindliche Großmengen-Referenzpreisliste, kein zugesicherter lokaler Abholpreis.

**Preise:** **Ankauf** — Gewerbliche „bis zu“-Referenzpreise: FE Scherenvormaterial 200 €/t, Mischschrott leicht 170 €/t, schwer 210 €/t, Sorte 3 240 €/t, Trägerschrott 230 €/t, Bremsscheiben 240 €/t, Zerlegematerial 540 €/t; NE: Kupfer Raff 8,50 €/kg, Millberry 9,50, Kupferschiene 9,20, Messing gemischt 4,50, Rotguss 8,00, Kabel 5,20, Alu 2,50, V2A 1,10, Zink 1,90, Blei 1,40, Zinn 22,00, Hartmetall/Wolframkarbid 55,00 €/kg. Betreiber kennzeichnet Preise als unverbindliche Ankauf-Orientierung, abhängig von Qualität/Menge/Tagesmarkt. **Verkauf** — keine Verkaufspreisliste. **Gebühren** — Leipzig-Seite nennt ca. 40–80 € für kleine Mengen/Einzelgeräte; dieselbe Seite sagt auch, Eisenschrott unter 300 kg werde nicht abgeholt. Vor Buchung konkret klären. Die Gewerbeseite bewirbt 3–40-m³-Container für Gewerbe/Industrie als mietfrei bei Schrottbefüllung; genaue Voraussetzungen und mögliche Zusatzkosten sind nicht als vollständiger Tarif ausgewiesen. **Geokodierung:** kein Leipziger Standort-Pin, da mobile Abholung ohne ausgewiesenen Hof.

**Preisprüfung 08.10.2026:** Alle 19 Obergrenzen unverändert gegenüber 07.10.2026. Originalbezeichnungen sind „Kabel (diverse)“, „Aluminium (div.)“ und „Hartmetall / Wolfram“; insbesondere keine Reinheit, Kabelausbeute oder Wolframkarbid-Spezifikation aus dem Preis ableiten. Alle Werte gelten laut abschließendem Hinweis für Metallankauf in Großmengen; kleinere Mengen haben andere Konditionen. Weder numerische Großmengenschwelle noch USt-Basis oder Preis-Gültigkeitsdatum ausgewiesen. Abrufdatum ist kein Veröffentlichungsdatum. Containerwerbung nennt ausdrücklich Lieferung und Abholung inklusive, aber keinen vollständigen Kosten-/Ausnahmetarif. Die zusätzlichen 20–50 € aus der Startseiten-FAQ sind Gegenbeleg zur Gebühreneinheitlichkeit, kein alternativ bestätigter Leipzig-Tarif.

**Handler-Einschätzung:** Technisch ist die öffentlich lesbare Gewerbeliste ein geeigneter Monitoring-Kandidat; fachlich derzeit **kein sicherer Standard-Handler für Leipziger Tages-/Privatabholpreise**. Ein bedingter Betreiber-Referenzpreis-Handler wäre nur mit sichtbarer Großmengen-/Unverbindlichkeitskennzeichnung, `price_kind: upto`, ausschließlich oberer Preisgrenze (keine erfundene Untergrenze), originalen Sortenvarianten, ehrlicher Datumsbasis und geklärter Betreiber-Zuordnung sinnvoll. Das bestehende Ingestion-Modell unterstützt `upto`; das löst aber weder die lokale Partner-/Tarifzuordnung noch die Verteilung derselben bundesweiten Liste auf mehrere Stadt-Dossiers. Vor Implementierung Owner-Entscheidung zur kanonischen Händlerzeile und Konditionsdarstellung; Gebühren separat lassen und 300 kg nicht auf Buntmetalle oder Großmengenpreise übertragen. Kein Handler in diesem Rechercheauftrag implementiert.

## Timeline

### Recherche 08.10.2026

- [Recherche 08.10.2026: Hourly Owner Check-in, Preislistenfund erneut direkt geprüft: Gewerbeseite enthält weiterhin 7 FE- und 12 NE-Positionen mit den oben dokumentierten unveränderten BIS-ZU-Werten. Hinweis wörtlich: „Alle genannten Preise sind unverbindliche Richtpreise für Metallankauf in Großmengen“, kleinere Mengen andere Konditionen, Tagespreise auf Anfrage; Menge, Qualität, Sortierung und Marktlage beeinflussen Auszahlung. Kein Preis-Gültigkeitsdatum, keine numerische Großmengenschwelle und keine USt-Basis ausgewiesen. Originalposition „Hartmetall / Wolfram“ belegt nicht die engere historische Bezeichnung Wolframkarbid. Lieferung/Abholung der Gewerbecontainer ausdrücklich inklusive, mietfrei bei Schrottbefüllung; keine vollständige Tarifübersicht. Quelle: https://schrottjungs.de/gewerbe/]
- [Recherche 08.10.2026: Betreiber-/Servicezuordnung beidseitig geprüft: Impressum nennt Schrottjungs UG, Magnus Ditz, Billwerder Steindamm 15a, 20537 Hamburg, HRB 196586 AG Hamburg und 0173-8705566. Northdata bestätigt Firma, Register, Anschrift und Eintragung 16.01.2026 mit Magnus Ditz; keine neue Registerstatusbehauptung aus fehlendem Statusfeld abgeleitet. Leipzig-Detailseite nennt Stadtteile, Umland und Barauszahlung für größere Mengen/hochwertige Metalle, Gewerbeseite bewirbt Deutschlandweit und enthält Leipzig als Formularauswahl. Das verbindet Betreiber und Einsatzgebiet, beweist aber keinen festen Leipziger Tarif oder Hof. Startseiten-FAQ nennt ausdrücklich lokale Partnerbetriebe, ohne einen Leipziger Partner zu identifizieren. Quellen: https://schrottjungs.de/impressum-datenschutz/ ; https://www.northdata.de/Schrottjungs%20UG,%20Hamburg/Amtsgericht%20Hamburg%20HRB%20196586 ; https://schrottjungs.de/schrottabholung-leipzig/ ; https://schrottjungs.de/gewerbe/ ; https://schrottjungs.de/]
- [Recherche 08.10.2026: Gegenbelege zu Mindestmengen/Gebühren erhalten: Leipzig-Schrottartenliste „Wir holen NICHT ab: ... Eisenschrott < 300kg“, zugleich Warnhinweis kleine Eisenmengen/Einzelgeräte nur gegen ca. 40–80 EUR; Leipzig-FAQ nennt 300 kg Mindestmenge und mögliche 40–80 EUR Gebühr, kleine Mengen wertvoller Buntmetalle kostenfrei. Startseite nennt mehrfach 40–80 EUR, FAQ „Kostet die Schrottabholung etwas?“ dagegen 20–50 EUR für Einzelgeräte/geringe Eisenmengen. Keine pauschale Annahme ab/unter 300 kg und keinen fixen Gebührenbereich als sicher gesetzt; pickup_json hält Widersprüche und Vorabklärung fest. Diese Kundengebühren sind keine Ankaufspreise. Quellen: https://schrottjungs.de/schrottabholung-leipzig/ ; https://schrottjungs.de/]
- [Recherche 08.10.2026: Handler-Gate: öffentlich veröffentlichte Betreiber-Richtpreisobergrenzen sind ein echter Preisfund, nicht bloß „Preise auf Anfrage“. Für Leipzig jedoch keine verbindlichen Tages-/Privattarife belegt; Großmengenbedingungen, Partnerausführung und bundesweite Mehrfach-Dossiers verhindern eine sichere unqualifizierte Zuordnung. Nur bedingtes Referenzpreis-Monitoring mit upto-Semantik und vorab geklärter kanonischer Händlerzeile/Konditionsdarstellung empfohlen; kein Handler implementiert. Quellen: https://schrottjungs.de/gewerbe/ ; https://schrottjungs.de/schrottabholung-leipzig/ ; https://schrottjungs.de/]
- [Recherche 08.10.2026: Direkte Sichtprüfung der Leipzig-Abholseite ergab neben den Leipzig-Bezügen Textreste zu Lüneburg, Oedeme und Berlin (einschließlich Container-/Termin-FAQ). Das spricht für redaktionelle Vorlagenreste und macht die konkreten lokalen Konditionen weniger belastbar; es ist kein Beleg, dass der Leipziger Service nicht besteht. Gewerbeseite: mietfreie Container 3–40 m³ bei Befüllung mit Schrott, ohne vollständige Tarif-/Zusatzkostenübersicht. Ankaufstabelle bleibt eine separate unverbindliche „bis zu“-Liste; kein Verkaufspreisblatt. Vor Abholung bzw. Containerbuchung Material, Menge, Liefergebiet und etwaige Gebühren direkt bestätigen. Quellen: https://schrottjungs.de/schrottabholung-leipzig/ ; https://schrottjungs.de/gewerbe/]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Preisrubrik Gewerbe direkt geprüft. Die ausdrücklich unverbindliche „bis zu“-Ankaufstabelle nennt FE in €/t: Scherenvormaterial 200, Mischschrott leicht 170, Mischschrott schwer 210, Sorte 3 240, Trägerschrott 230, Bremsscheiben 240, Zerlegematerial 540; NE in €/kg: Kupfer Raff 8,50, Millberry 9,50, Kupferschiene 9,20, Messing gemischt 4,50, Rotguss 8,00, Kabel 5,20, Alu 2,50, V2A 1,10, Zink 1,90, Blei 1,40, Zinn 22,00, Wolframkarbid 55,00. Laut Betreiber hängen Werte von Qualität, Menge und Tagesmarkt ab; Großmengen/aktuelle Rates auf Anfrage. Leipzig-Detailseite nennt ca. 40–80 € Gebühr für kleine Mengen/Einzelgeräte, enthält zugleich die Einschränkung, Eisenschrott unter 300 kg nicht abzuholen; Widerspruch nicht aufgelöst, Kondition vor Auftrag erfragen. Gewerbeseite bewirbt 3–40-m³-Container mietfrei bei Schrottbefüllung; genaue Voraussetzungen vor Auftrag klären, nicht als allgemeine Privathaushaltskondition verstehen. ANKAUF: obige Referenzpreise (nicht verbindlich). VERKAUF: keine Liste. GEBÜHREN: nur die bedingte ca.-Gebühr; Containerbedingung separat. Keine Leipziger Annahmestelle/Geokodierung; Quelle: https://schrottjungs.de/gewerbe/ ; https://schrottjungs.de/schrottabholung-leipzig/.]

### Recherche 03.10.2026 (Feedback #4431 Owner-Triage)

- [Recherche 03.10.2026: Leipzig-Seite bewirbt Abholung und Container, nennt aber keinen Leipziger Annahmehof und schließt Anlieferung nicht ausdrücklich aus. Die Seite führt auch Umlandorte auf, sodass eine frühere behauptete disjunkte Trennung von Leipzig/Umland nicht stimmt; vorhandene Dossiers bleiben ohne Merge bis zur redaktionellen Dublettenprüfung. Angaben zu Kleinmengen Eisen sind auf derselben Seite widersprüchlich und werden nicht als Kondition verallgemeinert. `dropoff_json` bleibt unbekannt.; Quelle(n): https://schrottjungs.de/schrottabholung-leipzig/, https://schrottjungs.de/impressum-datenschutz/, https://www.northdata.de/Schrottjungs%20UG,%20Hamburg/Amtsgericht%20Hamburg%20HRB%20196586]

### Recherche 01.10.2026 (Feedback 2680)

- [Recherche 01.10.2026: Feedback 2680 berechtigt — Betreiber-Primärquelle verifiziert (Impressum: Schrottjungs UG (haftungsbeschränkt), Billwerder Steindamm 15a, 20537 Hamburg, GF Magnus Ditz, HRB 196586 AG Hamburg, HR-kongruent per Northdata/Handelsregister; per-city Detailseite Leipzig mit Servicegebiet + Konditionen, aktuelle Inhalte; Owner-Ausnahme greift): Tel 0173/8705566, trader_type mobil (kein stationärer Platz, reine Abholung), pickup_json mit Abholkonditionen, website_status aktiv. Sitz Hamburg ≠ Einsatzgebiet (SN-mobil-Row bleibt, kein Phantom). Quelle(n): https://schrottjungs.de/schrottabholung-leipzig/, https://schrottjungs.de/impressum-datenschutz/, https://www.northdata.de/Schrottjungs%20UG,%20Hamburg/HRB%20196586]

### Importiert (Seed-Stand 2026-09-30)

- kostenlose Abholung, Barankauf Bunt/Kupfer/Kabel/VHM
- kein eigener Platz, mobil only
