---
slug: mv-trollenhagen-ds-rohstoffhandel
clear_fields: 'geo'
name: DS Rohstoffhandel
trader_type: schrottplatz
state: MV
city: Trollenhagen
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
provenance_section: Betriebe (Tabelle)
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# DS Rohstoffhandel

## Überblick

DS Rohstoffhandel ist nur über Portal-Leads (Fuchsberg 14, 17039 Trollenhagen-Podewall, Tel. 0395 4528862) greifbar. Die ähnlich benannte Domain ds-containerdienst.de gehört laut Vorrecherche einem anderen Unternehmen; Zuordnung, aktuelle Tätigkeit und Annahme bleiben offen. **Ankauf:** nicht bestätigt, keine Kurse. **Verkauf:** kein Produkt-/Preisblatt. **Gebühren:** keine verifizierten Abhol-, Container- oder Entsorgungstarife. Adresse nicht geokodieren, bevor Betreiber und tatsächlicher Kundenstandort bestätigt sind.

## Timeline

### Owner-GEO-Korrektur 25.10.2026

- [Owner-GEO-Korrektur 25.10.2026: Read-only `public.db` enthält 53.6071, 13.291. Nominatim-Reverse löst den Punkt an Kirchstraße 42 in Trollenhagen auf; der einzige Dossier-Adresslead ist dagegen Fuchsberg 14 in Trollenhagen-Podewall und nicht als Betreiberstandort bestätigt. `clear_fields: 'geo'` entzieht das nicht adressgenaue Paar beim nächsten regulären Seed. Keine Ersatzkoordinate. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.6071&lon=13.291&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 01.10.2026

- [Recherche 01.10.2026: nur Portal-Leads (schrottplatz-info/org: Fuchsberg 14, 17039 Trollenhagen Podewall, 0395 4528862) = kein Beleg; kein Betreiber-Webauftritt (ds-containerdienst.de ist anderes Unternehmen), kein HR-/Kommunalbeleg gefunden. Nichts gefüllt, Klärfall. Quelle(n): Websuche 01.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- Rohstoffhandel (lt. schrottplatz-info.de)

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Adresse **Fuchsberg 14, 17039
  Trollenhagen-Podewall** ist in OSM **nicht** erfasst; im abgetasteten Ortsbereich von Trollenhagen gibt es **keine**
  Straße dieses Namens. Quelle 1 (OSM, live geprüft am 04.11.2026): **Fensterprüfung (kein Fensterfehler):** das am
  04.11.2026 live abgerufene Kachelraster umfasst **13.2900/53.6000 bis 13.3000/53.6100**, **1 Kachel von je 0,01
  Grad**, **0 Kachel-Fehler**; die Kachel erfasst den **Ortsteil Trollenhagen** (POST 17039) mit **84** Adressobjekten
  auf **5** Straßen - **Am Anger** (1 bis 19 und 1a), **Am Weiher** (1 bis 19, 1a, 1b), **Buchhofer Straße** (1 bis 18
  und 12a, 12b), **Flughafenstraße** (9, 10) und **Kirchstraße** (2 bis 30) - eine Straße **„Fuchsberg"** ist dort
  **nicht** erfasst, und die Dossier-Hausnummer **14** findet sich auf keiner der fünf Straßen des Kachelausschnitts.
  Eine Suche nach **„DS Rohstoffhandel"**, **„Fuchsberg"**, **„Schrott"** und **„Altmetall"** über die Felder name,
  operator, brand, description, website, contact:website, shop, craft, amenity, landuse, man_made und industrial findet
  im Kachelausschnitt **keinen** Treffer - es gibt **keine** OSM-Betriebszuordnung. Der Kachelausschnitt deckt **nicht**
  den ganzen Ortsteil ab (die Ortsteilgrenze von **Trollenhagen-Podewall** liegt nördlich davon), das Fehlen der Straße
  darin ist deshalb **kein** Nachweis gegen ihre Existenz. Quelle 2 (Verzeichnis, im Dossier zitiert): die Portal-Leads
  (schrottplatz-info/schrottplatz.org) nennen **Fuchsberg 14, 17039 Trollenhagen Podewall**, Telefon 0395 4528862 - ein
  **Verzeichnislead** ohne Betreiberauftritt; die im Dossier genannte Domain **ds-containerdienst.de** gehört einem
  **anderen** Unternehmen und wurde **nicht** übernommen; ein HR- oder Kommunalbeleg ist nicht auffindbar. Ein erneuter
  Direktabruf der Portal-Adresse https://www.schrottplatz-info.de/schrottplatz/Trollenhagen/DS-Rohstoffhandel am
  04.11.2026 liefert zwar **HTTP 200**, führt aber auf die **Übersichtsseite „Schrottplätze nach Stadt"** und **nicht**
  mehr auf den Einzel-Eintrag, sodass die Lead-Angabe heute aus dieser Quelle **nicht mehr auslesbar** ist; der Altpunkt
  53.6071/13.291 löst live auf **Kirchstraße 42** in Trollenhagen auf und damit **nicht** auf die Dossier-Adresse, er
  bleibt über `clear_fields: 'geo'` zurückgenommen. **Warum keine Koordinate:** die Adresse ist heute **nicht mehr
  belegbar** (Portal-Eintrag nicht mehr abrufbar, keine OSM-Adresse) und es existiert **keine** Betreiber-, Register-
  oder Kommunalquelle; eine Punktwahl wäre eine **Rathausnahme**. `clear_fields: 'geo'` **bleibt** unverändert bestehen
  (es gibt keinen Ersatzpunkt), `status: pruefung` bleibt, Ankauf-, Verkaufs- und Gebührenangaben bleiben unbelegt, es
  wird **keine** Schließung behauptet. **Reichweitenklausel:** die Nichtabrufbarkeit des Portal-Eintrags und das Fehlen
  eines OSM-Tagings sind **kein** Nachweis für das Fehlen des Betriebs. Offen bleiben Inhaber und die bestätigte
  Anschrift; erst danach ist zu geokodieren. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=13.29,53.60,13.30,53.61 ;
  https://www.schrottplatz-info.de/schrottplatz/Trollenhagen/DS-Rohstoffhandel (am 04.11.2026 auf die Übersichtsseite
  umgeleitet) ; Websuche vom 01.10.2026 im Dossier dokumentiert (Abruf 04.11.2026).]
