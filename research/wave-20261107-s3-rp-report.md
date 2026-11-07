# Recherche-Welle 07.11.2026 — Shard S3 (20 Dossiers: Rheinland-Pfalz)

> 20 Händler-Dossiers aus Rheinland-Pfalz. Basis `1a934a3b`, Zweig `wave1030-s3`. Alle
> Vorwellen-Notizen sind **wortgleich** erhalten ✓. Je Dossier **genau eine** neue Sektion
> `### Recherche 07.11.2026` am Timeline-Ende ✓ (zwei Dossiers zusätzlich mit
> `### Geokodierung 07.11.2026` als letzter Sektion ✓).

## Anlass

Die 20 Dossiers wurden nacheinander abgearbeitet ✓. Schwerpunkt waren drei Quellen, die die
Vorwellen nicht erreicht hatten:

1. das **amtliche Fachbetrieberegister** (`fachbetrieberegister.gadsys.de`, clientseitig
   gerenderte Vaadin-App, per Playwright/Chromium bedient) — Namenssuche **und**
   Standortabfrage (3/5/10 km) in **beiden** Routen, dazu der **Detaildialog** je Zeile
   (Zeile markieren → „Details“) und das **Original-Zertifikats-PDF** über den
   sessiongebundenen Ressourcen-Link;
2. das **Internet Archive** (CDX + Snapshots) für die **archivierten Betreiber-Impressen** —
   der Weg, auf dem sich die offene *Mathilde-Becker-Frage* schließen ließ;
3. der **EU-VIES-Dienst** für die Gültigkeit der USt-IdNr.

| Kennzahl | Wert |
|---|---|
| Dossiers bearbeitet | **20** |
| Dossiers mit neuer Sektion `### Recherche 07.11.2026` | **20** (je **genau eine** ✓) |
| Dossiers mit **Frontmatter-Änderung** | **2** (Buchholz-Becher, Wittlich-Becker) |
| `lat`/`lon` 2 · `street` 2 · `postcode` 2 · `phone` 2 · `email` 2 · `website` 1 · `website_status` 2 · `name` 2 · `city` 1 · `description` 2 · `opening_hours` 1 | |
| Slugs geändert | **0** ✓ · Vorwellen-Notizen verändert | **0** ✓ |
| Neue Timeline-Notizen | **23** (20 Recherche + 3 Geokodierung) |
| Registerabfragen (Namens- und Radiusabfragen, beide Routen) | **rund 120** |
| Geladene Original-Zertifikats-PDFs | **1** (Lenhart-Efb, Ankreuzfelder gelesen) |
| USt-ID-Prüfungen (EU-VIES) | **5** |
| Gate-Status | `wave_gate` ✓ · `check_note_order` ✓ · `check_pre_wave_notes` ✓ · `check_cert_balance` ✓ · `cargo test -p schrott-mcp-ingestion seed` **15/15** ✓ |

## Was die Welle gefunden hat

**Zwei Dossiers, die aus einem Klärfall eine belegte Identität wurden ✓:**

* `rp-buchholz-53567-becher-wolfgang` — das amtliche Register führt
  **„Wolfgang Becher Autoverwertung“, Krautscheider Str. 25, 53567 Buchholz** unter der
  **Bescheinigung ZAES02400017 / 1** (Kennnummer G00324665). Der **Detaildialog** nennt
  wörtlich Telefon **02683 6280**, Fax **02683 7704** und **w.becher@t-online.de**; die
  Zusatzfilter `o=2` (Demontagebetrieb) **1 Treffer**, `o=1/o=3/o=4` je **0**. Der Detaildialog
  bestätigt damit die Verzeichnisangaben der Vorwelle (Das Örtliche, Gelbe Seiten,
  Ortsdienst) **statt** ihnen zu widersprechen. Der **gespeicherte GEO-Punkt lag in
  Niedersachsen** (Kassau/Clenze) und wurde durch den Objektpunkt des OSM-Adressobjekts
  **way/1005334533** ersetzt.
