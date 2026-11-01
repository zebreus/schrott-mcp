---
slug: th-gera-scholz-recycling-standort-gera
name: Scholz Recycling GmbH (Standort Gera)
trader_type: sonstige
state: TH
city: Gera
street: Gessentalstr. 5a
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.scholz-recycling.com/
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 3 (2026-09-27)
provenance_ankauf_raw: ja (Annahme Konzernstandard)
provenance_origin: table
---

# Scholz Recycling GmbH (Standort Gera)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- wie oben

### Recherche 01.11.2026

- [Recherche 01.11.2026: **Preisquelle fuer Gera: keine.** Die Standortseite des Betreibers]
  (https://www.scholz-recycling.com/standort/gera/) wurde am 01.11.2026 mit vollstaendigen Browser-Headern
  abgerufen (HTTP/2, User-Agent Chrome 131, `Accept`, `Accept-Language`, `Sec-Fetch-Dest/Mode/Site/User`,
  `Upgrade-Insecure-Requests`, `--compressed`, Cookie-Zustimmung `cookie_consent=yes; borlabs-cookie-consent=all`)
  und liefert HTTP 200 mit dem Titel "Gera &#8211; Scholz Recycling"; im gerenderten Text und im Roh-HTML stehen
  **keine** Zahlenwerte mit Einheit, kein Eurozeichen, kein "Ankaufspreis" und keine "Preisliste". Der
  Standort-JSON-Feed des Betreibers (live https://www.scholz-recycling.com/standorte-json-nicht-loeschen/) weist
  fuer diesen Betrieb "Scholz Recycling GmbH", "Gessentalstr. 5a", 07551 Gera, Telefon "+49 365 32002" und E-Mail
  "gera@scholz-recycling.de" aus und fuehrt 3 Nachweise (241228_ISO-50001_SRG_de_en_2024.pdf,
  251028_UeZEFB_GER.pdf, 251014_Zertifikat_VO-333_GER_.pdf) - **kein** Preisfeld. Der Sorten- und Annahmeumfang
  ist im Konzern-Bereich "Geschaeftsbereiche" (Stahlschrotte, NE-Metalle, Legierte Stahlschrotte, Deklassierte
  Stahlprodukte, Sonderlegierungen, Wertstoffe) und in "Services" beschrieben, aber **ohne** Zahlenwerte. Dort
  steht woertlich "Die Abrechnung erfolgt nach Qualitaet bzw. Klassifizierung und Gewicht, transparent und fair
  auf Basis der tagesaktuellen Marktpreise" und "Die Abrechnung erfolgt transparent und fair auf Basis der
  tagesaktuellen Marktpreise, die mithilfe der branchenueblichen Preisindizes wie z.B. BDSV oder LME ermittelt
  werden" - das ist die Preismethodik, kein Ankaufspreis. **Preisarten getrennt:** ANKAUF - **kein belegbarer
  Ankaufspreis** fuer diesen Standort, weder eine Preisliste mit Wert und Einheit noch eine Sortenliste mit
  Preisspalte; VERKAUF - keine Verkaufspreisliste, keine Vormaterialangebote; GEBUEHREN - keine Tarif- oder
  Gebuehrentafel fuer Containerdienst, Waage oder Annahme; MARKT- UND BOERSENKURS - die BDSV-Durchschnittspreise
  (live https://www.bdsv.org/unser-service/markt-preise/, Stand 09/2026: Sorte 1 = 254,6 EUR/t, Sorte 2/8 = 294,6
  EUR/t, Sorte 3 = 289,0 EUR/t, Sorte 4 = 302,9 EUR/t, Sorte 5 = 246,5 EUR/t) sind **Lagerverkaufspreise** der
  BDSV-Mitgliedsunternehmen und damit **kein** Haendler-Ankaufspreis; KOMMUNALE GEBUEHR - fuer diesen Standort
  nicht einschlaegig, es wird keine kommunale Gebuehr ausgewiesen. **Negativbefunde je Weg:** (1) die Startseite
  https://www.scholz-recycling.com/ antwortet auch mit vollstaendigen Browser-Headern mit **HTTP 503** (Edge-Block
  "The page is temporarily unavailable"); das ist **keine** Bot-Challenge wie bei TSR, denn die Inhalte sind ueber
  https://r.jina.ai/https://www.scholz-recycling.com/ vollstaendig abrufbar (HTTP 200, 18.726 Bytes) und alle
  Unterseiten liefern HTTP 200; (2) `robots.txt` (HTTP 200) verweist auf `wp-sitemap.xml`, dessen Index
  (posts-post, posts-page, posts-standort, taxonomies-category, users) **keine** Preis-URL enthaelt; (3) alle
  **185** Standortseiten des Betreibers wurden einzeln abgerufen, **null** davon traegt einen Preis-Marker; (4)
  die REST-API (294 Routen, `x-wp-total` posts 39 / pages 20 / media 408) fuehrt **keinen** Post-Type `preise`,
  und die Pfade /preise/, /ankaufpreise/, /schrottpreise/, /sorten/, /sortenprogramm/, /metallpreise/,
  /downloads/, /medien/, /leistungen/, /lieferanten/, /einkauf/, /kunden/, /kunden/schrottlieferanten/,
  /kunden/privatkunden/ und /einkaufsbedingungen/ liefern HTTP 500 (WordPress-404); die einzigen Pfade mit HTTP
  200 sind /service/ (Weiterleitung auf /services/, ohne Preise), /verkauf/ (Weiterleitung auf die
  Derichebourg-Verkaufsmeldung, ohne Preise) und /veroeffentlichungen/ (Weiterleitung auf
  /veroeffentlichungen/cbcr-reporting/, CbCR-Transparenz, ohne Preise), waehrend die Mediathek-Suche nach
  `preise`, `preisliste`, `ankauf`, `entgelt`, `vergütung`, `tarife` und `sorten` jeweils **null** Treffer
  liefert; (5) die Einkaufsbedingungen (AGB_EK_SRG_final_01.09.2026_DE.pdf und
  Allg_Vertragsbedingungen_Indirekter_Einkauf_09.10.2025_DE.pdf) nennen nur Zahlungsziele (30 Kalendertage netto)
  und Vertragsstrafen (0,2 % des Preises je Werktag, hoechstens 5 %) - **keine** Preise. **Wayback Machine, vom
  Screening nicht geprueft und hier erstmals geprueft:**
  `http://archive.org/wayback/available?url=scholz-recycling.com&timestamp=20261001` liefert den Snapshot
  20261009123137 (HTTP 200, ohne Preiszeile); die CDX-Liste
  `http://web.archive.org/cdx/search/cdx?url=scholz-recycling.com*&output=text&limit=800&collapse=urlkey` (1.807
  URL-Keys) und die Domain-Sicht auf scholz-recycling.de, scholz-ag.de und recy-online.scholz-group.com enthalten
  **keine** Preislisten-URL. Der einzige Preis-Kandidat der alten TYPO3-Site,
  `/fileadmin/PDF/europaeische_schrottsortenliste_deutsch.pdf` (Snapshot 20090509232626, live 29.982 Bytes), ist
  eine reine **Sortenliste ohne Preise** (E1/E2/E3/E8/E40/E5 mit Abmessungen und Schuttanteil, kein Eurobetrag) -
  also ein Sortenprogramm, kein Preis. Der Wayback-Snapshot der Standortseite ist fuer die Sichtung mitgeprueft:
  gera (14 Snapshots, letzter 20260517091337) - in **allen** Faellen dieselbe JS-Platzhalter-Seite ohne
  Zahlenwert, Eurozeichen oder "Preisliste". **Schwesterbetriebe und Gruppen-Domains:** `eko-schrottrecycling.de`,
  `studo-schrott.de`, `srt-schrott-recycling.de` und `scholz-industrie.de` sind **DNS-NXDOMAIN** und haben also
  keine eigene Preisliste; `scholz-recycling.de` und `scholz-group.com` loesen zwar auf 89.31.143.90 auf, der
  Verbindungsversuch laeuft aber in einen Timeout (keine Website);
  `recy-online.scholz-group.com:4437/RecyOnline/Login/` ist das EDI-Portal **login-geschuetzt** (Benutzername /
  Passwort) und damit fuer Dritte keine Preisquelle; die live erreichbaren Schwester `srw-recycling.de` (SRW
  metalfloat, 24 Sitemap-URLs) und `fegert-recycling.de` (Fegert, 5 Standorte) veroeffentlichen **keine** Preise.
  **Als Kandidat geprueft und verworfen:** der Marktplatz-Auftritt
  `https://www.wlw.de/de/showroom/scholz-recycling-aktuelle-schrottpreise` fuehrt unter der Ueberschrift "Arten
  von Scholz Recycling aktuelle Schrottpreise" Zahlenpaare wie "Kupferschrott ... 6,00 - 7,50 EUR",
  "Aluminiumschrott ... 1,80 - 2,40 EUR" und "Stahlschrott ... 0,15 - 0,30 EUR" je kg. Das ist **keine**
  Preisquelle der Scholz Recycling GmbH: die Seite ist eine SEO-Keyphrase-Seite mit automatisch erzeugten
  Abschnitten ("How to Choose scholz recycling aktuelle schrottpreise", "Wartungsanweisungen fuer scholz recycling
  aktuelle schrottpreise"), die angebotenen Produkte stammen von **anderen** Lieferanten (Mr.Schrott, Stuhlmann &
  Redzic, H. Carnuth KG), Scholz Recycling GmbH selbst hat dort kein Lieferantenprofil
  (`/de/companies/scholz-recycling-gmbh` = HTTP 404), die Werte stehen ohne Datum und mit "ca." - kein datierter,
  standortbezogener Betreiberpreis. Quelle(n): https://www.scholz-recycling.com/services/ ;
  https://www.scholz-recycling.com/geschaeftsbereiche/ ;
  https://www.scholz-recycling.com/standorte-json-nicht-loeschen/ ;
  https://www.scholz-recycling.com/wp-json/wp/v2/types ; https://www.scholz-recycling.com/wp-json/wp/v2/media ;
  https://www.scholz-recycling.com/wp-sitemap.xml ; https://www.scholz-recycling.com/robots.txt ;
  http://web.archive.org/cdx/search/cdx?url=scholz-recycling.com*&output=text&limit=800&collapse=urlkey ;
  https://www.bdsv.org/unser-service/markt-preise/ (Abruf 01.11.2026, 12:00-Welle)]
