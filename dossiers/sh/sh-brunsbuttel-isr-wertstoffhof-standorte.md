---
slug: sh-brunsbuttel-isr-wertstoffhof-standorte
name: ISR Recycling GmbH & Co. KG (Brunsbüttel)
trader_type: wertstoffhaendler
state: SH
city: Brunsbüttel
street: 'Am Südufer'
postcode: '25541'
phone: '04852 87244'
email: ''
opening_hours: ''
website: https://www.isr-recycling.de/
website_status: aktiv
status: pruefung
description: Lagerstandort der ISR Recycling GmbH & Co. KG in Brunsbüttel; für Schrott- und Materialannahme verweist der Betreiber auf Itzehoe.
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Schleswig-Holstein — Recherche (Stand 27.09.2026)
provenance_ankauf_raw: nein
provenance_origin: table
---

# ISR Wertstoffhof / Standorte

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026 (Feedback 2762, berechtigt): Betreiber-Standortseite: „Warehouse Brunsbüttel, Am Südufer, 25541 Brunsbüttel, Tel 0 48 52-87 2 44 / -80 11" + commissioned/admission — „We are currently not purchasing scrap or scrap metal. Materials are accepted at our location in Itzehoe." Gefüllt: street, postcode, phone, website_status aktiv; name präzisiert (Altlabel in Timeline). Status aktiv → pruefung (Ankauf derzeit negativ — buying status; kein Phantom: Lager existiert, ISR Itzehoe als Annahmestelle). EINZELBELEG (Betreiber DE+EN = eine Quelle; Impressum ohne HRB → Owner-Ausnahme n/a) — offengelegt. Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr + https://www.isr-recycling.de/index.php/en/standorte-isr-en (eine Quelle)]

### Importiert (Seed-Stand 2026-09-30)

- Wertstoffhof-Annahme (ISR-Gruppe)

### Korrektur 03.10.2026

