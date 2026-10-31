---
slug: sn-leipzig-04319-lks-kat
name: LKS Kat GmbH
trader_type: sonstige
state: SN
city: Leipzig
street: Riesaer Str. 245/247
postcode: '04319'
phone: 0341 26488134
email: ''
opening_hours: ''
website: https://lks-kat.de
website_status: blockiert
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# LKS Kat GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Katalysatoren — Entsorgungsfachbetrieb, Barankauf + Abholung (NEU)
- Adresse: Leipzig 04319

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur 30.09.2026 (Feedback-Triage): Feedback 1713 teilweise — kein reiner Platzhalter: Firma real (Riesaer Str. 245/247, 04319 Leipzig per Google-Index + Northdata HRB 31888), aber Website liefert nur Host-Europe-403 (temporaer defekt/Bot-Block); Adresse nachgetragen, status pruefung bis Re-Check. Website bleibt.]
- [Korrektur 30.09.2026 (Feedback-Triage): Re-Check fb1836 — lks-kat.de liefert per Direktabruf WEITER 403 (Root + /kontakt/, 30.09.2026), Befund bestaetigt → website_status blockiert (Bot-Block/Hardening; Google-Index zeigt weiterhin /kontakt- und /impressum-Inhalte, Homepage-Snippet wirkt kompromittiert/Spam — Klaerfall). Telefon 0341 26488134 2-quellig belegt (Betreiber-Impressum via Google-Index + kirner-abbruch.de + exa.ai) → gefuellt. Email weber@lks-kat.de nur Betreiber-Einzelbeleg → nicht gefuellt. Adresse Riesaer Str. 245/247, 04319 Leipzig weiter bestaetigt (Northdata HRB 31888). Quelle: Direktabruf 403, Google-Index lks-kat.de/kontakt + /impressum, northdata.de, kirner-abbruch.de.]

### Recherche 01.10.2026 (Feedback-Triage ID 3129)

