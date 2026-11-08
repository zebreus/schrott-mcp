---
slug: rp-trier-pfeil-heinz
lat: 49.7454006
lon: 6.6365441
name: Pfeil-Heinz
trader_type: sonstige
state: RP
city: Trier
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
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Pfeil-Heinz

## Überblick

„Pfeil-Heinz“ wird weiterhin nur durch Verzeichnisprofile mit Leostr. 6, 54290 Trier und 0170 6878799 konkretisiert. Das direkt gelesene 11880-Profil nennt Mo–Fr 08:00–17:30; ein weiteres Schrottplatzportal nennt Mo–Fr 08:00–17:00 und Annahme-Mindestgewichte 100 kg Fe/1 kg NE. Das Portal zeigt dynamisch „Geschlossen“, ohne dauerhaften Schließungsbeleg. Die Quellen sind keine Betreiber-/Registerbelege, und Stunden/Annahmebedingungen weichen voneinander ab.

Das frühere `street`-Feld enthielt den nicht normalisierten Komposittext „Pfeil-Heinz Trier Leostr. 6“; er wurde aus dem strukturierten Straßenfeld entfernt. Leostr. 6, 54290 Trier bleibt ein nicht verifizierter Verzeichnis-/Geocoding-Kandidat, keine bestätigte Betreiber- oder Annahmeanschrift.

**ANKAUF:** keine verifizierten Auszahlungskurse; Portal-Mindestgewichte sind nicht als Betreiberbedingungen belegt. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife. Keine Koordinate, bis Betreiber und Betriebs-/Annahmestelle bestätigt sind.

## Timeline

### Recherche 24.10.2026: nicht normalisierten Straßenwert bereinigt

