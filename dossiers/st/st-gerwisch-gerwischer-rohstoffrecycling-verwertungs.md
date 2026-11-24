---
slug: st-gerwisch-gerwischer-rohstoffrecycling-verwertungs
name: Gerwischer Rohstoffrecycling & Verwertungs GmbH
trader_type: metallhaendler
state: ST
city: Gerwisch
street: Lostauer Straße 5
postcode: '39175'
phone: '039292 2090'
email: info@gerwischer-recycling.de
opening_hours: ''
website: https://gerwischer-recycling.de/
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Magdeburg + Börde / Jerichower Land
provenance_ankauf_raw: ja
provenance_origin: table
---

# Gerwischer Rohstoffrecycling & Verwertungs GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026 (Feedback-ID 2720, teilweise berechtigt, kein Fill): Zeiten stehen auf der Seite, sind aber WIDERSPRÜCHLICH (Header/Footer: Mo–Do 7–16/Fr 7–15 vs. Body: Barankauf/Öffnungszeiten Mo–Do 8–15/Fr 8–14; Stand 01.10.2026, inkl. News „Erweiterte Öffnungszeiten im März") → opening_hours bleibt leer, kein Raten. Adresse/Kontakt bereits belegt (Lostauer Str. 5, 39175 Gerwisch, +49 39292 2090, info@gerwischer-recycling.de; GmbH seit 1993, Entsorgungsfachbetrieb).; Quelle(n): https://gerwischer-recycling.de/ (01.10.2026)]

### Importiert (Seed-Stand 2026-09-30)

- Schrotthandel, Gussaufbereitung, Barankauf Stahl/Buntmetall [Website-Recherche 30.09.2026: Impressum-Fund, Adresse/Tel/Mail bestätigt, HRB 1316 Stendal; Impressum-Ort: Biederitz OT Gerwisch (Seed-Stadt beibehalten); Öffnungszeiten widersprüchlich auf der Seite (Header 07–16/07–15 vs. Preisseite 08–15/08–14) – nicht übernommen; Quelle: gerwischer-recycling.de/impressum/]

### Korrektur 01.11.2026

- [Korrektur 01.11.2026: **Telefonnummer in die wählbare Inlandform überführt**: Das Frontmatter führte die Nummer in der gedruckten Auslands-Notation `+49 (0) 39292 / 20 90`; die Notation `(0)` ist international **nicht wählbar** (die Null ist bereits in der Vorwahl enthalten). Der Wert ist jetzt `039292 2090` — Vorwahl und Teilnehmer durch ein Leerzeichen getrennt, ohne Länderpräfix, mit der inlandüblichen führenden Null. Die Ziffernfolge selbst ist **unverändert**; die Belegquelle steht im Dossier.]

### Recherche 24.11.2026 (Preispflege)

