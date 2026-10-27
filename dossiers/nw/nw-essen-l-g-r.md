---
slug: nw-essen-l-g-r
clear_fields: geo
name: L.G.R.
trader_type: schrotthaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# L.G.R.

## Überblick

Ein starker Kandidat für das Kürzel „L.G.R.“ in Essen ist die L.G.R. Lothar Gedenk Recycling GmbH: Das Betreiber-Impressum nennt genau diese Firma, Karl-Legien-Straße 10, 45356 Essen und HRB 12622 beim Amtsgericht Essen; NorthData führt dieselbe Firmierung/Registerkennung/Anschrift und als Gegenstand An- und Verkauf von Altkleidern/-schuhen sowie Sammelcontainer. Die Betreiberseite beschreibt kommunale Erfassung und Recycling von Alttextilien und Altschuhen, nicht Metall- oder Schrottankauf. Die Namens-/Ortskongruenz ist stark, die Branchenzuordnung des Seeds widerspricht aber dem belegten Unternehmensgegenstand; daher keine stillschweigende Umklassifizierung oder Kontaktübernahme, Status `pruefung`.

**ANKAUF:** kein Metall-/Schrottankauf belegt; der Firmengegenstand betrifft Alttextilien und -schuhe. **VERKAUF:** keine Metall- oder Schrottverkaufspreise belegt. **GEBÜHREN:** keine Schrott-/Entsorgungstarife belegt. **GEO:** die Betreiberanschrift erst nach Klärung, ob der Seed tatsächlich diese Textilrecyclingfirma meint, als Händlerstandort geokodieren.

## Timeline

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Der Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), also auf der Fernbus-Spur des Essener Hauptbahnhofs und nicht auf einem Betriebsort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Das Betreiber-Impressum der L.G.R. Lothar Gedenk Recycling GmbH, HRB 12622 Amtsgericht Essen, nennt Karl-Legien-Straße 10, 45356 Essen, vertreten durch Marwin und Dario Gedenk; die Nominatim-Vorwärtssuche findet dort ein Gebäude (way 187631828, 51.4976865/6.9825390), rund 5,73 km vom gespeicherten Punkt entfernt. Neu belegt ist zudem die Verbindung zur Dossierzeile: zwei aktuelle Verzeichnisse führen „L.G.R. Recycling GmbH“ mit der Kategorie Schrotthandel an genau dieser Adresse mit identischer Telefonnummer, E-Mail und Domain. Damit widerspricht die eigene Zeile dem gespeicherten Punkt, und der Punkt trägt für sie keine Aussage. `clear_fields: geo` nimmt ihn beim regulären Seed zurück, **ohne** Ersatzkoordinate: Die Betreiberseite dokumentiert an dieser Adresse ausschließlich Textil- und Schuhrecycling, eine Annahme- oder Zufahrtskoordinate für Schrott wäre eine Behauptung, die keine Quelle trägt. Die Branchen- und Materialfrage (Textil gegenüber Metall) bleibt als eigene, nicht-GEO-Frage offen; `status: pruefung` bleibt. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/46961889/full.json ; https://lgr-essen.de/impressum/ ; https://lgr-essen.de/dienstleistungen/ ; https://www.northdata.de/L.G.R.+Lothar+Gedenk+Recycling+GmbH,+Essen/Amtsgericht+Essen+HRB+12622 ; https://www.11880.com/branchenbuch/essen-ruhr/060444309B102111494/l-g-r-recycling-gmbh.html ; https://ruhrpott-kurier.de/branchenbuch/essen/schrotthandel/l-g-r-recycling-gmbh-7951/ ; https://nominatim.openstreetmap.org/search?q=Karl-Legien-Stra%C3%9Fe+10%2C+45356+Essen&format=jsonv2&addressdetails=1 ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt als offenen Korrekturauftrag dokumentieren

