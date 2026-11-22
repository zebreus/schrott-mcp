---
slug: mv-ribnitz-damgarten-acs-jager-schrotthandel-abbruch-containe
clear_fields: 'dropoff_json,geo'
name: ACS Jäger (Schrotthandel, Abbruch, Containerdienst)
trader_type: containerdienst
state: MV
city: Ribnitz-Damgarten
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: http://www.acs-jaeger.de
website_status: blockiert
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Betriebe (Tabelle)
provenance_ankauf_raw: ja
provenance_origin: table
---

# ACS Jäger (Schrotthandel, Abbruch, Containerdienst)

## Überblick

„ACS Jäger“ ist als Betreiberidentität nicht bestätigt. Die Website unter `acs-jaeger.de` war bei den dokumentierten Direktabrufen blockiert; das Impressum/der Seiteninhalt ließ sich nicht verifizieren. Branchenverzeichnisse nennen Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg und „Andreas Jäger“, aber mit widersprüchlichen Telefonnummern (03821 708737 bzw. 0175 7847211). Das ist ein Recherche- und Adresslead, keine belastbare Zuordnung der Seed-Firma; website_status `blockiert` bleibt.

- **Annahme:** keine Betreiberbestätigung für öffentliche Schrottannahme, Abbruchannahme oder Containerstandort.
- **Ankauf:** Seed-Angabe „ja“ und Branchenbezeichnung sind keine bestätigte Vergütung; keine Kurse.
- **Verkauf:** keine Produkt-/Preisliste gefunden.
- **Gebühren:** keine belastbaren Abbruch-, Container- oder Entsorgungstarife.
- **GEO:** Bei den Borger Tannen 8 ist nur ein Verzeichniskandidat; keine Koordinaten gesetzt. Nicht mit „Allround Computer Service“ in Großkarolinenfeld bzw. `jaeger-acs.de` gleichsetzen.

## Timeline

### Owner-GEO-/Annahme-Korrektur 24.10.2026

- [Owner-GEO-Korrektur 24.10.2026: Die read-only Produktionszeile enthielt den Punkt 54.2422, 12.4567 und `dropoff_json={"allowed":true}`. Die aktuelle Recherche fand nur widersprüchliche Verzeichnis-Telefonangaben und eine blockierte Website; Betreiberidentität, heutige Betriebsstätte und öffentliche Annahme sind nicht bestätigt. Beide gespeicherten Zusagen werden deshalb mit `clear_fields: 'dropoff_json,geo'` zurückgenommen, nicht als Beweis für falschen Betrieb oder Annahmeverweigerung. Keine Kontaktdaten oder Ersatzkoordinate ergänzt; Status `pruefung` bleibt. Keine Produktionsdaten geändert. Quellen: read-only `public.db` (24.10.2026); die im Rechercheeintrag 24.10.2026 verlinkten Verzeichnis-/Websitequellen; `docs/trader-seed.md`.]

### Recherche 24.10.2026

