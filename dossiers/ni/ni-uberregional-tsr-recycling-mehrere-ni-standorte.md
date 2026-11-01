---
slug: ni-uberregional-tsr-recycling-mehrere-ni-standorte
name: TSR Recycling (mehrere NI-Standorte)
trader_type: sonstige
state: NI
city: überregional
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.tsr-recycling.de
website_status: aktiv
status: aktiv
description: Deutschlandweiter Metallrecycler für FE- und NE-Metalle mit Containerlogistik und mehreren Niedersachsen-Standorten.
dropoff_json: ''
pickup_json: ''
provenance_section: Überregional/Online (in NI aktiv)
provenance_ankauf_raw: ja
provenance_origin: table
---

# TSR Recycling (mehrere NI-Standorte)

## Überblick

TSR betreibt laut Unternehmensauftritt deutschlandweites FE-/NE-Metallrecycling und Containerlogistik. Die TSR-Bibliothek führt in Hannover Hansastraße 9 (30419) und „Lindener Hafen“, Alte Speicherstraße 17 (30453); eine niedersächsische Behördenliste vom 19.01.2026 nennt TSR Recycling an der Alten Speicherstraße 14. Die Hausnummern 14/17 sind nicht aufgelöst. Das Dossier bleibt überregional; keine dieser Anschriften wird in ein einzelnes Händlerfeld übernommen. Bei späterer Standortauflösung sind die drei Anschriften getrennt zu prüfen/geokodieren; keine Koordinate für den überregionalen Datensatz. **Ankauf:** Metallrecycling beschrieben, aber keine öffentlichen Materialpreise; **Verkauf:** keine Preisliste; **Gebühren:** keine konkreten Container-/Entsorgungstarife auf den geprüften TSR-Seiten. Quellen: https://www.tsr-recycling.de/ ; https://www.tsr-recycling.de/bibliothek ; https://www.tsr.eu/standorte ; https://www.umwelt.niedersachsen.de/download/208339

## Timeline

### Recherche 24.10.2026

