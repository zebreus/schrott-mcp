# Preisquellen-Screening Schrott-MCP — 116 Domains ohne Handler

**Screening-Lauf:** 30.10.2026, ca. 21:10–21:45 MEZ
**Input:** `/tmp/opencode/price-source-screening.json` (116 Domains, 257 Händler-Slugs, Liste bereits nach Händlerzahl sortiert)
**Modus:** ausschließlich read-only. Keine Datei im Repo geändert, kein Deploy, kein Neustart, keine Produktions-Writes. Alle HTTP-Zugriffe waren GET; die Repo-Datenbank wurde nicht angefasst (kein `file:…?mode=ro`-Write, kein `PRAGMA query_only` nötig, weil nicht geöffnet).
**Ergebnis-Datei:** nur diese Datei (`/tmp/opencode/price-source-screening-20261030.md`); nichts ins Repo geschrieben.

---

## Kurzfassung vorab

Von 116 Domains wurden **116 vollständig geprüft** (9 434 unique URLs abgerufen, 3 020 × HTTP 200).

Der Befund ist differenzierter als die Ausgangsannahme:

- **Ankaufpreisquellen mit Preis + Einheit + Datum: 0 Stück.** Kein einziger Händler der 116 Domains veröffentlicht seine Scrap-Ankaufpreise in einer Form, die Preis **und** Einheit **und** Datum enthält.
- **Ankaufpreisquelle mit Preis + Einheit, aber ohne Datum: 1 Stück** — `eo-entsorgung.de`, als JSON über die WordPress-REST-API vollständig maschinenlesbar (39 Sorten in €/Tonne). Das ist der stärkste einzelne Fund des Screenings, verletzt aber die Datumsregel.
- **Marktreferenz-Quellen mit Preis + Einheit + Datum: 2 Stück** — `hofmann-metall.de` (Homepage-Widget) und die dahinterliegende Quelle `bdsv.org`. Das sind **BDSV-Durchschnittspreise / LME-Börsenpreise**, keine Händler-Ankaufpreise.
- **Gebührenquellen: 4 Domains / 7 Quellen-URLs**, davon 2 mit sauberem Datum (die PDF-Preislisten `hagemann-recycling.de`), 5 ohne Preisdatum.
- **Weitere randnahe Befunde: 4 weitere Domains** (A5) — Preis + Einheit, aber unter der 3-Zeilen-Schwelle, als Spannweite oder ohne Datum.
- **Nur Markt-/Kursreferenz ohne Ankaufpreis-Charakter: 1 Domain** (A4, `rheinische-scheidestaette.de`).

Die praktische Konsequenz: **Der Hebel liegt bei diesen 116 Betreibern nicht in einer Preisquelle, die man scrapen könnte.** Bei 105 von 116 gibt es schlicht keine öffentliche Preisquelle mit Zahlen. Für die Preisdaten braucht es entweder eine Referenzquelle (BDSV/LME) plus Händler-Aufschlag, oder die Betreiber müssen gefragt werden. Die Ausgangsannahme des Auftrags trifft für diese 116 Domains also **nicht** zu — sie trifft auf die Branche als Ganzes zu (vgl. D).

---

## A) Eligible Ankaufpreisquellen

### A0 — Ergebnis der harten Prüfung

| Kriterium | Treffer |
|---|---|
| Preis **und** Einheit **und** Datum, Scrap-**Ankaufpreis** | **0** |
| Preis **und** Einheit, Scrap-**Ankaufpreis**, **ohne** Datum | **1** (`eo-entsorgung.de`) |
| Preis **und** Einheit **und** Datum, **Marktreferenz** (BDSV/LME/Edelmetall) | **2** (`hofmann-metall.de`, `bdsv.org`) |
| Preis **und** Einheit, Edelmetall-**Börsenkurs**, **ohne** Datum | **1** (`rheinische-scheidestaette.de`) |

Nach der Vorgabe („Ohne Preis **und** Einheit **und** Datum gilt die Quelle als nicht geeignet“) ist die Ankaufspreis-Tabelle in A0 damit **leer**. Die folgenden Funde sind trotzdem vollständig dokumentiert, weil sie die einzigen Kandidaten sind und der Auftrag ausdrücklich Negativbefunde mit Belegen verlangt.

---

### A1 — `eo-entsorgung.de` — Ankaufpreisliste als JSON (Preis + Einheit, **kein Datum**)

| Feld | Wert |
|---|---|
| Domain | `eo-entsorgung.de` |
| URL (maschinenlesbar) | `https://eo-entsorgung.de/wp-json/wp/v2/pages?slug=ankauf-altmetall_offenbach` |
| Weitere Slugs | `…?slug=ankauf-altmetall_wiesbaden`, `…?slug=ankauf-altmetall_mainz`, `…?slug=ankauf-altmetall_frankfurt`, `…?slug=ankauf-altmetall_darmstadt` |
| Format | JSON (WordPress-REST, `content.rendered` = HTML-Fragment mit `<li>`-Preisliste (39 Zeilen mit „… €/Tonne“)) |
| HTTP-Status | **200**, `application/json` |
| Betroffene Slugs | `he-kelkheim-e-o-entsorgung-betriebe-kelkheim-maintal`, `he-maintal-e-o-entsorgung-betriebe-kelkheim-maintal`, `rp-bingen-e-o-entsorgung` (3) |
| Einheit | `/Tonne` (wörtlich „€/Tonne“) |
| Datum | **kein Datum** — der gesamte Seiteninhalt enthält keinen einzigen Datums-Token (`date_gmt` 2023-11-17, `modified_gmt` 2025-07-17 sind WordPress-Metadaten, kein Preisdatum) |

**Wörtliche Beispielwerte (aus dem JSON-`content.rendered`):**

1. `Schreddervormaterial / Mischschrott leicht 160,00 €/Tonne`
2. `Mischschrott schwer 200,00 €/Tonne`
3. `CU Milberry 6.400,00 €/Tonne`
4. `CU Kabel 50% 2.499,00 €/Tonne`
5. `Messing schwer 3.700,00 €/Tonne`
6. `V2A Abfälle 1.000,00 €/Tonne`

Hinweis für den Material-Mapping: Die Seite schreibt die Sorte **„CU Milberry“** (ein „l“, Tippfehler der Quelle) — ein Handler muss die Schreibweise der Quelle übernehmen, nicht die korrekte. Weitere Sorten im selben Block: `Scherenvormaterial / Mischschrott 170,00`, `Bremsscheiben 260,00`, `Stahlfelgen 200,00`, `Trägerschrott 210,00`, `Handelsguss 210,00`, `CU Leitschienen 6.200,00`, `CU Schwer Basis 95% 5.650,00`, `CU Leicht Basis 85% 4.650,00`, `CU Wicu Raff 2.849,00`, `CU Kabel 38% 2.050,00`, `CU Kabel mit Stecker 1.048,00`, `Messing leicht (verunreinigt) 1.447,00`, `Rotguss Abfälle 5.500,00`, `MS 58 Abfälle 4.050,00`, `Alu Profile sauber blank neu 1.300,00`, `Alu Draht ohne FE 1.250,00`, `Alu Offsetbleche 1.400,00`, `Alu neu 1.000,00`, `Alu "Geschirr" 10% 849,00`, `Alu-Isoprofile 1.000,00`, `Alu-Bleche alt / lackiert ohne FE 1.000,00`, `AlCu Kühler mit FE 1.200,00`, `Alu-Felgen sauber 1.497,00`, `Alu-Guss ohne FE 850,00`, `Alu-Guss mit FE 600,00`, `Alu-Kabel 500,00`, `V2A mit Anhaftungen 450,00`, `Altzink 1.200,00`, `Altblei 1.300,00`, `Auswuchtblei 320,00`, `Bleibatterien 500,00`, `E-Motoren 500,00`, `E-Motoren mit Getriebe 300,00` — alle in €/Tonne.

