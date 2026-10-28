---
slug: ni-hameln-neermann-metallverwertung
name: Neermann Metallverwertung
trader_type: sonstige
state: NI
city: Hameln
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: 'Offener Namesake-/Merge-Fall: Die Seed-Zeile „Neermann Metallverwertung“ in Hameln ist nur durch einen Aggregator-Lead (Schrotthandel/Metallverwertung Neermann, Hohes Feld, 31787 Hameln, Mobilnummer, Abholhinweis) gestützt. Das amtliche Fachbetrieberegister kennt für Hameln (31785/31789) sieben anerkannte Betriebe, aber keinen „Neermann“; in OpenStreetMap ist im Umfeld des Lead-Standorts kein Schrott-/Metallbetrieb kartiert. Die Betreiberseite neermann-dienstleistungen.de betrifft Neermann Dienstleistungen in Uetersen und stellt keinen Bezug zum Hamelner Schrotthandel her. Ankauf, Verkauf und Gebühren sind nicht belegt; Straße, Kontakt und Öffnungszeiten bleiben leer, status pruefung.'
dropoff_json: ''
pickup_json: ''
provenance_section: Göttingen / Northeim / Goslar-Harz / Hildesheim / Weserbergland
provenance_ankauf_raw: ja
provenance_origin: table
---

# Neermann Metallverwertung

## Überblick

Der Hamelner Eintrag „Neermann Metallverwertung“ ist weiterhin **nur ein Verzeichnis-Lead**: Gelbe Seiten/11880 führen „Schrotthandel / Metallverwertung Neermann“ mit **Hohes Feld, 31787 Hameln**, einer Mobilnummer und einem Abholhinweis. In dieser Welle ist der Lead **nicht mehr reproduzierbar**: die 11880-Detailseite `…/hameln/060440092B107338668/schrotthandel-neermann.html` liefert am 28.10.2026 nur noch die Rubrikseite „Altmetallrecyclings Hameln“ (kein Treffer „Neermann“ im HTML). Das amtliche Fachbetrieberegister kennt für Hameln sieben anerkannte Betriebe (Enertec, Meyer Entsorgungs GmbH, PreZero Service Mitte, REYENND, Werner Otto, Weserbergland Entsorgung, myContainerdienst) — **keinen „Neermann“**; in OpenStreetMap ist im Umfeld des Lead-Standorts kein Schrott- oder Metallbetrieb kartiert. Die Betreiberseite `neermann-dienstleistungen.de` betrifft Neermann Dienstleistungen in **Uetersen** und stellt keinen Bezug her.

**Ankauf:** keine Kurse, kein Sortiment, keine Annahmebedingungen belegt. **Verkauf:** keine Preise belegt. **Gebühren:** keine Tarife belegt.

**Status:** Der Seed-Wert `aktiv` war nie belegt; die Recherche vom 04.10.2026 hat ihn ausdrücklich nur „als vorhandener Seed-Wert stehen“ lassen. Nach den Negativbefunden dieser Welle (nicht reproduzierbarer Verzeichniseintrag, kein Registereintrag, kein OSM-Objekt, Namensvetter in Uetersen) wird `status` auf **`pruefung`** korrigiert: Der **Ankaufsstatus** ist ungeklärt. **Keine Schließungsbehauptung** — ein Negativbefund beweist keine Nichtexistenz.

## Timeline

- [Owner-Korrektur 28.10.2026 (Korrekturvermerk): **Eine Formulierung war ungenau und ist präzisiert.** die Formulierung „OSM kennt dort derzeit keine Hausnummer des Lead-Standorts“ war ungenau: Die Straße Hohes Feld ist in OSM erfasst (Nominatim-Reverse auf „Hohes Feld, 31787 Hameln“), eine Hausnummer des Lead-Standorts jedoch nicht. Die Aussage „kein Geokodierpunkt“ bleibt richtig. Quellen: Nominatim-Reverse auf die genannte Adresse, live abgerufen am 28.10.2026.]

### Recherche 28.10.2026 (Shard C, 6h-Welle): Lead nicht mehr reproduzierbar, Status `aktiv` → `pruefung`

