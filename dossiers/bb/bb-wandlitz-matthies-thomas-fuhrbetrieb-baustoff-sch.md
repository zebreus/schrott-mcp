---
slug: bb-wandlitz-matthies-thomas-fuhrbetrieb-baustoff-sch
name: Matthies Thomas Fuhrbetrieb/Baustoff/Schrotthandel
trader_type: schrotthaendler
state: BB
city: Wandlitz
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
clear_fields: 'geo'
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Brandenburg (BB) – Recherche
provenance_ankauf_raw: ja
provenance_origin: table
---

# Matthies Thomas Fuhrbetrieb/Baustoff/Schrotthandel

## Überblick

Ein Gelbe-Seiten-Eintrag für „Matthies Thomas Fuhrbetrieb, Baustoff u. Schrotthandel“ nennt Liebenwalder Str. 8, 16348 Wandlitz-Zerpenschleuse, 033395 243 und kategorisiert Schrottankauf/Buntmetalle. Dies ist ein Branchenverzeichnis-Lead; ein Betreiberauftritt, Registerbeleg oder sichere Auflösung der abweichenden Creditreform-Schreibweise „Matties“ fehlt. **ANKAUF:** nur Verzeichnis-Kategorisierung, kein verifizierter Tarif. **VERKAUF / GEBÜHREN:** keine Liste/Tarife. Status `pruefung`; Adresse nicht ins Frontmatter oder GEO übernehmen.

## Timeline

### Recherche 24.10.2026

- [Recherche 24.10.2026: Gelbe-Seiten-Profil „Matthies Thomas Fuhrbetrieb, Baustoff u. Schrotthandel“ direkt gelesen: Liebenwalder Str. 8, 16348 Wandlitz-Zerpenschleuse, Telefon 033395 243; Rubriken/Leistungsworte umfassen Buntmetalle, Schrotthandel und Schrottankauf. Nur Aggregator-Eintrag, ohne Betreiberseite/Registerbeleg; die Namensabweichung zum bisherigen Creditreform-Lead „Thomas Matties“ bleibt ungelöst. Keine Anschrift/Telefon-Felder ergänzt, kein Aktivitätsstatus geändert. ANKAUF: Verzeichniskategorie, keine bestätigten Konditionen; VERKAUF/GEBÜHREN: keine Tarife oder Preise. Neue bedingte GEO-Kandidatenadresse, aber nicht geocodiert. Quelle: https://www.gelbeseiten.de/gsbiz/9f4b2b57-c9fc-400b-b2e4-ddf6124e3629]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthandel/Baustoff/Fuhrbetrieb

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall — NUR Creditreform-Lead („Thomas Matties -Fuhrbetrieb-“, Schreibweise abweichend), kein Betreiber-Auftritt (keine Website, kein Impressum), kein Register-Treffer, kein Adress-/Telefon-Zweitbeleg; Wandlitzer Branchenverzeichnis führt nur Daiko Recycling (Zehnpfuhlweg 3, Klosterfelde), nicht Matthies; KEIN Fill, Status aktiv NICHT belastbar → auf pruefung zurückgesetzt; Quelle(n): keine belegfähige Quelle (Creditreform-Lead allein + wandlitz.de Negativbeleg Daiko)]

### Recherche 13.11.2026

