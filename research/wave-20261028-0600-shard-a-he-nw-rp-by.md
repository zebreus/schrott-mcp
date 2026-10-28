# 6h-Welle 28.10.2026 06:00 UTC, Shard A (HE, NW, RP, BY) — 20 Dossiers

Recherche ausschließlich lesend; Produktionsdatenbank nur read-only
(`file:…public.db?mode=ro`, `PRAGMA query_only=ON`). Kein Commit, kein Push, kein Deploy.
Jedes der 20 Dossiers trägt genau eine Notiz vom 28.10.2026 im Format
`[Recherche 28.10.2026: …; Quelle(n): …]`; bestehende Timeline-Notizen wurden nicht
angefasst, Slugs blieben unverändert.

## Hebel

1. **Amtliches Fachbetrieberegister** (Fachbetrieberegister der Gemeinsamen Stelle
   Altfahrzeuge § 7 Abs. 2a AltfahrzeugV + Register der Entsorgungsfachbetriebe § 56 KrWG,
   Auszug 11.08.2026, gespiegelt über `schrottregister.pages.dev`). Der Spiegel weist sich
   selbst als „Unabhängiges Verzeichnis · kein amtliches Angebot“ aus und wird als
   Registerquelle geführt, nicht als Aggregator. **Treffer:** Bachinger (ZZHT01000289),
   Peter Huber Geretsried (ZAHS01800066), Stirling Neustadt (ZZST00200142),
   Demand-Recycling Landau (ZZGT00200091 + ZAGS00300012).
   **Negativbefunde ohne Beweiswert** (EFB-Anerkennung ist freiwillig): Bad Vilbel,
   Eschborn, Hanau, Schaafheim, Bergisch Gladbach, Bottrop, Porta Westfalica, Wuppertal,
   Koblenz sowie die Orte ohne Ortsseite Limeshain, Reinheim, Bruchköbel, Groß-Umstadt,
   Landstuhl, Kaltenengers, Obergünzburg.
2. **Amtliches Handelsregister** (`online-handelsregister.de`, Suche und Einzelabzug):
   Stirling-Industrie Rohstoff & Recycling GmbH, AG Fürth, HRB 9487, **aktiv**,
   Firmenadresse „Industriegebiet Diebach 67, 91413 Neustadt a.d. Aisch“;
   Autoverwertung Demand - Recycling e.K., AG Landau, **HRA 2369**. Alle übrigen
   Suchen negativ — bei Einzelunternehmern ohne HRB ohne Beweiswert.
3. **Betreiber-Primärquelle via Web-Archiv**, wo die Live-Domain blockiert, inaktiv oder
   tot ist: `korkmaz-gmbh.com` (10.12.2021), `stirling-metallhandel.de` (Impressum
   01.02.2023, Startseite 13.10.2025, An- und Verkauf 01.02.2023),
   `containerdienst-reining.com` (02.12.2021, leer), `bernhardt-hanau.de` (11.04.2026,
   STRATO-Parkseite), `schrottschneider.de` (08.08.2018, „Web Site Currently Not Available“).
4. **VIES** (`ec.europa.eu/taxation_customs/vies`): DE236814509 (Kollosche),
   DE128606493 (Bachinger), DE182230887 (Huber), DE813786386 (Stirling) jeweils gültige
   Umsatzsteuer-Registrierung. Für DE liefert VIES **keinen** Namen und **keine**
   Anschrift, ist also kein Identitätsbeleg.

## Ergebnis in Zahlen

| Kennzahl | Wert |
|---|---|
| Bearbeitete Dossiers | 20 (alle mit Notiz vom 28.10.2026) |
| Frontmatter geändert | 8 (6 mit vollständiger Füllung, 2 mit Teilfüllung/Status) |
| `status` → `aktiv` | 4 (Bachinger, Peter Huber, Stirling, Demand-Recycling) |
| `status` → `geschlossen` | 1 (Gebrüder Dörner) |
| `trader_type` → `schrotthaendler` | 1 (Bachinger) |
| `certifications` gesetzt | 4 (6 Zertifikatsnummern) |
| Neue Koordinaten | 0 (keine gesetzt) |
| `clear_fields`-Direktiven | 0 |
| Belegte Zahlenpreislisten (Ankauf) | **0** |
| Belegte Gebührentarife | **0 neu** (2 bereits bekannte Angaben neu bestätigt) |

## Frontmatter-Änderungen im Einzelnen

- **`by-biessenhofen-bachinger`** — Straße, PLZ, Telefon, E-Mail, Öffnungszeiten,
  Website, `website_status: aktiv`, `status: aktiv`, `trader_type: schrotthaendler`,
  Beschreibung, `certifications` ZZHT01000289.
- **`by-geretsried-autoverwertung-peter-huber`** — Straße, PLZ, Telefon, E-Mail,
  Öffnungszeiten, `status: aktiv`, Beschreibung, `certifications` ZAHS01800066.
- **`by-neustadt-a-d-aisch-stirling-industrie-metallhandel`** — Straße, PLZ, Telefon,
  E-Mail, Öffnungszeiten (Büro/Anlieferung getrennt), `status: aktiv`, Beschreibung,
  `certifications` ZZST00200142.
- **`rp-landau-autoverwertung-demand-recycling`** — Straße, PLZ, Telefon, E-Mail,
  Öffnungszeiten (saisonal), `status: aktiv`, Beschreibung, zwei `certifications`
  (ZZGT00200091, ZAGS00300012).