* `rp-wittlich-becker-mathilde` — die **archivierten Betreiber-Impressen** belegen die
  Inhaberkette **an derselben Anschrift, derselben Rufnummer und derselben behördlichen
  Registratur** (TÜV Saarland e.V., **TÜVSAA0017**): **Mathilde Becker** (2013, USt-Id
  DE149902143) → **Rudolf Becker** (2015, DE815430427) → **Hilde Kebschull** (ab 2016,
  DE301908361) → **Hilde Waltraud Kebschull** (2024). Das amtliche Register bestätigt den heutigen
  Stand mit **ZAKS00100015 / 1** („Autoverwertung Becker Inh. Hilde Waltraud Kebschull“,
  Fintenweg 5, 06571 7708, becker@autoverwertung-becker.de). Die USt-IdNr. **DE301908361** ist
  über EU-VIES gültig. Damit ist die seit dem 03.10.2026 offene Mathilde-Becker-Frage
  **geschlossen** und die bisherige Entschuldigung („keine Übernahme der Betreiberfelder“) ist
  überholt.

**Ein verschwundener Standort, der existierte ✓:**

* `rp-mainz-55116-gold-silber-ankauf` — das Internet Archive sichert die **eigene Filialseite**
  `goldankauf-silber.de/goldankauf-mainz/` in vier Fassungen (27.11.2020, 19.10.2021,
  29.11.2021, 14.08.2022) mit **Flachsmarkt 5, 55116 Mainz**, **06131 - 69 32 911**,
  **mainz@goldankauf-silber.de**, Öffnungszeiten Mo–Fr 10–18/Sa 10–16 und dem freibleibenden
  „10 % mehr!“; die Navigationsleiste des Betreibers führte „Goldankauf Mainz“ ausdrücklich. Die
  **Standortübersicht** führt die Filiale bis **12.09.2024** und **am 19.06.2025 nicht mehr** —
  die Filiale wurde also zwischenzeitlich entfernt. Die bisherige Vermutung „Verzeichnislead
  ohne Filialbeleg“ ist damit **widerlegt**, die offene Frage ist nur noch der Einstellungszeitpunkt.

**Eine geklärte Rechtsträgerkette ✓:**

* `rp-strohn-mullenbach-buchel-…` — **REDA Schrottdienst GmbH**, AG Koblenz **HRB 10993**,
  Im Wiesengrund 15, 53520 Müllenbach, Gegenstand „An- und Verkauf von Schrott und dessen
  Weiterverarbeitung“; Registerhistorie: Eintragung **22.03.2006** mit **Prokura: Ursula Daun**.
  Die Initialenfolge R.E.D.A. und der Familienname Daun erklären den Verzeichnisnamen
  **„Daun Alois“** — die bisherige Trennung der beiden ist aufgehoben. **Heinen** (Strohn) und
  **Stenzhorn** (Büchel) sind im Register **nicht** geführt (Namenssuche je 0 Treffer in beiden
  Routen).

**Eine belegte Dossier-Lücke, die keine Lücke war ✓:**

* `rp-andernach-spot-checks-…` — die **Autodemontage Mittelrhein OHG** (Bahnweg 2, 56626
  Andernach, **ZAES00100035 / 1**, Kennnummer G00527825, Rücknahme für **82 Hersteller**) ist
  über Betreiber-Impressum (**AG Koblenz HRA 12948**, Gesellschafter Alfred/Erhard Nettersheim,
  USt-Id DE149278801, **gültig** über VIES) und amtliches Register vollständig belegt. Die
  Betriebsadresse **Bahnweg 2** liegt **0,4 km** vom gespeicherten GEO-Punkt 50.4311/7.4042
  („12, Salentinstraße“) — derselbe Fehlpunkt, den die Vorwelle bereits für den
  Osnabrücker Andernach-Datensatz „Salentinstraße 12“ dokumentiert hatte.

## Negativbefunde mit Reichweite

