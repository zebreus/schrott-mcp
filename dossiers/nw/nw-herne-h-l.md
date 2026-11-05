---
slug: nw-herne-h-l
name: H&L
trader_type: mobil
state: NW
city: Herne
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

# H&L

## Überblick

Der Verzeichniseintrag ist nach wie vor der einzige Beleg: 11880 fuehrt **"Schrottabholung H&L"**, Hauptstr. 379, 44649 Herne (Wanne), **(0163) 1977700**, mit den Kategorien Haushaltsauflösung, Schrotthandel und Entsorgungsdienst. Die Telefonnummer war bisher nicht bekannt. Betreiberwebsite, Impressum, E-Mail und Registertreffer fehlen; alle Nennungen stammen aus einem Verzeichnis.

**Ankauf:** nicht belegt. **Verkauf:** keine Liste. **Gebuehren:** keine Angaben. **Geokodierung:** Die Hausnummer 379 existiert in OSM, allerdings mit der PLZ 44653 statt der im Verzeichnis genannten 44649.

## Timeline

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall, nur Portal-Einzelnachweise — 11880 listet Schrottabholung H&L, Hauptstr. 379, 44649 Herne (Schrotthandel/Entsorgung/Haushaltsauflösung, ohne Öffnungszeiten); keine Betreiber-Website, kein Registerbeleg gefunden. Alle Nennungen ein Portal (= eine Quelle) → KEINE Fills, Status bleibt pruefung. Lead für Folgewelle. Quelle(n): 11880 (Lead, Einzelbeleg)]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Herne

### Recherche 29.10.2026

