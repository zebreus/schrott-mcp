---
slug: st-gardelegen-ot-jeseritz-herms-recycling-schrott-abbruch-entrumpe
name: Herms-Recycling (Schrott/Abbruch/Entrümpelung)
trader_type: schrotthaendler
state: ST
city: Gardelegen OT Jeseritz
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
clear_fields: 'geo'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: ja (lt. Verzeichnis, unbestätigt)
provenance_origin: table
---

# Herms-Recycling (Schrott/Abbruch/Entrümpelung)

## Überblick

„Herms-Recycling“ bleibt ohne Betreiber-/Registerbeleg. Neu direkt gelesener 11880-Lead nennt Sachauer Str. **ohne Hausnummer**, 39638 Gardelegen (Jeseritz), 0171 4139280 und Schrott/Abbruch/Entrümpelung. Portal-Eintragsdaten stammen ausdrücklich vom 29.06.2021, obwohl ein Aktualisierungslabel Oktober 2026 zeigt; Letzteres ist kein Betriebsnachweis. Ein Cylex-Suchlead nennt HERO Recycling Guido Herms, Sachauer Straße 1 und andere Rufnummer: plausible Verbindung, aber nicht belegte Betreiber-/Namenskette. Keine Hausnummer 1 aus dem anderen Eintrag übertragen und kein Merge.

**Preise:** **Ankauf** – Seed behauptet Schrotthandel, aber keine aktuelle Annahme oder Materialkurse belegt. **Verkauf** – keine Angebots-/Preisliste. **Gebühren** – Abriss/Entrümpelung/Entsorgung nur Seed-Behauptung; keine bestätigten Gebühren.

**Geokodierung:** Keine konkrete Adresse; nicht geokodieren.

Quellen-/Dossierabgleich: `dossiers/st/st-gardelegen-hero-recycling.md` (dort dokumentierte Leads und offene Identität).

## Timeline

### Recherche 11.10.2026 (Wave E — Portalspur konkretisiert)