- [Recherche 01.10.2026: Feedback 3129 ('website_status falsch', tot-Forderung) als UNBERECHTIGT verworfen, Klärfall dokumentiert — Re-Abruf 01.10.2026 liefert WEITER 403 (kein Firmeninhalt abrufbar, Adresse/Telefon nicht verifizierbar), NICHT den gemeldeten Host-Europe-Platzhalter 'im Aufbau' (bot-abhängige Antworten: 403 vs. Platzhalter je nach Client — Bot-Block/Hardening); Firma real (Northdata HRB 31888, Google-Index mit /kontakt + /impressum + Telefon 0341 26488134); website_status bleibt blockiert (NICHT tot: Domain lebt, Firma existiert, Relaunch/Block ununterscheidbar); status bleibt pruefung bis Re-Check mit Firmeninhalt. Quelle(n): Direktabruf https://lks-kat.de → 403 (01.10.2026)]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprueft (Riesaer Straße, Leipzig).** Kachelraster ueber den **gesamten** Strassenverlauf (Strassenbbox 12.4468059/51.3344129 bis 12.5130145/51.3602760, **21 Kacheln, 0 Kachel-Fehler**). Zaehlung getrennt: **165 Adressobjekte / 142 verschiedene Hausnummern** auf der Strasse (35, 37, 39, 41, 43, 45, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 68, 70, 71, 72, 74, 75, 76, 86, 92, 96, 98, 100, 101, 102, 104, 108, 110, 112, 114, 116, 118, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 133, 134, 135, 136, 139, 140, 141, 142, 143, 144, 146, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 160, 162, 163, 164, 165, 167, 169, 170, 171, 172, 173, 175, 176, 177, 184, 188, 190, 191, 192, 193, 194, 196, 197, 198, 199, 201, 203, 214, 215, 216, 219, 221, 223, 225, 232, 234, 236, 241, 244, 245, 246, 247, 251, 35a, 35b, 35c, 39a, 43a, 43b, 53a, 58a, 64a, 64b, 64d, 74a, 74b, 128a, 149a, 149b, 149c, 149d, 214a), im gesamten Rastergebiet **3914 Adressobjekte mit Hausnummer / 453 verschiedene Hausnummern**; im Raster tragen 1933 Objekte `name=` und 719 Objekte `operator=`. **Hausnummer 245 existiert**: way/256942448 mit `addr:housenumber=245`, `addr:street=Riesaer Straße`, `building=yes`. **Hausnummer 247 existiert**: way/256942449 mit `addr:housenumber=247`, `addr:street=Riesaer Straße`, `building=yes`. Weitere Objekte mit Bezug zum Betrieb im Raster: **keines** - die Namenssuche nach „LKS“, „Katalysator“, „Edelmetall“, „Platin“ und „Palladium“ über die Felder name, operator, description und website findet **keinen** Treffer mit Betriebsbezug; node/4007614746 (`name=Volksbank`, `operator=Volksbank`) und die Objekte an der Straße Am Volksgut sind **andere** Betriebe. Die Dossier-Notation **245/247** löst sich in **zwei einzelne Hausnummern** auf, und **beide sind in OSM belegt**: way/256942448 mit `addr:housenumber=245`, `addr:street=Riesaer Straße`, `building=yes` und way/256942449 mit `addr:housenumber=247`, `addr:street=Riesaer Straße`, `building=yes` - beide **ohne** `name=`- und **ohne** `operator=`-Tag. Ein Objekt mit Betriebsbezug existiert im Raster **nicht**; die Namenssuche nach „LKS“, „Katalysator“, „Edelmetall“, „Platin“ und „Palladium“ ergibt im Raster keinen Treffer. Die Straße Riesaer Straße trägt im Raster **165 Adressobjekte / 142 verschiedene Hausnummern**. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, Kacheln live abgerufen) ; https://api.openstreetmap.org/api/0.6/<objekt>.json ; read-only `public.db` (31.10.2026).]
- [Recherche 31.10.2026: **Amtliche Bestätigung - die Website-Sperre wird durch das Register aufgefangen.** Der Direktabruf von lks-kat.de liefert weiterhin **HTTP 403** (Bot-Block; Root, /kontakt/, /katalysator-recycling/ und /unternehmen/ am 31.10.2026 erneut geprüft), `website_status: blockiert` bleibt damit zutreffend. **Im amtlichen Register** (Fachbetrieberegister / Register der Entsorgungsfachbetriebe, Abruf 11.08.2026) ist **„LKS Kat GmbH“ mit der Anschrift Riesaer Straße 245-247, 04319 Leipzig** geführt, **Zertifikatsnummer ZZFE00100296**, zertifizierte Tätigkeiten **Lagern / Behandeln / Verwerten**, Abfallschlüssel **160801** (gebrauchte Katalysatoren, die Gold, Silber, Rhenium, Rhodium, Palladium, Iridium oder Platin enthalten). Das Register bestätigt die Dossier-Adresse **amtlich und in der Bereichsnotation 245-247** - sie ist damit unabhängig vom gesperrten Webauftritt gedeckt. Der im Dossier als Einzelbeleg geführte Registerwert „Northdata HRB 31888“ und die Telefonnummer 0341 26488134 bleiben davon unberührt. **Hinweis:** das Register nennt **weder Telefon noch E-Mail** und **keine Preise**. Quelle(n): https://schrottregister.pages.dev/betrieb-lks-kat-gmbh-leipzig ; https://lks-kat.de/ (HTTP 403, Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise und Gebühren getrennt.** ANKAUF: die über den Suchindex sichtbaren Inhalte von lks-kat.de/katalysator-recycling/ und /unternehmen/ nennen **keine Zahlenpreise**, sondern die Preisart: **„Jeder Typ von Katalysator wird verwertet und zum Pauschalpreis pro Stück oder nach individueller Edelmetallanalyse angekauft.“** Das ist eine **Preisformel ohne Wert** - „Pauschalpreis pro Stück“ und „individuelle Edelmetallanalyse“ sagen nichts über die Höhe; die im Suchindex sichtbaren Kennzahlen „Wiederverwertungsanteil eines durchschnittlichen 6kg Katalysators“ und „Rückgewinnungsquote für Platin“ sind **ohne Zahlenwert** dargestellt und damit nicht belegbar. VERKAUF: die Materialien Platin, Palladium und Rhodium gehen zurück in die Industrie; **eine Verkaufs- oder Rückvergütungspreisliste wird nicht veröffentlicht**. GEBÜHREN: **keine** Entsorgungsgebührenordnung öffentlich; die Seite nennt die kostenfreie Rücknahme per „Übernahmeschein oder Entsorgungsnachweis“, aber keinen Katalogwert. **Bedingung statt Preis:** „Kleinstmengen an Katalysatoren von Privatpersonen oder Schraubern werden nur an unserem Standort in Leipzig entgegengenommen“ - die Annahme ist standortgebunden und nicht ortsungebunden; die Abholung erfolgt durch eigenes Fachpersonal, die Bezahlung **bar bei Warenübergabe**. **Weitere belegte Kontakte** (Suchindex, nicht Direktabruf): Telefon Wareneinkauf **0171/8499282** sowie die Fremdsprach-Telefonnummern +33 6 50 90 98 45 und +48 789 191 567 (Exportkontakte) und weber@lks-kat.de. Quelle(n): https://schrottregister.pages.dev/betrieb-lks-kat-gmbh-leipzig ; https://lks-kat.de/katalysator-recycling/ ; https://lks-kat.de/unternehmen/ ; https://lks-kat.de/kontakt/ (Direktabruf durchgehend HTTP 403 - Bot-Block; die Inhalte stammen aus dem Suchindex, Abruf 31.10.2026).]

### Geokodierung 31.10.2026

- [31.10.2026 (Geokodierung): **Koordinate gesetzt**: `lat`/`lon` = **51.3497305/12.501512** (Mittel ueber die 5 Knoten-Referenzen der Way-Definition von way/256942448, der Schliessknoten ist doppelt gezaehlt; der Punkt liegt innerhalb der Objekt-BBox 51.3495311/12.5009029 bis 51.3498635/12.5019392 und zusaetzlich innerhalb des Industriegebiets way/679246562 mit `landuse`=industrial). Quelle 1 (OSM): way/256942448 traegt `addr:housenumber`=245, `addr:street`=Riesaer Straße, `addr:city`=Leipzig; ein `addr:postcode` fehlt am Objekt, die PLZ steht an den Nachbarobjekten way/551894237 mit `addr:housenumber`=246 und way/551894239 mit `addr:housenumber`=244, beide mit `addr:postcode`=04319. Die Dossier-Notation 245/247 loest sich in zwei getrennte Objekte auf, way/256942448 (245) und way/256942449 (247), die rund 63 m auseinanderliegen und weder `name` noch `operator` tragen; gewaehlt ist das Objekt der ersten Nummer der Bereichsangabe. Welches der beiden Gebaeude den Betrieb traegt, ist damit nicht entschieden, der Punkt bezeichnet die belegte Anschrift und keine verifizierte Zufahrt. Quelle 2 (Betreiber): das amtliche Fachbetrieberegister fuehrt "LKS Kat GmbH" mit der Anschrift "Riesaer Straße 245-247, 04319 Leipzig" (Zertifikatsnummer ZZFE00100296, Taetigkeiten Lagern/Behandeln/Verwerten) und deckt damit die Bereichsangabe unabhaengig von OSM ab. Der bisher gespeicherte Produktionspunkt 51.354503225806454/12.369174193548387 liegt rund 9,2 km entfernt und wird durch diesen Punkt ersetzt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/256942448/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=12.494,51.345,12.509,51.354 ; https://schrottregister.pages.dev/betrieb-lks-kat-gmbh-leipzig]
