---
slug: hh-kirchwerder-adzovic-safet-schrotthandel
name: Adzovic Safet Schrotthandel
trader_type: schrotthaendler
state: HH
city: Kirchwerder
street: 'Warwischer Hauptdeich 90'
postcode: '21037'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Adzovic Safet Schrotthandel

## Überblick

Nur ein Einzel-Lead bei schrottplatz-info.de nennt Adzovic Safet Schrotthandel am Warwischer Hauptdeich 90, 21037; Betreiberwebsite, Registerbeleg und zweiter unabhängiger Nachweis fehlen. Ein hamburg.de-Eintrag zu „Adzovic“ nennt dagegen Von-Scheliha-Str. 34, 21035, Haushaltsauflösungen und Tel. 0176 65353877 – andere Adresse und Branche, daher keine sichere Personenzuordnung und kein Merge. Frontmatter-Kontakte bleiben leer; `status: pruefung`.

**Preise (Ankauf/Verkauf/Gebühren):** keine verifizierbaren Preis- oder Gebührenangaben. **Offen:** Betreiberidentität, tatsächlicher Standort, Betriebsform (Schrotthandel vs. Haushaltsauflösung) und Annahmebedingungen klären; keine Schließung aus fehlenden Quellen ableiten. Quellen siehe Recherche 02.10.2026.

**Recherche 29.10.2026:** Die Anschrift ist durch zwei unabhängige Verzeichnisbetreiber und durch das Adress-Objekt in OpenStreetMap gedeckt (siehe Timeline); ein Betreiber-, Impressum- oder Registerbeleg fehlt weiterhin. In Hamburg bestehen mindestens drei verschiedene „Adzovic“-Betriebe — eine Zusammenführung ist nicht belegbar. **Ankauf:** keine Kurse belegt. **Verkauf:** keine Liste. **Gebühren:** keine Tarife.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Schrott (Kleinhändler Vierlande)
- Adresse: Kirchwerder, Warwischer Hauptdeich 90, 21037

### Recherche 02.10.2026

- [Recherche 02.10.2026: miss / KLAERFALL - einziger Beleg schrottplatz-info.de (Warwischer Hauptdeich 90, 21037, ohne Tel./Website); keine Betreiber-Website, kein Registerbeleg, kein Zweitbeleg. hamburg.de-Adzovic-Eintrag (Von-Scheliha-Str. 34, 21035, Haushaltsaufloesungen, 0176 65353877) weicht in Adresse und Branche ab - Identitaet/Personenzoordnung ungeklärt, moeglichst Kleinst-/Kleinhaendler ohne oeffentlichen Auftritt; kein Fill; Quelle(n): schrottplatz-info.de (Einzel-Lead), hamburg.de (Abweichungs-Lead)]

### Recherche 29.10.2026

