---
slug: nw-hagen-zimahmo
lat: 51.3339207
lon: 7.4744235
name: ZIMAHMO
trader_type: mobil
state: NW
city: Hagen
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

# ZIMAHMO

## Überblick

Der Live-Auftritt `zimahmo-schrotthandel.de` benennt „ZIMAHMO Schrotthandel“ und Lies Boumaza als Einzelunternehmer und führt Selbecker Str. 213a, 58091 Hagen, Telefon 0176 70295506 sowie eine E-Mail-Adresse. Ein 11880-Profil mit „Vom Inhaber bestätigt“ spiegelt dieselben Angaben, ist aber ein Branchenverzeichnis und keine unabhängige zulässige Zweitquelle. Daher bleiben die Kontakt-/Adressangaben Recherchebelege und die Stammdatenfelder leer; der alte, nach Everswinkel rückwärts aufgelöste GEO-Punkt bleibt durch `clear_fields: 'geo'` entzogen.

**Ankauf:** der Betreiberauftritt wirbt mit kostenfreier Schrottabholung für Privat und Gewerbe in NRW und einer Abholprämie bei großen Mengen; Höhe, Materialpreise und Schwelle nur auf Anfrage, kein Zahlenkurs. **Verkauf:** keine Produkt- oder Verkaufspreisliste. **Gebühren:** Demontage und Entrümpelung werden als „Preis auf Anfrage“ angeboten; keine bezifferte Gebührenliste. Selbecker Str. 213a wäre erst nach unabhängiger Identitäts-/Standortbestätigung ein Geokodierungskandidat; keine Koordinate übernommen.

## Timeline

### Recherche 25.10.2026: Betreiberauftritt gefunden, Seed-Zuordnung offen

