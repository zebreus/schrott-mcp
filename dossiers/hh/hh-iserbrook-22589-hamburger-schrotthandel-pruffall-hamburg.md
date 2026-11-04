---
slug: hh-iserbrook-22589-hamburger-schrotthandel-pruffall-hamburg
name: Hamburger Schrotthandel
trader_type: schrotthaendler
state: HH
city: Iserbrook 22589
street: ''
postcode: ''
phone: ''
clear_fields: 'phone,website'
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Hamburger Schrotthandel→ PRÜFFALL: Hamburger Schrotthandel? Nein: "Hamburger Schrott"

## Überblick

Weiterhin ein **Prüffall**: Die neu direkt geöffnete Schrottplatz.org-Seite führt „Schrott Hamburger Schrott“ am Heerbrook 4, 22589, mit Tel. 0173 611 05 24 – passend zu den historischen Seed-Daten, aber nur als Portal-Lead. Sie belegt weder den Betreiber noch den aktuellen Betrieb; die frühere DNS-Prüfung vom 01.10.2026 dokumentierte `hamburger-schrott.de` als nicht auflösbar. Nicht mit dem anderswo ansässigen Anbieter hamburger-schrottverwertung.de zusammenführen.

**Preise (Ankauf/Verkauf/Gebühren):** weder der Portal-Lead noch eine verifizierte Betreiberquelle nennt Kurse oder Gebühren. **Offen:** Betreiber und Telefonnummer verifizieren; bis dahin keine Anlieferempfehlung, `status: pruefung` bleibt. Quellen- und Prüfverlauf siehe Timeline.

Unbelegte Telefon-/Domainzuordnung aus aktueller Frontmatter zurückgenommen; alte Werte in Timeline. Erneuter rekursiver DNS-Befund NXDOMAIN bestätigt nur fehlende Domainauflösung, nicht Identität oder Schließung. Ohne zugeordnete Betreiberwebsite website_status unbekannt.

## Timeline

- [Owner-Korrektur 10.10.2026: Explizite Seed-Direktive clear_fields für phone und website ergänzt. Leere Dossierwerte allein erhalten bestehende DB-Enrichmentwerte; nach geprüfter Veröffentlichung soll der normale Seed diese unbelegten Kontaktzuordnungen tatsächlich entfernen, nicht nur aus dem Markdown zurücknehmen. Keine direkte Produktionsmutation, keine GEO-Löschung, keine Schließungsbehauptung. Historische Werte und Gegenbelege unten erhalten. Grundlage: docs/trader-seed.md; research/checkin-20261010-waveD-nordwest-owner-gate.md.]

### Recherche 09.10.2026 / Korrektur 10.10.2026

