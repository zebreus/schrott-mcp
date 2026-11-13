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