16 Dossiers erhalten belegte Negativaussagen aus dem amtlichen Register. **Wichtig für die
Einordnung:** die EfbV- und die AltfahrzeugV-Anerkennung sind **freiwillig** — ein
Register-Nichteintrag ist **kein** Nachweis für fehlende Zertifizierung, für Nichtexistenz
oder für Betriebsende. Er belegt nur: *im Register stand am Abrufdatum unter diesem Namen und
an dieser Adresse kein Eintrag.*

| Dossier | Namenssuche | Standortabfrage | Befund |
|---|---|---|---|
| `rp-malborn-malborn` | „SE Schrott Recycling“ **0** | 54426 Malborn: 3 km **0/0**, 10 km **8/0** | 10-km-Treffer in Hermeskeil/Thalfang (Eu-Rec, Eiden Agro) — fremd |
| `rp-bellheim-lenhart-bellheim` | „Lenhart“ **1** (Robert Lenhart, Lingenfeld) | Am Weidensatz 33: 3 km **0/0**, 10 km **42/4** | „Lenhard“-Namensvetter in Lingenfeld/Lustadt |
| `rp-neustadt-binder-ramona` | „Binder“ **0**, „Scherr“ **0** | Branchweilerhofstr. 61: 3 km **0/1** | 0,4 km: Arno Metz / Autoverwertung Jeannine Metz-Bosch |
| `rp-ludwigshafen-tremel` | „Tremel“ Efb **5** (alle Bechhofen/Dombühl, BY), AfV **0** | Wachtenburgstr. 9: 3 km **18/6** | 6 AltfahrzeugV-Nachbarn in 67065, **kein Tremel** |
| `rp-kirrweiler-riehm-kirrweiler` | „Riehm“ **0** | Lerchenweide 8: 3 km **2/0** | PreZero Edenkoben, Gerst Bauservice |
| `rp-westheim-hms-westheim` | „HMS Westheim“ **0** | Am Schlittweg 1a: 3 km **10/3** | 2,4 km: kommunale **Biogutanlage BVB** (kein Schrottbetrieb) |
| `rp-bruchmuhlbach-bra-bruchmuhlbach` | „Schrotthandel Bra“ **0**, „Bra“ 263/8 | Bahnhofstr. 19B: 3 km **1/0** | TOI TOI Sanitärsysteme, Industriestraße 11 |
| `rp-koblenz-56073-kreutz-schrotthandel` | „Kreutz“ Efb **1** (REMEX Groß Kreutz, BB) | Dominicusstr. 43: 3 km **7/0** | MCR Riebe, Klaus Probst — **kein Kreutz**; auch die zweite Adresse ohne Treffer |
| `rp-trier-matthias-alten` | „Alten“ Efb **32** (alle Mayen/Altenburg/Waltenhofen), AfV **0** | Andreas-Hoevel-Str. 6: 3 km **0/0** | Trierer Großbetriebe erst ab 3,5 km |
| `rp-ockenheim-55437-schnell-jurgen` | „Schnell“ **2/1** (Wuppertal/Steinheim/Bad Kleinen) | 55437 Ockenheim: 3 km **6/0** | E&O Entsorgung, Hannawald (beide Bingen) |
| `rp-nassau-stoll-nassau` | „Stoll“ **0** | 56377 Nassau: 3 km **0/0** | 10-km-Treffer in Singhofen (REMONDIS, Quarz-Kieswerke) |
| `rp-neustadt-sterz-neustadt-wied` | „Sterz“ **0** | 53577 Neustadt (Wied): 3 km **2/0** | Schmitz Wiedmühle GmbH |
| `rp-neustadt-schwerdel-lachen-speyerdorf` | „Schwerdel“ **0** | 67435 Neustadt: 3 km **2/2** | nur Sascha Meyer (bereits als Gegenkandidat geführt) |
| `rp-hanhofen-lehr-martin-…` | „Lehr“ **6/2** (Schwandorf, Hofheim-Wallau) | Harthauser Str. 40: 3 km **0/0** | 10-km-Treffer in Speyer/Lingenfeld |
| `rp-dernbach-dernbach` | „Ludolf“ **3** (Containerdienst Ludolf & Söhne oHG, Hattingen) | Butterpfad 1: 3 km **2/0** | Tomas Entsorgung (fremd); **kein AltfahrzeugV-Betrieb in Dernbach** |
| `rp-strohn-…` (Heinen/Stenzhorn) | „Heinen“ **3** (Baesweiler), „Stenzhorn“ **0/0** | Alfbachstr. 2: 3 km **0/0**; Georgsweiler Str. 38: 3 km **0/1** | Ernst Scherer (Strohn), Autoverwertung Kunibert Hein (Büchel) |