- [Recherche 24.10.2026: Aktuelle Verzeichnis-Suchtreffer stützen weiterhin Andreas Jäger/Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg, lösen die Telefonnummer aber nicht auf: Das Telefonbuch führt 03821 708737, das Stadtbranchenbuch „Andreas Jäger“ an derselben Anschrift mit 0175 7847211. Beides sind Aggregator-Leads, keine Betreiberbestätigung. `http://www.acs-jaeger.de` blieb blockiert (HTTP 403/Timeout); keine Umleitung oder Verbindung zur namensähnlichen `jaeger-acs.de` aus Großkarolinenfeld hergestellt. Wegen nicht verifizierter Betreiberidentität/aktueller Tätigkeit status `aktiv` → `pruefung`; das bedeutet keine Schließungsfeststellung. Öffentliche Annahme, Ankauf, Verkauf und Gebühren bleiben unbelegt; keine Kontaktfelder/Koordinaten ergänzt. Adresse nur Geokodierungskandidat. Quellen: https://www.dastelefonbuch.de/Branchen/Schrotthandel/Stralsund ; https://adresse.dastelefonbuch.de/Ribnitz-Damgarten/2-0115907281761-Container-ACS-J%C3%A4ger-Ribnitz-Damgarten-Bei-den-Borger-Tannen.html ; https://ribnitz-damgarten.stadtbranchenbuch.com/3369216.html ; http://www.acs-jaeger.de ; https://jaeger-acs.de/ .]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — Betreiber-Website http://www.acs-jaeger.de nicht erreichbar (https Timeout, http 403, 01.10.2026); website_status blockiert bleibt. Nur Aggregator-Leads (Gelbe Seiten, Das Örtliche, Cylex: „ACS Jäger / Andreas Jäger", Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg, Tel. 03821/708737). Kein Register, kein Betreiber-Social. Adresse daher nicht frontmatter-fähig.; Quelle(n): Live-Abruf 01.10.2026 + Lead-Recherche (Aggregator-Leads, kein Beleg)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott, Abbruch, Container

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Für **ACS Jäger** sind Betreiberidentität,
  heutige Betriebsstätte und öffentliche Annahme nicht bestätigt, die zitierten Lead-Seiten sind heute **nicht mehr
  abrufbar**, und die Dossier-Adresse **Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg** ist im abgetasteten
  Stadtbereich **nicht** erfasst. Quelle 1 (OSM, live geprüft am 04.11.2026): **Fensterprüfung (kein Fensterfehler):**
  das am 04.11.2026 live abgerufene Kachelraster umfasst **12.4400/54.2800 bis 12.4900/54.3100**, **12 Kacheln von je
  0,01 Grad**, **0 Kachel-Fehler**; das Fenster erfasst den **Stadtbereich Ribnitz-Damgarten** mit **194**
  Adressobjekten auf **14** Straßen (Damgartener Straße, Gutsstraße, Hafenweg, Heideweg, Hummelberg, Langendammer
  Straße, Saaler Chaussee, Schwarze Straße, Seereihe, Waldemar-Schröder-Weg, Waldreihe, Wasserreihe, Weidensteig, Zum
  Deich) - eine Straße **„Bei den Borger Tannen"** ist dort **nicht** erfasst, und die Dossier-Hausnummer **8** findet
  sich auf keiner dieser Straßen. Eine Suche nach **„ACS Jäger"** und **„Andreas Jäger"** über die Felder name,
  operator, brand, description, website, contact:website, shop, craft, amenity, landuse, man_made und industrial findet
  im Fenster **keinen** Treffer - es gibt **keine** OSM-Betriebszuordnung. Das Raster deckt **nicht** den Ortsteil
  **Borg** ab, das Fehlen der Straße im Stadtbereich ist deshalb **kein** Nachweis gegen deren Existenz, aber **auch
  kein** Beleg für einen Standort des Betriebs. Quelle 2 (Verzeichnis/Betreiber, im Dossier bereits zitiert, live
  nachgelesen am 04.11.2026): die Portal-Adresse
  https://adresse.dastelefonbuch.de/Ribnitz-Damgarten/2-0115907281761-Container-ACS-J%C3%A4ger-Ribnitz-Damgarten-Bei-den-Borger-Tannen.html
  liefert heute **HTTP 410 (Gone)** und das Stadtbranchenbuch
  https://ribnitz-damgarten.stadtbranchenbuch.com/3369216.html **HTTP 403**; der im Dossier dokumentierte Stand war
  „Andreas Jäger", **Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg**, mit **zwei** voneinander abweichenden
  Telefonnummern (03821 708737 laut Das Telefonbuch, 0175 7847211 laut Stadtbranchenbuch) - beides **Verzeichnisleads**,
  keine Betreiberbestätigung. Die Betreiberdomain **http://www.acs-jaeger.de** bleibt **blockiert** (HTTP 403/Timeout)
  und wurde **nicht** mit der namensähnlichen, **anderen** Firma `jaeger-acs.de` aus Großkarolinenfeld gleichgesetzt.
  **Warum keine Koordinate:** die zitierten Lead-Seiten sind heute **nicht mehr abrufbar**, die Telefonnummer bleibt
  widersprüchlich und eine Betreiber- oder Registerquelle fehlt; die Dossier-Adresse ist damit **nicht mehr belegbar**
  (und in OSM ohne Betriebsbezug), eine Punktwahl wäre eine **Rathausnahme**. `clear_fields: 'dropoff_json,geo'`
  **bleibt** unverändert bestehen (es gibt keinen Ersatzpunkt), `status: pruefung` bleibt, Ankauf-, Verkaufs- und
  Gebührenangaben bleiben unbelegt, es wird **keine** Schließung behauptet. **Reichweitenklausel:** die
  Nichterreichbarkeit der Verzeichnisse und das Fehlen eines OSM-Tagings sind **kein** Nachweis für das Fehlen oder die
  Auflösung des Betriebs. Offen bleiben Inhaber, Standort und Annahme; erst nach deren Bestätigung ist zu geokodieren.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=12.44,54.28,12.49,54.31 ;
  https://adresse.dastelefonbuch.de/Ribnitz-Damgarten/2-0115907281761-Container-ACS-J%C3%A4ger-Ribnitz-Damgarten-Bei-den-Borger-Tannen.html
  (HTTP 410 am 04.11.2026) ; https://ribnitz-damgarten.stadtbranchenbuch.com/3369216.html (HTTP 403 am 04.11.2026) ;
  http://www.acs-jaeger.de (blockiert, Abruf 04.11.2026).]

### Recherche 22.11.2026 (Welle 0600 C)

- [Recherche 22.11.2026: **Der Negativbefund vom 04.11.2026 zur Straße wird korrigiert: `Bei den Borger Tannen` ist in OSM kartiert und trägt die Hausnummer 8, das Fehlen im Dossier war ein Fenstereffekt.** Eine flächige Suche über 54.15/12.30 bis 54.35/12.60 liefert für den Ortsteil Ribnitz-Damgarten-Borg die Straßenobjekte way/4867658, way/39331090, way/39331092, way/314279524, way/504821984, way/505020222, way/505020224, way/505020226, way/505020227 und way/505024810 sowie die Adressobjekte way/203952253 (`addr:housenumber`=1), way/256386330 (3), way/256386327 (4), way/256386336 (5), way/256386326 (6), way/256386339 (7), **way/479923675 (8)**, way/256112841 (9) und way/256112788 (10), alle mit `addr:street`=Bei den Borger Tannen. Das Adressobjekt way/479923675 trägt `addr:housenumber`=8 und `building`=yes bei 54.2268402/12.3723922; es trägt weder `name` noch `operator` und auch keine Postleitzahl- oder Ortsangabe. Das abgetastete Fenster der Welle vom 04.11.2026 reichte nur bis 12.49 Länge und damit östlich der Straße, weshalb der damalige Befund `nicht erfasst` nicht belastbar war. Reichweite: das Fehlen eines benannten Betriebsobjekts ist kein Nachweis für die Nichtexistenz des Betriebs; eine Koordinate wird deshalb nicht gesetzt, die Anschrift ist aber als belastbarer Adressanker für die spätere Prüfung dokumentiert. Quellen: https://overpass-api.de/api/interpreter ; https://api.openstreetmap.org/api/0.6/way/479923675/full.json .]
- [Recherche 22.11.2026: **Betreiberdomain und Verzeichnisse: Abrufbefunde, keine Gegenbelege.** `http://www.acs-jaeger.de` und `http://acs-jaeger.de` antworten am 22.11.2026 weiterhin mit HTTP 403, `https://www.acs-jaeger.de` bricht mit einem Verbindungsfehler ab; die Sperre besteht also unverändert fort. Der im Dossier zitierte Portal-Eintrag `Das Telefonbuch` zur Container-ACS Jäger antwortet mit HTTP 410 (Gone), das Stadtbranchenbuch mit HTTP 403; beide Verzeichnisquellen sind damit heute nicht mehr lesbar. Das ist ein Abrufbefund zur Lesbarkeit der Quellen und kein Nachweis für die Auflösung des Betriebs. Quellen: http://www.acs-jaeger.de ; https://adresse.dastelefonbuch.de/Ribnitz-Damgarten/2-0115907281761-Container-ACS-J%C3%A4ger-Ribnitz-Damgarten-Bei-den-Borger-Tannen.html ; https://ribnitz-damgarten.stadtbranchenbuch.com/3369216.html .]
- [Recherche 22.11.2026: **Amtliches Fachbetrieberegister, Handelsregister und Insolvenzveröffentlichungen: kein Treffer, Reichweitenklausel.** Für Ribnitz-Damgarten führt der Registerspiegel keine Ortsseite (Abruf 22.11.2026, HTTP 404); Reichweitenklausel: die Efb-Anerkennung nach Paragraf 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis. Die Namenssuche `ACS Jager Ribnitz` und `Andreas Jager Ribnitz` im aufbereiteten Handelsregisterportal liefert für Unternehmen wie für Personen `Keine Suchergebnisse!`; die Suche im amtlichen Insolvenzveröffentlichungsportal nach `Jäger` (Wildcard-Modus, Zeitraum 01.01.2000 bis 22.11.2026) liefert Veröffentlichungen zu anderen Personen an anderen Orten, aber keinen Treffer mit Sitz Ribnitz-Damgarten. Quellen: https://schrottregister.pages.dev/ort-ribnitz-damgarten ; https://www.online-handelsregister.de/suche/ACS%20Jager%20Ribnitz ; https://neu.insolvenzbekanntmachungen.de/ap/suche.jsf .]
- [Recherche 22.11.2026: **GEO gemessen: es ist keine Koordinate gespeichert, der zurückgenommene Punkt läge 5739 m neben der belegten Anschrift.** Der read-only-Abgleich `public.db` (22.11.2026) zeigt für diesen Slug keine Koordinate, der am 24.10.2026 zurückgenommene Punkt ist damit nicht mehr gespeichert; zur Einordnung wird der damalige Wert 54.2422/12.4567 herangezogen, der sich über die Nominatim-Rückauflösung auf `63, Damgartener Chaussee, Ribnitz, Damgarten, Ribnitz-Damgarten, 18311` (OSM way/105699707, `building`=commercial) auflösen ließe, also auf eine andere Straße; zum jetzt belegten Adressobjekt way/479923675 (Bei den Borger Tannen 8) beträgt der Abstand 5739 m (Haversine). `clear_fields: 'dropoff_json,geo'` bleibt unverändert bestehen, weil es keinen Ersatzpunkt gibt; der Status bleibt `pruefung`, und es wird keine Schließung behauptet. **Preise getrennt gefuehrt:** ANKAUF: keine bestätigte Vergütung und keine Kurse belegt; VERKAUF: keine Produkt- oder Preisliste; GEBÜHREN: keine belastbaren Abbruch-, Container- oder Entsorgungstarife belegt. Quellen: read-only `public.db` (22.11.2026); https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=54.2422&lon=12.4567&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/479923675/full.json .]
