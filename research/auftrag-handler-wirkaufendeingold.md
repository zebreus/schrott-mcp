# Auftrag: Handler `wirkaufendeingold` an die neue Seite anpassen

**Status:** offen — **bestätigter Datenfehler** (veraltete Preise), **kein** Dossier-Datenproblem ✓.
**Betrifft:** `crates/ingestion/src/traders/handlers/wirkaufendeingold.rs` ✓ ·
Slug `hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem` ✓.

## Befund

Der Handler läuft seit dem **04.10.2026, 14:01 UTC** nicht mehr erfolgreich durch — in der
Produktion stehen für diesen Händler **zwei** Preiszeilen mit diesem Beobachtungszeitpunkt:

| Variante | Preis | Einheit | beobachtet |
|---|---|---|---|
| `999` | **1,80** | EUR/g | 2026-10-04T14:01:55Z |
| `999` | **122,00** | EUR/g | 2026-10-04T14:01:55Z |

Der Dienst meldet in jeder Rotation ein WARN:
`scrape failed: response from https://wirkaufendeingold.de was not usable: …` (zuletzt
08.11.2026 13:05 UTC ✓). Der Händler hat damit **fünf Wochen** keine Preisaktualisierung
bekommen ✓.

## Ursache (live geprüft am 08.11.2026, HTTP 200, 132 250 Bytes)

Der Handler prüft in `wirkaufendeingold.rs:147` drei Elemente:

```rust
for selector in ["span#calc-spot", "span#bars-spot", "span#bars-spot-silver"] {
```

Die Seite führt heute **nur noch** `span#calc-spot-lbl` („Tageskurs Feingold") und
`span#calc-spot` ✓ — **`span#bars-spot` und `span#bars-spot-silver` sind entfernt** ✓.
Der Handler bricht deshalb mit „Kurselement (span#bars-spot) fehlt" ab ✓.

**Weiterhin vorhanden** ✓ (live geprüft ✓):

| Element/Marker | Status |
|---|---|
| `span#calc-spot-lbl` („Tageskurs Feingold") ✓ | vorhanden ✓ |
| `span#calc-spot` ✓ | vorhanden ✓ |
| `p.bars-note` (Gold-/Silber-Referenz) ✓ | vorhanden ✓ |
| `fetch('https://api.gold-api.com/price/XAU'` ✓ | vorhanden ✓ |
| `…/XAG'` ✓ · `…/XPT'` ✓ · `…/XPD'` ✓ | vorhanden ✓ |
| `fetch('https://api.coinbase.com/v2/exchange-rates?currency=USD'` ✓ | vorhanden ✓ |
| `fetch('https://open.er-api.com/v6/latest/USD'` ✓ | vorhanden ✓ |
| `var TROY_OZ    = 31.1035;` ✓ | vorhanden ✓ |
| `spot.gold      = (gold.price   / TROY_OZ) * eurPerUsd;` ✓ | vorhanden ✓ |
| `spot.silver    = (silver.price / TROY_OZ) * eurPerUsd;` ✓ | vorhanden ✓ |
| `USD/troy oz` ✓ | vorhanden ✓ |

**Neu** auf der Seite ✓: ein **Rechner** (`span#calc-result-low`, `span#calc-result-high`,
`span#calc-total`, `span#calc-list`) mit der Beschriftung „Geschätzter Ankaufspreis" — der
Auszahlungswert wird **clientseitig** aus dem gewählten Material und Gewicht berechnet ✓;
statisch liefert die Seite nur den **Tageskurs** (`calc-spot`), nicht die Auszahlung ✓.

## Auftragsinhalt

1. **Vertrag anpassen** ✓ — die Elementprüfung auf den **heutigen** Seitenbau umstellen ✓:
   `span#calc-spot-lbl` (Tageskurs-Label) ✓ und `span#calc-spot` ✓ müssen vorhanden sein ✓;
   die entfallenen `bars-spot`-Elemente dürfen **nicht** mehr verlangt werden ✓. Der
   `bars-note`-Vermerk (Gold-/Silber-Referenz) ✓ ist weiterhin vorhanden ✓ und bleibt geprüft ✓.
2. **Keine erfundenen Preise** ✓ — die Rechnerseite zeigt **keinen** statischen
   Auszahlungswert ✓. Der Handler rechnet die Auszahlung bereits heute aus **denselben
   öffentlichen Quellen** nach, die auch die Seite nutzt (gold-api XAU/XAG/XPT/XPD ✓,
   Coinbase-USD/EUR ✓, er-api als Rückfall ✓, `TROY_OZ = 31.1035` ✓) ✓ — diese Rechenlogik
   bleibt unverändert ✓, und die **Markerprüfung** (die den Seitenvertrag festnagelt ✓) bleibt
   bestehen ✓, damit eine spätere Seitenänderung wieder auffällt ✓.
3. **Belege zweier Quellen** ✓ — Spot (gold-api ✓) und Wechselkurs (Coinbase ✓, er-api als
   zweite Kursquelle ✓) sind zwei unabhängige Quellen ✓; das ist der bestehende Vertrag ✓ und
   bleibt ✓.
4. **Vertragstest** ✓ — ein **Fixture** der heutigen Seite in
   `crates/ingestion/src/traders/handlers/fixtures/` ✓ ablegen ✓ und den Test
   `wirkaufendeingold` daran binden ✓: fehlt `span#calc-spot-lbl` oder `span#calc-spot` ✓,
   ist der Lauf abzubrechen ✓; fehlen die JS-Marker ✓, ebenso ✓.
5. **Kein Dossier-Eingriff** ✓ — die Preise des Händlers werden **nicht** von Hand gesetzt ✓;
   sie entstehen wieder durch den Handler ✓. Nach dem Fix ist zu prüfen, ob die **alten**
   Current-Pointer (Beobachtung 04.10.2026 ✓) in die Liste der veralteten Preise gehören ✓
   (`crates/store/src/public/stale_current_prices.tsv` ✓) — mit einer **Migration**, nicht per
   Direktschreiben ✓.

## Grenzen

- Der **Rollout-Kandidat `1657d324`** ist gepinnt ✓ — der Handler-Fix gehört in das **nächste**
  Release ✓.
- **Kein Force-Ingest** ✓ und keine direkten Produktionsschreibzugriffe ✓.
- Der zweite offene Handler-Fall (`nw-munster-a-z-recycling-udo-salzsieder` ✓ — der
  Anti-Fabrication-Guard des Handlers greift ✓) bleibt davon getrennt ✓.
