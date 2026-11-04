---
slug: rp-rohrbach-sudpfalz-metall
lat: 49.1329322
lon: 8.1366168
name: Südpfalz Metall GmbH
trader_type: schrotthaendler
state: RP
city: Rohrbach
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://suedpfalzmetall-gmbh.de/
website_status: aktiv
status: pruefung
clear_fields: 'dropoff_json'
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Rheinland-Pfalz (RP) — Recherche
provenance_ankauf_raw: Ankauf Ja
provenance_origin: table
---

# Südpfalz Metall GmbH

## Überblick

Die Betreiberwebsite „Südpfalz Metall“ bewirbt einen Schrottplatz in Rohrbach und nennt Am Bellensee 6, Telefon +49 176 24226146, info@suedpfalzmetall-gmbh.de sowie Öffnungszeiten. Ihr Impressum nennt Janneck Knecht und HRB 33727, aber keinen Firmennamen. Der aktuelle Registerabdruck ordnet HRB 33727 der MBK Mietservice GmbH an genau dieser Anschrift und mit Janneck Knecht als Geschäftsführer zu; der eingetragene Unternehmensgegenstand ist jedoch der Verleih und Verkauf von Baumaschinen. Damit besteht eine starke Personen-/Adress-/Registerverbindung, aber keine bestätigte rechtliche Zuordnung des beworbenen Schrottbetriebs zur Seed-Entität **Südpfalz Metall GmbH**. `clear_fields: 'dropoff_json'` bleibt angemessen; der Altpunkt ist durch den am 04.11.2026 neu aufgelösten Punkt ersetzt. Die erreichbare Website ist kein unabhängiger Betriebsnachweis. „Faire Preise“ ohne Zahlen; keine Verkaufs- oder Gebührenliste.

## Timeline

### Owner-Audit 26.10.2026: HRB-Identität widerspricht Seedname