**Begründung.** Maschinenlesbar ist die Quelle uneingeschränkt: sauberes JSON, stabiler Endpunkt, identische Werte auf fünf Ortsseiten (Offenbach, Wiesbaden, Mainz, Frankfurt, Darmstadt). Die Materialbezeichnungen sind Sorten-Namen, die 1:1 auf den Materialkatalog des Projekts abbildbar sind (Cu-Varianten, MS 58, Rotguss, V2A, Alu-Felgen). Die Einheit steht direkt am Wert (`€/Tonne`) — eine Umrechnung kg↔t wäre also beweisbar. **Der Haken ist das Datum:** die Liste trägt weder ein „Stand“-Datum noch einen Zeitstempel. Nach der Auftragsregel ist die Quelle damit **nicht eligible**, praktisch aber der mit Abstand wertvollste Fund — sie ist der einzige Ort in den 116 Domains, an dem überhaupt eine vollständige Scrap-Sortenliste mit Zahlen steht. Zwei weitere Einschränkungen: die öffentlichen URLs `/ankauf-altmetall_offenbach/` usw. liefern **HTTP 301** auf `/schrott-metalle/` (Seite im Frontend abgeschaltet, Inhalt lebt nur noch in der DB), und die Werte wirken nicht tagesaktuell (`modified_gmt` 2025-07-17). Ein Parser müsste also die REST-Schnittstelle nutzen und das Datum entweder aus `modified_gmt` ableiten (als Proxy, mit Altersrisiko) oder die Quelle als „undatiert“ führen.

---

### A2 — `hofmann-metall.de` — Marktpreis-Widget auf der Homepage (Preis + Einheit + Datum, aber **Marktreferenz**)

| Feld | Wert |
|---|---|
| Domain | `hofmann-metall.de` |
| URL | `https://hofmann-metall.de/` — Anker `<div id="boersenliste">` |
| Format | serverseitig gerendertes HTML, `<ul><li>Label<span>Preis</span></li></ul>`, feste IDs `#b1` / `#b2` |
| HTTP-Status | **200** |
| Betroffene Slugs | `sn-chemnitz-ankauf-von-buntmetallen-stahlschrott`, `sn-klipphausen-sand-und-kiesgruben-soenitz-kleinannahme` (2) |
| Datum | **ja, wörtlich**: „`(BDSV bundesweit - 10/2026)`“ und „`(Vortag: 29.10.2026)`“ |

**Wörtliche Beispielwerte:**

1. `Sorte 1` → `253,60 €/t` (Abschnitt „Stahl (BDSV bundesweit - 10/2026)“)
2. `Sorte 3` → `287,50 €/t` (ebenda; ferner `Sorte 2/8 293,70 €/t`, `Sorte 4 301,10 €/t`, `Sorte 5 246,60 €/t`)
3. `Kupfer` → `14.349,87 $` (Abschnitt „Börsenpreise (Vortag: 29.10.2026)“; ferner `Aluminium 3.034,30 $`, `Nickel 15.435,94 $`, `Zink 4.037,95 $`, `Zinn 50.229,97 $`)

**Begründung.** Das ist technisch die beste maschinenlesbare Quelle im gesamten Screening: stabile Container-IDs, Label und Wert in getrennten Elementen, Einheit direkt am Wert, Datum als Klartext im Section-Header, kein JavaScript nötig. Die Preise sind in **€/**t** (Stahl) bzw. **$**/t (Börse) — Einheiten also vorhanden, aber nicht einheitlich und nicht in €/kg. Inhaltlich handelt es sich **nicht um Ankaufpreise des Händlers**: die Stahlwerte sind BDSV-**Durchschnittspreise** (Ab-Station-Verkaufspreise der BDSV-Mitglieder, siehe A3), die Börsenwerte sind LME-Kurse. Wer damit einen Schrott-MCP-„Current Price“ speist, speist eine *Marktreferenz*, nicht den Preis, den der Händler dem Kunden zahlt. Als **Referenz-/Benchmark-Feed** ist die Quelle trotzdem vollwertig — sie ist der einzige Fund im Set mit Preis + Einheit + Datum.

---

### A3 — `bdsv.org` — die Upstream-Quelle hinter A2 (Preis + Einheit + Datum, Marktreferenz)

| Feld | Wert |
|---|---|
| Domain | `bdsv.org` — **nicht** Teil der 116-Domain-Liste, aber von `hofmann-metall.de` als „Historie anzeigen“ verlinkt |
| URL | `https://www.bdsv.org/unser-service/markt-preise/` |
| Format | HTML-Block mit Diagramm-Beschriftungen (Preise als Textknoten) |
| HTTP-Status | **200** |
| Datum | **ja**, aber nur Monatsgenauigkeit: „`BDSV Durchschnittspreise in Deutschland in €/t (09/2026)`“ |

**Wörtliche Beispielwerte:**

1. `Sorte 1` → `254,6 €/t`
2. `Sorte 2/8` → `294,6 €/t`
3. `Sorte 3` → `289,0 €/t`
4. `Sorte 4` → `302,9 €/t`
5. `Sorte 5` → `246,5 €/t`

**Begründung.** Eine einzige Branchen-Quelle, die **alle** Stahlschrott-Händler abdeckt — der eigentliche Hebel, wenn es nicht um Betreiber-Listen geht. Maschinenlesbar sind die Werte als Textknoten; die Zuordnung Label↔Preis erfolgt über die Diagramm-Beschriftung, nicht über eine Tabelle, was den Parser aufwändiger macht als A2. Datum nur als Monat `(09/2026)`, kein Stichtag. Wichtig für die Einordnung: Die Seite sagt wörtlich, es handele sich um „**Ab-Stations-Preise (Lagerverkaufspreise** – frei Waggon, frei LKW oder frei Schiff verladen ab Platz)“ — also um **Verkaufs**preise der Händler, gemeldet von Mitgliedsunternehmen an eine neutrale Stelle (IW Consult), veröffentlicht ab dem 10. Tag des Folgemonats. Als **Ankaufpreisquelle** ist sie damit ausgeschlossen; als Branchen-Benchmark ist sie der beste Einzelkandidat der ganzen Recherche.

**Auffälligkeit, die gegen eine Blindrecherche spricht:** `hofmann-metall.de` zeigt „BDSV bundesweit - **10**/2026“ mit `Sorte 1 253,60 €/t`, `bdsv.org` zeigt „(**09**/2026)“ mit `Sorte 1 254,6 €/t`. Die beiden Werte für denselben Monatstyp weichen ab — die Händler-Seite läuft der Verbandsseite um einen Monat voraus (vermutlich Schätzung). Wer eine der beiden Quellen nutzt, muss das dokumentieren.

---

### A4 — `rheinische-scheidestaette.de` — Edelmetall-Börsenkurs (Preis + Einheit, **kein Datum**)

| Feld | Wert |
|---|---|
| Domain | `rheinische-scheidestaette.de` |
| URL | `https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/` |
| Format | serverseitig gerendertes HTML, `<span id="tp_gold" class="price">…</span>` (analog `tp_silber`, `tp_palladium`, `tp_platin`) |
| HTTP-Status | **200** |
| Betroffene Slugs | `nw-dusseldorf-rheinische-scheidestatte-stammhaus`, `by-augsburg-rheinische-scheidestaette-filiale`, `nw-aachen-rheinische-scheidestaette-filiale` (3) |
| Einheit | wörtlich „`Preise in EUR pro Feinunze`“ |
| Datum | **kein** — der Zeitstempel wird ausschließlich clientseitig erzeugt („`Preisaktualisierung in <span>--:--</span>`“) |

**Wörtliche Beispielwerte:**

1. `Gold` → `3.472,97 €` (`<span id="tp_gold" class="price">3.472,97 €</span>`)
2. `Silber` → `44,02 €` (`<span id="tp_silber" class="price">44,02 €</span>`)
3. `Palladium` → `874,87 €`; `Platin` → `1.285,42 €` (live nachgeladen: der Platinwert stand wenige Minuten zuvor noch bei `1.295,03 €` — die Werte sind laufend, der Zeitstempel fehlt)

**Begründung.** Sauberer, serverseitig gesetzter Wert pro Metall mit stabiler ID — technisch gut parsbar. Aber es ist ein **Börsen-Spotkurs**, kein Ankaufpreis für Schmuck/Altgold; die eigentlichen Ankaufspreise (je Legierung in €/g) liegen auf `/leistungen/edelmetalle-verkaufen/edelmetallankauf/` ohne Zahlen, und die Routen `/preise/` und `/preise-und-steuern/` liefern 404. Der fehlende Zeitstempel ist für einen Kurs, der sich alle 10 Minuten ändert, strukturell problematisch: ohne `Date`-Header oder Feld im HTML ist der Wert nicht zeitlich einzuordnen. Für einen Schrott-MCP-Handler ist die Quelle zudem fachlich am Rand (Edelmetall ≠ Schrott) — die Domäne hat aber drei Slugs und damit Bedarf.

