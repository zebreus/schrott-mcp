---
slug: hh-billstedt-mobiler-schrottservice-mobiler-schrottha
lat: 53.5357533
lon: 10.1268148
name: Mobiler Schrottservice (mobiler-schrotthändlerhamburg.de)
trader_type: mobil
state: HH
city: Billstedt
street: Brockhausweg 57
postcode: '22127'
phone: 01788028550, 015775230613
email: donatosteinbach@icloud.com
opening_hours: ''
website: https://mobiler-schrottservice.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: '{}'
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: ja (Abholung)
provenance_origin: table
---

# Mobiler Schrottservice (mobiler-schrotthändlerhamburg.de)

## Überblick

Mobiler Schrottservice ist ein Einzelunternehmen am Brockhausweg 57 in 22127 Hamburg-Billstedt,
das Schrott und Metalle wie Aluminium, Kupfer, Messing, Edelstahl, Kabel und Batterien abholt,
demontiert und transportiert, ohne dass eine Kundenanlieferung belegt wäre. Das Impressum des
Betreibers nennt Anschrift, beide Rufnummern und die E-Mail-Adresse, das OSM-Adressobjekt zur
Hausnummer 57 stützt die Anschrift, trägt aber weder Namen noch Betreiberangabe und weist die
Postleitzahl 22117 statt der gedruckten 22127 aus. Offen bleibt die Standortfrage, weil die
Anschrift als Kontakt- und Impressumsadresse geführt wird und ein zweiter gleichnamiger
Verzeichniseintrag an der Nummer 93 im selben Strassenzug ungeklärt ist; eine Übernahme von
Kontakten oder Leistungen aus diesem Eintrag ist nicht erfolgt. Ankaufpreise, Verkaufslisten
und Gebühren sind nicht belegt.

## Timeline

### Recherche 03.10.2026 (Feedback #4431 Owner-Triage)