## Preislistenfunde (ANKAUF / VERKAUF / GEBÜHREN strikt getrennt)

| Dossier | Art | Preis | Stand | Einheit | Quelle |
|---|---|---|---|---|---|
| `rp-ockenheim-55437-schnell-jurgen` | **GEBÜHREN** (Miete) | Materialcontainer 100,00 € / Woche bzw. 8,00 € / Kalendertag | gültig ab 16.05.2025, zzgl. MwSt. | Woche bzw. Kalendertag | `schnell-gruppe.de/wp-content/uploads/2025/05/Preisliste_2025.05.pdf`, S. 13 |
| | **GEBÜHREN** (Miete) | Bürocontainer 200,00 € / Woche bzw. 8,00 € / Kalendertag | gültig ab 16.05.2025 | Woche bzw. Kalendertag | ebd., S. 13 |
| | **GEBÜHREN** (Miete) | Bauwagen (3,5 m, Maulkupplung) 180,00 € / Woche bzw. 8,00 € / Kalendertag | gültig ab 16.05.2025 | Woche bzw. Kalendertag | ebd., S. 13 |
| | **GEBÜHREN** (Miete) | Baustellensicherung: Absturzsicherungs-Set 2,50 €/Tag bzw. 2,20 € ab 5 Tagen (Handling 10,00 €); Bakenleuchten 1,00 €/Tag bzw. 0,60 € (Handling 10,00 €); Verkehrsleitbaken-Set 3,00 €/Tag bzw. 2,10 € (Handling 10,00 €); Verkehrsschild-Set 2,65 €/Tag bzw. 2,00 € (Handling 10,00 €); Bauzaun-Set 0,50 €/Tag bzw. 0,40 € (Handling 10,00 €) | gültig ab 16.05.2025 | Tag / Handling | ebd., S. 14 |
| `rp-wittlich-becker-mathilde` | **ANKAUF** | **keine Kurse** — „Tagespreise“ auf Anfrage; Sorten Schrott, Altmetall, Eisen, Stahl, Edelstahl, Kupfer/Messing/Legierungen | 07.11.2026 | — | `autoverwertung-becker.de/schrotthandel` |
| | **VERKAUF** | keine Liste (Unfallfahrzeuge/Ersatzteile ohne Preise) | 07.11.2026 | — | `autoverwertung-becker.de/unternehmen` |
| | **GEBÜHREN** | keine Tarife; Container 3–15 m³ (Archivfassung 18.04.2013), Angebot auf Anfrage | 18.04.2013 | — | Wayback `…/leistungen/containerdienst.html` |
| `rp-bellheim-lenhart-bellheim` | **ANKAUF** | kein Ankauf beworben (Schrott nur als Entsorgungsgegenstand) | 07.11.2026 | — | `rohstoffe-lenhart.de/service.htm` |
| | **VERKAUF** | „An- und Verkauf von Gabelstaplern, Fahrzeugen“ ohne Preise | 07.11.2026 | — | ebd. |
| | **GEBÜHREN** | „Container größtenteils mietfrei“ (bedingt), keine Tarife; Größenangabe der Seite fehlerhaft „3-36 cm³“ | 07.11.2026 | — | ebd. |
| `rp-mainz-55116-gold-silber-ankauf` | **ANKAUF** | keine numerischen EUR/g-Kurse; freibleibendes „10 % mehr“ (nur Altschmuck, bei schriftlichem Konkurrenzangebot) | 27.10.–14.08.2022 (Filialseite) | — | Wayback `goldankauf-silber.de/goldankauf-mainz/` |
| `rp-andernach-spot-checks-…` | **ANKAUF** | keine (Angebot betrifft Alt-/Unfallfahrzeuge, nicht Schrott) | 07.11.2026 | — | `auto-mittelrhein.de` |
| übrige 15 Dossiers | — | **keine** Preisliste belegt | — | — | — |