- [Recherche 25.10.2026: Die aktuelle ZIMAHMO-Startseite und das Impressum direkt gelesen. Die Betreiberseite nennt Lies Boumaza, „ZIMAHMO Schrotthandel (Einzelunternehmer)“, Selbecker Str. 213a, 58091 Hagen, 0176 70295506 und info@zimahmo-schrotthandel.de; sie beschreibt mobile Schrottabholung, Demontage und Entrümpelungen. Das 11880-Profil führt dieselben Angaben und markiert sie „Vom Inhaber bestätigt“, bleibt nach Quellenstandard jedoch ein Aggregator und kein unabhängiger Zweitbeleg. Da Rechts-/Betreiberkette zum Seed nicht anderweitig bestätigt ist, keine Adresse/Kontakte in Frontmatter übernommen und `status: pruefung` beibehalten. **Ankauf:** kostenloser Abholservice behauptet; für große Mengen wird eine nicht bezifferte Abholprämie angeboten, individuelle Offerte auf Anfrage. **Verkauf:** keine öffentliche Produktpreisliste. **Gebühren:** Demontage/Entrümpelung „Preis auf Anfrage“. Adresse nur GEO-Kandidat nach unabhängiger Zuordnung. Der alte falsche GEO-Punkt wird separat mit `clear_fields: 'geo'` entfernt. Quellen: https://zimahmo-schrotthandel.de/ ; https://zimahmo-schrotthandel.de/index.php/impressum/ ; https://www.11880.com/branchenbuch/hagen-westfalen/060440092B108774987/zimahmo-schrotthandel.html .]

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-zimahmo` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. `clear_fields: 'geo'` zieht den falschen Punkt zurück; mangels belegter Händleradresse wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Hagen

### Recherche 03.10.2026

- [Recherche 03.10.2026: Kein eindeutig zuordenbarer Betreiberauftritt, Impressum, Register- oder Kommunalbeleg zu „ZIMAHMO“ in Hagen verifiziert; die Websuche lieferte keine belastbare Identitäts-/Standortquelle. NorthData-Namenssuche zeigte nur unscharfe, unpassende „Mahmoud“-Treffer ohne Hagen-Bezug (keine negative Registerauskunft). Deshalb keine Frontmatter-Fills, keine Telefonnummer, Adresse oder Koordinaten geraten; status bleibt pruefung, ohne Schließungsbehauptung. Quelle(n): keine belegfähige Quelle; Suchprotokolle (kein Sachbeleg): https://www.bing.com/search?q=%22ZIMAHMO%22+Hagen; https://www.northdata.de/?query=ZIMAHMO]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt**: `lat`/`lon` = **51.3339207/7.4744235** (Mittel der **10**
  eindeutigen Knoten von way/139550294; die BBox der eindeutigen Knoten reicht von 51.3337847/7.4742843 bis
  51.3340095/7.4745375 und **umschließt den Mittelwert**). Quelle 1 (OSM): way/139550294 trägt `addr:housenumber`=213a,
  `addr:street`=Selbecker Straße, `addr:city`=Hagen, `addr:postcode`=58091 und `building`=yes (live geladen am
  04.11.2026, HTTP 200); das Objekt benennt den Betrieb **nicht**, der Punkt bezeichnet daher die belegte Anschrift und
  keine verifizierte Zufahrt, Waage oder Annahmefläche. Das am 04.11.2026 live abgerufene Kachelraster über Hagen-Eilpe
  (7.4500/51.3100 bis 7.5000/51.3600, **25 Kacheln von je 0,01 Grad, 0 Kachel-Fehler**) erfasst die **Selbecker Straße**
  mit **239** Adressobjekten und **223** verschiedenen Hausnummern (1–288, darunter 101a, 103a, 111a, 114a, 120a, 129a,
  131a, 132a, 138a, 140a, 151a, 151b, 153a, 154a, 158a, 161a, 162a, 166a, 167a, 169a, 179a, 185a, 187a, 187b, 187c,
  187d, 189a, **213a**, 214a, 216a, 239a, 239b, 239d, 239e, 240a, 240b, 240c, 240d, 240e, 241, 242, 243, 244, 246, 248,
  248a, 250a, 252a, 261, 263, 267, 267a, 270, 271, 272, 272a, 274, 282, 288) - die **Dossier-Hausnummer 213a existiert**
  und ist im Raster das einzige Objekt mit dieser Nummer auf dieser Straße (die Nachbarnummer 213 trägt ein Restaurant
  „Rosengarten“); eine Namenssuche nach **„ZIMAHMO“** und **„Boumaza“** über die Felder name, operator, description,
  website, contact:website, brand, note und addr:housename findet im Raster **keinen** Treffer, es gibt also **keine**
  OSM-Betriebszuordnung. Quelle 2 (Betreiber): das **Impressum** der Betreiberseite
  https://zimahmo-schrotthandel.de/index.php/impressum/ nennt **Lies Boumaza, ZIMAHMO Schrotthandel (Einzelunternehmer),
  Selbecker Str. 213a, 58091 Hagen**, Telefon +49 (0)176 702 955 06 und E-Mail info@zimahmo-schrotthandel.de (live
  abgerufen 04.11.2026); die Startseite https://zimahmo-schrotthandel.de/ bewirbt mobile Schrottabholung für Privat und
  Gewerbe in NRW, Demontage und Entrümpelungen. Der im Dossier bereits zitierte 11880-Profilspiegel („Vom Inhaber
  bestätigt“) bleibt ein Aggregator und ist kein unabhängiger Zweitbeleg; die Betreiber-Primärquelle trägt die Adresse
  allein. Der bisher gespeicherte Produktionspunkt **51.90933/7.88825** (Mehringen, Kreis Warendorf, laut
  read-only-Abgleich vom 24.10.2026) wird durch diesen Punkt ersetzt. `clear_fields: 'geo'` **entfällt**, weil die
  Koordinate nun belegt vorliegt; `status: pruefung`, `street`, `postcode`, `phone`, `email`, `website`,
  `website_status`, `description`, `dropoff_json` und `pickup_json` bleiben unverändert - insbesondere bleibt der
  Eintrag ein **Prüfdatensatz**, weil ein zweiter unabhängiger Betreiber-/Registerbeleg zur Seed-Zeile fehlt und die
  Zustellart des mobilen Dienstes nicht als öffentliche Annahmestelle belegt ist. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/139550294/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.4500,51.3100,7.5000,51.3600 ; https://zimahmo-schrotthandel.de/
  ; https://zimahmo-schrotthandel.de/index.php/impressum/ (Abruf 04.11.2026).]
