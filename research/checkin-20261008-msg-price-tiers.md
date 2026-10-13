# MSG Gotha — Feedback #5558, Prüfung 08.10.2026

## Ergebnis und Grenzen

**Kernmeldung bestätigt:** Der aktuelle Handler verwirft bepreiste höhere Mengenstaffeln absichtlich. Produktion hat 46 Current-Zeilen ohne Mengenvarianten, während der heute ausgeführte Handler 45 Preiszeilen liefert. Für seine 45 aktuell katalogisierten Sorten enthält die Quelle **124 sichtbare positive Staffelpreise: 79 fehlen im Output**. Kein Codefix, Deployment, Feedbackabschluss oder DB-Schreibzugriff vorgenommen.

Die Pauschalisierungen des Feedbacks sind zu korrigieren: nicht jede Sorte hat drei bepreiste Staffeln; die t-Beschriftungen sind aktuell ausgeblendete CMS-Alternativen; „bis zu 11 %“ ist kein allgemeines Maximum. Beispielsweise Alu Späne 0,90 → 1,35 EUR/kg entspricht +50 %, Zinn Geschirr 17 → 20,50 +20,59 %. Rotguss Stücke fällt sogar von 7,20 auf 6,50/6,70; keine Monotonie erzwingen.

## Quellen und Betreiberzuordnung

