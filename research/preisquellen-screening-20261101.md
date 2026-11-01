# Preisquellen-Screening Ketten- und Großstandort-Domains — 01.11.2026

> **Korrektur vom 01.11.2026, 07:40 UTC (Owner-Prüfung).** Die Domain-Map dieses Screenings
> hat die Domains **systematisch falsch extrahiert** — jeweils fehlen die **führenden Zeichen**
> des Labels: `https://www.weckwerth-schrott.de` → `eckwerth-schrott.de`,
> `https://www.wertstoff-dealer.de` → `wertstoff-dealer.de`,
> `https://www.wasdrack.de` → `asdrack.de`, `https://www.wegrogmbh.de` → `egrogmbh.de`,
> `https://www.wittmann-recycling.de` → `ittmann-recycling.de`, `https://wil-ro.de` → `il-ro.de`,
> `https://www.weli-stahl.de` → `eli-stahl.de`. Daraus folgten 13 vermeintlich „tote“ Domains.
> **Live nachgeprüft (DNS + HTTP, beide Hostformen): alle diese Domains sind erreichbar** —
> einzige Ausnahme ist `rm-schrotthandel.de` (wirklich NXDOMAIN). Die Abschnitte **B.4** und
> **D.4** sind daher in diesem Punkt **überholt**: aus einer toten Domain darf kein
> `website_status: tot` abgeleitet werden. Die Wellen-Recherche 01.11. 06:00 hat die Korrektur
> bestätigt und die betroffenen Dossiers mit der belegten Negativaussage dokumentiert
> (`research/wave-20261101-0600-report.md`).


**Auftrag:** Fertigstellung des Domain-Screenings mit Challenge-Erkennung; Ergebnisbericht.
**Bearbeitet:** 01.11.2026, 03:35–04:20 UTC (Abrufe durchgehend am 01.11.2026).
**Methode:** Read-only HTTP-GET mit Browser-User-Agent, Timeout 25 s. Pro Domain: Startseite (https, Fallback http) → Challenge-Erkennung → robots.txt + sitemap.xml + robots-Sitemap-Hinweise → Link-Harvesting (Ankertext + URL) → generische Preispfade → Kandidatenabruf mit Marker-Sweep → PDF-Extraktion via `pdftotext -layout` → Preiszeilen-Erkennung (Wert **und** Einheit) → Datums-Token-Suche.
**Kein** Dossier wurde angefasst, **kein** Deploy, **kein** Produktions-Write, **keine** Git-Befehle. Ausgabe ausschließlich diese Datei.

**Datensatz:** `/tmp/opencode/screendata.json` (243 Domains, 1,55 MB), Rohcache `/tmp/opencode/cache` (15.385 HTTP-Antworten), Skript `/tmp/opencode/screen.py` + `/tmp/opencode/from_cache.py`.

---

## 0. Kurzfassung

| Kennzahl | Wert |
|---|---|
| Domains geprüft | **243** (Kette-/Großstandort-Liste, 1.923 Domains in der Repo-Domain-Map, davon diese 243 mit Vorrang) |
| Erreichbare Startseite (HTTP 200) | **221** |
| Domains mit **Preiszeilen (Wert + Einheit)** | **17** (davon 13 verschiedene Betreiber-Domains; `www.`-Doppel zählen doppelt) |
| Domains mit Bot-Challenge | **8** (davon 2 mit zusammen 42 Händler-Slugs) |
| Domains nicht erreichbar (DNS/TLS/Timeout) | **16** |
| Domains HTTP ≥ 400 | **6** |
| Erreichbar, aber **keine** Preiszeile gefunden | **191** (652 Slugs) |
| Abgedeckte Händler-Slugs (Domain→Slug-Map) | **809** (614 eindeutige Slugs) |

**Kernaussage:** Von 221 erreichbaren Ketten-/Großstandort-Domains tragen **17** (13 verschiedene Betreiber) überhaupt Zahlenwerte mit Einheit. **Vier** davon liefern eine vollständig datierte Ankaufpreisquelle (Preis + Einheit + Datum) — darunter **ein Neuzugang ohne Handler**: `pur-umwelt.com` (Stand 26.10.2026, 14:30 Uhr, 19 Sorten). **Zwei weitere Neuzugänge** liefern Zahlen + Einheit **ohne** Datum: `schrottjungs.de` (8 Slugs) und `schrotthandel-berlin.com` (Kiro, 750-kg-Mengenstaffel). Die beiden bekannten Treffer sind **beide weiterhin live und gültig**.

**Die Branchen-Grundregel bestätigt sich:** Der Hebel liegt nicht bei Domains, sondern bei **Betreibern, die eine Preislisten-Infrastruktur pflegen**. Die beiden größten Ketten (Scholz 48 Slugs, TSR 37 Slugs) liefern **nichts** — Scholz hat keine Preisquelle, TSR ist hinter einer Incapsula-Challenge vollständig blockiert. Das ist kein Screening-Mangel, sondern fehlende bzw. technisch gesperrte Publikation.

---

## A. Preisquellen mit Preis