- **`rp-koblenz-pms-schrotthandel`** — Straße, PLZ, Telefon, E-Mail, Beschreibung
  (Belegklärfall vom 05.10.2026 geschlossen).
- **`he-reinheim-alteisensammlung-korkmaz`** — Straße, PLZ (Anschrift jetzt
  betreiberbelegt statt Verzeichnislead).
- **`nw-wuppertal-elberfeld-gebruder-dorner`** — `status: geschlossen`, Beschreibung
  (Gesellschaft aufgelöst / Firma erloschen 07.02.2022).
- **`he-bad-vilbel-61118-nagel`** — `website_status: unbekannt`.

## Preise und Gebühren

In **keinem** der 20 Dossiers wurde eine Zahlenpreisliste für Ankauf, Verkauf oder
Gebühren gefunden. Belegt sind nur qualitative Aussagen („tagesaktuelle Preise“,
„faire Preise“, „Sortenreinheit entscheidet über den Ertrag“). Neu bestätigt wurden
zwei bereits dokumentierte Betreiberangaben: **Prämie 50–2.000 EUR je Altfahrzeug**
und **Müllsäcke 5 EUR/Stück** bei Demand-Recycling Landau (dort auch: Abholung im
50-km-Radius nach Termin, für Landau/SÜW kostenlos, Abmeldegebühr unbeziffert).
Preise Dritter (Wertstoffhof-Tarife der ZAW Darmstadt-Dieburg, Shop-Verkaufspreise von
BeKo) wurden **nicht** übernommen.

## Offene Owner-Punkte

1. **Namensangleichung** `by-neustadt-a-d-aisch-stirling-industrie-metallhandel`:
   amtliche Firmierung „Stirling-Industrie Rohstoff & Recycling GmbH“.
2. **Namens-/Ortsfrage** `by-biessenhofen-bachinger`: Betreiber und Register nennen
   „Bachinger Schrott & Metalle“ in **Altdorf**, das Dossier führt „Bachinger“ /
   Biessenhofen. `city` blieb unverändert.
3. **Namensfrage** `he-reinheim-alteisensammlung-korkmaz`: Betreiber firmiert als
   „Korkmaz Entsorgungsfachbetrieb“.
4. **Schreibweise** `rp-landau-autoverwertung-demand-recycling`: HR „Demand - Recycling“
   mit Leerzeichen.
5. **Nachfolger-Kette** `he-schaafheim-64850-he-reining-klaus-containerdienst`:
   Klaus Reining → Salih Gencsoy („Containerdienst Schaafheim“, Ringstraße 7, seit
   Mai 2020). Für den Nachfolger existiert **kein** eigenes Dossier.
6. **Merge-Kandidaten** (3 Dossiers, derselbe Reining-Betrieb):
   `he-schaafheim-64850-he-reining-klaus-containerdienst`,
   `he-schaafheim-64850-he-reining-klaus`, `he-schaafheim-64850-reining-klaus-containerdienst`.
   Im selben Cluster außerdem Zwillingspaare
   `he-schaafheim-64850-he-sell-oskar-schrotthdl` / `he-schaafheim-64850-sell-oskar-schrotthdl`
   und `he-schaafheim-64850-he-containerdienst-sell` / `he-schaafheim-64850-containerdienst-sell`.
7. **Namensvetter ohne Dossier**: zweiter „Adrijan Handel“ in Bottrop
   (Heimannstr. 104, 46240 Bottrop-Boy) — vom Dossier `nw-bottrop-adrijan` klar
   getrennt, aber ohne eigene Zeile.
8. **Dossier-Lücke**: Gerhard Höfling GmbH, Eichenweg 101, 64850 Schaafheim
   (HRB 92026 Darmstadt) ist der einzige EFB-registrierte Betrieb des Ortes
   (Hauptsitz hinter dem namenlosen Registereintrag „Unternehmenssitz“), hat aber
   kein Dossier.
9. **GEO jetzt freigabefähig** (Adresse auf zwei unabhängigen Quellen belegt, aber
   keine Koordinate gesetzt): Bachinger Altdorf, Peter Huber Geretsried,
   Stirling Neustadt, Demand-Recycling Landau, PMS Koblenz, Korkmaz Reinheim.
   Für `by-obergunzburg-zinth` liegen zwei Aggregator-Punkte vor (47,84226/10,42094
   und 47,84204/10,42103) — das sind keine zwei unabhängigen Belege, deshalb
   keine Koordinate.
10. **Einzelfüllung zur Owner-Entscheidung** `he-bruchkoebel-beko-muenzhandel-edelmetalle`:
    das Impressum ist vollständig (Name, Inhaber, Anschrift, Telefon, E-Mail,
    USt-IdNr. VIES-gültig), für diese Rechtsform existiert aber kein Registereintrag,
    damit greift die README-Ausnahme formal nicht. Kontaktfelder blieben leer.

## Geprüfte Quellen dieser Welle

`schrottregister.pages.dev` (Orts- und Betriebsseiten, Abzug 11.08.2026),
`online-handelsregister.de` (Suche + Einzelabzug HRB 9487 / HRA 2369),
`ec.europa.eu/taxation_customs/vies`, `web.archive.org` (CDX + Snapshots),
Betreiberseiten live: bekomuenzen.de, pms-schrotthandel.de (+ EFB-PDFs),
egrw.de/partner, autoverwertung-demand.de, autoverwertung-huber.de,
bachinger-schrott.de, containerdienst-schaafheim.de (JS-Module),
containerdienst-willefuehr.de, 11880-Detailseiten, Gewerbeverein Schaafheim.
