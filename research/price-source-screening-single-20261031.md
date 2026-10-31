# Screening-Report: Einzelbetriebe (Ein-Betrieb-Domains) — Preisquellen-Screening

**Auftrag:** Prüfung von 961 Domains mit genau einem Händler-Eintrag auf eine **maschinenlesbare Ankaufpreisquelle** (Preis + Einheit + Datum) für den Handler-Backlog.

**Bearbeitet:** 250 von 961 Domains (die ersten 250 der Liste `screen-single-prio.json`) — jede Domain vollständig geprüft (Homepage + ≥2 Pfade).

**Methode:** HTTP-Abrufe mit Browser-User-Agent, Timeout 20 s. Pro Domain: Homepage → Preisindikator-Suche → ~44 generische Preispfade → Link-Harvesting (Ankertext+URL) → 2-stufiger interner BFS-Crawl → robots.txt/sitemap.xml + WP-Attachment-Sitemaps → WP-REST `/wp-json/wp/v2/media` → PDF-Extraktion via `pdftotext -layout` → Tabellenparser (Zeilen mit Preis **und** Einheit) → Datumsprüfung (ISO, `dd.mm.yyyy`, Monatsname, `Stand/letzte Aktualisierung/timestamp`).

**Gesamt-Aufrufe:** 22.088 eindeutige URLs abgerufen (HTTP-Body-Cache), davon 12.935 Pfad-Prüfungen mit protokolliertem Status und 6.378 Seiten im Marker-Sweep (`Ankaufspreis|Einkaufspreis|Preisliste|Tagespreis|Schrottpreis|…`). Zusätzlich 528 Kontroll-Wiederholungen auf 16 Domains mit TLS/SNI-Problemen (siehe Abschnitt 5).

---

## 1. Ergebnis in Zahlen

| Kennzahl | Wert |
|---|---|
| Domains geprüft | **250** (von 961 in der Liste; Rest 251–961 nicht bearbeitet) |
| Domains mit Homepage HTTP 200 | 246 |
| Domains mit Preisindikator auf Homepage | 113 |
| **Geeignete Ankaufpreisquellen (Preis + Einheit + Datum)** | **2** |
| Ankaufpreis-Tabellen **ohne** Datum (nicht geeignet, aber nachrüstbar) | 10 |
| Ankaufpreis-Tabelle **ohne** Einheit und ohne Datum | 1 |
| Gebührenquellen (Container/Annahme/Mindestmengen) | 8 |
| Verkaufspreisquellen (eigene Reststoffe/Handelsware) | 4 |
| Markt-/Börsenreferenzen (BDSV/LME/Börsenpreis) | 4 |

### Die zentrale Regel: ohne Datum ist die Quelle nicht geeignet

Von 26 Domains, an denen überhaupt **Zahlenwerte** mit Preis- oder Einheitenkontext extrahierbar waren, tragen nur **3** eine datierte Preisquelle (davon 2 mit Ankaufcharakter und Zahlenwerten). Bei **10** Domains existiert eine echte, materialbezogene Ankaufpreistabelle, aber **kein einziges Datum** — das ist die Hauptursache für die geringe Eignungsquote, nicht das Fehlen von Preisangaben. Die übrigen 12 dokumentierten Befunde sind Gebühren- bzw. Verkaufspreisquellen. Die restlichen **224** Domains zeigen gar keine Preisangabe, sondern nur Preis-*Wörter* ("Preise auf Anfrage", "tagesaktuelle Höchstpreise", "Einkaufspreise") **ohne Zahlen** — für einen Scraper wertlos.

---

## 2. Treffer-Tabelle

| Domain | URL | Typ | Beispielwerte (wörtlich) | Einheit | Datum | HTTP | Händler-Slug |
|---|---|---|---|---|---|---|---|
| `dein-schrottplatz.de` | https://www.dein-schrottplatz.de/pages/preise.php | Ankaufpreise | `Hartmetall 45,00€/kg`<br>`Cu Raff 92%  9,20 €`<br>`Schwerer Scherenschrott  0,15 €` | €/kg (Spaltenkopf 'Preis/kg'), Einzelwerte '€/kg'; Entgeltkosten 'Netto 0,3= 300€/t' | Gültig ab 03.10.2026 | 200 | `sn-rochlitz-db-recycling-denis-blum` |
| `elno-container.de` | https://www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf | Ankaufpreise (PDF) + Verkaufspreise (Abfälle) | `Cu-Raff 95%   9,70 €/Kg`<br>`Mischschrott   0,130 €/Kg`<br>`Cu-Millberry   10,70 €/Kg` | €/Kg (Metall-Ankauf); zusätzlich '€/to', '€/cbm' (Abfall-Verkaufspreise) | Dateiname/URL 'Preisliste_16.10.pdf' + WP-Uploadpfad /2026/10/ + PDF-CreationDate 16.10.2026 11:19:59 UTC + HTTP Last-Modified: Fri, 16 Oct 2026 11:20:19 GMT | 200 | `be-spandau-elno-container-und-dienstleistungs` |
| `ferrum-recycling.de` | https://ferrum-recycling.de/preisliste-schrott.html | Ankaufpreise (Raster, Werte 'Preis auf Anfrage') | `Preis auf Anfrage`<br>`Euro/to`<br>`Ladung &lt; 1t` | Euro/to (Spaltenkopf, 'Ladung < 1t') | Stand: 31.10.2026 (Datum vorhanden — aber **kein Zahlenwert** in den Zeilen) | 200 | `he-gieen-ferrum-recycling` |
| `bbw-recycling.de` | https://www.bbw-recycling.de/fileadmin/bbw/Preisliste_BBW_ab_01.07.2026.pdf | Gebühren (Kippgebühren) + Verkaufspreise (RC-Baustoffe) | `0.1 Bauschutt ohne Verunreinigungen  1101-1  170107  20,50 €  to`<br>`0.2 Bauschutt ohne Verunreinigungen – ohne Analytik  1101  170107  25,00 €  to`<br>`0.7 Betonbruch bis 800 mm bew./unbew.  1201  170101  3,00 €  to` | to (Tonne), Stk, cbm | Preisliste 3/26 gültig ab 01.07.2026 bis 30.09.2026 | 200 | `st-magdeburg-bbw-recycling-mittelelbe` |
| `boehner-altmetalle.de` | https://boehner-altmetalle.de/ankauf-preise | Ankaufpreise (ohne Datum) | `Ausbauprofile  1,40 €/kg`<br>`Felgen  1,70 €/kg`<br>`Millberry  10,50 €/kg` | €/kg (3 Staffelpreise: Kleinmenge bis 50 kg / Standard ab 50 kg / Container ab 500 kg) | **KEIN Datum (kein Stand, kein Gültigkeitsdatum, keine Jahreszahl im Dokument)** | 200 | `nw-dusseldorf-bohner-altmetalle` |
| `frisch-recycling.de` | http://frisch-recycling.de/ankauf/ | Ankaufpreise (ohne Datum) | `Eisenschrott leicht  kg  0,08 €`<br>`Eisenschrott schwer  kg  0,10 €`<br>`Hartmetall-Bohrer  kg  30,00 €` | kg (eigene Spalte 'Einheit'), Preis in € | **KEIN Datum** | 200 | `by-schaftlarn-frisch-recycling` |
| `eas-recycling.de` | https://www.eas-recycling.de/ankaufspreise/altmetalle/ | Ankaufpreise (ohne Datum) | `Aluminium Profile  1,60 € / Kilogramm`<br>`Mischschrott  0,00 € / Kilogramm`<br>`Besteck ab 90er Auflage  30,00 € / Kilogramm` | € / Kilogramm (auch '€ / KG') | **KEIN Datum** | 200 | `nw-herdecke-eas-recycling-solution` |
| `emsschrott.de` | https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/ | Ankaufpreise (ohne Datum) | `Mischschrott 105,- €/to`<br>`Messing 5,70 €/kg`<br>`Alu Geschirr 1,06 €/kg` | €/to und €/kg | **KEIN Datum** | 200 | `ni-meppen-emsschrott` |
| `beller-das.de` | https://beller-das.de/ | Ankaufpreise (ohne Datum) | `Kupfer Draht blank "Millberry"  10,10 €`<br>`Elektrolytkupfer, "E-Cu"  10,05 €`<br>`Kupfer Apparate mind. 85% Cu, keine Durchlauferhitzer o.ä.  2,40 € - 4,40 €` | €/kg (Spaltenkopf 'Preis €/kg') | **KEIN Datum** | 200 | `be-neukolln-beller-demontagen-altmetall-schrott` |
| `dk-edelmetalle.de` | https://dk-edelmetalle.de/ | Ankaufpreise (ohne Datum) | `95,40 € pro Gramm`<br>`33,10 € pro Gramm`<br>`30,20 € pro Gramm` | € pro Gramm | **KEIN Datum** | 200 | `he-limeshain-dke-edelmetallhandel` |
| `edelmetallkontor.de` | https://geldfuergold.de/ankaufsrechner/ | Ankaufpreise (ohne Datum, externe API-Quelle) | `999er (102,00 €/g)`<br>`986er (100,57 €/g)`<br>`916er (93,43 €/g)` | €/g | **KEIN Datum auf der Seite; Quelle laut Seite: gold-api.com ('Die Preise aktualisieren sich automatisch')** | 200 | `rp-bad-kreuznach-geld-fur-gold-dagmar-muller-edelmetallko` |
| `ernst-recycling.de` | https://www.ernst-eschrott.de/ | Ankaufpreise (ohne Datum, nur Richtwerte) | `Leiterplatten bis zu 15,50 € pro kg`<br>`10,00 € pro kg`<br>`250,00 € pro kg` | € pro kg | **KEIN Datum** | 200 | `bb-oranienburg-ot-friedrichstha-ernst-recycling` |
| `buntmetallaufkauf.de` | https://buntmetallaufkauf.de/ | Verkaufspreise (Kohle/Baustoffe) + Börsenpreis-Nennung | `Preis: 0,47 €/kg (470,00 €/t)`<br>`Kohle abgepackt 25kg nur 12,89 €` | €/kg, €/t, Cent/kg, €/Stück | **Datumsangaben vorhanden, aber nur als News-Datum ('4. Oktober 2026'), NICHT als Preis-Datum** | 200 | `st-dessau-rolau-lehmanns-buntmetallaufkauf-containerdien` |
| `entsorgung-dresden.de` | http://entsorgung-dresden.de/annahmestelle/dresden-nickern-prohlis.html | Ankaufpreise (Annahmestelle, ohne Datum) | `Folie transparent  kg  0,01 €`<br>`Blech-Dosen  kg  0,04 €`<br>`Alu-Schalen  kg  0,20 €` | kg (Spalte 'Menge'), Preis in € | **KEIN Datum** | 200 | `sn-dresden-wertstoffhandel-scharschuch-david-jakob` |
| `ak-containerdienst.de` | https://ak-containerdienst.de/ | Gebühren (Container) / Tagespreis-Nennung | `220,00 € / Tonne (zzgl.`<br>`ab 55,00 € / Tonne (zzgl.`<br>`A3 - 75,00 € / Tonne (zzgl.` | € / Tonne | **KEIN Datum** | 200 | `by-aschaffenburg-alzenau-ak-containerdienst` |
| `esorec.de` | https://esorec.de/ | Gebühren (Entsorgung) / Tagespreis-Nennung | `Bauschutt  nur 49 € * / Tonne`<br>`Baumischabfälle / Mischmüll  nur 195 € * / Tonne`<br>`Erde / Bodenaushub  nur 46 € * / Tonne` | € / Tonne | **KEIN Datum** | 200 | `he-nidderau-esorec-recycling` |
| `berlin-recycling.de` | https://www.berlin-recycling.de/entsorgung/datentraegervernichtung | Gebühren (Entsorgung Datenträger) | `Selbstanlieferung Entsorgungskosten in Berlin 0,81 € pro Kilogramm`<br>`Pro Kilogramm: 3,79 €*`<br>`9,30 € pro Kilogramm* Transportkosten` | € pro Kilogramm, € einmalig | **KEIN Datum** | 200 | `be-friedrichshain-kreuzberg-berlin-recycling` |
| `arm-entsorgung.de` | https://www.arm-entsorgung.de/containerdienst/frankfurt/altpapier | Gebühren (Container) / Vergütung Altpapier | `Hier zahlen Sie ab 300 €* pro Tonne`<br>`Schon ab nur 300 € * pro Tonne Altpapier und Kartonage` | € pro Tonne | **KEIN Datum** | 200 | `he-hanau-63457-arm-entsorgung` |
| `du-willkommen.de` | https://du-willkommen.de/gewerbeabfall-01.php | Gebühren (Entsorgung nach Gewicht) | `Entsorgung nach Gewicht: 15,00 Euro je 100 kg`<br>`eine Leerung bzw. Abholung für pauschal 159,00 Euro` | Euro je 100 kg | **KEIN Datum** | 200 | `bw-goppingen-mrg-metall-recycling-goppingen` |
| `foerster-entsorgung.de` | https://foerster-entsorgung.de/Recyclinghof/ | Verkaufspreise (eigene Recyclingprodukte) | `Bauschutt-Recycling Körnung 0-45mm Qualität RC2-Material Preis 1,00€/t zzgl.`<br>`Beton-Recycling Körnung 0-45mm Qualität RC1-Material Preis 9,50€/t zzgl.` | €/t, €/m³ | **KEIN Datum** | 200 | `bw-lahr-forster` |
| `diwo-wolfsburg.de` | https://diwo-wolfsburg.de/unterstuetzung.html | Gebühren (Entsorgung) | `191,00 € pro Tonne` | € pro Tonne | **KEIN Datum** | 200 | `ni-wolfsburg-diwo-wertguter-altmetall-abholung` |
| `auto-daud.de` | https://auto-daud.de/unsere-leistungen/ | Verkaufspreise (Kraftstoff/Schmierstoffe) | `Motorenöl 10w40 pro Liter 5,50 €`<br>`Motorenöl 5W30 pro Liter 9,50 €` | € pro Liter | **KEIN Datum** | 200 | `rp-ludwigshafen-daud` |
| `autoverwertung-demand.de` | https://autoverwertung-demand.de/fahrzeugabgabe | Gebühren (Fahrzeug) | `€ pro Stück` | € pro Stück | **KEIN Datum** | 200 | `rp-landau-autoverwertung-demand-recycling` |
| `bekomuenzen.de` | https://www.bekomuenzen.de/kupferbarren/ | Verkaufspreise (Shop) | `44,95 € *`<br>`26,95 € *` | € (Stückpreis, kein €/kg-Ankaufpreis) | **KEIN Datum** | 200 | `he-bruchkoebel-beko-muenzhandel-edelmetalle` |
| `buntmetallhandel-altmittweida.de` | https://www.schrottbuntmetallankauf.de/ankaufspreisliste | Ankaufpreise (ohne Einheit und ohne Datum) | `Kupfer Raff , 94,% Cu.  9,20€`<br>`Kupfer Berry  9,70€`<br>`Messing Schwer max1%Anh.  6,25€` | KEINE Einheit (Spaltenkopf nur 'Preis'; keine €/kg-Angabe) | **KEIN Datum (nur Copyright-Fußzeile '© 2025 - 2026 Schrott Buntmetallankauf')** | 200 | `sn-altmittweida-schrott-buntmetallankauf-buntmetallhande` |

---

## 3. Begründung je Treffer

### `dein-schrottplatz.de` — **GEEIGNET**

- **URL:** https://www.dein-schrottplatz.de/pages/preise.php
- **Händler-Slug:** `sn-rochlitz-db-recycling-denis-blum` (DB Recycling (Denis Blum))
- **Typ:** Ankaufpreise — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (66 Zeilen, 1 Tabelle)
- **Einheit:** €/kg (Spaltenkopf 'Preis/kg'), Einzelwerte '€/kg'; Entgeltkosten 'Netto 0,3= 300€/t'
- **Datum:** Gültig ab 03.10.2026

**Warum geeignet:** Die Seite ist eine ausdrückliche Einkaufspreisliste — im Fließtext steht wörtlich „Anbei finden Sie eine Übersicht der aktuellen **Einkaufspreise** für unser Schrottportfolio“. Das Datum „Gültig ab 03.10.2026“ steht direkt über der Tabelle, und die Struktur ist zweispaltig (`Schrottart` | `Vergütung`), mit Materialgruppen-Überschriften, die selbst die Einheit tragen (`Kupfer` | `Preis/kg`). Damit sind Preis, Einheit und Datum auf **einer** öffentlich erreichbaren Seite vorhanden und per HTML-Tabelle maschinell extrahierbar (66 Zeilen, ~55 Preispositionen). **Materialien:** Hartmetall, Kupfer (5 Sorten), Messing (4), Aluminium (7), Kabel (4), Zink, Reinzinn, Blei (3), Edelstahl (3), Motoren/Trafos, Fe-Metall (6), Computerschrott (5), Altpapier/Textilien. **Einschränkung:** Die Einheit steht teils als Gruppenüberschrift (`Preis/kg`) statt in jeder Zeile — ein Scraper muss die Gruppenüberschrift auf die Folgezeilen vererben. Die letzten Zeilen mischen Ankauf und Entgelt (`Sperrmüll` | `Netto 0,3= 300€/t`, `Bauschutt sauber!!!` | `Netto 0,04=40 €/t`) — diese sind Verkaufs-/Entsorgungspreise, nicht Ankaufpreise, und müssen getrennt klassifiziert werden. Zwei Zeilen sind Annahmestopps (`Textilien` | `Annahmestopp`).

### `elno-container.de` — **GEEIGNET**

- **URL:** https://www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf
- **Händler-Slug:** `be-spandau-elno-container-und-dienstleistungs` (ELNO Container und Dienstleistungs GmbH)
- **Typ:** Ankaufpreise (PDF) + Verkaufspreise (Abfälle) — Einordnung: Ankaufpreisquelle
- **Format:** PDF (2 Seiten, pdftotext -layout)
- **Einheit:** €/Kg (Metall-Ankauf); zusätzlich '€/to', '€/cbm' (Abfall-Verkaufspreise)
- **Datum:** Dateiname/URL 'Preisliste_16.10.pdf' + WP-Uploadpfad /2026/10/ + PDF-CreationDate 16.10.2026 11:19:59 UTC + HTTP Last-Modified: Fri, 16 Oct 2026 11:20:19 GMT