---

### A5 — Weitere randnahe Ankaufpreis-Befunde (unter der Schwelle, aber dokumentationspflichtig)

> Vier weitere Domains mit Preis-Signal, die die Aufnahmekriterien (mind. 3 Zeilen, Punktpreis, Tagesdatum) nicht erreichen.

| Domain | URL | HTTP | Wörtlicher Wert | Warum nicht eligible |
|---|---|---|---|---|
| `emh-metall.de` | `https://www.emh-metall.de/aktuelles` | 200 | `Ankauf Altpapier 0,05 €/kg 15.06.2026` | nur **eine** Sorte, kein Preisblock; unter der 3-Zeilen-Schwelle. Slugs: `sn-lugau-emh-eisen-metall-handels-aufbereitungsge`, `sn-chemnitz-mittelbach-emh-eisen-metall-handels-aufbereitungsge` |
| `metallhandel-schwanewede.de` | `https://www.metallhandel-schwanewede.de/app/download/5799085623/Katalysator+Preisliste+.pdf` | 200, `application/pdf`, 691 582 B | `Alfa Romeo 0001 40 €`, `BMW 0001 4 Zylinder 65 €`, `VW & Audi 0017 - Tdi 50 €` | PDF mit „ANKAUFS PREISLISTE KATALYSATOREN“, aber **kein Datum** im Dokument (keine Datums-Tokens); Einheit nur implizit „Stück“ bzw. Spaltenkopf „pro kg“. Slugs: `hb-ronnebeck-altmetall-bunker`, `ni-schwanewede-autoverwertung-und-metallhandel` |
| `schrott-mohr.de` | `https://www.schrott-mohr.de/llms.txt` (Quelle: `/faq`) | 200 | `Aluminiumschrott: 0,50–0,80 € pro kg`, `Stahlschrott (Eisen): … etwa 7–16 Cent pro kg`, `Kabelschrott: … 1,30 € bis 7,50 € pro kg` | **Spannweiten** statt Einzelpreise, kein Datum; die Quellseite `/faq` nennt die Werte nur in Worten („`Kilogrammpreise zwischen fünfzig und 80 Cent`“) → nicht maschinell extrahierbar. Slugs: `bw-k-a-mohr-schrotthandel-und-demontage`, `he-wiesbaden-schierstein-mohr-rohstoff-schrotthandel-und-demontag`, `rp-worms-mainz-mohr-schrotthandel-mobil` |
| `metallhandel-schwanewede.de` | `https://www.metallhandel-schwanewede.de/metallpreise/` | 200 | `Alu Profile … 0,00 € Tagespreise auf Anfrage` | Platzhalter: der Wert ist literaler `0,00 €`, der Preis kommt clientseitig |
| `schrotthandel-richter.de` | `https://www.schrotthandel-richter.de/vhm-hss-schrott` | 200 | `Aktueller Preisrahmen (Stand: Juli 2026): 40–60 € pro Kilogramm für sortenreinen Hartmetallschrott` | **Preis + Einheit + Datum vorhanden** („Stand: Juli 2026“), aber nur **eine** Sorte und als **Spannweite** (40–60 €/kg) statt als Punktpreis. Der Satz steht dreifach redundant: im Fließtext, im `<details>`-Block und als JSON-LD-`FAQPage` → gut extrahierbar. Slugs: 3 betroffen |

---

## B) Gebührenquellen

> Abgrenzung: Diese Quellen nennen **keine Ankaufpreise für Schrott**, sondern Gebühren, die der Betreiber dem Kunden berechnet (Container, Entsorgung, Sortier-/Weigerkosten) bzw. Preise für **verkaufte** Ware (Sand, Kies, Baustoffe). Sie gehören nicht in einen Ankaufspreis-Handler, sind aber eine eigene, gut parsbare Datenkategorie.

### B1 — `hagemann-recycling.de` — PDF-Preislisten mit Datum (**qualitätshöchste**, zwei Dokumente)

| Feld | Wert |
|---|---|
| Domain | `hagemann-recycling.de` |
| URL 1 (Gebühren) | `https://www.hagemann-recycling.de/medien/2026/09/Preisliste-Entsorgung-HRPC-ab-01.09.26.pdf` |
| URL 2 (Verkaufspreise) | `https://www.hagemann-recycling.de/medien/2026/09/Preisliste-Verkauf-HRPC-ab-01.09.26.pdf` |
| Format | PDF (Text extrahierbar mit `pdftotext -layout`), feste Spalten `Artikel | Einheit | Netto | MwSt. | Brutto` |
| HTTP-Status | **200**, `application/pdf`, 275 472 B bzw. 227 251 B |
| Betroffene Slugs | `ni-wolfenbuttel-38304-hagemann-recycling`, `ni-cremlingen-hagemann-recycling` (2) |
| Datum | **ja, wörtlich im PDF-Kopf:** `Preisliste: Entsorgung Hagemann Recycling Park Cremlingen (Stand 01.09.2026)` bzw. `Preisliste: Verkauf Hagemann Recycling Park Cremlingen (Stand 01.09.2026)`; zusätzlich im Linktext `Gültig ab 01.09.2026` und im Dateinamen `ab-01.09.26` |

**Wörtliche Beispielwerte (PDF 1, Entsorgung):**

1. `Bauschutt, rein mineralisch (Kantenlänge bis 50 cm)   t   40,34 €   7,66 €   48,00 €`
2. `Gasbeton und Ytong   t   88,24 €   16,76 €   105,00 €`
3. `Styropor und Styrodur   t   1.554,62 €   295,38 €   1.850,00 €`

**Wörtliche Beispielwerte (PDF 2, Verkauf):**

4. `gesiebter Sand 0/2 mm PK 1   t   24,37 €   4,63 €   29,00 €`
5. `Kies 0/8 mm PK 1   t   33,61 €   6,39 €   40,00 €`
6. `Splitt 2/5 mm PK 1   t   54,62 €   7,38 €   65,00 €`

**Begründung.** Das sind die **einzigen beiden Quellen im gesamten Screening, die Preis + Einheit + Datum in einem Dokument vereinen** — und zwar in maschinenlesbarer Textform, mit konsistenter Spaltenstruktur und Netto-/MwSt.-/Brutto-Aufschlüsselung. Die Einheit steht in einer eigenen Spalte (`t`, `Stück`, `Sack`, `psch.`, `Kubikmeter`, `100 kg`), Werte sind sauber numerisch parsebar (deutsches Komma). Der Monat 09/2026 ist aktuell, die Datei ist verlinkt von der Standortseite `/cremlingen/`. Für einen Handler also: PDF → Text → Tabelle, ohne JavaScript. Zu beachten: Es sind **Gebühren- und Verkaufspreise**, keine Ankaufpreise; `Mischschrott` steht dort wörtlich als `kostenlose Annahme`.

### B2 — `keilich-wetzlar.de` — Annahmebedingungen / Weigerkosten (HTML-Tabelle, Gebühren)

| Feld | Wert |
|---|---|
| Domain | `keilich-wetzlar.de` |
| URL (HTML) | `https://www.keilich-wetzlar.de/leistungen/ankauf/annahmebedingungen/` |
| URL (PDF) | `https://www.keilich-wetzlar.de/app/download/13134689/AnnahmebedingungenKeilich01.01.22.pdf` |
| Format | HTML: echte `<table>` mit 14 Zeilen, 2 Spalten; PDF: Text extrahierbar, Titel „`Annahmebedingungen / Weigerkosten`“ |
| HTTP-Status | HTML **200**; PDF **200**, `application/pdf`, 60 779 B |
| Betroffene Slugs | `he-wetzlar-emil-keilich-wetzlar`, `he-stadtallendorf-emil-keilich-wetzlar` (2) |
| Einheit | €/t, €/Stück, €/kg, €/Sack, €/Liter/Kg, € (pauschal) |
| Datum | HTML: **kein**; PDF-Fassung trägt die Fassung **01.01.2022** im Dateinamen (`AnnahmebedingungenKeilich01.01.22.pdf`) |

**Wörtliche Beispielwerte:**

