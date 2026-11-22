---
slug: hb-syke-siegmayer-l
name: Siegmayer L.
trader_type: schrotthaendler
state: NI
city: Syke
street: Sudweyher Str. 34
postcode: '28857'
phone: 04242 931462
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Verzeichnis-Lead: Schrotthandel Siegmayer L., Sudweyher Str. 34, 28857 Syke, Tel. 04242 931462. Kein Betreiberauftritt und kein Registerbeleg; die Hausnummer 34 ist in OSM als Wohngebäude ohne Gewerbe-Tag erfasst, und das offizielle Branchenbuch der Stadt Syke führt an der Sudweyher Straße keinen Schrotthandel. ANKAUF, VERKAUF und GEBÜHREN sind nicht belegt. Status pruefung, keine Schließung behauptet.'
dropoff_json: ''
pickup_json: ''
certifications: ''
provenance_section: Neue Funde (nicht in den Tabellen oben / Nachtrag Runde 2)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Siegmayer L.

## Überblick

Die vorhandene Adresse Sudweyher Str. 34, 28857 Syke, und Telefon 04242 931462 stammen bislang nur aus Schrottplatz-/Branchenverzeichnissen. Weitere Verzeichnisse spiegeln diese Daten, sind aber keine unabhängigen Betreiberbelege. Ein Öffnungszeitenbuch führt außerdem einen „Ludwig Siegmayer“ an Zum Walde 7; ob das derselbe Mensch/Betrieb ist, ist offen und wird nicht zusammengeführt. Ein Quercheck zum HE-Dossier `he-ober-ramstadt-64372-siegmayer-markus-schrott` ergab einen anderen Telefonanschluss und ebenfalls widersprüchliche Verzeichnisadressen; gleicher Nachname allein ist kein Merge-Beleg. Die amtliche Stadtzuordnung ist Syke, Niedersachsen; deshalb ist `state` auf NI korrigiert, der unveränderliche Slug/Pfad `hb-...` bleibt wie vorgefunden.

**Preise:** keine Betreiberpreisliste und keine belastbaren Ankaufpreise, Verkaufspreise oder Gebühren. Aktiver Betrieb und Zusammenhang der Namens-/Adressvarianten bleiben ungeklärt.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott (klein)
- urspr. Website-Angabe: keine (nur schrottplatz-info-Beleg)
- Adresse: Syke (Fern-Umland), Sudweyher Str. 34, 28857, Tel. 04242 931462
- Adressbeleg: Seed-notes (schrottplatz-info)

### Recherche 03.10.2026