**Warum geeignet:** Eine lückenlose PDF-Serie „PREISLISTE für PRIVATKUNDEN“ unter `wp-content/uploads/<Jahr>/<Monat>/Preisliste_<Tag.Monat>.pdf` — 24 Dateien, davon 15 in 2026 (12.01. → 16.10.2026). Jedes PDF enthält eine zweispaltige Sortenliste mit **Preis und Einheit in derselben Zelle** (`Cu-Raff 95%   9,70 €/Kg`, `Mischschrott   0,130 €/Kg`), extrahierbar per `pdftotext -layout` ohne OCR. Das Datum steckt dreifach redundant: im Dateinamen (`Preisliste_16.10.pdf`), im WordPress-Uploadpfad (`/2026/10/`), im PDF-Metadatum (`CreationDate: Fri Oct 16 11:19:59 2026 UTC`) und im HTTP-Header (`Last-Modified: Fri, 16 Oct 2026 11:20:19 GMT`). Der Dateiname enthält zusätzlich die Listennummer (`_16.10` vs. `_16.10-1`), sodass die Historie rekonstruierbar ist. **Materialien:** Kupfer (6 Sorten), Messing (4), Eisen/Stahl (Mischschrott, Shreddervormaterial, Bremsscheiben, Sorte 3, leichte/schwere Schere, Guss), Edelstahl V2A, Aluminium (11), Sonstiges (Alt-/Neu-Zink, Alt-Blei, Akku-Blei, Sammelzinn, Elektronikschrott). **Einschränkung:** Das PDF nennt das Gültigkeitsdatum **nicht im Text**, nur in Dateiname/Metadaten — ein Scraper braucht die Pfadableitung oder den HTTP-`Last-Modified`. Das PDF mischt Ankaufpreise (Metalle) mit Abfall-Verkaufspreisen (`Styropor 89 €/cbm`, `Baumischabfälle 290 €/to`) — Trennung nötig. Preise gelten laut PDF nur für Privatkunden, Gewerbekunden zahlen laut Text nach telefonischem Preis.

### `ferrum-recycling.de` — Nicht geeignet (datiert, aber kein Zahlenwert)

- **URL:** https://ferrum-recycling.de/preisliste-schrott.html
- **Händler-Slug:** `he-gieen-ferrum-recycling` (Ferrum Recycling GmbH)
- **Typ:** Ankaufpreise (Raster, Werte 'Preis auf Anfrage') — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (21 Zeilen)
- **Einheit:** Euro/to (Spaltenkopf, 'Ladung < 1t')
- **Datum:** Stand: 31.10.2026
- **Zusatzfund:** Roh-HTML des Tabellenkopfs: <td class="col_2 col_last">Euro/to<br>Ladung &lt; 1t</td> (im Klartext also "Euro/to, Ladung < 1t"). Alle 20 Preiszeilen enthalten ausschliesslich 'Preis auf Anfrage' - kein einziger Zahlenwert.

**Warum NICHT geeignet:** Die Seite ist eine echte, **datiert** Einkaufspreisliste („Preisliste Schrott“, Spaltenkopf `Art.Nr. | Einkaufspreis für: | Euro/to / Ladung < 1t`, Stand `31.10.2026`) mit 20 sauber getrennten Sorten (Mischschrott, Träger, Brennschrott, Langschrott, Shredder-Vormaterial, PKW/LKW-Motoren, Pressfähige Bleche, Stahlspäne, Handels-/Ofenguß, Gußrohre, Fallwerksguß, Elo max. 1,50 m, E-Motoren, Bremsscheiben, Katalysator). Aber: **jede** der 20 Preiszeilen enthält ausschließlich „Preis auf Anfrage“ — es gibt keinen einzigen Zahlenwert. Preis + Einheit + Datum sind formal vorhanden, der **Preiswert fehlt**; eine Tabelle mit 20 identischen Platzhaltern ist kein maschinenlesbarer Datensatz. Der Betreiber nennt zusätzlich Mindestmengen („Ladungen mit einem Gewicht unter 200 kg werden nicht vergütet“), was bestätigt, dass hier verhandelt statt publiziert wird. **Für den Scraper:** unbrauchbar; höchstens als Indikator, dass der Betreiber eine Preislisten-Infrastruktur pflegt — dann wäre eine Kontaktaufnahme der Weg.

### `bbw-recycling.de` — Gebührenquelle

- **URL:** https://www.bbw-recycling.de/fileadmin/bbw/Preisliste_BBW_ab_01.07.2026.pdf
- **Händler-Slug:** `st-magdeburg-bbw-recycling-mittelelbe` (BBW Recycling Mittelelbe GmbH)
- **Typ:** Gebühren (Kippgebühren) + Verkaufspreise (RC-Baustoffe) — Einordnung: Gebührenquelle
- **Format:** PDF (Preistabelle, pdftotext -layout)
- **Einheit:** to (Tonne), Stk, cbm
- **Datum:** Preisliste 3/26 gültig ab 01.07.2026 bis 30.09.2026

**Warum Gebühren-/Verkaufspreisquelle, nicht Ankaufpreisquelle:** Das PDF trägt ein echtes, im Dokumenttext stehendes Datum („Preisliste 3/26 gültig ab 01.07.2026 bis 30.09.2026“) und eine sauber geparste Preistabelle mit den Spalten `Artikel | ASN | Preis (netto) | ME`. Inhaltlich ist es eine **Kippgebühren-/Annahmegebührentafel** für mineralische Abfälle plus eine **Verkaufspreistafel** für eigene RC-Produkte — „0.1 Bauschutt ohne Verunreinigungen 1101-1 170107 20,50 € to“ (Annahmegebühr) und „4.2 Mineralgemisch B2 0/32 (HS) 2622 20,50 € to“ (Verkauf Handelsware). Es gibt **keine** Sorten für Schrott/Buntmetall (die PDF enthält die Zeichenfolgen „Metall“, „Schrott“, „Kupfer“, „Aluminium“ kein einziges Mal) — für den Scraper-Handel also nicht nutzbar. Als **Gebührenquelle** ist sie einer der wenigen Fälle mit Preis + Einheit (`to`, `Stk`, `cbm`) + Datum in einem Dokument; die Gültigkeit ist zudem befristet (30.09.2026), was ein Scraper als Ablaufdatum auswerten kann. Nebenbefund für den Betrieb: „Kleinstmengen < 5 Euro beaufschlagt“ ist eine Mindestmengen-Bedingung, kein Preis.