- [Recherche 29.10.2026: Der 11880-Eintrag wurde erneut gelesen und liefert jetzt die previously fehlende Telefonnummer: "Schrottabholung H&L, Hauptstr. 379, 44649 Herne (Wanne), (0163) 1977700", Kategorien Haushaltsaufloesung, Schrotthandel und Entsorgungsdienst, weiterhin ohne Oeffnungszeiten, ohne E-Mail und ohne Website. Der Eintrag wurde laut 11880 am 13.10.2026 aktualisiert, die Eintragsdaten stammen vom 15.05.2024. Es gibt weiterhin keine Betreiberwebsite, kein Impressum und keinen Registertreffer; alle Nennungen stammen aus dem 11880-Netz, also aus einer Quelle, daher kein Fill und Status pruefung bleibt. Die Branchenseite Herne der Gelben Seiten fuehrt mehrere andere Schrotthaendler (Schrott Abholen NRW, Schrottfuesche, SOS.SCHROTTHANDEL, Schrottankauf Exclusiv, Schrottankauf Fix, Schrottabholung J&H Ruhr in Herten, Gläser Heiko in Recklinghausen, Alqemal Muhammadeid in Herne) aber kein H&L, ein zweiter Treffer fuer diesen Namen existiert in Herne nicht. OSM live geprueft: way/330044882 traegt addr:housenumber=379, addr:street=Hauptstraße, addr:city=Herne, addr:postcode=44653, building=apartments; die Vorwaerts-Nominatim-Suche nach "Hauptstraße 379, 44649 Herne" liefert genau dieses Objekt mit der Anzeige "379, Hauptstraße, Unser Fritz/Crange, Wanne-Bickern, Wanne, Herne, 44653". Die Hausnummer existiert also, aber OSM nennt die PLZ 44653 waehrend das Verzeichnis 44649 nennt - dieser PLZ-Konflikt bleibt offen und wird hier ausdruecklich als ungeklaert gefuehrt; das Objekt traegt keinen Name-, operator- oder Gewerbe-Tag. Die Ortsseite Herne des Registerabzugs aus Fachbetrieberegister und Register der Entsorgungsfachbetriebe (Stand 11.08.2026) fuehrt 8 Betriebe und kein H&L; das Register erfasst Entsorgungsbetriebe, der Negativbefund gilt nur fuer dieses Register. ANKAUF: nicht belegt; VERKAUF: keine Liste; GEBUEHREN: keine Angaben. Quellen: https://www.11880.com/suche/haushaltsaufloesung/herne ; https://www.11880.com/branchenbuch/herne-westfalen/120674719B112766065/schrottabholung-h-l.html ; https://api.openstreetmap.org/api/0.6/way/330044882.json ; https://nominatim.openstreetmap.org/search?q=Hauptstra%C3%9Fe+379%2C+44649+Herne&format=jsonv2&addressdetails=1 ; https://schrottregister.pages.dev/ort-herne]

### Recherche 05.11.2026

- [Recherche 05.11.2026: **Amtliches Fachbetrieberegister (E-15) am 05.11.2026 erneut geprüft: für „Schrottabholung H&L“ in Herne kein dem Dossierstandort zugeordneter Registereintrag
  (Negativaussage mit Reichweitenklausel).** Geprüft wurden beide Routen des amtlichen Fachbetrieberegisters (https://fachbetrieberegister.gadsys.de/fachbetrieberegister/),
  Route „Entsorgungsfachbetriebe“ und Route „Altfahrzeugverwertung“, durchgängig mit dem Filter „Art des Betriebs = Egal“ (is=1&e=1); die Standortabfragen liefen mit
  validiertem adr-Parameter, der die Adresse „Hauptstraße 379, 44649 Herne“ samt PLZ-Abgleich enthält (Punkt 7.1575589/51.5412298, per OSM/Nominatim auf Hauptstraße 379
  aufgelöst; dabei weicht die OSM-PLZ 44653 von der Verzeichnis-PLZ 44649 ab, die Abfrage lief trotzdem über die Dossieranschrift), in den Radien 3 km, 5 km und 10 km.
  **Namenssuche Route „Entsorgungsfachbetriebe“:** „H&L“ 0 Treffer, „Schrottabholung H&L“ 0, „Schrottabholung H & L“ 0, „H&L Schrott“ 0, „H&L GmbH“ 0, „Schrottabholung H&L
  GmbH“ 0, „H & L GmbH“ 159 Treffer (Namensvettern ohne Herne-Bezug) sowie „H & L“ 500 Treffer; die 500 Treffer sind das Anzeige-Maximum der Anwendung, die Trefferliste ist
  damit nicht vollständig darstellbar. **Route „Altfahrzeugverwertung“:** „H&L“ 0, „Schrottabholung H&L“ 0, „Schrottabholung H & L“ 0, „H&L Schrott“ 0, „H&L GmbH“ 0,
  „Schrottabholung H&L GmbH“ 0, „H & L GmbH“ 5 Treffer (namensfremde Autoverwerter) sowie „H & L“ 31 Treffer, darunter „R & M Schrott- und Metallhandels GmbH & Co. KG“,
  „Hagl Recycling GmbH & Co. KG“, „Borowski & Hopp GmbH & Co KG“ und „Autoverwertung H&S Naumann GbR“; keiner davon in Herne. **Standortabfrage:** Route
  „Entsorgungsfachbetriebe“ 3 km → 82 Treffer, 5 km → 100 Treffer, 10 km → 199 Treffer; Route „Altfahrzeugverwertung“ 3 km → 3 Treffer, 5 km → 3 Treffer, 10 km → 5 Treffer.
  **Nachbarsuche (Namenssuche der geografisch benachbarten Treffer):** „HEITKAMP Umwelttechnik GmbH“ 5 Treffer (5 Anlagen, Wilhelmstraße (Haus 12) 98, 44649 Herne), „Paul
  Jost GmbH“ 5 Treffer (davon 2 Anlagen in Herne, Heerstraße 20, 44653 Herne) und „Wilhelm Bötzel GmbH & Co. KG“ 8 Treffer; keine dieser Nachbarnamen führt zu einem
  H&L-Eintrag, eine Umbenennung ist nicht erkennbar. **Abgrenzung:** Die Treffer im 3-km-Radius sind „Paul Jost GmbH“ (ZZEE00300165 / 2 und / 3, Heerstraße 20, 44653 Herne,
  0,7 km), „REMONDIS Industrie Service GmbH & Co. KG, NL Herne“ (ZZMT00300052 / 1-4, Heerstr. 29-43, 44653 Herne, 0,9 km), „GERA Metallhandel GmbH“ (ZZEE00300220 / 1-2,
  Hohewardstraße 349 A, 45699 Herten, 1,2 km) sowie „REMONDIS Ruhr GmbH“ (ZZET00700290 / 1-20, Hafenstr. 4a-b, 44653 Herne) und „Wilhelm Bötzel GmbH & Co. KG“ (ZZKT00100301
  / 4-7, Hafenstraße 5-15 und 2a, 44653 Herne). Der einzige Registereintrag in der Dossier-PLZ 44649 ist der „Stammsitz“ der HEITKAMP Umwelttechnik GmbH (Zertifikat
  B+U-11/2026, Vorgangsnummer ZZGE001000277010, 5 Anlagen, gültig bis 15.10.2027, Zertifizierungsorganisation Entsorgergemeinschaft Bauen und Umwelt e.V., Mainz, HRB 4722
  Bochum), Wilhelmstraße (Haus 12) 98, 44649 Herne, Entfernung 1,4 km: andere Firma, andere Straße, andere Tätigkeiten. **Adressdifferenz:** Das Verzeichnis nennt
  Hauptstraße 379, 44649 Herne, das Register führt die Hausnummer 379 nicht; der gefundene Eintrag in derselben PLZ liegt an der Wilhelmstraße. **Ergebnis:**
  Zertifikatsnummer, Vorgangsnummer, Zertifizierungsorganisation, Ausstellungsdatum, Gültigkeitsdatum, Anlagenanzahl, zertifizierte Tätigkeit und Standort sind für „H&L“ am
  Dossierstandort nicht ausgewiesen; das Feld `certifications` bleibt unverändert leer. **Reichweite der Aussage:** geprüft wurden die oben genannten Namensvarianten
  (Rechtsform-Pool und Inhaber-Namensvetter) in beiden Registerrouten (durchgängig mit dem Filter „Art des Betriebs = Egal“, is=1&e=1), die Standortsuche mit validiertem
  adr-Parameter in den Radien 3 km, 5 km und 10 km sowie die Namenssuche der benachbarten Treffer; Abrufdatum 05.11.2026; die Trefferzahlen der einzelnen Abfragen sind oben
  wörtlich genannt. Ein Register-Nichteintrag ist kein Nachweis dafür, dass ein Betrieb keine Anerkennung besitzt: die Efb-Anerkennung nach § 56 KrWG ist freiwillig, und
  der Dritt-Spiegel schrottregister.pages.dev wurde nicht als Beleg herangezogen.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=H%26L; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=H%26L; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Hauptstra%C3%9Fe%20379%2C%2044649%20Herne%3B7.1575589%3B51.5412298%3B7.169559%3B51.553230%3B7.145559%3B51.529230&dst=3&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Hauptstra%C3%9Fe%20379%2C%2044649%20Herne%3B7.1575589%3B51.5412298%3B7.169559%3B51.553230%3B7.145559%3B51.529230&dst=3&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Hauptstra%C3%9Fe%20379%2C%2044649%20Herne%3B7.1575589%3B51.5412298%3B7.169559%3B51.553230%3B7.145559%3B51.529230&dst=5&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Hauptstra%C3%9Fe%20379%2C%2044649%20Herne%3B7.1575589%3B51.5412298%3B7.169559%3B51.553230%3B7.145559%3B51.529230&dst=5&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Hauptstra%C3%9Fe%20379%2C%2044649%20Herne%3B7.1575589%3B51.5412298%3B7.169559%3B51.553230%3B7.145559%3B51.529230&dst=10&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Hauptstra%C3%9Fe%20379%2C%2044649%20Herne%3B7.1575589%3B51.5412298%3B7.169559%3B51.553230%3B7.145559%3B51.529230&dst=10&is=1&e=1]
