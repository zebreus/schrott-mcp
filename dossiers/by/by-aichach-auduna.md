---
slug: by-aichach-auduna
name: AUDUNA
trader_type: sonstige
state: BY
city: Aichach
street: Oberbernbacher Weg 12
postcode: 86551
phone: 08251 9382225
email: info@auduna.de
opening_hours: 'Fr. 10:00-15:00, Sa. 10:00-15:00, nach Vereinbarung'
website: https://auduna.de
website_status: aktiv
status: aktiv
description: 'Gold- und Altmetallankauf in Aichach (Gold, Silber, Platin, Palladium, Kupfer, Messing, V2A, Alteisen). ANKAUF: Der Betreiber veröffentlicht eine live über eine eigene JSON-Schnittstelle aktualisierte Auszahlungsliste je Feingehalt, Legierung und Materialgruppe. VERKAUF: kein Verkauf von Metallen belegt. GEBÜHREN: keine Gebührenordnung veröffentlicht; Abholung nur nach telefonischer Vereinbarung.'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Neu: nur Verzeichnis-Beleg (lokaleschrottplatz.de, Ankauf unklar)'
provenance_ankauf_raw: unklar (Register-Prosa)
provenance_origin: prose
---

# AUDUNA

## Überblick

AUDUNA GmbH, HRB 27669 AG Augsburg, betreibt am Oberbernbacher Weg 12 in Aichach einen Gold- und Altmetallankauf. Das Impressum nennt Geschäftsführer Manfred Peter, die Kontaktseite die E-Mail info@auduna.de sowie die Öffnungszeiten Freitag und Samstag 10:00 bis 15:00 Uhr und Termine nach Vereinbarung.

Besonders zu beachten: Der Betreiber ist der bislang einzige Händler dieser Welle mit einer **live befüllten, ohne Anmeldung maschinenlesbaren Ankaufpreisliste**. Die Startseite trägt eine Tabelle mit 21 Preiszeilen, die über die eigene Schnittstelle https://auduna.de/api/metal-prices (JSON) versorgt wird; die Auszahlungsfaktoren je Legierung stehen in https://auduna.de/assets/js/metal-values.js. Die Edelmetallzeilen rechnen sich live aus dem Kursfeed, die Buntmetall- und Alteisenzeilen sind feste Preise. Das sind durchweg **Auszahlungspreise an den Verkäufer** (Ankaufseite), keine Verkaufskurse; ein Verkauf von Metallen wird nicht angeboten, und eine Gebührenordnung ist nicht veröffentlicht.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Registerfund ohne geprüfte Website (PLZ 86551)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website (Impressum: AUDUNA GmbH, Oberbernbacher Weg 12, 86551 Aichach, Tel. 08251 9382225, HRB 27669 AG Augsburg) + Creditreform-Gegenbeleg (Oberbernbacher Weg 12, 86551 Aichach, +49 8251 9382225) → street/postcode/phone/website belegt, website_status aktiv; E-Mail-Feld im Impressum leer (leer gelassen); Mobil 0160 2667599 nur Betreiber-Einzelbeleg; Profil: Edelmetall-/Altmetallankauf (Gold/Silber/Platin/Palladium, Kupfer, Messing, V2A, Alteisen), kein klassischer Schrottplatz; Quelle(n): auduna.de/impressum, firmeneintrag.creditreform.de]

### Recherche 30.10.2026