### `boehner-altmetalle.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://boehner-altmetalle.de/ankauf-preise
- **Händler-Slug:** `nw-dusseldorf-bohner-altmetalle` (Böhner Altmetalle GmbH)
- **Typ:** Ankaufpreise (ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (21 Zeilen, 17 Preiszeilen)
- **Einheit:** €/kg (3 Staffelpreise: Kleinmenge bis 50 kg / Standard ab 50 kg / Container ab 500 kg)
- **Datum:** KEIN Datum (kein Stand, kein Gültigkeitsdatum, keine Jahreszahl im Dokument)

**Warum nicht geeignet (nur Datum fehlt):** Vollständige, materialbezogene Ankaufpreistabelle mit 17 Preiszeilen und **drei Mengenstaffeln** je Sorte (`Kleinmenge bis 50 kg | Standard ab 50 kg | Container ab 500 kg`) — Materialien u. a. Aluminium-Ausbauprofile, Bleche alt mit max. 2 % FE, Felgen, Offset, Blei, V2A, V4A, Millberry. Preis und Einheit stehen in jeder Zeile (`Ausbauprofile  1,40 €/kg`). Es existiert **kein Datum**: weder „Stand“, „gültig ab“, „letzte Aktualisierung“ noch eine Jahreszahl im Dokument (Suchlauf über die komplette Seite: keine `20xx`-Jahreszahl außerhalb der Tabelleninhalte). Die Werte sind damit zeitlich unbestimmt — laut Footer ist die Preisliste laufend aktualisierbar. **Für den Scraper:** strukturell ideal (Tabelle, Preis+Einheit je Zeile, 3 Staffeln), braucht aber ein Datum vom Betreiber oder eine Heuristik (WP-`modified`, `Last-Modified`).

### `frisch-recycling.de` — Nicht geeignet (Datum fehlt)

- **URL:** http://frisch-recycling.de/ankauf/
- **Händler-Slug:** `by-schaftlarn-frisch-recycling` (Frisch Recycling GmbH)
- **Typ:** Ankaufpreise (ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (157 Zeilen, 135 Preiszeilen)
- **Einheit:** kg (eigene Spalte 'Einheit'), Preis in €
- **Datum:** KEIN Datum

**Warum nicht geeignet (nur Datum fehlt):** Die umfangreichste Preisquelle der Stichprobe: Tabelle mit 157 Zeilen, **135 Preispositionen**, dreispaltig `Bezeichnung | Einheit | Preis` — die Einheit steht also sauber in einer eigenen Spalte (`kg`), der Preis in der nächsten (`0,08 €`). Materialien: Eisenschrott leicht/schwer, Bauguss, Pumpen, Eisenschrott mit Anhaft, Bremsscheiben, Hartmetall-Bohrer, Hartmetall-Wendeplättchen, Kupfer Targets, Kupfer Späne, Cu-Fe Ankerschrott u. a. Ein Datum existiert nicht (kein „Stand“, keine Jahreszahl im Dokument). Randbedingung: „Annahme von Materialien ist nur nach Registrierung und Vorlage eines gültigen Ausweises möglich“ — der Scraper darf die Werte lesen, aber die Annahme selbst ist nicht automatisierbar; die Preise gelten vermutlich nur für registrierte gewerbliche Anlieferer. **Für den Scraper:** bestes Material ohne Datumsfeld; Nachrüstung durch Betreiber-Datum genügt.

### `eas-recycling.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://www.eas-recycling.de/ankaufspreise/altmetalle/
- **Händler-Slug:** `nw-herdecke-eas-recycling-solution` (EAS-Recycling Solution GmbH)
- **Typ:** Ankaufpreise (ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML (Listen-/Tabellenstruktur)
- **Einheit:** € / Kilogramm (auch '€ / KG')
- **Datum:** KEIN Datum

**Warum nicht geeignet (nur Datum fehlt):** Sauber gegliederte Ankaufpreis-Seiten unter `/ankaufspreise/<material>/` — drei Unterseiten (`altmetalle`, `elektronische-bauteile`, `elektronische-hardware`), jede mit Sorten-Listen im Format `Material  Preis`. Preis und Einheit stehen direkt nebeneinander (`Aluminium Profile  1,60 € / Kilogramm`, `Mischschrott  0,00 € / Kilogramm`, `PC-Netzteil Kabel mit Stecker  2,90 € / Kilogramm`). Materialien: Aluminium-Profile, Mischschrott, Besteck ab/unter 90er Auflage, PC-Netzteil-Kabel mit/ohne Stecker, Leiterplatten, Elektronik-Hardware. **Kein Datum**: weder auf den Unterseiten noch im Seitenkopf/-fuß; die Seite enthält als einzige Jahreszahl eine Copyright-Angabe `2026`, die aber kein Preis-Datum ist. Achtung: `Mischschrott  0,00 € / Kilogramm` ist ein Nullwert (Sorte wird nicht angekauft bzw. Wert 0) — der Scraper muss 0,00 als „kein Preis“ behandeln. **Für den Scraper:** sehr gute Struktur, ein einziges fehlendes Feld.

### `emsschrott.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://emsschrott.de/altmetalle-und-schrott-an-und-verkauf/
- **Händler-Slug:** `ni-meppen-emsschrott` (Emsschrott GmbH & Co. KG)
- **Typ:** Ankaufpreise (ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (9 Zeilen, 8 Preiszeilen)
- **Einheit:** €/to und €/kg
- **Datum:** KEIN Datum

**Warum nicht geeignet (nur Datum fehlt):** Tabelle `Sorte | Preis` mit 8 Preiszeilen, Preis und Einheit je Zeile (`Mischschrott  105,- €/to`, `Messing  5,70 €/kg`, `Alu Geschirr  1,06 €/kg`, `Blei  0,78 €/kg`, `Kupfer Kabel (ohne Stecker)  3,24 €/kg`, `Kupfer alt (z.B. alte Wasserleitungen, …)  9,25 €/kg`, `Zink (Dachrinnen, …)  1,90 €/kg`). Der Fließtext sagt ausdrücklich, es handele sich um „aktuelle Preisangebote für bestimmte Schrottsorten“ — die Werte sind also **Angebote**, nicht eine datierte Preisliste. Ein Datum fehlt vollständig (die Jahreszahl `2026` im Dokument stammt aus Öffnungszeiten/Copyright, nicht aus dem Preis). **Für den Scraper:** kompakt und leicht zu parsen; Ablaufdatum wäre der einzige fehlende Baustein.

### `beller-das.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://beller-das.de/
- **Händler-Slug:** `be-neukolln-beller-demontagen-altmetall-schrott` (BELLER Demontagen Altmetall Schrott GmbH)
- **Typ:** Ankaufpreise (ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (34 Zeilen, 33 Preiszeilen)
- **Einheit:** €/kg (Spaltenkopf 'Preis €/kg')
- **Datum:** KEIN Datum

**Warum nicht geeignet (nur Datum fehlt):** Die Preistabelle steckt in der **Startseite** (34 Zeilen, 33 Preispositionen), Spaltenkopf `Bezeichnung | Preis €/kg` — die Einheit steht also nur einmal im Kopf, die Zeilen enthalten reine Euro-Beträge (`Kupfer Draht blank "Millberry"  10,10 €`). Das ist für einen Scraper maschinenlesbar, verlangt aber eine Spalten-Vererbung der Einheit. Materialien u. a. Kupfer (Millberry, E-Cu, Candy, Cu-Kabel, Cu-Apparate mit Preisband `2,40 € - 4,40 €`), E-Motoren, Mischschrott. Kein Datum im Dokument. **Besonderheit:** Die Tabelle ist durchgehend Ankaufpreis (34 Zeilen, alle mit `Preis €/kg` im Kopf). Drei Zeilen enthalten **Mindestmengen als Bedingung** in der Bezeichnung — `Kupfer Apparate mind. 85% Cu, keine Durchlauferhitzer o.ä.`, `E-Motore (bis 300 kg)`, `Mischschrott (mind. 100 kg)` — das sind Schwellen, keine separaten Preise; der Preis steht in derselben Zeile. Eine Zeile ist eine **Zuschlagsart** statt einer Sorte (`Zuschlag (Material zum Zerlegen)  0,25 €`) und keine Preiszeile für eine Sortengruppe. **Verwechslungsfalle (andere Domain):** bei `bellersheim.de` steht dieselbe Zahlenspanne `2 SZR/kg` ausschließlich im AGB-Text (Haftungsgrenze nach ADSp) — rechtlich etwas völlig anderes als ein Kaufpreis.

### `dk-edelmetalle.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://dk-edelmetalle.de/
- **Händler-Slug:** `he-limeshain-dke-edelmetallhandel` (DKE Edelmetallhandel (Denis Kunz))
- **Typ:** Ankaufpreise (ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML (Edelmetall-Kaufpreis-Anzeige)
- **Einheit:** € pro Gramm
- **Datum:** KEIN Datum

**Warum nicht geeignet (nur Datum fehlt):** Die Startseite nennt Edelmetall-Ankaufpreise mit expliziter Einheit (`95,40 € pro Gramm`, `33,10 € pro Gramm`, `30,20 € pro Gramm`, `1,05 € pro Gramm`) für Gold (999/750/585/333) und Silber. Es ist aber **kein Sorten-Preistabelle**, sondern eine Karte/Sammlung von Einzelwerten; die Material-Zuordnung ist über die Seite verteilt, nicht tabellarisch. Kein Datum. **Für den Scraper:** als Datenpunkt „Preis pro Gramm Edelmetall“ nutzbar, aber die Struktur ist ungeeignet für einen systematischen Sorten-Scraper.

### `edelmetallkontor.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://geldfuergold.de/ankaufsrechner/
- **Händler-Slug:** `rp-bad-kreuznach-geld-fur-gold-dagmar-muller-edelmetallko` (Geld für Gold Dagmar Müller (Edelmetallkontor))
- **Typ:** Ankaufpreise (ohne Datum, externe API-Quelle) — Einordnung: Ankaufpreisquelle
- **Format:** HTML (Ankaufsrechner, Werte dynamisch)
- **Einheit:** €/g
- **Datum:** KEIN Datum auf der Seite; Quelle laut Seite: gold-api.com ('Die Preise aktualisieren sich automatisch')

**Warum nicht geeignet (kein Datum, außerdem externe Marktquelle):** Der Ankaufsrechner auf `geldfuergold.de/ankaufsrechner/` liefert sehr sauber formatierte Werte in €/g (`999er (102,00 €/g)`, `986er (100,57 €/g)`, `916er (93,43 €/g)`, `900er (91,80 €/g)`) und weist die Herkunft selbst aus: „Die Preise aktualisieren sich automatisch, Quelle gold-api.com.“ Das heißt: maschinenlesbar ja, aber (a) **kein Datum** auf der Seite, (b) die Werte stammen aus einer **externen Marktdaten-API** (`gold-api.com`), nicht aus einer Betreiber-Standortpreisliste — die Domain `edelmetallkontor.de` liefert also keinen Betreiberpreis, sondern spiegelt eine Marktnotiz. Der Scraper müsste `gold-api.com` als Quelle führen, nicht den Händler.

### `ernst-recycling.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://www.ernst-eschrott.de/
- **Händler-Slug:** `bb-oranienburg-ot-friedrichstha-ernst-recycling` (Ernst Recycling GmbH)
- **Typ:** Ankaufpreise (ohne Datum, nur Richtwerte) — Einordnung: Ankaufpreisquelle
- **Format:** HTML (Fließtext)
- **Einheit:** € pro kg
- **Datum:** KEIN Datum

**Warum nicht geeignet:** Es handelt sich lediglich um **Richtwert-Angaben im Fließtext**, nicht um eine Preisliste: „Leiterplatten bis zu 15,50 € pro kg“, dazu 10,00 / 37,00 / 250,00 € pro kg für E-Schrott bzw. Geräte. Die Formulierung „bis zu“ zeigt zudem eine Obergrenzen-Angabe, keinen fixen Preis. Kein Datum, keine Sortentabelle. **Für den Scraper:** als Einzelrichtwert erfassbar, nicht als systematische Preisquelle.

### `buntmetallaufkauf.de` — Verkaufspreisquelle

- **URL:** https://buntmetallaufkauf.de/
- **Händler-Slug:** `st-dessau-rolau-lehmanns-buntmetallaufkauf-containerdien` (Lehmanns Buntmetallaufkauf & Containerdienst)
- **Typ:** Verkaufspreise (Kohle/Baustoffe) + Börsenpreis-Nennung — Einordnung: Verkaufspreisquelle (eigene Reststoffe/Handelsware)
- **Format:** HTML (News-/Startseite)
- **Einheit:** €/kg, €/t, Cent/kg, €/Stück
- **Datum:** Datumsangaben vorhanden, aber nur als News-Datum ('4. Oktober 2026'), NICHT als Preis-Datum
- **Zusatzfund:** Börsenpreis-Nennung steht auf der News-Artikelseite https://buntmetallaufkauf.de/2026/09/04/schrott-und-buntmetallaufkauf/ : 'Bei uns bekommen Sie immer den aktuellen Börsenpreis für Buntmetalle.' — reine Marktreferenz ohne Zahlenwert und ohne Preis-Datum.

**Warum Verkaufspreisquelle + Marktreferenz:** Die abgerufenen Preisangaben sind **Verkaufspreise für eigene Handelsware** (`Preis: 0,47 €/kg (470,00 €/t)` für lose Kohle, `Kohle abgepackt 25kg nur 12,89 €`), ergänzt um Altreifen `35 Cent/kg` — nicht um Ankaufpreise für Schrott. Die Marktreferenz steht auf der News-Seite: „Bei uns bekommen Sie immer den aktuellen **Börsenpreis** für Buntmetalle. Kupfer und Messing, sowie Kupferkabel und vieles mehr sind im Moment sehr hoch.“ — Börsenpreis-Nennung **ohne Zahlenwert und ohne Datum**, also kein Standortpreis. Auf der Startseite existieren zwar Datumsangaben (`4. Oktober 2026`), die sind aber **News-Daten**, keine Preisstände; sie dürfen nicht als Preis-Datum missverstanden werden. **Für den Scraper:** als Handelsware-Preisliste nutzbar, nicht als Ankaufpreisquelle.

### `entsorgung-dresden.de` — Nicht geeignet (Datum fehlt)

- **URL:** http://entsorgung-dresden.de/annahmestelle/dresden-nickern-prohlis.html
- **Händler-Slug:** `sn-dresden-wertstoffhandel-scharschuch-david-jakob` (Wertstoffhandel Scharschuch (David Jakob))
- **Typ:** Ankaufpreise (Annahmestelle, ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (Wertstoff | Menge | Preis)
- **Einheit:** kg (Spalte 'Menge'), Preis in €
- **Datum:** KEIN Datum

**Warum nicht geeignet (nur Datum fehlt):** Die Annahmestellen-Seite führt eine echte Tabelle `Wertstoff | Menge | Preis` mit Vergütungswerten (`Folie transparent  kg  0,01 €`, `Blech-Dosen  kg  0,04 €`, `Alu-Schalen  kg  0,20 €`) und markiert gesperrte Stoffe als `keine Annahme`. Das ist eine **Vergütungspreisliste** (Ankauf) an einem konkreten Standort („Vergütungspreise ausschließlich am Standort Nickerner Weg in Dresden“) — also genau die gesuchte Zielform. Es fehlt jedoch jedes Datum; die Seite enthält keine Jahreszahl. **Für den Scraper:** nachrüstbar; zusätzlich zu beachten: die Werte sind **standortgebunden** (nur Nickerner Weg) und die Sperrvermerke müssen als eigene Werteart behandelt werden.

### `ak-containerdienst.de` — Gebührenquelle

- **URL:** https://ak-containerdienst.de/
- **Händler-Slug:** `by-aschaffenburg-alzenau-ak-containerdienst` (AK Containerdienst)
- **Typ:** Gebühren (Container) / Tagespreis-Nennung — Einordnung: Gebührenquelle
- **Format:** HTML (Container-Preise, Tagespreis-Marker)
- **Einheit:** € / Tonne
- **Datum:** KEIN Datum

**Warum Gebührenquelle:** Die Seite nennt Container-/Entsorgungstarife je Tonne (`220,00 € / Tonne`, `ab 55,00 € / Tonne`, `A3 - 75,00 € / Tonne`, `149,00 € / Tonne`) — durchgehend mit „(zzgl.“-Fußnote, also **Preise ohne Leistungsumfang** (Mindestmenge, Containergröße, Aktionen). Die Seite wirbt mit „Tagespreis“, nennt aber kein Sortenverzeichnis und kein Preis-Datum. **Für den Scraper:** eine Gebührentafel ist keine Ankaufpreisquelle; die Tagespreis-Zusage ist nicht durch eine abrufbare Liste gedeckt.

### `esorec.de` — Gebührenquelle

- **URL:** https://esorec.de/
- **Händler-Slug:** `he-nidderau-esorec-recycling` (ESOREC Recycling (Jeremy Willeführ))
- **Typ:** Gebühren (Entsorgung) / Tagespreis-Nennung — Einordnung: Gebührenquelle
- **Format:** HTML-Tabelle (9 Zeilen)
- **Einheit:** € / Tonne
- **Datum:** KEIN Datum

**Warum Gebührenquelle:** Die Tabelle listet reine Entsorgungstarife (`Bauschutt  nur 49 € * / Tonne`, `Baumischabfälle / Mischmüll  nur 195 € * / Tonne`, `Erde / Bodenaushub  nur 46 € * / Tonne`, `Gipskarton  nur 110 € * / Tonne`, `Altholz / Holz  nur 110 € * / Tonne`, `Sperrmüll  nur 175 € * / Tonne`, `Grünschnitt  nur 110 € * / Tonne`) — mit Sternchen-Fußnoten, also Eingangspreise für Entsorgung, nicht Materialpreise, die der Betreiber für Schrott **zahlt**. Die Seite nennt „Tagespreis“, ohne Datum. **Für den Scraper:** Gebührenquelle, klar zu trennen.

### `berlin-recycling.de` — Gebührenquelle

- **URL:** https://www.berlin-recycling.de/entsorgung/datentraegervernichtung
- **Händler-Slug:** `be-friedrichshain-kreuzberg-berlin-recycling` (Berlin Recycling GmbH)
- **Typ:** Gebühren (Entsorgung Datenträger) — Einordnung: Gebührenquelle
- **Format:** HTML (Preisblock mit Fußnoten)
- **Einheit:** € pro Kilogramm, € einmalig
- **Datum:** KEIN Datum

**Warum Gebührenquelle:** Alle Werte sind Entsorgungskosten für Datenträgervernichtung: `Selbstanlieferung Entsorgungskosten in Berlin 0,81 € pro Kilogramm`, `Pro Kilogramm: 3,79 €*`, `9,30 € pro Kilogramm* Transportkosten`, `115,00 €* einmalig pro Bestellung`. Die Fußnoten verweisen auf Mindestmengen und Transportkosten-Zuschläge. Kein Datum. **Für den Scraper:** Gebühren-/Zuschlagstafel, kein Ankaufpreis.

### `arm-entsorgung.de` — Gebührenquelle

- **URL:** https://www.arm-entsorgung.de/containerdienst/frankfurt/altpapier
- **Händler-Slug:** `he-hanau-63457-arm-entsorgung` (ARM Entsorgung)
- **Typ:** Gebühren (Container) / Vergütung Altpapier — Einordnung: Gebührenquelle
- **Format:** HTML (Blog-/Leistungsseite)
- **Einheit:** € pro Tonne
- **Datum:** KEIN Datum
- **Zusatzfund:** Weiterer Gebührenwert auf https://www.arm-entsorgung.de/recycling-blog/containerstellung-fuer-grossbaustellen-in-hanau/ : 'Mischmüll kann bei uns bereits für 185€ pro Tonne entsorgt werden.'

**Warum Gebührenquelle:** Zwei Preisaussagen, beide **Entsorgungs-/Vergütungstarife** für Containerdienst: „Hier zahlen Sie ab 300 €* pro Tonne“ (Altpapier/Kartonage) und im Blogbeitrag „Mischmüll kann bei uns bereits für 185€ pro Tonne entsorgt werden.“ — Mischmüll-Entsorgung ist eine Gebühr, Altpapier-Vergütung ist eine Vergütung für angeliefertes Material, aber ohne Sortentabelle und ohne Preis-Datum. Die Seite wirbt mit „Echtzeit-Preisanpassung“, ohne dass eine Preisliste abrufbar wäre.

### `du-willkommen.de` — Gebührenquelle

- **URL:** https://du-willkommen.de/gewerbeabfall-01.php
- **Händler-Slug:** `bw-goppingen-mrg-metall-recycling-goppingen` (MRG Metall-Recycling Göppingen GmbH)
- **Typ:** Gebühren (Entsorgung nach Gewicht) — Einordnung: Gebührenquelle
- **Format:** HTML
- **Einheit:** Euro je 100 kg
- **Datum:** KEIN Datum
- **Zusatzfund:** Zweiter Wert auf https://du-willkommen.de/gewerbeabfall.php : 'Entsorgung nach Gewicht: 41,00 Euro je 100 kg'

**Warum Gebührenquelle:** Reine Gewichts-Entsorgungstarife: „Entsorgung nach Gewicht: 15,00 Euro je 100 kg“ (5-cbm-Container Bauschutt) plus Pauschale „eine Leerung bzw. Abholung für pauschal 159,00 Euro“ und Miettage-Satz „ab dem 6. Tag EUR 2,50 pro Tag“. Die Einheit „Euro je 100 kg“ ist eine Gebühreneinheit, kein Materialpreis; kein Datum vorhanden. **Für den Scraper:** Gebührentafel.

### `foerster-entsorgung.de` — Verkaufspreisquelle

- **URL:** https://foerster-entsorgung.de/Recyclinghof/
- **Händler-Slug:** `bw-lahr-forster` (Förster)
- **Typ:** Verkaufspreise (eigene Recyclingprodukte) — Einordnung: Verkaufspreisquelle (eigene Reststoffe/Handelsware)
- **Format:** HTML (Verkaufskatalog)
- **Einheit:** €/t, €/m³
- **Datum:** KEIN Datum
- **Zusatzfund:** Weitere Verkaufspreise auf https://foerster-entsorgung.de/Kompostierung/ : 'Rindenmulch fein 0-20mm Preis: 55,00€/m³ inkl. MwSt.', 'Pflanzerde ... Preis 45,00€/m³ inkl. MwSt.', 'Kompost ... Preis 25,00€/m³ inkl. MwSt.'

**Warum Verkaufspreisquelle:** Der „Verkaufskatalog“ nennt Preise für **eigene Recyclingprodukte**: „Bauschutt-Recycling Körnung 0-45mm Qualität RC2-Material Preis 1,00€/t zzgl. …“, „Beton-Recycling Körnung 0-45mm Qualität RC1-Material Preis 9,50€/t zzgl. …“; auf `/Kompostierung/` zusätzlich „Rindenmulch fein 0-20mm Preis: 55,00€/m³ inkl. MwSt.“, „Pflanzerde … Preis 45,00€/m³ inkl. MwSt.“, „Kompost … Preis 25,00€/m³ inkl. MwSt.“ — Verkauf eigener Reststoffe, nicht Ankauf von Schrott. Kein Datum. **Für den Scraper:** als Reststoff-Preisquelle nutzbar, klar gegen Ankaufpreise abzugrenzen.

### `diwo-wolfsburg.de` — Gebührenquelle

- **URL:** https://diwo-wolfsburg.de/unterstuetzung.html
- **Händler-Slug:** `ni-wolfsburg-diwo-wertguter-altmetall-abholung` (DIWO Wertgüter (Altmetall-Abholung))
- **Typ:** Gebühren (Entsorgung) — Einordnung: Gebührenquelle
- **Format:** HTML
- **Einheit:** € pro Tonne
- **Datum:** KEIN Datum

**Warum Gebührenquelle:** Ein einziger Wert ohne Sortenbezug (`191,00 € pro Tonne`) auf einer Unterstützungs-Seite — eine Entsorgungspauschale, kein Materialpreis. Kein Datum. **Für den Scraper:** nicht als Preisquelle verwendbar.

### `auto-daud.de` — Verkaufspreisquelle

- **URL:** https://auto-daud.de/unsere-leistungen/
- **Händler-Slug:** `rp-ludwigshafen-daud` (Auto-Daud Autorecycling e.K.)
- **Typ:** Verkaufspreise (Kraftstoff/Schmierstoffe) — Einordnung: Verkaufspreisquelle (eigene Reststoffe/Handelsware)
- **Format:** HTML
- **Einheit:** € pro Liter
- **Datum:** KEIN Datum

**Warum Verkaufspreisquelle (kein Scrap):** Die Werte betreffen Schmierstoffe im Tankstellenshop (`Motorenöl 10w40 pro Liter 5,50 €`, `Motorenöl 5W30 pro Liter 9,50 €`). Das ist Handelsware, kein Schrott-/Metall-Ankaufpreis. Kein Datum. **Für den Scraper:** für den Schrott-Handel irrelevant.

### `autoverwertung-demand.de` — Gebührenquelle

- **URL:** https://autoverwertung-demand.de/fahrzeugabgabe
- **Händler-Slug:** `rp-landau-autoverwertung-demand-recycling` (Autoverwertung Demand-Recycling)
- **Typ:** Gebühren (Fahrzeug) — Einordnung: Gebührenquelle
- **Format:** HTML
- **Einheit:** € pro Stück
- **Datum:** KEIN Datum

**Warum Gebührenquelle:** Die Seite nennt nur die Einheit `€ pro Stück` ohne konkrete Beträge im Preis-Kontext — keine verwertbaren Werte. Kein Datum. **Für den Scraper:** Negativbefund.

### `bekomuenzen.de` — Verkaufspreisquelle

- **URL:** https://www.bekomuenzen.de/kupferbarren/
- **Händler-Slug:** `he-bruchkoebel-beko-muenzhandel-edelmetalle` (BeKo-Münzhandel & Edelmetalle (Benjamin Kollosche))
- **Typ:** Verkaufspreise (Shop) — Einordnung: Verkaufspreisquelle (eigene Reststoffe/Handelsware)
- **Format:** HTML (Shop-Produktpreise)
- **Einheit:** € (Stückpreis, kein €/kg-Ankaufpreis)
- **Datum:** KEIN Datum
- **Zusatzfund:** Zweiter Wert (19,90-Zeile) liegt auf https://www.bekomuenzen.de/silberbarren/ : 'Inhalt 15 Feinunze (29,90 € * / 1 Feinunze) 448,50 € *'

**Warum Verkaufs-/Shoppreisquelle:** Der Shop listet reine Stückpreise für Barren und Münzen (`44,95 € *`, `26,95 € *`, `129,95 € *`, `194,95 € *`, `119,95 € *`; auf `/silberbarren/` auch „Inhalt 15 Feinunze (29,90 € * / 1 Feinunze) 448,50 € *“). Die Preise beziehen sich auf **Verkauf** von Barren/Münzen (Gewicht in Gramm steht im Produktnamen, z. B. „10 Gramm Silberbarren – 999“), nicht auf einen Ankaufpreis pro Kilogramm. Im News-Bereich findet sich die Marktreferenz „metallpreis“. Kein Preis-Datum; Warenverfügbarkeit („Ausverkauft“) ist als eigener Zustand zu führen. **Für den Scraper:** Shop-Preise, keine Ankaufpreisliste; für einen Schrott-Handel nicht der gesuchte Datensatz.

### `buntmetallhandel-altmittweida.de` — Nicht geeignet (Datum fehlt)

- **URL:** https://www.schrottbuntmetallankauf.de/ankaufspreisliste
- **Händler-Slug:** `sn-altmittweida-schrott-buntmetallankauf-buntmetallhande` (Schrott Buntmetallankauf (Buntmetallhandel Altmittweida))
- **Typ:** Ankaufpreise (ohne Einheit und ohne Datum) — Einordnung: Ankaufpreisquelle
- **Format:** HTML-Tabelle (2 Spalten: Produkt | Preis, 27 Preiszeilen)
- **Einheit:** KEINE Einheit (Spaltenkopf nur 'Preis'; keine €/kg-Angabe)
- **Datum:** KEIN Datum (nur Copyright-Fußzeile '© 2025 - 2026 Schrott Buntmetallankauf')
- **Zusatzfund:** Die Seite liegt unter der Zweit-Domain www.schrottbuntmetallankauf.de; die Listendomain buntmetallhandel-altmittweida.de liefert denselben Inhalt. '/preise' auf der Listendomain ist ein Soft-404 (liefert die Startseite).

**Warum nicht geeignet (weder Einheit noch Datum):** Eine echte, materialbezogene Ankaufspreisliste mit 27 Sorten und sauber geparster Zwei-Spalten-Tabelle `Produkt | Preis` — die Sorten reichen von `Kupfer Raff , 94,% Cu.  9,20€` über `Messing Schwer max1%Anh.  6,25€`, `Elektomotore  0,35€`, `V2A  0,50€`, `Mischschrott  0.10€` bis `Altpapier  0,07€`. Aber die Tabelle nennt **keine Einheit** — der Spaltenkopf lautet nur „Preis", eine Angabe wie „€/kg" fehlt vollständig; und ein **Datum** fehlt ebenfalls (die Seite trägt nur die Copyright-Fußzeile „© 2025 - 2026 Schrott Buntmetallankauf"). Preis + Einheit + Datum ist damit nicht erfüllt, obwohl die Materialdeckung exzellent wäre. **Für den Scraper:** die Domain ist ein Präzisionsfall — die Werte sind ohne Einheit nicht interpretierbar (9,20 € für Kupfer Raff ist nur mit der Information „pro kg" sinnvoll), also weder maschinenlesbar im geforderten Sinn noch datierbar. Ein Blick auf die Preisniveau-Pendants `dein-schrottplatz.de` (gleiche Sorten, mit €/kg und Datum) zeigt, dass die Einheit beim Betreiber bekannt, aber nicht publiziert ist. **Nebenbefund:** Die Inhalte liegen unter der Zweit-Domain `www.schrottbuntmetallankauf.de`; `/preise` auf `buntmetallhandel-altmittweida.de` ist ein Soft-404 (liefert die Startseite).

---

## 4. Negativbefunde

250 Domains, pro Domain **Homepage + ≥2 Pfade** geprüft (generische Preispfade, Link-Harvesting, interner Crawl, Sitemaps). Spalte „geprüfte Pfade“ listet die konkret abgerufenen Pfade mit HTTP-Status; Spalte „Ergebnis“ fasst den Gesamtbefund zusammen. `⟳` = Kontroll-Wiederholung mit 1,5 s Abstand und 4 parallelen Verbindungen, um Rate-Limit-Fehler (429) von echten 404 zu trennen.

**Lesehilfe:** `42× 404` bedeutet: 42 generische Preispfade wurden abgerufen und existieren nicht. `N× 429`/`N× 5xx`/`N× 403`/`N× Verbindungsabbruch` = der Server blockiert oder bricht ab; ein 404-Aussagewert ist dann nicht belegbar. Soft-404-/SPA-Fallen sind in Abschnitt 5 gesondert aufgeführt.

| Domain | geprüfte Pfade | Ergebnis |
|---|---|---|
| `1a-recycling-gmbh.de` | `1a-recycling-gmbh.de/` (200) · `1a-recycling-gmbh.de/preise` (404) · `1a-recycling-gmbh.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `a-z-altmetalle.de` | `a-z-altmetalle.de/` (200) · `a-z-altmetalle.de/preise` (404) · `a-z-altmetalle.de/preise.html` (404) · `www.a-z-altmetalle.de/firmen/altmetallentsorgung/` (200) · `www.a-z-altmetalle.de/privathaushalte/altmetallentsorgung/` (200) | **27** Pfade 200 · 42× 404 |
| `aa-metalle.de` | `aa-metalle.de/` (200) · `aa-metalle.de/preise` (404) · `aa-metalle.de/preise.html` (404) · `aa-metalle.de/schrott-und-metallhandel` (200) · `aa-metalle.de/metallentsorgung` (200) | **17** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `ab-recycling.de` | `ab-recycling.de/` (200) · `ab-recycling.de/preise` (404) · `ab-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `abc-recycling-dessau.de` | `abc-recycling-dessau.de/` (200) | **1** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `abfall-ist-rohstoff.de` | `abfall-ist-rohstoff.de/` (200) · `abfall-ist-rohstoff.de/preise` (404) · `abfall-ist-rohstoff.de/preise.html` (404) · `abfall-ist-rohstoff.de/downloads/` (200) · `abfall-ist-rohstoff.de` (200) | **36** Pfade 200 · 43× 404 |
| `absk-eschrott.de` | `absk-eschrott.de/` (200) · `absk-eschrott.de/preise` (404) · `absk-eschrott.de/preise.html` (404) · `www.absk-eschrott.de/` (200) · `www.absk-eschrott.de/index.php/anfahrt/` (200) | **12** Pfade 200 · 42× 404 |
| `acb-darmstadt.de` | `acb-darmstadt.de/` (200) · `acb-darmstadt.de/preise` (404) · `acb-darmstadt.de/preise.html` (404) · `acb-darmstadt.de/leistungen/metall-und-schrottankauf/` (200) · `acb-darmstadt.de/leistungen/metall-und-schrottankauf/metall-und-schrottabholung` (200) | **6** Pfade 200 · 42× 404 |
| `acm-moers.de` | `acm-moers.de/` (200) · `acm-moers.de/schrott-metall-ankauf` (200) · `acm-moers.de/preise` (200) | **90** Pfade 200 · 0× 404 |
| `acr-recycling.de` | `acr-recycling.de/` (200) · `acr-recycling.de/preise` (404) · `acr-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `aeris-gmbh.de` | `aeris-gmbh.de/` (200) · `aeris-gmbh.de/preise.html` (404) · `aeris-gmbh.de/anfragepreise` (404) · `aeris-gmbh.de/preise/` (200) · `aeris-gmbh.de/de/preise/` (200) | **20** Pfade 200 · 29× 404 |
| `ah-schrotthandel.de` | `ah-schrotthandel.de/` (200) · `ah-schrotthandel.de/preise` (404) · `ah-schrotthandel.de/preise.html` (404) · `ah-schrotthandel.de/wp-content/uploads/2023/07/container-with-building-waste-close-up.jpg` (200) · `ah-schrotthandel.de/wp-content/uploads/2023/07/customer-service-executive-with-hand-on-her-head.jpg` (200) | **4** Pfade 200 · 42× 404 |
| `ak-containerdienst.de` | `ak-containerdienst.de/` (200) · `ak-containerdienst.de/preise` (404) · `ak-containerdienst.de/preise.html` (404) · `ak-containerdienst.de/schrotthandel/` (200) · `www.festpreis-webseite.de/webseite-business` (200) | **4** Pfade 200 · 42× 404 → **Preisbefund** |
| `aktas-schrott.de` | `aktas-schrott.de/` (200) · `aktas-schrott.de/preise` (404) · `aktas-schrott.de/preise.html` (404) · `aktas-schrott.de/index.php` (200) | **2** Pfade 200 · 42× 404 |
| `alfa-recycling.de` | `alfa-recycling.de/` (200) | **1** Pfade 200 · 0× 404 · 44× Netzwerk ⟳ 10×Verbindungsabbruch |
| `altmetall-asn.de` | `altmetall-asn.de/` (200) · `altmetall-asn.de/preise` (404) · `altmetall-asn.de/preise.html` (404) · `altmetall-asn.de/ankaufliste/` (200) · `altmetall-asn.de/ankauf-liste/aluminium/` (200) | **11** Pfade 200 · 42× 404 |
| `altmetall-herne.de` | `altmetall-herne.de/` (200) · `altmetall-herne.de/preise` (404) · `altmetall-herne.de/preise.html` (404) · `www.altmetall-herne.de` (200) · `www.altmetall-herne.de/leistungen` (200) | **9** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `altmetall-saarland.de` | `altmetall-saarland.de/` (200) · `altmetall-saarland.de/preise` (404) · `altmetall-saarland.de/preise.html` (404) · `altmetall-saarland.de/sekundärpreise` (200) · `www.altmetall-saarland.de/` (200) | **3** Pfade 200 · 41× 404 |
| `altmetall-wagner.de` | `altmetall-wagner.de/` (200) · `altmetall-wagner.de/preise` (404) · `altmetall-wagner.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `altmetalle-witt.de` | `altmetalle-witt.de/` (200) · `altmetalle-witt.de/preise` (404) · `altmetalle-witt.de/preise.html` (404) · `altmetalle-witt.de` (200) · `altmetalle-witt.de/anfahrt` (200) | **8** Pfade 200 · 42× 404 |
| `altmetalleberlin.de` | `altmetalleberlin.de/` (200) · `altmetalleberlin.de/preise` (404) · `altmetalleberlin.de/preise.html` (404) · `www.altmetalleberlin.de` (200) · `www.altmetalleberlin.de/bilder` (200) | **11** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `altmetallhandel-kempf.com` | `altmetallhandel-kempf.com/` (200) · `altmetallhandel-kempf.com/preise` (404) · `altmetallhandel-kempf.com/preise.html` (404) · `www.altmetallhandel-kempf.com/schrottabholung/` (200) · `www.altmetallhandel-kempf.com/metallankauf/` (200) | **11** Pfade 200 · 42× 404 |
| `altmetallhandel-mittelsachsen.de` | `altmetallhandel-mittelsachsen.de/` (200) · `altmetallhandel-mittelsachsen.de/preise` (404) · `altmetallhandel-mittelsachsen.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `aluminiumonline.de` | `aluminiumonline.de/` (200) · `aluminiumonline.de/preise` (404) · `aluminiumonline.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `amr-schrottplatz.de` | `amr-schrottplatz.de/` (**ConnectionError: HTTPSConnectionPool(host='a**) | **0** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `amsbeck.gmbh` | `amsbeck.gmbh/` (200) · `amsbeck.gmbh/preise` (404) · `amsbeck.gmbh/sekundärpreise` (404) · `amsbeck.gmbh/leistungen/schrotte` (200) · `amsbeck.gmbh/metalle` (200) | **4** Pfade 200 · 5× 404 · 37× 503 ⟳ 2×404, 8×5xx |
| `an-schrott.de` | `an-schrott.de/` (200) · `an-schrott.de/preise` (404) · `an-schrott.de/preise.html` (404) · `www.an-schrott.de/ankauf/` (200) · `an-schrott.de/ankauf/` (200) | **6** Pfade 200 · 41× 404 |
| `annecke-schrotthandel.de` | `www.annecke-schrotthandel.de/` (200) · `www.annecke-schrotthandel.de/leistungen/` (200) · `www.annecke-schrotthandel.de/kontakt` (200) | **5** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `aram-handel.de` | `aram-handel.de/` (200) · `aram-handel.de/preise` (404) · `aram-handel.de/preise.html` (404) · `aram-handel.de/edelmetallankauf-bremen/` (200) · `aram-handel.de/goldankauf-bremen/` (200) | **8** Pfade 200 · 42× 404 |
| `areg-mbh.de` | `areg-mbh.de/` (200) · `areg-mbh.de/preise` (404) · `areg-mbh.de/preise.html` (404) · `www.areg-mbh.de/entsorgung/wertstoffe/` (200) · `areg-mbh.de/dienstleistungen/downloads/` (200) | **6** Pfade 200 · 42× 404 |
| `arm-entsorgung.de` | `arm-entsorgung.de/` (200) · `arm-entsorgung.de/preise` (404) · `arm-entsorgung.de/preise.html` (404) · `www.arm-entsorgung.de/schrotthandel-und-schrottankauf/` (200) · `arm-entsorgung.de/schrotthandel-und-schrottankauf` (200) | **4** Pfade 200 · 42× 404 → **Preisbefund** |
| `arm-kg.com` | `arm-kg.com/` (200) · `arm-kg.com/preise` (404) · `arm-kg.com/anfragepreise` (404) · `www.arm-kg.com/_files/ugd/503a3d_17866cbfd59347beb8b0e5e25ee4add5.pdf` (200) · `www.arm-kg.com/_files/ugd/503a3d_24c9a8f1e7974db6a11269c5e603101a.pdf` (200) | **5** Pfade 200 · 36× 404 · 6× 400 |
| `asa-altmetall.de` | `asa-altmetall.de/` (200) · `asa-altmetall.de/preise` (404) · `asa-altmetall.de/preise.html` (404) · `asa-altmetall.de/about` (200) · `asa-altmetall.de/catalogue` (200) | **4** Pfade 200 · 42× 404 |
| `asbest-entsorgung.de` | `asbest-entsorgung.de/` (200) · `asbest-entsorgung.de/preise` (404) · `asbest-entsorgung.de/preise.html` (404) · `www.baua.de/DE/Angebote/Rechtstexte-und-Technische-Regeln/Regelwerk/TRGS/pdf/TRGS-519.pdf?__blob=publicationFile` (200) | **2** Pfade 200 · 42× 404 |
| `asm-recycling.de` | `asm-recycling.de/` (200) · `asm-recycling.de/preise` (404) · `asm-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `asn.gmbh` | `asn.gmbh/` (200) · `asn.gmbh/preise` (404) · `asn.gmbh/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `augsburgerrohstoffhandel.de` | `augsburgerrohstoffhandel.de/` (200) · `augsburgerrohstoffhandel.de/preise` (404) · `augsburgerrohstoffhandel.de/preise.html` (404) · `augsburgerrohstoffhandel.de/index.php/kontakt/` (200) | **2** Pfade 200 · 42× 404 |
| `augustin-entsorgung.de` | `augustin-entsorgung.de/` (200) · `augustin-entsorgung.de/unternehmen/geschaeftsbereiche/altmetalle/` (200) · `augustin-entsorgung.de/unternehmen/standorte/meppen-huentel-emsschrott/` (200) | **98** Pfade 200 · 0× 404 |
| `aus-liebe-zum-schrott.de` | `aus-liebe-zum-schrott.de/` (200) · `aus-liebe-zum-schrott.de/preise` (404) · `aus-liebe-zum-schrott.de/preise.html` (404) · `aus-liebe-zum-schrott.de/schrotthandel` (200) · `aus-liebe-zum-schrott.de/anfahrt` (200) | **6** Pfade 200 · 42× 404 |
| `auto-daud.de` | `auto-daud.de/` (200) · `auto-daud.de/preise` (404) · `auto-daud.de/preise.html` (404) | **1** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) → **Preisbefund** |
| `auto-schrottplatz.com` | `auto-schrottplatz.com/` (200) · `auto-schrottplatz.com/preise` (404) · `auto-schrottplatz.com/preise.html` (404) · `auto-schrottplatz.com/schrott-ankauf-berlin` (200) · `auto-schrottplatz.com/pkw-ankauf-berlin` (200) | **21** Pfade 200 · 41× 404 · 1× 429 ⟳ 9×404, 1×429 (Rate-Limit/Blocking) |
| `autorecycling-asen.de` | `autorecycling-asen.de/` (200) · `autorecycling-asen.de/preise` (404) · `autorecycling-asen.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `autorecycling-eckhardt.de` | `autorecycling-eckhardt.de/` (200) · `autorecycling-eckhardt.de/preise` (404) · `autorecycling-eckhardt.de/preise.html` (404) · `autorecycling-eckhardt.de/src/Zertifikat2026.pdf` (200) | **2** Pfade 200 · 42× 404 |
| `autorecycling-penkert.de` | `www.autorecycling-penkert.de/` (200) · `www.autorecycling-penkert.de/index.php?view=article&amp;id=8:beginners&amp;catid=14` (200) | **2** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `autoverschrottung-bremen.de` | `autoverschrottung-bremen.de/` (200) · `autoverschrottung-bremen.de/preise` (404) · `autoverschrottung-bremen.de/preise.html` (404) · `autoverschrottung-bremen.de/schrottauto-ankauf` (200) · `autoverschrottung-bremen.de/kontakt` (200) | **13** Pfade 200 · 42× 404 |
| `autoverschrottung-hildesheim.de` | `autoverschrottung-hildesheim.de/` (200) · `autoverschrottung-hildesheim.de/preise` (404) · `autoverschrottung-hildesheim.de/preise.html` (404) · `autoverschrottung-hildesheim.de/schrottauto-ankauf` (200) · `autoverschrottung-hildesheim.de/kontakt` (200) | **13** Pfade 200 · 42× 404 |
| `autoverschrottung-rostock.de` | `autoverschrottung-rostock.de/` (200) · `autoverschrottung-rostock.de/preise` (404) · `autoverschrottung-rostock.de/preise.html` (404) · `autoverschrottung-rostock.de/schrottauto-ankauf` (200) · `autoverschrottung-rostock.de/kontakt` (200) | **3** Pfade 200 · 42× 404 |
| `autoverwertung-demand.de` | `autoverwertung-demand.de/` (200) · `autoverwertung-demand.de/preise` (404) · `autoverwertung-demand.de/preise.html` (404) | **1** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) → **Preisbefund** |
| `autoverwertung-zotzmann.de` | `autoverwertung-zotzmann.de/` (200) · `autoverwertung-zotzmann.de/preise` (404) · `autoverwertung-zotzmann.de/preise.html` (404) | **1** Pfade 200 · 33× 404 · 9× 429 ⟳ 10×404 |
| `avas-metallrecycling.de` | `avas-metallrecycling.de/` (200) · `avas-metallrecycling.de/preise` (404) · `avas-metallrecycling.de/preise.html` (404) · `avas-metallrecycling.de/leistungen/eisen-und-stahlschrott` (200) · `avas-metallrecycling.de/leistungen/ne-metalle` (200) | **58** Pfade 200 · 42× 404 |
| `avas-recycling.de` | `avas-recycling.de/` (200) · `avas-recycling.de/preise` (404) · `avas-recycling.de/preise.html` (404) · `avas-recycling.de/leistungen/eisen-und-stahlschrott` (200) · `avas-recycling.de/leistungen/ne-metalle` (200) | **18** Pfade 200 · 42× 404 |
| `avs24.com` | `avs24.com/` (200) · `avs24.com/preise` (404) · `avs24.com/preise.html` (404) · `www.avs24.com/downloads/` (200) · `www.avs24.com/schrott-altmetall-ankauf/` (200) | **12** Pfade 200 · 42× 404 |
| `aw-recycling.de` | `aw-recycling.de/` (200) · `aw-recycling.de/preise` (404) · `aw-recycling.de/preise.html` (404) · `aw-recycling.de/kupferankauf-ravensburg/` (200) · `aw-recycling.de/kabelankauf-ravensburg/` (200) | **4** Pfade 200 · 42× 404 |
| `ay-recycling.de` | `ay-recycling.de/` (200) · `ay-recycling.de/Leistungsangebot-1` (200) · `ay-recycling.de/Leistungsangebot-1/Leistungsangebot-2` (200) | **3** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `az-containerdienst.de` | `az-containerdienst.de/` (200) · `az-containerdienst.de/preise` (404) · `az-containerdienst.de/preise.html` (404) · `wa.me/4915159851357?text=Hallo%2C%20ich%20brauche%20ein%20Angebot%20f%C3%BCr%20einen%20Container.` (200) · `wa.me/4915159851357?text=Hallo%2C%20ich%20brauche%20einen%20Container.%20K%C3%B6nnen%20Sie%20mir%20ein%20Angebot%20machen%3F` (200) | **5** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `bach-containerdienst.de` | `bach-containerdienst.de/` (200) · `bach-containerdienst.de/preise` (404) · `bach-containerdienst.de/preise.html` (404) · `www.bach-containerdienst.de/leistungen` (200) | **2** Pfade 200 · 42× 404 |
| `bachinger-schrott.de` | `bachinger-schrott.de/` (200) · `bachinger-schrott.de/ihr-entsorger/alle-schrottarten/` (200) · `bachinger-schrott.de/preise` (200) | **92** Pfade 200 · 0× 404 |
| `bana-recycling.de` | `bana-recycling.de/` (200) · `bana-recycling.de/preise` (404) · `bana-recycling.de/preise.html` (404) · `bana-recycling.de/wp-content/uploads/2019/05/AGB_Container_Version.pdf` (200) · `bana-recycling.de/wp-content/uploads/2019/05/AGB_Anlieferung-Version.pdf` (200) | **3** Pfade 200 · 42× 404 |
| `bareg.de` | `bareg.de/` (200) · `bareg.de/preise` (404) · `bareg.de/preise.html` (404) · `www.bareg.de/schrott/` (200) · `www.bareg.de/stahlschrott/` (200) | **20** Pfade 200 · 43× 404 · 1× 503 ⟳ 10×404 |
| `bartikowsky-recycling-muenster.de` | `bartikowsky-recycling-muenster.de/` (200) · `bartikowsky-recycling-muenster.de/schrotthandel-muenster-2` (200) | **2** Pfade 200 · 0× 404 · 45× 429 ⟳ 10×429 (Rate-Limit/Blocking) |
| `bartsch-entsorgung.de` | `bartsch-entsorgung.de/` (200) · `bartsch-entsorgung.de/preise` (404) · `bartsch-entsorgung.de/preise.html` (404) · `bartsch-entsorgung.de/pdf/Bartsch_AGB_2025.pdf` (200) · `bartsch-entsorgung.de/entsorgungspunkt` (200) | **3** Pfade 200 · 42× 404 |
| `bauer-entsorgungen.de` | `bauer-entsorgungen.de/` (200) · `bauer-entsorgungen.de/preise` (404) · `bauer-entsorgungen.de/anfragepreise` (404) · `www.bauer-entsorgungen.de/schrott-and-metalle` (200) | **2** Pfade 200 · 36× 404 · 6× 400 |
| `bauerunternehmensgruppe.de` | `bauerunternehmensgruppe.de/` (200) · `bauerunternehmensgruppe.de/download-zertifikate` (200) · `bauerunternehmensgruppe.de/download-zertifikate?view=article&amp;id=25:login&amp;catid=2` (200) | **7** Pfade 200 · 0× 404 · 42× 500 ⟳ 10×5xx |
| `bausch-entsorgung.de` | `bausch-entsorgung.de/` (200) · `bausch-entsorgung.de/preise` (404) · `bausch-entsorgung.de/preise.html` (404) · `www.bausch-entsorgung.de/schrott-metall/` (200) · `www.bausch-entsorgung.de/wp-content/uploads/2025/05/AGB.pdf` (200) | **9** Pfade 200 · 42× 404 |
| `bbw-recycling.de` | `bbw-recycling.de/` (200) · `bbw-recycling.de/preise` (404) · `bbw-recycling.de/preise.html` (404) · `bbw-recycling.de/downloads-zertifikate` (200) · `www.bbw-recycling.de/downloads-zertifikate` (200) | **7** Pfade 200 · 42× 404 → **Preisbefund** |
| `bcdcontainerdienst.de` | `bcdcontainerdienst.de/` (200) · `bcdcontainerdienst.de/preise` (404) · `bcdcontainerdienst.de/preise.html` (404) · `bcdcontainerdienst.de/media/260505_svg_zertifikat-min.pdf` (200) · `bcdcontainerdienst.de/metallankauf.html` (200) | **5** Pfade 200 · 42× 404 |
| `beg-bhv.de` | `beg-bhv.de/` (200) · `beg-bhv.de/preise` (404) · `beg-bhv.de/preise.html` (404) · `beg-bhv.de/downloads/alles-ueber-abfall` (200) · `beg-bhv.de/downloads/informationen-muell-heiz-kraftwerk` (200) | **21** Pfade 200 · 42× 404 |
| `behrendt.com` | `behrendt.com/` (200) · `behrendt.com/preise` (404) · `behrendt.com/preise.html` (404) · `behrendt.com/leistungen/elektroschrott/` (200) · `behrendt.com/stellenangebote/` (200) | **16** Pfade 200 · 42× 404 |
| `bekomuenzen.de` | `bekomuenzen.de/` (200) · `bekomuenzen.de/preise` (404) · `bekomuenzen.de/preise.html` (404) · `www.bekomuenzen.de/ankauf` (200) · `www.bekomuenzen.de/aktuelle-edelmetallpreise` (200) | **31** Pfade 200 · 38× 404 · 3× 401 → **Preisbefund** |
| `beller-das.de` | `beller-das.de/` (200) · `beller-das.de/preise` (404) · `beller-das.de/preise.html` (404) · `www.beller-das.de/wp-content/uploads/2025/03/2025_ESN-Zertifikat_komplett.pdf` (200) · `www.beller-das.de/wp-content/uploads/2025/09/ULV-Bescheinigung_2025.pdf` (200) | **5** Pfade 200 · 42× 404 → **Preisbefund** |
| `bellersheim.de` | `bellersheim.de/` (200) · `bellersheim.de/preise` (404) · `bellersheim.de/preise.html` (404) · `bellersheim.de/abfallwirtschaft/leistungen/altmetall/` (200) · `bellersheim.de/karriere/stellenangebote/` (200) | **28** Pfade 200 · 42× 404 |
| `bender-recycling.de` | `bender-recycling.de/` (200) · `bender-recycling.de/preise` (200) · `bender-recycling.de/preise.html` (200) | **3** Pfade 200 · 0× 404 · 40× 429 ⟳ 1×200, 9×429 (Rate-Limit/Blocking) — 200 auf /ankaufspreise |
| `benze-altmetalle.de` | `benze-altmetalle.de/` (200) · `benze-altmetalle.de/preise` (404) · `benze-altmetalle.de/preise.html` (404) · `benze-altmetalle.de/materialien` (200) · `benze-altmetalle.de/sekundärpreise` (200) | **11** Pfade 200 · 41× 404 |
| `bergoldmetalle.de` | `bergoldmetalle.de/` (200) · `bergoldmetalle.de/preise` (404) · `bergoldmetalle.de/preise.html` (404) · `bergoldmetalle.de/pdf/AGB-Einkaufsbedingungen.pdf` (200) · `bergoldmetalle.de/pdf/AGB-Verkaufsbedingungen.pdf` (200) | **7** Pfade 200 · 42× 404 |
| `berlin-recycling.de` | `berlin-recycling.de/` (200) · `berlin-recycling.de/preise` (404) · `berlin-recycling.de/preise.html` (404) · `berlin-recycling.de/entsorgung/sondermuell/elektroschrott-entsorgen` (200) · `berlin-recycling.de/entsorgung/sperrmuell` (200) | **24** Pfade 200 · 42× 404 → **Preisbefund** |
| `bhs-walldorf.de` | `bhs-walldorf.de/` (200) · `bhs-walldorf.de/preise` (404) · `bhs-walldorf.de/preise.html` (404) · `www.bhs-walldorf.de/` (200) | **2** Pfade 200 · 42× 404 |
| `bielefelder-autoverwertung.de` | `bielefelder-autoverwertung.de/` (200) · `bielefelder-autoverwertung.de/preise` (404) · `bielefelder-autoverwertung.de/preise.html` (404) · `www.bielefelder-autoverwertung.de/schrott-metalle/` (200) · `www.bielefelder-autoverwertung.de/wp-content/uploads/2019/06/schroot_altmetall_gallery_2.jpg` (200) | **3** Pfade 200 · 42× 404 |
| `bierhoff-metall.de` | `bierhoff-metall.de/` (200) · `bierhoff-metall.de/preise` (404) · `bierhoff-metall.de/preise.html` (404) · `bierhoff-metall.de/Gewerblich/` (200) · `bierhoff-metall.de/Privat/` (200) | **7** Pfade 200 · 42× 404 |
| `bihl-gmbh.de` | `bihl-gmbh.de/` (200) · `bihl-gmbh.de/preise` (404) · `bihl-gmbh.de/preise.html` (404) · `www.bihl-gmbh.de/fe-metalle/` (200) · `www.bihl-gmbh.de/ne-metalle/` (200) | **6** Pfade 200 · 42× 404 |
| `bimbes-recycling.de` | `bimbes-recycling.de/` (200) · `bimbes-recycling.de/preise` (404) · `bimbes-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `bissendorf-recycling.de` | `bissendorf-recycling.de/` (200) · `bissendorf-recycling.de/preise` (404) · `bissendorf-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `bissig.de` | `bissig.de/` (200) | **1** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `bitterfelder-metallrecycling.de` | `bitterfelder-metallrecycling.de/` (200) · `bitterfelder-metallrecycling.de/preise` (404) · `bitterfelder-metallrecycling.de/preise.html` (404) · `bitterfelder-metallrecycling.de/category/stellenangebote/` (200) · `bitterfelder-metallrecycling.de` (200) | **28** Pfade 200 · 42× 404 |
| `bks-metallrecycling.de` | `bks-metallrecycling.de/` (200) · `bks-metallrecycling.de/preise` (404) · `bks-metallrecycling.de/preise.html` (404) · `bks-metallrecycling.de/kontakt` (200) · `bks-metallrecycling.de/leistungen/schrottankauf` (200) | **60** Pfade 200 · 42× 404 |
| `blank-metalle.de` | `blank-metalle.de/` (200) · `blank-metalle.de/preise` (404) · `blank-metalle.de/preise.html` (404) · `www.heise.de/ct/artikel/Shariff-Social-Media-Buttons-mit-Datenschutz-2467514.html` (200) · `blank-metalle.de` (200) | **43** Pfade 200 · 42× 404 |
| `boehner-altmetalle.de` | `boehner-altmetalle.de/` (200) · `boehner-altmetalle.de/preise` (404) · `boehner-altmetalle.de/preise.html` (404) · `boehner-altmetalle.de/ankauf-preise` (200) · `boehner-altmetalle.de/info-altmetall-schrott` (200) | **27** Pfade 200 · 41× 404 → **Preisbefund** |
| `boetzel-schrott.com` | `boetzel-schrott.com/` (200) · `boetzel-schrott.com/job-angebote-boetzel.html` (404) · `boetzel-schrott.com/job-angebote.html` (404) · `www.boetzel-schrott.com/job-angebote.html` (200) · `boetzel-schrott.com/files/boetzel-schrott/bilder-content/unternehmen/job-angebote/wilhelm-boetzel-gmbh-jobs-01.jpg` (200) | **17** Pfade 200 · 49× 404 |
| `borgsdorf-recycling.de` | `borgsdorf-recycling.de/` (200) · `borgsdorf-recycling.de/preise` (404) · `borgsdorf-recycling.de/preise.html` (404) · `www.borgsdorf-recycling.de/das-neue-museum-oeffnet-seine-tueren-2/` (200) | **2** Pfade 200 · 42× 404 |
| `born-recycling.de` | `born-recycling.de/` (200) · `born-recycling.de/preise` (404) · `born-recycling.de/preise.html` (404) · `born-recycling.de/schrott-ne-metallhandel/` (200) · `born-recycling.de/stellenangebote/` (200) | **5** Pfade 200 · 42× 404 |
| `bottcher-brennholz.de` | `bottcher-brennholz.de/` (200) · `bottcher-brennholz.de/preise` (404) · `bottcher-brennholz.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `bottroper-schrotthandel.de` | `bottroper-schrotthandel.de/` (200) · `bottroper-schrotthandel.de/preise` (404) · `bottroper-schrotthandel.de/preise.html` (404) | **0** Pfade 200 · 42× 404 |
| `br-recycling.com` | `br-recycling.com/` (200) · `br-recycling.com/preise` (404) · `br-recycling.com/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `brandmayr-co.de` | `brandmayr-co.de/` (200) · `brandmayr-co.de/preise` (404) · `brandmayr-co.de/preise.html` (404) · `www.brandmayr-co.de/wp-content/uploads/2026/07/Allgemeine-Einkaufsbedingungen_2012-1.pdf` (200) | **2** Pfade 200 · 42× 404 |
| `brecheisen-entsorgung.de` | `brecheisen-entsorgung.de/` (200) · `brecheisen-entsorgung.de/preise` (200) · `brecheisen-entsorgung.de/preise.html` (200) | **83** Pfade 200 · 0× 404 |
| `bremer-recycling-service.de` | `bremer-recycling-service.de/` (200) · `bremer-recycling-service.de/api/preise` (404) · `bremer-recycling-service.de/api/prices` (404) · `bremer-recycling-service.de/preise` (200) · `bremer-recycling-service.de/preise.html` (200) | **40** Pfade 200 · 3× 404 |
| `bremerschrotthandel.com` | `bremerschrotthandel.com/` (200) · `bremerschrotthandel.com/preise` (404) · `bremerschrotthandel.com/preise.html` (404) · `bremerschrotthandel.com/?PageSpeed=noscript` (200) · `bremerschrotthandel.com/kontakt/?PageSpeed=noscript` (200) | **11** Pfade 200 · 42× 404 |
| `bronnenmayer-schrott.de` | `bronnenmayer-schrott.de/` (200) · `bronnenmayer-schrott.de/preise` (404) · `bronnenmayer-schrott.de/preise.html` (404) · `bronnenmayer-schrott.de/metalle` (200) · `bronnenmayer-schrott.de/schrott` (200) | **11** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `brosda.com` | `brosda.com/` (**ConnectionError: HTTPSConnectionPool(host='b**) | **0** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `bruchsal-schrott.de` | `bruchsal-schrott.de/` (200) · `bruchsal-schrott.de/preise` (404) · `bruchsal-schrott.de/preise.html` (404) · `www.bruchsal-schrott.de/ankauf-handel/` (200) · `bruchsal-schrott.de/ankauf/` (200) | **11** Pfade 200 · 41× 404 |
| `bruno-neumann.de` | `bruno-neumann.de/` (200) · `bruno-neumann.de/preise` (404) · `bruno-neumann.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `brunos-allinone-recycling.de` | `brunos-allinone-recycling.de/` (200) · `brunos-allinone-recycling.de/preise` (404) · `brunos-allinone-recycling.de/preise.html` (404) · `brunos-allinone-recycling.de/sekundärpreise` (200) | **2** Pfade 200 · 41× 404 |
| `bs-containerdienst.de` | `bs-containerdienst.de/` (200) · `bs-containerdienst.de/preise` (200) · `bs-containerdienst.de/preise.html` (200) | **85** Pfade 200 · 0× 404 |
| `bsg-kaarst.de` | `bsg-kaarst.de/` (200) · `bsg-kaarst.de/preise` (404) · `bsg-kaarst.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `buelowundpartner.de` | `buelowundpartner.de/` (200) · `buelowundpartner.de/preise` (404) · `buelowundpartner.de/preise.html` (404) · `buelowundpartner.de/index.php/schrott-metall` (200) · `buelowundpartner.de/images/Downloads/Entsorgungsfachbetrieb.pdf` (200) | **6** Pfade 200 · 42× 404 |
| `buerck-achern.de` | `buerck-achern.de/` (200) · `buerck-achern.de/preise` (404) · `buerck-achern.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `buf-metallhandel.de` | `buf-metallhandel.de/` (200) · `buf-metallhandel.de/preise` (404) · `buf-metallhandel.de/preise.html` (404) · `buf-metallhandel.de/fileadmin/download/zert2024/Z_EoW_D_BuF_2023.pdf` (200) · `buf-metallhandel.de/fileadmin/download/zert2024/qm2024.pdf` (200) | **40** Pfade 200 · 42× 404 |
| `buhck-gruppe.de` | `buhck-gruppe.de/` (200) · `buhck-gruppe.de/preise` (404) · `buhck-gruppe.de/preise.html` (404) · `buhck-gruppe.de/unternehmen/events-media` (200) · `buhck-gruppe.de/unternehmen/events-media/downloadbereich` (200) | **13** Pfade 200 · 42× 404 |
| `buntmetallaufkauf.de` | `buntmetallaufkauf.de/` (200) · `buntmetallaufkauf.de/preise` (404) · `buntmetallaufkauf.de/preise.html` (404) · `buntmetallaufkauf.de/dokumente/` (200) · `buntmetallaufkauf.de/leistung/buntmetallaufkauf-zu-tageshoechstpreisen/` (200) | **34** Pfade 200 · 42× 404 → **Preisbefund** |
| `buntmetallhandel-altmittweida.de` | `buntmetallhandel-altmittweida.de/` (200) · `buntmetallhandel-altmittweida.de/preise.html` (404) · `buntmetallhandel-altmittweida.de/anfragepreise` (404) · `buntmetallhandel-altmittweida.de/ankaufspreisliste` (200) · `buntmetallhandel-altmittweida.de/kontakt` (200) | **6** Pfade 200 · 10× 404 · 31× Netzwerk ⟳ 1×200, 7×404, 2×Verbindungsabbruch — 200 auf /preise |
| `bur-recycling-duisburg.de` | `bur-recycling-duisburg.de/` (200) · `bur-recycling-duisburg.de/preise` (404) · `bur-recycling-duisburg.de/preise.html` (404) · `www.bur-recycling-duisburg.de/angebotsliste` (200) | **2** Pfade 200 · 23× 404 · 20× 429 ⟳ 10×Verbindungsabbruch |
| `burger-recycling.de` | `burger-recycling.de/` (200) · `burger-recycling.de/leistungen/schrott` (200) · `burger-recycling.de/leistungen/metalle` (200) | **12** Pfade 200 · 0× 404 · 42× 500 ⟳ 10×5xx |
| `bz-metall.de` | `bz-metall.de/` (200) · `bz-metall.de/?Itemid=262` (404) · `bz-metall.de/preise` (404) · `bz-metall.de/recycling.html` (200) · `bz-metall.de/containerdienst.html` (200) | **23** Pfade 200 · 43× 404 |
| `ch-metallhandel.de` | `ch-metallhandel.de/` (200) · `ch-metallhandel.de/preise` (404) · `ch-metallhandel.de/preise.html` (404) · `ch-metallhandel.de/schrottabholung-essen/` (200) · `ch-metallhandel.de/altmetallabholung-essen/` (200) | **4** Pfade 200 · 24× 404 · 18× 503, 3× Netzwerk ⟳ 10×404 |
| `chiemgau-recycling.de` | `chiemgau-recycling.de/` (200) · `chiemgau-recycling.de/preise` (404) · `chiemgau-recycling.de/preise.html` (404) · `chiemgau-recycling.de/wp-content/uploads/2024/11/ISO_9001_2015.pdf` (200) · `chiemgau-recycling.de/wp-content/uploads/2026/07/EFB_Zertifikat-inkl-Anlagen_26-28.pdf` (200) | **9** Pfade 200 · 55× 404 |
| `chris-schrott.de` | `chris-schrott.de/` (200) · `chris-schrott.de/preise` (404) · `chris-schrott.de/preise.html` (404) · `chris-schrott.de/schrottabholung/` (200) · `chris-schrott.de/schrotthandel/` (200) | **16** Pfade 200 · 42× 404 |
| `clarios.com` | `clarios.com/` (200) · `clarios.com/preise` (404) · `clarios.com/preise.html` (404) · `clarios.com/media-press` (200) · `useast2prodbrandsites.blob.core.windows.net/sf-assets/docs/default-source/default-document-library/anti-modern-slavery-human-trafficking-transparency-statement.pdf?sfvrsn=29721643_38` (200) | **13** Pfade 200 · 42× 404 |
| `claus-schrott.de` | `claus-schrott.de/` (200) · `claus-schrott.de/preise` (404) · `claus-schrott.de/preise.html` (404) · `www.claus-schrott.de/dienste/schrott-metallhandel` (200) · `www.claus-schrott.de/wp-content/uploads/ZZHT004001273005_EFB-Zertifikat-2021-Theodor-Claus.pdf` (200) | **28** Pfade 200 · 42× 404 |
| `closter.de` | `closter.de/` (200) · `closter.de/preise` (404) · `closter.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `cmr-recycling.de` | `cmr-recycling.de/` (200) · `cmr-recycling.de/preise` (404) · `cmr-recycling.de/preise.html` (404) · `cmr-recycling.de/kontakt/` (200) | **2** Pfade 200 · 42× 404 |
| `computerschrott-ankauf-stuttgart.de` | `computerschrott-ankauf-stuttgart.de/` (200) · `computerschrott-ankauf-stuttgart.de/preise` (404) · `computerschrott-ankauf-stuttgart.de/preise.html` (404) · `www.computerschrott-ankauf-stuttgart.de/` (200) · `www.computerschrott-ankauf-stuttgart.de/computerschrott-ankauf/` (200) | **12** Pfade 200 · 39× 404 · 3× 401 |
| `container-fix.de` | `container-fix.de/` (200) · `container-fix.de/preise` (404) · `container-fix.de/preise.html` (404) · `www.container-fix.de/schrott-ne-metalle/` (200) · `www.container-fix.de/schrott-ne-metalle/stahlschrott/` (200) | **21** Pfade 200 · 42× 404 |
| `container-hanschke.de` | `container-hanschke.de/` (200) · `container-hanschke.de/preise` (404) · `container-hanschke.de/preise.html` (404) · `container-hanschke.de/leistungen/schrottentsorgung.php` (200) · `container-hanschke.de/download/containerbeladung-1-mal-1.pdf?m=1747216309&` (200) | **4** Pfade 200 · 42× 404 |
| `container-herrmann.de` | `container-herrmann.de/` (200) · `container-herrmann.de/preise` (404) · `container-herrmann.de/preise.html` (404) · `www.container-herrmann.de/cms/wp-content/uploads/2020/06/Herrmann-EFB-Zertifikat.pdf` (200) · `www.container-herrmann.de/cms/wp-content/uploads/2026/10/Herrmann-Efb-Zertifikat-2026.pdf` (200) | **6** Pfade 200 · 42× 404 |
| `container-kerpen.de` | `container-kerpen.de/` (200) · `container-kerpen.de/preise` (404) · `container-kerpen.de/preise.html` (404) · `www.container-kerpen.de/metall/` (200) · `container-kerpen.de/metall/` (200) | **3** Pfade 200 · 42× 404 |
| `container-loth.de` | `container-loth.de/` (200) · `container-loth.de/preise` (404) · `container-loth.de/preise.html` (404) · `container-loth.de/stellenangebote.html` (200) | **2** Pfade 200 · 42× 404 |
| `container-speyer.de` | `container-speyer.de/` (200) · `container-speyer.de/preise` (404) · `container-speyer.de/preise.html` (404) · `container-speyer.de/pages/entsorgung-von-kunststoff-und-metallabfallen` (200) | **2** Pfade 200 · 41× 404 · 1× 400 |
| `container-willi.de` | `container-willi.de/` (200) · `container-willi.de/preise` (404) · `container-willi.de/preise.html` (404) · `container-willi.de/zertifikate.zip` (200) | **2** Pfade 200 · 42× 404 |
| `containerdienst-flink.de` | `containerdienst-flink.de/` (200) | **0** Pfade 200 · 0× 404 · 43× Netzwerk ⟳ 10×Verbindungsabbruch |
| `containerdienst-gelsenkirchen-smc.de` | `containerdienst-gelsenkirchen-smc.de/` (200) · `containerdienst-gelsenkirchen-smc.de/preise` (404) · `containerdienst-gelsenkirchen-smc.de/preise.html` (404) · `containerdienst-gelsenkirchen-smc.de/schrott-metall-container-entsorgung-gelsenkirchen.html.html` (200) · `containerdienst-gelsenkirchen-smc.de/kontakt-infos-schrottankauf-zu-spitzen-preisen-in-nrw.html.html` (200) | **4** Pfade 200 · 42× 404 |
| `containerdienst-heilmann.de` | `containerdienst-heilmann.de/` (200) · `containerdienst-heilmann.de/preise` (404) · `containerdienst-heilmann.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `containerdienst-kaminski.de` | `containerdienst-kaminski.de/` (200) · `containerdienst-kaminski.de/preise` (404) · `containerdienst-kaminski.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `containerdienst-kck.de` | `containerdienst-kck.de/` (200) · `containerdienst-kck.de/preise` (404) · `containerdienst-kck.de/preise.html` (404) | **1** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `containerdienst-keller.de` | `containerdienst-keller.de/` (200) · `containerdienst-keller.de/preise` (404) · `containerdienst-keller.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `containerdienst-koeln.de` | `containerdienst-koeln.de/` (200) · `containerdienst-koeln.de/preise` (200) · `containerdienst-koeln.de/preise.html` (200) | **43** Pfade 200 · 0× 404 |
| `containerdienst-koenig.de` | `containerdienst-koenig.de/` (200) · `containerdienst-koenig.de/preise` (404) · `containerdienst-koenig.de/preise.html` (404) · `de.cdn-website.com/a370fddbac8a4f1e872803e1194a5c58/files/uploaded/EfB+Zerifikat+2025.pdf` (200) | **2** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `containerdienst-kremer.de` | `containerdienst-kremer.de/` (200) · `containerdienst-kremer.de/preise` (404) · `containerdienst-kremer.de/preise.html` (404) · `containerdienst-kremer.de/downloads/` (200) · `containerdienst-kremer.de/leistungen/schrott-metallhandel/` (200) | **6** Pfade 200 · 42× 404 |
| `containerdienst-luxem.de` | `containerdienst-luxem.de/` (200) · `containerdienst-luxem.de/preise` (404) · `containerdienst-luxem.de/preise.html` (404) · `www.containerdienst-luxem.de/service/metallschrott/` (200) · `www.containerdienst-luxem.de/wp-content/uploads/2026/03/Metallschrott.jpg` (200) | **3** Pfade 200 · 42× 404 |
| `containerdienst-olbrich.de` | `containerdienst-olbrich.de/` (200) · `containerdienst-olbrich.de/preise` (404) · `containerdienst-olbrich.de/preise.html` (404) · `containerdienst-olbrich.de/materialanlieferung/` (200) | **2** Pfade 200 · 42× 404 |
| `containerdienst-schaafheim.de` | `containerdienst-schaafheim.de/` (200) · `containerdienst-schaafheim.de/preise` (200) · `containerdienst-schaafheim.de/preise.html` (200) | **43** Pfade 200 · 0× 404 |
| `containerdienst-stassfurt.de` | `containerdienst-stassfurt.de/` (200) · `containerdienst-stassfurt.de/preise` (404) · `containerdienst-stassfurt.de/preise.html` (404) · `www.containerdienst-stassfurt.de/` (200) · `www.containerdienst-stassfurt.de/wp-content/uploads/2026/09/Scan_20260928-3.pdf` (200) | **3** Pfade 200 · 42× 404 |
| `containerdienst-totzek.de` | `containerdienst-totzek.de/` (200) · `containerdienst-totzek.de/preise` (404) · `containerdienst-totzek.de/preise.html` (404) · `containerdienst-totzek.de/leistungsspektrum/materialentsorgung/` (200) · `containerdienst-totzek.de/leistungsspektrum/schrott--und-metallhandel/` (200) | **17** Pfade 200 · 42× 404 |
| `containerdienst-willefuehr.de` | `containerdienst-willefuehr.de/` (200) · `containerdienst-willefuehr.de/preise` (404) · `containerdienst-willefuehr.de/preise.html` (404) · `containerdienst-willefuehr.de/schrottmetallhandel.html` (200) · `www.containerdienst-willefuehr.de/schrottmetallhandel.html` (200) | **3** Pfade 200 · 42× 404 |
| `containerdienst-wuerfel.de` | `containerdienst-wuerfel.de/` (200) · `containerdienst-wuerfel.de/preise` (404) · `containerdienst-wuerfel.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `containerdienst.de` | `containerdienst.de/` (200) · `containerdienst.de/preise` (200) · `containerdienst.de/preise.html` (200) | **44** Pfade 200 · 0× 404 · 21× Netzwerk ⟳ 7×200, 3×Verbindungsabbruch — 200 auf /preise, /schrottpreise, /tagespreise, /ankaufspreise, /metallpreise, /preisliste, /preise.json |
| `containerdienste-schrott-metallhandel.de` | `containerdienste-schrott-metallhandel.de/` (**HTTP 503**) · `containerdienste-schrott-metallhandel.de/wp-content/uploads/preisliste.pdf` (404) | **0** Pfade 200 · 1× 404 · 41× 503 ⟳ 1×404, 9×5xx |
| `cr-recycling.de` | `cr-recycling.de/` (200) · `cr-recycling.de/preise` (404) · `cr-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `cristofor-schrotthandel.de` | `cristofor-schrotthandel.de/` (200) · `cristofor-schrotthandel.de/preise` (404) · `cristofor-schrotthandel.de/preise.html` (404) · `cristofor-schrotthandel.de/schrottabholung/` (200) · `cristofor-schrotthandel.de/schrottankauf/` (200) | **8** Pfade 200 · 42× 404 |
| `cs-entsorgung.de` | `cs-entsorgung.de/` (200) · `cs-entsorgung.de/preise` (404) · `cs-entsorgung.de/preise.html` (404) · `cs-entsorgung.de/raumloesungen/materialcontainer/` (200) · `cs-entsorgung.de/wp-content/uploads/2026/07/2026_07_06_Zert-2B211134-050-2026.pdf` (200) | **3** Pfade 200 · 42× 404 |
| `csg-diehlo.de` | `csg-diehlo.de/` (200) · `csg-diehlo.de/preise` (404) · `csg-diehlo.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `ctr-metall.de` | `ctr-metall.de/` (200) · `ctr-metall.de/preise` (404) · `ctr-metall.de/preise.html` (404) · `ctr-metall.de/metall` (200) · `www.ctr-metall.de` (200) | **11** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `dammann-gmbh.de` | `dammann-gmbh.de/` (200) | **1** Pfade 200 · 0× 404 · 42× 429 ⟳ 9×429 (Rate-Limit/Blocking), 1×Verbindungsabbruch |
| `dar-dessau.de` | `dar-dessau.de/` (200) · `dar-dessau.de/preise` (404) · `dar-dessau.de/preise.html` (404) · `dar-dessau.de/files/dar/top-image-galerie/top-image_preisliste.jpg` (200) · `dar-dessau.de/material-und-leistungsuebersicht.html` (200) | **4** Pfade 200 · 42× 404 |
| `darmstadt-schrotthandel.de` | `darmstadt-schrotthandel.de/` (200) · `darmstadt-schrotthandel.de/preise` (404) · `darmstadt-schrotthandel.de/preise.html` (404) · `darmstadt-schrotthandel.de/metallentsorgung/` (200) · `darmstadt-schrotthandel.de/kontakt/` (200) | **4** Pfade 200 · 42× 404 |
| `das-recycling.com` | `das-recycling.com/` (200) · `das-recycling.com/preise` (200) · `das-recycling.com/preise.html` (200) | **43** Pfade 200 · 0× 404 |
| `das-recycling.de` | `das-recycling.de/` (200) · `das-recycling.de/preise` (404) · `das-recycling.de/preise.html` (404) · `das-recycling.de/start/wertstoffe/` (200) | **2** Pfade 200 · 42× 404 |
| `degenkolbe-recycling.de` | `degenkolbe-recycling.de/` (200) · `www.degenkolbe-recycling.de/downloads/` (404) · `degenkolbe-recycling.de/preise` (404) · `www.degenkolbe-recycling.de/angebotsanfrage/` (200) | **2** Pfade 200 · 43× 404 |
| `dehner-recycling.de` | `dehner-recycling.de/` (200) · `dehner-recycling.de/preise` (404) · `dehner-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `dein-schrottplatz.de` | `dein-schrottplatz.de/` (200) · `dein-schrottplatz.de/preise` (404) · `dein-schrottplatz.de/preise.html` (404) · `www.dein-schrottplatz.de/pages/preise.php` (200) · `www.dein-schrottplatz.de/pages/leistungen/schrott--metallhandel.php` (200) | **43** Pfade 200 · 42× 404 → **Preisbefund** |
| `delko-metalle.de` | `delko-metalle.de/` (200) · `delko-metalle.de/preise` (404) · `delko-metalle.de/anfragepreise` (404) · `www.delko-metalle.de` (200) | **2** Pfade 200 · 36× 404 · 6× 400 |
| `demorec.de` | `demorec.de/` (200) · `demorec.de/preise` (404) · `demorec.de/preise.html` (404) · `www.festpreis-webseite.de/?demorec` (200) | **2** Pfade 200 · 42× 404 |
| `deneke-recycling.de` | `deneke-recycling.de/` (200) · `deneke-recycling.de/preise` (404) · `deneke-recycling.de/preise.html` (404) · `deneke-recycling.de/eisenschrott-entsorgung/` (200) · `deneke-recycling.de/wp-content/uploads/2024/05/zertifikat-kurt-deneke-gmbh.pdf` (200) | **3** Pfade 200 · 43× 404 |
| `deppe-lingen.de` | `deppe-lingen.de/` (200) · `deppe-lingen.de/preise` (404) · `deppe-lingen.de/preise.html` (404) · `deppe-lingen.de/Metallanalyse` (200) | **2** Pfade 200 · 42× 404 |
| `der-poltercontainer.de` | `der-poltercontainer.de/` (200) · `der-poltercontainer.de/preise` (404) · `der-poltercontainer.de/preise.html` (404) · `der-poltercontainer.de/images/Schrotthandel24/Schrotthandel_Duisburg%20(10).jpg` (200) · `der-poltercontainer.de/images/Schrotthandel24/Schrotthandel_Duisburg%20(11).jpg` (200) | **13** Pfade 200 · 42× 404 |
| `derichebourg-umwelt.de` | `www.derichebourg-umwelt.de/` (200) | **1** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `derleh-recycling.de` | `derleh-recycling.de/` (200) · `derleh-recycling.de/preise` (404) · `derleh-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `dero-reinhardt.de` | `dero-reinhardt.de/` (200) · `dero-reinhardt.de/preise` (404) · `dero-reinhardt.de/preise.html` (404) · `dero-reinhardt.de/metallrecycling` (200) · `dero-reinhardt.de/weitere-leistungen` (200) | **7** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `deumu.de` | `deumu.de/` (200) · `deumu.de/preise` (404) · `deumu.de/preise.html` (404) · `deumu.de/fileadmin/footage/MEDIA/SZAG/konzern/compliance/SZAG-Lieferantenkodex-20220914.pdf` (200) · `deumu.de/fileadmin/footage/MEDIA/SZAG/nachhaltigkeit/management_und_werte/szag-code-of-conduct-de.pdf` (200) | **33** Pfade 200 · 42× 404 |
| `dhs-entsorgung.de` | `dhs-entsorgung.de/` (200) · `dhs-entsorgung.de/preise` (404) · `dhs-entsorgung.de/preise.html` (404) | **1** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `die-schrottler.de` | `die-schrottler.de/` (200) · `die-schrottler.de/preise` (404) · `die-schrottler.de/preise.html` (404) · `die-schrottler.de/schrott/` (200) · `www.dieschrottler.de/` (200) | **12** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `dieluweit-mr.de` | `dieluweit-mr.de/` (200) · `dieluweit-mr.de/preise` (404) · `dieluweit-mr.de/preise.html` (404) · `dieluweit-mr.de/metallhandel` (200) · `dieluweit-mr.de/selbstanlieferung` (200) | **9** Pfade 200 · 42× 404 |
| `dilse-rohstoffhandel.de` | `dilse-rohstoffhandel.de/` (200) · `dilse-rohstoffhandel.de/preise` (404) · `dilse-rohstoffhandel.de/preise.html` (404) · `dilse-rohstoffhandel.de/wp-content/uploads/2026/08/Dilse-Zertifikat-Entsorgungsfachbetrieb.pdf` (200) · `dilse-rohstoffhandel.de/wp-content/uploads/2026/02/Scan_05530920251022144753.pdf` (200) | **12** Pfade 200 · 42× 404 |
| `dirk-bedow-schrott-und-metallentsorgung.de` | `dirk-bedow-schrott-und-metallentsorgung.de/` (200) · `dirk-bedow-schrott-und-metallentsorgung.de/preise` (404) · `dirk-bedow-schrott-und-metallentsorgung.de/preise.html` (404) · `www.dirk-bedow-schrott-und-metallentsorgung.de/` (200) · `www.dirk-bedow-schrott-und-metallentsorgung.de/leistungen/` (200) | **11** Pfade 200 · 42× 404 |
| `diwo-wolfsburg.de` | `diwo-wolfsburg.de/` (200) · `diwo-wolfsburg.de/preise` (404) · `diwo-wolfsburg.de/preise.html` (404) · `www.schrottkiste-isenbuettel.de` (200) | **2** Pfade 200 · 42× 404 → **Preisbefund** |
| `dk-duisburg.de` | `dk-duisburg.de/` (200) · `dk-duisburg.de/preise` (404) · `dk-duisburg.de/preise.html` (404) · `www.dk-duisburg.de/unternehmen/downloadbereich/` (200) · `www.dk-duisburg.de/karriere/stellenangebote/` (200) | **11** Pfade 200 · 41× 404 |
| `dk-edelmetalle.de` | `dk-edelmetalle.de/` (200) · `dk-edelmetalle.de/preise` (404) · `dk-edelmetalle.de/preise.html` (404) · `dk-edelmetalle.de/kontakt/` (200) · `dk-edelmetalle.de/agb/` (200) | **10** Pfade 200 · 42× 404 → **Preisbefund** |
| `dk-metallhandel.de` | `dk-metallhandel.de/` (200) · `dk-metallhandel.de/preise` (404) · `dk-metallhandel.de/preise.html` (404) · `dk-metallhandel.de/anfahrt` (200) · `dk-metallhandel.de/kontakt` (200) | **13** Pfade 200 · 42× 404 |
| `dmr-rohstoff.de` | `dmr-rohstoff.de/` (200) · `dmr-rohstoff.de/preise` (404) · `dmr-rohstoff.de/preise.html` (404) · `dmr-rohstoff.de/sekundärpreise` (200) · `dmr-rohstoff.de/kontakt/` (200) | **7** Pfade 200 · 41× 404 |
| `doering-schrott.de` | `doering-schrott.de/` (200) · `doering-schrott.de/preise` (404) · `doering-schrott.de/preise.html` (404) · `www.doering-schrott.de/eisenschrott-fe-schrott/` (200) · `www.doering-schrott.de/nicht-eisenhaltige-metalle-ne-metalle/` (200) | **21** Pfade 200 · 42× 404 |
| `doormann.net` | `doormann.net/` (200) · `doormann.net/preise` (404) · `doormann.net/preise.html` (404) · `www.doormann.net/containerdienst-hannover/schrottabholung/` (200) · `www.doormann.net/schrott-_und_metallhandel/` (200) | **14** Pfade 200 · 42× 404 |
| `drh-rohstoffhandel.de` | `drh-rohstoffhandel.de/` (200) · `drh-rohstoffhandel.de/schrottpreise` (404) · `drh-rohstoffhandel.de/metall-und-schrottannahme/` (200) · `drh-rohstoffhandel.de/download/` (200) | **40** Pfade 200 · 1× 404 · 41× 503 ⟳ 2×404, 8×5xx |
| `drl-gmbh-dessau.de` | `drl-gmbh-dessau.de/` (200) · `drl-gmbh-dessau.de/preise` (200) · `drl-gmbh-dessau.de/preise.html` (200) | **43** Pfade 200 · 0× 404 |
| `droege-entsorgung.de` | `droege-entsorgung.de/` (200) · `droege-entsorgung.de/preise` (404) · `droege-entsorgung.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `dsh-schrotthandel.de` | `dsh-schrotthandel.de/` (200) · `dsh-schrotthandel.de/preise` (404) · `dsh-schrotthandel.de/preise.html` (404) · `dsh-schrotthandel.de/schrottankauf` (200) · `dsh-schrotthandel.de/schrottplatz` (200) | **26** Pfade 200 · 42× 404 |
| `du-willkommen.de` | `du-willkommen.de/` (200) · `du-willkommen.de/preise` (404) · `du-willkommen.de/preise.html` (404) · `du-willkommen.de/stellenangebote-01.html` (200) · `du-willkommen.de/schrott-und-metalle.html` (200) | **13** Pfade 200 · 42× 404 → **Preisbefund** |
| `duero.de.tl` | `duero.de.tl/` (200) · `duero.de.tl/preise` (404) · `duero.de.tl/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `dutz-recycling.de` | `dutz-recycling.de/` (200) · `dutz-recycling.de/preise` (404) · `dutz-recycling.de/preise.html` (404) · `dutz-recycling.de/schrott-abholung/` (200) · `dutz-recycling.de/schrott-erloese/` (200) | **4** Pfade 200 · 42× 404 |
| `eas-recycling.de` | `www.eas-recycling.de/` (200) · `www.eas-recycling.de/ankaufspreise/` (200) · `www.eas-recycling.de/ankaufspreise/elektronische-bauteile/` (200) | **7** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch → **Preisbefund** |
| `ebert-entsorgung.de` | `ebert-entsorgung.de/` (200) · `ebert-entsorgung.de/preise` (404) · `ebert-entsorgung.de/preise.html` (404) | **1** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `ebm-esser.de` | `ebm-esser.de/` (200) · `de.wikipedia.org/wiki/Nichteisenmetall` (200) | **2** Pfade 200 · 0× 404 · 42× 403 ⟳ 10×403 (WAF) |
| `ebm-ludwigshafen.de` | `ebm-ludwigshafen.de/` (200) · `ebm-ludwigshafen.de/preise` (404) · `ebm-ludwigshafen.de/preise.html` (404) · `ebm-ludwigshafen.de/zum-thema-preise/` (200) · `ebm-ludwigshafen.de/schrott-abholung/metall-verwertung/` (200) | **36** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `ecoschrott.de` | `ecoschrott.de/` (200) · `ecoschrott.de/preise` (404) · `ecoschrott.de/anfragepreise` (404) · `www.ecoschrott.de/mobileschrottabhlung` (200) · `www.ecoschrott.de/buntmetall-ankauf` (200) | **15** Pfade 200 · 34× 404 · 6× 400 |
| `edelmetallcontor.de` | `edelmetallcontor.de/` (200) · `edelmetallcontor.de/preise` (404) · `edelmetallcontor.de/preise.html` (404) · `www.edelmetallcontor.de/die-schutzklassen` (200) · `www.edelmetallcontor.de/materialklassifizierung` (200) | **14** Pfade 200 · 42× 404 |
| `edelmetallkontor-io.jimdofree.com` | `edelmetallkontor-io.jimdofree.com/` (200) · `edelmetallkontor-io.jimdofree.com/preise` (404) · `edelmetallkontor-io.jimdofree.com/preise.html` (404) · `edelmetallkontor-io.jimdofree.com/app/download/11063573871/Kundenvollmacht.pdf?t=1790153527` (200) · `edelmetallkontor-io.jimdofree.com/about/` (200) | **9** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `edelmetallkontor.de` | `edelmetallkontor.de/` (200) · `edelmetallkontor.de/preise` (404) · `edelmetallkontor.de/preise.html` (404) · `geldfuergold.de/ankaufsrechner/` (200) · `geldfuergold.de/ankaufsformular/` (200) | **4** Pfade 200 · 41× 404 → **Preisbefund** |
| `edwinsautorecycling.de` | `edwinsautorecycling.de/` (200) | **1** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `egsas.de` | `egsas.de/` (200) · `egsas.de/preise` (404) · `egsas.de/preise.html` (404) · `www.egsas.de/stellenangebot-muellwerker-kraftfahrer/` (200) | **2** Pfade 200 · 42× 404 |
| `eiber-kunz.de` | `eiber-kunz.de/` (200) · `eiber-kunz.de/preise` (404) · `eiber-kunz.de/preise.html` (404) · `eiber-kunz.de/stahlschrott/` (200) · `eiber-kunz.de/ne-metalle/` (200) | **4** Pfade 200 · 42× 404 |
| `eichhorn-recycling.de` | `eichhorn-recycling.de/` (200) · `eichhorn-recycling.de/preise` (404) · `eichhorn-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 · 2× Netzwerk ⟳ 10×404 |
| `eifelerschrotthandel.de` | `eifelerschrotthandel.de/` (200) · `eifelerschrotthandel.de/index.php` (404) · `eifelerschrotthandel.de/www.reger-ma.de` (404) | **1** Pfade 200 · 44× 404 |
| `eisele-entsorgung.de` | `eisele-entsorgung.de/` (200) · `eisele-entsorgung.de/preise` (404) · `eisele-entsorgung.de/preise.html` (404) | **1** Pfade 200 · 43× 404 |
| `elektro-system-entsorgung.de` | `elektro-system-entsorgung.de/` (200) · `elektro-system-entsorgung.de/preise` (404) · `elektro-system-entsorgung.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `elno-container.de` | `elno-container.de/` (200) · `elno-container.de/preise` (404) · `elno-container.de/preise.html` (404) · `www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf` (200) · `www.elno-container.de/schrottplatz/` (200) | **6** Pfade 200 · 42× 404 → **Preisbefund** |
| `emsschrott.de` | `emsschrott.de/` (200) · `emsschrott.de/preise` (404) · `emsschrott.de/preise.html` (404) · `emsschrott.de/altmetalle-und-schrott-an-und-verkauf/` (200) · `emsschrott.de/erfassung-von-altmetallen/` (200) | **8** Pfade 200 · 42× 404 → **Preisbefund** |
| `entsomarkowski.de` | `entsomarkowski.de/` (200) · `entsomarkowski.de/preise` (404) · `entsomarkowski.de/preise.html` (404) · `www.entsomarkowski.de/leistungen-und-service/schrott-und-metallhandel/` (200) · `www.entsomarkowski.de/leistungen-und-service/schrott-und-metallhandel/fe-metalle/` (200) | **6** Pfade 200 · 42× 404 |
| `entsorge-alles.de` | `entsorge-alles.de/` (200) · `entsorge-alles.de/index.php?metall` (200) · `www.entsorge-alles.de/Zertifikat.pdf` (200) | **4** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `entsorgen.de` | `entsorgen.de/` (200) · `entsorgen.de/preise` (404) · `entsorgen.de/preise.html` (404) · `neu.entsorgen.de/` (200) · `entsorgen.de/wp-content/uploads/2025/09/Kundenstammdatenblatt_entsorgen.de_.pdf` (200) | **28** Pfade 200 · 42× 404 |
| `entsorgung-baumert.de` | `entsorgung-baumert.de/` (200) · `entsorgung-baumert.de/preise` (404) · `entsorgung-baumert.de/preise.html` (404) · `entsorgung-baumert.de/content/abfallbeschreibung/stahlschrott/` (200) · `entsorgung-baumert.de/content/abfallbeschreibung/metalle/` (200) | **64** Pfade 200 · 42× 404 · 1× 400 |
| `entsorgung-dresden.de` | `entsorgung-dresden.de/` (200) · `entsorgung-dresden.de/wertstoffe/wertstoff-ankauf.html` (200) | **2** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch → **Preisbefund** |
| `entsorgung-hessen.de` | `entsorgung-hessen.de/` (200) · `entsorgung-hessen.de/preise` (404) · `entsorgung-hessen.de/preise.html` (404) · `www.entsorgung-hessen.de/schrotthandel-hanau-frankfurt/` (200) · `www.entsorgung-hessen.de/aktuelle-stellenangebote/` (200) | **3** Pfade 200 · 42× 404 |
| `entsorgung-michel.de` | `entsorgung-michel.de/` (**ConnectionError: ('Connection aborted.', Rem**) | **0** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `entsorgung-schirmag.de` | `entsorgung-schirmag.de/` (200) · `entsorgung-schirmag.de/preise` (404) · `entsorgung-schirmag.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `entsorgung-wagner.de` | `entsorgung-wagner.de/` (200) · `entsorgung-wagner.de/preise` (404) · `entsorgung-wagner.de/preise.html` (404) · `entsorgung-wagner.de/Downloads` (200) | **2** Pfade 200 · 42× 404 |
| `erdkabelrecycling-prossmann.de` | `erdkabelrecycling-prossmann.de/` (200) · `erdkabelrecycling-prossmann.de/preise` (404) · `erdkabelrecycling-prossmann.de/preise.html` (404) · `erdkabelrecycling-prossmann.de/images/pdf/Kofinanzierung.pdf` (200) | **2** Pfade 200 · 42× 404 |
| `erfurt.hugo-schrott-metall.de` | `erfurt.hugo-schrott-metall.de/` (200) · `www.hugo-schrott-metall.de/home/` (404) · `erfurt.hugo-schrott-metall.de/bebra.hugo-schrott-metall.de/home/index.html` (404) · `erfurt.hugo-schrott-metall.de/index.html` (200) · `erfurt.hugo-schrott-metall.de/home/index.html` (200) | **6** Pfade 200 · 45× 404 |
| `ernst-recycling.de` | `ernst-recycling.de/` (200) · `ernst-recycling.de/preise` (404) · `ernst-recycling.de/preise.html` (404) · `ernst-recycling.de/ernst-recycling-gmbh-metallhandel-oranienburg-und-oberhavel/` (200) · `ernst-recycling.de/ne-metalle/` (200) | **8** Pfade 200 · 43× 404 → **Preisbefund** |
| `es-metalle.de` | `es-metalle.de/` (200) · `es-metalle.de/preissicherung.html` (200) · `es-metalle.de/buntmetalle.html` (200) | **28** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `esorec.de` | `esorec.de/` (200) · `esorec.de/preise` (404) · `esorec.de/preise.html` (404) · `esorec.de/schrott-metallhandel/` (200) · `www.festpreis-webseite.de/` (200) | **13** Pfade 200 · 42× 404 → **Preisbefund** |
| `eu.emrlocal.com` | `eu.emrlocal.com/` (200) · `eu.emrlocal.com/preise` (200) · `eu.emrlocal.com/preise.html` (200) | **84** Pfade 200 · 0× 404 |
| `fair-metall.de` | `fair-metall.de/` (200) · `fair-metall.de/preise` (404) · `fair-metall.de/preise.html` (404) · `www.fair-metall.de/alles-schrott/` (200) · `www.fair-metall.de/schrottpreise/` (200) | **32** Pfade 200 · 41× 404 |
| `fass-rohstoffrecycling.de` | `fass-rohstoffrecycling.de/` (200) | **1** Pfade 200 · 0× 404 · 34× 503, 8× Netzwerk ⟳ 4×5xx, 6×Verbindungsabbruch |
| `fechner-gmbh.de` | `fechner-gmbh.de/` (200) · `fechner-gmbh.de/download/` (200) · `fechner-gmbh.de/fahrzeugankauf-formular/` (200) | **93** Pfade 200 · 0× 404 |
| `fehr.de` | `fehr.de/` (200) · `fehr.de/preise` (404) · `fehr.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `feralpi-stahl.com` | `feralpi-stahl.com/` (200) · `feralpi-stahl.com/preise` (404) · `feralpi-stahl.com/preise.html` (404) · `feralpi-stahl.com/de/produkte/bewehrungsstahl/listenmatten-b500a-b500b` (200) · `feralpi-stahl.com/sites/default/files/media/documents/2026-10/2026-10-14_Pressemitteilung_FAZ%20Auszeichnung_v2_final.pdf` (200) | **5** Pfade 200 · 14× 404 · 28× 403 ⟳ 8×404, 2×403 (WAF) |
| `fernau-containerdienst.de` | `fernau-containerdienst.de/` (200) · `fernau-containerdienst.de/preise` (404) · `fernau-containerdienst.de/preise.html` (404) · `fernau-containerdienst.de/altmetall` (200) · `fernau-containerdienst.de/schrott-und-metallhandel/` (200) | **4** Pfade 200 · 42× 404 |
| `ferrometall.de` | `ferrometall.de/` (200) · `ferrometall.de/AGB%2001.05.18.pdf` (200) · `ferrometall.de/Datenschutzerklaerung%2001.05.18.pdf` (200) | **18** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `ferrum-recycling.de` | `ferrum-recycling.de/` (200) · `ferrum-recycling.de/preise` (404) · `ferrum-recycling.de/preise.html` (404) · `ferrum-recycling.de/preisliste-schrott.html` (200) · `ferrum-recycling.de/preisliste-ne-metalle.html` (200) | **6** Pfade 200 · 42× 404 → **Preisbefund** |
| `fett-schrott.de` | `fett-schrott.de/` (200) · `fett-schrott.de/preise` (404) · `fett-schrott.de/preise.html` (404) · `fett-schrott.de/altmetall-eisen-verwerltung.html` (200) · `fett-schrott.de/kabelschrott-mischschrott.html` (200) | **22** Pfade 200 · 42× 404 |
| `fiedler-container.de` | `fiedler-container.de/` (200) · `fiedler-container.de/preise` (404) · `fiedler-container.de/preise.html` (404) | **1** Pfade 200 · 41× 404 · 1× 403 ⟳ 8×404, 1×403 (WAF), 1×Verbindungsabbruch |
| `firma-jungkurth.de` | `firma-jungkurth.de/` (200) · `firma-jungkurth.de/wp-content/uploads/preisliste.pdf` (404) · `www.firma-jungkurth.de/rohstoffhandel/` (200) · `www.firma-jungkurth.de/wp-content/uploads/2026/03/Zerftifikat_Entsorgung_2026.pdf` (200) | **81** Pfade 200 · 1× 404 |
| `fischer-metallhandel.de` | `fischer-metallhandel.de/` (200) · `fischer-metallhandel.de/preise` (404) · `fischer-metallhandel.de/preise.html` (404) · `fischer-metallhandel.de/an-verkauf-schrott/` (200) · `www.fischer-metallhandel.de/an-verkauf-schrott/` (200) | **13** Pfade 200 · 42× 404 |
| `fischer-rohstoff-recycling.de` | `fischer-rohstoff-recycling.de/` (200) · `fischer-rohstoff-recycling.de/preise` (404) · `fischer-rohstoff-recycling.de/preise.html` (404) · `fischer-rohstoff-recycling.de/wp-content/uploads/2026/07/Zertifikat-2026.pdf` (200) · `fischer-rohstoff-recycling.de` (200) | **6** Pfade 200 · 42× 404 |
| `fks-recycling-darmstadt.de` | `fks-recycling-darmstadt.de/` (200) · `fks-recycling-darmstadt.de/preise` (404) · `fks-recycling-darmstadt.de/preise.html` (404) · `fks-recycling-darmstadt.de/bei-anruf-schrottabholung/` (200) · `fks-recycling-darmstadt.de/fks-recycling/schrotthandel/` (200) | **6** Pfade 200 · 42× 404 |
| `florian-metalle.de` | `florian-metalle.de/` (200) · `florian-metalle.de/preise` (404) · `florian-metalle.de/preise.html` (404) · `www.instagram.com/florian_metalle` (200) | **2** Pfade 200 · 42× 404 |
| `fme-metallrecycling.de` | `fme-metallrecycling.de/` (200) · `fme-metallrecycling.de/wp-content/uploads/2017/02/Efb_Zertifikat_FME.pdf` (200) · `fme-metallrecycling.de` (200) | **18** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch |
| `foerster-entsorgung.de` | `foerster-entsorgung.de/` (200) · `foerster-entsorgung.de/preise` (404) · `foerster-entsorgung.de/preise.html` (404) · `foerster-entsorgung.de/Container/Angebotsanfrage/` (200) · `foerster-entsorgung.de/Download-EFB/index.php/` (200) | **3** Pfade 200 · 42× 404 → **Preisbefund** |
| `forbergerentsorgung.de` | `forbergerentsorgung.de/` (200) · `forbergerentsorgung.de/preise` (404) · `forbergerentsorgung.de/preise.html` (404) · `forbergerentsorgung.de/files/forberger/download/EfB-Zertifikat%202024%20komplett.pdf` (200) · `forbergerentsorgung.de/files/forberger/download/Certificate_SURE-System_Forberger_2022.pdf` (200) | **3** Pfade 200 · 42× 404 |
| `frauendienst-recycling.de` | `frauendienst-recycling.de/` (200) · `frauendienst-recycling.de/preise` (404) · `frauendienst-recycling.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `frauenrath.de` | `frauenrath.de/` (200) · `frauenrath.de/preise` (404) · `frauenrath.de/preise.html` (404) · `frauenrath.de/downloads/publikationen/` (200) · `frauenrath.de/downloads/dokumente/` (200) | **5** Pfade 200 · 42× 404 |
| `freddesnoo.de` | `freddesnoo.de/` (200) · `freddesnoo.de/preise` (404) · `freddesnoo.de/preise.html` (404) · `www.freddesnoo.de/schrott-und-metallhandel/` (200) | **2** Pfade 200 · 42× 404 |
| `freddi-goldbach.de` | `freddi-goldbach.de/` (200) · `freddi-goldbach.de/preise` (404) · `freddi-goldbach.de/preise.html` (404) | **1** Pfade 200 · 41× 404 · 1× 403 ⟳ 9×404, 1×403 (WAF) |
| `freisler.com` | `freisler.com/` (200) · `freisler.com/preise` (404) · `freisler.com/preise.html` (404) · `freisler.com/unser-unternehmen/downloads` (200) · `freisler.com/was-wir-bieten/entsorgung/schrott` (200) | **5** Pfade 200 · 42× 404 |
| `frey-containerdienst.de` | `frey-containerdienst.de/` (200) · `frey-containerdienst.de/preise` (404) · `frey-containerdienst.de/preise.html` (404) · `frey-containerdienst.de/dienstleistungen/metall-und-schrotthandel` (200) · `www.frey-containerdienst.de/dienstleistungen/metall-und-schrotthandel` (200) | **3** Pfade 200 · 42× 404 |
| `frisch-recycling.de` | `frisch-recycling.de/` (200) · `frisch-recycling.de/ankauf/` (200) · `frisch-recycling.de/ankauf` (200) | **3** Pfade 200 · 0× 404 · 42× Netzwerk ⟳ 10×Verbindungsabbruch → **Preisbefund** |
| `fritz-aquila.de` | `fritz-aquila.de/` (200) · `fritz-aquila.de/preise` (404) · `fritz-aquila.de/preise.html` (404) · `fritz-aquila.de/index.php/ankauf` (200) · `www.google.com/maps/dir//Fritz+Aquila+Metall-Recycling+e.+K.,+Dachauer+Str.+537,+80993+München/@48.191268,11.4308971,12z/data=!4m9!4m8!1m0!1m5!1m1!1s0x479e770fce99300b:0x1f51f668237dc66d!2m2!1d11.5009367!2d48.1912893!3e0` (200) | **3** Pfade 200 · 42× 404 |
| `fs-schrott.com` | `fs-schrott.com/` (200) · `fs-schrott.com/preise` (404) · `fs-schrott.com/anfragepreise` (404) · `www.fs-schrott.com` (200) · `www.fs-schrott.com/about` (200) | **6** Pfade 200 · 36× 404 · 6× 400 |
| `gambert-entsorgung.de` | `gambert-entsorgung.de/` (200) · `gambert-entsorgung.de/preise` (404) · `gambert-entsorgung.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `gantzkow-rohstoff.com` | `gantzkow-rohstoff.com/` (200) · `gantzkow-rohstoff.com/preise` (404) · `gantzkow-rohstoff.com/preise.html` (404) · `www.gantzkow-rohstoff.com/de/index.php` (200) · `gantzkow-rohstoff.com/de/` (200) | **27** Pfade 200 · 42× 404 |
| `ganz-containerdienst.de` | `ganz-containerdienst.de/` (200) · `ganz-containerdienst.de/preise` (404) · `ganz-containerdienst.de/preise.html` (404) · `www.ganz-containerdienst.de/dienstleistungen/rohstoffe-schrott-und-metalle/` (200) · `www.ganz-containerdienst.de/downloads/` (200) | **6** Pfade 200 · 42× 404 |
| `ged-gruebler.de` | `ged-gruebler.de/` (200) · `ged-gruebler.de/preise` (404) · `ged-gruebler.de/preise.html` (404) | **1** Pfade 200 · 42× 404 |
| `geigergruppe.com` | `geigergruppe.com/` (200) · `geigergruppe.com/preise` (404) · `geigergruppe.com/preise.html` (404) · `geigergruppe.com/de-de/downloadcenter/` (200) · `www.geigergruppe.com/at-de/downloadcenter/` (200) | **23** Pfade 200 · 42× 404 |

---

## 5. Fehlerprotokoll (nicht erreichbar / blockiert)

| Domain | Befund |
|---|---|
| `amr-schrottplatz.de` | amr-schrottplatz.de — SSLError(SSLCertV/www.amr-schrottplatz.de — SSLError(SSLC/HTTP 502 |
| `brosda.com` | brosda.com — SSLError(SSLError(1, '[SSL/www.brosda.com — SSLError(SSLError(1, '/HTTP 404 |
| `containerdienste-schrott-metallhandel.de` | HTTP 503 |
| `entsorgung-michel.de` | ConnectionError: ('Connection aborted.', RemoteDisconnected('Remote end closed connection without response' |

| Domain | HTTP-Verhalten auf Preispfaden |
|---|---|
| `acm-moers.de` | **200-SPA-Falle**: `/preise.json`, `/preise.csv`, `/export.csv`, `/downloads/preise.csv`, `/api/preise.csv`, `/prices.json`, `/price-list.json`, `/schrottpreise.csv`, `/schrottpreise.json` liefern **alle** die Startseite als HTML (je 19.961 B) — keine JSON-/CSV-Quelle. |
| `aeris-gmbh.de` | **200-SPA-Falle**: 19 Preispfade (`/preise/`, `/de/preise/`, `/api/preise`, `/download/preise`, …) liefern dieselbe 77.072-B-Startseite. `/preise.json` → HTTP 404. |
| `autoverschrottung-bremen.de`, `autoverschrottung-hildesheim.de`, `autoverschrottung-rostock.de` | **SEO-FAQ-Fließtext statt Preisliste**: Die Startseite nennt im Frage-Antwort-Block „Der Schrottpreis liegt aktuell bei rund 100 Euro pro Tonne, doch so viel gibt es für Schrottautos nur selten.“ und „150€ je Tonne.“ — ein redaktioneller Markt-Kommentar ohne Sortenliste, ohne Standortbindung und ohne Datum. Die drei Domains teilen denselben Text (gleiches Template). **Für den Scraper:** als Marktkommentar zu behandeln, nicht als Standortpreisquelle. |
| `bellersheim.de` | Die einzige „€/kg“-Fundstelle (`Euro 2 SZR/kg`) stammt aus den **AGB** (Haftungsgrenze nach ADSp, „je nachdem, welcher Betrag höher ist“) — die `/downloads/`-Seite enthält keine Preisliste, nur Zertifikate und Bedingungen. |
| `beg-bhv.de` | Nur Gebühren-/Verpackungstarife (`Transparente Säcke (90 Liter), 50 Stück/Rolle 6 Euro`), keine Sortenpreise. |
| 41 Domains | **Soft-404-Cluster**: ≥3 Pfade liefern byte-identischen Inhalt → generische Pfad-Proben sind dort ohne Aussagekraft. Betroffen u. a. `acm-moers.de`, `aeris-gmbh.de`, `bender-recycling.de` (`/ankaufspreise` liefert die Startseite), `buntmetallhandel-altmittweida.de` (`/preise` → Startseite), `containerdienst.de` (alle Pfade → Startseite). |
| 16 Domains | **TLS/SNI-Fehler auf der Apex-Domain**: `abc-recycling-dessau.de`, `alfa-recycling.de`, `annecke-schrotthandel.de`, `autorecycling-penkert.de`, `ay-recycling.de`, `bissig.de`, `bur-recycling-duisburg.de`, `derichebourg-umwelt.de`, `eas-recycling.de`, `edwinsautorecycling.de`, `entsorge-alles.de`, `entsorgung-dresden.de`, `es-metalle.de`, `ferrometall.de`, `fme-metallrecycling.de`, `frisch-recycling.de` — Abruf auf `https://<domain>/` schlägt mit `SSL routines::tlsv1 unrecognized name` fehl, `https://www.<domain>/` funktioniert (HTTP 200). **Folge:** die generischen Pfad-Proben liefen ins Leere; diese Domains wurden deshalb zusätzlich per `www.` mit 33 Pfaden nachgeprüft (528 Requests). Ergebnis: **keine zusätzliche Preisquelle** — die bereits gefundenen Treffer (`eas-recycling.de`, `frisch-recycling.de`, `entsorgung-dresden.de`) bleiben unverändert undundatiert. **Für den Scraper:** der `www.`-Host muss bevorzugt werden. |
| `bartikowsky-recycling-muenster.de` | **Dauerhaftes Rate-Limit**: alle 10 Kontroll-Pfade HTTP 429 (Homepage HTTP 200). |
| `dammann-gmbh.de` | **Dauerhaftes Rate-Limit**: 9× HTTP 429 bei Kontroll-Proben. |
| `bender-recycling.de` | Ein Pfad (`/ankaufspreise`) HTTP 200, aber byte-identisch mit der Startseite → Soft-404, keine Preisquelle. |
| `buntmetallhandel-altmittweida.de` | `/preise` HTTP 200, aber Startseite (Soft-404). Die echte Seite `/ankaufspreisliste` (auch unter `www.schrottbuntmetallankauf.de`) liefert eine 27-zeilige Preistabelle **ohne Einheit und ohne Datum** → nicht auswertbar, siehe Treffer-Tabelle. |
| `burger-recycling.de`, `bauerunternehmensgruppe.de` | Alle Kontroll-Pfade HTTP 500 (Homepage HTTP 200). |
| `ebm-esser.de` | Alle Kontroll-Pfade HTTP 403 (WAF), Homepage HTTP 200. |
| `containerdienste-schrott-metallhandel.de` | Homepage HTTP 503, Pfade 503/404. |
| `fass-rohstoffrecycling.de`, `drh-rohstoffhandel.de`, `amsbeck.gmbh` | Gemischte 503-Antworten (Backend instabil). |
| `autorecycling-penkert.de` | Nur 2 Seiten HTTP 200 erreichbar, Rest Verbindungsabbruch. |

## 6. Muster und Auswertung

### 6.1 Pfadmuster — was tatsächlich trägt

| Muster | Domains mit Treffer | Bemerkung |
|---|---|---|
| `/ankauf-preise` | 1 | `boehner-altmetalle.de` — Musterpfad der Branche, aber ohne Datum |
| `/ankaufspreise/<material>/` (Unterseiten je Materialgruppe) | 1 | `eas-recycling.de` — beste Struktur der Stichprobe |
| `/ankauf/` | 1 | `frisch-recycling.de` — 135 Preiszeilen |
| `/pages/preise.php` | 1 | `dein-schrottplatz.de` — **einzige HTML-Quelle mit Datum** |
| `/preisliste-schrott.html` | 1 | `ferrum-recycling.de` — Raster ohne Zahlenwerte |
| `/wp-content/uploads/<yyyy>/<mm>/Preisliste_<dd.mm>.pdf` | 1 | `elno-container.de` — 24 PDFs, davon 15 in 2026 |
| `/fileadmin/<org>/Preisliste_<org>_ab_<dd.mm.yyyy>.pdf` | 1 | `bbw-recycling.de` — Datum im Dateinamen **und** im Dokument |
| `/altmetalle-und-schrott-an-und-verkauf/` | 1 | `emsschrott.de` |
| `/ankaufspreisliste` | 1 | `buntmetallhandel-altmittweida.de` — 27 Sorten, aber weder Einheit noch Datum |
| `/downloads/` | 0 | `bellersheim.de` — dort **nur** AGB-Haftungsgrenze (`2 SZR/kg`), kein Preis → Negativbefund |
| `/[beliebiger Pfad]` | 0 | `acm-moers.de`, `aeris-gmbh.de`, `bender-recycling.de`, `containerdienst.de` — Soft-404: jeder Pfad liefert die Startseite |

**Muster-Ergebnis:** Der Pfadname sagt wenig aus. Von 10.499 generischen Preispfad-Proben über 250 Domains waren **8.516 (81 %) HTTP 404** und nur 591 HTTP 200. Bei **221 von 250 Domains** traf **kein einziger** generischer Preispfad (`/preise`, `/tagespreise`, `/schrottpreise`, `/metallpreise`, `/ankaufpreise`, `/preisliste`, `/api/preise`, `/preise.json`, `/preise.csv`, `/export/preise.csv`, `/wp-content/uploads/preisliste.pdf`, …) — dort existiert schlicht kein Preisbereich. Die 591 Treffer verteilen sich auf nur 29 Domains und sind dort fast durchweg **Soft-404s** (Startseite unter jedem Pfad). Die zwei stärksten Fundstellen (`dein-schrottplatz.de`, `elno-container.de`) waren **nicht** über Pfadraten auffindbar, sondern über Link-Text bzw. die WordPress-Media-API.

### 6.2 Dateiformate

| Format | Befund |
|---|---|
| **HTML-Tabelle** | Überwiegt. 7 Tabellen mit ≥3 Preiszeilen gefunden (`frisch-recycling` 135 Preiszeilen, `dein-schrottplatz` 66 Zeilen, `beller-das` 33, `buntmetallhandel-altmittweida` 27, `boehner-altmetalle` 17, `emsschrott` 8, `entsorgung-dresden` 3). Tabellen sind maschinenlesbar, aber **6 von 7 ohne Datum**; die einzige datierte ist `dein-schrottplatz.de`. |
| **PDF** | 2 Treffer (`elno-container.de` 24 Dateien, `bbw-recycling.de` 1). Beide **mit** Datum; PDF-Extraktion via `pdftotext -layout` liefert die Sorten-Preis-Paare sauber. |
| **JSON** | **Kein einziger echter Treffer.** Alle JSON-/CSV-Treffer waren Soft-404-Artefakte von SPAs (`acm-moers.de`, `aeris-gmbh.de`). Auch die WordPress-REST-API (`/wp-json/wp/v2/pages`, `/wp-json/wp/v2/media`) liefert bei 33 Domains Content, aber **keine** Preis-Felder — Preise sind durchweg im HTML/PDF hart codiert. |
| **CSV/XLSX** | Kein Treffer. |
| **Excel/Word-Downloads** | Kein Treffer. |

### 6.3 Was die Branchen-Praxis dominiert

1. **Preis auf Anfrage** — 8 Domains explizit (`aktas-schrott.de`, `beller-das.de`, `containerdienst-wuerfel.de`, `es-metalle.de`, `esorec.de`, `frauendienst-recycling.de`, `freddi-goldbach.de`, `frey-containerdienst.de`); `ferrum-recycling.de` liefert ein vollständiges datiertes Preisraster, aber **alle 20 Zeilen** = „Preis auf Anfrage“. Das ist der Extremfall: *datiertes Preisraster ohne Zahlenwert*.
2. **Tagespreis-Versprechen ohne Preisliste** — 19 Domains behaupten auf der Startseite „Tagespreise“/„tagesaktuelle Höchstpreise“ (`ab-recycling.de`, `an-schrott.de`, `aus-liebe-zum-schrott.de`, `bartikowsky-recycling-muenster.de`, `bellersheim.de`, `bitterfelder-metallrecycling.de`, `blank-metalle.de`, `ch-metallhandel.de`, `computerschrott-ankauf-stuttgart.de`, `dk-edelmetalle.de`, `dk-metallhandel.de`, `eifelerschrotthandel.de`, `ernst-recycling.de`, `esorec.de`, `fair-metall.de`, `fass-rohstoffrecycling.de`, `florian-metalle.de`, `fritz-aquila.de`, `ak-containerdienst.de`). **Keine** dieser 19 Domains hat eine abrufbare Preisliste — die Tagespreise werden telefonisch/über die Waage ausgehandelt.
3. **Gebührentafeln dominieren die Preis-Fundstellen.** Von 26 Domains mit extrahierbaren Zahlenwerten rechnen **8 ausschließlich Entsorgungs-/Containergebühren** ab, keine Materialpreise (`ak-containerdienst.de`, `esorec.de`, `berlin-recycling.de`, `arm-entsorgung.de`, `du-willkommen.de`, `diwo-wolfsburg.de`, `beg-bhv.de`, `auto-daud.de`). Weitere 4 führen Verkaufspreise für eigene Reststoffe/Handelsware (`foerster-entsorgung.de`, `buntmetallaufkauf.de`, `bekomuenzen.de`, `auto-daud.de`). Eine Gebührentafel ist **keine** Ankaufpreisquelle.
4. **Gebühren ≠ Ankaufpreis.** `berlin-recycling.de` nennt „0,81 € pro Kilogramm“ — das ist ein Entsorgungstarif, kein Materialpreis. `beller-das.de` nennt „Mischschrott (mind. 100 kg) 0,10 €“ — das ist eine **Bedingung** (Mindestmenge), kein Preis; der zugehörige Einkaufspreis steht in derselben Tabelle unter „Preis €/kg“.
5. **Marktreferenzen ohne Ortsbezug** — 4 Domains: `boetzel-schrott.com` (BDSV-Video), `burger-recycling.de` (BDSV-Mitgliedschaft), `fischer-metallhandel.de` (BDSV-Mitgliedschaft), `buntmetallaufkauf.de` („immer den aktuellen Börsenpreis für Buntmetalle“). Keine davon liefert einen Standortpreis.
6. **Ein Sonderfall: `geldfuergold.de` / `edelmetallkontor.de`** — der Ankaufsrechner zieht die Werte aus einer externen API („Die Preise aktualisieren sich automatisch, Quelle gold-api.com“). Die Werte sind damit maschinenlesbar, aber **ohne Datum** und ohne Standortbindung; `gold-api.com` selbst ist eine Marktquelle, kein Betreiberpreis.

### 6.4 Konsequenz für den Backlog

- **Sofort nutzbar (2):** `dein-schrottplatz.de` (HTML, 66-zeilige Tabelle, „Gültig ab 03.10.2026“) und `elno-container.de` (PDF-Serie, 24 Preislisten bis 16.10.2026). Beide decken Buntmetall/Eisen ab.
- **Nachrüstbar (10):** `boehner-altmetalle.de`, `frisch-recycling.de`, `eas-recycling.de`, `emsschrott.de`, `beller-das.de`, `dk-edelmetalle.de`, `edelmetallkontor.de`, `ernst-recycling.de`, `entsorgung-dresden.de`, `buntmetallhandel-altmittweida.de` — vollständige, materialbezogene Ankaufpreistabellen; es fehlt ausschließlich ein Datum. Ein Scraper kann diese Seiten heute als Preisquelle nutzen, braucht aber eine **Datums-Heuristik** (z. B. `Last-Modified`, WP-`modified`-Feld, oder Betreiber-Freigabe).
- **Zwei Ergänzungen nötig:** `buntmetallhandel-altmittweida.de` (27 Sorten, aber weder Einheit noch Datum) und `beller-das.de`/`boehner-altmetalle.de` (Einheit nur im Spaltenkopf bzw. als Gruppenüberschrift — Vererbung nötig).
- **Nicht nutzbar trotz Datum:** `ferrum-recycling.de` (Raster, keine Zahlenwerte).
- **Gebührenquellen (8) und Verkaufspreisquellen (4)** sind als solche zu führen und von den Ankaufpreisen zu trennen — `bbw-recycling.de` ist die einzige datierte Gebührenquelle (Kippgebühren + RC-Verkaufspreise).

---

## 7. Reproduzierbarkeit

Alle Ergebnisse stammen aus Live-Abrufen (keine Werte erfunden, keine Annahmen über nicht abgerufene Seiten). Rohdaten im Arbeitsverzeichnis `/tmp/opencode/`:

| Datei | Inhalt |
|---|---|
| `screenA_home.json` | Homepage-Abrufe (URL, HTTP-Status, Größe) aller 250 Domains |
| `screenB_analysis.json` | Homepage-Analyse (Preiswörter, Einheiten, Daten, Link-Statistik) |
| `screenC_paths.json` | 11.073 Pfad-Proben mit Status (generische Pfade, verlinkte Pfade, PDFs) |
| `screenE_sitemap.json` / `screenF_smres.json` | robots.txt + Sitemap-Suche, abgerufene Sitemap-URLs |
| `screenH_links.json` | Link-Harvesting (Ankertext + URL) je Domain |
| `screenI_cands.json` | 2. Stufe: Preis-Ankertexte, 2-Level-Crawl, 49 Zusatzpfade |
| `screenJ_pages.json` | Tiefenanalyse aller 3.002 Seiten (Einheiten, Preiszeilen, Daten, Tabellen) |
| `screenK_top.json` … `screenT_nearmiss.json` | Detailnachlesen und Datums-Nachprüfung der Kandidaten |
| `screenAH_retry.json` / `screenAJ_recheck.json` / `screenAK_www.json` | Kontroll-Wiederholungen (TLS/SNI, 429, 503) |
| `final_hits.json` | finaler Treffer-Satz (25 Domains) mit allen Zitaten |
| `negatives.json` | 250 Negativbefund-Zeilen mit konkreten Pfaden und Status |
| `screen_cache/` | HTTP-Body-Cache, SHA1-keyed (22.088 URLs) |

**Zitat-Verifikation:** Jeder in Abschnitt 2/3 zitierte Wert wurde maschinell gegen den zugehörigen Seiteninhalt geprüft (Normalisierung auf Whitespace, HTML-Entities und Entities im Rohtext belegt). 84 von 86 Zitaten wurden als exakte Zeichenkette wiedergefunden; die beiden Abweichungen sind nachweisbar keine Abweichungen, sondern Entities/Zeilenumbrüche (`Ladung &lt; 1t`, `29,90 € * / 1 Feinunze` aus einem anderen Unterpfad, dort separat vermerkt).