## Offene Fragen

* **Mainz (Gold & Silber Ankauf):** Wann wurde die Filiale Flachsmarkt 5 eingestellt? Die
  Standortübersicht belegt sie am **12.09.2024**, am **19.06.2025** nicht mehr; der genaue
  Zeitpunkt und die Gründung (Verlegung?) sind offen. **Keine** Aussage zur heutigen Mainzer
  Kundenannahme.
* **Wittlich (Becker):** Die Inhaberkette ist über die Impressen belegt, **nicht** über ein
  Handelsregister — die USt-IdNr. DE149902143 (Mathilde Becker) und DE815430427 (Rudolf
  Becker) wurden **nicht** über VIES geprüft; die Kontaktdaten stammen aus Impressum +
  amtlichem Register, die **Annahme von Privatkunden** ist damit **nicht** belegt.
* **Buchholz (Becher):** Die AltfahrzeugV-Bescheinigung belegt die Anerkennung als
  **Demontagebetrieb**, **nicht** die heutige Annahme von Privatkunden und **keine** Preise.
* **Ockenheim (Schnell):** Das Impressum nennt **keine** HR-Nummer, die EfbV-/AltfahrzeugV-
  Anerkennung des Wertstoffhofs ist **nicht** belegt. Das Handelsregister führt unter derselben
  Anschrift die **SB Waschplatz Jürgen Schnell GmbH** (AG Mainz HRB 23115) - **anderer
  Geschäftszweck** (Waschplatz, Diesel, Autobedarf), als Beleg für den Schrottbetrieb **nicht**
  übernommen.
* **Ludwigshafen (Tremel):** Der Name ist amtlich **nicht** belegt; die sechs AltfahrzeugV-
  Betriebe in 67065 Mundenheim zeigen ein dichtes Recycling-Umfeld, in dem ein nicht
  registrierter Kleinbetrieb existieren kann.
* **Strohn/Müllenbach/Büchel:** Für **REDA Schrottdienst GmbH** ist **keine** EfbV-/AltfahrzeugV-
  Anerkennung ausgewiesen; die **Eintragung 2006** und die **Prokuratur** sind Registerhistorie,
  kein Betriebsnachweis.
* **Hanhofen (Lehr):** Der Bericht vom 10.05.2022 über die Stilllegung des Containerdienstes
  ist **historisch**; der heutige Betrieb bleibt offen, die Verzeichnisadresse ist amtlich
  unbelegt.
* **Dernbach:** Das Grundstück des Kult-Schrottplatzes gehört laut AUTO ZEITUNG **Manni Ludolf**;
  Peter wurde 2019 **geräumt** und kehrte 2025 zurück. Die **Verknüpfung mit „Butterpfad 1“**
  ist **offen**; in Dernbach ist **kein** AltfahrzeugV-Betrieb zertifiziert.
* **Westheim (HMS):** Ob „HMS“ je ein Betrieb war und ob „Maximilian Jäger“ derselbe ist, ist
  **offen**; die Haunnummern führt in Westheim nur die **kommunale Biogutanlage**.
* **Malborn / Riehm / Stoll / Sterz / Binder / Schwerdel / Tremel / Kreutz / Alten:** jeweils
  **kein** Betreiber- oder Registerbeleg; die Verzeichnisleads bleiben **Leads**.

## Neue Adressen mit Geokodierungsbedarf