- [Owner-Korrektur 27.10.2026: Essener Fernbusbahnhof-Punkt als offenen Korrekturauftrag dokumentieren: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Dieser Punkt liegt laut OSM auf der Straße „Freiheit“, 45128 Essen (way 46961889, `highway=service`, `bus=designated`, `vehicle=no`), genauer auf dem Fernbusbahnhof am Essener Hauptbahnhof (Haltestellenknoten 599294635, „Essen, Fernbusbahnhof“). Das ist eine Busspur und kein Gewerbestandort; in den Umweltinspektionsberichten der Bezirksregierung Düsseldorf für die Stadt Essen gibt es keinen Eintrag auf dieser Straße. Die Betreiberseiten der L.G.R. Lothar Gedenk Recycling GmbH nennen Karl-Legien-Straße 10 und bestätigen dort die Annahme von Altkleidern und Schuhen; das belegt eine tatsächliche Annahmestelle dieser Textilrecyclingfirma, aber nicht Schrott- oder Metallannahme und nicht, dass sie die als `schrotthaendler` importierte Zeile ist. Eine kuratierte Adresse fehlt, der Identitäts- und Branchenwiderspruch ist offen. Der Punkt wird deshalb nicht pauschal zurückgenommen (kein Pauschal-Clear), sondern als reiner Stadt-Platzhalter dokumentiert: Er ist **ein offener Korrekturauftrag** und erst nach Klärung der Identität mit einer kuratierten Adresse oder einer belegten Annahmestelle zu entscheiden. Bis dahin kein Ersatzpunkt und keine Statusänderung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://lgr-essen.de/impressum/ ; https://lgr-essen.de/dienstleistungen/ ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 26.10.2026: Unternehmensgegenstand erneut gegengeprüft

- [Recherche 26.10.2026: Das direkt gelesene Betreiber-Impressum bestätigt L.G.R. Lothar Gedenk Recycling GmbH, Karl-Legien-Straße 10, 45356 Essen, HRB 12622, Amtsgericht Essen, vertreten durch Marwin und Dario Gedenk; die Über-uns-Seite beschreibt die operative Geschäftsführung durch die zweite Generation und kommunale Erfassung/Recycling von Alttextilien und Altschuhen. Das stärkt die mögliche Identität zum Kürzel/Ort, beseitigt aber nicht den Branchenwiderspruch zum Schrott-Seed. Keine Felder übernommen oder Branche umklassifiziert. **Ankauf:** kein Metall-/Schrottankauf belegt; **Verkauf:** keine Metall-/Schrottliste; **Gebühren:** keine Schrotttarife. GEO erst nach Seed-Zuordnung. Quellen: https://lgr-essen.de/impressum/ ; https://lgr-essen.de/ueber-uns/ ; https://lgr-essen.de/dienstleistungen/ .]

### Recherche 24.10.2026

- [Recherche 24.10.2026: Betreiber-Impressum direkt gelesen: L.G.R. Lothar Gedenk Recycling GmbH, Karl-Legien-Straße 10, 45356 Essen, HRB 12622 Amtsgericht Essen, vertreten durch Marwin und Dario Gedenk. Die Betreiberseite beschreibt Erfassung/Recycling von Alttextilien und Altschuhen; das NorthData-Profil bestätigt Firmierung, HRB, Anschrift und den Gegenstand An-/Verkauf von Altkleidern/-schuhen und Sammelcontainern; dort ist der Jahresabschluss 2024 als am 05.02.2026 publiziert gelistet. Zusammen ist das ein starker möglicher Identitätsmatch zum Kürzel/Ort, aber ein inhaltlicher Widerspruch zur Seed-Branche Schrott. Keine Kontakt-/Website-Felder ergänzt und keine automatische Umklassifizierung vorgenommen. ANKAUF: keine Metall-/Schrottkurse; VERKAUF: keine Metall-Preisliste; GEBÜHREN: keine Schrotttarife. GEO erst nach Bestätigung der Seed-Zuordnung. Quellen: https://lgr-essen.de/impressum/ ; https://lgr-essen.de/ ; https://www.northdata.de/L.G.R.+Lothar+Gedenk+Recycling+GmbH,+Essen/Amtsgericht+Essen+HRB+12622]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Essen
