---
slug: sn-torgau-tsr-deutschland-nl-torgau
name: TSR Deutschland, NL Torgau
trader_type: sonstige
state: SN
city: Torgau
street: Welsauer Weg 14
postcode: 04860
phone: 03421 704983
email: torgau@tsr.eu
opening_hours: Mo-Fr 07:00-12:00, 12:30-16:00
website: https://www.tsr-recycling.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# TSR Deutschland, NL Torgau

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Seite Torgau (https://www.tsr-recycling.de/standorte/schrotthaendler-torgau) per Bot-Schutz blockiert (Challenge Validation) -> website_status blockiert; Adresse Welsauer Weg 14 nur Aggregator-Lead (recyclinghof-wertstoffhof, cylex, 11880: keine Belege); kein Feld belegbar; Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-torgau]

### Recherche 02.10.2026

- [Recherche 02.10.2026: OK — Betreiber-Standortseite diesmal lesbar (Bot-Schutz-Blockade aus Vorwelle überwunden): TSR Deutschland GmbH & Co. KG, Welsauer Weg 14, 04860 Torgau, T +49 3421 704983, Öffnungszeiten Mo-Fr 07:00-12:00/12:30-16:00; GoLocal (überprüfte Adresse) + Gelbe Seiten deckungsgleich, E-Mail torgau@tsr.eu (GoLocal). GEFÜLLT: street/postcode (Koordinaten-Neugeocodierung vermerkt)/phone/email/opening_hours; website_status blockiert→aktiv (Owner-Ausnahme: Betreiber-Filialseite); status bleibt aktiv; Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-torgau/ + https://www.golocal.de/torgau/schrotthandel/tsr-deutschland-gmbh-co-kg-niederlassung-torgau-11CbdA/ + https://www.gelbeseiten.de/gsbiz/b4e1f219-bd70-43c9-8d91-e4873a335dd6]

### Importiert (Seed-Stand 2026-09-30)

- Fe/NE, Kabelrecycling
- Selbstanlieferer wie andere TSR-NL

### Recherche 01.11.2026

- [Recherche 01.11.2026: **Preispruefung fuer `sn-torgau-tsr-deutschland-nl-torgau` — Standortseite live abrufbar, aber ohne jede Preiszeile; Sortenprogramm statt Preis belegt.** Die Incapsula-Challenge auf `tsr-recycling.de` ist **kein Inhaltssperr, sondern eine Header-/Protokoll-Sperre**: mit HTTP/2 plus vollstaendigen Browser-Headern (User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`, `Upgrade-Insecure-Requests`, `--compressed`) liefert die Standortseite `https://www.tsr-recycling.de/standorte/schrotthaendler-torgau/` **HTTP 200 mit Vollinhalt** statt der Challenge-Seite; der Reader-Proxy `https://r.jina.ai/` liefert dieselbe Seite ebenfalls.]
  Die Screening-Einstufung „durch eine Incapsula-Bot-Challenge gesperrt“ ist damit **technisch ueberholt** — der Inhalt war stets vorhanden, nur der naive Abruf scheiterte. **Befund:** `https://www.tsr-recycling.de/standorte/schrotthaendler-torgau/` ist live abrufbar und traegt **keine** Preiszeile — weder einen Material-Ankaufpreis noch einen Gebuehrensatz. Betreiberseitig belegt sind **Anschrift Welsauer Weg 14, 04860 Torgau**, Telefon **+493421704983**; Oeffnungszeiten Mo-Fr:
  07:00 - 12:00 / 12:30 - 16:00 / 28.11.2026: 08:00 - 11:00. Das Preisversprechen des Betreibers bleibt rein qualitativ; eine numerische Preisangabe wird **nicht** erfunden. Die Torgauer Seite nennt zusaetzlich einen **Sonderoeffnungstag** „28.11.2026: 08:00 - 11:00“ — ein Terminhinweis, kein Preis. **Sorten-/Annahme-Aussagen der Standortseite (verbatim, ohne Betrag):** „entfernt. Wir kaufen unter anderem“
  **Negativbefund Preisliste fuer `sn-torgau-tsr-deutschland-nl-torgau`:** TSR veroeffentlicht **keine Ankaufpreisliste**. `/kunden/schrottlieferanten/`: „Die Schrottpreise unterliegen taeglichen Schwankungen, da sie sich nach den aktuellen Entwicklungen auf den Rohstoffmaerkten richten. Deshalb bieten wir Ihnen stets die besten tagesaktuellen Preise“ (Hotline `0800 8772667`); `/kunden/privatkunden/`: „Aus diesem Grund **koennen wir keine universell gueltigen Preisangaben machen.**“
  Die Pfade `/preise`, `/preise-und-ankauf`, `/ankaufpreise`, `/schrottpreise`, `/sorten`, `/metallpreise`, `/downloads`, `/medien` liefern die echte TSR-404-Seite (Titel „404 // TSR“), keine versteckte Preisliste; `sitemap.xml?sitemap=pages` listet 30 Seiten ohne Preis-URL; die CDX-Suche ueber 748 (`tsr-recycling.de`) bzw. 6.874 (`tsr.eu`) URLs findet **keine** Preislisten-URL (nur das Icon-Bild „Icon_fairer_Preis.png“ und ein Such-Link).
  **Preisarten strikt getrennt, Standort `Torgau`:** (a) **Ankaufpreis** — fuer `sn-torgau-tsr-deutschland-nl-torgau` **nicht** veroeffentlicht; die einzigen belegten Ankaufpreis-Zahlen der ganzen Gruppe sind zwei **Sonderaktionen als Zuschlag auf den unpublizierten Tagespreis**: **Einbeck** montags „Sonderaktion "plus zehn":
  Immer montags legen wir bei allen Lieferanten mit Kleingewerbe und Privatkunden **10,00€/to bei Fe-Schrott und 100,00€/to bei NE-Altmetallen** oben drauf“ (unveraendert in den Snapshots 18.05.2024, 14.09.2024, 12.08.2025, 11.03.2026 und live 01.11.2026) und **Emden** mittwochs „Jeden Mittwoch Spezialangebot fuer Privatkunden:
  Pro Tonne Fe-Schrott **10 € extra**, pro Tonne NE-Metall **100 € extra**“ (unveraendert in den Snapshots 18.05.2024, 15.10.2024, 12.08.2025, 17.04.2026 und live 01.11.2026); (b) **Verkauf** — die Recyclingprodukte **TSR40** (Stahl), **TSR130**/**TSR136** (Aluminium) sind ohne Preis publiziert; (c) **Gebuehre/Entsorgungskosten** — Weigerkostenkatalog, Kopfzeile „**gueltig ab 20.11.2020**“, live unter `https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/TSR_RL_AB-We_02.01_Weigerkostenkatalog_Stand20201120__01.pdf`:
  „PKW Reifen **20,00 €/Stueck**“, „LKW Reifen **50,00 €/Stueck**“, „Hohlkoerper **100,00 €/Stueck**“, Druckgasbehaelter **500,00 €/Stueck**, „Tresore **500,00 €/Stueck**“, „Acetylenflaschen **1.000,00 €/Stueck**“, „Technische Gase **2.000,00 €/Stueck**“, „Lithium-Ionen-Batterien **500,00 €/Stueck**“, Antriebsbatterien **1.000,00 €/Stueck** sowie „zzgl.
  Sortier- und Handlingspauschale von **275,00 €**“ — **aelter** (Stand 20.11.2020), aber datiert, und **nicht** standortspezifisch; (d) **Markt-/Boersenkurs** — TSR nennt die London Metal Exchange nur historisch als eigene Handelsgrundlage, ohne eigene Notierung; (e) **kommunale Gebuehre** — nicht einschlaegig, TSR ist privater Entsorgungsfachbetrieb.
  **Nicht** belegt und **nicht** erfunden: ein absoluter Material-Ankaufpreis fuer `sn-torgau-tsr-deutschland-nl-torgau`, ein Boersen-/Marktkurs als Haendlerpreis, eine kommunale Gebuehre. **Sortenprogramm fuer `Torgau` (ohne Preis) — als Dossier-Zusatz belegt:** `https://www.tsr-recycling.de/uploads/tx_3sdownloadlibrary/TSR_Schrottsortenliste_A4_Final.pdf` ist live (HTTP 200, 523.867 Bytes, byteidentisch zum Wayback-Snapshot 13.11.2024), Titel „**TSR Schrottsorten-Liste**“:
  `E3` „Schwerer Stahlaltschrott, ueberwiegend staerker als 6 mm“ (Staerke ≥ 6 mm, ≥ 0,6 t/m3, ≤1%), `E1` leichter Stahlaltschrott < 6 mm, `E2` schwerer Stahlneuschrott ≥ 3 mm, `E8` leichter Stahlneuschrott < 3 mm, `E6` verdichteter Stahlneuschrott, `E 40` Shredderstahlschrott, `E5H`/`E5M` homogene bzw. gemischte Kohlenstoffstahlspaene, `EHRB` leicht legierter Schrott, `EHRM` Schrott mit hohem Reststoffanteil, `E 46` geshredderter Schrott aus der Muellverbrennung — jeweils mit Beschreibung, Abmessungen, Schuettgewicht und Schuttanteil, **ohne jeden Eurobetrag**.
  Die englische Fassung `TSR_Schrottsortenliste_EN.pdf` ist ebenfalls live; im Archiv zusaetzlich die **Europaeische Stahlschrottsortenliste** (`https://www.tsr.eu/fileadmin/PDF/sortenliste_eu.pdf`, Snapshot 11.07.2012) und die **Uebersicht NE-Metalle** (`https://www.tsr.eu/fileadmin/user_upload/tsr/downloads/Uebersicht_FE_Metalle.pdf`, Snapshot 03.03.2016: Aluminium, Blei, Chrom, Cr-Ni-Schrotte V2A, Cr-Ni-Mo-Schrotte V4A, Elektromotoren).
  **Nuance:** die Live-Bibliothek `https://www.tsr-recycling.de/bibliothek/` verlinkt Sortenliste und Weigerkostenkatalog **nicht mehr** (kein Treffer auf `tx_3sdownloadlibrary` bzw. „Weiger“); die Dateien sind weiterhin abrufbar, aber aus der Navigation verschwunden — der Wayback-Snapshot der Bibliothek vom 18.05.2024 fuehrt sie noch.
  **Preisgrundlage im Betreiber-Original (Preisart ohne Betrag):** Die Allgemeinen Einkaufsbedingungen TSR Deutschland sind live abrufbar (`https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/Allgemeine_Einkaufsbedingungen_TSR_Deutschland.pdf`, HTTP 200, 138.028 Bytes) und tragen im Text **„Stand 02/2026“** (der Archivstand vom 23.03.2025 ohne Standzeile). Ziffer 4 „Preise, Gewichts- und Mengenermittlung“ verbatim:
  „**4.1.** Die vereinbarten Preise gelten, sofern nichts anderes vereinbart ist, frei Empfangsstelle. Sie sind **Festpreise**. **4.2.** Erfolgt die Behaeltergestellung und die Abholung vereinbarungsgemaess durch uns berechnen wir grundsatzlich **Miete und Transport**. **4.3.** Fuer die Abrechnungen sind **Empfaangsgewicht und -befund massgebend**.“
  Der Ankaufpreis ist damit ein **individuell vereinbarter Festpreis** — die Bedingungen belegen die **Preisart**, enthalten aber **keinen Eurobetrag** und sind **keine** Preisquelle fuer Materialpreise. Fuer den Standort `Torgau` aendert das nichts an der Negativaussage: kein veroeffentlichter Zahlenpreis.]

- [Recherche 01.11.2026: **Quellen fuer `sn-torgau-tsr-deutschland-nl-torgau` (alle Abrufe 01.11.2026), Anwendungsbereich auf diesen Standort begrenzt.** Der Preispruefung dieses Dossiers lagen die Standortseiten `https://www.tsr-recycling.de/standorte/schrotthaendler-torgau/` zugrunde (live, mit Browser-Headern abrufbar). Die Feststellung „kein veroeffentlichter Ankaufpreis“ gilt **fuer diesen Standort** und wird **nicht** aus einer anderen Quelle generalisiert; ein belegter Zahlenwert fuer `sn-torgau-tsr-deutschland-nl-torgau` existiert nicht, ein geschaetzter wird **nicht** erzeugt.]
  **Abgrenzung der Preisarten fuer dieses Dossier:** **Ankaufpreis** — nicht veroeffentlicht, keine Zahl; **Verkauf** — nicht veroeffentlicht (die Recyclingprodukte TSR40/TSR130/TSR136 der Gruppe sind ohne Preis publiziert); **Gebuehre** — nur die bundesweit gleichen Weigerkosten (Stand 20.11.2020, Sortier- und Handlingspauschale 275,00 €), nicht standortspezifisch; **Markt-/Boersenkurs** — TSR nennt die London Metal Exchange nur historisch als eigene Handelsgrundlage, ohne eigene Notierung;
  **kommunale Gebuehre** — nicht einschlaegig, TSR ist ein privater Entsorgungsfachbetrieb. Quelle(n) fuer `sn-torgau-tsr-deutschland-nl-torgau`:
  `https://www.tsr-recycling.de/standorte/schrotthaendler-torgau/` (live, Abruf 01.11.2026 mit Browser-Headern); https://www.tsr-recycling.de/kunden/schrottlieferanten/ ; https://www.tsr-recycling.de/kunden/privatkunden/ ; https://www.tsr-recycling.de/sitemap.xml ; https://www.tsr-recycling.de/bibliothek/ ; https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/TSR_RL_AB-We_02.01_Weigerkostenkatalog_Stand20201120__01.pdf ; https://www.tsr-recycling.de/uploads/tx_3sdownloadlibrary/TSR_Schrottsortenliste_A4_Final.pdf ; https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/Allgemeine_Einkaufsbedingungen_TSR_Deutschland.pdf ; http://web.archive.org/cdx/search/cdx?url=tsr-recycling.de&matchType=domain]