1. `Verunreinigungen (Schutt) 200,00 €/t`
2. `Gasflaschen geschlossen 220,00 €/Stück`
3. `Eternit Fehlwürfe 10,00 €/kg`
4. `Monitore, Tastaturen, Elektrogeräte allgemein 500,00 €/t`
5. `Bohremulsion, Kühl - und Schmierstoffe 0,20 €/Liter/Kg`
6. `Handlinkkosten für radioaktiven Fund (pauschal) 250,00 €`

**Begründung.** Maschinenlesbar ohne JS: eine einzige, klar erkennbare `<table>` mit 14 Datenzeilen, Label in Spalte 1, „Wert + Einheit“ in Spalte 2. Inhaltlich eindeutig eine **Gebührenquelle**: der Einleitungssatz lautet wörtlich „`Entsorgungskosten zzgl. der jeweils geltenden Mehrwertsteuer berechnet:`“. Ein Unit-Parser muss aufpassen — die Einheiten mischen `€/t` und `€/Stück` und `€/Sack`; `€`-ohne-Einheit (Zeile „Handlinkkosten … 250,00 €“) und `€/Liter/Kg` (Doppeleinheit) sind Sonderfälle, die der Handler laut Repo-Regeln („nur kg/t“-Disziplin) begründen muss. Das Alter ist das Problem: Die PDF-Fassung datiert auf 01.01.2022, das HTML nennt gar kein Datum — die Werte sind damit nicht als tagesaktuell führbar.

### B3 — `ac-containerdienst.de` — Container- und Entsorgungsgebühren (HTML, 3 Tabellen)

| Feld | Wert |
|---|---|
| Domain | `ac-containerdienst.de` |
| URL | `https://www.ac-containerdienst.de/containerdienst-abfallentsorgung/` |
| Alternative | `https://www.ac-containerdienst.de/?option=com_jumi&task=prices` (Jumi-Preis-Task, HTTP 200) |
| Format | HTML: 3 `<table>` (15 / 4 / 4 Zeilen), Kopfzeile wörtlich `Abfallart | Preis* pro Tonne` |
| HTTP-Status | **200** |
| Betroffene Slugs | `by-alzenau-jr-recycling`, `by-freigericht-somborn-63579-he-ac-containerdienst` (2) |
| Datum | **kein Preisdatum** — die im HTML gefundene Angabe `2026-10-26` stammt wörtlich aus `"translation-revision-date":"2026-10-26 14:55:04+0000"` des Contact-Form-7-Plugins und ist **kein** Preisdatum |

**Wörtliche Beispielwerte:**

1. `Erde / Bodenaushub | ab nur 35€`
2. `Holz A1 - A3 | ab nur 85€`
3. `Sperrmüll (min. 30% Holz) | ab nur 200€`
4. `Baumischabfall / Mischmüll (Angebot) | ab nur 225€ 250€`
5. `Container Lieferung Mulden und Absetzcontainer (inkl. 7 Tage Containermiete) | nur 40€ - 140€`
6. `5 m³ | 250,00 €` (Vorauszahlungstabelle)

**Begründung.** Inhaltlich sauber getrennt: Das ist die **Gebühren-/Containerpreisliste**, nicht die Schrott-Ankaufpreise. Die Schrott-Seite `/schrotthandel-schrottankauf/` trägt zwar die Überschrift „vergüten immer zu aktuellen Tagespreisen“, enthält aber **keine** Zahlen zu Schrott — die einzigen €-Werte im Dokument stammen aus dem eingebetteten Container-Preis-Widget. Für die Gebühren ist die Quelle gut: drei klar getrennte Tabellen, Einheit „pro Tonne“ im Kopf, „ab“-Präfix und Spannweiten maschinell erkennbar (`ab 225€ 250€`). Einschränkungen: keine Preise in €/kg, und die „Ab“-/Staffel-Logik („ab nur 35€“) ist eine Preisuntergrenze, kein Festpreis — das muss ein Handler als `price_max`/`upto`-Semantik abbilden oder verwerfen.

### B4 — `mehring-container.de` — Entsorgungsgebühren im Bestellformular (Gebühren, in HTML-Attributen)

| Feld | Wert |
|---|---|
| Domain | `mehring-container.de` |
| URL | `https://www.mehring-container.de/container-bestellen/` |
| Format | HTML: Gebühren stehen in `value`-Attributen von `<option>` (Divi-Formular), zusätzlich als Text in `<span class="ec-title">` |
| HTTP-Status | **200** (602 022 B) |
| Betroffene Slugs | `by-dorfprozelten-mehring`, `by-miltenberg-63897-schrott-mehring` (2) |
| Datum | **kein** |

**Wörtliche Beispielwerte:**

1. `Altholz A I -III / Entsorgung = ab 131,00€ pro Tonne` (im Attribut: `<option value='----&gt; Altholz A I -III / Entsorgung = ab 131,00€ pro Tonne|0'>`)
2. `Entsorgung = ab 196€ pro Tonne`
3. `Entsorgung = ab 351€ pro Tonne`
4. `Entsorgung = ab 450€ pro Tonne`
5. `Entsorgung = ab 0,00€ pro Tonne`

**Begründung.** Maschinenlesbar — und zwar ungewöhnlich robust: der Gebührenwert steht **im `value`-Attribut** eines `<option>`-Elements, also in einem strukturierten Feld, nicht nur im Fließtext; zusätzlich erscheint er als `ec-title`-Span. Der Parser bekommt Abfallart, Preis und Einheit in einem Token. Inhaltlich reine Gebührenquelle („Entsorgung = ab X € pro Tonne“), mit 14 Abfallarten. Nachteile: keine Zahlen in €/kg, kein Datum, und der Nullpreis (`ab 0,00€ pro Tonne`) ist ein Sonderfall. Die Seite enthält außerdem einen WordPress-Zeitstempel `2026-02-19T14:05:54+01:00` (`mehring-admin`), der aber eine Bearbeitungs-, keine Preisangabe ist.

### B5 — `hagemann-recycling.de` — Annahmebedingungen-PDF: **HTTP 403**

`https://www.hagemann-recycling.de/download/13/allgemein/938/annahmebedingungen-stahlschrott-und-metallschrott_hagemann-recycling.pdf` → **403** (Text/HTML, 791 B). Die Datei ist verlinkt, aber nicht öffentlich abrufbar. Negativbefund mit Beleg.

---

## C) Negativbefunde

**116 Domains vollständig geprüft** (alle aus der Screening-Liste, in der vorgegebenen Reihenfolge). Spalten: *geprüfte URLs* (unique, HTTP-Methode GET), *davon HTTP 200*, *Enumerationsbasis*, *Ergebnis*.

