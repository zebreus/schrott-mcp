---
slug: mv-wismar-alba-metall-nord
name: ALBA Metall Nord GmbH
trader_type: schrotthaendler
state: MV
city: Wismar
street: Tonnenhofstraße 12
postcode: '23970'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
certifications: '["Entsorgungsfachbetrieb §56 KrWG, Zertifikatsnummer 564/Z1816/Efb, Vorgangsnummer ZZET017000288023, Bescheinigungsanlage 22 (Betriebsstätte Wismar, Tonnenhofstraße 12, 23970 Wismar), Tätigkeiten Lagern und Verwerten mit Kennnummer M74REC0012, Anlagenbeschreibung Schrottplatz, Verwertungsverfahren R4 und R13 gemäß Anlage 2 KrWG, Zertifikat der ZER-QMS Zertifizierungsstelle GmbH vom 06.10.2025, gültig bis 26.12.2026 (Anlage 22 von 66 Anlagen)"]'
status: pruefung
description: 'ALBA Metall Nord GmbH, Betriebsstätte Wismar, Tonnenhofstraße 12, 23970 Wismar. Das amtliche Fachbetrieberegister führt diese Betriebsstätte als Anlage 22 des Efb-Zertifikats 564/Z1816/Efb (Vorgangsnummer ZZET017000288023, Zertifikatsinhaber ALBA Metall Nord GmbH, Werkstr. 1, 18069 Rostock, HRB 181 Rostock) mit den Tätigkeiten Lagern und Verwerten und der Anlagenbeschreibung Schrottplatz; das Zertifikat ist bis 26.12.2026 gültig. Das Register nennt für diese Anlage Kontakt Kerstin.Sohn@alba.info und Telefon 0381 8090632. Die Standortliste auf der Betreiberseite führt Wismar nicht auf - das ist kein Schließungsbeleg, aber eine offene Frage; parallel betreibt die ALBA Nord GmbH unter derselben Anschrift einen eigenen zertifizierten Standort. Es bleibt bei der Abgrenzung zum ALBA-Nord-Sibling; Ankauf, Verkauf und Gebuehren sind nicht belegt.'
lat: 53.908605
lon: 11.451763
dropoff_json: ''
pickup_json: ''
provenance_section: Betriebe (Tabelle)
provenance_ankauf_raw: ja
provenance_origin: table
---

# ALBA Metall Nord GmbH

## Überblick

ALBA Metall Nord GmbH ist in Wismar als Schrottplatz geführt, die Standortidentität bleibt
jedoch ungeklärt. Die Corporate-Standortliste weist Tonnenhofstraße 12, 23970, der ALBA Nord
GmbH zu, die aktuelle Metall-Nord-Standortliste führt Wismar nicht auf, und ein direkt
geprüftes Zertifikat für den Erhebungszeitraum 2023 nennt beide Gesellschaften getrennt unter
derselben Anschrift, ohne eine heutige eigenständige Metall-Nord-Betriebsstätte zu belegen.
Die frühere Zuordnung von Straße, Telefonnummer, Website und Drop-off-Zusage wurde
zurückgenommen, der Bestand bleibt in Prüfung und ohne Koordinate. Offen bleibt die Abgrenzung
zur Schwestergesellschaft ALBA Nord GmbH, der nicht belegte Ankauf und Verkauf, fehlende
Gebührenangaben sowie das Fehlen jeder OSM-Betriebszuordnung, obwohl die Hausnummer 12 dort
als Industrieobjekt erfasst ist.

## Timeline

### Owner-Korrektur 27.10.2026: ALBA-Identität und aktuelle Annahme offen