- [Recherche 28.10.2026: **Der Verzeichnis-Lead ist nicht mehr abrufbar.** Die 11880-Detailseite „Schrotthandel Neermann, Hameln“ (https://www.11880.com/branchenbuch/hameln/060440092B107338668/schrotthandel-neermann.html) liefert am 28.10.2026 HTTP 200, aber nur die Sammelseite „Altmetallrecyclings Hameln“ (End-URL `https://www.11880.com/suche/altmetallrecycling/hameln`); im HTML kommt der Name „Neermann“ **nicht** mehr vor. Die Detailseite des Verzeichniseintrags ist damit entfallen bzw. in die Rubrikseite umgeleitet. Das ist ein **starkeres** Negativsignal als die bisherigen Abgrenzungshinweise, aber weiterhin **kein** Nichtexistenzbeweis. Quelle(n): https://www.11880.com/branchenbuch/hameln/060440092B107338668/schrotthandel-neermann.html (Abruf 28.10.2026).]
- [Recherche 28.10.2026: Amtliches Fachbetrieberegister (Auszug 11.08.2026, Spiegel SchrottRegister) führt für **Hameln (31785, 31789)** sieben anerkannte Betriebe: Enertec Hameln GmbH (Heinrich-Schoormann-Weg 1, Verwerten/Beseitigen), Meyer Entsorgungs GmbH (Wallbaumstraße 20), PreZero Service Mitte GmbH & Co. KG (Dieselstraße 7), REYENND GmbH (Marienthaler Straße 8), Werner Otto GmbH (Düth 40), Weserbergland Entsorgung GmbH (Marienthaler Straße 6), myContainerdienst GmbH (Marienthaler Straße 8) — **keinen „Neermann“** und keinen Betrieb in der Straße „Hohes Feld“. Negativbefund ohne Beweiswert (Efb-Anerkennung freiwillig). Quellen: https://schrottregister.pages.dev/ort-hameln (Abruf 28.10.2026).]
- [Recherche 28.10.2026: OpenStreetMap-Abgleich im Umfeld des Lead-Standorts **Hohes Feld, 31787 Hameln** (`api.openstreetmap.org/api/0.6/map`): **kein** Kartier-Objekt zu „Neermann“, Schrott, Metallrecycling, Wertstoffhof oder `industrial=scrap_yard`; benannt sind dort ausschließlich Läden, Apotheken, gastronomische Betriebe und der KreisAbfallWirtschaft (Ohsener Straße 98) sowie eine Schlosserei (Schlosserei Uhlit GmbH & Co. KG, `craft=metal_construction`) — **kein** Schrotthandel. Quelle: https://api.openstreetmap.org/api/0.6/map?bbox=9.3600,52.0900,9.4000,52.1200 (Abruf 28.10.2026).]
- [GEO-Abgleich 28.10.2026: Der in `public.db` gespeicherte Punkt **52.1058333/9.3671333** löst per Nominatim auf den OSM-Hausnummernknoten **node 5708110617** (live geprüft über api.openstreetmap.org: `addr:street=Sandstraße`, `addr:housenumber=8A`, `addr:city=Hameln`, angelegt 2018) auf — also **nicht** auf „Hohes Feld, 31787 Hameln“. Der Bestandspunkt ist damit ein Alt-Seed-Punkt ohne Dossierbeleg; das Dossier führt keine Koordinaten, deshalb **kein** `clear_fields: geo` (Owner-Entscheidung) und keine neue Geokodierung. Quellen: https://api.openstreetmap.org/api/0.6/node/5708110617.json ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=52.1058333&lon=9.3671333&addressdetails=1 ; read-only `public.db` (28.10.2026).]
- [Status-Korrektur 28.10.2026: `status` von `aktiv` auf **`pruefung`** korrigiert. Begründung: `aktiv` war ein nie belegter Seed-Wert; nach drei unabhängigen Negativprüfungen (nicht reproduzierbarer Verzeichniseintrag, kein Registereintrag, kein OSM-Objekt) und dem weiterhin offenen Namensvetter (Neermann Dienstleistungen, Uetersen) ist der **Ankaufsstatus** ungeklärt — genau der Fall für `pruefung`. **Ausdrücklich keine Schließungsbehauptung** und keine Nichtexistenzbehauptung; das Frontmatter-Feld `provenance_ankauf_raw: ja` bleibt als Seed-Herkunft unangetastet. Quellen: siehe oben.]
- [Preise/Ankauf/Verkauf/Gebühren 28.10.2026: **ANKAUF** — kein Ankaufspreis, kein Materialsortiment, keine Mindestmenge, keine Annahmebedingung belegt; der Lead nennt lediglich einen „Abholhinweis“. **VERKAUF** — kein Absatzsortiment, keine Verkaufspreise belegt. **GEBÜHREN** — keine Entsorgungs-, Abhol- oder Annahmegebühren belegt. Quellen: siehe oben.]
- [Offene Punkte 28.10.2026: (1) Gewerbeauskunft der Stadt Hameln (Hohes Feld, 31787) zu Name, Gewerbestand und Tätigkeit; der Lead ist aus den Verzeichnissen verschwunden, der Seed-Betrieb muss also anders bestätigt werden. (2) Verträglichkeit mit „Neermann Dienstleistungen“ (Uetersen) ist zu prüfen (Namensvetter). (3) Sollte die Adresse bestätigt werden, wäre **Hohes Feld, 31787 Hameln** neu zu geokodieren — OSM kennt dort die Straße Hohes Feld, aber keine Hausnummer des Lead-Standorts.]

### Importiert (Seed-Stand 2026-09-30)

- Metallverwertung

### Recherche 04.10.2026

- [Recherche 04.10.2026: Offener Namesake-/Merge-Fall: Gelbe Seiten listet „Schrotthandel / Metallverwertung Neermann“ in Hohes Feld, 31787 Hameln, mit Mobilnummer und Abholhinweis; dies bleibt ein nicht verifizierter Aggregator-Lead. Die Betreiberseite neermann-dienstleistungen.de weist dagegen Neermann Dienstleistungen in Uetersen aus und stellt keinen Bezug zum Hamelner Schrotthandel her. Anschrift/Kontakte/Leistungen werden daher nicht übertragen; status aktiv bleibt als vorhandener Seed-Wert stehen, nicht als neu verifizierte Aussage. Keine Schließungsbehauptung; Quelle(n): https://www.gelbeseiten.de/gsbiz/08436375-5ad6-477e-8229-4ac419c28e9f ; https://neermann-dienstleistungen.de ; https://www.11880.com/branchenbuch/hameln/060440092B107338668/schrotthandel-neermann.html]