- [Recherche 03.10.2026: Offizielle Stadt-Syke-Angaben ordnen Syke dem Bundesland Niedersachsen, Landkreis Diepholz, zu; das widerspricht dem bestehenden Pfad/Slug hb- und state HB. State/Pfad/Slug nicht geändert, weil Slugs unveränderlich sind und eine Umordnung in einen anderen State außerhalb dieses Shards liegt — Owner-Klärfall. Die Händleradresse/Telefonnummer bleiben nur Seed-/Schrottplatz-Info-Leads, kein unabhängiger Betreiber-/Registerbeleg; website_status leer→unbekannt, status pruefung bleibt, keine Schließung behauptet.; Quelle(n): https://www.syke.de/leben/ueber-syke/, https://www.syke.de/portal/startseite.html]

### Recherche 06.10.2026

- [Korrektur 06.10.2026: geografische Zuordnung von Syke erneut anhand der offiziellen Stadtseite geprüft; `state` von HB auf NI gesetzt und Dossier unter `dossiers/ni/` abgelegt. Der unveränderliche Slug bleibt `hb-syke-siegmayer-l`. Die Verzeichnisdaten zu Sudweyher Str. 34/04242 931462 wurden in einer frischen Suche erneut gefunden, aber nur auf Aggregatoren; eine andere Verzeichnisangabe zu Ludwig Siegmayer, Zum Walde 7, beweist keine Identität und wird nicht zusammengeführt. Quercheck mit dem separaten HE-Siegmayer-Dossier: anderer Telefonanschluss, kein Beleg für dieselbe Person/Firma; kein Merge. Kein Betreiberbeleg und keine Preise für Ankauf, Verkauf oder Gebühren gefunden. Quelle(n): https://www.syke.de/leben/ueber-syke/ ; https://www.schrottplatz-info.de/schrottplatz/Syke/Siegmayer-L- ; https://www.schrottplatz.org/syke/schrott-siegmayer-l-aYEPvj.html ; https://www.oeffnungszeitenbuch.de/filiale/Syke-Ludwig%20Siegmayer-2720779D.html (Verzeichnisleads); Dossiervergleich: `dossiers/he/he-ober-ramstadt-64372-siegmayer-markus-schrott.md`]

### Recherche 30.10.2026

- [Recherche 30.10.2026: Amtliches Fachbetrieberegister (Spiegel schrottregister.pages.dev, Registerabruf 11.08.2026): für Syke existiert **keine Ortsseite** (HTTP 404, Seite nicht gefunden). Das ist **kein Negativbefund**, weil das Verzeichnis nur Orte mit Registereinträgen führt und die Efb-Anerkennung freiwillig ist. Quelle(n): https://schrottregister.pages.dev/ort-syke (Abruf 30.10.2026)]
- [Recherche 30.10.2026: OSM-Adressprüfung live am Objekt. Raster bbox=8.8236414,52.9390717,8.8529780,52.9608815 über den gesamten Verlauf der Straße "Sudweyher Straße" in Syke (Ortsteil Barrien), Filter addr:street=Sudweyher Straße: 45 Objekte | 43 verschiedene Hausnummern. Die Seed-Hausnummer 34 ist erfasst: way/260924504 mit addr:housenumber=34, addr:street=Sudweyher Straße, addr:postcode=28857, addr:city=Syke, addr:country=DE, addr:suburb=Barrien, **building=residential**. Ein name-, operator-, shop- oder industrial-Tag ist nicht gesetzt; präzise: die Adresse existiert in OSM als Wohnhaus, ein Schrottbetrieb ist dort nicht kartiert. Hinweis zur Zählung: zwei Rohwerte der Karte sind zusammengesetzte Notationen ("12,12a" und "2,2b") und wurden vom Scanwerkzeug als je ein Eintrag gezählt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/260924504/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=8.8236414,52.9390717,8.8529780,52.9608815 (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Amtliches Branchenbuch der Stadt Syke gegengeprüft. Das von der Stadt veröffentlichte Branchenbuch führt für die Sudweyher Straße die Einträge "EDEKA Stellmann" (7), "Falken-Apotheke Barrien" (11), "Nienaber Elektrotechnik GmbH" (12), "Charisma" (22), "Meyer Raumausstattung" (26), "Finance Consult Maklerpool GmbH & Co. KG" (37), "Avanti Malerfachbetrieb" (31) und "Elsner Klaus, Sanitär-, Heizungs- und Gasinstallationen" (48). Für die Hausnummer **34 ist kein Gewerbeeintrag** der Stadt Syke hinterlegt und **kein Schrotthandel** an dieser Straße genannt; die zweite Verzeichnisangabe "Ludwig Siegmayer, Zum Walde 7" aus der Vorwelle bleibt weiterhin ohne Zuordnung. Quelle(n): https://www.syke.de/regional/branchenbuch/gesamt/uebersicht.html?brotID=0&gewerblich=1&id=0&naviID=0&no_kreisweit=0&showall=1&vereine=0 (Abruf 30.10.2026)]
- [Recherche 30.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. ANKAUF: die Schrottkategorie stammt aus dem Verzeichnislead; ein Sortenangebot und **Zahlenpreise** sind nicht belegt. VERKAUF: kein Beleg. GEBÜHREN: keine Tarifordnung und keine Gebührenangabe. Der Status `pruefung` bleibt unverändert; eine Schließung wird nicht behauptet. Quelle(n): https://www.syke.de/regional/branchenbuch/gesamt/uebersicht.html (Abruf 30.10.2026)]

### Recherche 02.11.2026

- [Recherche 02.11.2026: **Zertifikats-Recherche ohne Treffer.** Für `Siegmayer L.` (Sudweyher Str. 34, 28857 Syke, Tel. 04242 931462; laut Dossier nur Verzeichnisdaten, zudem eine abweichende Verzeichnisadresse „Zum Walde 7“ und ein Namensvetter `he-ober-ramstadt-64372-siegmayer-markus-schrott` mit anderem Anschluss) wurde im amtlichen Fachbetrieberegister weder ein Efb-Zertifikat noch ein AltfahrzeugV-Eintrag gefunden; `certifications` bleibt leer. Suchen und Befunde (live am 02.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt): (1) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Siegmayer` -> **0 Treffer**, ebenso `…/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Siegmayer` -> **0 Treffer** (Filter „Art des Betriebs“ = Demontagebetrieb); (2) Radiusabfrage mit **3 km** um die Dossier-Adresse `…/Entsorgungsfachbetriebe/adr=Sudweyher Str. 34, 28857 Syke, Deutschland;…&dst=3&is=1&e=1` -> **0 Treffer**, dieselbe Abfrage in der Route `…/Altfahrzeugverwertung/adr=…&dst=3&is=1&e=1` -> **0 Treffer**; (3) **Positivkontrolle** mit **10 km** Radius um dieselbe Adresse -> `…/Entsorgungsfachbetriebe/adr=…&dst=10&is=1&e=1` -> **19 Treffer** (u. a. `Lübber Erd- und Tiefbau GmbH`, Bassumer Landstraße 60, 28857 Syke, 4,7 km, `ZZBT00500041 / 1`; `Weyher Entsorgung GmbH`, Werkstraße 5, 28844 Weyhe, 5,0 km, `ZZGT00200131 / 1` bis `/ 3`; `Autohaus Hinrichsen GmbH`, Syker Straße 90, 28816 Brinkum, 7,1 km) und `…/Altfahrzeugverwertung/adr=…&dst=10&is=1&e=1` -> **1 Treffer** (`Edwin`s Autorecycling Inh. Edwin Dembeck`, Gutenbergstr. 10, 28816 Stuhr, `ZACS00100018 / 1`, 9,6 km) - die Abfragetechnik funktioniert also; **an der Sudweyher Str. 34 und im Umkreis 3 km gibt es jedoch keinen Registereintrag**.
    Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt.
    Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Siegmayer ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Siegmayer ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Sudweyher%20Str.%2034,%2028857%20Syke,%20Deutschland;8.830987;52.9435276;8.90254159876075;52.98664653639957;8.75943240123925;52.90040866360044&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Sudweyher%20Str.%2034,%2028857%20Syke,%20Deutschland;8.830987;52.9435276;8.90254159876075;52.98664653639957;8.75943240123925;52.90040866360044&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Sudweyher%20Str.%2034,%2028857%20Syke,%20Deutschland;8.830987;52.9435276;8.90254159876075;52.98664653639957;8.75943240123925;52.90040866360044&dst=10&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Sudweyher%20Str.%2034,%2028857%20Syke,%20Deutschland;8.830987;52.9435276;8.90254159876075;52.98664653639957;8.75943240123925;52.90040866360044&dst=10&is=1&e=1 (Abruf 02.11.2026).]


### Recherche 22.11.2026 (Welle 1200 B)

- [Recherche 22.11.2026: **Der Registerbefund bleibt negativ, der Verzeichniseintrag ist weiterhin live, und der gespeicherte Punkt trifft exakt die Adresse - aber an einem Wohnhaus ohne Gewerbe-Tag.** (1) **Amtliches Fachbetrieberegister, live 22.11.2026 (clientseitig gerenderte Vaadin-App, Ergebnisgrid vollstaendig durchscrollt):** Die Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Siegmayer` liefert **0 Treffer**, ebenso `…/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Siegmayer` -> **0 Treffer** (Filter „Art des Betriebs" = Demontagebetrieb). **Reichweitenklausel:** die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt; die Aussage gilt fuer die Namensvariante „Siegmayer" und den Filter „Art des Betriebs" = Egal (`is=1&e=1`). Der Negativbefund vom 02.11.2026 bleibt damit **unveraendert bestaetigt**; `certifications` bleibt leer.
  (2) **Verzeichnisstand live geprueft (22.11.2026):** Das Schrottplatz-Portal `https://www.schrottplatz-info.de/schrottplatz/Syke/Siegmayer-L-` ist weiterhin **live** (HTTP 200); die im Dossier dokumentierte abweichende Verzeichnisangabe **„Ludwig Siegmayer, Zum Walde 7"** bleibt **ohne** Zuordnung und wird **nicht** zusammengefuehrt; der Namensvetter `he-ober-ramstadt-64372-siegmayer-markus-schrott` (anderer Anschluss) bleibt **getrennt**. Das offizielle Branchenbuch der Stadt Syke fuehrt an der Sudweyher Strasse weiterhin **keinen** Schrotthandel und **keinen** Gewerbeeintrag zur Hausnummer 34; die Stadt kennzeichnet ihre Eintraege als freiwillig, das ist **kein** Negativbeweis.
  (3) **GEO gemessen, nicht gesetzt:** die read-only Produktionszeile enthaelt **52.9435276/8.830987**; die Nominatim-Rueckwaertsaufloesung ergibt **„34, Sudweyher Stra&szlig;e, Barrien, Syke, 28857"** und trifft damit **exakt** das belegte Adressobjekt **way/260924504** (`addr:housenumber`=34, `addr:street`=Sudweyher Stra&szlig;e, `addr:postcode`=28857, `addr:city`=Syke, `addr:suburb`=Barrien, `building`=**residential**), das laut Recherche vom 30.10.2026 **keinen** name-, operator-, shop- oder industrial-Tag traegt. Der Punkt ist damit **adresskonform**, stuetzt aber **keinen** Gewerbebetrieb; es wird **keine** Koordinate neu gesetzt und **keine** Aenderung an `street`, `postcode`, `phone` oder `status` vorgenommen.
  (4) **Preise strikt getrennt, Stand 22.11.2026:** **ANKAUF** - die Schrottkategorie stammt aus dem Verzeichnislead; ein Sortenangebot und **Zahlenpreise** sind **nicht** belegt; **VERKAUF** - kein Beleg; **GEBUEHREN** - keine Tarifordnung und keine Gebuehrenangabe. Der Status `pruefung` bleibt unveraendert, eine Schliessung wird **nicht** behauptet. **Offen:** der ausgeschriebene Gewerbenamen, die Zuordnung der historischen Rufnummer und ob an der Sudweyher Strasse 34 ueberhaupt eine Schrottannahme stattfindet.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Siegmayer ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Siegmayer ; https://www.schrottplatz-info.de/schrottplatz/Syke/Siegmayer-L- ; https://www.syke.de/regional/branchenbuch/gesamt/uebersicht.html ; https://api.openstreetmap.org/api/0.6/way/260924504/full.json ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=52.9435276&lon=8.830987&addressdetails=1 ; read-only `public.db` (22.11.2026) (Abruf 22.11.2026).]