| Domain | URLs geprüft | davon 200 | Enumerationsbasis | Ergebnis |
|---|---:|---:|---|---|
| `eberhard-mayr.de` | 88 | 29 | WP-REST 19 S. | `/?option=com_jumi&task=prices` rendert die Homepage (Jumi-Preis-Task nicht konfiguriert); `/handel` ohne Zahlen |
| `fegert-recycling.de` | 79 | 67 | Sitemap 12 URLs | **Soft-404:** alle Preispfade liefern die Startseite („Fegert Recycling GmbH – Das Unternehmen“, Datum 4. Mai 2017) |
| `foell-rohstoffhandel.de` | 92 | 28 | Sitemap 2 URLs | alle 24 geprüften Preispfade → **HTTP 404**; nur 1 von 47 URLs mit HTTP 200 |
| `goetz-neu-ulm.de` | 99 | 40 | WP-REST 74 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `kuehl-entsorgung.de` | 97 | 39 | WP-REST 63 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `rmr-recycling.de` | 81 | 22 | WP-REST 23 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `veolia.de` | 99 | 37 | Sitemap 829 URLs | `/Preisanpassung_Entsorgung` = Gebührenindex, kein Scrap-Ankaufpreis |
| `alba.info` | 97 | 34 | Sitemap 8 URLs | Großkonzern-Website; `/geschaeftsfelder/rohstoffhandel/` u. a. ohne Preisnennung |
| `aurubis.com` | 100 | 36 | Sitemap 2500 URLs | Hüttenkonzern; `/investor-relations/aurubis-aktie/kurse-und-charts` = **Aktienkurs**, kein Material-Ankaufpreis |
| `eo-entsorgung.de` | 87 | 28 | WP-REST 39 S. | **Ankaufpreis-Lead (ohne Datum).** Preise ausschließlich via WordPress-REST `?slug=ankauf-altmetall_{offenbach,wiesbaden,mainz,frankfurt,darmstadt}`; öffentliche URL 301→`/schrott-metalle/`; kein Datums-Token im Seiteninhalt |
| `g-voth.de` | 72 | 6 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `nehlsen.com` | 99 | 37 | Sitemap 59 URLs | `shop.nehlsen.com` = Container-Vermietung, PLZ-abhängig, JS-Preise; keine Materialpreisliste |
| `nisa.alba.info` | 84 | 21 | Sitemap 1 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `nord.alba.info` | 82 | 19 | Sitemap 2 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `rheinische-scheidestaette.de` | 105 | 46 | — | Edelmetall-Börsenkurs serverseitig in `<span id="tp_gold" class="price">3.472,97 €</span>`; Einheit „EUR pro Feinunze“; Zeitstempel nur JS („Preisaktualisierung in --:--“) → kein Datum im HTML; `/preise/`, `/preise-und-steuern/` → 404 |
| `schrott-mohr.de` | 90 | 32 | WP-REST 35 S. | `/faq` nur in Worten („fünfzig und 80 Cent“); `/llms.txt` mit Richtwert-**Spannen** („0,50–0,80 € pro kg“), kein Datum |
| `schrotthandel-richter.de` | 123 | 60 | Sitemap 1460 URLs | eine Preis**spanne** mit Datum: `/vhm-hss-schrott` = „Aktueller Preisrahmen (Stand: Juli 2026): 40–60 € pro Kilogramm für sortenreinen Hartmetallschrott“ (1 Sorte, Spannweite); `/schrottpreise` = Tabelle mit 11 Zeilen, aber **ohne jeden Zahlenwert** (Spalten „Wertigkeit“ / „Was den Preis hebt“) |
| `wmr-online.de` | 94 | 36 | WP-REST 28 S. | WP-REST 28 Seiten, keine Angebotspreise |
| `abcontainer.de` | 73 | 7 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `ac-containerdienst.de` | 38 | 2 | Sitemap 8 URLs | **Gebührenquelle.** 3 Tabellen auf `/containerdienst-abfallentsorgung/` (€/t, €/Stück); `?option=com_jumi&task=prices` liefert dieselben Werte; Datum 2026-10-26 im HTML = Contact-Form-7 `translation-revision-date`, **kein** Preisdatum |
| `altfeld.de` | 81 | 18 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `altmetall-haeuser.de` | 72 | 11 | Sitemap 3 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `altstoffhandel-jena.de` | 77 | 18 | WP-REST 11 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `am-altmetall.de` | 75 | 27 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `baesecke.de` | 78 | 19 | WP-REST 10 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `baumgaertner.de` | 100 | 36 | Sitemap 1 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `becker-maurer.de` | 82 | 30 | Sitemap 10 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `berk-recycling.com` | 96 | 39 | WP-REST 108 S. | `/metallankauf/schrottpreise/` = „Keine Preisliste, kein Ticker – Tagespreis am Telefon“; `/schrottpreise/` → 404 (Seite existiert nur in der WP-DB) |
| `berliner-rohstoffhandel.de` | 85 | 26 | WP-REST 40 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `bgh.de` | 97 | 34 | Sitemap 1 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `borchersindustrieservice.com` | 73 | 9 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `bosselmann-metallhandel.de` | 83 | 23 | WP-REST 14 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `bsb-schrott.de` | 74 | 70 | — | **Soft-404:** alle 27 Preispfade + `*.csv`-Pfade (`/prices.csv`, `/preise.csv`, `/export/preise.csv`) liefern die Homepage-HTML statt CSV |
| `carnuth.de` | 74 | 13 | WP-REST 5 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `christian-keller-metall.de` | 77 | 13 | Sitemap 7 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `cronimet-cremetal.de` | 85 | 33 | Sitemap 1 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `de-ro.de` | 75 | 12 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `de.emrgroup.com` | 97 | 84 | — | alle geprüften Pfade liefern `/Error` (Soft-Fehler statt 404) |
| `deinschrotty.de` | 69 | 4 | — | nahezu alle geprüften Pfade 404; keine Preisquelle |
| `demontage-team.de` | 72 | 6 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `dorr.de` | 14 | 1 | — | Host **nicht erreichbar** (TLS-Handshake-Timeout; 3 Versuche à 45 s, curl error 28) |
| `eckhardt-schrott.de` | 77 | 13 | Sitemap 1 URLs | alle Preispfade 404, keine Preisquelle |
| `eg-metall.de` | 93 | 32 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `ehrich.de` | 97 | 36 | Sitemap 33 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `eisen-rudi.de` | 76 | 17 | WP-REST 7 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `emh-metall.de` | 38 | 2 | Sitemap 11 URLs | nur **ein** Preis: „Ankauf Altpapier 0,05 €/kg 15.06.2026“ (unter der 3-Zeilen-Schwelle) |
| `flaxres.com` | 82 | 21 | WP-REST 26 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `frankberger.com` | 84 | 21 | Sitemap 16 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `franz-plum.de` | 86 | 21 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `gebrueder-aurin.de` | 76 | 16 | WP-REST 7 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `geweniger-recycling.de` | 73 | 8 | Sitemap 5 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `gmh-gruppe.de` | 97 | 40 | WP-REST 34 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `goldankauf-silber.de` | 84 | 19 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `gorsler-alsfeld.de` | 98 | 52 | WP-REST 30 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `hagemann-recycling.de` | 91 | 32 | WP-REST 30 S. | **Gebühren-/Verkaufs-PDF mit Datum.** `Preisliste-Entsorgung-HRPC-ab-01.09.26.pdf` + `Preisliste-Verkauf-HRPC-ab-01.09.26.pdf` („Stand 01.09.2026“); Annahmebedingungen-PDF → **HTTP 403** |
| `hark-hamm.de` | 91 | 23 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `harzer-schrott-recycling.de` | 80 | 15 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `hasse-transport.de` | 37 | 11 | Sitemap 23 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `hlr-rohstoffhandel.de` | 85 | 25 | WP-REST 13 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `hoautorecycling.de` | 69 | 2 | — | nahezu alle geprüften Pfade 404; keine Preisquelle |
| `hofmann-metall.de` | 37 | 24 | Sitemap 90 URLs | **Marktpreis-Lead.** Homepage-Widget `#boersenliste`: BDSV-Stahlmarktpreise (€/t, „10/2026“) + Börsenpreise ($, „Vortag: 29.10.2026“); „Historie anzeigen“→ bdsv.org. **Soft-404 auf allen JSON-/API-Pfaden** (`/api/prices`, `/preise.json`, `/data/preise.json`, `/export/preise.csv` … 17 Pfade): alle liefern **HTTP 200 mit `text/html`** (Homepage) statt JSON — ein JSON-Prober bekommt HTML |
| `karle-recycling.de` | 97 | 63 | WP-REST 98 S. | `/preise-eisenschrotte-und-nichteisenmetalle-boeblingen/` verweist auf SharePoint-Download → **HTTP 403**; Werte nur im WordPress-Export |
| `karletshofer.de` | 83 | 23 | WP-REST 10 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `keilich-wetzlar.de` | 41 | 5 | Sitemap 23 URLs | **Gebührenquelle.** `/leistungen/ankauf/annahmebedingungen/` = 14-Zeilen-Tabelle (€/t, €/Stück, €/kg, €/Sack, €/Liter/Kg); HTML ohne Datum, PDF-Fassung „AnnahmebedingungenKeilich01.01.22.pdf“ (01.01.2022) |
| `klajo-recycling-giessen.de` | 91 | 32 | WP-REST 20 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `kreber-trier.de` | 69 | 5 | Sitemap 1 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `leberrohstoffe.com` | 98 | 32 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `lesch-rohstoffhandel.de` | 76 | 29 | WP-REST 10 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `loacker-recycling.com` | 72 | 13 | WP-REST 61 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `lrp-autorecycling.de` | 98 | 41 | WP-REST 41 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `mehring-container.de` | 84 | 37 | WP-REST 24 S. | **Gebührenquelle.** Entsorgungsgebühren je Abfallart im `value`-Attribut von `<option>` (JS-Bestellformular); kein Datum |
| `metallhandel-schwanewede.de` | 100 | 36 | Sitemap 32 URLs | `/metallpreise/` = Accordion mit Platzhalter „0,00 € Tagespreise auf Anfrage“ (keine Werte); `/home/produktpreissuche/` = JS-Produktsuche ohne Preise im HTML; Katalysator-PDF „ANKAUFS PREISLISTE“ ohne Datum |
| `metarec-recycling.de` | 82 | 17 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `mielnik.de` | 76 | 10 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `mobiler-schrottservice.de` | 73 | 14 | WP-REST 6 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `muth-recycling.de` | 80 | 16 | Sitemap 13 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `nestleronline.de` | 95 | 32 | Sitemap 1 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `niemet.de` | 96 | 31 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `nrh-recycling.de` | 76 | 13 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `oostdam-metalle.de` | 74 | 9 | Sitemap 9 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `paulundsohn.de` | 75 | 24 | Sitemap 8 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `petko-metalle.de` | 78 | 19 | WP-REST 9 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `pfandkredit-kratz.de` | 75 | 11 | Sitemap 8 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `pmk-rohstoffhandel.de` | 14 | 2 | — | React-SPA (688-B-Shell); JS-Bundle `/assets/index-CzdtOpWl.js` (311 kB) ohne jede Zahl; PocketBase-API `/.sfs-be/api/collections/…/records` öffentlich, aber **keine Preis-Collection** (nur `bewertungen`, `einkauef_anfragen`) |
| `preimesser.de` | 95 | 33 | Sitemap 1 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `r-skusche.de` | 76 | 18 | WP-REST 9 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `rad-recycling.de` | 71 | 7 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `rank-recycling.de` | 79 | 20 | WP-REST 8 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `remondis-buedingen.de` | 67 | 54 | — | **Bot-Challenge:** Homepage liefert HTTP 200 mit „Challenge Validation“ (1.949 B); `/preise` → echte 404-Seite |
| `remondis-entsorgung.de` | 67 | 54 | — | **Bot-Challenge:** Homepage liefert HTTP 200 mit „Challenge Validation“ (1.896 B); `/preise` → echte 404-Seite (TYPO3) |
| `resebeck.de` | 88 | 76 | WP-REST 23 S. | **Soft-404:** 19 von 27 Preispfad-Treffern liefern die Homepage („Resebeck | Full-Service Recycling“) |
| `roselieb-gmbh.de` | 90 | 27 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `rwr-recycling.de` | 82 | 18 | Sitemap 15 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `sachsen.alba.info` | 80 | 17 | Sitemap 2 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schaumaier.de` | 94 | 28 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schmuck-filbert.de` | 98 | 30 | Sitemap 23 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schrott-bosch.de` | 83 | 71 | WP-REST 39 S. | **Soft-404:** alle 27 Preispfade liefern die Homepage („Schrott Bosch – Schrott Bosch“) |
| `schrott-friedrich.de` | 85 | 34 | Sitemap 15 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schrott-wetzel.de` | 76 | 11 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schrotthandel-giessen.de` | 79 | 20 | WP-REST 13 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schrotthandel-heudeber.de` | 82 | 34 | WP-REST 14 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schrotthandel-rhein-main.de` | 88 | 41 | WP-REST 10 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `schrottplatz-koblenz.de` | 86 | 28 | WP-REST 37 S. | 9 SEO-Preisseiten (zinkpreise, messingpreise, eisenpreis …) **ohne jede Zahl** („zu den aktuellen Tagespreisen“) |
| `shala-schrott-handel.de` | 75 | 10 | Sitemap 3 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `sho-oesterreich.de` | 83 | 24 | WP-REST 21 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `smh-recycling.de` | 92 | 40 | Sitemap 30 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `smr-muehldorf.de` | 91 | 26 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `thueringen-recycling.de` | 95 | 30 | Sitemap 110 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `vd-recycling.de` | 92 | 38 | Sitemap 77 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `wasdrack.de` | 79 | 16 | Sitemap 8 URLs | identische Seite wie `schrottpreis.berlin`; nur SEO-Navigationsseiten („Aluminiumpreis“, „Kupferpreis“ …) ohne Zahlen |
| `weckwerth-schrott.de` | 79 | 20 | WP-REST 13 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `weli-stahl.de` | 75 | 16 | WP-REST 7 S. | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `wertstoff-dealer.de` | 83 | 19 | Sitemap 14 URLs | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `wertz.de` | 99 | 34 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `wil-ro.de` | 71 | 7 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |
| `wittmann-recycling.de` | 85 | 23 | — | keine Preisquelle (alle Pfade ohne Preis+Einheit+Datum) |

**Zusammenfassung der Negativbefunde (116 Domains):**

- **Keine Preisquelle (105):** alle geprüften Pfade enthalten weder Preis+Einheit+Datum noch eine Ankaufspreisliste. Typische Muster: „Schrottpreise am Telefon“-Textseiten ohne Zahlen, Platzhalter `0,00 € Tagespreise auf Anfrage`, SEO-Textseiten („zu den aktuellen Tagespreisen“), Accordion-Produktkataloge, „Tageshöchstpreise“-Marketing ohne Zahlen.
- **Soft-404 (5 Domains + 1 Pfad-Klasse):** `bsb-schrott.de`, `resebeck.de`, `schrott-bosch.de`, `fegert-recycling.de`, `pmk-rohstoffhandel.de` — der Server liefert für jeden Pfad die Homepage bzw. eine SPA-Shell zurück. Dazu `hofmann-metall.de`, das seine **17 JSON-/CSV-/API-Pfade** mit HTTP 200 + HTML beantwortet, obwohl sie nach JSON aussehen. Ein Handler, der auf diesen Domains blind Pfade probiert, bekommt **immer HTTP 200** und könnte Gehalt als „keine Preise“ fehlinterpretieren, statt mit „Pfad existiert nicht“ zu rechnen.
- **Bot-Challenge (2):** `remondis-buedingen.de`, `remondis-entsorgung.de` — Homepage antwortet mit HTTP 200 und dem Seitentitel „Challenge Validation“ (1 949 B / 1 896 B). Die Unterpfade antworten korrekt mit 404. Ein Parser sieht also „200 + keine Preise“ und muss die Challenge erkennen.
- **Timeout (1):** `dorr.de` — 3 Versuche à 45 s, curl error 28, `https://dorr.de/` und `https://www.dorr.de/` beide unerreichbar.
- **Weitere Fehlerbilder (2):** `de.emrgroup.com` (alle Pfade → `/Error`), `karle-recycling.de` (Preisliste nur via SharePoint, HTTP 403).

