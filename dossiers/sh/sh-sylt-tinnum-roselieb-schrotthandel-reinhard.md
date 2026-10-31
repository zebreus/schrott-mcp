---
slug: sh-sylt-tinnum-roselieb-schrotthandel-reinhard
name: Roselieb Schrotthandel GmbH, Reinhard
trader_type: schrotthaendler
state: SH
city: Sylt/Tinnum
street: Liiger Hörn 3
postcode: 25980
phone: 04651 935509
email: ''
opening_hours: ''
website: https://roselieb-gmbh.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 3 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Roselieb Schrotthandel GmbH, Reinhard

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Insulaner-Betrieb, Branche „Schrott" (Gelbe Seiten, Tel. 04651 93 55 09)

### Korrektur 30.09.2026

- [Korrektur 30.09.2026: status aktiv → pruefung — Existenz nur durch Verzeichnisbeleg, kein unabhaengiger Zweitbeleg (Feedback 1464/1465); Re-Pruefung per Folgewelle.]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Fill Liiger Hörn 3, 25980 Sylt/Tinnum, Website roselieb-gmbh.de (aktiv); Caveat: Betreiber-Seite (Impressum HRB 100262, Kontakt) nennt Sylt-Standort nicht explizit, daher kein aktiv-Upgrade, Status bleibt pruefung; Adressänderung: Koordinaten neu zu geocodieren; Quelle(n): Gelbe Seiten, Betreiber-Seiten roselieb-gmbh.de]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprueft (Liiger Hörn, Tinnum) - die Dossier-Hausnummer 3 existiert dort nicht.** Kachelraster ueber den **gesamten** Strassenverlauf (Strassenbbox 8.3232640/54.8956131 bis 8.3287995/54.8980677, **1 Kachel, 0 Kachel-Fehler**); die Bbox wurde aus der **vollen Geometrie** der vier Strassenways mit dem OSM-Strassennamen **Liiger Hörn** gebildet (way/120271108, way/120271109, way/120271113, way/120271122). Zaehlung getrennt: **20 Adressobjekte / 20 verschiedene Hausnummern** auf der Liiger Hörn (2, 4, 6, 7, 8, 10, 11, 12, 14, 15, 16, 18, 19, 20, 22, 30, 36, 42, 48, 58); im Raster tragen **0** der 20 Adressobjekte `name=` und **0** `operator=`. **Die Hausnummer 3 existiert in OSM auf der Liiger Hörn NICHT** - die Strasse ist in OSM lueckenlos durchnummeriert, die Nummer 3 fehlt zwischen 2 und 4. Das ist **zu schreiben, nicht zu uebergehen**. Die Adressobjekte tragen durchgaengig `addr:city=Sylt`, `addr:suburb=Tinnum` und `addr:postcode=25980` - so node/10074902694 mit `addr:housenumber=2`, `addr:street=Liiger Hörn`, `addr:city=Sylt`, `addr:suburb=Tinnum`, `addr:postcode=25980` (ohne `building`-Tag) und way/196798860 mit `addr:housenumber=19`, `addr:street=Liiger Hörn`, `addr:city=Sylt`, `addr:suburb=Tinnum`, `addr:postcode=25980`, `building=house` - **kein** Objekt traegt einen `name=`- oder `operator=`-Bezug zum Betrieb; das ist eine Aussage ueber den OSM-Bestand und **kein** Negativbeleg ueber den Betrieb. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3232640,54.8956131,8.3287995,54.8980677 ; https://api.openstreetmap.org/api/0.6/node/10074902694.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Die Website im Dossier gehoert zu einem ANDEREN Unternehmen - die Verwechslung ist aufgeklaert.** Das Dossier-Feld `website` zeigt auf **https://roselieb-gmbh.de**. Das Impressum dieser Seite (https://www.roselieb-gmbh.de/impressum/, HTTP 200 am 31.10.2026) nennt „**Reinhard Roselieb Containerdienst, Schrott- und Metallhandelsges. mbH**, diese vertreten durch die Geschäftsführer: **Reinhard Roselieb sen.**, **Reinhard Roselieb jun.**, Eingetragener Firmensitz **Bleckenweg 106, 29227 Celle**, Telefon **05141 / 83353**, E-Mail **reinhard@roselieb-gmbh.de**, Amtsgericht Lüneburg **HRB 100262**, USt-IdNumber **DE 115 116 579**, Steuer-Nummer FA Celle 17/203/51314 - **keine** Sylter Adresse. Die Startseite beschreibt „Familiengeführter Betrieb mit über 55 Jahren Erfahrung im **Containerdienst, Schrott- und Metallhandel**“ mit der „**Mickey Mouse**“-Meldung („Auf Celles Straßen unterwegs“) und einem Onlineshop fuer Abfallarten (Altholz, Baumischabfall, Bauschutt, Erdaushub, Gruenabfall, Siedlungsabfall/Sperrmuell, „**Schrott-Abholung**“). **Konsequenz:** `roselieb-gmbh.de` ist die Website der **Celler** Gesellschaft; der **Sylter** Betrieb „Roselieb Schrotthandel GmbH, Reinhard“ (Liiger Hörn 3, 25980 Sylt-Tinnum, Tel. 04651 935509) hat **keine eigene** Website. Das Dossier-Feld `website` bleibt **unveraendert** (die Aenderung der Quellenliste bzw. das Entfernen der URL ist nicht zulaessig), die Zuordnung wird aber in diesem Vermerk **ausdruecklich relativiert**: die Gelbe Seiten-Seite des Sylter Eintrags verlinkt „Webseite“ auf **http://www.roselieb-gmbh.de** und setzt die Kontakt-E-Mail auf **reinhard@roselieb-gmbh.de** - die Verzeichnisse verweisen also auf die Celler Firma. `website_status: aktiv` ist fuer die Domain korrekt (HTTP 200), sagt aber **nichts** ueber den Sylter Standort. Quelle(n): https://www.roselieb-gmbh.de/impressum/ ; https://www.roselieb-gmbh.de/ ; https://www.gelbeseiten.de/gsbiz/1215227d-65d0-4808-ab43-ff7a8cca833f ; https://www.dasoertliche.de/Themen/Liiger-H%C3%B6rn/Tinnum-Gem-Sylt.htm (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Die Dossier-Adresse ist durch vier unabhaengige Verzeichnisse gedeckt, der Beleg fuer den Schrottbetrieb bleibt aber duenn.** Die Adresse **Liiger Hörn 3, 25980 Sylt-Tinnum** und die Telefonnummer **04651 93 55 09** stehen **deckungsgleich** in Gelbe Seiten, Das Oertliche (Branche „Recycling“ und „Schrott“), 11880 („Altmaterialhandel“, „Schrotthandel“, zuletzt aktualisiert 14.09.2022, Eintragsdaten 29.06.2021) und Oeffnungszeitenbuch. Das Oertliche weist in einer Umkreisliste weitere Schrotthaendler der Insel aus (Schrotthandel Nissen, Hochhörn 23, 25927 Neukirchen, 22,11 km; Hals Metall AG, Degelk 7, 25946 Norddorf, 24,07 km) - das stuetzt die **Existenz eines Schrotthandels auf Sylt**, nicht aber zwingend den Dossier-Datensatz. Die Vorwellen-Notiz vom 02.10.2026 („Betreiber-Seite (Impressum HRB 100262, Kontakt) nennt Sylt-Standort nicht explizit, daher kein aktiv-Upgrade“) ist damit **unveraendert bestaetigt**; `status: pruefung` bleibt korrekt. Quelle(n): https://www.gelbeseiten.de/gsbiz/1215227d-65d0-4808-ab43-ff7a8cca833f ; https://www.dasoertliche.de/Themen/Liiger-H%C3%B6rn/Tinnum-Gem-Sylt.htm ; https://www.11880.com/branchenbuch/sylt/060440090B41995641/roselieb-schrotthandel-gmbh-reinhard.html ; https://www.oeffnungszeitenbuch.de/filiale/Sylt-Roselieb%2520Schrotthandel%2520GmbH%252C%2520Reinhard-2531902U.html (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise und Gebuehren getrennt.** ANKAUF: **keine Ankaufspreisliste** fuer den Sylter Standort; die Celler Website der gleichnamigen Gruppe hat einen Onlineshop fuer Abfallarten mit „Schrott-Abholung“, nennt aber auf der Startseite **keine** Preise (die Preis-Seiten des Celler Shops sind JS-gerendert und wurden in dieser Recherche **nicht** ausgewertet) - die Preise der Celler Gesellschaft duerfen dem Sylter Betrieb **nicht** zugerechnet werden. VERKAUF: **keine Verkaufspreisliste** fuer Sylt. GEBUEHREN: **keine Tarife**; die Oeffnungszeitenbuch fuehrt den Sylter Eintrag ohne Oeffnungszeiten („Keine Öffnungszeiten“), es gibt also auch keine belegte Annahmezeit. Quelle(n): https://www.roselieb-gmbh.de/ ; https://www.11880.com/branchenbuch/sylt/060440090B41995641/roselieb-schrotthandel-gmbh-reinhard.html ; https://www.oeffnungszeitenbuch.de/filiale/Sylt-Roselieb%2520Schrotthandel%2520GmbH%252C%2520Reinhard-2531902U.html (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Fragen.** 1) Ob „Roselieb Schrotthandel GmbH, Reinhard“ auf Sylt eine **eigenstaendige Gesellschaft** oder eine **Ausseneinheit/Tochter** der Celler Reinhard Roselieb Containerdienst, Schrott- und Metallhandelsges. mbH (HRB 100262) ist, laesst sich aus den verfuegbaren Quellen **nicht** entscheiden - das Register der Celler Gesellschaft nennt nur den Sitz in Celle. Der Namensbestandteil „Reinhard“ im Dossier-Namen und die von den Verzeichnissen verlinkte Celler Adresse sprechen fuer eine Verbindung, sind aber **kein** Registerbeleg. 2) Die **Hausnummer 3** ist in OSM nicht erfasst; der Seed-Punkt ist **neu zu geokodieren**. Zu geokodieren ist: **Liiger Hörn 3, 25980 Sylt-Tinnum**. 3) **Oeffnungszeiten** sind fuer den Sylter Standort **nicht** belegt (11880: „Keine Oeffnungszeiten“); `opening_hours` bleibt leer. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3232640,54.8956131,8.3287995,54.8980677 ; https://www.roselieb-gmbh.de/impressum/ ; https://www.oeffnungszeitenbuch.de/filiale/Sylt-Roselieb%2520Schrotthandel%2520GmbH%252C%2520Reinhard-2531902U.html ; read-only `public.db` (31.10.2026).]