- [Owner-Audit 26.10.2026: Der aktuelle Registerabdruck AG Landau, HRB 33727 (Abruf 26.10.2026, 10:39), führt nicht „Südpfalz Metall GmbH“, sondern MBK Mietservice GmbH, Geschäftsanschrift Am Bellensee 6, 76865 Rohrbach bei Landau, Unternehmensgegenstand Verleih und Verkauf von Baumaschinen, Geschäftsführer Janneck Hieronymus Mata Knecht; letzter Registereintrag 03.07.2023. Eine Registersuche nach dem exakten Firmennamen „Südpfalz Metall GmbH“ bei AG Landau ergab 0 Treffer. Die Betreiberwebsite nennt im Impressum denselben HRB-Wert, Janneck Knecht und dieselbe Adresse, aber keinen Firmennamen. Das belegt eine Personen-/Adress-/Registernummernverbindung zu MBK, nicht die rechtliche Zuordnung oder unabhängige Bestätigung des beworbenen Schrottbetriebs; die registerliche Tätigkeit passt nicht zum Schrottangebot. Deshalb `status` von `aktiv` auf `pruefung` zurückgenommen (Identitäts-/Betriebsprüfung, keine Schließungsbehauptung). Die Gemeindeseite Rohrbach listet ausgewählte Betriebe des Gewerbegebiets, nicht Südpfalz Metall; sie sagt zugleich „und viele mehr“ und ist daher weder Vollregister noch Negativbeleg. `clear_fields: 'dropoff_json,geo'` bleibt angemessen: keine bestätigte Annahme für den Seed-Rechtsträger und kein belegter Ersatzpunkt; Adresse/Kontakte bleiben ungefüllt. `website_status: aktiv` beschreibt nur die erreichbare Website. Keine Produktionsdaten abgefragt oder geändert. Quellen: https://www.handelsregister.de/rp_web/erweitertesuche/welcome.xhtml (AG Landau, HRB 33727; aktueller Abdruck und exakte Firmensuche); https://suedpfalzmetall-gmbh.de/ ; https://suedpfalzmetall-gmbh.de/impressum ; kommunaler, nicht abschließender Gewerbeüberblick: https://www.rohrbach-pfalz.de/wirtschaft/gewerbegebiet/ (Abruf 26.10.2026).]

### Owner-GEO-/Annahme-Korrektur 24.10.2026

- [Owner-GEO-Korrektur 24.10.2026: Read-only-Produktionspunkt 49.81445, 9.499233 reverse-geocodiert zu MSP 31 bei Hasloch, Bayern, nicht zu Rohrbach in der Südpfalz. Außerdem enthielt die Produktionszeile `dropoff_json={"allowed":true}`. Die Betreiberwebsite beschreibt zwar einen Schrottplatz und Ankauf von Privatpersonen, die rechtliche Identität mit der Seed-Entität Südpfalz Metall GmbH bleibt mangels verifizierter Registerbrücke aber offen; die Recherche hat daher weder eine verifizierte Annahmestelle noch einen zugeordneten Punkt für diesen Seed. `clear_fields: 'dropoff_json,geo'` entzieht beide nicht ausreichend belegten Bestandswerte; kein Ersatzpunkt und keine Produktionsänderung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.81445&lon=9.499233&zoom=18&addressdetails=1 ; Betreiber-/Identitätsprüfung im Rechercheeintrag 24.10.2026; read-only `public.db`; `docs/trader-seed.md`.]

### Recherche 24.10.2026

- [Recherche 24.10.2026: Betreiberseiten Startseite, Leistungen und Impressum direkt gelesen. Der Auftritt wirbt ausdrücklich mit Ankauf von Metallen von Privatpersonen, Unternehmen und Industriebetrieben und nennt seine Anlage „Schrottplatz bei Landau“; Kontaktblock und Öffnungszeiten führen Am Bellensee 6, 76865 Rohrbach sowie Mo–Fr 08:00–12:00/13:00–16:00. Damit ist ein kundenorientierter Standort auf der Website beschrieben, nicht nur eine abstrakte Kontaktseite. Die rechtliche Brücke zur Seed-Entität bleibt aber offen: Im Impressum steht Janneck Knecht mit HRB 33727/AG Landau, kein Firmenname; ein unabhängiger Registertreffer wurde in dieser Prüfung nicht verifiziert. Website-Unterseiten sind keine voneinander unabhängigen Quellen, daher keine Frontmatter-Fills und kein GEO. **Ankauf:** nur Werbeaussagen/„faire Preise“, keine Zahlenkurse; **Verkauf:** keine Produktpreisliste; **Gebühren:** Container-/Abholkosten nicht beziffert. Möglicher GEO-Kandidat erst nach Betreiber-/Identitätsbestätigung: Am Bellensee 6, Rohrbach. Quellen (ein Betreiberauftritt): https://suedpfalzmetall-gmbh.de/ ; https://suedpfalzmetall-gmbh.de/leistungen/ ; https://suedpfalzmetall-gmbh.de/impressum .]

### Recherche 13.10.2026

- [Recherche 13.10.2026: Homepage, Leistungen und Impressum aktuell direkt abgerufen. Leistungen nennen Ankauf von Mischschrott, Kupfer, Messing, Kabeln, Altzinkblech und weiteren Metallen, Abholung „größerer Mengen“ sowie Container; Adresse, Telefon, E-Mail und Öffnungszeiten stehen im Seitenfuß. Impressum nennt Janneck Knecht, Am Bellensee 6, HRB 33727, Registergericht Amtsgericht Landau; die sichtbaren Kontaktfelder im Impressum selbst enthalten weiterhin Platzhalter, die Angaben stehen separat im Seitenfuß. Firmenname fehlt im Impressum, und ein unabhängiger Registertreffer/Live-Status zu HRB 33727 wurde nicht bestätigt. Daher weiterhin Einzelbeleg und keine Felder gefüllt; kein Schluss zur Betriebsaktivität allein aus der abrufbaren Website. ANKAUF: nur „faire Preise“/Angebot auf Anfrage, keine Zahlenkurse; VERKAUF: keine Warenpreisliste; GEBÜHREN: Container-/Abholkosten nicht veröffentlicht. Ein einzelner Homepage-Kundenkommentar („komplett kostenlos“) ist anekdotisch, keine allgemeine Gebührenregel. Quelle(n): https://suedpfalzmetall-gmbh.de/; https://suedpfalzmetall-gmbh.de/leistungen/; https://suedpfalzmetall-gmbh.de/impressum]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Re-Check Homepage 02.10.2026 bestätigt Kontaktdaten aus Klärfall-Notiz 01.10.2026 (Am Bellensee 6, 76865 Rohrbach, Tel. +49 176 24226146, info@suedpfalzmetall-gmbh.de, Mo-Fr 08:00-12:00/13:00-16:00; Leistungen: Altmetall-Ankauf Alu/Edelstahl/Buntmetalle, Mischschrott-Recycling, Abholservice, Kupfer/Messing/Kabel). Weiterhin EINZIGE Quelle (Betreiber, schwaches Impressum ohne verifizierbaren HRB-Treffer), daher kein Frontmatter-Fill, Status aktiv mit website aktiv bleibt bestehen, lat/lon-Fehlvermerk (Unterfranken statt Rohrbach, Koordinaten neu zu geocodieren) bleibt gültig. Quelle(n): suedpfalzmetall-gmbh.de Homepage + /impressum (eine schwache Quelle, Re-Check)]

- [Recherche 01.10.2026: Feedback 2599 berechtigt als Klärfall. lat/lon-Fehler bestätigt: DB Unterfranken statt 76865 Rohrbach (Koordinaten neu zu geocodieren). Betreiber-Impressum schwach: nennt Person statt Firma, Kontaktfelder mit Platzhaltern, HRB-33727-Angabe ohne Northdata-Treffer (unverifiziert). Footer-Kontakt Am Bellensee 6, 76865 Rohrbach, Tel. +49 176 24226146, info@suedpfalzmetall-gmbh.de, Mo-Fr 08:00-12:00/13:00-16:00 — NUR hier, Frontmatter leer. NICHT mit Braun (gleiche Adresse) mergen, gegenseitig referenzieren. website_status aktiv (HTTP 200); Quelle(n): https://suedpfalzmetall-gmbh.de/impressum (eine schwache Quelle)]

### Importiert (Seed-Stand 2026-09-30)

- Altmetalle, Mischschrott, Kupfer, Messing, Kabel, Zinkblech

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Koordinate gesetzt: 49.1329322, 8.1366168** (Mittelwert der eindeutigen Knoten des
  Objekts **way/373032573**, vier eindeutige Knoten; der Punkt liegt innerhalb der Objekt-BBox
  8.1365207/49.1328691 bis 8.1367130/49.1329954). **Quelle 1 (OSM, live geladen am 04.11.2026, HTTP 200):**
  **way/373032573** trägt `addr:street`=Am Bellensee, `addr:housenumber`=**6**, `addr:city`=Rohrbach,
  `addr:postcode`=76865, `addr:country`=DE, `building`=yes und `source`=bing - Straße, Hausnummer, PLZ und Ort
  stimmen mit der auf der Betreiberwebsite genannten Anschrift **Am Bellensee 6, 76865 Rohrbach** überein.
  **Fensterprüfung (kein Fensterfehler):** das am 04.11.2026 live abgerufene Kachelraster über Rohrbach umfasst
  **8.1200/49.1250 bis 8.1450/49.1500**, **9 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler**; das Fenster enthält
  die gesuchte Straße **nachweislich** mit **5** Adressobjekten auf „Am Bellensee" und den Hausnummern 3, 4, 5, 6 und
  10 - die **Dossier-Hausnummer 6 existiert** und ist im Raster das einzige Objekt mit dieser Nummer auf dieser
  Straße. Eine Namenssuche nach **„Südpfalz"** über die Felder name, operator, description, website,
  contact:website, brand, note und addr:housename findet im Raster **kein Objekt mit Betriebsbezug zum Dossierbetrieb** (die Namensobjekte dort sind
  „Clubhaus SpVgg Rohrbach", „Schützenhaus" und „Betonwerk Dyckerhoff") - es gibt also **keine** OSM-Betriebszuordnung,
  nur die Adressangabe. Quelle 2 (Betreiber, im Dossier bereits zitiert, live nachgelesen am 04.11.2026): die
  **Betreiberwebsite** https://suedpfalzmetall-gmbh.de/ liefert **HTTP 200** und nennt live im Kontaktblock
  **„Adresse: Am Bellensee 6, 76865 Rohrbach"**, Telefonnummer **+49 176 24226146**,
  `info@suedpfalzmetall-gmbh.de` und die Öffnungszeiten Mo–Fr 08:00–12:00 / 13:00–16:00; das **Impressum** nennt
  live **„Janneck Knecht, Am Bellensee 6, 76865 Rohrbach, Handelsregister: HRB 33727, Registergericht: Amtsgericht
  Landau, USt-IdNr DE36 1039 244"** - dieselbe Anschrift wie im Dossier. **Zurücknahme des `geo`-Tokens
  (Begründung):** `clear_fields: 'dropoff_json,geo'` wurde um das Token **geo** gekürzt (das Token `dropoff_json`
  bleibt **unverändert** bestehen), weil der Seed jetzt eine belegte Koordinate führt; der Seed lehnt ein
  `geo`-Clear neben belegtem `lat`/`lon` ab, und der Punkt liegt exakt auf der von der Betreiberwebsite genannten
  Anschrift. **Abgrenzung (ausdrücklich offengelegt):** Die Koordinate bezeichnet die **Anschrift**, an der die
  Betreiberwebsite den Schrottplatz bewirbt; sie **behauptet keine** rechtliche Zuordnung des Seed-Namens
  **„Südpfalz Metall GmbH"** zu dieser Adresse. Der im Dossier dokumentierte **Registerabdruck** ordnet HRB 33727
  (AG Landau) der **MBK Mietservice GmbH** mit Geschäftsanschrift **Am Bellensee 6, 76865 Rohrbach** und
  Geschäftsführer Janneck Hieronymus Mata Knecht sowie dem Unternehmensgegenstand „Verleih und Verkauf von
  Baumaschinen" zu; eine Registersuche nach dem exakten Firmennamen „Südpfalz Metall GmbH" ergab **0 Treffer**.
  Personen-, Adress- und Registernummernverbindung sind damit stark, die **rechtliche Zuordnung des
  Schrottbetriebs zum Seed-Rechtsträger bleibt offen** - die Koordinate löst diese Frage **nicht** auf und ist
  **kein** Beleg für eine aktive Annahme. **Qualifizierung (unverändert übernommen):** `status: pruefung`, `city`,
  `street` (leer), `postcode` (leer), `phone` (leer), `email` (leer), `opening_hours` (leer), `website`,
  `website_status` und `description` (leer) bleiben unverändert; die im Dossier festgehaltene **Lücke bei den
  Kontaktfeldern** (Platzhalter im Impressum) und das **Fehlen bezifferter Preise oder Gebühren** bleiben
  unverändert. Quelle(n): https://api.openstreetmap.org/api/0.6/way/373032573/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.1200,49.1250,8.1450,49.1500 ;
  https://suedpfalzmetall-gmbh.de/ ; https://suedpfalzmetall-gmbh.de/impressum (Abruf 04.11.2026).]