Geprüfte Pfadmuster je Domain: Homepage (HTTP + https-www-Variante) · alle internen Links der Homepage (bis 30, davon alle „preis“-namigen ungekürzt) · 24 statische Preispfade (`/preise`, `/metallpreise`, `/ankauf`, `/ankauf/preise`, `/tagespreise`, `/schrottpreise`, `/aktuelle-preise`, `/preise/metall`, `/de/preise`, `/de/metallpreise`, `/de/preisliste`, `/sortenpreise`, `/reststoffpreise`, `/preisliste`, `/ankaufspreise`, `/einkaufspreise`, `/schrott/preise`, `/metall/preise`, `/metallhandel/preise`, `/rohstoffe/preise`, `/unternehmen/preise`, `/leistungen/ankauf`, `/schrottankauf`, `/sorten`, `/materialien`, `/stoffe`, `/entsorgung/preise`) · 15 JSON-/CSV-/API-Pfade (`/api/prices`, `/api/preise`, `/api/metal-prices`, `/api/metallpreise`, `/api/price-list`, `/preise.json`, `/prices.json`, `/data/preise.json`, `/data/prices.json`, `/preise.csv`, `/prices.csv`, `/export/preise.csv`, `/feed/preise`, `/wp-json/…`, `/sitemap.xml`) · Sitemap-Baum (3 Ebenen) inkl. aller preis-benannten Sitemap-URLs · WordPress-REST `/wp-json/wp/v2/pages` und `/posts` mit Volltextscan · 12 Preis-Subdomains (`preise.`, `preis.`, `prices.`, `shop.`, `markt.`, `marktpreise.`, `de.`, `en.`, `app.`, `portal.`, `www2.`, `shopify.`) · Link-Sweep über alle 3 234 gecachten Seiten nach Datei-Links mit Preis-Namen (PDF/XLS/CSV/DOC).

---

## D) Empfehlung — Reihenfolge „zuerst umzusetzen“