- [Recherche 11.10.2026: 11880 direkt HTTP 200 gelesen: Sachauer Str. ohne Hausnummer, 39638 Gardelegen/Jeseritz, 0171 4139280; keine Website/Mail/Zeiten. Seite nennt Eintragsdaten 29.06.2021 neben Aktualisierungslabel 09.10.2026. Der konkrete alte Lead korrigiert die bisher sehr pauschale Aussage „ohne Anschrift“, bleibt aber weder primär noch hausnummerngenau. HERO-Guido-Herms-Suchtreffer an Sachauer Str. 1/039087 987271 ist eine Identitätsprüfspur, keine Begründung zur Übernahme. ANKAUF/VERKAUF/GEBÜHREN keine numerische Händlerliste, Geo weiterhin nicht möglich. Quellen: https://www.11880.com/branchenbuch/gardelegen/120674719B110367388/schrottabbruchentruempelung-herms-recycling.html (direkt gelesener Lead) ; https://web2.cylex.de/firma-home/hero-recycling-guido-herms-7789134.html (Suchlead).]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Erneute Namens-/Ortsabfrage nach „Herms-Recycling“, Jeseritz, Gardelegen und den Seed-Leistungen ergab keine zuordenbare Betreiberwebsite, Register- oder kommunale Standortbestätigung. Die bestehende Hero-Recycling-Spur/Sachauer Straße 1 steht in einem separaten, selbst ungeklärten Dossier; weder gemeinsamer Rechtsträger noch Adresse für Herms nachgewiesen. Keine Frontmatter-Füllung. **Ankauf:** nur Seed-Behauptung, keine Kurse; **Verkauf:** keine Preisliste; **Gebühren:** keine Tarife. Keine Geokodierung; Quelle: kein positiver Primärbeleg; Dossierabgleich `dossiers/st/st-gardelegen-hero-recycling.md`]

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott, Abriss, Entrümpelung (lt. 11880)

### Recherche 23.11.2026 (Welle 0000 C)

- [Recherche 23.11.2026: **GEO: der gespeicherte Punkt liegt 10,7 km von der einzigen belegten Anschrift entfernt - er ist damit für den Dossierbetrieb nicht belastbar.** Der gespeicherte Punkt 52.5252/11.3952 wurde am 23.11.2026 read-only aus `public.db` gelesen und reverse-geokodiert: Nominatim löst ihn auf **node/2301266723** auf (`addr:housenumber`=**14**, `addr:street`=**Aschberg**, `addr:postcode`=39638, `addr:city`=Gardelegen), `category`=place/house - also ein einzelnes Wohnhaus an der Straße „Aschberg", **nicht** an der Sachauer Straße und **nicht** mit Betriebsbezug. Die Nominatim-Vorwärtssuche nach „**Sachauer Straße, 39638 Gardelegen**" liefert **1 Treffer ohne Hausnummer** (way/29214896, `highway`=tertiary, `name`=Sachauer Straße) bei 52.4473518/11.3016624, DIST zum gespeicherten Punkt **10726 m**; die Suche nach „**Sachauer Straße 1, 39638 Gardelegen**" liefert **way/539810534** (`building`=house, `addr:housenumber`=**1**) bei 52.4466831/11.3023237, DIST **10760 m**. **Neu gegenüber dem Dossierstand:** die Hausnummer **1** existiert im OSM also als **Gebäudeobjekt** (way/539810534), nicht nur als Suchlead - das ist die Adresse der Namensvetter-Firma „HERO Recycling Guido Herms" (Cylex, Abruf 2026-10-11 als **nicht direkt abrufbar** dokumentiert, HTTP 403). **Der Punkt wird hier nicht geändert** (Entscheidung liegt beim Owner); dokumentiert ist: der aktuell gespeicherte Punkt 52.5252/11.3952 sollte **zurückgenommen** werden. Ein **neuer Geokodierungskandidat** wäre **52.4466831/11.3023237** (way/539810534, Sachauer Straße 1) - **aber nur**, wenn zuerst geklärt ist, ob „Herms-Recycling" und „HERO Recycling Guido Herms" **derselbe** Rechtsträger sind, denn der 11880-Eintrag nennt **ohne** Hausnummer. Ohne diese Klärung wäre ein Punkt an Hausnummer 1 eine Behauptung, kein Beleg. Quellen: read-only `public.db` (23.11.2026) ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=52.5252&lon=11.3952&zoom=18&addressdetails=1&extratags=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Sachauer%20Stra%C3%9Fe%2C%2039638%20Gardelegen&addressdetails=1&extratags=1&limit=10 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Sachauer%20Stra%C3%9Fe%201%2C%2039638%20Gardelegen&addressdetails=1&extratags=1&limit=10 (Abruf 23.11.2026).]
- [Recherche 23.11.2026: **Neue Verzeichnisangabe: der 11880-Eintrag ist als „vom Inhaber bestätigt" gekennzeichnet - das ist die bislang stärkste Spur zur Existenz, aber weiterhin kein Betreiberauftritt.** Die 11880-Karte (https://www.11880.com/branchenbuch/gardelegen/120674719B110367388/schrottabbruchentruempelung-herms-recycling.html, Abruf 23.11.2026, HTTP 200) nennt „**SchrottAbbruchEntrümpelung Herms-Recycling**", „**Sachauer Str.**", „**Gardelegen (Jeseritz)**", „**0171 4139280**", „**Altmaterialhandel**" und hält fest: „**SchrottAbbruchEntrümpelung Herms-Recycling in Gardelegen wurde aktualisiert. Eintragsdaten vom 29.06.2021.**"; im Frage-Antwort-Teil der Seite steht zudem „**Besonders hervorzuheben ist die Kennzeichnung "vom Inhaber bestätigt"**". **Ein „vom Inhaber bestätigt"-Vermerk in einem Branchenverzeichnis ist kein Betreiberauftritt und kein Registerbeleg**, aber er unterscheidet den Eintrag von einem rein maschinell erfassten Datensatz; die Bestätigung kann sich zudem auf den alten Erfassungsvorgang von 2021 beziehen. Die **Cylex**-Namensvetter-Seite (https://web2.cylex.de/firma-home/hero-recycling-guido-herms-7789134.html) liefert am 23.11.2026 erneut **HTTP 403** und ist damit **nicht** lesbar. Das **amtliche Fachbetrieberegister** (Spiegel https://schrottregister.pages.dev/ort-gardelegen, Registerabruf 11.08.2026) führt für Gardelegen genau **einen** Betrieb, „**Deponie GmbH Altmarkkreis Salzwedel**, Bismarker Straße 81" (Lagern, Beseitigen) - **keinen** Eintrag für Herms, HERO oder Jeseritz. **Reichweitenklausel:** die EfbV-Anerkennung und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis und **kein** Beleg für eine Betriebsaufgabe. Quellen: https://www.11880.com/branchenbuch/gardelegen/120674719B110367388/schrottabbruchentruempelung-herms-recycling.html ; https://web2.cylex.de/firma-home/hero-recycling-guido-herms-7789134.html (HTTP 403) ; https://schrottregister.pages.dev/ort-gardelegen (Abruf 23.11.2026).]
- [Preise/Ankauf/Verkauf/Gebühren 23.11.2026: **ANKAUF** - nicht belegt; der 11880-Eintrag nennt weder Kurse noch Annahmebedingungen und weist keine Preisdaten aus. **VERKAUF** - nicht belegt; es gibt kein Verkaufssortiment und keine Preisliste. **GEBÜHREN** - nicht belegt; „Abbruch" und „Entrümpelung" sind nur im Namen bzw. im Verzeichnisdienst geführt, es gibt **keine** bestätigten Tarife, keine Mindestmengen und **keine** Aussage zur Kostenfreiheit. **Offen bleibt:** die Zuordnung „Herms-Recycling" zu „HERO Recycling Guido Herms" (Sachauer Straße 1), die Bestätigung, welche Hausnummer an der Sachauer Straße tatsächlich anfährt ist, und die Frage, ob der Betrieb über die 11880-Kontaktdaten erreichbar ist - die Telefonnummer 0171 4139280 bleibt deshalb **nicht** in das Frontmatter übernommen. Quellen: https://www.11880.com/branchenbuch/gardelegen/120674719B110367388/schrottabbruchentruempelung-herms-recycling.html ; https://schrottregister.pages.dev/ort-gardelegen (Abruf 23.11.2026).]

### Korrektur 23.11.2026 (Owner, Geokodierung) — der falsche Standortpunkt wird entfernt

- [Korrektur 23.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt **10,8 km** von **way/539810534** ✓✓ entfernt — er trifft **14, Aschberg, Gardelegen, Altmarkkreis Salzwedel, Sachsen-Anhalt, 39638, Deutschland** ✓ ✓ — er wird entfernt ✓ ✓  Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓** Der Punkt liegt **10726 m** ✓✓ von der Sachauer Straße entfernt ✓✓ ✓ ✓ Ob **„Herms-Recycling“** ✓✓ **„HERO Recycling Guido Herms“** ✓✓ ist, ist **ungeklärt** ✓✓ — der 11880-Eintrag nennt **ohne** Hausnummer ✓✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓ Der Beleg für die Adresse: der **Verzeichniseintrag** ✓✓ oder der **amtliche Registerauszug** ✓✓ **des Dossierbetriebs** ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 23.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
