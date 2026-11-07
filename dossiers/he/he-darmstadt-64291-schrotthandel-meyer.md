---
slug: he-darmstadt-64291-schrotthandel-meyer
name: Schrotthandel Meyer
trader_type: schrotthaendler
state: HE
city: Darmstadt 64291
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
clear_fields: 'geo'
provenance_section: Nachtrag Mömlingen-70km (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schrotthandel Meyer

## Überblick

Die Identität des Seeds bleibt offen, aber der **Namens- und Adresskonflikt ist präzisiert**: OSM trägt an **way/801683671** den Namen **„Schrotthandel Meyer"** mit `addr:street`=**Am Sportplatz**, `addr:housenumber`=**12**, `addr:postcode`=64291, `addr:suburb`=Wixhausen, `landuse`=industrial, `industrial`=**scrap_yard** (live gelesen, ohne `operator`/`website`/`phone`); **an derselben Hausnummer** führen die Verzeichnisse „**Der Zerseeger Schrotthandel**" (0177 9 33 37 33) und „**Schrotthandel in Darmstadt**" (0177 9333733) — die identische Nummer und die identische Adresse sprechen stark dafür, dass es **ein Betrieb unter zwei Namen** ist und der OSM-Name veraltet ist. Ein Betreiber- oder Registerbeleg, der die beiden Namen zusammenführt, fehlt weiterhin.

**Namensvetter, nicht übernommen:** die Sascha-Meyer-Betriebe in **Neustadt a. d. Weinstraße** (Im Altenschemel 40 a, 67435), `MB Schrotthandel` (Marco Bock, Messeler Str. 103, Darmstadt), `Fischer Eisen- und Schrottgroßhandel GmbH` (Am Güterbahnhof, 64293 Darmstadt, HRB 1144 — **gelöscht** 2008) sowie die Nachbarhausnummer **Am Sportplatz 11** mit „Levy David Schrott und Metallhandel" und „Schrott Darmstadt Lagerin".

**Preise:** Ankauf — keine Kurse; Verkauf — keine Verkaufspreise; Gebühren — keine Tarife. **Geocoding:** `clear_fields: 'geo'` bleibt bestehen; die **Hausnummer 12 in 64291 Darmstadt-Wixhausen** ist objektgenau in OSM belegt und damit der **GEO-Kandidat**, eine Koordinate wird ohne bestätigte Annahmefunktion und ohne Namensauflösung nicht gesetzt.

## Timeline

### Korrektur 24.10.2026: Falschen Produktionspunkt zurückziehen

- [Korrektur 24.10.2026: Read-only-Gegenprüfung der Produktionszeile `he-darmstadt-64291-schrotthandel-meyer` ergab `49.8718375, 8.6533`. Die Nominatim-Rückwärtssuche löst diesen Punkt auf Schuchardstraße 3, Darmstadt-Mitte, PLZ 64283 auf; die separate PLZ-Suche ordnet 64291 Wixhausen zu. Der aktuelle OSM-Weg 801683671 trägt zwar den Namen „Schrotthandel Meyer“ und wird in der Suche mit Sensfelder Hof 12, 64291 ausgegeben, belegt aber weder Betreiberidentität noch Kundenannahme. Damit ist der gespeicherte Punkt dem bestehenden Datensatz nicht zuzuordnen. `clear_fields: 'geo'` zieht nur das falsche Koordinatenpaar beim nächsten regulären Seed zurück; es wird kein neuer Punkt gesetzt. Die Produktionsdatenbank wurde nicht beschrieben. Quellen: https://nominatim.openstreetmap.org/reverse?lat=49.8718375&lon=8.6533&format=jsonv2&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=64291%20Darmstadt&format=jsonv2&addressdetails=1&limit=5 ; https://www.openstreetmap.org/way/801683671.]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall/miss mit Fremdtreffer — kein Darmstadt-Beleg zu Schrotthandel Meyer (nur OSM-Node Am Sportplatz ohne Nr., Dublettenverdacht Levy/Zerseeger); einziger verifizierter Meyer-Betrieb ist Schrotthandel Meyer (Sascha Meyer), Im Altenschemel 40a, 67435 Neustadt a. d. Weinstraße (Betreiber-Website + Gelbe Seiten, seit 1994) — Fremdtreffer, keine Zuordnung nach Darmstadt → keine Frontmatter-Fills. Quelle(n): https://schrotthandel-meyer.de/ (+ /impressum) + Gelbe-Seiten-Eintrag Meyer Sascha Neustadt]

### Importiert (Seed-Stand 2026-09-30)

- Schrott — nur OSM-Node (PRÜFFALL; mögl. Dublette Levy/Zerseeger-Seed)
- urspr. Website-Angabe: keine
- Adresse: Darmstadt 64291, Am Sportplatz (o. Nr.)

### Recherche 03.10.2026

### Recherche 04.10.2026

- [Recherche 04.10.2026: Die Gegenrecherche findet einen Mapcarta/OSM-Punkt „Schrotthandel Meyer“ an Am Sportplatz in Wixhausen (ohne Hausnummer/Betreiberbeleg); er bleibt ein einzelner Karten-Lead, kein unabhängiger Identitätsnachweis. Die Betreiberseite schrotthandel-meyer.de betrifft weiterhin Sascha Meyer in Neustadt a. d. Weinstraße; die Darmstädter Seite nennt Marco Bock, nicht Meyer. Keine belastbare Verknüpfung mit dem Dossier und keine Frontmatter-Füllung; Straße, Kontaktdaten und Website bleiben offen, status pruefung bleibt; Quelle(n): https://mapcarta.com/W801683671; https://schrotthandel-meyer.de/; https://darmstadt-schrotthandel.de/impressum]

- [Recherche 03.10.2026: Zwei Seiten der Identitätsprüfung bleiben getrennt. Die OSM/Nominatim-Abfrage zu „Am Sportplatz, 64291 Darmstadt“ findet nur die Straße im Ortsteil Wixhausen, keinen Händler und keine Hausnummer; deshalb weder Straßenadresse noch Koordinaten ableitbar. Die Betreiber-Website Schrotthandel Meyer wurde vertieft einzeln abgerufen (Startseite, Impressum, Leistungen, Service und Autoverwertung): sie nennt Sascha Meyer und ausschließlich Im Altenschemel 40a, 67435 Neustadt a. d. Weinstraße; dies ist ein realer Namensvetter/Fremdtreffer, kein Darmstadt-Nachweis. In Darmstadt 64291 findet sich separat die Betreiberseite MB Schrotthandel; deren Impressum nennt Marco Bock, Messeler Str. 103, nicht Meyer und nicht Am Sportplatz. Startseite, Über-uns-, Metallentsorgungs-, Entrümpelungs-, Kontakt- und Impressumsseite gehören zu derselben Betreiberquelle und sind kein unabhängiger Zweitbeleg. ENF und weitere Verzeichnisse bleiben Leads. Kein belastbarer Beleg verbindet eine dieser Firmen mit dem Dossier; Name, Adresse, Website und Koordinaten bleiben offen, status pruefung bleibt bestehen.; Quelle(n): https://nominatim.openstreetmap.org/search?q=Am%20Sportplatz%2C%2064291%20Darmstadt&format=jsonv2; https://schrotthandel-meyer.de/; https://schrotthandel-meyer.de/index.php/impressum; https://schrotthandel-meyer.de/index.php/leistungen; https://schrotthandel-meyer.de/index.php/service; https://schrotthandel-meyer.de/index.php/autoverwertung; https://darmstadt-schrotthandel.de/; https://darmstadt-schrotthandel.de/ueber-uns; https://darmstadt-schrotthandel.de/metallentsorgung; https://darmstadt-schrotthandel.de/entruempelung; https://darmstadt-schrotthandel.de/kontakt; https://darmstadt-schrotthandel.de/impressum; https://de.enfmetal.com/schrotthandel-meyer]

### Recherche 06.10.2026

- [Recherche 06.10.2026: Quellenabgleich bestätigt den bisherigen Negativbefund: weder der unnummerierte Kartenpunkt in Wixhausen noch der reale Namensvetter in Neustadt oder MB Schrotthandel belegt den Seedbetrieb. Keine Zuordnung oder Frontmatter-Änderung. Ankauf-/Verkaufspreise und Gebühren bleiben mangels identifizierten Betreibers offen; ohne Hausnummer keine Geokodierung. Quelle(n): https://mapcarta.com/W801683671; https://schrotthandel-meyer.de/; https://darmstadt-schrotthandel.de/impressum]

### Recherche 02.11.2026

- [Recherche 02.11.2026: **Zertifikats-Recherche ohne Treffer.** Für `Schrotthandel Meyer` (64291 Darmstadt-Wixhausen; im Frontmatter ohne Straße, der OSM-Lead `Sensfelder Hof 12` ist kein Betreiberbeleg) wurde im amtlichen Fachbetrieberegister weder ein Efb-Zertifikat noch ein AltfahrzeugV-Eintrag gefunden; `certifications` bleibt leer. Suchen und Befunde (live am 02.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt): (1) Radiusabfrage um den Kartenlead `…/Entsorgungsfachbetriebe/adr=Sensfelder Hof 12, 64291 Darmstadt, Deutschland;…&dst=3&is=1&e=1` -> **3 Treffer**, sämtlich `Darmstädter Recycling Zentrum GmbH (DRZ GmbH)`, Röntgenstraße 12, 64291 Darmstadt, ZZHT01000192 / 1, 2, 3 (1,7 km) - **Fremdbetrieb, kein Treffer auf `Meyer`**; (2) Kontrollabfrage um die Ortsmitte `…/Entsorgungsfachbetriebe/adr=64291 Darmstadt, Deutschland;…&dst=3&is=1&e=1` -> **4 Treffer** (`Eigenbetrieb für kommunale Aufgaben und Dienstleistungen`, Eckhardwiesenstraße 25, 64289 Darmstadt, ZZHT01000201 / 8, 2,2 km, und die DRZ GmbH, 2,7 km) - **kein Treffer auf `Meyer`**; (3) die Abfrage in der Route `…/Altfahrzeugverwertung` liefert für beide Adressen **0 Treffer**; (4) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Meyer` -> **78 Treffer** bundesweit - **keiner in Darmstadt oder Wixhausen**; (5) `…/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Meyer` -> **3 Treffer**: `Sascha Meyer Schrott und Metallhandel`, Im Altenschemel 40 a, 67435 Neustadt an der Weinstraße, ZZFT01300307 / 2; `Autoverwertung Meyer Inhaber Sascha Meyer`, Im Altenschemel 40a, 67435 Neustadt a. d. Weinstraße, ZAHS01300056 / 1; `Meyer Autofachmarkt und Autoverwerter e. K. Inhaberin Heike Wähner`, Heidenheimer Weg 27, 89564 Nattheim, ZALS00800059 / 1 - **Fremdbetriebe**; das Dossier hält bereits fest, dass die Neustädter Sascha-Meyer-Betriebe nicht mit dem Darmstädter Seed verbunden sind und `darmstadt-schrotthandel.de` einen anderen Betrieb (Marco Bock) führt. Die Efb-Anerkennung nach § 56 KrWG und die Anerkennung nach AltfahrzeugV sind freiwillig; ein Register-Nichteintrag ist deshalb kein Nichtexistenzbeweis und widerlegt weder Betrieb noch Annahme noch Zertifizierungsbedarf. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Sensfelder%20Hof%2012%2C%2064291%20Darmstadt%2C%20Deutschland%3B8.6300074%3B49.9319221%3B8.6420074%3B49.9439221%3B8.6180074%3B49.9199221&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Sensfelder%20Hof%2012%2C%2064291%20Darmstadt%2C%20Deutschland%3B8.6300074%3B49.9319221%3B8.6420074%3B49.9439221%3B8.6180074%3B49.9199221&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=64291%20Darmstadt%2C%20Deutschland%3B8.6737771%3B49.9274921%3B8.685777100000001%3B49.9394921%3B8.6617771%3B49.9154921&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=64291%20Darmstadt%2C%20Deutschland%3B8.6737771%3B49.9274921%3B8.685777100000001%3B49.9394921%3B8.6617771%3B49.9154921&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Meyer ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Meyer (Abruf 02.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Adresse ist in OSM **wohl** vorhanden,
  aber die **zweite Quelle fehlt**. Quelle 1 (OSM, live geprüft am 05.11.2026): **Fensterprüfung (kein Fensterfehler):**
  das live abgerufene Kachelraster über den Ortsteil Darmstadt-Wixhausen umfasst **8.6100/49.9100 bis 8.6700/49.9600**,
  **30 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**, 83668 Elemente; die Straße "Am Sportplatz" ist dort mit **13**
  Adressobjekten und **12** verschiedenen Hausnummern erfasst (1, 2, 3, 5, 6, 7, 7a, 9, 9a, 10, **12**, 14), die Nummer 12
  zweifach: way/223816048 (`building`=yes, ohne `name`) und **way/801683671** mit `name`=**Schrotthandel Meyer**,
  `industrial`=**scrap_yard**, `landuse`=industrial. way/801683671, live geladen am 05.11.2026 (HTTP 200,
  https://api.openstreetmap.org/api/0.6/way/801683671/full.json), trägt `addr:housenumber`=12, `addr:street`=**Am Sportplatz**,
  `addr:city`=Darmstadt, `addr:postcode`=**64291**, `addr:suburb`=Wixhausen, `addr:country`=DE; **10** Knoten, davon **9**
  eindeutig, Objekt-BBox 8.6296555/49.9314659 bis 8.6303465/49.9321343, der Punkt läge innerhalb. **Adresskorrektur zur
  Vorwellen-Notiz:** die alte Vermerk nennt den OSM-Lead mit "Sensfelder Hof 12" - das ist **falsch zugeordnet**; das Objekt
  trägt `addr:street`=**Am Sportplatz**. "Sensfelder Hof 12" ist in OSM way/8877713 mit `addr:city`=**Weiterstadt**,
  `addr:postcode`=**64331**, `addr:suburb`=Gräfenhausen (live geladen 05.11.2026, HTTP 200) - eine **andere Gemeinde**,
  die nicht die Seed-Adresse sein kann. Quelle 2 (Betreiber/Register): **fehlt für diese Anschrift** - für "Schrotthandel
  Meyer" in Darmstadt-Wixhausen existiert **kein** Betreiberauftritt und **kein** Registereintrag: die im Dossier zitierten
  Betreiberseiten betreffen Sascha Meyer (Im Altenschemel 40a, 67435 Neustadt a. d. Weinstraße) und MB Schrotthandel
  (Marco Bock, Messeler Str. 103), und das amtliche Altfahrzeugverwertungsregister führt "Sascha Meyer Schrotthandel und
  Metallhandel", "Autoverwertung Meyer Inhaber Sascha Meyer" (beide Im Altenschemel 40a, 67435 Neustadt) sowie "Meyer
  Autofachmarkt und Autoverwerter" (Heidenheimer Weg 27, 89564 Nattheim) - sämtlich **Fremdbetriebe**; die Namenssuche
  "Meyer" liefert **keinen** Treffer in Darmstadt-Wixhausen. **Warum keine Koordinate:** der OSM-Lead trägt zwar den
  Seed-Namen und ist als Schrottplatz (`industrial`=scrap_yard) kartiert, aber eine OSM-Kartenlage allein ist nach dem
  Quellenstandard **kein** Betreiber- oder Registerbeleg; der Altpunkt 49.8718375/8.6533 (Schuchardstraße 3,
  Darmstadt-Mitte, anderer Ortsteil und andere PLZ als 64291) wird mit `clear_fields: 'geo'` ohnehin zurückgenommen und
  **nicht** ersetzt. `status` bleibt `pruefung`. **Reichweitenklausel:** das Fehlen eines Betreiber- oder Registerbelegs ist
  **kein** Nichtexistenzbeweis; die OSM-Adresse selbst ist belegt, ebenso wie die Abweichung "Am Sportplatz" gegen
  "Sensfelder Hof". Offen bleibt, ob der Seed einen eigenständigen Darmstädter Betrieb bezeichnet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/801683671/full.json ;
  https://api.openstreetmap.org/api/0.6/way/223816048/full.json ;
  https://api.openstreetmap.org/api/0.6/way/8877713/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.61,49.91,8.67,49.96 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.62,49.93,8.64,49.95 (Abruf 05.11.2026)]

### Geokodierung 05.11.2026 (Nachprüfung)

- [Geokodierung 05.11.2026: **Klärfall - keine Koordinate gesetzt, `clear_fields: 'geo'` bleibt bestehen.** Die
  Nachprüfung bestätigt den Seed-Lead in OSM und korrigiert die Zuordnung der Ortsangabe, ändert aber am Ergebnis
  nichts: Die **Adresse des Seeds liegt in OSM vor** (Straße "Am Sportplatz", 64291 Darmstadt-Wixhausen), der
  **Betriebs- oder Registerbeleg fehlt weiterhin**. Quelle 1 (OSM, live geprüft am 05.11.2026): **Fensterprüfung
  (kein Fensterfehler):** das Feinraster **8.6200/49.9250 bis 8.6400/49.9400** umfasst **4 Kacheln von je 0,01 Grad**,
  **0 Kachel-Fehler**, 4563 Elemente und **umschließt die Objekt-BBox vollständig**; die Straße "Am Sportplatz" ist
  dort mit **13 Adressobjekten** und **12** verschiedenen Hausnummern erfasst (1, 2, 3, 5, 6, 7, 7a, 9, 9a, 10, **12**,
  14), die Nummer 12 zweifach: way/223816048 (`building`=yes, ohne `name`) und **way/801683671**. way/801683671, live
  geladen am 05.11.2026 (HTTP 200, https://api.openstreetmap.org/api/0.6/way/801683671/full.json), trägt
  `name`=**Schrotthandel Meyer**, `addr:housenumber`=12, `addr:street`=**Am Sportplatz**, `addr:city`=Darmstadt,
  `addr:postcode`=**64291**, `addr:suburb`=Wixhausen, `addr:country`=DE, `industrial`=**scrap_yard**,
  `landuse`=industrial; **10** Knoten, davon **9** eindeutig, Objekt-BBox 8.6296555/49.9314659 bis
  8.6303465/49.9321343, der Mittelwert der eindeutigen Knoten (49.9317997/8.6299112) läge innerhalb. Das Objekt trägt
  **weder** `operator` **noch** `website` **noch** `phone`; eine Suche nach "Meyer|Schrott|Scrap|Recycling|Metall" über
  die Felder name, operator, brand, description, website, contact:website, shop, craft, addr:housename, phone,
  contact:phone im Feinraster liefert **neben** way/801683671 nur **drei** Wertstoffcontainer fremder Betreiber
  (node/12453073056, node/12453094525 mit `operator`=EAD, node/12453095993 mit `operator`=Baliz Textilwerke, alle bei
  49.9301/8.6391) - **kein** weiterer Betriebsbezug zu "Meyer". **Adresskorrektur zur Vorwellen-Notiz, jetzt
  objektgenau belegt:** die alte Vermerk nennt den OSM-Lead mit "Sensfelder Hof 12" - das ist **falsch zugeordnet**;
  "Sensfelder Hof 12" ist way/8877713 (live geladen am 05.11.2026, HTTP 200) mit `addr:housenumber`=12,
  `addr:street`=Sensfelder Hof, `addr:city`=**Weiterstadt**, `addr:postcode`=**64331**, `addr:suburb`=Gräfenhausen,
  Objekt-BBox 8.6195982/49.9319129 bis 8.6198393/49.9321204, also **713 m** vom Objekt way/801683671 entfernt und in
  einer **anderen Gemeinde**. Das Kontrollfenster **8.6000/49.9200 bis 8.6600/49.9500** (**18 Kacheln**, **0
  Kachel-Fehler**, 47215 Elemente) zeigt zudem **zwei namensgleiche** Straßen "Am Sportplatz": die 13
  Darmstädter Adressobjekte (Hausnummern 1-14 ohne die 11, `addr:city`=Darmstadt, `addr:postcode`=64291, `addr:suburb`=Wixhausen) und eine
  weitergehende Reihe in **Weiterstadt-Gräfenhausen** (64331, 26 Adressknoten mit den Nummern 1-20 einschließlich
  11, node/11135870426 bei 49.9257924/8.607426, **1.743 m** entfernt) - die Trennung gelingt allein über
  `addr:city`/`addr:postcode`. Quelle 2 (Betreiber/Register): **fehlt weiterhin** - für "Schrotthandel Meyer" in
  64291 Darmstadt-Wixhausen existiert **kein** Betreiberauftritt und **kein** Registereintrag: die im Dossier
  zitierten Betreiberseiten betreffen Sascha Meyer (Im Altenschemel 40a, 67435 Neustadt a. d. Weinstraße) und
  MB Schrotthandel (Marco Bock, Messeler Str. 103), und das amtliche Altfahrzeugverwertungsregister führt "Sascha Meyer
  Schrotthandel und Metallhandel", "Autoverwertung Meyer Inhaber Sascha Meyer" (beide Im Altenschemel 40a, 67435
  Neustadt) sowie "Meyer Autofachmarkt und Autoverwerter" (Heidenheimer Weg 27, 89564 Nattheim) - sämtlich
  **Fremdbetriebe**; die Namenssuche "Meyer" liefert **keinen** Treffer in Darmstadt-Wixhausen. **Warum keine
  Koordinate:** der OSM-Lead trägt zwar den Seed-Namen und ist als Schrottplatz (`industrial`=scrap_yard) kartiert,
  aber eine OSM-Kartenlage allein ist nach dem Quellenstandard **kein** Betreiber- oder Registerbeleg; der Altpunkt
  49.8718375/8.6533 (Schuchardstraße 3, Darmstadt-Mitte, anderer Ortsteil und andere PLZ als 64291) wird mit
  `clear_fields: 'geo'` ohnehin zurückgenommen und **nicht** ersetzt. `status` bleibt `pruefung`. **Reichweitenklausel:**
  das Fehlen eines Betreiber- oder Registerbelegs ist **kein** Nichtexistenzbeweis; die OSM-Adresse selbst ist
  belegt, ebenso wie die Korrektur "Am Sportplatz" gegen "Sensfelder Hof". Offen bleibt, ob der Seed einen
  eigenständigen Darmstädter Betrieb bezeichnet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/801683671/full.json ;
  https://api.openstreetmap.org/api/0.6/way/223816048/full.json ;
  https://api.openstreetmap.org/api/0.6/way/8877713/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.62,49.925,8.64,49.94 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.6,49.92,8.66,49.95 (Abruf 05.11.2026)]

### Recherche 07.11.2026

- [Recherche 07.11.2026: **Die Namensverwirrung an Am Sportplatz 12 ist aufgelöst: OSM trägt dort „Schrotthandel Meyer", die Verzeichnisse führen an derselben Hausnummer „Der Zerseeger Schrotthandel" — es ist mit hoher Wahrscheinlichkeit derselbe Betrieb unter zwei Namen.** Die OSM-Kacheln um den Ortsteil wurden live über die OSM-API gelesen (Fenster 8.625/49.929 bis 8.637/49.936, 1.923 Elemente): **way/801683671** trägt weiterhin `name`=**Schrotthandel Meyer**, `addr:street`=**Am Sportplatz**, `addr:housenumber`=**12**, `addr:postcode`=64291, `addr:city`=Darmstadt, `addr:suburb`=Wixhausen, `landuse`=industrial, `industrial`=**scrap_yard**; daneben way/223816048 (`building`=yes) mit derselben Hausnummer 12. Auf **derselben Hausnummer** weisen die Verzeichnisse: **„Der Zerseeger Schrotthandel", Am Sportplatz 12, 64291 Darmstadt-Wixhausen, 0177 9 33 37 33** (Gelbe Seiten, live gelesen, mit Öffnungszeiten Mo–Fr 08:00–17:00, Sa 08:00–12:00 und Leistungsumfang „Schrott"; Branchenseite Gelbe Seiten bestätigt zugleich **„Levy David Schrott und Metallhandel"** an der **Nachbarhausnummer Am Sportplatz 11**, 0174 4 82 50 27) sowie **„Schrotthandel in Darmstadt", Am Sportpl. 12, 64291 Darmstadt, 0177 9333733**, „Laut eigener Angabe: Container, Entrümpelung", Google-Bewertung 3,5 von 5 (2 Bewertungen), Öffnungszeiten Mo–Fr 08:00–17:00, Sa/So 08:00–12:00 (`recyclinghof-wertstoffhof.de`, live gelesen). Die **Nummer 0177 9333733 ist in beiden Verzeichnissen identisch** und die **Adresse identisch** — das ist ein **starker Indiz**, dass „Schrotthandel Meyer" (OSM-Name) und „Der Zerseeger Schrotthandel" (Verzeichnisname) **derselbe Betrieb an Am Sportplatz 12** sind und der OSM-Name **veraltet** ist. **Beleggrenze:** Beides sind **Karten-/Verzeichnisangaben**; ein Betreiberauftritt, ein Registereintrag oder ein amtlicher Nachweis fehlt. Der **Namensvetter „Schrotthandel Meyer"** bleibt davon **unberührt**: die Sascha-Meyer-Betriebe in **Neustadt an der Weinstraße** (Im Altenschemel 40 a, 67435) sind ein **anderer Ort** und werden nicht übernommen; ebenso `MB Schrotthandel` (Marco Bock, Messeler Str. 103, Darmstadt) und `Fischer Eisen- und Schrottgroßhandel GmbH` (Am Güterbahnhof, 64293 Darmstadt, HRB 1144 — **gelöscht** 2008 nach Insolvenz, NorthData live gelesen). Quellen: https://api.openstreetmap.org/api/0.6/map.json?bbox=8.625,49.929,8.637,49.936 ; https://www.gelbeseiten.de/gsbiz/61244adb-16c2-4918-8641-6746e5f840a9 ; https://recyclinghof-wertstoffhof.de/schrottplatz/hessen/darmstadt/schrottthandel-in-darmstadt/ ; https://www.northdata.de/Fischer%20Eisen-%20und%20Schrottgro%C3%9Fhandelsgesellschaft%20mbH,%20Darmstadt/Amtsgericht%20Darmstadt%20HRB%201144 (Abruf 07.11.2026)]
- [Recherche 07.11.2026: **Amtliches Fachbetriebseregister geprüft — kein dem Dossierstandort zugeordneter Nachweis (Negativaussage mit Reichweite).** Das amtliche Fachbetriebseregister (`fachbetrieberegister.gadsys.de`, clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) wurde am 07.11.2026 in beiden Routen mit dem Filter „Art des Betriebs = Egal" (is=1&e=1) geprüft; Namenssuchen mit dst=10, Standortabfragen mit den Radien 3 und 10 km. **Namenssuche Route „Entsorgungsfachbetriebe":** „Schrotthandel Meyer" → **0 Treffer**; „Meyer" → **78 Treffer** (unverändert gegenüber der Vorwellen-Notiz vom 02.11.2026, davon **keiner** in Darmstadt-Wixhausen); „**Zerseeger**" → **0 Treffer**; „**Levy Schrott**" → **0 Treffer**. **Route „Altfahrzeugverwertung":** „Zerseeger" → **0 Treffer**; „Pohl" → **0 Treffer**; „Meyer" → **3 Treffer** (unverändert: Sascha Meyer Schrott und Metallhandel / Autoverwertung Meyer Inhaber Sascha Meyer, beide Im Altenschemel 40 a, 67435 Neustadt a. d. Weinstraße, sowie Meyer Autofachmarkt und Autoverwerter e. K., Heidenheimer Weg 27, 89564 Nattheim — **Fremdbetriebe**). **Standortabfrage** um „Am Sportplatz 12, 64291 Darmstadt-Wixhausen": EfB-Route dst=3 → **3 Treffer** (unverändert: `Darmstädter Recycling Zentrum GmbH (DRZ GmbH)`, Röntgenstraße 12, 64291 Darmstadt, `ZZHT01000192 / 1, 2, 3`, 1,7 km — **Fremdbetrieb**); AltfahrzeugV-Route dst=3 → **0 Treffer**. Die Trefferzeilen wurden über die Grid-Paginierung vollständig gelesen; die Zahl der gelesenen Zeilen stimmt in jeder Abfrage mit der angezeigten Trefferzahl überein. **Ergebnis:** Für „Schrotthandel Meyer" weist das amtliche Fachbetriebseregister zum Abrufdatum 07.11.2026 keinen Eintrag aus, der dem Dossierstandort zugeordnet ist; Zertifikatsnummer, Vorgangsnummer, Zertifizierungsorganisation, Ausstellungsdatum, Gültigkeitsdatum, Anlagenanzahl und zertifizierte Tätigkeit sind **nicht ausgewiesen**. Das Feld `certifications` bleibt leer. **Reichweite:** Die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis und widerlegt weder Betrieb noch Annahme noch Zertifizierungsbedarf. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrotthandel%20Meyer ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Zerseeger ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Am%20Sportplatz%2012%2C%2064291%20Darmstadt-Wixhausen%2C%20Deutschland%3B8.6300074%3B49.9319221%3B8.6420074%3B49.9439221%3B8.6180074%3B49.9199221&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Am%20Sportplatz%2012%2C%2064291%20Darmstadt-Wixhausen%2C%20Deutschland%3B8.6300074%3B49.9319221%3B8.6420074%3B49.9439221%3B8.6180074%3B49.9199221&dst=3&is=1&e=1 (Abruf 07.11.2026)]
- [Recherche 07.11.2026: **Die Geokodierfrage ist jetzt objektgenau geklärt — der Seed-Namens-Lead liegt adressgenau in OSM, aber die Koordinate wird weiterhin nicht gesetzt, weil die Namenszuordnung „Schrotthandel Meyer" ↔ „Der Zerseeger Schrotthandel" nur eine Indizienkette ist.** Die OSM-Adresse ist live bestätigt: **way/801683671**, Objekt-BBox 8.6296555/49.9314659 bis 8.6303465/49.9321343, 10 Knoten, davon 9 eindeutig, `addr:street`=Am Sportplatz, `addr:housenumber`=12, `addr:postcode`=64291, `addr:city`=Darmstadt, `addr:suburb`=Wixhausen, `landuse`=industrial, `industrial`=scrap_yard, **keine** Tags `operator`, `website`, `phone` oder `brand`. Die „13 Adressobjekte / 12 Hausnummern" der Vorwellen-Notiz sind **reproduziert**: im live gelesenen Fenster liegen die Adressobjekte Am Sportplatz 1, 2, 3, 5, 6, 7, 7a, 9, 9a, 10, 12 (zweifach: way/223816048 und way/801683671) und 14 sowie die Nachbarobjekte in **Wolfsgartenallee**; das Objekt trägt **keinen** Betriebsbezug außer dem veralteten Namen. **Konsequenz:** Die OSM-Adresse selbst ist belegt (Straße, Hausnummer, PLZ, Ortsteil) und **löst die Vorwellen-Frage nach der Adresslage vollständig**; es fehlt jedoch weiterhin ein **Betreiber- oder Registerbeleg**, der die Kartennamen „Schrotthandel Meyer" und „Der Zerseeger Schrotthandel" am selben Objekt zusammenführt. `clear_fields: 'geo'` bleibt deshalb bestehen und **es wird keine Koordinate gesetzt** — die Verzeichnisangabe „Der Zerseeger Schrotthandel" mit Nummer 0177 9333733 ist ein Aggregator-Lead, und die Zuordnung zweier Marken auf eine Hausnummer ist keine belegte Identität. Die **Hausnummer 12 in 64291 Darmstadt-Wixhausen** ist damit der **Geocoding-Kandidat**; die Nachbarhausnummer **11** trägt „Levy David Schrott und Metallhandel" (0174 4 82 50 27) und **„Schrott Darmstadt Lagerin"** (0178 7074924) — zwei **andere** Betriebe, **kein** Beleg für den Seed-Eintrag. Quellen: https://api.openstreetmap.org/api/0.6/map.json?bbox=8.625,49.929,8.637,49.936 ; https://www.gelbeseiten.de/branchen/schrottplatz/darmstadt%20stadtteil%20wixhausen ; https://lokaleschrottplatz.de/search/stahl-eisen-schrotthaendler-darmstadt/ (Abruf 07.11.2026)]