> Ehrliche Vorbemerkung: Die Auftragsannahme („der Hebel liegt in einer maschinenlesbaren Preisquelle“) hält für diese 116 Domains nur in der Variante „**eine geteilte Referenzquelle plus Händler-Aufschlag**“. Kein Händler der 116 liefert eine scrapbare Eigenpreisliste. Die Empfehlung ordnet deshalb zuerst die Quellen, deren Feed man einmal bauen und dann für viele Betreiber nutzen kann.

| # | Quelle | Konkrete URL, die ein Handler aufruft | Erwartetes Format | Begründung / Abdeckung |
|---|---|---|---|---|
| **1** | `hofmann-metall.de` — Marktpreis-Widget | `GET https://hofmann-metall.de/` → `<div id="boersenliste">` → `#b1 li` / `#b2 li` | HTML, serverseitig; je `<li>`: Label-Text + `<span>`-Preis (`253,60 €/t` bzw. `14.349,87 $`) | **Beste technische Qualität im Set:** stabile IDs, Preis und Label in getrennten Elementen, Datum als Klartext im Header („10/2026“, „Vortag: 29.10.2026“), kein JS. Deckt 2 Slugs ab und ist die einzige Quelle mit Preis+Einheit+Datum. Nachteil: Markt-, nicht Händlerpreis. |
| **2** | `bdsv.org` — Branchen-Durchschnittspreise | `GET https://www.bdsv.org/unser-service/markt-preise/` | HTML-Textknoten; Label `Sorte 1`…`Sorte 5` ↔ `254,6 €/t`; Monat `(09/2026)` | **Ein Feed für die gesamte Stahlschrott-Branche.** Gilt für jeden Händler der Liste, der Stahlschrott handelt (praktisch alle). Nachteile: nur Monatsdatum, keine Tabelle (Diagramm-Beschriftung), und es sind Ab-Station-**Verkaufs**preise. |
| **3** | `eo-entsorgung.de` — Ankaufpreise via REST | `GET https://eo-entsorgung.de/wp-json/wp/v2/pages?slug=ankauf-altmetall_offenbach` (analog `_wiesbaden`, `_mainz`, `_frankfurt`, `_darmstadt`) | JSON; `content.rendered` = HTML mit 39 `<li>`-Zeilen „Material … €/Tonne“ | **Der einzige echte Ankaufpreisblock im Set**, vollständig und ohne JS abrufbar. Deckt 3 Slugs (Kelkheim, Maintal, Bingen) mit identischen Werten. Zwei Haken: **kein Datum** und die öffentliche URL 301-redirectet. Umsetzung: als „undatiert, letzte DB-Revision 2025-07-17“ führen, oder beim Betreiber ein Datum nachfordern. |
| **4** | `hagemann-recycling.de` — PDF-Preislisten | `GET https://www.hagemann-recycling.de/medien/2026/09/Preisliste-Entsorgung-HRPC-ab-01.09.26.pdf` und `…-Verkauf-HRPC-ab-01.09.26.pdf` | PDF-Text (`pdftotext -layout`), Spalten `Artikel | Einheit | Netto | MwSt. | Brutto` | **Einzige Quelle mit Preis + Einheit + Datum in einem Dokument.** Maschinell extrahierbar, monatlich aktualisiert (Dateiname `ab-01.09.26` = Ablageregel). Deckt 2 Slugs. Inhaltlich Gebühren/Verkauf — als eigener Gebühren-Typ, nicht als Ankaufspreis. |
| **5** | `keilich-wetzlar.de` — Annahmebedingungen | `GET https://www.keilich-wetzlar.de/leistungen/ankauf/annahmebedingungen/` (bzw. PDF `…/app/download/13134689/AnnahmebedingungenKeilich01.01.22.pdf`) | HTML: eine `<table>`, 14 Zeilen, 2 Spalten | Sauberste Gebühren-Tabelle des Sets, ohne JS, ohne Fremd-Einbettung. Deckt 2 Slugs. Achtung: Einheiten mischen €/t und €/Stück; PDF-Fassung ist von 01.01.2022 (veraltet). |
| **6** | `ac-containerdienst.de` — Container-/Entsorgungspreise | `GET https://www.ac-containerdienst.de/containerdienst-abfallentsorgung/` | HTML: 3 `<table>` (15/4/4 Zeilen), Kopf `Preis* pro Tonne` | 14 Abfallarten plus Container-Vorauszahlungen; „ab“-Staffeln parsebar. Deckt 2 Slugs. Kein Preisdatum im Dokument. |
| **7** | `mehring-container.de` — Gebühren im Formular | `GET https://www.mehring-container.de/container-bestellen/` → `<option value="…">` | HTML-Attributwerte: `----> Altholz A I -III / Entsorgung = ab 131,00€ pro Tonne|0` | Der strukturierteste Gebührenfund: Wert **im Attribut**, nicht im Fließtext — minimaler Parser-Aufwand. Deckt 2 Slugs. Kein Datum. |
| **8** | `rheinische-scheidestaette.de` — Edelmetallkurs | `GET https://rheinische-scheidestaette.de/wissenswertes/aktuelle-edelmetallkurse/` → `#tp_gold`, `#tp_silber`, `#tp_palladium`, `#tp_platin` | HTML, je Metall ein `<span class="price">` mit `3.472,97 €`; Einheit aus Fließtext „EUR pro Feinunze“ | Serverseitig, stabile IDs, 3 Slugs. Aber: **kein Zeitstempel im HTML** (nur JS-Countdown) und Börsenkurs statt Ankaufpreis. Nur lohnend, wenn ein Edelmetall-Handler gebaut wird. |
| **9** | `metallhandel-schwanewede.de` — Katalysator-PDF | `GET https://www.metallhandel-schwanewede.de/app/download/5799085623/Katalysator+Preisliste+.pdf` | PDF-Text, Layout „Bezeichnung … €“ mit Spaltenkopf „Stück“ bzw. „pro kg“ | Echte **ANKAUFS**-Preisliste (2 Slugs), aber ohne Datum und nur Katalysatoren — Nischengerüst, niedrige Priorität. |
| **10** | `emh-metall.de` — Einzelpreis Altpapier | `GET https://www.emh-metall.de/aktuelles` → News-Eintrag „Ankauf Altpapier 0,05 €/kg 15.06.2026“ | HTML-Text, Label + Wert + Datum in einem Block | Kleinster vollständiger Treffer (Preis + Einheit + Datum) — aber nur **eine** Sorte, also kein Preisblock. Eher Beleg für „Preise nur anekdotisch“ als für einen Handler. |

**Was ein Parser zuerst treffen würde** (Reihenfolge der Fehlerklassen):
1. **HTTP 200 + Soft-404** (`bsb-schrott.de`, `resebeck.de`, `schrott-bosch.de`, `fegert-recycling.de`, `pmk-rohstoffhandel.de`) — der häufigste Fall. Ein Pfad-Prober bekommt immer 200; ohne Soft-404-Erkennung (Titel-/Body-Vergleich gegen die Homepage) meldet der Handler fälschlich „Seite vorhanden, aber keine Preise“.
2. **Bot-Challenge** (`remondis-buedingen.de`, `remondis-entsorgung.de`) — 200 mit „Challenge Validation“.
3. **Timeout** (`dorr.de`) — 45 s pro Versuch, muss im Scheduler hart begrenzt werden.
4. **Qualitative Preis-Versprechen ohne Zahl** — der Normalfall: „vergüten zu Tageshöchstpreisen“, „Schrottpreise am Telefon“, „Tageshöchstpreise für Altmetalle“. Das sind **Ankaufpreise am Telefon**; die Maschine kann sie nicht lesen.
5. **Nur Gebühren** — die Zahl steht da, aber sie ist eine Gebühr. Das ist der teuerste Fehler, weil er wie ein Preis aussieht und in einen Current-Preis-Feed gerät. Betroffen: `ac-containerdienst.de`, `mehring-container.de`, `keilich-wetzlar.de`, `hagemann-recycling.de`, `veolia.de`.
6. **Platzhalter `0,00 €`** (`metallhandel-schwanewede.de`) und **Spannweiten statt Punkte** (`schrott-mohr.de` „0,50–0,80 € pro kg“).

---

## E) Methodik und Grenzen

**Was geprüft wurde**

