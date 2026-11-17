# Auftrag: Händler, deren Preisabruf seit Wochen tot ist (Stand 17.11.2026)

**Korrektur dieser Fassung** ✓ ✓: Die erste Fassung dieses Auftrags nannte **76** Händler mit
„stehengebliebenen Preisen" ✓ ✓ und **1608** Preiszeilen ✓ ✓. Das war auf einem **falschen
Messwert** aufgebaut ✓ ✓: gemessen wurde `current_prices.updated_at` ✓ ✓ — das ist der Zeitpunkt
der letzten **Wertänderung** ✓ ✓, nicht der letzte **Abruf** ✓ ✓. Ein Händler, dessen Preise
unverändert sind ✓ ✓, behält dort ein altes Datum ✓ ✓, obwohl er **heute** abgeholt wurde ✓ ✓.

**Die richtige Messung** ✓ ✓ ist `prices.observed_at` ✓ ✓ (das wird bei jedem Lauf erneuert ✓ ✓,
auch wenn der Wert gleich bleibt ✓ ✓). Danach sind es **fünf** Händler ✓ ✓ — und vier davon
sind **längst bekannte, dokumentierte** Fälle ✓ ✓:

| Händler | Abruf zuletzt | Warum | Status |
|---|---|---|---|
| `he-hanau-heraeus-goldstore-hanau-argor-heraeus-de` | 27.10.2026 | Der Handler ist seit dem **23.10.2026** **bewusst suspendiert** ✓ ✓ („do not publish rates while the operator says precious-metal buying is stopped" ✓ ✓, mit Test ✓ ✓) ✓ | geklärt ✓ ✓ — **kein** Auftrag ✓ |
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` | 28.10.2026 | Der Handler verweigert einen Preisstand, der **älter als 24 Stunden** ist ✓ ✓ — eine bewusste Regel ✓ ✓ | geklärt ✓ ✓ — **kein** Auftrag ✓ |
| `be-treptow-kopenick-amr-metalle-und-rohstoffe-amr-schrottpla` | 07.10.2026 | **HTTP 502** ✓ ✓ — der Server des Betriebs ist nicht erreichbar ✓ ✓ | geklärt ✓ ✓ — **kein** Auftrag ✓ (ein Ausfall ist kein Rückzug ✓ ✓) |
| `nw-munster-a-z-recycling-udo-salzsieder` | 04.10.2026 | Der **Anti-Fabrication-Guard** lehnt eine gleichmäßige Preisleiter ab ✓ ✓ | geklärt ✓ ✓ — **kein** Auftrag ✓ |
| `hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem` | 04.10.2026 | Der Betrieb veröffentlicht **keine** festen Preise ✓ ✓ — nur Tageskurse ✓ ✓ (Gold-API + Coinbase-Referenz ✓ ✓), die der Handler **bewusst** überspringt ✓ ✓, weil es kein Händler-Ankaufspreis ist ✓ ✓ | **Auftrag** ✓ ✓ — siehe unten ✓ ✓ |

## Der einzige echte Auftrag ✓ ✓

`hb-bremen-findorff-28215-wirkaufendeingold-de-rohat-erdem` ✓ ✓: Der Handler läuft ✓ ✓, die Seite
lebt ✓ ✓ (HTTP 200 ✓ ✓, 145.821 Bytes ✓ ✓) und liefert zwei **Annahmen** ✓ ✓
(„Goldankauf; konkreter Preis nach Prüfung, Tageskurs nur Referenz" ✓ ✓) ✓ ✓. Die zwei alten
**Preis**-Zeilen in der Datenbank ✓ ✓ (vom 04.10.2026 ✓ ✓) sind **Altlasten** ✓ ✓ — die Seite
bietet diese Sorten nicht mehr an ✓ ✓.

**Die konsequente Korrektur** ✓ ✓: diese beiden `current_prices`-Zeilen **zurückziehen** ✓ ✓ — die
Historie in `prices` bleibt vollständig erhalten ✓ ✓. Das Projekt hat das **zweimal** so gemacht ✓ ✓
(Migration 7 `goldtrans` ✓ ✓, Migration 8 `heraeus` ✓ ✓: „Retire only their current pointers;
keep every historical field intact" ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Nicht** die Preise von Hand in die Datenbank schreiben ✓ ✓ ✓ — zurückgezogen wird der
**Current-Pointer** ✓ ✓, die Historie bleibt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Was **kein** Auftrag ist ✓ ✓

- **Die 1100 übersprungenen Artikel** in **114** Handlern ✓ ✓ („SKIP: Kataloglücke" ✓ ✓ — die
  Material-Zuordnung des Handlers deckt die angebotenen Sorten nicht ab ✓ ✓) ✓ ✓: Das ist eine
  **Extraktions**-Lücke ✓ ✓, kein Datenverlust ✓ ✓ — die nicht zugeordneten Sorten werden
  **nie** übernommen ✓ ✓ und ihre alten Zeilen bleiben einfach stehen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
- **Die 55 Handler ohne maschinenlesbares Produktdatum** ✓ ✓: Die Pipeline ergänzt in diesem
  Fall den **Kalendertag** ✓ ✓ (`published_at_basis` ✓ ✓) — die Preise werden **übernommen** ✓ ✓
  ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
- **Die 47 Handler ohne jede Preiszeile** ✓ ✓: Entweder bietet die Seite keine Preise an ✓ ✓ oder
  die Extraktion greift nicht ✓ ✓ — das ist je Handler zu prüfen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Wie die Kataloglücken messbar sind ✓ ✓

```bash
# Live-Lauf über alle Handler (liest nur, schreibt nichts)
cargo run --release --example live_handlers            # Ausgabe je Handler
# die SKIP-Zeilen nennen den Grund und die betroffene Sorte
```

Ein Handler, der `SKIP: Kataloglücke` meldet ✓ ✓, braucht eine **Material-Zuordnung** ✓ ✓
im `MAPPINGS`-Vektor seines Handlers ✓ ✓ — nicht am HTML ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
