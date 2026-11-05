---
slug: rp-ludwigshafen-ah
clear_fields: 'geo'
name: AH
trader_type: schrotthaendler
state: RP
city: Ludwigshafen
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
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# AH

## Überblick

„AH“ in Ludwigshafen bleibt ein ungeklärter Seed-Klärfall. Eine echte, ähnlich benannte AH GmbH Metall- und Schrotthandel betreibt eine Website für Mannheim (Fruchtbahnhofstraße 13, 68159 Mannheim; HRB 737082); die abweichende Stadt und der fehlende Identitätsbeleg reichen nicht aus, um diese Gesellschaft dem Ludwigshafener Datensatz zuzuordnen.

**Preise:** **ANKAUF** – die Mannheimer Website veröffentlicht keine konkreten Schrottkurse; dies ist nicht dem Ludwigshafener Dossier zurechenbar. **VERKAUF** – keine öffentliche Preisliste gefunden. **GEBÜHREN** – Containergrößen von 3–40 m³ sind genannt, aber kein Tarif; keine Preisübertragung auf Ludwigshafen.

## Timeline

### Recherche 27.10.2026

- [Recherche 27.10.2026: Die Domains der namensähnlichen AH GmbH in Mannheim wurden erneut direkt aufgerufen, lieferten jedoch keine lesbaren Seiteninhalte. Der vorhandene frühere Betreiber-/Impressumsbeleg für AH GmbH, HRB 737082, Fruchtbahnhofstraße 13, Mannheim, bleibt vom Ludwigshafener Seed getrennt; der aktuelle Abruffehler beweist weder Aufgabe noch eine Verbindung. Kein Feld-/GEO-Fill. ANKAUF: für „AH“ Ludwigshafen keine Kurse; die getrennte Mannheimer Firma hatte ebenfalls keine veröffentlichten Zahlenkurse. VERKAUF: keine zurechenbare Liste. GEBÜHREN: keine zurechenbaren Tarife. Quellen: https://ah-schrotthandel.de/ ; https://ah-schrotthandel.de/impressum/ (Transportfehler am 27.10.2026; nicht als Schließungsnachweis gewertet).]

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 49.4866, 8.4181. Nominatim-Reverse löst den Punkt an der Industriestraße 2b in Ludwigshafen-Friesenheim auf. Das Dossier hat keine Betreiber-/Adressbrücke zu diesem Ort; die einzige konkrete namensähnliche AH GmbH ist in Mannheim. `clear_fields: 'geo'` entzieht den nicht zugeordneten Altpunkt beim nächsten regulären Seed. Keine Ersatzkoordinate. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.4866&lon=8.418099999999999&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: Erneute Namens-/Ortsprüfung erbrachte keinen eigenständigen Betreiberbeleg für „AH“ in Ludwigshafen. Die einzige konkret beschriebene namensähnliche Gesellschaft bleibt die AH GmbH Metall- und Schrotthandel in Mannheim (Fruchtbahnhofstraße 13, 68159; HRB 737082); abweichender Ort und fehlende Identitätsbrücke schließen eine Übernahme aus. Deren Betreiberseiten liefern ebenfalls keine bezifferten Ankaufkurse, Verkaufspreise oder Container-/Entsorgungstarife; nicht dem Ludwigshafener Seed zurechenbar. ANKAUF/VERKAUF/GEBÜHREN für diesen Seed: offen. Keine Felder oder GEO geändert. Quellen zur Abgrenzung: https://ah-schrotthandel.de/ ; https://ah-schrotthandel.de/impressum/ .]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (nur Kürzel überliefert)
- Adresse: Ludwigshafen

### Recherche 08.10.2026

- [Recherche 08.10.2026: Namensgleicher Kandidat klar abgegrenzt, nicht zugeordnet: Betreiber-Website und Impressum führen die AH GmbH Metall- und Schrotthandel, Geschäftsführer Mehmet Ali Halici, HRB 737082, Fruchtbahnhofstraße 13, 68159 Mannheim, Tel. 0621 17895089, E-Mail info@ah-schrotthandel.de. Betreiberseiten beschreiben Recycling/Containerdienst (3–40 m³), nennen aber keine konkreten Ankaufspreise, Verkaufspreise oder Container-/Entsorgungsgebühren. Mannheim ist nicht Ludwigshafen; kein Beleg verbindet diese Firma mit dem Seed „AH“ in Ludwigshafen, daher keinerlei Felder übertragen. Quelle(n): https://ah-schrotthandel.de/ ; https://ah-schrotthandel.de/impressum/]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Zeile trägt nur das **Kürzel „AH“** und
  `city`=**Ludwigshafen**; `street` und `postcode` sind **leer**, und die einzige konkret belegte Gesellschaft mit
  ähnlichem Namen ist die **AH GmbH Metall- und Schrotthandel** in **Mannheim** - eine **andere Stadt**, ohne
  Identitätsbrücke. **Was geprüft wurde (live am 03.11.2026):** (a) Die OSM-Prüfung im Ludwigshafener Raum
  (Kachelraster über **bbox=8.4000,49.4700,8.4500,49.5100**, live abgerufen 03.11.2026) findet **kein** Objekt
  mit `name=`/`operator`=**AH** oder einem Schrott-/Metallbetriebs-Bezug zu diesem Kürzel; die dort gefundenen
  Adressobjekte tragen durchweg **andere** Betriebe (z. B. node/12102657950 „Bulgarien Market“ Rottstraße 95,
  way/12988380859 Moltkestraße 9-11) - ein **Objekt, das den eigenen Betrieb benennt**, existiert **nicht**. (b) Die
  **Namensvetter-Kette** aus dem Dossier bleibt unverändert: Die **AH GmbH**, Geschäftsführer **Mehmet Ali Halici**,
  **HRB 737082**, **Fruchtbahnhofstraße 13, 68159 Mannheim**, Telefon 0621 17895089,
  info@ah-schrotthandel.de (https://ah-schrotthandel.de/ und /impressum/, live abgerufen 03.11.2026) ist **kein**
  Ludwigshafener Betrieb; Mannheim ist **nicht** Ludwigshafen, und es fehlt jeder Beleg, der diese Firma mit dem
  Seed-Kürzel „AH“ in Ludwigshafen verbindet. (c) Ein **eigenes** Ludwigshafener Unternehmen mit den Initialen
  „AH“ ist im Dossier **nicht** dokumentiert. **Quelle 1 (Betreiber):** die Betreiber-Website
  ah-schrotthandel.de (live abgerufen 03.11.2026) - sie belegt den **Mannheimer** Betrieb, **nicht** die
  Ludwigshafener Zeile. **Quelle 2 (OSM):** das Kachelraster und die genannten Adressobjekte (live abgerufen
  03.11.2026 über https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.4700,8.4500,49.5100). **Was fehlt:**
  eine **Betreiber- oder Registerquelle** für ein **Ludwigshafener** Unternehmen mit dem Kürzel „AH“ samt
  Anschrift; ohne sie ist die Zuordnung des NamensveTTers **nicht** möglich und ein Punkt wäre geraten. `status`
  bleibt **pruefung**, `street`/`postcode`/`website` bleiben leer, `clear_fields: 'geo'` bleibt bestehen. Ein
  **Nichtexistenz- oder Schließungsbeweis** ist damit **nicht** verbunden. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4000,49.4700,8.4500,49.5100 (Kachelraster, live abgerufen
  03.11.2026) ; https://ah-schrotthandel.de/impressum/ (live abgerufen 03.11.2026) ; Recherche-Vermerke vom
  08.10. und 27.10.2026 im Dossier.]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - weiterhin keine Koordinate.** Die Zeile trägt nur das **Kürzel „AH"** und
  `city`=**Ludwigshafen**; `street` und `postcode` sind **leer**, und die einzige konkret belegte Gesellschaft mit
  ähnlichem Namen ist die **AH GmbH Metall- und Schrotthandel** in **Mannheim** - eine **andere Stadt**, ohne
  Identitätsbrücke. **Was geprüft wurde (live am 05.11.2026):** (a) **Suchfenster (0,02°-Kacheln):** **bbox=8.4200,
  49.4700,8.4600,49.4900** = 2 Kacheln à 0,02° (8.4200/49.4700/8.4400/49.4900 und 8.4400/49.4700/8.4600/49.4900), live
  abgerufen am 05.11.2026; **Zensus:** **4.012** Adressobjekte im Fenster; eine Suche über **alle** Tag-Werte nach dem
  eigenständigen Token **„AH"** (Wortgrenze) liefert **0 Objekte** in **66.203** Elementen - ein Objekt mit einem
  `name=`/`operator=`-**AH**-Bezug existiert **nicht**. Eine Suche nach „**schrott**", „**recycl**", „**containerdienst**",
  „**metallhandel**" oder „**schrottplatz**" liefert **40** Treffer, die **sämtlich kommunale Wertstoffcontainer**
  (`amenity`=recycling, teils `operator`=Knettenbrech + Gurdulic) sind - **kein** Objekt benennt einen Schrottbetrieb
  oder ein Unternehmen mit dem Kürzel „AH". (b) Die **Namensvetter-Kette** aus dem Dossier bleibt unverändert: Die
  **AH GmbH**, Geschäftsführer **Mehmet Ali Halici**, **HRB 737082**, **Fruchtbahnhofstraße 13, 68159 Mannheim**, Telefon
  0621 17895089, info@ah-schrotthandel.de (https://ah-schrotthandel.de/ und /impressum/, live abgerufen 03.11.2026) ist
  **kein** Ludwigshafener Betrieb; Mannheim ist **nicht** Ludwigshafen, und es fehlt jeder Beleg, der diese Firma mit dem
  Seed-Kürzel „AH" in Ludwigshafen verbindet. (c) **Registerabgleich (neu):** Der Abzug des **Fachbetrieberegisters der
  Gemeinsamen Stelle Altfahrzeuge und des Registers der Entsorgungsfachbetriebe** (Stand 11.08.2026, über den Spiegel
  schrottregister.pages.dev) führt für **Ludwigshafen am Rhein** dreizehn Betriebe (Autoverwertung Ludwigshafen,
  Flomersheimer Straße 51; Günther Schmelzer GmbH, Erbachstraße 23; Höhl KG, Torfstechering 3 [2 Einträge]; L & Z
  Transformatoren und Industrieservice GmbH, Rheinhorststraße 37; Müllheizkraftwerk Ludwigshafen, Industriestraße 1;
  R. S. Abschleppdienst KG, Torfstechering 5; Rhein-Fass GmbH & Co. KG, Großpartstraße 2 a; Scherer & Kohl GmbH,
  Inselstraße 20; Wirtschaftsbetr. Ludwigshafen, Kaiserwörthdamm 3; Betriebsstätte Ludwigshafen, Saarburgerstr. 33;
  Horst Lehr, Lagerplatz) - **keinen** mit dem Kürzel „AH". **Quelle 1 (Betreiber):** die Betreiber-Website
  ah-schrotthandel.de (live abgerufen 03.11.2026) - sie belegt den **Mannheimer** Betrieb, **nicht** die Ludwigshafener
  Zeile. **Quelle 2 (OSM):** das Kachelraster (live abgerufen 05.11.2026 über
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4200,49.4700,8.4400,49.4900 und
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4400,49.4700,8.4600,49.4900). **Was fehlt:** eine **Betreiber-
  oder Registerquelle** für ein **Ludwigshafener** Unternehmen mit dem Kürzel „AH" samt Anschrift; ohne sie ist die
  Zuordnung des Namensvetters **nicht** möglich und ein Punkt wäre geraten. `status` bleibt **pruefung**,
  `street`/`postcode`/`website` bleiben leer, `clear_fields: 'geo'` bleibt bestehen. **Reichweitenklausel:** Das Fehlen
  eines Betriebsobjekts in OSM oder eines Registereintrags ist **kein** Nichtexistenz- oder Schließungsbeweis.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4200,49.4700,8.4400,49.4900 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.4400,49.4700,8.4600,49.4900 ;
  https://ah-schrotthandel.de/impressum/ ; https://schrottregister.pages.dev/ort-ludwigshafen ;
  Recherche-Vermerke vom 08.10. und 03.11.2026 im Dossier (live abgerufen 05.11.2026).]
