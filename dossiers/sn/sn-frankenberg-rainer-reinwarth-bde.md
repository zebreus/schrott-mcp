---
slug: sn-frankenberg-rainer-reinwarth-bde
lat: 50.9048037
lon: 13.0317228
name: Rainer Reinwarth BDE
trader_type: sonstige
state: SN
city: Frankenberg
street: ''
postcode: ''
phone: ''
clear_fields: 'dropoff_json,phone'
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Rainer Reinwarth BDE

## Überblick

„Rainer Reinwarth BDE“ bleibt ein Verzeichniskandidat. Am 18.10.2026 gelesene Gelbe-Seiten-Detailseite und Örtliche-Rückwärtssuche nennen Hohe Str. 31, 09669 Frankenberg/Sa. und 037206 70542, keine ausreichende aktuelle Betreiber-/Registerbestätigung oder bestätigte Annahmestelle. lokaleschrottplatz ist hier nur Alt-Timeline, nicht heute gelesen. Straße/PLZ/Telefon im Dossier als unzureichend belegte aktuelle Fakten zurückgenommen; keine Widerlegung der Adresse/Nummer. Status `pruefung`, kein Schließungsbefund. Die alte Produktionskoordinate und Annahmezusage sind nicht dem Kandidaten zugeordnet und werden beim nächsten regulären Seed ausdrücklich zurückgenommen; es gibt keinen Ersatzpunkt und keine Aussage über tatsächliche Annahme oder Schließung.

- **Ankauf:** keine aktuelle Annahme oder bezifferten Kurse bestätigt.
- **Verkauf:** keine diesem Kandidaten bestätigte Preisliste.
- **Gebühren:** keine bestätigten Tarife.
- **Geokodierung:** kein Betreiberstandort belastbar belegt; `clear_fields: geo` zieht den nicht zugeordneten Altpunkt beim nächsten regulären Seed zurück. Status `pruefung` zieht alte Servicebehauptungen nicht automatisch zurück; dafür wird `dropoff_json` separat geleert.

## Timeline

### Recherche 27.10.2026: amtlicher Registerabgleich Frankenberg