Sortiert nach Nutzbarkeit: **A1** = Preis + Einheit + Datum (voll eligible), **A2** = Preis + Einheit, aber ohne Datum bzw. nur Obergrenze („bis zu"), **A3** = Gebührenquelle (Preis, aber kein Ankaufpreis), **A4** = dynamische Börsen-/Spot-Referenz (kein Händler-Ankaufspreis).

### A1 — Preis + Einheit (Datum-Status je Quelle einzeln ausgewiesen)

**Achtung zur Lesart:** Von den sechs Quellen in A1 tragen nur **vier** ein konkretes, belegtes Datum (A1.1, A1.2, A1.3, A1.6). **A1.4** (`schrottjungs.de`) und **A1.5** (`schrotthandel-berlin.com`) haben Zahlenwerte und Einheit, aber **kein** Datum — sie sind damit **nicht eligible** und stehen hier nur, weil sie die substanziellsten Neuzugänge sind. Die Datum-Status-Zeile in jeder Quelle ist maßgeblich.

---

#### A1.1 `dein-schrottplatz.de` — DB Recycling, Denis Blum, Rochlitz

| Feld | Wert |
|---|---|
| Betreiberdomain | `www.dein-schrottplatz.de` |
| URL | `https://www.dein-schrottplatz.de/pages/preise.php` |
| Abrufzeitpunkt | 01.11.2026, 03:36–03:40 UTC |
| statisch/dynamisch | **statisch** (serverseitig gerendertes HTML, 66 Tabellenzeilen) |
| Einheit | `€/kg` (Spaltenkopf „Preis/kg"), Einzelwerte als „€/kg"; Entgeltzeilen als „Netto 0,3= 300€/t" |
| Mindestmenge | keine; **zwei Annahmestopps** (Textilien, Schuhe) |
| Gültigkeitsdatum | **„Gültig ab 03.10.2026"** (letzte Tabellenzeile) |
| Erreichbare Händler-Slugs | `sn-rochlitz-db-recycling-denis-blum` (1) — **bereits durch `db_recycling.rs` abgedeckt** |

**Wörtliche Preisfragmente (verbatim, inkl. Einheit und Sortenbezug):**

> `Hartmetall 45,00€/kg`
> `Cu Raff 92%` | `9,20 €`
> `Cu schwer` | `10,00 €`
> `Cu Oberleitung/Freileitung` | `10,00 €`
> `Cu Lackdraht/Berry` | `9,70 €`
> `Cu Millberry` | `11,40 €`
> `Ms schwer` | `6,20 €`
> `Ms Raff max. 10% Anhaftung` | `4,00 €`
> `Ms Raff Späne` | `5,00 €`
> `Cu-Ms Kühler o. Fe` | `5,00 €`
> `Al Draht blank` | `1,90 €`
> `Al Freileitung o. Fe` | `1,70 €`
> `Al Profil Si 0,5 blank` | `1,90 €`
> `Al Felgen sauber` | `2,00 €`
> `Al Blech/Guß mit 2% Fe` | `1,00 €`
> `Al Getriebe` | `0,40 €`
> `Al-Cu Kühler o. Fe` | `4,20 €`
> `Cu-Shredderkabel` | `3,80 €`
> `Cu-Schlitzkabel 60%` | `5,80 €`
> `Al-Shredderkabel` | `0,10 €`
> `Al-Schlitzkabel` | `1,00 €`
> `Zinkblech` | `2,00 €`
> `Reinzinn` | `30,00 €`
> `Altblei` | `1,20 €`
> `Wucht-/Kabelblei` | `0,40 €`
> `Batterieblei/Bleiakkus` | `0,30 €`
> `Chromstahl` | `0,16 €`
> `V2A` | `0,80 €`
> `V4A` | `1,60 €`
> `E-motoren` | `0,80 €`
> `Getriebemotoren` | `0,30 €`
> `Schwerer Scherenschrott` | `0,15 €`
> `Leichter Scherenschrott` | `0,13 €`
> `Kernschrott` | `0,18 €`
> `Gußschrott` | `0,18 €`
> `Mischschrott` | `0,10 €`
> `Schreddervormaterial` | `0,05 €`
> `PC-komplett` | `0,60 €`
> `Laufwerk` | `0,20 €`
> `Festplatte` | `1,40 €`
> `Netzteil` | `0,20 €`
> `Leiterplatte Klasse 1` | `10,00 €`
> `Leiterplatte Klasse 3` | `0,80 €`
> `Altpapier` | `0,08 €`

**Ankaufcharakter (Fließtext, verbatim):** „Anbei finden Sie eine Übersicht der aktuellen **Einkaufspreise** für unser Schrottportfolio." / „Bei größeren Mengen erfragen Sie bitte die Einkaufspreise unter der …"

**Mindestmenge / Staffeln:** Die Seite nennt **keine** Mengenstaffel; „Bei größeren Mengen erfragen Sie bitte die Einkaufspreise" ist eine Aufforderung zur Einzelanfrage, kein Schwellenwert.

**Abgrenzung:** Die letzten sechs Zeilen mischen Ankauf und Entgelt und sind **kein** Ankaufpreis:
> `Sperrmüll` | `Netto 0,3= 300€/t`
> `Autoreifen (mit und ohne Felge)` | `2,50 €`
> `Bauschutt sauber!!!` | `Netto 0,04=40 €/t`
> `Grünschnitt / Altholz` | `Netto 0,15= 150€/t`
> `Transport Absetzcontainer` | `100,00 €`
> `Textilien` | `Annahmestopp`
> `Schuhe` | `Annahmestopp`

**Einheiten-Fallstrick:** Die Einheit steht als Gruppenüberschrift (`Kupfer` | `Preis/kg`), nicht in jeder Zeile — ein Parser muss die Gruppenüberschrift auf die Folgezeilen vererben.

---

#### A1.2 `elno-container.de` — ELNO Container- und Dienstleistungs GmbH, Berlin-Spandau

| Feld | Wert |
|---|---|
| Betreiberdomain | `www.elno-container.de` |
| URL (aktuell) | `https://www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf` |
| Abrufzeitpunkt | 01.11.2026, 03:36 UTC |
| statisch/dynamisch | **statisch** (PDF, 2 Seiten A4, `pdftotext -layout` ohne OCR) |
| Einheit | `€/Kg` (Metall-Ankauf); zusätzlich `€/to` und `€/cbm` für Abfall-Verkaufspreise |
| Mindestmenge | keine; **gilt ausdrücklich nur für Privatkunden** |
| Gültigkeitsdatum | **nicht im Text** — dreifach redundant anderswo: Dateiname `Preisliste_16.10.pdf`, WP-Uploadpfad `/2026/10/`, PDF-`CreationDate: D:20261016131959+02'00'`, HTTP `Last-Modified: Fri, 16 Oct 2026 11:20:19 GMT` |
| Erreichbare Händler-Slugs | `be-spandau-elno-container-und-dienstleistungs` (1) — **bereits durch `elno.rs` abgedeckt** |

**Wörtliche Preisfragmente (verbatim, Spalten „Material | Preis"):**

> `Cu-Raff 95%` | `9,70 €/Kg`
> `Cu-Millberry` | `10,70 €/Kg`
> `Cu.Haardraht` | `10,30 €/Kg`
> `Cu-Kerze, sauber/ neu` | `10,30 €/Kg`
> `Cu-Schienen, sauber` | `10,50 €/Kg`
> `Cu Kabel ca. 38%` | `3,40 €/Kg`
> `Ms-Milbe, 1% Anhaftung` | `5,70 €/Kg`
> `Ms-Raff (Wasseruhren etc.)` | `3,10 €/Kg`
> `Ms-58` | `5,90 €/Kg`
> `Cu-MS Kühler` | `3,00 €/Kg`
> `V2A` | `0,60 €/Kg`
> `V2A Verhüttung` | `0,20 €/Kg`
> `V4A` | `1,35 €/Kg`
> `Mischschrott` | `0,130 €/Kg`
> `Shreddervormaterial` | `0,07 €/Kg`
> `Bremsscheiben` | `0,165 €/Kg`
> `Sorte 3` | `0,210 €/Kg`
> `Leichte Schere` | `0,145 €/Kg`
> `Schwere Schere` | `0,155 €/Kg`
> `Guss` | `0,140 €/Kg`
> `Alt-Zink` | `1,50 €/Kg`
> `Neu-Zink` | `1,60 €/Kg`
> `Alt-Blei` | `0,90 €/Kg`
> `Akku-Blei, z.B. Autobatterien` | `0,275 €/Kg`
> `Sammelzinn` | `5,00 €/Kg`
> `Elektronikschrott (Platinen, Netzteile, Rechner etc.)` | `Auf Anfrage`
> `Al-Draht, ohne Anhaftungen` | `1,95 €/Kg`
> `Al-Profile, gemischt` | `1,90 €/Kg`
> `Al-Profile, Länge über 1m` | `1,70 €/Kg`
> `Al-Blech ohne Anhaftung` | `1,20 €/Kg`
> `Al-Geschirr, max. 5% FE` | `1,00 €/Kg`
> `Al-Verhüttung` | `0,20 €/Kg`
> `Al-Kabel, dick` | `0,45 €/Kg`
> `Al-Guss ohne Anhaftung` | `0,95 €/Kg`
> `Al-Guss, 5% FE` | `0,85 €/Kg`
> `Al-Felgen mit Anhaftung` | `1,60 €/Kg`
> `Al-Felgen ohne Anhaftung` | `1,60 €/Kg`
> `Al-Cu Kühler mit Anhaftung` | `3,50 €/Kg`
> `Al-Cu Kühler ohne Anhaftung` | `3,70 €/Kg`

**Abgrenzung (Verkaufspreise, kein Ankauf):** `Styropor 89 €/cbm`, `Baumischabfälle 290 €/to`, `Asbesthaltige Baustoffe 280 €/to`.

**Kundenkreis-Einschränkung (verbatim, PDF-Kopf):** „Die Preise gelten **ausschließlich für Privatkunden**, bitte fragen sie als gewerblicher Kunde Preise bei uns telefonisch an. Die Preise sind freibleibend und können täglich variieren."

**PDF-Serie:** Die Preise liegen alsmonatliche Serie unter `wp-content/uploads/<Jahr>/<Monat>/Preisliste_<Tag.Monat>.pdf`. Die Serie ist **nicht über ein Verzeichnislisting oder eine Sitemap** auffindbar (`/wp-content/uploads/2026/` → HTTP 403, `wp-sitemap.xml` listet nur `page-sitemap.xml`); der Zugriff läuft ausschließlich über den Homepage-Anker „Unsere Altmetall-Preise". Stichprobe: `Preisliste_16.10.pdf` → HTTP 200, `Preisliste_15.09.pdf` → HTTP 404, `2025-01-06-07-43-1.pdf` → HTTP 200. **Die Historie ist damit nicht per URL erratbar** — der Handler muss die Historie über Wayback/Archiv rekonstruieren.

**Zwei Live-URLs derselben PDF:** Neben dem Upload-Pfad liefert die Domain auch `https://www.elno-container.de/preisliste_16-10/` (HTTP 200, gleicher Inhalt, gleicher `Last-Modified`) — eine für Handler nutzbare, stabile_html-Variante.

---

#### A1.3 `pur-umwelt.com` — PUR Umwelt (P. U. Richter), Bonn-Bad Godesberg / Hannover — **NEUZUGANG**

| Feld | Wert |
|---|---|
| Betreiberdomain | `pur-umwelt.com` |
| URL | `https://pur-umwelt.com/schrottpreise/` (identisch: `https://pur-umwelt.com/schrottankauf/`, `/preise`, `/preisliste`) |
| Abrufzeitpunkt | 01.11.2026, 03:53 UTC |
| statisch/dynamisch | **statisch** (serverseitig gerendertes HTML) |
| Einheit | `€/t` bzw. `€/to` (Tonnen) — alle Werte in €/Tonne, eine Sorte als `-€/to` |
| Mindestmenge | **explizit, dreistufig** (siehe unten) |
| Gültigkeitsdatum | **„gültig ab: 26.10.2026, 14:30 Uhr"** (verbatim, im Seitentitelbereich) |
| Erreichbare Händler-Slugs | `nw-bonn-pur-umwelt-p-u-richter`, `ni-hannover-p-u-richter-umweltdienste-rheinland` (2) — **kein Handler vorhanden** |

**Wörtliche Preisfragmente (verbatim):**

> `FE-Schrott: Misch./Händler` | `175,-€/to`
> `Misch./Privat ab 100 kg` | `135,-€/to`
> `FE-Guss` | `180,-€/to`
> `Bremsscheiben` | `200,00 €/t`
> `Eisenspäne` | `140,-€/to`
> `Alu-Blech rein alt` | `1.800,00 €/t`
> `Alu-Felgen unsauber` | `2.200,00 €/to`
> `Alu-Geschirr` | `1.500,00 €/t`
> `Alu Iso-Profile` | `1.800,00 €/t`
> `Blei ohne Anhaftungen` | `1.100,00 €/t`
> `Blei-Batterien von Fahrzeugen` | `auf Anfrage`
> `Kupferkabel mit Stecker` | `1.700,00 €/t`
> `Kupferkabel ohne Stecker 38 % CU` | `3.800,00 €/t`
> `Kupfer Millberry` | `10.000,00 €/t`
> `Kupfer schwer Raff, Basis 95%` | `8.500,00 €/t`
> `Messing schwer - o. Hülsen/Schläuche` | `5.700,00 €/t`
> `Mischschrott - Händler` | `175,00 €/t`
> `V2A` | `1.000,00 €/t`
> `V4A` | `1.800,00 €/t`
> `VA-Späne` | `800,00 €/t`
> `Verhüttung` | `350,00 €/t`
> `Altzink` | `2.000,00 €/t`
> `Zinngeschirr` | `14.000,00 €/t`

**Mindestmengen / Staffeln (verbatim, drei separate Sätze):**

> „Ankaufspreise **frei** Friesdorfer Straße 176 in 53175 Bonn-Bad Godesberg. Preise ohne Gewähr. Preisänderungen vorbehalten."
> „**Mindermengenabzug von 10% bei Metallen bei weniger als 200 kg!** Auszahlung bzw. Vergütungsanspruch für Blech- und Mischschrott **erst ab einer Menge von mehr als 100 kg pro Materialgruppe!!**"
> „Wichtiger Hinweis: **Ab einer Menge von 500 kg pro Sorte muss der Preis vorher angefragt und gemeinsam fixiert werden.** Wir kaufen auch Hartmetall/Widia (Bohrer, Wendeplättchen). Bitte telefonischer Kontakt unter +49 174 3398109 (Herr C. Axer)"

**Einordnung:** Der Widerspruch „gültig ab 26.10.2026" gegen den Hinweis „Ab 500 kg muss der Preis vorher angefragt werden" ist **kein** Widerspruch: Für Mengen < 200 kg gibt es einen definierten Abzug, für > 500 kg ist der Preis *nicht* veröffentlicht. Die veröffentlichte Tabelle ist damit der Preis für die mittlere Mengenklasse — für den Scraper als `price_kind: "exact"` mit `price_min`/`price_max`-Randnotiz buchbar, aber die Mengenstaffel muss im `notes`-Feld mitgeführt werden.

**Ausschluss:** „⇒ Grundsätzlich keine Annahme von Elektronik Schrott ⇐" (verbatim im Seitenkopf) — E-Schrott ist nicht Anlagerohstoff.

---

#### A1.4 `schrottjungs.de` — Schrottjungs (8 Slugs) — **NEUZUGANG, ohne Datum**

| Feld | Wert |
|---|---|
| Betreiberdomain | `schrottjungs.de` |
| URL | `https://schrottjungs.de/gewerbe/` |
| Abrufzeitpunkt | 01.11.2026, 03:40 UTC |
| statisch/dynamisch | **statisch** |
| Einheit | `€/Tonne` (FE) und `€/kg` (NE) |
| Mindestmenge | „Gültig ab Großmengen" (kein Zahlenwert) |
| Gültigkeitsdatum | **kein konkretes Datum** — aber „Gültig ab" als Formulierung vorhanden; zusätzlich „Tagespreise auf Anfrage" |
| Erreichbare Händler-Slugs | **8** (davon 7 ohne Handler): `be-berlinweit-mobil-schrottjungs-mobile-abholung`, `by-u-a-munchen-schrottjungs-mobil-bundesweit`, `hb-bremen-schrottjungs-bremen`, `hh-hamburgweit-mobile-abholdienste-schrottjungs-schrott`, `ni-hannover-schrottjungs-abholung`, `sl-saarbrucken-umland-schrottjungs-mobil`, `sn-markkleeberg-taucha-schkeudi-schrottjungs-schrottabholung-leipzig`, `sn-mobil-leipzig-schrottjungs-schrottabholung-leipzig` |

**Wörtliche Preisfragmente (verbatim):**

FE-Schrott / Eisenschrott (Richtpreise pro Tonne):
> `Scherenvormaterial` | `bis zu 200 €/Tonne`
> `Mischschrott leicht` | `bis zu 170 €/Tonne`
> `Mischschrott schwer` | `bis zu 210 €/Tonne`
> `Sorte 3` | `bis zu 240 €/Tonne`
> `Trägerschrott` | `bis zu 230 €/Tonne`
> `Bremsscheiben` | `bis zu 240 €/Tonne`
> `Zerlegematerial` | `bis zu 240 €/Tonne`
> `NE` | `bis zu 540 €/Tonne`

Buntmetalle / NE-Metalle (Richtpreise pro Kilogramm):
> `Kupfer Raff` | `bis zu 8,50 €/kg`
> `Kupfer Millberry` | `bis zu 9,50 €/kg`
> `Kupferschiene` | `bis zu 9,20 €/kg`
> `Messing gemischt` | `bis zu 4,50 €/kg`
> `Rotguss` | `bis zu 8 €/kg`
> `Kabel (diverse)` | `bis zu 5,20 €/kg`
> `Aluminium (div.)` | `bis zu 2,50 €/kg`
> `Edelstahl V2A` | `bis zu 1,10 €/kg`
> `Zink` | `bis zu 1,90 €/kg`
> `Blei` | `bis zu 1,40 €/kg`
> `Zinn` | `bis zu 22 €/kg`
> `Hartmetall / Wolfram` | `bis zu 55 €/kg`

**Status-Aussage (verbatim):**
> „Metallpreise für Buntmetalle - Richtpreise pro Tonne · Gültig ab Großmengen · Tagespreise auf Anfrage"
> „Alle genannten Preise sind **unverbindliche Richtpreise** für Metallankauf in Großmengen."
> „Tagesaktuelle Metallpreise erhalten Sie auf Anfrage. Die tatsächlichen Metallankaufspreise könn[t]en [abweichen]"

**Klassifikation:** Wegen „bis zu" sind die Werte **Obergrenzen**, keine fixen Preise — das entspricht `price_kind: "approx"` bzw. `price_max` mit `price_min: None`, **nicht** `exact`. Der Repozweig hat für diesen Fall bereits eine Präzedenz: `mc_schrott.rs` bucht „unverbindliche Richtpreise" bewusst als `exact` mit `confidence: 1.0` (dokumentiert im Handler-Kopf, mit ausdrücklichem Hinweis auf die Gegenpräzedenzen `gerwischer`/`limekon` mit `approx`/0.5). Für `schrottjungs` ist die Entscheidung dieselbe wie dort zu treffen und zu begründen.

---

#### A1.5 `schrotthandel-berlin.com` — Kiro Schrotthandel, Berlin-Pankow — **NEUZUGANG, ohne Datum**

| Feld | Wert |
|---|---|
| Betreiberdomain | `schrotthandel-berlin.com` |
| URLs | `https://schrotthandel-berlin.com/` (Start, 4 Zeilen) + `https://schrotthandel-berlin.com/arten-von-altmetall/{aluminium,blei,kupfer,messing,...}/` (Sorten-Seiten) |
| Abrufzeitpunkt | 01.11.2026, 03:40 UTC |
| statisch/dynamisch | **statisch** |
| Einheit | `€/kg` |
| Mindestmenge | **ja, explizit als Mengenstaffel**: „(Ab 750 kg: …)" |
| Gültigkeitsdatum | **kein Datum** — kein Stand, keine Jahreszahl im Preisblock |
| Erreichbare Händler-Slugs | `be-pankow-kiro-schrotthandel-kafedzhiev`, `be-pankow-schrottmeister-altmetallhandel` (2; 1 davon ohne Handler) |

**Wörtliche Preisfragmente (verbatim, Startseite):**

> `Kupfer Millberry` | `10.90 €/kg`
> `Kupfer Raff` | `10.25 €/kg`
> `Messing` | `6.15 €/kg`
> `Kupferkabel` | `3.20 €/kg`

**Wörtliche Preisfragmente (verbatim, Sorten-Seiten mit Mengenstaffel):**

> „Preise für Kupferschrott bei Kiro Schrotthandel"
> `Kupferkabel:*` | `3.10 €/kg` | `(Ab 750 kg: 3.30 €)`
> `Kupfer Raff 95%:*` | `9.30 €/kg` | `(Ab 750 kg: 9.50 €)`
> `Kupfer Millberry:*` | `9.95 €/kg` | `(Ab 750 kg: 10.15 €)`

> „Preise für Aluminiumschrott bei Kiro Schrotthandel"
> `Aluminium/ Sauber - Sorte 1*:` | `1.35 €/kg` | `(Ab 750 kg: 1.55 €)`
> `Aluminium max. 5% Anhaftung*:` | `1.15 €/kg` | `(Ab 750 kg: 1.35 €)`
> `Aluminiumkabel*:` | `4.85 €/kg` | `(Ab 750 kg: 5.05 €)`

> „Preise für Bleischrott bei Kiro Schrotthandel"
> `Altbleischrott*:` | `0.95 €/kg` | `(Ab 750 kg: 1.05 €)`
> `Kabelschälblei*:` | `0.30 €/kg` | `(Ab 750 kg: 0.45 €)`

**Achtung — Widerspruch in der eigenen Quelle:** Die **Startseite** nennt für dieselben Sorten *andere* Werte als die Sorten-Seiten (`Kupfer Millberry` 10.90 €/kg auf der Startseite vs. 9.95 €/kg auf `/kupfer/`; `Kupfer Raff` 10.25 vs. 9.30; `Messing` 6.15 vs. — ). Die Startseite nennt außerdem eine **Mindestmenge implizit** über „Wir bezahlen Sie für Ihren Schrott und …". Die Startseiten-Werte wirken wie ein **älterer Stand**. Ohne Datum ist **nicht entscheidbar**, welche Liste aktuell ist — das ist der entscheidende Grund, warum diese Quelle trotz Zahlenwerten **nicht eligible** ist. Der Sollwert ist mit dem Betreiber zu klären, bevor irgendein Wert gebucht wird.

---

#### A1.6 `mcschrott.de` — MC Schrott (6 Annahmestellen) — Bestätigung, **Handler existiert bereits**

| Feld | Wert |
|---|---|
| Betreiberdomain | `mcschrott.de` |
| URLs | `https://mcschrott.de/` (Auszug je Standort) + `https://mcschrott.de/preise/` (vollständige Liste, 59 Positionen in 7 Tabellen) |
| Abrufzeitpunkt | 01.11.2026, 03:41 UTC |
| statisch/dynamisch | **statisch** (HTML-Tabellen); die Standortwahl ist **clientseitig** — der Server liefert für `?standort=<key>` byte-identisches HTML |
| Einheit | `€/kg` („Alle Angaben verstehen sich in Euro pro Kilogramm (€/kg) und als Nettopreise") |
| Mindestmenge | keine Mengenstaffel |
| Gültigkeitsdatum | **„Stand: 30.10.2026."** (verbatim im Hero-Lede) |
| Erreichbare Händler-Slugs | 6, **alle durch `mc_schrott.rs` abgedeckt** |

**Wörtliche Preisfragmente (verbatim, `/preise/`):**

> `Kupfer Millberry 99,9 %, Drähte aus Kabeln stärker als 1 mm, glänzend, unlegiert, sauber ohne Anhaftungen` | `10,50 €`
> `Kupfer Raff 95 %, oxidiert oder verunreinigt` | `9,40 €`
> `Kupfer Kabel dünn bis 10 mm² Querschnitt` | `3,00 €`
> `Kupfer Kerze 99 %, unlegiert, ohne Anhaftungen, glänzend, feste Stücke, alle Materialstärken` | `9,50 €`
> `Kupfer Kabel dick ab 10 mm² Querschnitt` | `3,50 €`
> `Kupfer Leitschiene aus Schaltschränken` | `10,00 €`
> `Kupfer Messing Kühler Kupferkühlernetz mit Messingrahmen, aus Oldtimern` | `2,90 €`
> `Kupfer Alu Kühler mit Kupferrohren und Alulamellen, als Wärmetauscher in Heizungs- oder Industrieanlagen` | `1,80 €`
> `Kupfer Berry 98 %, dünne glänzende Drähte, oxidierte Stücke ohne Anhaftungen von Öl, Fett, Lack und sonstigem Schmutz` | `9,60 €`
> `Kupfer Späne Abfälle der Zerspanung von Kupfer, max. 2 % Flüssigkeitsanteil` | `3,40 €`
> `Alu Draht luftgeschwärzt saubere, oxidierte Drähte aus Kabeln` | `1,20 €`
> `Alu Draht blank saubere, unoxidierte Drähte aus Kabeln` | `1,30 €`
> `Alu Gemisch sauber z. B. Rohre, Bleche, Kühlkörper ohne Anhaftungen` | `0,95 €`
> `Alu Guss sauber Getriebedeckel, Getriebegehäuse und Ähnliches ohne Anhaftungen` | `1,00 €`
> `Alu Profile blank unbeschichtet, sauber, ohne Eisen- und Kunststoffanhaftungen` | `1,20 €`
> `Alu Felgen sauber ohne Nabendeckel, Ventile, Wuchtgewichte und Stahlsitze für Radbolzen` | `1,20 €`
> `Messing max. 3 % Anhaftung – Wasser-, Gashähne und Adapterstücke, nicht magnetische Vorhängeschlösser ohne Bügel, Schließzylinder und Schlüssel, Gleitlagerbuchsen` | `6,00 €`
> `V2A Stahl Pfannen, Töpfe, Blechspülbecken, Flexschläuche für Wasserarmaturen` | `0,70 €`
> `V4A Stahl Geländer von Häfen, Schwimmbädern oder Schiffen, Chemietanks` | `1,20 €`
> `Zink alt/neu Dachrinnen oder -bleche, historisches Geschirr` | `1,60 €`
> `Blei Bleistangen, Angelblei` | `1,00 €`
> `Schälblei Bleiummantelung aus Erdkabeln, muss öl-, fett- und teerfrei sein` | `0,50 €`
> `Auswuchtblei Schlag- und Klebegewichte zum Auswuchten` | `0,10 €`
> `Altschrott Sorte 3 Stahl mindestens 6 mm stark, maximal 1,5 × 0,5 × 0,5 m` | `0,17 €`
> `Mischschrott Stahlschrott mit geringen Mengen anderer Metallen und Anhaftungen` | `0,15 €`
> `Gussschrott schwere Heizkörper und Öfen` | `0,15 €`
> `Bremsscheiben aus Gusseisen` | `0,15 €`
> `Mischschrott leicht / Waschmaschinen Herde, Geschirrspüler, Waschmaschinen, Wäschetrockner und sehr dünnwandige Stahlprodukte` | `0,07 €`
> `E-Motoren elektrische Antriebe und Generatoren ohne Anbauteile sowie Kupferspulen mit Eisenkern – in Elektrofahrzeugen und elektrischen Maschinen, aber auch in vielen Elektrowerkzeugen oder Haushaltsgeräten` | `0,35 €`
> `E-Motoren mit Getriebe elektrische Antriebe mit angebautem Getriebe oder Ähnlichem` | `0,20 €`
> `Papier Zeitungen, Zeitschriften, Kataloge, Druckerpapier` | `0,10 €`
> `Chromstahl Stahl mit über 10 % Chromanteil` | `auf Anfrage`
> `Widiaplättchen Hartmetallschneiden für Werkzeugmaschinen` | `auf Anfrage`
> `Bohrer Hartmetallbohrer für Werkzeugmaschinen` | `auf Anfrage`
> `Katalysator klein …` | `auf Anfrage`
> `Katalysator groß …` | `auf Anfrage`

**Standort-Spezifika (verbatim):** „Auszug für unseren Standort **Brandenburg an der Havel, Stand 30.10.2026**". Der Rostock-Satz weicht ab und steht in einem eigenen Attribut `data-price-ros` (53 solcher Attribute in der Seite, z. B. `data-price-ros="10.5"` gegen `Kupfer Millberry 10,50 €`). Das bestehende `mc_schrott.rs` verarbeitet genau das.

**Zwei nicht im Preisblock, aber preisrelevant (verbatim, unter den Tabellen):**
> „Was abgezogen wird: Große Mengen nicht metallischer Materialien in einer Lieferung werden nicht vergütet, sondern mit **0,25 € je Kilogramm** berechnet. Reifen werden nach Stück abgerechnet: **30,00 € je PKW-Reifen**, **50,00 € je LKW-Reifen**. Diese Beträge stehen nicht in der Preisliste und sind im Rechner nicht enthalten – sie werden an der Annahmestelle von der Gutschrift abgezogen."

**Wichtig für den Bestand:** Das Repo-Handler-Dokument (`mc_schrott.rs`, Zeile 36) nennt als erwartetes Seitentdatum **„Stand: 22.09.2026."** — **live ist es inzwischen „Stand: 30.10.2026."** Der Handler liest das Datum dynamisch (`html.find("Stand:")`, Zeile 668), läuft also korrekt; **nur der Doc-Kommentar ist veraltet**. Das ist ein Kommentar-Fix, kein Code-Fix — im Auftrag wurde **kein** Dossier/Handler geändert.

---

### A2 — Preis + Einheit, aber ohne Datum (nachrüstbar, nicht eligible)

---

#### A2.1 `eo-entsorgung.de` — E. O. Entsorgung (3 Slugs) — JSON-API, **stärkster Einzelbefund ohne Datum**

| Feld | Wert |
|---|---|
| Betreiberdomain | `eo-entsorgung.de` |
| URL (maschinenlesbar) | `https://eo-entsorgung.de/wp-json/wp/v2/pages?slug=ankauf-altmetall_offenbach` |
| Weitere Slugs | `…?slug=ankauf-altmetall_wiesbaden`, `…?slug=ankauf-altmetall_mainz`, `…?slug=ankauf-altmetall_frankfurt`, `…?slug=ankauf-altmetall_darmstadt` |
| Format | **JSON** (WordPress-REST), `content.rendered` = HTML-Fragment mit `<li>`-Preisliste |
| HTTP | **200**, `application/json` |
| Einheit | `€/Tonne` (verbatim „€/Tonne") |
| Datum | **kein Preis-Datum**. WordPress-Metadaten: `date_gmt 2023-11-17`, `modified_gmt 2025-07-17` — Metadaten, kein Preisdatum |
| Erreichbare Händler-Slugs | `he-kelkheim-e-o-entsorgung-betriebe-kelkheim-maintal`, `he-maintal-e-o-entsorgung-betriebe-kelkheim-maintal`, `rp-bingen-e-o-entsorgung` (3) — kein Handler |

**Wörtliche Preisfragmente (verbatim, 39 Zeilen; Auszug):**

> `Schreddervormaterial / Mischschrott leicht` | `160,00 €/Tonne`
> `Scherenvormaterial / Mischschrott` | `170,00 €/Tonne`
> `Mischschrott schwer` | `200,00 €/Tonne`
> `Bremsscheiben` | `260,00 €/Tonne`
> `Stahlfelgen` | `200,00 €/Tonne`
> `Trägerschrott` | `210,00 €/Tonne`
> `Handelsguss` | `210,00 €/Tonne`
> `CU Milberry` | `6.400,00 €/Tonne`
> `CU Leitschienen` | `6.200,00 €/Tonne`
> `CU Schwer Basis 95%` | `5.650,00 €/Tonne`
> `CU Leicht Basis 85%` | `4.650,00 €/Tonne`
> `CU Wicu Raff` | `2.849,00 €/Tonne`
> `CU Kabel 50%` | `2.499,00 €/Tonne`
> `CU Kabel 38%` | `2.050,00 €/Tonne`
> `CU Kabel mit Stecker` | `1.048,00 €/Tonne`
> `Messing schwer` | `3.700,00 €/Tonne`
> `Messing leicht (verunreinigt)` | `1.447,00 €/Tonne`
> `Rotguss Abfälle` | `5.500,00 €/Tonne`
> `MS 58 Abfälle` | `4.050,00 €/Tonne`
> `Alu Profile sauber blank neu` | `1.300,00 €/Tonne`
> `Alu Draht ohne FE` | `1.250,00 €/Tonne`
> `Alu Offsetbleche` | `1.400,00 €/Tonne`
> `Alu neu` | `1.000,00 €/Tonne`
> `Alu "Geschirr" 10%` | `849,00 €/Tonne`
> `Alu-Isoprofile` | `1.000,00 €/Tonne`
> `Alu-Bleche alt / lackiert ohne FE` | `1.000,00 €/Tonne`
> `AlCu Kühler mit FE` | `1.200,00 €/Tonne`
> `Alu-Felgen sauber` | `1.497,00 €/Tonne`
> `Alu-Guss ohne FE` | `850,00 €/Tonne`
> `Alu-Guss mit FE` | `600,00 €/Tonne`
> `Alu-Kabel` | `500,00 €/Tonne`
> `V2A mit Anhaftungen` | `450,00 €/Tonne`
> `Altzink` | `1.200,00 €/Tonne`
> `Altblei` | `1.300,00 €/Tonne`
> `Auswuchtblei` | `320,00 €/Tonne`
> `Bleibatterien` | `500,00 €/Tonne`
> `E-Motoren` | `500,00 €/Tonne`
> `E-Motoren mit Getriebe` | `300,00 €/Tonne`

**Zwei Fallstricke:** (1) Die Quelle schreibt die Sorte **„CU Milberry"** mit einem „l" (Tippfehler der Quelle) — der Parser muss die Schreibweise der Quelle übernehmen, nicht die korrekte. (2) Die öffentlichen HTML-URLs `/ankauf-altmetall_offenbach/` usw. liefern **HTTP 301** auf `/schrott-metalle/` (Frontend abgeschaltet) — der Inhalt lebt nur noch in der DB und ist über die REST-Schnittstelle zu holen.

**Aber:** Das Repo hat diese Quelle bereits! `crates/ingestion/src/traders/handlers/e_o_entsorgung.rs` — ich habe den Handler nicht geändert, aber er existiert. **Kein Neuzugang**, sondern Bestätigung, dass die JSON-Quelle live ist.

---

#### A2.2 `emsschrott.de` — EMS Schrott (3 Slugs)

| Feld | Wert |
|---|---|
| Betreiberdomain | `emsschrott.de` |
| URL | `https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/` |
| Abrufzeitpunkt | 01.11.2026, 03:41 UTC |
| statisch/dynamisch | statisch |
| Einheit | gemischt `€/to` und `€/kg` |
| Datum | **kein Datum** |
| Erreichbare Händler-Slugs | `ni-meppen-emsschrott`, `ni-nordhorn-emsschrott-zweigstelle-nordhorn`, `ni-werlte-emsschrott-zweigstelle-werlte` (3) — kein Handler |

**Wörtliche Preisfragmente (verbatim, Spalten „Sorte | Preis"):**

> `Mischschrott` | `105,- €/to`
> `Messing` | `5,70 €/kg`
> `Alu Geschirr` | `1,06 €/kg`
> `Alu Felgen` | `1,77 €/kg`
> `Blei` | `0,78 €/kg`
> `Kupfer Kabel (ohne Stecker)` | `3,24 €/kg`
> `Kupfer alt (z.B. alte Wasserleitungen, …)` | `9,25 €/kg`

Identisch bestätigt in `research/price-source-screening-single-20261031.md` (dort: „KEIN Datum"). **Kein Neuzugang**, aber weiterhin live und undatiert.

---

#### A2.3 `ne-metalle-gmbh.de` — NE Metalle (2 Slugs)

| Feld | Wert |
|---|---|
| Betreiberdomain | `ne-metalle-gmbh.de` |
| URL | `https://ne-metalle-gmbh.de/` |
| Abrufzeitpunkt | 01.11.2026, 03:41 UTC |
| statisch/dynamisch | statisch |
| Einheit | `€/ Kg` (verbatim, mit Leerzeichen), `€/ Tonne` |
| Datum | **kein Datum** — Überschrift verbatim „Aktueller Auszug unserer Preisliste:" |
| Erreichbare Händler-Slugs | `bb-doberlug-kirchhain-ne-metalle-filiale`, `bb-spremberg-ne-metalle` (2; 1 ohne Handler) |

**Wörtliche Preisfragmente (verbatim):**

> „Aktueller Auszug unserer Preisliste:"
> `Ms (Messing)` | `6,00€ / Kg`
> `Cu (Kupfer)` | `10,00 € - 11,00 € / Kg`
> `Cu-Kabel mind. 38%` | `3,30 € / Kg`
> `Mischschrott` | `140€ / Tonne`
> `Papier` | `0,14 €/ Kg`
> „Gewerbe und Industrie auf Anfrage!"
> „**Bei Überweisung erhöht sich der Preis um 50-100 € / t**"

Die letzte Zeile ist eine **Zahlungsart-Aufschlag** (Zahlungsart ≠ Preisvariante) — im Repomodell als `notes`, nicht als eigener Materialpreis. Die Kupferzeile ist eine **Spannweite** (10,00–11,00 €/Kg), kein Punktpreis.

---

#### A2.4 `madi-schrott.de` — MADI Metall Recycling (2 Slugs)

| Feld | Wert |
|---|---|
| Betreiberdomain | `madi-schrott.de` |
| URL | `https://madi-schrott.de/` |
| Abrufzeitpunkt | 01.11.2026, 03:41 UTC |
| statisch/dynamisch | statisch |
| Einheit | `€/kg` und `€/t` |
| Datum | **kein Preis-Datum**; einzige Datumsangabe im Dokument ist `8.9.2024` (unklarer Bezug, **kein** Preisdatum) |
| Erreichbare Händler-Slugs | `hh-hammerbrook-madi-metall-recycling`, `ni-rosengarten-madi-metall-recycling` (2) — beide bereits durch einen Handler abgedeckt |

**Wörtliche Preisfragmente (verbatim):**

> „Für Eisenschrott zahlen wir **bis zu 220 €/t**, für Mischschrott **bis zu 210 €/t** - je nach Sortenmischung."
> `Kupferschrott` | `bis zu 9,50 €/kg` | „je nach Sorte."
> `Stahlschrott` | `bis zu 220 €/t` | „je nach Sorte."
> `Kabelschrott` | `bis zu 5 €/kg` | „je nach Sorte."
> `Messingschrott` | `bis zu 6 €/kg` | „je nach Sorte."

Wieder durchgängig „bis zu" = Obergrenze.

---

#### A2.5 `altpapierankauf-schulz.de` — Altpapierankauf Schulz (2 Slugs)

| Feld | Wert |
|---|---|
| Betreiberdomain | `altpapierankauf-schulz.de` |
| URL | `https://altpapierankauf-schulz.de/standorte/greifswald/`, `…/standorte/pasewalk/` |
| Abrufzeitpunkt | 01.11.2026, 03:41 UTC |
| statisch/dynamisch | statisch, **je Standort eigene Seite** |
| Einheit | `€/ Kg` und `€/ to` (beide) |
| Datum | **kein Datum** — einziger Preis-Hinweis verbatim: „*Preise können sich jederzeit ändern" |
| Erreichbare Händler-Slugs | `mv-annahmestellen-greifswald-an-altpapierankauf-schulz`, `mv-pasewalk-altpapierankauf-schulz` (2) — kein Handler |

**Wörtliche Preisfragmente (verbatim, Greifswald):**

> `Glanzpapier (Kataloge, Apotheken-, TV-, Frauenzeitschriften, Werbung)` | `0,06 € / Kg` | `0,08 € / Kg` | `60 € / to` | `80 € / to`
> `Pappe (derzeit keine Annahme)` | `0,00 € / Kg` | `0 € / to`
> `Bücher` | `0,03 € / Kg` | `30 € / to`
> `tragbare Altkleider` | `0,00 € / Kg` | `0 € / to`
> `tragbare Schuhe` | `0,00 € / Kg` | `0 € / to`

**Achtung:** Die Glanzpapier-Zeile trägt **zwei** kg- und **zwei** to-Werte in einer Zelle — vermutlich „Privat / Gewerbe" oder „Annahme / Abholung". **Ohne Kopfzeile im Extraht ist die Zuordnung nicht auflösbar** — die Seite muss vor einem Parser-Entwurf zellenweise inspiziert werden. Die `0,00 €`-Zeilen sind **Annahmestopps**, keine Preise.

---

#### A2.6 `emh-metall.de` — EMH Eisen-Metall (2 Slugs)

| Feld | Wert |
|---|---|
| Betreiberdomain | `www.emh-metall.de` |
| URL | `https://www.emh-metall.de/aktuelles` |
| Abrufzeitpunkt | 01.11.2026, 03:41 UTC |
| statisch/dynamisch | statisch |
| Einheit | `€/kg` |
| Datum | **ja, im News-Datum**: `15.06.2026` |
| Erreichbare Händler-Slugs | `sn-chemnitz-mittelbach-emh-eisen-metall-handels-aufbereitungsge`, `sn-lugau-emh-eisen-metall-handels-aufbereitungsge` (2) — kein Handler |

**Wörtliches Preisfragment (verbatim):**

> `Ankauf Altpapier 0,05 €/kg` | `15.06.2026`

**Einordnung:** Das Datum ist ein **News-Datum**, kein „gültig ab"-Gültigkeitsdatum — die Seite ist ein „Aktuelles"-Feed. Formal Preis + Einheit + Datum vorhanden, aber die **Halbwertszeit** des Eintrags ist unbekannt (letzter Eintrag 15.06.2026, Stand heute 01.11.2026 ⇒ ~4,5 Monate alt). Als `published_at` mit `confidence` entsprechend zu gewichten, nicht als tagesaktueller Preis.

---

#### A2.7 `schrott-mohr.de` — Mohr (3 Slugs)

| Feld | Wert |
|---|---|
| Betreiberdomain | `www.schrott-mohr.de` |
| URL | `https://www.schrott-mohr.de/llms.txt` (**LLM-Text-Fassung der FAQ**) |
| Abrufzeitpunkt | 01.11.2026, 03:41 UTC |
| statisch/dynamisch | statisch |
| Einheit | `€ pro kg` |
| Datum | **kein Datum** — verbatim „diese Werte können sich jederzeit ändern" |
| Erreichbare Händler-Slugs | `bw-k-a-mohr-schrotthandel-und-demontage`, `he-wiesbaden-schierstein-mohr-rohstoff-schrotthandel-und-demontag`, `rp-worms-mainz-mohr-schrotthandel-mobil` (3) — kein Handler |

**Wörtliches Preisfragment (verbatim):**

> „Die Preise schwanken stark und richten sich nach dem tagesaktuellen Börsenkurs. Folgende Richtwerte wurden in der FAQ genannt (diese Werte können sich jederzeit ändern):"
> „**Aluminiumschrott:** 0,50–0,80 € pro kg; saubere, sortenrein getrennte Legierungen erzielen höhere Preise"

**Bemerkenswert:** Die Domain liefert unter `/llms.txt` eine **maschinenlesbare Textfassung** der FAQ — ein ungewöhnlich bequemer Scraping-Pfad. Die Werte sind Richtwerte mit Spanne, kein Punktpreis.

---

### A3 — Gebührenquellen (Preis + Einheit, aber **kein** Ankaufpreis)

| Domain | URL | Wörtliches Fragment | Einheit | Datum | Slugs |
|---|---|---|---|---|---|
| `keilich-wetzlar.de` | `https://www.keilich-wetzlar.de/leistungen/ankauf/annahmebedingungen/` | „Es werden die folgenden **Entsorgungskosten** zzgl. der jeweils geltenden Mehrwertsteuer berechnet: \| Verunreinigungen (Schutt) \| 200,00 €/t \| PKW Reifen \| 15,00 €/Stück \| LKW Reifen \| 45,00 €/Stück \| Sonstige Reifen (auch auf Felge) \| 25,00 €/Stück \| Gasflaschen geschlossen \| 220,00 €/Stück \| Acetylenflaschen \| 480,00 €/Stück \| Hohlkörper …" | €/t, €/Stück | **kein Datum** | `he-stadtallendorf-emil-keilich-wetzlar`, `he-wetzlar-emil-keilich-wetzlar` (2) |
| `ac-containerdienst.de` | `https://www.ac-containerdienst.de/schrotthandel-schrottankauf/` | „Abfallsorte (Preise zzgl. 19% Mwst.) Erde / Bodenaushub (ab 35€ / t) …" / „HERBST-ANGEBOT \| Mischabfall nur 225€/t*" | €/t | **kein Datum** | `by-alzenau-jr-recycling`, `by-freigericht-somborn-63579-he-ac-containerdienst` (2) |

Beide sind **Entsorgungs-/Annahmegebühren**, keine Ankaufpreise — für das Schrott-Ankaufpreismodell nicht verwendbar, aber für eine separate Gebühren-Preisart nutzbar.

### A4 — Dynamische Markt-/Spot-Referenzen (kein Händler-Ankaufspreis)

| Domain | URL | Wörtliches Fragment | Einheit | Datum | Slugs |
|---|---|---|---|---|---|
| `hofmann-metall.de` | `https://hofmann-metall.de/` (Anker `<div id="boersenliste">`) | „Marktpreise \| Stahl \|(**BDSV bundesweit - 10/2026**)\| Sorte 1 \| 253,60 €/t \| Sorte 2/8 \| 293,70 €/t \| Sorte 3 \| 287,50 €/t \| Sorte 4 \| 301,10 €/t \| Sorte 5 \| 246,60 €/t" und „Börsenpreise \|(**Vortag: 29.10.2026**)\| EUR/USD \| 1,11911 \| Aluminium \| 3.034,30 $ \| Kupfer \| 14.349,87 $ \| Nickel \| 15.435,94 $ \| Zink \| 4.037,95 $ \| Zinn \| 50.229,97 $" | €/t, $ (USD), EUR/USD | **ja**: „BDSV bundesweit - 10/2026" + „Vortag: 29.10.2026" | `sn-chemnitz-ankauf-von-buntmetallen-stahlschrott`, `sn-klipphausen-sand-und-kiesgruben-soenitz-kleinannahme` (2) |
| `www.ophirum.de` | `https://www.ophirum.de/goldankauf` | „Unsere aktuellen Legierungspreise: 333 Goldlegierung \| 333/1000 \| **30,74 €** \| pro Gramm; 585 Goldlegierung \| 585/1000 \| **53,96 €** \| pro Gramm; 750 Goldlegierung \| 750/1000 \| **69,18 €** \| pro Gramm; 900 Goldlegierung \| 900/1000 \| **85,05 €** \| pro Gramm; 916 Goldlegierung \| 916/1000 \| **86,56 €** \| pro Gramm; 999 Goldlegierung \| 999/1000 \| **102,85 €** \| pro Gramm" / „**Stand: 01.11.2026 04:43:13**" | €/g (pro Gramm) | **ja, sekundengenau und live** (zwei Abrufe 8 s apart: 04:44:10 / 04:44:18) | `hb-bremen-28203-ophirum-bremen-by-goldfuxx`, `he-frankfurt-60311-ophirum`, `he-hanau-63450-goldfuxx-hanau-ophirum` (3) |
| `www.proaurum.de` | `https://www.proaurum.de/shop/preisliste-onlineshop/gold/` und `…/silber/` | „**Basiskurse ab 2026-11-01 03:40:13**" / „**Zuletzt aktualisiert am 01.11.2026 um 04:40:39**" / Tabelle 1: „Goldbarren 1 Gramm Umicore \| 1,00 g \| 109,00 € \| 128,50 €" / Tabelle 2: „Goldmünze 1 Unze Krügerrand aktueller Jahrgang \| 31,10 g \| 3.365,00 € \| 3.660,00 €" / Silber: „Silberbarren 10 Gramm Umicore \| 10,00 g \| 13,10 € \| 29,75 €" | €/Stück (Edelmetall-Handelsware) | **ja, sekundengenau und live** | 11 Slugs (pro aurum) |
| `philoro.de` | `https://philoro.de/preisliste/alle` | 4 Tabellen mit 54/32/26/15 Zeilen, Spalten „Artikel \| Name \| Feingewicht \| Verkaufspreis \| Kaufpreis", z. B. „1991 \| Gold Philharmoniker 1 oz - 2026 \| 31.10 g \| 3.454,94 € \| 3.625,81 €", „2513 \| Goldbarren 250 g gegossen - philoro \| 250.00 g \| 27.329,26 € \| 28.544,89 €" | €/Stück | **nein** — Seite sagt „Die Preise werden alle 3 Minuten aktualisiert" (Wortlaut), aber **kein** Zeitstempel im Dokument | 5 Slugs (philoro) |
| `rheinische-scheidestaette.de` | `https://rheinische-scheidestaette.de/` | „Edelmetall-Kurse \| Silber \| 45,04 \| € \| -0.022 \| % \| Gold \| 3.549,06 \| € \| -0.102 \| % \| Palladium \| 874,66 \| € \| -0.295 \| % \| Platin \| 1.285,59 \| € \| -0.274 \| %" / „Unsere Edelmetallpreise unterliegen u.a. dem Börsenkurs." | €/g | **nein** | 13 Slugs |

**Abgrenzung (verbindlich):** Diese fünf sind **Spot-/Börsenkurse**, keine Händler-Ankaufpreise für Schrott. Das Repo hat dafür bereits eine Präzedenz: `crates/store/src/public/prices.rs` (Zeilen 188 ff.) führt `notes = 'Tageskurs Feingold'` mit `'Spot-Referenz, kein Händler-Ankaufspreis (Feingold)'` und filtert solche Zeilen aus der Preisvergleichslogik. **Pro Aurum** und **Philoro** sind zudem *Verkaufs*preise für Anlagegold/-silber (Handelsware), nicht Ankaufpreise für Schrott — der „Kaufen"-Wert bei Pro Aurum ist der Ankauf, aber für *Investmentgold*, nicht für Altmetall; die Fußnote lautet verbatim „** Ankaufspreis = netto; Kaufpreis = brutto."

**Der eigentliche Fund bei Pro Aurum / Ophirum:** Beide liefern einen **live, sekundengenauen Zeitstempel** („Zuletzt aktualisiert am 01.11.2026 um 04:40:39", „Stand: 01.11.2026 04:43:13") — technisch die **besten** Zeitstempel der ganzen Recherche, aber auf **falscher** Preisart (Spot statt Schrott-Ankauf). Für die 11 Pro-Aurum- und 3 Ophirum-Slugs wäre das ein echter Mehrwert, wenn das Repo Edelmetall-Referenzkurse als eigene Preisart führen will.

---

## B. Negativbefunde

### B.1 Bot-Challenge (technisch gesperrt) — 8 Domains, 44 Slugs betroffen

Alle betroffene Domains liefern **HTTP 200** mit einer **Incapsula/Imperva-Challenge-Seite** (`<title>Challenge Validation</title>`, `function cp_clge_done(){location.reload(true);}`, Challenge-Token-Attribute). Die Erkennung ist **echt**: die Challenge-Seite enthält keinen Betreiberinhalt, und jeder Pfad (auch `/preise`, `/preisliste`, `/downloads`, `/wp-json/…`) liefert dasselbe Challenge-HTML.

| Domain | Betroffene Händler-Slugs | Slugs | konkreter Grund |
|---|---|---|---|
| `tsr-recycling.de` | `hb-industriehafen-tsr-recycling-nl-bremen`, `sn-brand-erbisdorf-tsr-brand-erbisdorf`, `sn-chemnitz-tsr-deutschland-nl-chemnitz`, `sn-dresden-tsr-deutschland-nl-dresden-hafen`, `sn-heidenau-tsr-deutschland-nl-dresden-heidenau`, `sn-riesa-tsr-deutschland-nl-riesa`, `sn-torgau-tsr-deutschland-nl-torgau`, `bw-mannheim-heilbronn-stuttgart-tsr-deutschland`, `bb-brandenburg-an-der-havel-tsr-deutschland`, `bb-groraschen-tsr-recycling-standort-lt-tsr-standortli`, `bb-hennigsdorf-tsr-recycling-standort-lt-tsr-standortli`, `bb-neuruppin-tsr-deutschland`, `bb-schorfheide-tsr-deutschland`, `nw-dortmund-tsr-deutschland-ndl-dortmund`, `nw-duisburg-tsr-deutschland-ndl-duisburg`, `nw-gelsenkirchen-tsr-deutschland-ndl-gelsenkirchen`, `nw-mulheim-an-der-ruhr-tsr-recycling-ndl-mulheim`, `sh-neumunster-tsr-recycling-nl-neumunster`, `be-mitte-tsr-deutschland-westhafen`, `be-neukolln-tsr-deutschland-neukolln`, `st-dessau-rolau-tsr-recycling-niederlassung-dessau`, `st-halle-tsr-recycling-niederlassung-halle`, `st-magdeburg-tsr-recycling-niederlassung-magdeburg`, `st-querfurt-tsr-recycling-standort-querfurt`, `st-strobeck-tsr-recycling-standort-strobeck-halberst`, `he-darmstadt-nord-tsr-deutschland-standort-darmstadt`, `he-dillenburg-tsr-deutschland-standort-dillenburg`, `he-frankfurt-am-main-tsr-deutschland-standort-frankfurt`, `he-freigericht-63579-tsr-deutschland`, `he-ginsheim-gustavsburg-tsr-deutschland-ginsheim-gustavsburg`, `hh-veddel-rothenburgsort-tsr-recycling-nl-hamburg-ne-hovestr-30-h`, `rp-koblenz-tsr-recycling-nl-koblenz`, `ni-bad-essen-tsr-deutschland-bad-essen-bei-osnabruck`, `ni-einbeck-tsr-deutschland-zweigstelle-einbeck`, `ni-emden-tsr-deutschland-standort-emden`, `ni-hannover-tsr-deutschland-lindener-hafen-nordhafen`, `ni-uberregional-tsr-recycling-mehrere-ni-standorte` | **37** | Incapsula-Challenge auf **allen** Pfaden (verifiziert an `/`, `/preise`, `/sitemap.xml`, `/wp-json/wp/v2/media`); `robots.txt` antwortet **HTTP 403** |
| `tsr.eu` | (2 Slugs in der Map, gleiche TSR-Gruppe) | 2 | identische Incapsula-Challenge (`/_eECjdE/...`), Challenge-Pfad pro Request neu |
| `www.remondis.de` | — (die Remondis-Slugs der Domain-Map gehören zu `remondis-entsorgung.de`, `remondis-buedingen.de`, `remondis-rotenburg.de`, `remondis-industrie-service.de`, `remondis-sondershausen.de`, `remondis-rhein-berg.de` — **keine** davon ist `www.remondis.de`) | **0** | Incapsula-Challenge auf der Startseite. Die 4 Veolia-Slugs (`th-eisenberg-veolia-…`, `mv-rostock-18146-srr-recycling`, `bb-frankfurt-veolia-…`, `by-schwabach-bonn-…`) hängen an **`veolia.de`**, das **nicht** blockiert ist. |
| `remondis-entsorgung.de` | `mv-neustrelitz-17235-naw-recycling`, `bb-beelitz-ot-zauchwitz-recycling-center-zauchwitz`, `ni-wolfenbuttel-weibel-2-standorte` (3) | 3 | Incapsula-Challenge auf der Startseite |
| `remondis-buedingen.de` | `he-buedingen-63654-remondis-metallannahme`, `he-langenselbold-63505-remondis-metallannahme` (2) | 2 | Incapsula-Challenge auf der Startseite |
| `www.remondis-ost.de` | — (kein Map-Eintrag für diese Domain; die Remondis-Ost-Slugs liegen bei `remondis-entsorgung.de`) | 0 | Incapsula-Challenge auf der Startseite |

**Konsequenz:** Für die **TSR-Gruppe (39 Slugs)** ist ein Scraper-Durchlauf **nicht** möglich. Die Challenge ist eine **Browser-Interaktion** (`cp_clge_done` → Reload), also **nicht** umgehbar ohne JS-Ausführung — und eine Umgehung war **nicht** Ziel des Auftrags (kein Sperrumgehen). Der Weg ist entweder (a) ein Browser-Renderer, der die Challenge einmalig löst und das Session-Cookie mitnimmt, oder (b) die Preisquelle ist bei TSR ohnehin **nicht öffentlich** — was die Tatsache stützt, dass TSR auf der Branchen-Übersichtsseite üblicherweise gar keine Preise publiziert. **Bis dahin: TSR-Slugs bleiben ohne belegten Ankaufpreis.**

### B.2 Nicht erreichbar (DNS aufgelöst, aber kein HTTP) — 3 Domains

| Domain | Slugs | konkreter Grund |
|---|---|---|
| `milon-recycling.de` | 2 | Startseite **HTTP 500** (Serverfehler) |
| `schrotthandel-forst.de` | 2 | Startseite **HTTP 503** (Service Unavailable) |
| `www.thueringen-recycling.de` | 3 | Verbindungs-Timeout nach 15–20 s (TCP kommt nicht zustande) |

### B.3 HTTP ≥ 400 — 6 Domains

| Domain | Slugs | konkreter Grund |
|---|---|---|
| `mr-online.de` | 3 | Startseite **HTTP 403** (403 in 0,03 s — also serverseitig, kein Bot-Filter; **kein** Incapsula-Challenge-HTML) |
| `www.alba.info` | 4 | Startseite **HTTP 429** (Too Many Requests) — **rate-limitbedingt**, nicht dauerhaft: mit 30 s Abstand und Pause bleibt es bei 429 |
| `sachsen.alba.info` | 6 | **HTTP 429**, gleiche Ursache |
| `metall.alba.info` | 29 | **HTTP 429**, gleiche Ursache — **wichtigster Einzelpunkt: 29 Slugs der ALBA-Gruppe** |
| `schrott-lahib.jimdosite.com` | 2 | **HTTP 403** (Jimdo-Hosting, fremde Domain) |
| `www.riwald.de` | 5 | **Sonderfall:** `https://riwald.de/` scheitert an **TLS-Zertifikatsfehler** („no alternative certificate subject name matches target hostname 'riwald.de'"), `http://riwald.de/` läuft in einen Timeout. **`https://www.riwald.de/` liefert HTTP 200.** Das Zertifikat passt also nur auf den `www.`-Host — **der Screenshot-Befund „nicht erreichbar" wäre falsch.** In der Domain-Map steht die Domain ohne `www.` ⇒ die Zuordnung muss auf `www.riwald.de` laufen. **Preise: keine** — die Startseite trägt keine Zahlenwerte. |

### B.4 DNS-Auflösung schlägt fehl (Domain existiert nicht mehr) — 13 Domains

Verifiziert mit `getent hosts` (kein Resolver-Flake) **und** mit der `www.`-Variante:

| Domain | Slugs | konkreter Grund |
|---|---|---|
| `asdrack.de` | 2 | DNS: „Name or service not known" — auch `www.asdrack.de` unauflösbar |
| `eckwerth-schrott.de` | 2 | DNS unauflösbar (auch www.) |
| `ecmetall.de` | 0 | DNS unauflösbar (auch www.) |
| `eli-stahl.de` | 2 | DNS unauflösbar (auch www.) |
| `ertstoff-dealer.de` | 5 | DNS unauflösbar — **größte betroffene Gruppe unter den Toten** |
| `egrogmbh.de` / `www.egrogmbh.de` | 3 | DNS unauflösbar (beide Hostformen) |
| `il-ro.de` / `www.il-ro.de` | 3 | DNS unauflösbar (beide Hostformen) |
| `ittmann-recycling.de` | 2 | DNS unauflösbar (auch www.) |
| `rm-schrotthandel.de` | 2 | DNS unauflösbar (auch www.) |
| `www.prehans.com` | 0 | DNS unauflösbar |
| `www.rhr-recycling.de` | 0 | DNS unauflösbar |
| `www.scho-ferro-metall.de` | 0 | DNS unauflösbar |

**Wichtig für den Auftrag:** Der Grund ist **nicht** „Seite weg, Inhalt verloren" — die **Preisquelle** ist bei diesen Betreibern nie zutage gekommen, und die **Dossiers** dieser Slugs haben jetzt eine belastbare Negativaussage: „Domain existiert nicht mehr (DNS NXDOMAIN)". Das ist eine **Aufwertung** des `website_status` von `unbekannt` auf `tot` — mit Ausnahme, dass die Domain-Map-Einträge erst geprüft werden müssen.

### B.5 Erreichbar, aber keine Preisquelle (größte Gruppe)

**191 Domains** mit zusammen **652 Slugs** sind erreichbar (HTTP 200), tragen aber **keine** Preiszeile. Die drei größten:

| Domain | Slugs | konkreter Befund |
|---|---|---|
| `scholz-recycling.com` | **48** | Startseite HTTP 200, `robots.txt` 200, `sitemap.xml` 200 — **keine Preisliste im Sitemap**, kein Preisindikator auf der Startseite, kein Preis-Treffer in 130 Kandidatenpfaden. **Die größte Einzelgruppe des Screenings und die mit den meisten Slugs — und sie hat keine öffentliche Preisquelle.** |
| `metall.alba.info` | **29** | **HTTP 429** (rate-limitbedingt). Vor der Sperre war die Seite erreichbar und enthielt **keine** Preiszeile: `/ankauf/sorten/` (102 KB) enthält **null** Preisangaben, nur „Preise ✓ Hier informieren" und Fließtext; es gibt Sorten-URLs, aber keine Zahlen. **Selbst ohne 429 hätte die Domain keine Ankaufpreisquelle geliefert.** |
| `rheinische-scheidestaette.de` | 13 | Nur Edelmetall-Spot-Kurse (→ A3), **keine** Schrott-Ankaufpreise |
| `isr-recycling.de` | 11 | Nur AGB-/Zertifikat-PDFs (`/images/pdf/agb-isr.pdf` usw.), **keine** Preisliste. Die 47 Marker-Treffer des Screenings sind **ausschließlich** Zertifikats-/AGB-Seiten — kein einziger Preiswert. |
| `proaurum.de` | 11 | Edelmetall-Shop-Preisliste (→ A3), **keine** Schrottpreise |
| `steil.de` | 10 | `abfallsorten` listet `Altpapierhandel`, `Sonderabfallentsorgung` — **null** Zahlenwerte |
| `schrott24.de` | 9 | Die Preistabelle ist **clientseitig gerendert** (React, `priceTable-module--ListOfPrices--413cc`): der serverseitige HTML enthält **null** Euro-Zeichen und **null** Preiszeilen. Das Preismodell steckt im Metaloop-B2B-Portal (`https://www.metaloop.com/pro?lng=de`) bzw. im B2B-Bereich hinter **Login** („Login", „Einkauf", „Verkauf", „Login") — **für Dritte nicht abrufbar**. Die 15 Preis-Seiten (`/schrottpreise/`, `/kupferpreis/`, `/eisenpreise/`, …) liefern alle denselben JS-Shell (432 KB, 0 Preiszeilen). |
| `jakob-becker.de` | 7 | `/downloads/` enthält nur EFB-/Zertifikats-PDFs, **keine** Preisliste |
| `nisa.alba.info` / `nord.alba.info` | 14 | ALBA-Gruppe, gleiche 429-Sperre, keine Preise |

**Zusatzbefund zu den ALBA-Domains:** `www.alba.info`, `sachsen.alba.info`, `metall.alba.info` liefern alle **HTTP 429** — das ist eine **anbieterseitige Rate-Limit-Sperre gegen automatisierte Abrufe** (alle drei Antworten in ~2,1 s mit identischem Code). **Kein** Scraper-Bypass wurde versucht; das ist eine bewusste Betreiberentscheidung, die respektiert wird.

---

## C. Nachprüfung der zwei bekannten Treffer

| | `dein-schrottplatz.de/pages/preise.php` | `elno-container.de`-PDF-Serie |
|---|---|---|
| **Noch abrufbar?** | **Ja** — HTTP 200, 66 Tabellenzeilen, Inhalt identisch zum Screenings-Befund vom 30./31.10. | **Ja** — HTTP 200, 210.166 Bytes, **MD5-identisch** zum Repo-Fixture `elno-20261017.pdf` (`23de902cb23595d5b808ae643bc47eee`) |
| **Noch gültig?** | **Ja** — Gültigkeitsdatum **„Gültig ab 03.10.2026"**, also 29 Tage alt bei Abruf am 01.11.2026. Die Liste ist damit im Gültigkeitsfenster. | **Ja** — Dateiname `Preisliste_16.10.pdf`, WP-Pfad `/2026/10/`, PDF-`CreationDate D:20261016131959+02'00'`, HTTP `Last-Modified: Fri, 16 Oct 2026 11:20:19 GMT`. Stand 16.10.2026, 16 Tage alt. **Die PDF trägt das Datum nicht im Text** — die Drei-Ebenen-Redundanz außerhalb des PDFs ist der Grund, warum sie trotzdem datierbar ist. |
| **Für welche Slugs nutzbar?** | `sn-rochlitz-db-recycling-denis-blum` (1) — **bereits durch `db_recycling.rs` abgedeckt**, Handler läuft, Datum wird dynamisch geparst | `be-spandau-elno-container-und-dienstleistungs` (1) — **bereits durch `elno.rs` abgedeckt** |
| **Neue Erkenntnisse gegenüber dem bekannten Befund** | **Keine Preisänderung.** Aber: die Repo-**Dokumentation** ist veraltet — `db_recycling.rs` Zeile 7 und Zeile 365 nennen als Beispiel- und Kommentar-Datum **„Gültig ab 09.09.2026"**, live steht **„Gültig ab 03.10.2026"**. Der Code ist **nicht** betroffen (er parst `label.to_lowercase().starts_with("gültig ab")` dynamisch, Zeile 367 ff.). **Kommentar-Drift, kein Fehler.** | **(a)** Es existiert eine **zweite Live-URL mit HTML statt PDF**: `https://www.elno-container.de/preisliste_16-10/` → HTTP 200, gleicher Inhalt, gleicher `Last-Modified`. Für `elno.rs`, das derzeit den PDF-URL aus dem Homepage-Anker ableitet (`pdf_url()`, Zeile 46 ff.), ist das eine **Robustheitsoption ohne Scraping-Risiko**. **(b)** Die Historie ist **nicht** per URL erratbar: `/wp-content/uploads/2026/` → HTTP 403, `wp-sitemap.xml` → nur `page-sitemap.xml`, `Preisliste_15.09.pdf` → HTTP 404. **(c)** Die Repo-Fixture-Datei `elno-20261017.md` dokumentiert bereits, dass die Oktober-Liste die September-Liste ersetzt hat und dass das PDF „No printed publication date" hat — **das ist bestätigt**. |

**Gesamtbewertung C:** Beide bekannten Treffer sind **unverändert live und gültig**. Es gibt **keinen** Grund, sie neu zu recherchieren, und **keinen** Anlass für eine Dossier-Änderung. Die einzigen offenen Punkte sind zwei **Kommentar-/Doku-Drift**-Hinweise (`db_recycling.rs` nennt das alte Datum; `mc_schrott.rs` nennt „Stand: 22.09.2026" statt live „Stand: 30.10.2026") — beide **nicht** angefasst, weil der Auftrag Dossier-/Code-Änderungen ausschließt.

---

## D. Empfehlung — die nächsten 10–20 Dossiers mit belegbarem Ankaufpreis

**Auswahlkriterium:** Der Preis muss (a) auf einer **live erreichbaren** Seite des Betreibers stehen, (b) **Wert + Einheit** enthalten, (c) **materialseitig** zum Schrott-/Altmetall-Geschäft des Dossiers passen, und (d) mit **restlichen Unsicherheiten** (Datum, Staffel, Preisart) bewertbar sein. „Belegbar" heißt hier: ein belegter **Preisstandort**, nicht zwingend ein sofort buchbarer `exact`-Wert.

### D.1 — Priorität 1: Preis + Einheit, **kein Handler vorhanden** (8 Dossiers)

| # | Dossier-Slug | Domain | Preisquelle | Warum belegbar |
|---|---|---|---|---|
| 1 | `nw-bonn-pur-umwelt-p-u-richter` | `pur-umwelt.com` | `https://pur-umwelt.com/schrottpreise/` | **Vollständig eligible:** 19 Sorten in €/t, „gültig ab: 26.10.2026, 14:30 Uhr". Mengenregeln (10 % Abzug < 200 kg, erst ab > 100 kg für Blech/Mischschrott, > 500 kg Anfrage) sind **publiziert** und damit buchbar. Einziger Preisstandort: Bonn-Bad Godesberg. |
| 2 | `ni-hannover-p-u-richter-umweltdienste-rheinland` | `pur-umwelt.com` | `https://pur-umwelt.com/schrottpreise/` | Gleiche Liste. **Vor dem Buchen prüfen**, ob der Hannover-Standort (Anschrift im Dossier) die Bonner Liste mitträgt — die Liste ist mit „frei Friesdorfer Straße 176 in 53175 Bonn-Bad Godesberg" **standortbezogen**. Trägt der Hannover-Standort sie nicht, ist das Dossier nur über die Gruppenliste erreichbar. |
| 3 | `be-pankow-kiro-schrotthandel-kafedzhiev` | `schrotthandel-berlin.com` | `https://schrotthandel-berlin.com/` + `/arten-von-altmetall/*/` | Zahlen + Einheit + **Mengenstaffel** („Ab 750 kg: …"). **Aber ohne Datum**, und die Startseite nennt **andere** Werte als die Sorten-Seiten. Der Widerspruch muss vor dem Buchen aufgelöst werden (welche Liste ist aktuell?). |
| 4 | `be-pankow-schrottmeister-altmetallhandel` | `schrotthandel-berlin.com` | dito | Gleiche Quelle, gleiche Vorbehalte. Dossier „Schrottmeister" — die Domain ist „Kiro"; die Identität zwischen beiden Dossiers und der Domain ist **nicht** belegt, das muss beim Anfassen mitgeprüft werden. |
| 5 | `hb-bremen-schrottjungs-bremen` | `schrottjungs.de` | `https://schrottjungs.de/gewerbe/` | 8 Slugs, eine Liste, **ein** Preiswerk für alle. 8 FE-Sorten + 12 NE-Sorten mit Einheit. „bis zu" ⇒ als Obergrenze buchen. **Kein Datum** ⇒ `published_at: None` oder `confidence` niedrig. **Beste Multi-Slug-Qualität im Set.** |
| 6 | `ni-hannover-schrottjungs-abholung` | `schrottjungs.de` | dito | dito |
| 7 | `sl-saarbrucken-umland-schrottjungs-mobil` | `schrottjungs.de` | dito | dito |
| 8 | `hh-hamburgweit-mobile-abholdienste-schrottjungs-schrott` | `schrottjungs.de` | dito | dito |

**Von diesen 8 ist nur #1 und #2 (beide `pur-umwelt.com`) sofort als `exact` buchbar** — dort sind Datum, Einheit, Sorte und Mengenregeln gleichzeitig belegt. #3–#8 brauchen mindestens eine Klärung (Widerspruch bzw. fehlendes Datum), bevor ein Wert in die Datenbank wandert.

### D.2 — Priorität 2: Preis + Einheit vorhanden, kein Handler, Datum nachrüstbar (9 Dossiers)

| # | Dossier-Slug | Domain | Preisquelle | Was fehlt |
|---|---|---|---|---|
| 9 | `he-kelkheim-e-o-entsorgung-betriebe-kelkheim-maintal` | `eo-entsorgung.de` | `https://eo-entsorgung.de/wp-json/wp/v2/pages?slug=ankauf-altmetall_offenbach` | 39 Sorten in €/Tonne, JSON. **Kein Preis-Datum.** Repo-Handler `e_o_entsorgung.rs` existiert bereits — das Dossier braucht den Preis nur **zugeordnet**, nicht erfunden. |
| 10 | `he-maintal-e-o-entsorgung-betriebe-kelkheim-maintal` | `eo-entsorgung.de` | dito | dito |
| 11 | `rp-bingen-e-o-entsorgung` | `eo-entsorgung.de` | dito | dito |
| 12 | `ni-meppen-emsschrott` | `emsschrott.de` | `https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/` | 7 Sorten, €/to + €/kg. **Kein Datum.** Bestätigt seit dem 31.10. unverändert live. |
| 13 | `ni-nordhorn-emsschrott-zweigstelle-nordhorn` | `emsschrott.de` | dito | dito — Zweigstelle desselben Betriebs, eine Liste. |
| 14 | `ni-werlte-emsschrott-zweigstelle-werlte` | `emsschrott.de` | dito | dito |
| 15 | `mv-annahmestellen-greifswald-an-altpapierankauf-schulz` | `altpapierankauf-schulz.de` | `https://altpapierankauf-schulz.de/standorte/greifswald/` | Nur **Altpapier/Textilien** — passt materialseitig nur, wenn das Dossier Altpapier ankauft. **Vorab prüfen**, ob die Doppelspalte (zwei kg- und zwei to-Werte) zwei Kundenarten sind. |
| 16 | `mv-pasewalk-altpapierankauf-schulz` | `altpapierankauf-schulz.de` | `https://altpapierankauf-schulz.de/standorte/pasewalk/` | dito |
| 17 | `sn-lugau-emh-eisen-metall-handels-aufbereitungsge` | `emh-metall.de` | `https://www.emh-metall.de/aktuelles` | „Ankauf Altpapier 0,05 €/kg" mit News-Datum 15.06.2026. Nur Altpapier, und ~4,5 Monate alt ⇒ `confidence` entsprechend. |

### D.3 — Priorität 3: belegbar, aber Preis ist Referenz/Teilmenge (6 Dossiers)

| # | Dossier-Slug | Domain | Was zu erwarten ist |
|---|---|---|---|
| 18 | `sn-chemnitz-ankauf-von-buntmetallen-stahlschrott` | `hofmann-metall.de` | BDSV-Marktpreise **mit** Datum („BDSV bundesweit - 10/2026", „Vortag: 29.10.2026"). **Kein Händler-Ankaufspreis** — als Referenz im Repomodell führen, nicht als Ankaufpreis buchen (Präzedenz `prices.rs` Z. 188 ff.). |
| 19 | `sn-klipphausen-sand-und-kiesgruben-soenitz-kleinannahme` | `hofmann-metall.de` | dito |
| 20 | `he-frankfurt-60311-ophirum` | `www.ophirum.de` | **Live, sekundengenauer Zeitstempel** („Stand: 01.11.2026 04:43:13") + 6 Legierungspreise in €/g. Beste Zeitstempelqualität der Recherche — aber Spot/Referenz, kein Schrott-Ankauf. |
| 21 | `he-hanau-63450-goldfuxx-hanau-ophirum` | `www.ophirum.de` | dito |
| 22 | `hb-bremen-28203-ophirum-bremen-by-goldfuxx` | `www.ophirum.de` | dito |
| 23 | `he-frankfurt-60313-philoro-edelmetalle` | `philoro.de` | „Kaufpreis"-Spalte der Preisliste, kein Zeitstempel im Dokument (nur „Die Preise werden alle 3 Minuten aktualisiert"). Ohne Datum **nicht** eligible. |

### D.4 — ausdrücklich **nicht** empfohlen

* **Die 39 TSR-Slugs** (`tsr-recycling.de`, `tsr.eu`): Incapsula-Challenge auf allen Pfaden, keine Browser-Interaktion im Auftrag. Ohne Renderer oder ohne öffentliche Preisquelle kein belegbarer Preis. **Nicht** durch eine Schätzung ersetzen.
* **Die 48 Scholz-Slugs** (`scholz-recycling.com`): keine Preisquelle, 130 Kandidatenpfade geprüft, Sitemap ohne Preis-URL. Ein belegbarer Preis existiert dort derzeit nicht.
* **Die 29 ALBA-Metall-Slugs** (`metall.alba.info`): HTTP 429 (anbieterseitige Rate-Limit-Sperre) — und selbst ohne die Sperre enthält `/ankauf/sorten/` keine Zahlenwerte.
* **Die 5 TSR-/Remondis-DNS-Toten** (13 Domains, 22 Slugs): Domain existiert nicht mehr. Hier ist die richtige Aktion **nicht** ein Preis, sondern eine belegte Negativaussage (`website_status: tot`).
* **`riwald.de` (5 Slugs):** „nicht erreichbar" wäre **falsch** — `https://www.riwald.de/` liefert HTTP 200. Nur: **keine** Preise. Richtige Aktion: Domain auf `www.` korrigieren, Preis bleibt offen.

---

## Anhang — Reproduzierbarkeit

* **Skript:** `/tmp/opencode/screen.py` (Abruf + Challenge-Erkennung + Preis-Extraktion), `/tmp/opencode/from_cache.py` (Auswertung ohne Netz, aus dem HTTP-Cache).
* **Cache:** `/tmp/opencode/cache` — 15.385 HTTP-Antworten (gzip + Meta-JSON mit Status/Headern/Abrufzeitstempel).
* **Rohdaten:** `/tmp/opencode/screendata.json` — 243 Domain-Datensätze mit allen Kandidatenpfaden, Statuscodes, extrahierten Preiszeilen, Datums-Tokens und Challenge-Treffern.
* **Domain→Slug-Map:** `/tmp/opencode/domain_map.txt` (1.923 Domains, 2.342 Slugs), abgeleitet aus den Repo-Dossiers.
* **Abrufzeitpunkte:** alle HTTP-Abrufe am **01.11.2026** zwischen 03:35 und 04:20 UTC (der Zeitstempel steht je Cache-Eintrag in der `.meta`-Datei unter `ts`).
* **Keine Dossier-Änderung, kein Deploy, kein Produktions-Write, keine Git-Befehle.**
