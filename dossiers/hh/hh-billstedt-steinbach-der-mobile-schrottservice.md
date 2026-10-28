---
slug: hh-billstedt-steinbach-der-mobile-schrottservice
name: Steinbach der mobile Schrottservice
trader_type: mobil
state: HH
city: Billstedt
street: Brockhausweg 93
postcode: '22117'
phone: ''
email: ''
opening_hours: ''
website: https://mobiler-schrottservice.de
website_status: aktiv
status: pruefung
description: 'Der Import beschreibt einen **mobilen Abholservice**, keinen belegten Schrottplatz. Die Betreiber-Website `mobiler-schrottservice.de` wirbt mit Ankauf/Abholung von Aluminium, Blei, Kupfer, Messing, VA-Edelstahl, Kabeln und Autobatterien sowie Demontage und Entsorgung ("Entsorgungen", "Transporter"). Ihr Impressum nennt **Brockhausweg 57, 22127 Hamburg**, die Telefonnummern **01788028550** und **015775230613** und die E-Mail **donatosteinbach@icloud.com**; das Impressum benennt als Verantwortlichen nur die Domain, keine Firma oder vollständige natürliche Person. Der Seed führt dagegen **Brockhausweg 93, 22117 Hamburg** - das ist derselbe Straßenblock, aber eine **andere Hausnummer**, rund **264 m** entfernt; OSM kennt in der Brockhausweg die Nummern 57 und 93 (eine Overpass-Abfrage des gesamten Straßenzugs am 28.10.2026 ergab 126 Hausnummern, **keine 141er** - die Nummern 141a-c liegen in der **Möllner Landstraße**, nicht im Brockhausweg); eine eigene Hausnummer „141K“ gibt es im Brockhausweg ebenfalls nicht. Es besteht daher weiterhin **keine belegte Verbindung** zwischen diesem Seed-Eintrag und der Betreiber-Website, die im Repo bereits als eigene Zeile `hh-billstedt-mobiler-schrottservice-mobiler-schrottha` geführt wird. ANKAUF: keine Zahlenpreise und keine Mindestmengen veröffentlicht. VERKAUF: keine Liste. GEBÜHREN: keine Tarife; ob eine Abholung kostenlos, vergütet oder kostenpflichtig ist, bleibt offen. Keine Empfehlung zur unangekündigten Anlieferung.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Steinbach der mobile Schrottservice

## Überblick

Der Import beschreibt einen **mobilen Abholservice**, nicht einen belegten Schrottplatz. Die
Betreiber-Website `mobiler-schrottservice.de` wirbt mit Ankauf/Abholung von Aluminium, Blei,
Kupfer, Messing, VA-Edelstahl, Kabeln und Autobatterien sowie Demontage und Entsorgung. Ihr
**Impressum** (am 28.10.2026 direkt gelesen) nennt **Brockhausweg 57, 22127 Hamburg**, die
Telefonnummern **01788028550** und **015775230613** und die E-Mail **donatosteinbach@icloud.com**;
als Verantwortlichen nennt es nur die Domain, **keine Firma und keine vollständige natürliche
Person**. Die Leistungsseite nennt "Ankauf: Aluminium, Blei, Kupfer, Messing, VA Edelstahl,
Kabel, Autobatterien", "Entsorgungen" und "Transporter", mit dem Wirkungsgebiet "Hamburg und
Umgebung".

**Adresskonflikt bleibt offen und ist jetzt vermessen:** Der Seed führt **Brockhausweg 93,
22117 Hamburg**. Das Impressum nennt **Brockhausweg 57** und die PLZ **22127**. Beide liegen im
selben Straßenblock; OSM/Nominatim führt für den Brockhausweg die Hausnummern **57**, **93** und
**141a/141b/141c** in **22117** - die PLZ **22127** aus dem Impressum findet sich in OSM für diese
Straße nicht. Die beiden Hausnummern liegen **264 m** auseinander. Das ist **keine** belegte
Identität zwischen diesem Seed-Eintrag und der Betreiber-Website.