- [Recherche 03.10.2026: Aktuelle Betreiberseite nennt Brockhausweg 57 und Schrottankauf/Abholung, schließt Kundenanlieferung aber nicht aus; die Adresse ist allein Impressums-/Kontaktadresse, keine nachgewiesene Annahmestelle. Gleichnamiger Eintrag „Steinbach“ an Nr. 93 mit anderer Telefonnummer stammt aus Verzeichnissen und bleibt getrennte, ungeklärte Lead-Identität. Kein Merge und kein `dropoff_json.allowed=false`; Frontmatter bleibt unbekannt.; Quelle(n): https://mobiler-schrottservice.de/, https://mobiler-schrottservice.de/leistungen/, https://mobiler-schrottservice.de/impressum/]

### Importiert (Seed-Stand 2026-09-30)

- Alu, Kupfer, Messing, VA, Kabel, Batterien; Abholung/Demontage

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2201 teilweise berechtigt — Kontakt-Fills (PLZ, Telefon, E-Mail) + website_status aktiv per Betreiber-Kontaktseite (Brockhausweg 57, 22127 Hamburg); trader_type schrotthaendler → mobil (reiner Ankauf-/Abhol-/Demontage-/Transport-Dienst, keine Anlieferung belegt); Einzelunternehmen ohne HRB → EINZELBELEG, Zweitbeleg offen; Dossier-Name nennt Fremd-Domain mobiler-schrotthändlerhamburg.de, Website ist mobiler-schrottservice.de (Alias vermerkt); Adresse neu → Koordinaten neu geocodieren; Quelle(n): https://mobiler-schrottservice.de/ + /kontakt/]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Keine Koordinate gesetzt — Klärfall dokumentiert.** Die Betreiberquelle ist hier
  belastbar: Das **Impressum** des Betreibers (live abgerufen 03.11.2026, https://mobiler-schrottservice.de/impressum/)
  nennt **„mobiler-schrottservice.de, Brockhausweg 57, 22127 Hamburg"**, Telefon **01788028550** und **015775230613**,
  E-Mail donatosteinbach@icloud.com — identisch mit der Dossier-Adresse, -Telefonnummern und -E-Mail. **Die
  geokodierungsseitige Quelle fehlt jedoch:** Eine OSM-Hausnummerensuche (api.openstreetmap.org/api/0.6/map.json,
  0,01-Grad-Kacheln) im Umfeld der Dossier-Adresse **Brockhausweg 57** wurde in dieser Welle angestoßen und über das
  Kachelraster **53.5800-53.6200 / 9.9600-10.0600** weitergeführt; sie wurde durch die API-Drosselung (HTTP 429 „You
  have downloaded too much data") nur **teilweise** abgeschlossen und ist deshalb **kein** belastbarer Negativbefund —
  ein Objekt mit `addr:housenumber`=57 und `addr:street`=Brockhausweg wurde in den abgerufenen Kacheln **nicht**
  angetroffen, die Dossier-Hausnummer ist damit in OSM **bisher nicht nachgewiesen**. Ohne Objekt ist kein „Mittelwert
  der eindeutigen Knoten" bildbar und kein Punkt innerhalb einer Objekt-BBox prüfbar; eine Position „ungefähr im
  Brockhausweg" wäre eine geratene Koordinate. **Konsequenz:** Die Dossier-Adresse bleibt ungeokodiert, obwohl sie
  betreiberseitig belegt ist — die Nachpflege ist eine OSM-Datenlücke, kein Beleg gegen die Adresse. **Abgrenzung,
  unverändert übernommen:** Das Impressum ist eine **Kontakt-/Impressumsadresse**; das Dossier hält fest, dass eine
  Kundenanlieferung dort **nicht** ausgeschlossen, aber **nicht** belegt ist — eine spätere Koordinate würde daher die
  **Betreiberanschrift** markieren, keinen Annahmehof. `status: aktiv`, `street`/`postcode`, Telefon/E-Mail,
  `website_status: aktiv` und alle übrigen Frontmatter-Felder bleiben unverändert; es wird **keine** Anlieferung
  behauptet. **Nächster Schritt für den Owner:** OSM-seitig das Adressobjekt Brockhausweg 57 ergänzen bzw.
  identifizieren und die Koordinate danach aus dem Objekt nachziehen. Quelle(n):
  https://mobiler-schrottservice.de/impressum/ ; https://mobiler-schrottservice.de/leistungen/ ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9600,53.5800,10.0600,53.6200 (Abruf 03.11.2026, teilweise
  gedrosselt).]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt: `lat`/`lon` = 53.5357533/10.1268148.** Das im Vermerk vom
  03.11.2026 dokumentierte Kachelraster **9.9600-10.0600 / 53.5800-53.6200** wurde jetzt
  **vollständig** abgerufen (40 von 40 Kacheln zu 0,01 Grad, **0 Kachel-Fehler**); es enthält **kein**
  Objekt mit `addr:street`=Brockhausweg, denn das Fenster liegt rund **10,5 km** vom Adressort entfernt
  (es erfasst Winterhude/Alsterdorf/Barmbek/Ohlsdorf) und war damit **fehlerhaft platziert**: der
  damalige Negativbefund war kein Daten-, sondern ein Fensterfehler. **Korrigiertes Fenster (live,
  vollständig): 10.1100-10.1400 / 53.5250-53.5500** (9 Kacheln zu 0,01 Grad,
  api.openstreetmap.org/api/0.6/map.json, Abruf 04.11.2026, **0 Kachel-Fehler**) liegt auf der
  Brockhausweg in **Kirchsteinbek/Billstedt** und erfasst **127** Objekte mit `addr:street`=Brockhausweg
  (Hausnummern 1 bis 93 mit Buchstaben-Suffixen); **genau ein** Objekt trägt `addr:housenumber`=**57**,
  ein zweites Objekt mit derselben Hausnummer gibt es im Fenster nicht. **Quelle 1 (OSM-Objekt, live
  geladen 04.11.2026):** **way/297248994**
  (https://api.openstreetmap.org/api/0.6/way/297248994/full.json) trägt `addr:housenumber`=**57**,
  `addr:street`=**Brockhausweg**, `addr:city`=Hamburg, `addr:country`=DE, `addr:postcode`=**22117**,
  `building`=apartments, `roof:shape`=gabled; das Objekt hat **5 eindeutige Knoten**, der Mittelwert der
  Knoten (53.5357533/10.1268148) liegt **innerhalb** der Objekt-BBox (53.5356713/10.1266870 bis
  53.5358267/10.1269101). **Quelle 2 (Betreiber):** Das **Impressum** des Betreibers
  (https://mobiler-schrottservice.de/impressum/, live abgerufen 04.11.2026) nennt **„mobiler-schrottservice.de,
  Brockhausweg 57, 22127 Hamburg"**, Telefon **01788028550** und **015775230613**, E-Mail
  donatosteinbach@icloud.com: die gesetzte Koordinate ist damit die **Impressumsanschrift** des
  Betreibers. **PLZ-Abweichung, ausdrücklich:** OSM trägt für die Brockhausweg die PLZ **22117**, das
  Impressum nennt **22127**; die Dossier-Felder `street`/`postcode` bleiben deshalb **unverändert**
  (Brockhausweg 57 / 22127), die PLZ-Differenz ist damit **kein** Geokodierungsfehler, sondern eine
  uneinheitliche PLZ-Angabe. **Abgrenzung, unverändert übernommen:** Das Objekt trägt **keinen** Namen,
  **keinen** `operator` und **keinen** Branchenbezug zu Schrott-, Metall- oder Kabelhandel (im Fenster
  trägt nur way/1429547730 „Freiwillige Feuerwehr Kirchsteinbek", Nr. 2, einen Namen); ein
  `building`=apartments ist kein Betriebsbeleg. Das Impressum ist eine **Kontakt-/Impressumsadresse**,
  das Dossier hält fest, dass eine Kundenanlieferung dort **nicht** ausgeschlossen, aber **nicht**
  belegt ist: die Koordinate markiert die **Betreiberanschrift**, keinen Annahmehof, und es wird
  **keine** Anlieferung behauptet. **Nachbarschaft bleibt getrennt:** Die rund **263 m** weiter
  nordwestlich liegende **Brockhausweg 93** (way/55466738, Mittelpunkt 53.5373866/10.1239735, 22117) ist
  der Sitz des im Repo als `hh-billstedt-steinbach-der-mobile-schrottservice` geführten, **ungeklärten**
  Lead-Eintrags; es
  erfolgt **kein** Merge, keine Übernahme von Kontakten oder Leistungen und keine Änderung an
  `dropoff_json`/`pickup_json`. **Reichweite / Fehlbelegklausel:** Die **Hausnummernlage in OSM** und
  die **Efb-/AltfahrzeugV-Lage** sind **nicht** gleichbedeutend mit „kein Betrieb am Ort": die jetzt
  gefundene Hausnummer 57 belegt **nur** eine Adresse, sie belegt **weder** einen Schrottbetrieb **noch**
  eine Annahme von Privatkunden, und umgekehrt belegt das Fehlen eines Efb-/AltfahrzeugV-Eintrags oder
  eines Betriebsnamens **nicht**, dass an dieser Anschrift kein Betrieb existiert. `status: aktiv`,
  `street`/`postcode`, Telefon/E-Mail, `website_status: aktiv` und alle übrigen Frontmatter-Felder
  bleiben unverändert; der Vermerk vom 03.11.2026 bleibt **wortgleich** erhalten und wird durch diesen
  Vermerk **fortgeschrieben**: dessen Aussage „keine Koordinate gesetzt" galt dem fehlerhaft
  platzierten Fenster, nicht der Adresse. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/297248994/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=10.1100,53.5250,10.1400,53.5500 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9600,53.5800,10.0600,53.6200 ;
  https://mobiler-schrottservice.de/impressum/ (Abruf 04.11.2026).]