- [Korrektur 03.10.2026 (Audit-Feedback #4519/#4521/#4523): Die aktuelle Betreiber-Standortseite führt Brunsbüttel ausdrücklich als Lager und sagt, dass dort derzeit kein Schrott-/Metallankauf erfolgt und Materialannahme über Itzehoe läuft. Den zeitgebundenen Satz aus der description in eine stabile Standortbeschreibung überführt und `provenance_ankauf_raw` von `ja` auf `nein` für diesen Branch-Stand korrigiert; `status: pruefung` bleibt, da die Seite den Lagerstandort bestätigt, aber keinen Annahmebetrieb in Brunsbüttel. Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr ; https://www.isr-recycling.de/index.php/en/standorte-isr-en]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprueft (Am Südufer, Brunsbüttel) - die Strasse traegt in OSM keine einzige Hausnummer.** Kachelraster ueber den **gesamten** Strassenverlauf (Strassenbbox 9.1558015/53.8957820 bis 9.1797247/53.9104806, **1 Kachel, 0 Kachel-Fehler**); die Bbox wurde aus den drei Strassenways mit `name=Am Südufer` gebildet (way/141046669, way/222162445, way/535225835). Zaehlung getrennt: **0 Adressobjekte / 0 verschiedene Hausnummern** auf der Am Südufer - im gesamten Raster gibt es **kein** Objekt, das zugleich `addr:housenumber` und `addr:street` mit dem Wert **Am Südufer** traegt. Das ist eine Aussage ueber den OSM-Bestand und **kein** Negativbeleg ueber den Betrieb. Zum Vergleich tragen die benachbarten Strassen im selben Raster sehr wohl Adressobjekte (Bahnhofstraße 16, Fährstraße 30, Holstenring 21, Steinburgstraße 13, Dithmarscher Ring 5, Schleswiger Straße 7, Holsteneck 7, Schoofstraße 4, Koogsweg 1, Ostermoorer Straße 1, Ostermoorweg 1) - die Am Südufer ist in OSM ein **reines Infra- und Hafengelände ohne Adressierung**. Zwei Objekte im Raster mit Betriebsbezug, beide **Fremdbetriebe**: node/3193699145 mit `amenity=recycling`, `recycling_type=container`, `operator=Abfallwirtschaft Dithmarschen`, `recycling:glass_bottles=yes`, `recycling:paper=yes` und node/3193699146 mit `amenity=recycling`, `recycling_type=container`, `operator=Loop Kleider&Schuhe`, `recycling:clothes=yes`. Ein Objekt mit einem `name`- oder `operator`-Bezug zur ISR gibt es im Raster **nicht**. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.1558015,53.8957820,9.1797247,53.9104806 ; https://api.openstreetmap.org/api/0.6/node/3193699145.json ; https://api.openstreetmap.org/api/0.6/node/3193699146.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Standortseite live gelesen: der Lagerstandort Brunsbüttel ist weiterhin ausdruecklich kein Ankaufsort.** Die Standortseite (https://www.isr-recycling.de/index.php/standorte-isr, HTTP 200 am 31.10.2026) fuehrt den Eintrag woertlich als „**Lager Brunsbüttel**, ISR Recycling GmbH & Co. KG, Am Südufer, 25541 Brunsbüttel, 0 48 52 - 87 2 44 / 0 48 52 - 80 11“ mit dem Hinweis: „**Zurzeit kein Ankauf von Schrott und Altmetallen. Die Annahme von Material erfolgt über unseren Standort in Itzehoe.**“ - der Wortlaut der Vorwellen-Notiz vom 03.10.2026 ist damit **unveraendert bestaetigt**; `provenance_ankauf_raw: nein` und `status: pruefung` bleiben korrekt. Die **Annahmestelle im Programm** ist der Schrottplatz **Itzehoe, Hafenstraße 35** (Mo.–Fr. 7.00–17.00 Uhr, „vergüten zu Höchstpreisen“) und der **Wertstoffhof Itzehoe, Carl-Zeiss-Straße 6** (Mo.–Fr. 8.00–17.00 Uhr, Sa. 8.00–14.00 Uhr). `website_status` war bereits `aktiv` und bleibt unveraendert. Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr ; https://www.isr-recycling.de/index.php/en/standorte-isr-en (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise und Gebuehren getrennt (Brunsbüttel).** ANKAUF: **keine Ankaufspreisliste** - fuer Brunsbüttel ist laut Betreiber **kein Ankauf** von Schrott und Altmetallen vorgesehen; die Preisangaben der Gruppe („vergüten zu Höchstpreisen“) beziehen sich auf **Itzehoe**, nicht auf Brunsbüttel. VERKAUF: **keine Verkaufspreisliste**. GEBUEHREN: **keine Tarife** fuer Lagerung, Container oder Annahme am Lagerstandort; die Annahme selbst faellt in Itzehoe an, wo fuer den Wertstoffhof die „derzeit gültige Entgeltordnung“ des Kreises Steinburg gilt (Betrag nicht veroeffentlicht). Quelle(n): https://www.isr-recycling.de/index.php/standorte-isr (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Frage: die Dossier-Adresse traegt in OSM keine Hausnummer - Koordinaten sind neu zu setzen.** Die Dossier-Adresse „Am Südufer“ (ohne Hausnummer) ist in OSM **nicht adressiert**; der Lagerstandort ist in OSM **nicht** als Betriebsobjekt erfasst (weder `name=` noch `operator=` mit ISR-Bezug im Raster). Der Seed-Punkt 53.895/9.1048 liegt ausserhalb der Strassenbbox der Am Südufer (9.1558015–9.1797247) und damit **nicht** auf dieser Strasse - er ist fuer die Geokodierung **zurueckzunehmen** bzw. neu zu setzen. Zu geokodieren ist: **Am Südufer, 25541 Brunsbüttel** (Lagenzufahrt; die Hausnummer ist beim Betreiber **nicht** erfragbar, da die Standortseite keine nennt). Ob der Lagerstandort zusaetzlich unter einer firmierten Strassenadresse laeuft, ist offen. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.1558015,53.8957820,9.1797247,53.9104806 ; https://www.isr-recycling.de/index.php/standorte-isr ; read-only `public.db` (31.10.2026).]