- [Recherche 30.10.2026: Betreiber-Primärquelle vollständig ausgewertet. Das Impressum nennt AUDUNA GmbH, Geschäftsführer Manfred Peter, Oberbernbacher Weg 12, 86551 Aichach, Telefon 08251 9382225 und Mobil 0160 266 7599, Handelsregister Augsburg HRB 27669 und Steuernummer 102/121/81434; die Seite trägt den Copyright-Hinweis 2026. Die Kontaktseite nennt die Öffnungszeiten wörtlich "Fr.: 10-15 Uhr", "Sa.: 10-15 Uhr" und "Und nach Vereinbarung". Die E-Mail info@auduna.de ist im HTML als mailto mit numerischen Zeichenentitäten hinterlegt und wird zusätzlich im Footer per ROT13-Skript ausgeschrieben, sie ist damit technisch verifizierbar. Register (HRB 27669 AG Augsburg) plus Impressum mit Nummer, aktuelle Inhalte und standortbezogene Seiten erfüllen die Betreiber-Ausnahme; deshalb status von pruefung auf aktiv gesetzt und email sowie opening_hours gefüllt. Quelle(n): https://auduna.de/impressum; https://auduna.de/kontakt; https://auduna.de/]
- [Recherche 30.10.2026: ANKAUF, Edelmetalle, live. Die Startseite trägt die Tabelle "Geld gibt's von der AUDUNA GmbH für:" mit 21 Preiszeilen, befüllt aus der eigenen Schnittstelle https://auduna.de/api/metal-prices (JSON, ohne Anmeldung). Abruf am 30.10.2026, timestamp 1793372043 (2026-10-30 14:54 UTC): Feingold 999 rund 112,12 EUR/g, Gold 750 rund 77,20 EUR/g, Gold 585 rund 60,21 EUR/g, Gold 333 rund 34,28 EUR/g, Feinsilber 999 rund 1,36 EUR/g, Silber 925 rund 1,06 EUR/g, Silber 800 rund 0,92 EUR/g, Feinplatin 999 rund 39,46 EUR/g, Platin 950 rund 31,65 EUR/g, Feinpalladium 999 rund 26,99 EUR/g, Palladium 950 rund 21,64 EUR/g. Die Auszahlungsfaktoren stehen wörtlich in metal-values.js, etwa { name: "Gold 585 (14kt)", price: 0.585 * 0.912 } und { name: "Silber 925 (Sterling)", price: 0.925 * 0.802 }. Das sind Auszahlungspreise an den Verkäufer (Ankaufseite), keine Verkaufskurse. Quelle(n): https://auduna.de/api/metal-prices; https://auduna.de/assets/js/metal-values.js; https://auduna.de/]
- [Recherche 30.10.2026: ANKAUF, Buntmetalle und Alteisen, feste Preise. Die Werte stehen wörtlich in metal-values.js und werden in metal-prices.js mit folgenden Einheiten gerendert: Kupferdraht rosafarbig 11,10 EUR/kg, Kupfer ohne Anhaftungen 10,30 EUR/kg, Kupferkabel ohne Anhaftungen 3,40 EUR/kg, Messing ohne Anhaftungen 6,10 EUR/kg, Aluminium ab 1mm 1,30 EUR/kg, V2A (nicht magnetisch) 0,55 EUR/kg, Alteisen Schrott >1t 115,00 EUR/t, Alteisen Schrott 201-999kg 0,09 EUR/kg, Alteisen Schrott 0-200kg 0,07 EUR/kg, Geschirrzinn 17,06 EUR/kg. Die Seite vermerkt wörtlich "Preise bitte anfragen. Auszahlung sofort bar oder per Überweisung, wie Sie wünschen." und "Alle Preisangaben sind unverbindlich." Quelle(n): https://auduna.de/assets/js/metal-values.js; https://auduna.de/assets/js/metal-prices.js; https://auduna.de/]
- [Recherche 30.10.2026: ANKAUF, Händlerpreise. Der Abschnitt dealer_plated liefert, ebenfalls aus dem Kursfeed abgeleitet: Versilbertes Besteck Ms (90-350) 42,95 EUR/kg, Versilbertes Besteck Ms Messer (90-350) 12,89 EUR/kg, Versilbertes Besteck V2A (90-350) 20,05 EUR/kg, Versilbertes Besteck V2A Messer (90-350) 6,44 EUR/kg, Versilbertes Besteck Ms (0-89) 20,05 EUR/kg, Versilbertes Besteck Ms Messer (0-89) 6,44 EUR/kg, Hotelsilber (90-350) 24,05 EUR/kg, Geschirrzinn Händler 27,06 EUR/kg. Die verlinkte Seite /haendler ist erreichbar, enthält aber keine Preistabelle; die ebenfalls verlinkte Seite /preistabelle liefert HTTP 404. Quelle(n): https://auduna.de/haendler; https://auduna.de/preistabelle; https://auduna.de/assets/js/metal-values.js]
- [Recherche 30.10.2026: VERKAUF und GEBÜHREN getrennt geprüft. Die Unterseiten /gold, /silber, /platin, /palladium und /versilbertes führen je eine Tabelle mit den Spalten "Feingehalt" und "Ankaufspreis"; /gold ist mit "Aktuelle Edelmetallpreise & Ankaufskurse" überschrieben und bietet zusätzlich einen Goldrechner ("Aktueller Kurs", "Geschätzter Wert") sowie den Aufruf "Kostenlos anfragen". Ein Verkauf von Gold, Silber, Platin oder Palladium wird nicht angeboten, die Startseite sagt ausdrücklich "Wir kaufen alle Barren und Münzen ... Schmuck, Modeschmuck, Essbesteck und Töpfe,Trachtenschmuck, Vergoldetes, Alufelgen, Edelsteine, Münzen, Uhren, Omaschmuck". Gebühren: keine Gebührenordnung und kein Entgelt veröffentlicht; die Abholung ist nur nach telefonischer Terminvereinbarung möglich. Quelle(n): https://auduna.de/gold; https://auduna.de/silber; https://auduna.de/versilbertes; https://auduna.de/]
- [Recherche 30.10.2026: OSM-Straßenscan. Rasterfenster bbox=11.1233009,48.4589424,11.1349180,48.4711331, gesamter Straßenverlauf Oberbernbacher Weg in Aichach (ways 24335850, 24335880, 24335883, 24335884, 24335885, 229049640, 25014936, 334066953, 510002191), Straße "Oberbernbacher Weg", Filter addr:street mit addr:housenumber. Ergebnis live: 29 Adressobjekte mit 23 verschiedenen Hausnummern (1, 1c, 1d, 2, 3, 3a, 4, 5, 6, 6a, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 20, 22, 24). Die Seed-Hausnummer 12 ist damit live als Adressobjekt erfasst. Geokodierung: Punkt für Oberbernbacher Weg 12, 86551 Aichach ist noch zu setzen. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=11.1233009,48.4589424,11.1349180,48.4711331]