- [Recherche 10.10.2026: Wave-D-Systemresolver gaierror -2 bedeutet nur fehlende Auflösung dieser Umgebung, kein ausgelesener autoritativer RCODE. Unabhängige Google-DNS-A-Abfrage für hamburger-schrott.de ergibt Status 3/NXDOMAIN, negative SOA de., Kommentar Response from 194.246.96.1, AD false. Rekursiver RCODE-Befund, keine eigene direkte autoritative/DNSSEC-Verifikation. Domain nicht auflösbar, kein Betriebsschließungsbeweis oder Nachweis der Betreiberzuordnung. Quelle(n): https://dns.google/resolve?name=hamburger-schrott.de&type=A ; geprüfter Host hamburger-schrott.de (Wave-D-Systemresolver 09.10.2026)]
- [Korrektur 10.10.2026: Unbelegte Legacy phone 0173 6110524 und website https://hamburger-schrott.de aus aktueller Frontmatter geleert; zugeordneter website_status vorher tot auf unbekannt, weil keine verifizierte Betreiberwebsite zugeordnet ist. Alle alten Werte hier erhalten; Domain-NXDOMAIN als technisches Arbeitswissen bleibt gültig. Heerbrook 4/22589/Telefon nur Seed-/Portal-Lead, keine zwei zulässigen Identitäts-/Kontaktquellen. status pruefung, Ort/Slug/GEO/Typ erhalten; keine Anliefer-/Preis-/Gebührenzusage, keine Schließung aus Nichtfund. Namensähnliche Hamburger Anbieter nicht übernehmen; Quelle(n): https://www.schrottplatz.org/hamburg/schrott-hamburger-schrott-aYEPV3.html ; README.md, Source hierarchy; obiger DNS-Befund und ältere Timeline]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Direktabruf der Portal-Detailseite „Schrott Hamburger Schrott“ ergab Heerbrook 4, 22589 Hamburg-Iserbrook und Tel. 0173 6 11 05 24 – identisch zu den historischen Seed-Werten. Das ist ein weiterer Aggregator-Lead, kein Betreiber-/Registerbeleg; keine Telefonnummer/Adresse in Frontmatter übertragen und keine Identität oder Aktivität bestätigt. Profil enthält keine Ankaufskurse, Verkaufspreise oder Gebühren. Klärfall Betreiber/Telefon bleibt offen; Quelle(n): https://www.schrottplatz.org/hamburg/schrott-hamburger-schrott-aYEPV3.html; README.md, Source hierarchy.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott — nur Verzeichnis + Tel 0173 6110524 (PRÜFFALL)
- Adresse: Iserbrook 22589, Heerbrook 4

### Recherche 01.10.2026

- [Recherche 01.10.2026: kein Betreiber auffindbar; hinterlegte Domain tot; namensähnliche Domain gehört anderem Anbieter; Quelle(n): Dossier-PRÜFFALL, https://hamburger-schrottverwertung.de/ (fremder Anbieter)]
- hamburger-schrottverwertung.de ist ein anderer Anbieter (kostenlose Schrottabholung, keine Iserbrook-Adresse), NICHT mit Dossier verknüpfen.
- Klärfall Folgewelle: Tel. 0173 6110524 gegenrecherchieren oder Dossier schließen.

### Recherche 01.10.2026 (Feedback-Triage #3086)

- [Recherche 01.10.2026: Berechtigt — Domain-Tod per DNS bestätigt (NXDOMAIN für hamburger-schrott.de; website_status tot bestand bereits zurecht). Prüf-Notizen aus name-Feld in Timeline überführt (name bereinigt). Namensvetter-Check: kein realer „Hamburger Schrotthandel" in Iserbrook auffindbar (Treffer: hamburger-schrottverwertung.de = fremder Anbieter, PMK/SMH/Schmidt = andere Firmen) — Phantom-Verdacht, KEINE Adress-/Kontakt-Fills, status pruefung bleibt, Klärfall (Tel. 0173 6110524 weiter offen); Quelle(n): DNS-NXDOMAIN + Websuche 01.10.2026]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Keine Koordinate gesetzt — Klärfall dokumentiert.** Der Eintrag „Hamburger Schrott" ist
  im Dossier ausdrücklich als **Prüffall** geführt: Die einzige belegte Spur ist ein **Verzeichnislead**
  (schrottplatz.org, live 07.10.2026: „Schrott Hamburger Schrott", **Heerbrook 4, 22589 Hamburg**, Tel. 0173 611 05 24,
  identisch mit dem historischen Seed-Wert), der weder Betreiber noch aktuellen Betrieb belegt; die zugehörige Domain
  **hamburger-schrott.de** ist per DNS **NXDOMAIN** (Google-DNS-A-Abfrage, Status 3, Vermerk vom 10.10.2026) — ein
  Recursor-Befund, kein Schließungs- oder Identitätsbeleg. Namensähnliche Hamburger Anbieter (u. a.
  hamburger-schrottverwertung.de) sind ausdrücklich **nicht** zu vermischen. Eine OSM-Hausnummerensuche
  (api.openstreetmap.org/api/0.6/map.json, 0,01-Grad-Kacheln) im Umfeld der Dossier-Adresse **Heerbrook 4** wurde in
  dieser Welle angestoßen und im Umfeld der Adresse weitergeführt; sie wurde durch die API-Drosselung (HTTP 429) nur
  **teilweise** abgeschlossen und ist deshalb **kein** belastbarer Negativbefund — für die Entscheidung ist das **nicht
  maßgeblich**, weil für die Dossier-Adresse keine Betreiber- oder Registerquelle vorliegt. **Konsequenz:** Die
  Zwei-Quellen-Regel ist nicht erfüllt (kein OSM-Objekt, keine Betreiber-/Registerquelle); eine Position auf der
  Heerbrook wäre eine Behauptung über einen Portal-Eintrag ohne Betreiber. `status: pruefung`, `street`/`postcode`
  (leer) und alle übrigen Frontmatter-Felder bleiben unverändert; `clear_fields: 'phone,website'` bleibt unverändert
  bestehen (enthält **kein** `geo`-Token, da keine Koordinate gesetzt wird — der Seed lehnt `clear_fields` neben
  belegtem `lat`/`lon` ab). Es wird **keine** Schließung behauptet. **Nächster Schritt für den Owner:** Betreiber- oder
  Gewerbenachweis (bzw. belastbare Zuordnung des Telefonleiters 0173 6110524) für Heerbrook 4; danach erneute
  OSM-Hausnummerensuche. Quelle(n): https://www.schrottplatz.org/hamburg/schrott-hamburger-schrott-aYEPV3.html
  (Verzeichnislead) ; https://dns.google/resolve?name=hamburger-schrott.de&type=A ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.8950,53.4450,9.9150,53.4650 (Abruf 03.11.2026).]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Keine Koordinate gesetzt - die Dossier-Adresse ist in OSM belegt, der Betrieb aber nicht.** Die Nachprüfung
  ersetzt den unvollständigen Scan der Vorwelle durch ein vollständiges Fenster und liefert erstmals einen positiven
  Adressbefund. **Fenster und Raster:** Kachelraster **9.8210/53.5677 bis 9.8410/53.5877** (Hamburg-Iserbrook,
  Heerbrook), **4 Kacheln von je 0,01 Grad**, **0 Kachel-Fehler** (api.openstreetmap.org/api/0.6/map.json, Abruf
  04.11.2026). Das Fenster enthält die gesuchte Straße **nachweislich**: der **Heerbrook** ist mit **17**
  Straßenobjekten darin erfasst. **Census im Fenster:** **47** Adressobjekte mit `addr:street`=Heerbrook und
  `addr:postcode`=22589 mit **47** verschiedenen Hausnummern (1, 2, 3, **4**, 5, 7, 8, 9, 10, 11, 13, 15, 17, 19, 21,
  23, 25, 27, 29, 29a-29d, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73,
  75, 77, 79, 81, 83) - die **Dossier-Hausnummer 4 existiert**. **Quelle 1 (OSM-Objekt, live geladen 04.11.2026):**
  **way/95961991** (https://api.openstreetmap.org/api/0.6/way/95961991/full.json) trägt `addr:housenumber`=**4**,
  `addr:street`=**Heerbrook**, `addr:city`=Hamburg, `addr:country`=DE, `addr:postcode`=**22589**,
  `building`=**apartments**, `building:levels`=**9**, `roof:levels`=0, `roof:shape`=flat, Version 5 (Stand
  06.06.2026); das Objekt hat **15** eindeutige Knoten, der Mittelwert der eindeutigen Knoten ist **53.5779762 /
  9.8313797**, die Objekt-BBox ist 53.5778652/9.8312289 bis 53.5780903/9.8315936, der Punkt liegt **innerhalb** der
  BBox. Das Objekt trägt **weder `name=` noch `operator=`** - in OSM ist die Nummer 4 ein **9-geschossiges
  Wohngebäude**, kein Schrottplatz. **Quelle 2 (Betreiber-/Register, weiterhin nicht erfüllt):** Die einzige Spur
  bleibt ein **Verzeichnislead** (schrottplatz.org, live 07.10.2026: „Schrott Hamburger Schrott", **Heerbrook 4,
  22589 Hamburg**, Tel. 0173 611 05 24, identisch mit dem historischen Seed-Wert), der weder Betreiber noch aktuellen
  Betrieb belegt; die zugehörige Domain **hamburger-schrott.de** ist per DNS **NXDOMAIN** (Google-DNS-A-Abfrage,
  Status 3, Vermerk vom 10.10.2026) - ein Recursor-Befund, kein Schließungs- oder Identitätsbeleg. Namensähnliche
  Hamburger Anbieter (u. a. hamburger-schrottverwertung.de) sind ausdrücklich **nicht** zu vermischen.
  **Konsequenz:** Die Zwei-Quellen-Regel ist nicht erfüllt (kein Betreiber-/Registerbeleg); eine Position an der
  Heerbrook 4 würde zudem ein **Wohngebäude** als öffentliche Annahmeadresse ausgeben. **Negativaussage,
  qualifiziert:** Auf dem Heerbrook trägt **kein** Objekt einen Namens-, Betreiber- oder Branchenbezug zu einem
  Schrott- oder Metallhändler; das einzige benannte Adressobjekt der Straße ist ein Waschsalon (way/95962047, Nr. 8,
  `name`=„Mein wunderbarer Waschsalon"). **Reichweite / Fehlbelegklausel:** Die Existenz des Adressobjekts 4 belegt
  **die Adresse**, **nicht** den Betrieb „Hamburger Schrott" und **nicht** eine Annahme- oder Abholfunktion; die in
  OSM hinterlegte Wohnnutzung ist eine **Standangabe** und **kein** Beleg gegen eine gewerbliche Nutzung desselben
  Hauses. `status: pruefung`, `street`/`postcode` (beide leer) und alle übrigen Frontmatter-Felder bleiben
  unverändert; `clear_fields: 'phone,website'` bleibt unverändert bestehen (enthält **kein** `geo`-Token, da keine
  Koordinate gesetzt wird - der Seed lehnt `clear_fields` neben belegtem `lat`/`lon` ab); es wird **keine**
  Schließung behauptet. **Nächster Schritt für den Owner:** Betreiber- oder Gewerbenachweis (bzw. belastbare
  Zuordnung des Telefonleiters 0173 6110524) für Heerbrook 4; danach erneute OSM-Hausnummerensuche. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.8210,53.5677,9.8410,53.5877 ;
  https://api.openstreetmap.org/api/0.6/way/95961991/full.json ;
  https://www.schrottplatz.org/hamburg/schrott-hamburger-schrott-aYEPV3.html (Verzeichnislead) ;
  https://dns.google/resolve?name=hamburger-schrott.de&type=A (Abruf 04.11.2026).]