- [Recherche 27.10.2026: Amtlicher Fachbetrieberegister als neue Gegenquelle herangezogen (GESA/eEFBV-Auszug, Stand 11.08.2026): Die Ortsseite „Frankenberg" fasst beide gleichnamigen Orte zusammen (Postleitzahlen 09669 Frankenberg/Sa. und 35066 Frankenberg/Eder) und führt drei zertifizierte Betriebe: Autoverwertung Hallenberger e. K. (Siegener Str. 10), Moritz Damm & Sohn KG (Äußere Chemnitzer Str. 49) und Volker Lungwitz Schrotthandel e. K. (Mühlenstraße 7). Ein Eintrag „BDE Dienstleistungen Reinwarth Rainer" oder die Hohe Str. 31 ist nicht enthalten. Das Register enthält nur zertifizierte Metall- und Fahrzeugbetriebe; das Fehlen belegt weder Nichtexistenz noch Geschäftsaufgabe, es existiert aber kein EFB-/Demontage-Nachweis für den Kandidaten; Quelle(n): https://schrottregister.pages.dev/ort-frankenberg]
- [Recherche 27.10.2026: Der Namensvetter für „Reinwarth" bleibt negativ: Eine bundesweite Suche nach Reinwarth führt weiterhin nur fach- bzw. ortsfremde Treffer ohne Bezug zu Hohe Str. 31, und die kommunale Firmendatenbank Frankenberg/Sa. liefert bei der abrufbaren Suchmaske keine auswertbare Trefferliste — das ist kein amtlicher Negativnachweis. **Ankauf:** keine Annahmebedingungen oder Euro-Kurse belegt; **Verkauf:** keine Liste; **Gebühren:** keine Tarife. Straße, Postleitzahl und Telefon bleiben weiterhin nicht belegt und leer; die Clear-Regel (`phone`, `dropoff_json`, `geo`) und `status: pruefung` bleiben unverändert, es wird weder eine Koordinate geraten noch eine Schließung behauptet; Quelle(n): https://www.frankenberg-sachsen.de/Wirtschaft/firmendatenbank/ ; https://www.gelbeseiten.de/gsbiz/c4323bb5-5bdf-44ae-acc8-998d3cd7db95]

### Recherche 21.10.2026

- [Recherche 21.10.2026: Gelbe-Seiten-Einzelseite und Das-Örtliche-Rückwärtssuche live gelesen: beide zeigen „BDE Dienstleistungen Reinwarth Rainer Metall Kabel Schrott“, Hohe Str. 31, 09669 Frankenberg/Sa., 037206 70542. Das sind weiterhin Branchenverzeichnis-Leads und weder ein Betreiber-/Registerbeleg noch zwei zulässige unabhängige Nachweise für aktuelle Erreichbarkeit/Annahme. Die kommunale Firmensuche liefert bei der abrufbaren Suchmaske keine individuell auswertbare Trefferliste; das ist kein amtlicher Negativnachweis. Keine Ersatzkontakte oder Betriebsfortführung abgeleitet. ANKAUF: keine verifizierten Annahmebedingungen/Euro-Kurse; VERKAUF: keine Liste; GEBÜHREN: keine Tarife. Vorhandene Clear-Regel/Frontmatter nicht geändert. Quellen (Leads/Prüfroute): https://www.gelbeseiten.de/gsbiz/c4323bb5-5bdf-44ae-acc8-998d3cd7db95 ; https://www.dasoertliche.de/?form_name=search_inv&ph=03720670542 ; https://www.kreischa.de/inhalte/kreischa/_inhalt/wirtschaft/firmen-neu/gewerbesuche .]

### Owner-Korrektur 22.10.2026

- [Owner-Korrektur 22.10.2026: Eine frische read-only-Abfrage der exakten Produktionszeile fand weiterhin `dropoff_json={"allowed":true}` und GEO 50.9259/13.0596. Die dokumentierten Reviews vom 18. und 21.10. fanden nur Verzeichnis-Leads, keine aktuelle Betreiber-/Registerbestätigung, keine bestätigte Annahmestelle und keine Zuordnung dieses Punkts zum Händler. Das beweist weder eine falsche Adresse noch eine Annahmeverweigerung; es reicht aber nicht, die positive Pin- und Annahmebehauptung weiterzuführen. Owner zieht deshalb ausschließlich `dropoff_json` und das alte GEO-Paar via `clear_fields: 'dropoff_json,geo'` beim regulären Seed zurück; `phone`-Clear bleibt bestehen. Ergebnis ist unbekanntes `{}` und GEO `NULL`, nicht `allowed:false`, Schließung oder Ersatzkoordinate. Quellen/Evidenz: `research/checkin-20261018-0930-review-reinwarth.md`; `research/checkin-20261018-1331-release-gates.md`; Recherche 21.10.2026 oben; read-only Produktionsabfrage vom 22.10.2026.]

### Importiert (Seed-Stand 2026-09-30)

- Fe/Alu/Cu/Edel/Blei/Messing
- Annahme lt. lokaleschrottplatz

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — Lead Frankenberg, Hohe Str. 31 nur Verzeichnisbeleg (lokaleschrottplatz, Einzelbeleg, unsicher); keine Website gefunden; keine Frontmatter-Fills; Quelle(n): recherche/sn.md]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Gelbe Seiten fuehrt BDE Dienstleistungen Reinwarth Rainer Metall Kabel Schrott, Hohe Str. 31, 09669 Frankenberg/Sa., Tel. 037206 70542, deckungsgleich mit Vorwellen-Lead; Strasse/PLZ/Telefon gefuellt; keine Betreiber-Website gefunden; Status bleibt aktiv auf doppelter Verzeichnisbasis; Quelle(n): Gelbe Seiten Treffer Frankenberg plus lokaleschrottplatz-Lead aus Vorwelle]

### Korrektur 18.10.2026

- [Korrektur 18.10.2026: Gelbe-Seiten-Detail/Ortssuche sowie Örtliche-Telefonrückwärtssuche heute gelesen: BDE Dienstleistungen Reinwarth Rainer Metall Kabel Schrott, Hohe Str. 31, 09669 Frankenberg/Sa., 037206 70542 bleiben Verzeichnisleads; keine aktuelle Nummerninhaberschaft/Erreichbarkeit bestätigt. Bundesweite Namenssuche liefert andere fach-/ortsfremde Reinwarth-Leads, keine belegte Nachfolge; Adressgegenseite nicht abschließend geklärt. Kommunale freiwillige Firmendatenbank liefert bei lesenden Suchabfragen Reinwarth/BDE/Hohe keine Treffer, kein amtlicher Gewerberegisterauszug oder Nichtexistenzbeweis. NorthData-Abruf nur allgemeine Suchseite, kein konkreter Registerbeleg; lokaleschrottplatz heute nicht abgerufen. Straße/PLZ/Telefon im Frontmatter zurückgenommen, clear_fields phone eng begrenzt freigegeben, aktiv → pruefung. Keine Ersatzdaten, Schließung oder zugeordneten Preis-/Gebührenbeträge. Historische 01./02.10.-Abrufe nicht rückwirkend verifiziert, uncommitteter 12.10.-Entwurf belegt keinen damaligen Abruf. Ungeprüftes erhaltenes GEO bleibt Veröffentlichungsblocker, keine GEO-Aktion. Quellen: https://www.gelbeseiten.de/gsbiz/c4323bb5-5bdf-44ae-acc8-998d3cd7db95 ; https://www.gelbeseiten.de/suche/reinwarth/frankenberg%20sa ; https://www.dasoertliche.de/?form_name=search_inv&ph=03720670542 ; https://www.gelbeseiten.de/suche/reinwarth/bundesweit ; https://www.frankenberg-sachsen.de/Wirtschaft/firmendatenbank/]

### Owner-Korrektur 18.10.2026 (Seed-Verhalten)

- [Owner-Korrektur 18.10.2026: Heutiger mode=ro/query_only-Abgleich mit public.db zeigt Telefon 037206 70542, Hohe Str. 31/09669 und Status aktiv weiterhin gespeichert; Bestandsaufnahme, keine sachliche Verifikation oder Bestätigung der behaupteten 12.10.-Abfrage. Nach Integration/Rebuild entfernt clear_fields phone das Telefon und bleibt bei jedem Seed gegen spätere Drift autoritativ, auch bei gleichem Seed-Hash. Kernadresse/Status werden beim geänderten Seed aktualisiert. Fehlende Dossierkoordinaten behalten das DB-Paar 50.9259/13.0596 unverändert und unvalidiert. Zusätzlich gespeichertes dropoff_json allowed true bleibt bei leerem Dossierfeld ebenfalls erhalten, durch pruefung weder gelöscht noch bestätigt; gesonderte qualifizierte Bereinigung offen. Keine zusätzlichen Clear-Tokens freigegeben. Ungeklärte Identität/Punktzuordnung bleibt Veröffentlichungsblocker; keine Produktions-/GEO-Mutation dieses Check-ins. Grundlage docs/trader-seed.md und heutige Abfrage /tmp/opencode/checkin-20261018-0930-production-gate.json.]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **50.9048037/13.0317228** (Mittel der **4**
  eindeutigen Knoten von way/386317391; die BBox der eindeutigen Knoten reicht von 50.9047290/13.0315967 bis
  50.9048784/13.0318488 und **umschließt den Mittelwert**). Quelle 1 (OSM, live geladen am 04.11.2026, HTTP 200):
  **way/386317391** trägt `addr:street`=Hohe Straße, `addr:housenumber`=**31**, `addr:city`=Frankenberg/Sa.,
  `addr:postcode`=09669 sowie `building`=**apartments**, `building:levels`=3, `roof:levels`=1 - Straße, Hausnummer,
  PLZ und Ort stimmen mit der Dossier-Adresse **Hohe Str. 31, 09669 Frankenberg/Sa.** überein. **Fensterprüfung (kein
  Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster umfasst **13.0200/50.9000 bis 13.0500/50.9200**,
  **6 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**; das Fenster enthält die gesuchte Straße **nachweislich** mit den
  Straßenobjekten **way/25443806**, **way/1208328774** und **way/1208328775** sowie **20** Adressobjekten auf
  „Hohe Straße" mit **20** verschiedenen Hausnummern (2, 5, 7, 9, 11, 13, 15, 16, 20, 21, 24, 28, 29, 30, **31**, 31a,
  35, 36, 37, 38) - die **Dossier-Hausnummer 31 existiert** und ist im Raster das einzige Objekt mit dieser Nummer
  (31a ist ein eigenes Nachbarobjekt). Eine Namenssuche nach **„Reinwarth"** und **„BDE"** über die Felder name,
  operator, description, website, contact:website, brand, note und addr:housename findet im Raster **keinen** Treffer -
  es gibt also **keine** OSM-Betriebszuordnung, nur die Adressangabe. Quelle 2 (Verzeichnis, im Dossier bereits
  zitiert, live nachgelesen am 04.11.2026): die **11880-Verzeichnissuche** liefert **HTTP 200** und führt live **„BDE
  Dienstleistungen Reinwarth Rainer Metall Kabel Schrott, Altmaterialhandel, Schrotthandel & Altmetallrecycling, Hohe
  Str. 31, 09669 Frankenberg, Sachsen, (037206) 70542"**; das im Dossier dokumentierte **Gelbe-Seiten-Profil** nennt
  dieselbe Anschrift und Nummer. **Zurücknahme des `geo`-Tokens (Begründung):** `clear_fields: 'dropoff_json,geo,phone'`
  wurde um das Token **geo** gekürzt (die Tokens `dropoff_json` und `phone` bleiben **unverändert** bestehen), weil es
  den nicht zuordenbaren Produktionspunkt fernhalten sollte; der hier gesetzte Punkt liegt **exakt** auf der belegten
  Anschrift, während `clear_fields` nicht gleichzeitig `geo` neben belegtem `lat`/`lon` stehen darf.
  **Offener Zweitstandort (unverändert übernommen):** dieselbe 11880-Suche führt zusätzlich **„BDE Dienstleistungen
  Reinwarth Rainer Schrott Metalle A&V, Altmaterialhandel & Schrotthandel, Am Chemnitzberg 17, 09328 Lunzenau
  (Göritzhain), (037383) 61701"** - eine **zweite** Anschrift desselben Namens in einem anderen Ort; die Dossierzeile
  bleibt der Frankberger Standort, eine Gleichsetzung der beiden Anschriften wird **nicht** behauptet.
  **Qualifizierung (unverändert übernommen):** Der Punkt bezeichnet die **verzeichnisgestützte Anschrift**, nicht
  einen verifizierten Annahmehof - das OSM-Objekt ist als **Wohnhaus** erfasst, ein Betriebs- oder Recycling-POI
  existiert dort nicht, und es wird weder Fortbestand noch Schließung behauptet. `status: pruefung`, `city`,
  `street`, `postcode` (leer), `phone` (leer), `email`, `opening_hours`, `website`, `website_status` und `description`
  bleiben unverändert. Quelle(n): https://api.openstreetmap.org/api/0.6/way/386317391/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=13.0200,50.9000,13.0500,50.9200 ;
  https://www.11880.com/branchenbuch/frankenberg-s-a/060442240B110998711/rainer-reinwarth-bde.html (Abruf 04.11.2026).]