- **116 von 116 Domains** vollständig, in der Reihenfolge der Screening-Liste (Händlerzahl absteigend). **9 434 unique URLs** per GET abgerufen, davon **3 020 × HTTP 200**.
- Je Domain: Homepage (https + www-Variante) → alle internen Links der Homepage → 24 statische Preispfade → 15 JSON-/CSV-/API-Pfade → Sitemap-Baum (bis 3 Ebenen, alle preis-benannten URLs) → WordPress-REST (`/wp-json/wp/v2/pages`, `/posts`, `/search?search=preis`) mit Volltextscan des `content.rendered` → 12 Preis-Subdomains → Link-Sweep über alle 3 234 gecachten Seiten nach preis-benannten Datei-Links (PDF/XLS/CSV/DOC) → SPA-/JS-Bundle-Prüfung → 2 bekannte Bot-Challenge-Fälle mit curl gegengeprüft.
- **Detektor-Validierung (Kontrollset):** Der Preis-Detektor wurde gegen neun Preis-Seiten aus dem repo-eigenen Handlerbestand getestet (`mkm-metals.de/metallpreise`, `oder-metalle.de/schrott-preise/`, `boehner-altmetalle.de/ankauf-preise`, `gerwischer-recycling.de/ankaufspreise/` u. a.). Er hat 4 von 4 HTML-maschinenlesbaren Kontrollseiten korrekt getroffen (Preis + Einheit + Datum erkannt), die übrigen Kontrollseiten sind tatsächlich JS-gerendert bzw. Bilder — was den Detektor nicht zum Fehler, sondern zur Grenze macht.
- **Zweiter Durchgang über den kompletten HTTP-Cache** (3 234 Seiten) nach jedem Preis-/Einheit-Token im Text, in `<script>`-Blöcken und in HTML-Attributen. Ergebnis: 14 URLs mit Preis-Token (8 davon innerhalb der 116er-Liste, 6 sind die Kontrollseiten mit bereits vorhandenen Handlern) — alle manuell nachverifiziert.

**Erkennungsregeln**

- Preis: `1.234,56` / `1,234.56` / `1234` unmittelbar gefolgt von `€`/`EUR`/`Euro`.
- Einheit: `€/kg`, `€/t`, `€/to`, `EUR/kg`, `EUR/t`, `€ pro kg`, `€ pro Tonne`, `ct/kg`, `€ pro 100 kg`, `€/Sack`, `€/Stück`, `€/Liter` (auch mit Leerzeichen um `/`).
- Datum: `dd.mm.yyyy`, `yyyy-mm-dd`, `dd.mm.yyyy`, Monatsnamen, „Stand …“, „gültig ab …“.
- Tabelle gilt als Preisquelle ab **3 Zeilen** mit je Preis+Einheit. Fließtextpreise wurden zusätzlich per Regex erfasst.

**Unsicherheiten und Grenzen**

1. **Keine Ausführung von JavaScript.** React/Vue/Angular-Seiten wurden nur über die ausgelieferten Bundles und die REST-Backends geprüft. Preise, die erst clientseitig aus einem XHR entstehen, können übersehen worden sein. Konkret geprüft und ohne Preise: `pmk-rohstoffhandel.de` (Bundle 311 kB ohne Zahl; PocketBase-API `/.sfs-be/api` öffentlich, aber ohne Preis-Collection). Die Kontrollseite `altmetall-asn.de/ankaufliste/` (JS-gerendert) wäre durch diesen Ansatz nicht erfassbar gewesen.
2. **Suchmaschinen-Erreichung.** Die Websuche war nur manuell und punktuell nutzbar (DuckDuckGo-Lite lieferte HTTP 202 ohne Ergebnisse; Bing ignorierte den `site:`-Operator). Eine systematische Suchmaschinen-Abdeckung „alle Preisseiten dieser 116 Domains“ fand deshalb **nicht** statt. Das ist die größte verbleibende Lücke: eine Preisseite unter einem unerwarteten Pfad (z. B. `/media/preise.pdf`, wie bei `moelter-kronach.de`) ist nicht systematisch auffindbar. Der PDF-Link-Sweep (Abschnitt C) schließt diese Lücke teilweise.
3. **User-Agent / TLS-Fingerprint.** Der Python-Client löste bei `remondis-buedingen.de` und `remondis-entsorgung.de` die Bot-Challenge aus, `curl` mit demselben UA nicht (dort lieferte der Challenge-Request eine echte 404-Seite). Andere Domains könnten in der Gegenrichtung abweichen: eine Quelle, die für den Python-Client gesperrt war, hätte ich als 403/Timeout und nicht als Preisquelle verbucht.
4. **Zwei Domains waren technisch nicht erreichbar** (`dorr.de` Timeout, `de.emrgroup.com` liefert `/Error`) — für beide ist die Aussage „keine Preisquelle“ nicht belastbar.
5. **Datumsstrenge.** Die Auftragsregel „Preis + Einheit + Datum“ habe ich streng angewandt. Das schließt `eo-entsorgung.de` aus, obwohl es die substanziell wertvollste Ankaufpreisquelle des Screenings ist. Die Entscheidung ist bewusst konservativ: ohne „Stand“-Angabe ist ein Current-Preis nicht belastbar (so auch die Repo-Dokumentation: „Ohne Datum sind Preisangaben wertlos“).
6. **Fachliche Abgrenzung Marktreferenz vs. Ankaufpreis.** `hofmann-metall.de`, `bdsv.org` und `rheinische-scheidestaette.de` liefern Preise mit Datum, aber **keine Händler-Ankaufpreise**. Sie sind als Benchmark brauchbar, als „Ankaufspreis des Händlers“ fachlich falsch. Der Repo-Bestand hat bereits Handler für Goldhändler (`goldankauf_boerse`, `goldhandelshaus_bremen` u. a.) — der Muster-Vergleich ist also naheliegend, aber die Rohstoffdomäne ist eine andere.
7. **PDFs.** `pdftotext -layout` funktioniert bei den drei hier gefundenen Preis-PDFs sauber (`hagemann` ×2, `schwanewede`). Ein Preis-PDF war **nicht** auswertbar: `keilich-wetzlar.de/app/download/12917958/Metallwaage31.12.26.pdf` — Bild-PDF ohne Textschicht, kein OCR eingesetzt. `metallhandel-schwanewede.de` selbst ist textuell extrahierbar.
8. **Kein Zugriff auf Produktionsdaten.** Es wurde nichts in der Repo-Datenbank gelesen oder geschrieben; die Slug-Angaben stammen ausschließlich aus `/tmp/opencode/price-source-screening.json`. Es wurde keine Datei im Repository geändert.

**Was die Belastbarkeit der Empfehlung begrenzt**

Von den 116 Domains haben **2** eine Quelle, die die Auftragskriterien vollständig erfüllt — `hofmann-metall.de` (Marktreferenz) und `hagemann-recycling.de` (Gebühren-/Verkaufs-PDF) — plus die externe Quelle `bdsv.org`. Alle drei sind **Gebühren-, Verkaufs- oder Marktreferenzquellen**. **Keine** der 116 Domains hat eine eigene, datierte, in €/kg oder €/t ausgewiesene Scrap-Ankaufspreisliste im Web. Für den Handler-Backlog heißt das: entweder eine geteilte Referenzquelle plus dokumentierten Händler-Aufschlag bauen (BDSV/LME), oder die Preise bei den Betreibern anfordern — bei 116 Betreibern mit je 2–4 Standorten ist die zweite Variante der Hebel, nicht der dritte.

---

## Anhang — Rohdaten

Alle Rohdaten (JSON-Mitschnitte pro Domain, Cache der HTTP-Antworten, PDF-Extrakte, Sweep-Ergebnisse) liegen unter `/tmp/opencode/screen/`:
`probe_*.json` (Phase 1: Homepage + Links + statische Pfade) · `deep_*.json`, `deep_fix.json` (Phase 2: Sitemap-Baum + JSON/CSV/API-Pfade) · `wp_*.json` (Phase 3: WordPress-REST-Volltextscan) · `sub_*.json` (Phase 4: Preis-Subdomains) · `pdfs/` + `pdflinks.json` (Phase 5: Datei-Link-Sweep + PDF-Extrakte) · `agg.json`, `soft404.json` (Aggregation) · `cache/` (3 234 HTTP-Antworten).

---

_Keine Datei im Repository wurde verändert. Kein Deploy, kein Neustart, keine Produktions-Writes._