- [Recherche 29.10.2026: Anschrift jetzt durch zwei unabhaengige Verzeichnisbetreiber plus Adress-Objekt in OpenStreetMap gedeckt. (1) schrottplatz.org fuehrt live abgerufen den Eintrag „Schrott Adzovic Safet Schrotthandel“, Warwischer Hauptdeich 90, 21037 Hamburg, Tel. (0172) 5 46 13 41, Kategorie Schrottplatz, 0 Bewertungen, Dienstleistungen „Schrotthandel“; die Telefonnummer ist damit erstmals belegt, aber nur EIN Verzeichnisbeleg und daher nicht ins Frontmatter uebernommen. (2) OSM-Praesenzpruefung ueber api.openstreetmap.org (Abruf 29.10.2026): die Strasse „Warwischer Hauptdeich“ (Kirchwerder/Bergedorf, 21037) traegt 15 Objekte mit addr:street; das sind 15 verschiedene Hausnummern (72, 74, 76, 78, 80, 80a, 84, 88, 90, 92, 94, 104, 106, 114, 116) — die Hausnummer 90 EXISTIERT als Gebaeude way/257411602 (addr:city=Hamburg, addr:postcode=21037, building=yes) ohne Namens-, Betreiber- oder Branchen-Tag; street/postcode deshalb ins Frontmatter ergaenzt, phone bleibt leer, status pruefung bleibt (kein Betreiber-/Registerbeleg). (3) Namesaken-Abgrenzung: ein dritter Adzovic-Betrieb ist live belegt — das Impressum von entsorgungen24.de nennt „David Adzovic, Mozartstrasse 6, 22083 Hamburg“, Entruempelung, Tel. +49 176 2277 2608; zusammen mit dem hamburg.de-Lead (Von-Scheliha-Str. 34, 21035, Haushaltsaufloesungen, 0176 65353877) sind es drei verschiedene Hamburger „Adzovic“-Betriebe mit unterschiedlicher Branche und Telefonnummer — keine Personenzuordnung und kein Merge. ANKAUF: keine Kurse, keine Annahmebedingungen; VERKAUF: keine Liste; GEBUEHREN: keine Tarife. Quelle(n): https://www.schrottplatz.org/hamburg/schrott-adzovic-safet-schrotthandel-aYEPVH.html ; https://entsorgungen24.de/impressum/ ; OpenStreetMap via api.openstreetmap.org (way/257411602, Stand 29.10.2026)]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Keine Koordinate gesetzt — Klärfall dokumentiert.** Das OSM-Adressobjekt existiert und
  wurde live nachgeladen: **way/257411602** (live geladen 03.11.2026,
  https://api.openstreetmap.org/api/0.6/way/257411602/full.json) trägt `addr:housenumber`=**90**,
  `addr:street`=**Warwischer Hauptdeich**, `addr:city`=Hamburg, `addr:postcode`=**21037**, `building`=yes; das Objekt
  hat **8 eindeutige Knoten**, der Mittelwert der Knoten (53.4255317/10.1222117) liegt innerhalb der Objekt-BBox
  (53.4254810/10.1221255 bis 53.4256361/10.1223382). Die Dossier-Adresse und -Postleitzahl sind damit objektgenau
  belegt. **Warum trotzdem keine Koordinate:** Für die Adresse **Warwischer Hauptdeich 90** existiert im Dossier
  **keine** Betreiber- oder Registerquelle — die beiden Verzeichnisbetreiber (schrottplatz.org, schrottplatz-info.de)
  sind nach der Repo-Quellenhierarchie **Leads, kein Beleg**, und der OSM-Knoten selbst trägt **weder `name=` noch
  `operator=`** (kein Betriebsbezug, der das Gebäude dem Dossier-Händler zuordnet). Die im Dossier genannten
  Adzovic-Belege betreffen **andere** Betriebe an anderen Anschriften (hamburg.de: „Adzovic", Von-Scheliha-Str. 34,
  21035, Haushaltsauflösungen; Impressum entsorgungen24.de: „David Adzovic, Mozartstrasse 6, 22083 Hamburg",
  Entrümpelung) — eine Personenzuordnung oder ein Merge ist damit **nicht** belegbar. Nach der Zwei-Quellen-Regel
  (OSM-Objekt **und** eine im Dossier zitierte Betreiber-/Registerquelle) ist die zweite Quelle für diese Adresse nicht
  erfüllt; ein Punkt wäre hier eine Behauptung, keine Geokodierung. **Negativaussage, qualifiziert:** Der
  Adress-Objekt-Nachweis belegt **das Gebäude der Hausnummer 90**, nicht die Betreiberidentität „Adzovic Safet
  Schrotthandel"; `street`/`postcode` bleiben unverändert im Frontmatter, `phone` bleibt leer, `status: pruefung`
  bleibt. **Nächster Schritt für den Owner:** ein aktueller Gewerbe-/Betreibernachweis (Impressum mit Rechtsträger oder
  ein Registerauszug) für den Standort Warwischer Hauptdeich 90 — dann kann way/257411602 ohne Neuladen als Koordinate
  übernommen werden. Quelle(n): https://api.openstreetmap.org/api/0.6/way/257411602/full.json ;
  https://www.schrottplatz.org/hamburg/schrott-adzovic-safet-schrotthandel-aYEPVH.html (live abgerufen 03.11.2026,
  Verzeichnislead); Bestandsvermerke vom 29.10.2026 und 02.10.2026 dieses Dossiers.]