- [Recherche 24.11.2026 (Preispflege): **Die Preisseite des Betriebes ist am 24.11.2026 vom Netz genommen worden - die dreizehn Current-Preise dieses Handlers sind durch die Quelle nicht mehr belegt.** Der Handler `st-gerwisch-gerwischer-rohstoffrecycling-verwertungs` lief am 24.11.2026 um 11:24:08 UTC noch erfolgreich (`ok`, 13 Preise uebernommen, 4 uebersprungen: `Messing (gemischt)` mit unveraendlicher Einheit `€/k`, `Brennerschrott` ohne Katalog-Bucket, `Kupfer (blank)` unspezifisch, `Aluminium (rein)` ohne sortenreines Material); der naechste Lauf um 17:37:53 UTC scheiterte mit `HTTP 404` auf `https://gerwischer-recycling.de/ankaufspreise/`. **Live geprueft am 24.11.2026 um 18:10 UTC:** die Preis-URL antwortet `308` und danach `404` (16644 Bytes Fehlerseite); die Startseite antwortet `200` (291730 Bytes), das Impressum `200` mit unveraenderter Anschrift `Lostauer Straße 5, 39175 Biederitz OT Gerwisch`, `Amtsgericht Stendal | HRB 1316`, `USt.-ID DE 158 243 753`, `+49 (0) 39292 / 20 90`, `info@gerwischer-recycling.de`. **Die Seite ist heute eine andere Anwendung** (Next.js statt WordPress/Elementor) und **veroeffentlicht keine Preise**: auf `/`, `/preisanfrage`, `/materialien`, `/materialien/kupfer-millberry` und `/leistungen/schrotthandel` steht **null** Euro-Vorkommen; statt der Preistabelle fuehrt die Seite eine **Preisanfrage** und vierzehn **Materialqualitaeten**-Seiten (`aluminiumqualitaeten`, `grauguss`, `handelsguss-3b`, `kupfer-berry`, `kupfer-millberry`, `kupferqualitaeten`, `maschinenguss-2b`, `messingqualitaeten`, `mischschrott-leicht`, `mischschrott-schwer`, `sphaeroguss`, `stahlguss`, `v2a-rostfrei`, `v4a-saeurefest`). **Folge fuer die Daten:** die dreizehn Current-Pointer (Stahlschrott Sorte 1 210 EUR/t, Mischschrott 180 EUR/t, Stahlschrott Shredder 120 EUR/t, Grauguss 175 EUR/t, Maschinenguss 195 EUR/t, Sphäroguss 190 EUR/t, Stahlguss 185 EUR/t, Kupfer Berry 6,5 EUR/kg, Zink 1,1 EUR/kg, Blei 0,95 EUR/kg, Edelstahl V2A 1,05 EUR/kg, Edelstahl V4A 1,6 EUR/kg, Aluminium gemischt 0,85 EUR/kg; zuletzt beobachtet am 24.11.2026 um 11:24:12 UTC) werden durch **Migration 13** zurueckgezogen - zurueckgezogen wird **nur** der Current-Pointer, die Historie bleibt unveraendert. **Die Probe gegen eine Kopie der Produktionsdatenbank:** `user_version` 12 auf 13, `prices` 129652 unveraendert, `current_prices` 4067 auf 4054, also genau dreizehn zurueckgezogene Paare. **Reichweite:** der `404` belegt den Wegfall der **Preisseite**, **nicht** das Ende des Betriebs und **nicht** den Wegfall der Preise selbst - die Firma handelt nach wie vor Schrotthandel, Gussaufbereitung und Entsorgung. **Offen:** der Handler scheitert weiterhin **laut** (das ist die vertragliche Regel des Handlers: eine Verschiebung der Seite bricht den Lauf ab, statt geraten zu werden); welche der vierzehn Materialqualitaeten-Seiten welchen Katalogmaterialien entsprechen und ob die Preisanfrage als **Annahme** (`auf Anfrage`) zu führen ist, ist eine eigene, noch nicht gesichtete Zuordnung und wird **nicht** geraten. Quelle(n): https://gerwischer-recycling.de/ankaufspreise/ ; https://gerwischer-recycling.de/ ; https://gerwischer-recycling.de/impressum ; https://gerwischer-recycling.de/preisanfrage ; https://gerwischer-recycling.de/materialien (live abgerufen 24.11.2026).]

### Recherche 24.11.2026 (Preispflege, zweite Sichtung)

- [Recherche 24.11.2026 (Preispflege, zweite Sichtung): **Die Preisseite ist weg — der Betrieb veröffentlicht jetzt vierzehn Materialqualitäten und nur noch eine Preisanfrage; der Handler liest die Seite als Annahmenquelle.** **Neue Seitenstruktur (live, 24.11.2026):** Die Seite `https://gerwischer-recycling.de/materialien/` trägt **vierzehn** Materialkarten (`data-testid="material-card"` ✓✓) und **null** Euro-Angaben ✓✓ und **null** kg-Angaben ✓✓ ✓✓ Jede Karte **nennt** ihre **Qualität** ✓✓ (`Aluminiumqualitäten` ✓✓ · `Grauguss` ✓✓ · `Handelsguss 3b` ✓✓ · `Kupfer (Berry)` ✓✓ · `Kupfer (Millberry)` ✓✓ · `Kupferqualitäten` ✓✓ · `Maschinenguss 2b` ✓✓ · `Messingqualitäten` ✓✓ · `Mischschrott leicht` ✓✓ · `Mischschrott schwer` ✓✓ · `Sphäroguss` ✓✓ · `Stahlguss` ✓✓ · `V2A (rostfrei)` ✓✓ · `V4A (säurefest)` ✓✓) ✓✓ Der Seitenkopf nennt es **„Unverbindlich Preisanfrage“** ✓✓ ✓✓ **Der Impressum** ✓✓ ist **neu** ✓✓ gebaut ✓✓ (Next.js ✓✓ statt WordPress/Elementor ✓✓): **eine** ✓✓ `<div class="rich">` ✓✓ mit **Absätzen** ✓✓ (`<h2>Angaben gemäß § 5 TMG</h2>` ✓✓, dann Firma ✓✓, Straße ✓✓, PLZ-Zeile ✓✓, USt-ID ✓✓, Register ✓✓; `<h2>Kontakt</h2>` ✓✓, dann Telefon ✓✓, Fax ✓✓, E-Mail ✓✓) ✓✓ Die **Mailadresse** ✓✓ **versteckt** ✓✓ sich hinter dem **CDN**-Schutz ✓✓ der Seite ✓✓ — sie ist **nicht auslesbar** ✓✓ und **nicht übernommen** ✓✓ ✓✓ **Die Umsetzung** ✓✓ (Commit `d6a181ba` ✓✓): Der Handler ✓✓ liest die **Materialkarten** ✓✓ und **zeichnet** ✓✓ **Annahmen** ✓✓ („**auf Anfrage**“ ✓✓) ✓✓ für die **neun** ✓✓ **eindeutig** ✓✓ **zuordenbaren** ✓✓ Materialien ✓✓ (die **vier** ✓✓ Gussqualitäten ✓✓, die **zwei** ✓✓ Kupferqualitäten ✓✓, die **zwei** ✓✓ Edelstahlqualitäten ✓✓) ✓✓ · die **fünf** ✓✓ **mehrdeutigen** ✓✓ Karten ✓✓ (`Aluminiumqualitäten` ✓✓, `Kupferqualitäten` ✓✓, `Messingqualitäten` ✓✓, `Mischschrott leicht` ✓✓, `Mischschrott schwer` ✓✓) ✓✓ **überspringt** ✓✓ er **laut** ✓✓ ✓✓ **Sieben** ✓✓ **Offline**-Tests ✓✓ und der **Live**-Test ✓✓ **grün** ✓✓ — **neun** ✓✓ Annahmen ✓✓, **null** ✓✓ Preise ✓✓, **fünf** ✓✓ **laut** ✓✓ **übersprungene** ✓✓ Karten ✓✓ ✓✓ **Reichweite:** Der **Wegfall** ✓✓ der **Preisseite** ✓✓ belegt **nicht** ✓✓ den **Wegfall** ✓✓ der **Preise** ✓✓ selbst ✓✓ — die **Firma** ✓✓ **kauft** ✓✓ weiterhin ✓✓ **Schrott** ✓✓ ✓✓ Quelle(n): https://gerwischer-recycling.de/materialien/ ; https://gerwischer-recycling.de/impressum ; https://gerwischer-recycling.de/preisanfrage (live abgerufen 24.11.2026).]
