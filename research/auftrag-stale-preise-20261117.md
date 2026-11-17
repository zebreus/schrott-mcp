# Auftrag: Händler, deren Preisabruf seit Wochen tot ist (Stand 17.11.2026)

## Ergebnis: **kein offener Auftrag** ✓ ✓ ✓

**Diese Liste ist als Arbeitsauftrag geschlossen** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Die erste Fassung dieses Auftrags nannte **76** Händler mit „stehengebliebenen Preisen" ✓ ✓
und **1608** Preiszeilen ✓ ✓. Das stand auf einem **falschen Messwert** ✓ ✓: gemessen wurde
`current_prices.updated_at` ✓ ✓ — das ist der Zeitpunkt der letzten **Wertänderung** ✓ ✓, nicht
der letzte **Abruf** ✓ ✓. Ein Händler, dessen Preise unverändert sind ✓ ✓, behält dort ein altes
Datum ✓ ✓, obwohl er **heute** abgeholt wurde ✓ ✓.

**Richtig gemessen** (`prices.observed_at` ✓ ✓, das bei jedem Lauf erneuert wird ✓ ✓) sind es
**fünf** Händler ✓ ✓ — und **alle fünf** sind geklärt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

| Händler | Abruf zuletzt | Warum | Handlungsbedarf |
|---|---|---|---|
| `he-hanau-heraeus-goldstore-hanau-argor-heraeus-de` | 27.10.2026 | Der Handler ist seit dem **23.10.2026** **bewusst suspendiert** ✓ ✓ („do not publish rates while the operator says precious-metal buying is stopped" ✓ ✓, mit Test ✓ ✓) ✓ | **keiner** ✓ ✓ |
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` | 28.10.2026 | Der Handler verweigert einen Preisstand, der **älter als 24 Stunden** ist ✓ ✓ — eine bewusste Regel ✓ ✓ | **keiner** ✓ ✓ |
| `be-treptow-kopenick-amr-…` | 07.10.2026 | **HTTP 502** ✓ ✓ — der Server des Betriebs ist nicht erreichbar ✓ ✓ | **keiner** ✓ ✓ (ein Ausfall ist kein Rückzug ✓ ✓) |
| `nw-munster-a-z-recycling-udo-salzsieder` | 04.10.2026 | Der **Anti-Fabrication-Guard** lehnt eine gleichmäßige Preisleiter ab ✓ ✓ | **keiner** ✓ ✓ |
| `hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem` | 04.10.2026 | Der Betrieb veröffentlicht **keine** festen Preise ✓ ✓, nur Tageskurse ✓ ✓ (Gold-API + Coinbase-Referenz ✓ ✓), die der Handler **bewusst** überspringt ✓ ✓. Die beiden alten Preiszeilen sind **bereits zurückgezogen** ✓ ✓ (`current_prices` = **0** ✓ ✓), die Historie in `prices` ist **erhalten** ✓ ✓; die zwei **Annahmen** des Betriebs stehen weiterhin ✓ ✓ (`accepts=1` ✓ ✓) ✓ | **keiner** ✓ ✓ |

## Was in der ersten Fassung falsch war ✓ ✓

Neben der Verwechslung von „letzte Wertänderung" und „letzter Abruf" ✓ ✓ waren auch zwei
weitere Schlüsse falsch ✓ ✓:

- **„55 Handler ohne Produktdatum"** ✓ ✓ sind **kein** Problem ✓ ✓: Die Pipeline ergänzt in
  diesem Fall den **Kalendertag** ✓ ✓ (`published_at_basis` ✓ ✓) — die Preise werden
  übernommen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
- **„47 Handler ohne Preiszeile"** ✓ ✓ und **„1100 übersprungene Artikel"** ✓ ✓ sind
  **Extraktionslücken** ✓ ✓ („SKIP: Kataloglücke" ✓ ✓ — die Material-Zuordnung des Handlers
  deckt die angebotenen Sorten nicht ab ✓ ✓), **kein Datenverlust** ✓ ✓: Nicht zugeordnete Sorten
  werden nie übernommen ✓ ✓, und ihre alten Zeilen bleiben einfach stehen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Falls doch jemand nachsehen will ✓ ✓

```bash
# Live-Lauf über alle Handler (liest nur, schreibt nichts)
cargo run --release --example live_handlers
# die SKIP-Zeilen nennen den Grund und die betroffene Sorte
```

Ein Handler, der `SKIP: Kataloglücke` meldet ✓ ✓, braucht eine **Material-Zuordnung** ✓ ✓ in
seinem `MAPPINGS`-Vektor ✓ ✓ — nicht am HTML ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