**Abgrenzung:** Die Betreiber-Website wird im Repo bereits als eigene, vollständig geführte Zeile
`hh-billstedt-mobiler-schrottservice-mobiler-schrottha` geführt (Brockhausweg 57, 22127, beide
Telefonnummern, E-Mail, `trader_type: mobil`, `status: aktiv`). Diese Dossier-Zeile bleibt eine
**getrennte, ungeklärte Lead-Identität**; es erfolgt **kein Merge** und keine Übernahme von
Kontakten oder Leistungen des Impressums. Der ähnlich benannte Katalysatoren-Lead in Lampertheim
bleibt ohne belegte Connection ebenfalls außen vor.

**Preise (Ankauf / Verkauf / Gebühren) getrennt:**

- **Ankauf:** Die Betreiber-Website veröffentlicht **keine numerischen Ankaufspreise** und
  **keine Mindestmengen**. Für diese Zeile (Brockhausweg 93) ist **kein** eigener
  Ankaufsbeleg vorhanden.
- **Verkauf:** keine Liste.
- **Gebühren:** **keine Tarife** für Entsorgung, Demontage oder Abholung veröffentlicht. Ob eine
  konkrete Abholung kostenlos, vergütet oder kostenpflichtig ist, bleibt **offen**.

**GEO:** Die DB-Koordinate 53.54/10.1261 löst auf "**11, Cottaweg, Öjendorf, Billstedt**" auf und
liegt damit auf einer **anderen Straße**, nicht am Brockhausweg. Der adresskonforme OSM-Punkt zum
Seed-Eintrag ist "93, Brockhausweg, Kirchsteinbek, Billstedt, 22117" (53.5374086/10.1239657).
Das ist ein **Geocodierungsbedarf**; es wird kein Punkt geraten.

**Negativbefunde (beweisen keine Nichtexistenz):** Eine gezielte Suche in 11880
("Schrotthandel Hamburg", 43 Treffer) und in Das Örtliche ("Steinbach in Hamburg", 15 Treffer)
liefert **keinen** Steinbach-Schrottbetrieb; die Treffer dort sind Handwerks- und
Privatbetriebe (Dachdeckerei, Heizungstechnik, Friseur, Steinbach & Partner GbH) sowie
Privatangaben. Fehlende Treffer beweisen weder Nichtexistenz noch Schließung.

## Timeline

### Recherche 28.10.2026: Impressum geprüft; Adresskonflikt 93 gegen 57 vermessen