| Dossier | Anschrift | Beleg | Bedarf |
|---|---|---|---|
| `rp-buchholz-53567-becher-wolfgang` | Krautscheider Str. 25, 53567 Buchholz (Westerwald)-Krautscheid | amtl. ZAES02400017/1 + 3 Verzeichnisse + OSM way/1005334533 | **erledigt** — 50.6969414 / 7.3788087 gesetzt; der Altpunkt (Niedersachsen) ersetzt |
| `rp-wittlich-becker-mathilde` | Fintenweg 5, 54516 Wittlich | Betreiber-Impressum + amtl. ZAKS00100015/1 + OSM way/906599843 | **erledigt** — 49.9790749 / 6.8650096 gesetzt; der alte Ortsmittelpunkt ersetzt |
| `rp-ockenheim-55437-schnell-jurgen` | Gaulsheimer Str. 18, 55437 Ockenheim | Betreiber-Standortseite + Impressum + Handelsregister (SB Waschplatz Jürgen Schnell GmbH, HRB 23115) | **offen** — die **Hausnummer 18 existiert auf der Gaulsheimer Straße in OSM nicht** (belegt: 3a, 3b, 5, 11, 13, 15, 16, 17, 19, 21, 23); der gespeicherte Punkt 49.9437/7.9713 liegt auf der **Schulstraße** |
| `rp-andernach-spot-checks-…` | Bahnweg 2, 56626 Andernach-Miesenheim | Betreiber-Impressum + amtl. ZAES00100035/1 | **offen** — in OSM existiert die Straße „Bahnweg“ (way/35015872, `highway`=service) **ohne jedes** `addr:housenumber`; der Fehlpunkt 50.4311/7.4042 („12, Salentinstraße“) bleibt in der bestehenden Zeile `rp-andernach-autodemontage-mittelrhein-nettersheim` bestehen |
| `rp-dernbach-dernbach` | Butterpfad 1, 56428 Dernbach | OSM way/524194981 (`addr:housenumber`=1, `building`=house) | **Hausnummer existiert** (50.4539586 / 7.7831973) — aber **kein** Betriebszusammenhang zum Ludolf-Schrottplatz; deshalb **kein** Punkt übernommen |
| `rp-mainz-55116-gold-silber-ankauf` | Flachsmarkt 5, 55116 Mainz | archivierte Betreiber-Filialseite (4 Fassungen) | **weiterhin offen** — laut Vorwellen-Prüfung existiert die **Hausnummer 5** auf dem Flachsmarkt in OSM nicht (belegt sind 1, 2, 3, 4-6, 7, 8, 10, 11, 16, 22, 24, 26, 28, 30, 32, 34, 38) |
| `rp-ludwigshafen-tremel` | Wachtenburgstr. 9, 67065 Ludwigshafen-Mundenheim | Verzeichnis + amtliches Umfeld | Koordinate 49.4554044 / 8.4194830 (OSM node/12828823015, `addr:housenumber`=9) **besteht** und bleibt unverändert |

## Nicht übernommen (bewusst)

* **Zahlen/Mindestmengen/Öffnungszeiten aus Verzeichnis-Portalen** ohne Betreiberbestätigung
  (u. a. „SE Schrott Recycling“, „Schrott und Metallhandel Stoll“, „Schrotthandel Bra“): als
  Portal-Vorlagen bzw. unbestätigte Angaben gekennzeichnet, **nicht** in Felder übernommen.
* **Fremde Betriebe an Nachbaradressen** (Arno Metz, Sascha Meyer, Lenhard/Lenhart in
  Lingenfeld, Robert Lenhart, Ludolf & Söhne in Hattingen, Kunibert Hein, Schmitz Wiedmühle,
  Tomas Entsorgung, Containerdienst „Flink“ u. a.): im Dossier benannt, **nicht** übernommen.
* **Der Spiegel `schrottregister.pages.dev`**: ausschließlich zum **Auffinden** von
  Zertifikatsnummern verwendet, **nie** als Beleg zitiert.
* **Preise fremder Händler** (GoldSilberShop/SOLIT Management GmbH, Rheinstr. 103 Mainz) und
  **Containerdienst-Flink** (Schönenberg-Kübelberg): **nicht** übernommen.