- [Recherche 13.11.2026: **Die Lead-Adresse existiert in OSM als Hausnummer, der Betrieb dort aber nicht; ein Aggregator liefert erstmals ein Leistungsprofil, und der gespeicherte Punkt ist ein blosser Ortsmittelpunkt.** (1) **OSM live geprueft (13.11.2026):** das Objekt **way/477307903** traegt `addr:housenumber`=**8**, `addr:street`=**Liebenwalder Straße**, `addr:postcode`=**16348**, `addr:city`=**Wandlitz**, `addr:suburb`=**Zerpenschleuse**, `building`=yes - **die Hausnummer der Lead-Adresse ist damit objektgenau belegt**, das Objekt fuehrt aber **weder `name` noch `operator`**, der Betrieb ist in OSM also nicht kartiert. Das Kachelraster **bbox=13.5180,52.8480,13.5320,52.8580** (live, https://api.openstreetmap.org/api/0.6/map.json?bbox=13.5180,52.8480,13.5320,52.8580) liefert **3.025 Elemente**; der einzige Treffer mit Recyclingbezug ist **node/12134955525** (`amenity`=recycling, `recycling_type`=container, `recycling:glass_bottles`=yes) - ein **Glascontainer**, kein Schrottbetrieb. (2) **Verzeichnisstand:** Gelbe Seiten fuehrt weiterhin „Matthies Thomas Fuhrbetrieb, Baustoff u. Schrotthandel", Liebenwalder Str. 8, 16348 Wandlitz-Zerpenschleuse, **033395 243**, mit den Rubriken Buntmetalle, Schrotthandel und Schrottankauf; mapquest beschreibt zusaetzlich ein Leistungsprofil „Handel mit Buntmetallen, Schrott, Altstoffen und Metallabfaellen, dazu Schrottankauf" - beide sind **Aggregatorangaben**, keine Betriebsaussage, und die Namensabweichung zum Creditreform-Lead „Thomas Matties" bleibt ungeklaert. Eine Zusammenfuehrung mit **Daiko Recycling** (Zehnpfuhlweg 3, Klosterfelde) findet nicht statt. (3) **GEO:** die read-only Produktionszeile enthaelt **52.742/13.458**; die Nominatim-Rueckwaertsaufloesung ergibt **„Wandlitz, 16348"** - den **Ortsmittelpunkt der Gemeinde**, also keinen Punkt dieses Betriebs; das Token **`geo` wird in `clear_fields` gesetzt** und zieht den Punkt beim naechsten regulaeren Seed zurueck, **ohne Ersatzkoordinate**, weil die Anschrift nur Aggregator-Beleg ist. Das belegte OSM-Adressobjekt **way/477307903** (Mittelpunkt der eindeutigen Knoten **52.8526707/13.5244393**) bleibt als **GEO-Kandidat** dokumentiert und **nicht** gesetzt, bis ein Betreiber- oder Registerbeleg die Zuordnung der Hausnummer 8 zum Betrieb stuetzt. `street`, `postcode`, `phone`, `email` bleiben leer, `status` bleibt `pruefung`. (4) **Preise getrennt gefuehrt:** **Ankauf** - nur die Verzeichniskategorie „Schrottankauf" bzw. das Aggregator-Leistungsprofil, **keine Kurse, keine Sortenliste, keine Mindestmenge**; **Verkauf** - keine Preisliste; **Gebuehren** - keine Abhol-, Container- oder Baustofftarife, eine kostenlose Abholung wird nicht behauptet. **Offen:** Betreiberidentitaet und Schreibweise (Matthies/Matties), Inhaber, heutige Taetigkeit und jede Preisangabe. Quelle(n): https://www.gelbeseiten.de/gsbiz/9f4b2b57-c9fc-400b-b2e4-ddf6124e3629 ; https://api.openstreetmap.org/api/0.6/way/477307903/full.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=13.5180,52.8480,13.5320,52.8580 ; https://api.openstreetmap.org/api/0.6/node/12134955525.json ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=52.742&lon=13.458&addressdetails=1 ; read-only `public.db` (13.11.2026) (Abruf 13.11.2026).]

### Recherche 22.11.2026 (Welle 0000 C)

- [Recherche 22.11.2026: **Amtliches Fachbetrieberegister: in Wandlitz kein Matthies-Datensatz, Reichweitenklausel.** Die Ortsseite des Registerspiegels (Stand des Registerabrufs 11.08.2026, Abruf 22.11.2026) führt für Wandlitz drei Betriebe (`BSB Fa. Robert Berndt`, Schönerlinder Bahnhofstraße 3, Lagern und Behandeln; `DAIKO Ali Sensecer Nachfolger Jascha Sensecer e.K.`, Zehnpfuhlweg 3, Lagern, Behandeln, Verwerten; `ISR Recycling GmbH & Co. KG Standort Wandlitz`) und vermerkt, dass zwei davon als Demontagebetrieb anerkannt sind; ein Eintrag zu `Matthies` fehlt, ebenso die Anschrift `Liebenwalder Straße 8`. Reichweitenklausel: die Anerkennung nach § 56 KrWG und die Anerkennung nach AltfahrzeugV sind freiwillig, ein Nichteintrag ist weder ein Existenzbeweis noch ein Beleg für eine Betriebsaufgabe. Der Abruf des amtlichen Portals (GADSYS/GESA) auf die Suchroute `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Matthies` liefert am 22.11.2026 wieder nur die Vaadin-JavaScript-Anwendung mit 1.189 Byte HTML, also einen Abrufbefund und keinen Negativbefund. Quelle: https://schrottregister.pages.dev/ort-wandlitz .]
- [Recherche 22.11.2026: **Register-Namenssuche ohne Treffer, ohne Beweiswert.** Die Suche `Matthies Wandlitz` im amtlichen Registerauszugsportal liefert am 22.11.2026 für Unternehmen wie für Personen `Keine Suchergebnisse!`; die abweichende Schreibweise `Matties` bleibt damit ungeklärt und ein Registerbeleg fehlt weiterhin, was ein Einzelgewerbe nicht widerlegt. Quelle: https://www.online-handelsregister.de/suche/Matthies+Wandlitz (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **Die kommunale Gewerbestelle der Gemeinde Wandlitz ist jetzt benannt — das nichtöffentliche Gewerberegister bleibt der einzige belastbare Weg.** Das Gewerbeamt der Gemeinde sitzt im Rathaus, Prenzlauer Chaussee 157, 16348 Wandlitz, zuständige Ansprechpartnerin Frau J. Schilling, Telefon 033397 360-440, `gewerbeamt(at)wandlitz.de`; die Gemeinde beschreibt selbst, dass das Gewerbeamt `das nichtöffentliche Gewerberegister` führt und Anmeldungen nach § 14 GewO entgegennimmt. Eine Anfrage ist nicht gestellt. Die Websuche der Gemeinde (`/suche/?tx_solr[q]=Matthies`) liefert am 22.11.2026 kein Ergebnis, und das Verzeichnis `Unternehmen, Selbständige, Freiberufler vor Ort` der Gemeinde (Selbsteintrag, keine amtliche Liste) enthält in der abgerufenen Rubrik keinen Eintrag `Matthies`; beides sind begrenzte Suchbefunde ohne Beweiswert. Quellen: https://www.wandlitz.de/dienstleistung/anzeigen/id/1396/gewerbe.html ; https://www.wandlitz.de/suche/?tx_solr%5Bq%5D=Matthies ; https://www.wandlitz.de/verzeichnis/index.php?kategorie=1377 (Abruf 22.11.2026).]
- [Recherche 22.11.2026: **GEO: das Adressobjekt ist unverändert belegt, der zurückgezogene Punkt ist kein Ortsmittelpunkt, sondern ein Straßenknoten 13,09 km entfernt.** Das OSM-Objekt way/477307903 trägt weiterhin `addr:housenumber`=8, `addr:street`=Liebenwalder Straße, `addr:postcode`=16348, `addr:city`=Wandlitz, `addr:suburb`=Zerpenschleuse, `building`=yes, aber weder `name` noch `operator` — die Hausnummer der Lead-Adresse ist objektgenau belegt, der Betrieb ist nicht kartiert (Mittelwert 52.8526855/13.5244346). Der am 13.11.2026 zurückgenommene Produktionspunkt 52.742/13.458 löst sich live auf way/1032613961 auf, die B 273 im Ortsteil Wandlitz (`junction`=roundabout, `ref`=B 273) — er trifft also einen Straßenknoten in der Gemeinde, nicht einen Ort im Sinne eines zentralen Punktes der Anschrift, und liegt 13,09 km vom Adressobjekt entfernt (Haversine). Die Rücknahme per `clear_fields` bleibt damit richtig, ein Ersatzpunkt wird nicht gesetzt; über Setzen oder Entfernen entscheidet der Owner in einer eigenen datierten Korrektur-Sektion. Quellen: https://api.openstreetmap.org/api/0.6/way/477307903/full ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.742&lon=13.458&zoom=18&addressdetails=1 ; read-only `public.db` (22.11.2026).]
- [Recherche 22.11.2026: **Preise und offene Fragen.** ANKAUF: keine bestätigten Konditionen, die Rubriken Schrottankauf und Buntmetalle bleiben Aggregatorangaben; VERKAUF: keine Verkaufspreisliste; GEBÜHREN: keine Tarife. Offen bleiben (1) eine Gewerbeauskunft der Gemeinde Wandlitz zur Person und zum Gewerbestand an `Liebenwalder Straße 8`, (2) die Klärung der Schreibweise `Matthies` gegenüber `Matties` und (3) eine Bestätigung, ob der Betrieb an dieser Adresse überhaupt noch annimmt. Quellen: siehe oben.]