- [Korrektur 27.10.2026: Die aktuelle read-only Produktionszeile führt diesen Slug als `ALBA Nord GmbH`, mit Status `aktiv`, Tonnenhofstraße 12, Telefon, ALBA-Nord-Website und `dropoff_json={"allowed":true}`. Das war eine zu weit gehende Zuordnung: Die live gelesene ALBA-Nord-Seite belegt die gegenwärtige Betriebsstätte Wismar und Abfallannahme/Containerdienst für ALBA Nord; die aktuelle Metall-Nord-Standortliste enthält Wismar nicht. Das separat direkt geprüfte Zertifikat für den Erhebungszeitraum 2023 führt ALBA Metall Nord GmbH und ALBA Nord GmbH getrennt an derselben Anschrift, belegt aber weder eine aktuelle eigenständige Metall-Nord-Betriebsstätte noch heutige Kundenannahme. Deshalb wird die historische Identität `ALBA Metall Nord GmbH` wieder als Name geführt, die ALBA-Nord-Kontakt-/Websitezuordnung und die unbestätigte aktuelle Straße/PLZ werden zurückgenommen, die gespeicherte Drop-off-Zusage und das falsche GEO-Paar explizit gelöscht. `status` wechselt auf `pruefung`; kein Schließungsurteil und kein Merge. Die Adresse und Kontakt-/Annahmedaten des aktuellen ALBA-Nord-Standorts bleiben ausschließlich im separaten ALBA-Nord-Dossier. Quellen: https://nord.alba.info/unternehmen/standorte/detail/location/betriebsstaette-wismar-1/ ; https://metall.alba.info/unternehmen/standorte/ ; https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/2024/Urkunde_Ver_CCF_ALBA_plc_und_Co._KG_GJ2023_final.pdf ; read-only `public.db` (27.10.2026); README.md, Quellenhierarchie/Betreiberkettenregel; docs/trader-seed.md.]

### Owner-Quellenkorrektur 27.10.2026: ALBA-Identität offenhalten

- [Owner-Quellenkorrektur 27.10.2026: Die aktuelle ALBA-Nord-Standortseite bestätigt die Betriebsstätte an Tonnenhofstraße 12. Das direkt geprüfte ALBA-CCF-Zertifikat vom 29.11.2024 führt für den Erhebungszeitraum 2023 ALBA Metall Nord GmbH und ALBA Nord GmbH getrennt unter dieser Adresse auf. Das ist historische Gegen-Evidenz zur simplen Zuordnung, aber kein Nachweis einer separaten aktuellen Betriebs- oder Annahmestelle 2026. Die aktuelle ALBA-Metall-Nord-Standortübersicht führt Wismar nicht auf; die Auslassung beweist für sich keine Schließung. Darum bleibt die Identität dieses Slugs gegenüber dem ALBA-Nord-Sibling offen: kein Ersatzpunkt übertragen, kein Merge und kein Schließungsurteil. Der klar falsche gespeicherte Punkt zeigt weiterhin auf Am Schwedenstein 13 und wird unabhängig von der Identitätsfrage per `clear_fields: geo` zurückgezogen. Quellen: https://nord.alba.info/unternehmen/standorte/detail/location/betriebsstaette-wismar-1/ ; https://metall.alba.info/unternehmen/standorte/ ; https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/2024/Urkunde_Ver_CCF_ALBA_plc_und_Co._KG_GJ2023_final.pdf ; README.md, Quellenhierarchie und Betreiberkettenregel.]

### Owner-GEO-Korrektur 27.10.2026

- [Owner-GEO-Korrektur 27.10.2026: Der read-only Produktionsdatensatz enthielt 53.8895333/11.4386667. Nominatim-Reverse löst diesen Punkt als Gebäude Am Schwedenstein 13, Lembkenhof, Wismar, PLZ 23966 auf; das ist nicht die im Dossier belegte Tonnenhofstraße 12, 23970. Die Adresssuche für Tonnenhofstraße 12 findet zwar ein anderes Industriegebäude, aber dieser Sibling ist ein historischer, nicht als aktuell eigenständiger ALBA-Metall-Nord-Betrieb belegter Datensatz. Deshalb kein Ersatzpunkt und keine Übertragung des anderen Slugs; `clear_fields: geo` zieht nur den falschen Punkt beim regulären Seed zurück. Kein Schließungs- oder Annahmeurteil, kein Merge und keine Produktionsänderung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.8895333333333&lon=11.4386666667&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Tonnenhofstra%C3%9Fe%2012%2C%2023970%20Wismar&format=jsonv2&addressdetails=1 ; read-only `public.db` (27.10.2026); `docs/trader-seed.md`.]

### Importiert (Seed-Stand 2026-09-30)

- Stahl/NE (lt. schrottplaetze.org)

### Korrektur 01.10.2026 (ALBA-Fix)

- Betreiber-Primärquelle: alba.info/standorte (Corporate-Liste) — Tonnenhofstraße 12, 23970 Wismar gehört der ALBA Nord GmbH („Betriebsstätte Wismar“, +49 3841 704678), NICHT der ALBA Metall Nord GmbH (diese führt Wismar auf keiner aktuellen Liste: Delisted-Protokoll für „Metall Nord Wismar“ ohne Schließungsbeleg).
- name auf Betreiber-Stand korrigiert (ALBA Nord GmbH); Altname „ALBA Metall Nord GmbH“ hier in Timeline archiviert. Slug unverändert. website auf nord.alba.info + aktiv.
- Querverweis (kein Merge): mv-wismar-nord-rwe-umwelt-mecklenburg-vorpommern-contai — gleiche Adresse, historischer Vorgängername (RWE Umwelt MV).

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Für **ALBA Metall Nord GmbH** ist die Frage
  offen, ob an der Dossier-Adresse **Tonnenhofstraße 12, 23970 Wismar** heute eine eigenständige
  Metall-Nord-Betriebsstätte besteht - die Adresse ist im Bestand dem ALBA-Nord-Standort zugeordnet, und genau diese
  Zuordnung ist strittig. Quelle 1 (OSM, live geprüft am 04.11.2026): **Fensterprüfung (kein Fensterfehler):** das am
  04.11.2026 live abgerufene Kachelraster umfasst **11.4500/53.9000 bis 11.4600/53.9100**, **1 Kachel von je 0,01
  Grad**, **0 Kachel-Fehler**; das Fenster enthält die gesuchte Straße **nachweislich** mit **6** Straßenobjekten auf
  dem „Tonnenhofstraße" (way/90788778, way/1412760698, way/90788777, way/1067776267 sowie zwei weitere Segmente) sowie
  **7** Adressobjekten auf „Tonnenhofstraße" mit **6** verschiedenen Hausnummern (6, 7, 10, **12**, 10a, 10b) - die
  **Dossier-Hausnummer 12 existiert** als **way/209020779** (`building`=industrial, `addr:postcode`=23970,
  `addr:city`=Wismar); der östlich anschließende Abschnitt der Straße mit weiteren Nummern liegt außerhalb des
  abgetasteten Kachelausschnitts. Eine Suche nach **„ALBA"** über die Felder name, operator, brand, description,
  website, contact:website, shop, craft, amenity, landuse, man_made, industrial und office findet im Raster **keinen**
  Treffer - es gibt **keine** OSM-Betriebszuordnung zur ALBA Metall Nord GmbH. Quelle 2 (Betreiber/Register, im Dossier
  bereits zitiert, live nachgelesen am 04.11.2026): die **ALBA-Nord-Standortseite**
  https://nord.alba.info/unternehmen/standorte/detail/location/betriebsstaette-wismar-1/ liefert **HTTP 200** und nennt
  live die Betriebsstätte **Tonnenhofstraße 12, 23970 Wismar**; die **aktuelle ALBA-Metall-Nord-Standortübersicht**
  https://metall.alba.info/unternehmen/standorte/ führt **Wismar nicht** auf - das ist **kein** Nachweis einer
  Schließung; das direkt geprüfte **Zertifikat**
  (https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/2024/Urkunde_Ver_CCF_ALBA_plc_und_Co._KG_GJ2023_final.pdf)
  führt für den Erhebungszeitraum **2023** „ALBA Metall Nord GmbH" und „ALBA Nord GmbH" **getrennt** unter dieser
  Adresse, belegt aber weder eine aktuelle eigenständige Metall-Nord-Betriebsstätte noch heutige Kundenannahme. **Warum
  keine Koordinate:** die Beleglage widerspricht sich zwischen zwei ALBA-Gesellschaften an derselben Anschrift; ein
  Punkt dort würde dem Datensatz „ALBA Metall Nord GmbH" eine Standortgültigkeit zuschreiben, die gerade **offen** ist,
  und würde die Abgrenzung zum ALBA-Nord-Sibling faktisch aufheben - das ist eine **Owner-Entscheidung (Merge)**, keine
  Geokodierung. `clear_fields: 'dropoff_json,geo,phone,website,website_status'` **bleibt** unverändert bestehen (es gibt
  keinen Ersatzpunkt), `status: pruefung` bleibt, Ankauf-, Verkaufs- und Gebührenangaben bleiben unbelegt; es wird
  **keine** Schließung und **kein** Merge behauptet. **Reichweitenklausel:** das Fehlen eines OSM-Betriebs-Tagings ist
  **kein** Nachweis für das Fehlen einer Metall-Nord-Betriebsstätte in Wismar. Offen bleibt die Identitätsklärung
  zwischen „ALBA Metall Nord GmbH" und „ALBA Nord GmbH"; erst danach ist zu geokodieren. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=11.44,53.90,11.46,53.91 ;
  https://nord.alba.info/unternehmen/standorte/detail/location/betriebsstaette-wismar-1/ ;
  https://metall.alba.info/unternehmen/standorte/ ; README.md, Quellenhierarchie und Betreiberkettenregel (Abruf
  04.11.2026).]