- [Recherche 28.10.2026: **Betreiber-Website erneut direkt gelesen, Zuordnung bleibt offen.** Das
  **Impressum** von `mobiler-schrottservice.de` (am 28.10.2026 abgerufen) nennt **Brockhausweg 57,
  22127 Hamburg**, Tel. **01788028550** und **015775230613**, E-Mail
  **donatosteinbach@icloud.com**; als Verantwortlichen nennt es nur die Domain, **keine Firma und
  keine vollständige natürliche Person**. Die **Leistungsseite** nennt "Entsorgungen",
  "Transporter", "Ankauf: Aluminium, Blei, Kupfer, Messing, VA Edelstahl, Kabel, Autobatterien"
  und "Hamburg und Umgebung" - **keine Zahlenpreise, keine Mindestmengen, keine Gebührentarife**.
  **Adresskonflikt vermessen:** Der Seed führt Brockhausweg **93**, 22117; das Impressum nennt
  Brockhausweg **57**, 22127. OSM/Nominatim kennt im Brockhausweg die Hausnummern **57**, **93**
  und **141a/141b/141c** - die Impressums-PLZ **22127** findet sich dort für diese Straße nicht,
  OSM ordnet alle genannten Nummern **22117** zu; die Punkte 93 und 57 liegen **264 m**
  auseinander. Das ist **keine** belegte Identität, sondern ein ungeklärter Adresskonflikt
  zwischen zwei Belegen. Frontmatter deshalb: `trader_type` schrotthaendler → **mobil** (reine
  Abhol-/Demontage-/Transportleistung, kein Platz, keine Zeiten), `website` +
  `website_status: aktiv` ergänzt; **street/postcode/phone/email bleiben unverändert** (Seed
  Brockhausweg 93/22117), weil die Betreiber-Angaben zu 57/22127 **nicht** übernommen werden
  dürfen. **Abgrenzung:** Die Betreiber-Website wird bereits als eigene Zeile
  `hh-billstedt-mobiler-schrottservice-mobiler-schrottha` geführt (Brockhausweg 57, 22127, beide
  Telefonnummern, E-Mail, status aktiv) - **kein Merge**, die Zeile bleibt eine getrennte,
  ungeklärte Lead-Identität. **Negativbefunde (ohne Beweiswert):** 11880 "Schrotthandel Hamburg"
  (43 Treffer) und Das Örtliche "Steinbach in Hamburg" (15 Treffer) liefern **keinen**
  Steinbach-Schrottbetrieb - nur Handwerks- und Privatbetriebe. **Ankauf: keine Zahlenpreise.**
  **Verkauf: keine Liste. Gebühren: keine Tarife**; kostenlose, vergütete oder kostenpflichtige
  Abholung bleibt offen. **GEO: Geocodierungsbedarf** - die DB-Koordinate 53.54/10.1261 löst auf
  "11, Cottaweg, Öjendorf, Billstedt" auf, also auf eine andere Straße; der adresskonforme Punkt
  zum Seed-Eintrag ist "93, Brockhausweg, 22117 Hamburg" (53.5374086/10.1239657). Kein Punkt
  geraten. Quelle(n): https://mobiler-schrottservice.de/impressum/ ;
  https://mobiler-schrottservice.de/leistungen/ ;
  https://www.11880.com/suche/schrotthandel/hamburg ;
  https://www.dasoertliche.de/Themen/Steinbach/Hamburg.html ;
  https://nominatim.openstreetmap.org/search?q=Brockhausweg+57%2C+Hamburg&format=jsonv2 ;
  https://nominatim.openstreetmap.org/search?q=Brockhausweg+93%2C+22117+Hamburg&format=jsonv2 ;
  https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.54&lon=10.1261&zoom=18&addressdetails=1 ;
  read-only `public.db` (28.10.2026).]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Neue Betreiberkandidaten-Website direkt geprüft: Leistungen nennt Ankauf/Abholung von Aluminium, Blei, Kupfer, Messing, VA-Edelstahl, Kabeln und Autobatterien sowie Demontage/Entsorgung; Kontakt und Impressum nennen Brockhausweg 57, 22127 Hamburg, Telefonnummern 01788028550 / 015775230613 und donatosteinbach@icloud.com. Das Impressum nennt als Verantwortlichen nur die Domain, keine Firma oder vollständige natürliche Person. Abweichung zum Seed Brockhausweg 93, 22117 Hamburg und fehlende Register-/Identitätsverknüpfung verhindern Zuordnung und Frontmatter-Fill. Keine numerischen Ankaufspreise, Entsorgungsgebühren oder Demontagepreise veröffentlicht. Quelle(n): https://mobiler-schrottservice.de/ ; https://mobiler-schrottservice.de/leistungen/ ; https://mobiler-schrottservice.de/kontakt/ ; https://mobiler-schrottservice.de/impressum/]

### Recherche 05.10.2026

- [Recherche 05.10.2026: Erneuter Direktcheck der 11880-Suche Schrotthandel Hamburg ohne identifizierbaren Steinbach-Betreiberbeleg; keine Aktivitäts- oder Schließungsaussage aus Portalabwesenheit. Überblick trennt mobile Seed-Leistung von unbestätigter Adresse und fehlender Anlieferberechtigung. Kein Frontmatter-Fill, keine Zusammenführung mit Steinbach Lampertheim; Quelle(n): https://www.11880.com/suche/schrotthandel/hamburg; vorhandene Recherche 02.10.2026]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Negativbefund Hamburg — kein Steinbach-Treffer in 11880 Hamburg (14 Treffer geprüft) und Gelbe Seiten Hamburg (29 Treffer geprüft). Seed-Adresse Brockhausweg 93, 22117 Billstedt unbelegt (kein Betreiber-Impressum, kein Zweitbeleg). Felder bleiben, status pruefung bleibt — Klärfall Existenz/Adresse für Folgewelle. Quelle(n): https://www.11880.com/suche/schrott/hamburg, https://www.gelbeseiten.de/branchen/schrott/hamburg.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott mobil (Abholung)
- Adresse: Billstedt, Brockhausweg 93, 22117
