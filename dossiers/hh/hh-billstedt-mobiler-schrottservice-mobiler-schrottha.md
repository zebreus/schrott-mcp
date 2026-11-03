---
slug: hh-billstedt-mobiler-schrottservice-mobiler-schrottha
name: Mobiler Schrottservice (mobiler-schrotthändlerhamburg.de)
trader_type: mobil
state: HH
city: Billstedt
street: Brockhausweg 57
postcode: '22127'
phone: 01788028550, 015775230613
email: donatosteinbach@icloud.com
opening_hours: ''
website: https://mobiler-schrottservice.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: '{}'
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: ja (Abholung)
provenance_origin: table
---

# Mobiler Schrottservice (mobiler-schrotthändlerhamburg.de)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 03.10.2026 (Feedback #4431 Owner-Triage)

- [Recherche 03.10.2026: Aktuelle Betreiberseite nennt Brockhausweg 57 und Schrottankauf/Abholung, schließt Kundenanlieferung aber nicht aus; die Adresse ist allein Impressums-/Kontaktadresse, keine nachgewiesene Annahmestelle. Gleichnamiger Eintrag „Steinbach“ an Nr. 93 mit anderer Telefonnummer stammt aus Verzeichnissen und bleibt getrennte, ungeklärte Lead-Identität. Kein Merge und kein `dropoff_json.allowed=false`; Frontmatter bleibt unbekannt.; Quelle(n): https://mobiler-schrottservice.de/, https://mobiler-schrottservice.de/leistungen/, https://mobiler-schrottservice.de/impressum/]

### Importiert (Seed-Stand 2026-09-30)

- Alu, Kupfer, Messing, VA, Kabel, Batterien; Abholung/Demontage

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2201 teilweise berechtigt — Kontakt-Fills (PLZ, Telefon, E-Mail) + website_status aktiv per Betreiber-Kontaktseite (Brockhausweg 57, 22127 Hamburg); trader_type schrotthaendler → mobil (reiner Ankauf-/Abhol-/Demontage-/Transport-Dienst, keine Anlieferung belegt); Einzelunternehmen ohne HRB → EINZELBELEG, Zweitbeleg offen; Dossier-Name nennt Fremd-Domain mobiler-schrotthändlerhamburg.de, Website ist mobiler-schrottservice.de (Alias vermerkt); Adresse neu → Koordinaten neu geocodieren; Quelle(n): https://mobiler-schrottservice.de/ + /kontakt/]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Keine Koordinate gesetzt — Klärfall dokumentiert.** Die Betreiberquelle ist hier
  belastbar: Das **Impressum** des Betreibers (live abgerufen 03.11.2026, https://mobiler-schrottservice.de/impressum/)
  nennt **„mobiler-schrottservice.de, Brockhausweg 57, 22127 Hamburg"**, Telefon **01788028550** und **015775230613**,
  E-Mail donatosteinbach@icloud.com — identisch mit der Dossier-Adresse, -Telefonnummern und -E-Mail. **Die
  geokodierungsseitige Quelle fehlt jedoch:** Eine OSM-Hausnummerensuche (api.openstreetmap.org/api/0.6/map.json,
  0,01-Grad-Kacheln) im Umfeld der Dossier-Adresse **Brockhausweg 57** wurde in dieser Welle angestoßen und über das
  Kachelraster **53.5800-53.6200 / 9.9600-10.0600** weitergeführt; sie wurde durch die API-Drosselung (HTTP 429 „You
  have downloaded too much data") nur **teilweise** abgeschlossen und ist deshalb **kein** belastbarer Negativbefund —
  ein Objekt mit `addr:housenumber`=57 und `addr:street`=Brockhausweg wurde in den abgerufenen Kacheln **nicht**
  angetroffen, die Dossier-Hausnummer ist damit in OSM **bisher nicht nachgewiesen**. Ohne Objekt ist kein „Mittelwert
  der eindeutigen Knoten" bildbar und kein Punkt innerhalb einer Objekt-BBox prüfbar; eine Position „ungefähr im
  Brockhausweg" wäre eine geratene Koordinate. **Konsequenz:** Die Dossier-Adresse bleibt ungeokodiert, obwohl sie
  betreiberseitig belegt ist — die Nachpflege ist eine OSM-Datenlücke, kein Beleg gegen die Adresse. **Abgrenzung,
  unverändert übernommen:** Das Impressum ist eine **Kontakt-/Impressumsadresse**; das Dossier hält fest, dass eine
  Kundenanlieferung dort **nicht** ausgeschlossen, aber **nicht** belegt ist — eine spätere Koordinate würde daher die
  **Betreiberanschrift** markieren, keinen Annahmehof. `status: aktiv`, `street`/`postcode`, Telefon/E-Mail,
  `website_status: aktiv` und alle übrigen Frontmatter-Felder bleiben unverändert; es wird **keine** Anlieferung
  behauptet. **Nächster Schritt für den Owner:** OSM-seitig das Adressobjekt Brockhausweg 57 ergänzen bzw.
  identifizieren und die Koordinate danach aus dem Objekt nachziehen. Quelle(n):
  https://mobiler-schrottservice.de/impressum/ ; https://mobiler-schrottservice.de/leistungen/ ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9600,53.5800,10.0600,53.6200 (Abruf 03.11.2026, teilweise
  gedrosselt).]
