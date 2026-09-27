# Guide: einen neuen Händler-Handler bauen

Zielgruppe: Subagents, die Preis-Handler für weitere Händler schreiben.
Ein Handler ist **eine Datei** in `crates/ingestion/src/traders/handlers/`
plus **eine Zeile** in `handlers::all()`. Lies zuerst `traders/mod.rs`
(Skalierungskonzept) und einen bestehenden Handler mit ähnlicher
Seitenform (Tabelle → `vedder.rs`, Karten/Paare → `lausitz.rs`,
Homepage-Block → `tappe.rs`, „bis zu"-Karten → `kupferhelden.rs`,
Kategorie-Übersicht → `metallankauf24.rs`).

## Arbeitsablauf (in dieser Reihenfolge)

1. **Recon:** Seite per `curl`/webfetch holen, **rohes HTML** ansehen
   (nicht das gerenderte Markdown — Parser sehen HTML). Finde: Wo genau
   stehen Preise? Tabelle, Karten, Fließtext? Gibt es ein Datum? Steht
   die Einheit pro Zeile, im Header oder gar nicht? Gibt es Störstellen
   (Footer mit €-Beträgen, Cookie-Banner, doppelte Blöcke)?
2. **Slug prüfen:** `SELECT slug FROM traders WHERE slug LIKE '%name%'`
   — der Handler-`SLUG` muss exakt einem `traders.slug` entsprechen,
   sonst schlägt der Step laut fehl (so gewollt).
3. **Fixture-Test zuerst:** Realen HTML-Ausschnitt als `FIXTURE`-Konstante
   in den Test übernehmen, `parse()` + `grade_for()` testen. Erst wenn
   der Test das tut, was die Seite zeigt, `scrape()` verdrahten.
4. **Live prüfen:** `cargo run -p schrott-mcp-ingestion --example
   live_handlers <slug>` — schreibt nichts in die DB. Zeilenzahl mit der
   Seite vergleichen, jeden Skip rechtfertigen.
5. **Deploy-Verifikation:** Nach Deploy per Dashboard-Trigger forcen,
   Step-Zeile + `v_current_prices` per MCP abfragen.

## Mapping-Regeln (Material + Variante)

- `grade_for()` ist eine **explizite Match-Tabelle**, kein Fuzzy-Matching.
  Jede neue Händlerbezeichnung landet bewusst auf einem Material — oder
  auf `None` (laut geskippt, gezählt, im Step-Detail sichtbar).
- **Niemals raten:** Mehrdeutige Kategorien (`"Kabel / E-Motoren"`,
  `"Messing / Rotguss"` als Maximum) → `None`. Generische Labels
  (`"Aluminium"`, `"Edelstahl"`, `"Kupferschrott 2"`) → generische
  Katalogmaterialien (`aluminium-gemischt` …), nie auf eine spezifische
  Sorte. Fehlendes Katalogmaterial → `None` (Katalog erweitern ist ein
  separater, bewusster Schritt).
- **Variante ist Pflicht-Denken:** Zwei Sorten desselben Händlers zum
  selben Material mit verschiedenen Preisen (`"Zinn 80-98%"` vs.
  `"Zinn 50-59%"`, `"Messing (große/kleine Teile)"`) brauchen
  verschiedene `variant`-Werte — sonst kollabieren sie auf einen
  willkürlichen „aktuellen Preis“. `''` heißt Standardsorte.
- **Rohlabel immer in `notes` behalten** (Nachvollziehbarkeit).

## Parsing-Regeln (alle aus echten Fehlern)

- **Gefenstert parsen, nie die ganze Seite.** Anker suchen (Überschrift
  wie `"Unsere Preise"`, Tabellenkopf `"Materialbezeichnung"`), Start
  UND Ende begrenzen. Footer-`€`-Beträge paaren sich sonst mit
  irgendwelchen Labels zu Phantompreisen.
- **Nie die erste Tabelle/Liste nehmen.** Tabelle am Kopfinhalt wählen;
  fehlt sie → `Err` (laut), nicht leere Erfolgsmeldung.
- **Leere Ergebnisse sind Fehler:** `parse()` mit 0 Zeilen gibt
  `IngestError::Parse` zurück. Stille Erfolge mit 0 Zeilen verstecken
  Redesigns.