- [Recherche 24.10.2026: Bei der Adressprüfung festgestellt, dass `street` bislang den Kompositwert „Pfeil-Heinz Trier Leostr. 6“ (Name + Ort + Straßenlead) statt einer normalisierten Straße enthielt. Den strukturierten Wert deshalb geleert; die vollständige bisherige Zeichenfolge hier erhalten. Die zuvor direkt gelesenen Verzeichnisprofile von 11880 und Lokaleschrottplatz nennen übereinstimmend Leostr. 6, 54290 Trier und 0170 6878799, bleiben aber Portal-Leads ohne bestätigte Betreiberidentität oder öffentliche Annahme. Leostr. 6 ist daher nur ein möglicher Geocoding-Kandidat nach Identitäts-/Standortprüfung, keine Koordinatenfreigabe. **Ankauf:** keine verifizierten Kurse; Portal-Mindestgewichte nicht bestätigt. **Verkauf:** keine Liste. **Gebühren:** keine Tarife. Quellen: https://www.11880.com/branchenbuch/trier/060440092B112767456/pfeil-heinz.html ; https://lokaleschrottplatz.de/pfeil-heinz/ ; frühere Kompositangabe: Seed-Frontmatter vor Korrektur und Importnotiz unten.]

### Owner-GEO-Prüfung 23.10.2026

- [Owner-GEO-Prüfung 23.10.2026: Read-only-Abgleich der Produktionszeile: gespeicherter Altpunkt 49.7534167, 6.6370833. Die aktuelle Recherche belegt für diesen Seed keinen adressgenauen, dem Betreiber und einer öffentlichen Annahmestelle zugeordneten Punkt; ein Ersatz wäre geraten. `clear_fields: 'geo'` entzieht das Alt-Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate“, nicht Schließung oder Annahmeverweigerung. Keine Ersatzkoordinate geraten, Produktionsdatenbank nicht geschrieben. Quellen: read-only-Abgleich `public.db` vom 23.10.2026; Rechercheeintrag 23.10.2026 mit den dort verlinkten Quellen; `docs/trader-seed.md`.]


### Recherche 23.10.2026

- [Recherche 23.10.2026: 11880-Profil und Lokaleschrottplatz-Seite direkt gelesen. Beide führen Leostr. 6, 54290 Trier und 0170 6878799; 11880 nennt Mo–Fr 08:00–17:30 (Profil aktualisiert am 04.10.2026), Lokaleschrottplatz Mo–Fr 08:00–17:00, Sa/So geschlossen, Statusanzeige „Geschlossen“ und Mindestgewichte 100 kg Eisen/1 kg NE. Das sind Portalangaben, nicht unabhängig bestätigte Betreiberbedingungen; die unterschiedlichen Stunden und dynamische Statusanzeige erlauben keine Betriebs- oder Schließungsaussage. Die Mindestmengen sind keine Preise und keine Gebühren. Keine Frontmatter-Änderung; Leostr. 6 bleibt bis zur Betreiber-/Registerbestätigung nicht GEO-freigegeben. Quellen: https://www.11880.com/branchenbuch/trier/060440092B112767456/pfeil-heinz.html ; https://lokaleschrottplatz.de/pfeil-heinz/ .]

### Importiert (Seed-Stand 2026-09-30)

- Leostr. 6
- urspr. Website-Angabe: keine
- Adresse: Trier

### Recherche 02.10.2026

- [Recherche 02.10.2026: Adress-Konsens über 4 Aggregatoren (firmania/meinestadt/cylex/lokaleschrottplatz: Petra/Pfeil-Heinz Schrotthandel und Entsorgungsbetrieb, Leostr. 6, 54290 Trier, 0170 6878799) + 11880-Eintrag (Stand 05/2026); aber alles Leads ohne Betreiber-/Registerbeleg → kein Fill (Seed-Rohtext in street unangetastet), pruefung bleibt; Quelle(n): s.o. Aggregatoren (Leads)]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt: 49.7454006, 6.6365441** (Mittelwert der eindeutigen Knoten des
  Objekts **way/160207789**, achtzehn eindeutige Knoten; der Punkt liegt innerhalb der Objekt-BBox
  6.6363418/49.7452683 bis 6.6367713/49.7456357). **Quelle 1 (OSM, live geladen am 04.11.2026, HTTP 200):**
  **way/160207789** trägt `addr:street`=Leostraße, `addr:housenumber`=**6**, `addr:city`=Trier,
  `addr:postcode`=54290, `addr:country`=DE, `building`=yes - Straße, Hausnummer und Ort stimmen mit der im Dossier
  belegten Seed-Adresse **Leostr. 6, 54290 Trier (Trier-Süd)** überein. **Fensterprüfung (kein Fensterfehler):** das
  am 04.11.2026 live abgerufene Kachelraster über Trier-Süd umfasst **6.6300/49.7400 bis 6.6400/49.7500**, **1
  Kachel von je 0,01 Grad**, **0 Kachel-Fehler**; das Fenster enthält die gesuchte Straße **nachweislich** mit den
  Straßenobjekten **way/19564626**, **way/507924665** und **way/540487717** (alle `name`=Leostraße) sowie **7**
  Adressobjekten auf „Leostraße" mit den Hausnummern 1, 2, 3, 4, 5, 6 und 7 - die **Dossier-Hausnummer 6 existiert**
  und ist im Raster das einzige Objekt mit dieser Nummer auf dieser Straße. Eine Namenssuche nach **„Pfeil"** über
  die Felder name, operator, description, website, contact:website, brand, note und addr:housename findet im Raster
  **keinen** Treffer - es gibt also **keine** OSM-Betriebszuordnung, nur die Adressangabe. Quelle 2 (Betreiber/
  Verzeichnis, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): die **11880-Unternehmensprofilseite**
  liefert **HTTP 200** und führt live **„Pfeil-Heinz"** mit der Branche **Altmetallrecycling** an
  **Leostr. 6, 54290 Trier (Trier-Süd)**, Telefon **(0170) 6878799**; das **Schrottplatz-Portal LokaleSchrottplatz**
  nennt live denselben Betrieb mit **Leostraße 6, 54290 Trier** und derselben Nummer. **Zurücknahme des
  `geo`-Tokens (Begründung):** `clear_fields: 'geo'` **entfällt ersatzlos**, weil der Seed jetzt eine belegte
  Koordinate führt; der Seed lehnt ein `geo`-Clear neben belegtem `lat`/`lon` ab, und der Punkt liegt exakt auf der
  durch zwei unabhängige Quellen gedeckten Anschrift. **Qualifizierung (unverändert übernommen):** Der Punkt
  bezeichnet die **verzeichnisgestützte Anschrift**, keinen verifizierten Annahmehof; das im Dossier dokumentierte
  **Fehlen eines Betreiber- oder Registerbelegs**, die **abweichenden Öffnungszeiten** (11880 Mo–Fr 08:00–17:30
  gegenüber Portal Mo–Fr 08:00–17:00) und die **Portal-Mindestgewichte** (100 kg Fe / 1 kg NE) bleiben unbestätigte
  Portalangaben - eine Ankaufzusage, ein Annahmehof oder ein Öffnungsnachweis wird durch die Koordinate **nicht**
  belegt. `status: pruefung`, `city`, `street` (leer), `postcode` (leer), `phone` (leer), `email`, `opening_hours`,
  `website`, `website_status` und `description` bleiben unverändert. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/160207789/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=6.6300,49.7400,6.6400,49.7500 ;
  https://www.11880.com/branchenbuch/trier/060440092B112767456/pfeil-heinz.html ;
  https://lokaleschrottplatz.de/pfeil-heinz/ (Abruf 04.11.2026).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **In OSM existiert weiterhin kein Objekt, das den Betrieb benennt - die Namenssuche findet in Trier nur einen Namensvetter.** Die Nominatim-Namenssuche (live am 08.11.2026) nach „Pfeil-Heinz" liefert **keinen** Treffer; die Suche nach „Pfeil Trier" liefert ausschließlich einen **Denkmal**-Eintrag „Wilhelm Pfeil" (node/11725375070, `historic`=memorial, Hornstraße 24, Trier) und weitere Personen-/Straßentreffer, aber **kein** Objekt mit `shop=`, `craft=`, `industrial=` oder `landuse=commercial` und **kein** `name`-Treffer zu „Pfeil-Heinz". Die im Dossier am 04.11.2026 dokumentierte Feststellung (kein Betriebsobjekt im Kachelraster über 6.6300/49.7400 bis 6.6400/49.7500) wird damit bestätigt: die Koordinate 49.7454006 / 6.6365441 bezeichnet die **verzeichnisgestützte Anschrift** Leostraße 6, keinen benannten Annahmehof. Quelle(n): https://nominatim.openstreetmap.org/search?q=Pfeil-Heinz&format=jsonv2&limit=8&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Pfeil+Trier&format=jsonv2&limit=8&addressdetails=1 (Abruf 08.11.2026).]
- [Recherche 08.11.2026: **Amtliche Registerprüfung: kein eingetragener Rechtsträger mit Bezug zu „Pfeil" oder „Pfeil-Heinz" in Trier.** Die **offizielle** Namenssuche im Handelsregisterportal (handelsregister.de, normale Suche, Modus „alle Schlagwörter enthalten", Abruf 08.11.2026) liefert für den Ortfilter **Trier** mit den Schreibweisen „Pfeil" und „Pfeil-Heinz" jeweils **0 Treffer**. **Reichweite:** das Registerportal führt ausschließlich Rechtsträger mit Registereintrag, also **keine** Einzelunternehmer; ein Nichttreffer ist deshalb **kein** Nichtexistenz- oder Schließungsbeweis, sondern belegt nur: „unter diesem Namen ist kein eingetragener Rechtsträger geführt". Das amtliche Fachbetrieberegister (`fachbetrieberegister.gadsys.de`) war am 08.11.2026 zudem **technisch nicht erreichbar** (Namenssuche „Pfeil" nur mit Suchformular, ohne Trefferliste, Wartezeit bis 60 s); ein Registerbefund zur AltfahrzeugV-Anerkennung liegt damit **nicht** vor, und deren Fehlen wäre ohnehin **kein** Existenz- oder Schließungsbeweis. Quelle(n): https://www.handelsregister.de/rp_web/auswahl.do ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Pfeil (Abruf 08.11.2026).]
- [Recherche 08.11.2026: **ANKAUF / VERKAUF / GEBÜHREN: keine belegte Preisliste; die Portal-Mindestgewichte bleiben Portalangaben.** Die Verzeichnisprofile nennen Mindestgewichte von 100 kg Eisen und 1 kg Nichteisenmetalle sowie abweichende Öffnungszeiten (11880 Mo-Fr 08:00-17:30 gegenüber Portal Mo-Fr 08:00-17:00); das ist ein **Verzeichniswert**, kein bestätigtes Angebot des Betriebs. **ANKAUF:** keine Auszahlungskurse und keine betreiberbestätigten Annahmebedingungen. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife; kommunale Wertstoffhof-Tarife wurden bewusst nicht als Händlergebühren herangezogen. Quelle(n): https://www.11880.com/branchenbuch/trier/060440092B112767456/pfeil-heinz.html ; https://lokaleschrottplatz.de/pfeil-heinz/ (Abruf 08.11.2026).]