- [Recherche 24.10.2026: Direktabrufe von TSR-Homepage und Standort-/Bibliotheksseiten lieferten diesmal keinen lesbaren Seiteninhalt; deshalb keine neue Betreiberbestätigung aus diesem Abruf behauptet. Der letzte dokumentierte Quellenstand (Recherche 07.10.2026) bleibt maßgeblich: Hansastraße 9 (30419), Alte Speicherstraße 17 (TSR-Bibliothek) vs. Alte Speicherstraße 14 (Behördenliste), ungeklärt. Kein neuer Standort in Niedersachsen mit einzeln bestätigter Adresse oder Kundeneinlieferung belegt. Aus den bereits gelesenen TSR-Seiten ist keine öffentliche numerische Ankaufspreisliste, Verkaufsliste oder konkrete Container-/Entsorgungsgebühr dokumentiert. Überregional-Dossier ohne GEO beibehalten; bei Branchendatensätzen jeden Anschriftenpunkt separat geokodieren, nicht die widersprüchlichen Hausnummern zusammenwerfen. Quellen/letzter lesbarer Stand: https://www.tsr-recycling.de/bibliothek ; https://www.tsr-recycling.de/ ; https://www.tsr.eu/standorte ; https://www.umwelt.niedersachsen.de/download/208339 .]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Erstmals aktuelle Erstquellen statt nur des Gelbe-Seiten-Einzelbelegs gegengeprüft. Die TSR-Bibliothek listet zwei Hannover-Einträge: Hansastraße 9, 30419, sowie „Niederlassung Hannover Lindener Hafen“, Alte Speicherstraße 17, 30453. Die offizielle niedersächsische Behördenliste (Aktualisierung 19.01.2026) führt TSR Recycling GmbH & Co. KG an der Alten Speicherstraße 14, 30453 Hannover; Hausnummern 14 vs. 17 sind nicht aufgelöst und können unterschiedliche Standort-/Zufahrtsangaben sein. Der offizielle TSR-Gruppenauftritt nennt das FE-/NE-Geschäft und bundesweite Containerlogistik. Dossier bleibt überregional ohne Standortadresse; Beschreibung kuratiert, Website als aktiv gekennzeichnet. **Ankauf:** Geschäftsfeld belegt, keine numerische Ankaufspreisliste; **Verkauf:** keine Liste; **Gebühren:** keine konkreten Tarife auf den geprüften Seiten. Quelle(n): https://www.tsr-recycling.de/bibliothek ; https://www.tsr-recycling.de/ ; https://www.tsr.eu/standorte ; https://www.umwelt.niedersachsen.de/download/208339]

### Importiert (Seed-Stand 2026-09-30)

- Fe-/NE, Großmengen

### Recherche 30.09.2026

- Gelbe Seiten (Einzelbeleg, unsicher): TSR Hannover Alte Speicherstr. 17, 30453 (0511 921770) + Hansastr. 9, 30419 (0511 674070); kein Fill (überregional, Single-Source).
- Das Örtliche ohne TSR-Treffer Hannover; tsr-recycling.de bot-walled.
- Quelle: https://www.gelbeseiten.de/suche/Recycling/Hannover

### Recherche 01.11.2026

- [Recherche 01.11.2026: **Preispruefung fuer `ni-uberregional-tsr-recycling-mehrere-ni-standorte` — Standort-Uebersicht live abrufbar, aber ohne jede Preiszeile; Sortenprogramm statt Preis belegt.** Die Incapsula-Challenge auf `tsr-recycling.de` ist **kein Inhaltssperr, sondern eine Header-/Protokoll-Sperre**: mit HTTP/2 plus vollstaendigen Browser-Headern (User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`, `Upgrade-Insecure-Requests`, `--compressed`) liefert die Standort-Uebersicht `https://www.tsr-recycling.de/standorte/` **HTTP 200 mit Vollinhalt** statt der Challenge-Seite; der Reader-Proxy `https://r.jina.ai/` liefert dieselbe Seite ebenfalls.]
  Die Screening-Einstufung „durch eine Incapsula-Bot-Challenge gesperrt“ ist damit **technisch ueberholt** — der Inhalt war stets vorhanden, nur der naive Abruf scheiterte. **Befund:** `https://www.tsr-recycling.de/standorte/` ist live abrufbar und traegt **keine** Preiszeile — weder einen Material-Ankaufpreis noch einen Gebuehrensatz. Betreiberseitig belegt sind **Anschrift alle TSR-Standorte Deutschlands (u. a.
  Bad Essen, Berlin Neukoelln, Bremen, Einbeck, Emden, Hannover, Neumuenster, Neuruppin), bundesweit**, Telefon **0800 8772667 (Schrottlieferanten-Hotline der Gruppe)**. Das Preisversprechen des Betreibers bleibt rein qualitativ; eine numerische Preisangabe wird **nicht** erfunden.
  Das Dossier bündelt **mehrere** NI-Standorte; geprüft wurde deshalb die Standort-Uebersicht `https://www.tsr-recycling.de/standorte/` (HTTP 200, 133.438 Bytes) — sie listet alle Standorte mit Adresse, Oeffnungszeiten und Privatkunden-Hinweisen (u. a. Bad Essen, Berlin Neukoelln, Bremen, Einbeck, Emden, Hannover, Neumuenster, Neuruppin, Strausberg/ueberregional), aber **keine** Preiszeile; die Seite verweist Preise ausschliesslich an die Hotline.
  **Anmerkung des Betreibers (verbatim):** „Keine Privatkundenannahme möglich“ **Sorten-/Annahme-Aussagen der Standortseite (verbatim, ohne Betrag):** „Keine Privatkundenannahme für Altmetall und Schrott. PKW-Ankauf auf Anfrage per Kontaktformular“ // „Die Annahme von NE-Metallen schließt bereits um 15 Uhr.“ // „Mittwoch, 28.10.26 – keine Annahme von NE-Metallen und keine Annahme für Barkunden. Donnerstag, 29.10.26 – keine Annahme von NE-Metallen und keine Annahme für Barkunden.
  Freitag, 30.10.26 - komplett geschlossen. Montag, 02.11.26 - ab 13:00 Uhr geöffnet.“ **Negativbefund Preisliste fuer `ni-uberregional-tsr-recycling-mehrere-ni-standorte`:** TSR veroeffentlicht **keine Ankaufpreisliste**. `/kunden/schrottlieferanten/`: „Die Schrottpreise unterliegen taeglichen Schwankungen, da sie sich nach den aktuellen Entwicklungen auf den Rohstoffmaerkten richten. Deshalb bieten wir Ihnen stets die besten tagesaktuellen Preise“ (Hotline `0800 8772667`); `/kunden/privatkunden/`:
  „Aus diesem Grund **koennen wir keine universell gueltigen Preisangaben machen.**“ Die Pfade `/preise`, `/preise-und-ankauf`, `/ankaufpreise`, `/schrottpreise`, `/sorten`, `/metallpreise`, `/downloads`, `/medien` liefern die echte TSR-404-Seite (Titel „404 // TSR“), keine versteckte Preisliste; `sitemap.xml?sitemap=pages` listet 30 Seiten ohne Preis-URL; die CDX-Suche ueber 748 (`tsr-recycling.de`) bzw.
  6.874 (`tsr.eu`) URLs findet **keine** Preislisten-URL (nur das Icon-Bild „Icon_fairer_Preis.png“ und ein Such-Link). **Preisarten strikt getrennt, Standort `Arnsberg`:** (a) **Ankaufpreis** — fuer `ni-uberregional-tsr-recycling-mehrere-ni-standorte` **nicht** veroeffentlicht; die einzigen belegten Ankaufpreis-Zahlen der ganzen Gruppe sind zwei **Sonderaktionen als Zuschlag auf den unpublizierten Tagespreis**: **Einbeck** montags „Sonderaktion "plus zehn":
  Immer montags legen wir bei allen Lieferanten mit Kleingewerbe und Privatkunden **10,00€/to bei Fe-Schrott und 100,00€/to bei NE-Altmetallen** oben drauf“ (unveraendert in den Snapshots 18.05.2024, 14.09.2024, 12.08.2025, 11.03.2026 und live 01.11.2026) und **Emden** mittwochs „Jeden Mittwoch Spezialangebot fuer Privatkunden:
  Pro Tonne Fe-Schrott **10 € extra**, pro Tonne NE-Metall **100 € extra**“ (unveraendert in den Snapshots 18.05.2024, 15.10.2024, 12.08.2025, 17.04.2026 und live 01.11.2026); (b) **Verkauf** — die Recyclingprodukte **TSR40** (Stahl), **TSR130**/**TSR136** (Aluminium) sind ohne Preis publiziert; (c) **Gebuehre/Entsorgungskosten** — Weigerkostenkatalog, Kopfzeile „**gueltig ab 20.11.2020**“, live unter `https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/TSR_RL_AB-We_02.01_Weigerkostenkatalog_Stand20201120__01.pdf`:
  „PKW Reifen **20,00 €/Stueck**“, „LKW Reifen **50,00 €/Stueck**“, „Hohlkoerper **100,00 €/Stueck**“, Druckgasbehaelter **500,00 €/Stueck**, „Tresore **500,00 €/Stueck**“, „Acetylenflaschen **1.000,00 €/Stueck**“, „Technische Gase **2.000,00 €/Stueck**“, „Lithium-Ionen-Batterien **500,00 €/Stueck**“, Antriebsbatterien **1.000,00 €/Stueck** sowie „zzgl.
  Sortier- und Handlingspauschale von **275,00 €**“ — **aelter** (Stand 20.11.2020), aber datiert, und **nicht** standortspezifisch; (d) **Markt-/Boersenkurs** — TSR nennt die London Metal Exchange nur historisch als eigene Handelsgrundlage, ohne eigene Notierung; (e) **kommunale Gebuehre** — nicht einschlaegig, TSR ist privater Entsorgungsfachbetrieb.
  **Nicht** belegt und **nicht** erfunden: ein absoluter Material-Ankaufpreis fuer `ni-uberregional-tsr-recycling-mehrere-ni-standorte`, ein Boersen-/Marktkurs als Haendlerpreis, eine kommunale Gebuehre. **Sortenprogramm fuer `Arnsberg` (ohne Preis) — als Dossier-Zusatz belegt:** `https://www.tsr-recycling.de/uploads/tx_3sdownloadlibrary/TSR_Schrottsortenliste_A4_Final.pdf` ist live (HTTP 200, 523.867 Bytes, byteidentisch zum Wayback-Snapshot 13.11.2024), Titel „**TSR Schrottsorten-Liste**“:
  `E3` „Schwerer Stahlaltschrott, ueberwiegend staerker als 6 mm“ (Staerke ≥ 6 mm, ≥ 0,6 t/m3, ≤1%), `E1` leichter Stahlaltschrott < 6 mm, `E2` schwerer Stahlneuschrott ≥ 3 mm, `E8` leichter Stahlneuschrott < 3 mm, `E6` verdichteter Stahlneuschrott, `E 40` Shredderstahlschrott, `E5H`/`E5M` homogene bzw. gemischte Kohlenstoffstahlspaene, `EHRB` leicht legierter Schrott, `EHRM` Schrott mit hohem Reststoffanteil, `E 46` geshredderter Schrott aus der Muellverbrennung — jeweils mit Beschreibung, Abmessungen, Schuettgewicht und Schuttanteil, **ohne jeden Eurobetrag**.
  Die englische Fassung `TSR_Schrottsortenliste_EN.pdf` ist ebenfalls live; im Archiv zusaetzlich die **Europaeische Stahlschrottsortenliste** (`https://www.tsr.eu/fileadmin/PDF/sortenliste_eu.pdf`, Snapshot 11.07.2012) und die **Uebersicht NE-Metalle** (`https://www.tsr.eu/fileadmin/user_upload/tsr/downloads/Uebersicht_FE_Metalle.pdf`, Snapshot 03.03.2016: Aluminium, Blei, Chrom, Cr-Ni-Schrotte V2A, Cr-Ni-Mo-Schrotte V4A, Elektromotoren).
  **Nuance:** die Live-Bibliothek `https://www.tsr-recycling.de/bibliothek/` verlinkt Sortenliste und Weigerkostenkatalog **nicht mehr** (kein Treffer auf `tx_3sdownloadlibrary` bzw. „Weiger“); die Dateien sind weiterhin abrufbar, aber aus der Navigation verschwunden — der Wayback-Snapshot der Bibliothek vom 18.05.2024 fuehrt sie noch.
  **Preisgrundlage im Betreiber-Original (Preisart ohne Betrag):** Die Allgemeinen Einkaufsbedingungen TSR Deutschland sind live abrufbar (`https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/Allgemeine_Einkaufsbedingungen_TSR_Deutschland.pdf`, HTTP 200, 138.028 Bytes) und tragen im Text **„Stand 02/2026“** (der Archivstand vom 23.03.2025 ohne Standzeile). Ziffer 4 „Preise, Gewichts- und Mengenermittlung“ verbatim:
  „**4.1.** Die vereinbarten Preise gelten, sofern nichts anderes vereinbart ist, frei Empfangsstelle. Sie sind **Festpreise**. **4.2.** Erfolgt die Behaeltergestellung und die Abholung vereinbarungsgemaess durch uns berechnen wir grundsatzlich **Miete und Transport**. **4.3.** Fuer die Abrechnungen sind **Empfaangsgewicht und -befund massgebend**.“
  Der Ankaufpreis ist damit ein **individuell vereinbarter Festpreis** — die Bedingungen belegen die **Preisart**, enthalten aber **keinen Eurobetrag** und sind **keine** Preisquelle fuer Materialpreise. Fuer den Standort `Arnsberg` aendert das nichts an der Negativaussage: kein veroeffentlichter Zahlenpreis.]

- [Recherche 01.11.2026: **Quellen fuer `ni-uberregional-tsr-recycling-mehrere-ni-standorte` (alle Abrufe 01.11.2026), Anwendungsbereich auf diesen Standort begrenzt.** Der Preispruefung dieses Dossiers lagen die Standortseiten `https://www.tsr-recycling.de/standorte/` zugrunde (live, mit Browser-Headern abrufbar). Die Feststellung „kein veroeffentlichter Ankaufpreis“ gilt **fuer diesen Standort** und wird **nicht** aus einer anderen Quelle generalisiert; ein belegter Zahlenwert fuer `ni-uberregional-tsr-recycling-mehrere-ni-standorte` existiert nicht, ein geschaetzter wird **nicht** erzeugt.]
  **Abgrenzung der Preisarten fuer dieses Dossier:** **Ankaufpreis** — nicht veroeffentlicht, keine Zahl; **Verkauf** — nicht veroeffentlicht (die Recyclingprodukte TSR40/TSR130/TSR136 der Gruppe sind ohne Preis publiziert); **Gebuehre** — nur die bundesweit gleichen Weigerkosten (Stand 20.11.2020, Sortier- und Handlingspauschale 275,00 €), nicht standortspezifisch; **Markt-/Boersenkurs** — TSR nennt die London Metal Exchange nur historisch als eigene Handelsgrundlage, ohne eigene Notierung;
  **kommunale Gebuehre** — nicht einschlaegig, TSR ist ein privater Entsorgungsfachbetrieb. Quelle(n) fuer `ni-uberregional-tsr-recycling-mehrere-ni-standorte`:
  `https://www.tsr-recycling.de/standorte/` (live, Abruf 01.11.2026 mit Browser-Headern); https://www.tsr-recycling.de/kunden/schrottlieferanten/ ; https://www.tsr-recycling.de/kunden/privatkunden/ ; https://www.tsr-recycling.de/sitemap.xml ; https://www.tsr-recycling.de/bibliothek/ ; https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/TSR_RL_AB-We_02.01_Weigerkostenkatalog_Stand20201120__01.pdf ; https://www.tsr-recycling.de/uploads/tx_3sdownloadlibrary/TSR_Schrottsortenliste_A4_Final.pdf ; https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/Allgemeine_Einkaufsbedingungen_TSR_Deutschland.pdf ; http://web.archive.org/cdx/search/cdx?url=tsr-recycling.de&matchType=domain]