- [Preisliste](https://www.msg-metallrecycling.de/preisliste), live HTTP 200; Handler: 238280 Bytes. Betitelt „Preise Metallschrott-Ankauf in Gotha / Thüringen“; Anlieferung ausdrücklich Gleichenstraße 36, 99867 Gotha.
- [Impressum](https://www.msg-metallrecycling.de/legal/impressum), live: **MSG Recycling, Inhaber Helge Göbbel**, Gleichenstraße 36, 99867 Gotha, Telefon +49 (0) 3621 3394 851, info@msg-metallrecycling.de. Adresse, Kontakt und Geschäftstätigkeit stimmen mit Dossier und Produktionshändler id 2223 überein. Keine Anzeichen einer Betreiberverwechslung; allerdings Betreiber-Einzelbeleg, keine zwei unabhängigen Quellen.
- Betreiberseite nennt Mo–Fr 8–16 Uhr, Sa/So geschlossen; Dossierzeiten kongruent.
- Feedback #5558 read-only aus `internal.db`: medium, erstellt `2026-10-08T19:16:08Z`; Meldung nennt konkrete Staffelbeispiele und fordert wechselnde Produkte/Staffeln vollständig zu berücksichtigen.

## Einheit, Sichtbarkeit und Bedingungen

Die Seite erklärt ausdrücklich: „freibleibende **Preise pro Kilogramm**“; nur bei „Anlieferung und Abgabe direkt bei MSG Recycling“. Börsenkurs, Abnehmerpreise, Beschaffenheit und Menge können die Vergütung verändern. Aktualisierung üblicherweise montags, bei Schwankungen öfter. „Tagesaktuell“ belegt kein konkretes Veröffentlichungsdatum; Handler `published_at=None` ist dafür korrekt.

HTML bei Alu Bleche blank:

```html
<div class="pricedisplay">
  <div class="text-size-medium">ab 1 kg:</div>
  <div class="text-size-medium w-condition-invisible">ab 1 t:</div>
  <div class="text-size-medium">1.30</div>
  <div class="text-size-medium">€</div>
</div>
```

Alle heute sichtbaren Schwellen sind kg, keine sichtbare t-Schwelle gefunden. Textkonvertierung ohne CSS-Sichtbarkeitsprüfung zeigt beide und ist irreführend. Bei Bremsscheiben ist die gesamte leere `ab 1 kg`-Zeile `w-condition-invisible`; erste echte Staffel ab 100 kg. Das sind **Anlieferungsmengen**, unabhängig von der Preiseinheit. Produktion normalisiert FE korrekt auf EUR/t (z.B. 0,19 EUR/kg → 190 EUR/t), aber die Mengenbedingung darf dadurch nicht von 100 kg zu 100 t werden.

Sondermetalle Indium/Iridium/Kobalt/Molybdän/Niob/Tantal enthalten jeweils drei `10.00`-CMS-Werte in **komplett ausgeblendeten** `pricedisplay`-Elementen; sichtbar ist „Preis auf Anfrage“. Nicht als echte aktuelle Kursangebote aufnehmen. Handler ignoriert deren Sichtbarkeit, verwirft sie derzeit wegen fehlendem Materialmapping. Deshalb sind seine „94 höhere Staffeln übersprungen“ **nicht** gleichbedeutend mit 94 fehlenden gültigen Katalogpreisen.

Live-HTML-Inventur im Bereich ab `preiseSchrott` bis Footer: 66 Produktkarten; sichtbare positive Preise je Karte: 38×3, 6×2, 7×1, 15×0 = 133. Davon 124 auf den 45 vom Handler tatsächlich ausgegebenen Sorten. Rest sind aktuell unmapped Mischmaterial-/Besteckangebote; nicht mit Staffelverlust vermischen.

## Konkreter Vergleich

Alle Quellwerte EUR/kg; „—“ bedeutet leer, nicht null Euro.

| Sorte | ab 1 kg | ab 100 kg | ab 1000 kg | aktueller Handler / DB |
|---|---:|---:|---:|---|
| Millberry | 9,90 | 10,10 | 11,00 | 9,90, Variante `Millberry` |
| Kupfer Schwer | 9,30 | 9,50 | 9,70 | 9,30, `Schwer` |
| Kabel 80% Kupfer | 7,20 | 7,40 | 7,50 | 7,20, `80%` |
| V4A Edelstahl | 1,40 | 1,60 | 1,80 | 1,40, leere Variante |
| Hartmetall Stückschrott | 45,00 | 50,00 | 55,00 | 45,00, `Stückschrott` |
| Zinn Geschirr | 17,00 | 19,00 | 20,50 | 17,00, `Geschirr` |
| Alu Späne | 0,90 | 1,05 | 1,35 | 0,90, `Späne` |
| Rotguss Stücke | 7,20 | 6,50 | 6,70 | 7,20, `Rotguss Stücke` |
| Bremsscheiben | — | 0,19 | 0,21 | Handler 0,19 EUR/kg; DB 190 EUR/t, `Bremsscheiben` |
| Bronze Schrott | 5,30 | — | — | 5,30; keine höheren Preise erfinden |

## Produktionsbefund (ausschließlich read-only)

SQLite geöffnet mit `file:/var/lib/schrott-mcp/{public,internal}.db?mode=ro`, zusätzlich `PRAGMA query_only=ON`.

```sql
SELECT COUNT(*), SUM(variant LIKE '%ab %')
FROM v_current_prices
WHERE trader_slug='th-gotha-msg-metallrecycling-msg-schrott-und-meta';
-- 46 | 0
```

Letzter produktiver Schritt #4735, run #1146: `ok`, 45 Preise übernommen; `2026-10-08T16:20:28.955846143+00:00` bis `16:20:29.927380565+00:00`. Vorheriger Schritt #4633 ebenfalls 45. Reguläre aktuellen Beobachtungen `16:20:29.848127453+00:00`.

**Zusätzliche reproduzierbare Altlast:** `messing / Wasseruhren / 3,00 EUR/kg` von `2026-09-29T18:45:21.638632402+00:00` bleibt Current, obwohl der heutige Handler ausschließlich `messing-leicht / Wasseruhren / 3,00` ausgibt. Daher 46 DB-Current-Zeilen versus 45 Livepreise; nicht eine zusätzliche aktuelle Sorte. Bei Variantenmigration/wegfallenden Produkten ausdrücklich Current-Invalidierung planen, keine historische Beobachtung löschen.

## Reproduktion ohne Datenbank

Cargo liegt in dieser Umgebung nicht im PATH; absoluten Pfad verwenden:

```sh
/root/.cargo/bin/cargo run -q -p schrott-mcp-ingestion --example live_handlers -- th-gotha-msg-metallrecycling-msg-schrott-und-meta > /tmp/opencode/msg-live-20261008.txt
```

Tatsächlich ausgeführt; HTTP 200, 45 Preiszeilen, Kontakt Gotha kongruent; u.a.:

```text
kupfer-millberry 9.900 EUR/kg conf=Some(1.0) [Millberry (ab 1 kg:)]
SKIP: 94 höhere Staffeln übersprungen (nur kleinste Staffel je Sorte erfasst)
```

Scharfer, ausgeführter Red-Check auf realem Handleroutput:

```sh
python3 - <<'PY'
from pathlib import Path
s=Path('/tmp/opencode/msg-live-20261008.txt').read_text()
rows=[l for l in s.splitlines() if 'conf=' in l and '[Millberry (' in l]
print('Actual Millberry rows:',rows)
assert len(rows)==3 and all(any(t in r for r in rows) for t in ['ab 1 kg:', 'ab 100 kg:', 'ab 1000 kg:']), 'MISSING_TIERS: expected 3 Millberry rows, got '+str(len(rows))
PY
```

Ergebnis: Exit 1, `AssertionError: MISSING_TIERS: expected 3 Millberry rows, got 1`. Das aktuelle Output-Artefakt ist der deterministische schnelle Loop; der HTTP-Abruf selbst bleibt naturgemäß veränderlich. Quell-HTML separat unter `/tmp/opencode/msg-preisliste-20261008.html`, mit `curl -fsSL https://www.msg-metallrecycling.de/preisliste -o ...` erfasst. Diese temporären Dateien sind keine committed Fixtures.

## Ursache und nächste Schritte für Owner

1. `crates/ingestion/src/traders/handlers/msg_metallrecycling_gotha.rs:381–441`: `kept` erlaubt genau den ersten Preis pro Karte, jeder weitere wird nur gezählt. Keine Extraktionspanne oder fehlender JavaScript-Render. Bestehender Test `smallest_tier_wins_and_window_holds` schreibt dieses unvollständige Verhalten ausdrücklich fest.
2. `grade_for` bildet nur die Sorte auf eine statische Variante ab; Schwelle landet ausschließlich im `label`/notes. Dedup in `scrape` nutzt `(material, variant, price.to_bits())`, nicht die Schwelle. Nach einfachem Entfernen von `kept` würden verschiedene Staffeln in derselben Current-Identität kollabieren; identische Preise verschiedener Schwellen sogar vorher dedupliziert.
3. `ScrapedPrice.variant` in `traders/mod.rs:106` ist `&'static str`. Dynamisch wechselnde Schwellen passen nicht in eine endlich hardcodierte Tabelle. Owner sollte den vereinbarten Varianten-/Mengenvertrag umsetzen (z.B. owned Variante mit separater Schwellen-Dimension) und alle Call-Sites prüfen; **keine** `Box::leak`-Umgehung.
4. Primärquelle explizit auf sichtbare Preisreihen und sichtbare kg/t-Zelle parsen; derzeit `cells[0]`/`cells[2]` ohne Sichtbarkeitsprüfung. Echte Einheiten aus Quellenhinweis belegen statt Markt-Plausibilität; Modulkommentar „Einheit steht nie explizit da“ ist falsch. Preisnormalisierung darf Schwelle nicht umrechnen.
5. Regressionen: alle drei Millberry-Staffeln, erste leere Staffel bei FE, einpreisige Bronze, fallende Rotguss-Staffeln, identische Werte bei verschiedenen Schwellen, versteckte Dummywerte, spätere neue Schwellen/umbenannte Sorten, unbekannte Einheiten, 0,00 weiterhin getrennt bewerten. Sichtbare Mengenwerte dynamisch lesen, nicht bloß 1/100/1000 hardcoden. Katalogmapping bleibt eine eigene, explizite Freigabegrenze.
6. Vor Veröffentlichung Current-Reconciliation für entfernte/umgemappte Sorten und alte staffellose Varianten klären; heute belegte Wasseruhren-Altlast berücksichtigen. Append-only Historie erhalten. Danach echter Handlervergleich, Schema-/Ingestiontests und read-only Produktionsspotcheck. Heute weder diese Reparatur noch Vollständigkeit nach Deployment behauptet.

## Hypothesenbewertung

Live-Red-Check grenzt den Fehler auf die einzelne vollständig bepreiste Millberry-Karte ein. Kandidaten: bewusste Parserauswahl (bestätigt durch `kept` und Skip), Current-Key-Kollision (zusätzlicher erforderlicher Fix, nicht Ursache des bereits reduzierten Outputs), falsch gelesene t-Alternativen (Feedbackpräzisierung, aktuell nicht Ursache), veralteter Produktionshandler (als alleinige Ursache widerlegt: aktueller lokaler Handler reproduziert). Fix-Phasen bewusst an Owner übergeben.