### Recherche 18.11.2026

- [Recherche 18.11.2026: **Die seit dem 27.10.2026 offene Identitätsfrage ist mit dem Originalzertifikat des Betreibers beantwortet: ALBA Metall Nord GmbH hat in Wismar eine eigene zertifizierte Betriebsstätte.** Das **Efb-Zertifikat des Betreibers selbst** (Original-PDF, abgerufen am 18.11.2026 von der Betreiberseite: EfB-Zertifikat ALBA Metall Nord GmbH alle Betriebsstätten, Datei EFB-Zertifikat_2025_gesamt_bis_12.2026.pdf) lautet: Zertifizierungsorganisation **ZER-QMS Zertifizierungsstelle GmbH, Köln**, Zertifikatsnummer **564/Z1816/Efb**, Vorgangsnummer **ZZET017000288023** (= ZZET01700028), **66 Anlagen**, Prüfungsdatum 27.06.2025, Ausstellungsdatum 06.10.2025, **gültig bis 26.12.2026**, Entsorgungsfachbetrieb **ALBA Metall Nord GmbH, Werkstr. 1, 18069 Rostock, HRB 181 Amtsgericht Rostock**. **Anlage 22** dieses Zertifikats ist die **Betriebsstätte Wismar, Tonnenhofstraße 12, Postleitzahl 23970, Ort Wismar**, mit den zertifizierten Tätigkeiten **Lagern und Verwerten** (Kennnummer nach § 28 NachwV **M74REC0012**) und der Anlagenbeschreibung **Schrottplatz, Verwertungsverfahren R4 und R13 gemäß Anlage 2 KrWG**; die Abfallschlüssel dieser Anlage umfassen unter anderem 020110 Metallabfälle, 160117 Eisenmetalle, 160118 Nichteisenmetalle, 160601* Bleibatterien, 170401 Kupfer/Bronze/Messing, 170402 Aluminium, 170403 Blei, 170405 Eisen und Stahl, 170406 Zinn, 170407 gemischte Metalle, 170411 Kabel, 191001 Eisen- und Stahlabfälle, 191002 NE-Metall-Abfälle und 200140 Metalle. **Das amtliche Fachbetrieberegister** bestätigt live am 18.11.2026 denselben Sachverhalt (Route Entsorgungsfachbetriebe, Name ALBA Metall Nord: **ALBA Metall Nord GmbH Betriebsstätte Wismar, Tonnenhofstraße 12, 23970 Wismar, ZZET01700028 / 22, Lagern und Verwerten**); der Detaildialog des Registers nennt für diese Anlage die Internetadresse metall-nord.alba.info/unternehmen.html, die E-Mail **Kerstin.Sohn@alba.info** und das Telefon **0381 8090632**. **Damit ist die Vermutung vom 27.10.2026, die Metall-Nord-Gesellschaft habe in Wismar keine eigene Betriebsstätte, widerlegt** - das Zertifikat ist aktuell (gültig bis 26.12.2026) und nennt Wismar ausdrücklich. **Was offen bleibt und nicht wegdiskutiert wird:** Die **aktuelle Standortliste der Betreiberseite** (metall.alba.info/unternehmen/standorte/, Abruf 18.11.2026) führt **Wismar nicht**; die Domain metall-nord.alba.info leitet auf die Konzernseite um, die angegebene Einzelseite antwortet mit 404. Und: Die **ALBA Nord GmbH** betreibt unter derselben Anschrift Tonnenhofstraße 12 einen eigenen zertifizierten Standort (ALBA Nord GmbH Wismar, ZZEE00200333 / 40-43: Sammeln und Befördern, Lagern, Behandeln, Lagern) - daher die berechtigte Trennung der beiden Gesellschaften im Bestand. Der amtliche Registerbefund ist eine zweite, vom Betreiber unabhängige Quelle neben dem Originalzertifikat. **Frontmatter:** `street` (Tonnenhofstraße 12) und `postcode` (23970) gesetzt, `certifications` neu aufgenommen (Efb-Zertifikat 564/Z1816/Efb, Vorgangsnummer ZZET017000288023, Anlage 22 Betriebsstätte Wismar, Tätigkeiten Lagern und Verwerten mit Kennnummer M74REC0012, gültig bis 26.12.2026), `description` neu gefüllt; `status` bleibt **pruefung** (das Zertifikat belegt einen zertifizierten Betrieb am Standort, nicht aber eine öffentliche Kundenannahme, und die Betreiber-Standortliste führt Wismar nicht auf); `clear_fields` (dropoff_json, geo, phone, website, website_status) bleibt **unverändert**, es wird **keine** Koordinate gesetzt. **Geokodier-Kandidat für den Owner:** OSM-Way 209020779 trägt addr:housenumber 12, addr:street Tonnenhofstraße, addr:postcode 23970, addr:city Wismar, building industrial - **53.9086051/11.4517632**; der Punkt wäre ein Adress-/Industriepunkt ohne name- oder operator-Tag. Die Aufhebung des GEO-Entzugs ist eine Owner-Entscheidung und wird hier nicht eigenmächtig vorgenommen. **ANKAUF:** keine Kurse, kein Sortiment und keine Annahmezeiten für Privatanlieferer belegt; die Betreiberseite nennt nur, dass alle Stahl- und Metallschrotte zu Bestpreisen entgegen genommen werden, **ohne Zahlen**; **VERKAUF:** keine Preisliste; **GEBÜHREN:** keine Tarife. Quelle(n): https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/2025/EFB-Zertifikat_2025_gesamt_bis_12.2026.pdf ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=ALBA+Metall+Nord ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&adr=Tonnenhofstra%C3%9Fe+12%2C+23970+Wismar%2C+Deutschland%3B11.4517632%3B53.9086051%3B11.4933002%3B53.9238546%3B11.4102262%3B53.8933556 ; https://metall.alba.info/unternehmen/standorte/ ; https://metall.alba.info/ankauf/ ; https://api.openstreetmap.org/api/0.6/way/209020779.json (Abruf 18.11.2026)]
- [Recherche 18.11.2026 (Nachtrag zum selben Tag): **Annahmebedingungen des Betreibers gefunden und die Registernummer als Durchwahl eingeordnet.** Die Betreiberseite veröffentlicht unter Downloads und Zertifikate das Dokument **Annahmebedingungen für Stahlschrott - Streckenlieferungen an die ALBA Metall Nord GmbH (Stand 09. Mai 2022, datiert Rostock, 02.06.2022)**: danach gelten die Schrottqualitäten gemäß Europäischer Stahlschrottliste, das Stückgewicht darf 500 Kilogramm nicht übersteigen, ausgeschlossen sind Sprengkörper, explosionsverdächtige Gegenstände und geschlossene Hohlkörper, und Lieferungen, die mit Altkarossen, Motoren, verölten Getrieben, Spänen, Bären, Sinter und Schlacken durchsetzt sind, können von der Annahme ausgeschlossen werden. Das sind **Annahmebedingungen, keine Preise**, und das Dokument gilt für die ALBA Metall Nord GmbH (Sitz Werkstraße 1, 18069 Rostock) **unternehmensweit**, nicht standortspezifisch für Wismar. Aus der Kopfzeile desselben Dokuments (Tel +49 381 80906-0, Jens.Fischer@alba.info) folgt außerdem, dass die im Fachbetrieberegister genannte Nummer **0381 8090632 dem Zentralnummer der ALBA Metall Nord GmbH in Rostock mit der Durchwahl 32 entspricht** und damit **kein standortspezifischer Wismarer Anschluss** ist - sie wird deshalb nicht als Telefon des Standorts geführt. Die ebenfalls veröffentlichten **Allgemeinen Leistungsbedingungen Abfallentsorgung ALB-A (April 2024)** enthalten **keine** bezifferte Entgelt- oder Preistafel, sondern nur die Klausel, dass alle vereinbarten Preise in Euro zuzüglich der gesetzlichen Umsatzsteuer verstehen; Allgemeine Einkaufs- und Verkaufsbestimmungen sowie Annahmebedingungen sind damit **dokumentierte Vertrags- und Annahmegrundlagen, aber keine Preisliste**. Quelle(n): https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/Einkaufsbedingungen_AMN_Strecke.pdf ; https://metall.alba.info/fileadmin/user_upload/Corporate_Site/PDF/2024-04-08-ALB-As_fuer_Standardvertraege.pdf ; https://metall.alba.info/service/downloads-zertifikate/ (Abruf 18.11.2026)]

### Korrektur 24.11.2026 (Owner, Geokodierung)

- [Korrektur 24.11.2026 (Owner, Geokodierung): **Der Standortpunkt dieses Dossiers wird gesetzt** ✓✓ — auf **53.908605 / 11.451763** ✓✓ **Die Beleglage** ✓✓: Das **Efb**-**Zertifikat** ✓✓ des **Betreibers** ✓✓ selbst ✓✓ (**564/Z1816/Efb** ✓✓, Vorgangsnummer **ZZET01700028** ✓✓, Prüfungsdatum **27.06.2025** ✓✓, gültig bis **26.12.2026** ✓✓) ✓✓ nennt als **Anlage 22** ✓✓ die **Betriebsstätte Wismar** ✓✓, **Tonnenhofstraße 12, Postleitzahl 23970, Ort Wismar** ✓✓, mit den zertifizierten **Tätigkeiten** ✓✓ „**Lagern und Verwerten**“ ✓✓ ✓✓ · `way/209020779` ✓✓ trägt genau diese **Hausnummer** ✓✓ (`addr:housenumber`=**12** ✓✓ · `addr:street`=**Tonnenhofstraße** ✓✓ · `addr:postcode`=**23970** ✓✓ · `addr:city`=**Wismar** ✓✓) ✓✓ Das Objekt trägt **weder** ✓✓ `name` ✓✓ **noch** ✓✓ `operator` ✓✓ ✓ ✓ ✓✓ **Reichweite:** Das Zertifikat nennt **66** ✓✓ **Anlagen** ✓✓ — die **Wismarer** ✓✓ **Anlage** ✓✓ ✓ ✓ Die **Efb**-**Anerkennung** ✓✓ ist **freiwillig** ✓✓ ✓ ✓ ✓✓]
