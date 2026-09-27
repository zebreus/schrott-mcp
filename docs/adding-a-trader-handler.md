# Guide: einen neuen Händler-Handler bauen

Für Subagents, die Preis-Handler schreiben. Ein Handler ist **eine Datei** in `crates/ingestion/src/traders/handlers/` plus **eine Zeile** in `handlers::all()`. Erst `traders/mod.rs` (Konzept) und einen Handler ähnlicher Seitenform lesen (Tabelle → `vedder.rs`, Karten/Paare → `lausitz.rs`, Homepage-Block → `tappe.rs`, „bis zu"-Karten → `kupferhelden.rs`, Kategorie-Übersicht → `metallankauf24.rs`, Listen ohne Preise → `esh.rs`/`quell.rs`). Keine spekulativen Felder/Helfer: was kein Handler braucht, existiert nicht (Pro-Material-Daten und Unit-Katalog sind genau deshalb wieder rausgeflogen).

## Arbeitsablauf

1. **Recon:** rohes HTML ansehen (Parser sehen HTML, kein Markdown). Wo stehen Preise? Tabelle, Karten, Fließtext? Datum? Einheit pro Zeile, im Header oder gar nicht? Störstellen (Footer-€, Cookie-Banner, Doppelblöcke)?
2. **Slug prüfen:** muss exakt einem `traders.slug` entsprechen, sonst Step-Fehler (so gewollt).
3. **Fixture-Test zuerst:** *realen* HTML-Ausschnitt als `FIXTURE` übernehmen — vereinfachte Nachbauten geben falsche Sicherheit (live stand die Adresse in `<h1>`, nicht `<p>`). `parse()` + `grade_for()` testen, dann `scrape()` verdrahten.
4. **Live prüfen:** `cargo run -p schrott-mcp-ingestion --example live_handlers <slug>` (schreibt nichts in die DB). Zeilenzahl vs. Seite, jeden Skip rechtfertigen.
5. **Deploy-Verifikation:** forcen (Force-Runs hängen *by design* Duplikate an), Step-Zeile + `v_current_prices` per MCP prüfen.

## Mapping (`grade_for`: explizite Tabelle, kein Fuzzy)

- Jede Bezeichnung landet bewusst auf einem Material — oder `None` (laut geskippt, gezählt, im Step-Detail). Mehrdeutiges (`"Kabel / E-Motoren"`, `"Messing / Rotguss"` als Maximum) → `None`. Generisches (`"Aluminium"`, `"Kupferschrott 2"`) → generisches Material, nie spezifische Sorte. Fehlendes Katalogmaterial → `None` (Erweiterung = separater Schritt).
- **Arme spezifisch-vor-generisch ordnen** (`"Kupfer"` fängt sonst `"Kupferschrott 1 ECU"`), pro Label testen.
- **Variante mitdenken:** zwei Sorten, ein Material, zwei Preise → zwei `variant`-Werte, sonst kollabieren sie auf einen willkürlichen Current-Preis. `''` = Standardsorte.
- Rohlabel immer in `notes` (Nachvollziehbarkeit); Fan-out wo nötig (`"V2A und V4A"` → zwei Materialien).

## Parsing (alles aus echten Fehlern)

- **Fenster, nie Ganzseite:** Anker für Start UND Ende (`"Unsere Preise"`, Tabellenkopf). Footer-€ paart sich sonst mit Labels zu Phantompreisen.
- **Tabelle/Liste am Kopfinhalt wählen** (`"Materialbezeichnung"`), nie die erste. Fehlt sie → `Err`, kein leerer Erfolg. **0 Zeilen = `Err`** (stille Erfolge verstecken Redesigns).
- **`<script>`/`<style>`/JSON-LD nie als Text lesen** — nur Content-Elemente selektieren, sonst kleben Skript-Fetzen Adressen zusammen (`"Boden 2365795"`).
- **Units maßgeschneidert:** eigenes `unit_of` pro Handler, kennt genau die Schreibweisen dieser Seite (`"EUR / KG"`, `"x pro to"`, `"€/KG"` — nur kg/t). Unbekannt → Skip `"... (Einheit unverständlich: ...)"`. Seiten-Default nur als dokumentierte, markt-plausibilisierte Konstante; explizit-fremd (`"pro Sack"`, erkennbar an `/`/`"pro"`) skippt trotzdem. Tonne-als-Kilo = 1000×-Fehler.
- **Geteilt nur:** `fetch_text`, `parse_eur` (inkl. Tausenderpunkt), `parse_de_date`. Kein neuer Shared-Parser; Seitenspezifisches (Units, Datums-*Finden*, Kontakt) als kleine Funktion in den Handler.
- **Doppelblöcke** (`"Gültig ab"`-Repeat) per `(Material, Variante, Preis)` dedupen — nach dem Mapping, nicht auf Rohlabels.
- **Puffer disziplinieren:** an Headers/Terminatoren zurücksetzen; Terminator (`"... auf Anfrage"`) beendet die Box. `€`-Text ohne Ziffern = Header, kein Label; Texte >120 Zeichen = Prosa, kein Label.

## Unsicherheit & Provenienz

- Exakt: `confidence: Some(1.0)`, `price_kind: "exact"`. `"bis zu"`: `price = price_max = beworben`, `confidence: 0.5`, `price_kind: "upto"` — nie `price_max` ohne Kind.
- Seitendatum → Outcome-`published_at` (ein Datum pro Seite; Pro-Material-Daten gibt es nicht); kein Datum → `None` (`observed_at` = Alter).
- `source_url` = Preisseite, nie Homepage. Normierung nur bei beweisbarer Umrechnung (kg↔t in Katalogeinheit).

## Scheduler

Standard `every_6h()` (Hash-Stagger); `DailyAt` mit Berlin-Zeiten nur bei belegtem Tagesrhythmus. Rest (Timeout, Steps, Journal, Canary, Acceptance) passiert in `run_due_with`/`record()` — Handler: nur Fetch+Parse+Mapping.

## Impressum (bespoke!)

- URL pro Handler hartkodiert, live verifiziert — nie raten/teilen. Umzug → lauter Step-Fehler.
- `extract_info` pro Handler, Anker der echten Seite (Vedder-`<dl>`, Lausitz-`data-bind`, Tappe-`<dl>`, Kupferhelden-`<p>`+`<h2>Kontakt</h2>`, M24-`div.inhalt`, ESH-`Inhaber:`, Quell-`<h1>`). Fehlende Anker → `Parse`-Error, nie raten/fallback. Infos über mehrere Seiten → pro Seite eigene URL + eigener Block, kein Crawler.
- **Gotchas:** `<br`-Split hinterlässt Tag-Reste (`class="…"` parst als Text) → erst alles bis zum ersten `>` verwerfen. Sibling-Walk findet bei wildem Nesting nichts → dokumentweit suchen, aber Anker-Heading als Muss. E-Mail braucht eigene Regel (Telefon-Tokenfilter stoppt am ersten Buchstaben). Seiten-Macken (∂-Mails, Hex-mailto, `"Spähne"`, geklebte PLZ) als tolerierte Varianten **mit Test** im Handler.
- `set_trader_info` schreibt nur echte Änderungen (Stadt nur in leere Zellen — Seed-Stadtteile sind präziser), loggt alt→neu.
- Listen ohne Preise → `ScrapedAcceptance` (Mapping wie Preise). Handler ohne Preise sind normal (Canary nur bei 0 Preisen UND 0 Annahmen).

## Checkliste (vor „fertig")

- [ ] Zeilenzahl == Seite minus begründete Skips; jeder Skip nachvollziehbar?
- [ ] Varianten trennen alle Sorten? Einheiten plausibel (kg 1–2-stellig, t 3-stellig)?
- [ ] `published_at` wo die Seite eins nennt? Force-Run ok, keine Canary, MCP zeigt Zeilen?
- [ ] `cargo test --workspace` grün?

## Galerie (nicht wiederholen)

| Fehler | Heute |
|---|---|
| Generischer Arm fängt spezifisches Label | spezifisch zuerst + Tests pro Label |
| `unwrap_or("EUR/kg")` | lautes Skippen |
| `"1.100"` → 1.1 | Tausender-Regel + Test |
| Erste `<table>` | Kopf-Selektion |
| Ganzseiten-Walk | Fenster (Start+Ende) |
| Skript als Text | nur Content-Elemente |
| Vereinfachte Fixture | realer HTML-Ausschnitt |
| Sorten kollabiert | `variant` |
| `price_max` ohne Kind | `price_kind` |