- **Einheiten nie still defaulten** (ein Tonnenpreis als Kilo ist ein
  1000-facher Fehler): pro Zeile parsen; was der Parser nicht kennt,
  wird geskippt (`"... (Einheit unverständlich: ...)"`). Seiten-globale
  Einheit nur als **dokumentierte, begründete** Konstante (Marktgrößen-
  plausibilisiert wie bei Metallankauf24) — und eine explizit-fremde
  Einheit (`"pro Sack"`) skippt trotzdem (`has_unit_markers`).
- **Geteilte Helfer benutzen:** `fetch_text` (HTTP + Statusprüfung),
  `parse_eur` (deutsches Format inkl. Tausenderpunkt), `eur_unit`
  (token-basiert — `"EUR / T"` mit Leerzeichen!), `parse_de_date`.
  Keine eigenen Regex-Suppen dafür.
- **Doppelte Blöcke deduplizieren** (Lausitz-`"Gültig ab"`-Repeat) per
  `(Material, Variante, Preis)` — aber erst *nach* dem Mapping, nicht
  auf Rohlabels (Schreibvarianten!).
- **Beschriftungs-Puffer zurücksetzen** an Headers/Terminatoren; ein
  Terminator (`"... auf Anfrage"`) beendet die Box. `€`-Text ohne
  Ziffern ist ein Header, kein Label (klebt sonst an der nächsten Sorte).

## Unsicherheit & Provenienz (explizit modellieren, nie wegmitteln)

- Exakte Listenpreise: `confidence: Some(1.0)`, `price_kind: "exact"`.
- `"bis zu"`: `price = price_max = beworbener Wert`, `confidence: 0.5`,
  `price_kind: "upto"`. Niemals `price_max = price` ohne Kind.
- Seitendatum → `published_at` (pro Material wenn vorhanden, sonst
  Seiten-Fallback); kein Datum → `None` (Alter = `observed_at`).
- `source_url` ist immer die **Preisseite**, nie die Homepage.
- Normierung nur bei exakt beweisbarer Umrechnung (kg↔t in
  Katalogeinheit); alles andere bleibt wie zitiert.

## Scheduler-Integration

- `schedule`: Standard `Schedule::every_6h()` (Hash-Stagger übernimmt).
  Nur bei belegtem Tagesrhythmus `DailyAt` mit Berlin-Zeiten.
- Der Rest (Timeout 120 s, Steps, Fetch-Journal inkl. Fehler, Canary,
  Acceptance-Ableitung) passiert von selbst in `run_due_with`/`record()`.
  Handler kümmern sich nur um Fetch+Parse+Mapping.

## Verifikations-Checkliste (vor „fertig")

- [ ] Zeilenzahl == Seite (abzüglich begründeter Skips)?
- [ ] Jeder Skip im Step-Detail nachvollziehbar (Material-Lücke,
        mehrdeutig, Einheit)?
- [ ] Varianten trennen alle Sorten (kein Key doppelt)?
- [ ] Einheiten plausibel (kg-Preise einstellig–zweistellig,
        t-Preise dreistellig)?
- [ ] `published_at` gesetzt wo die Seite eins nennt?
- [ ] Force-Run: Step `ok`, keine Canary-Warnung, MCP-Query zeigt Zeilen?
- [ ] `cargo test --workspace` grün?

## Galerie realer Fehler (bitte nicht wiederholen)

| Fehler | Schaden | Heute |
|---|---|---|
| `Zinn …`→Alu durch Arm-Reihenfolge | falsches Material | Tests pro Label |
| `unwrap_or("EUR/kg")` | 1000×-Risiko | lautes Skippen |
| `"1.100"`→1.1 | 1000×-Fehler | Tausender-Regel + Test |
| Erste `<table>` genommen | falsche Tabelle | Kopf-Selektion |
| Seitenweiter Text-Walk | Phantompreise | Fenster |
| Sorten auf ein Material | willkürlicher Current-Preis | `variant` |
| `price_max = price` ohne Kind | mehrdeutige Semantik | `price_kind` |
| `Entered`-Span über `.await` | kompiliert nicht (nicht `Send`) | Felder statt Spans |
| Slugs von Hand umbenannt | Duplikat-Orphans | Slugs sind stabil |
| Converter löscht Anreicherung | Datenverlust | Preserve-Keys |
| Migration vs. neue Semantik | Backfill-Clobber | `user_version`-Gate |
| `cp`-Backup bei WAL | leere Backups | `sqlite3 .backup` |
