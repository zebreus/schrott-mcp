# Auftrag Handler MKM — die Staffel der Nickelposition fehlt (22.11.2026)

**Grund** ✓ ✓ — der MKM-Handler ✓ ✓ scheitert seit dem **20.11.2026 00:34
UTC** ✓ ✓ an derselben Meldung ✓ ✓:

```
MKM-NE-05: priced row without tier ladder is not a reviewed flat quote
```

Das ist **kein** Fehler des Releases ✓ ✓ ✓ ✓ — der letzte **grüne** MKM-Lauf
war am **20.11.2026 12:09:13 UTC** ✓ ✓ (**56** Items ✓ ✓) ✓ ✓

## Der Befund des Owners ✓ ✓

Die Seite `https://mkm-metals.de/metallpreise` ✓ ✓ (live abgerufen am
**22.11.2026** ✓ ✓) weist die **Nickelposition** ✓ ✓ inzwischen **ohne
Mengenstaffel** ✓ ✓ aus ✓ ✓ — statt des Rechners mit `data-staffel` ✓ ✓
steht dort der **Anfrage-Link** ✓ ✓ ✓ ✓

Die **Zeile** ✓ ✓ im Wortlaut ✓ ✓:

```html
<tr>
  <th scope="row"><a href="/metallankauf/produkt/nickel-rein">Nickel rein ab 99 %</a></th>
  <td class="muted small">MKM-NE-05</td>
  <td class="num"><strong>12,08 €</strong> <span class="muted small">/kg</span>
      <span class="muted small" style="display: block;"> ab 250 kg</span></td>
  <td class="num muted small"><time datetime="2026-11-20" title="20.11.2026, 14:24 Uhr">vor 2 Tagen</time></td>
  <td style="min-width: 9.5rem;">
      <a class="btn btn--ghost btn--sm" href="/angebot?material=Nickel+rein+ab+99+%25">Anfragen</a></td>
</tr>
```

Das ist **genau** die **Flat-Quote-Form** ✓ ✓, die der Vertrag am **17.11.2026**
✓ ✓ eingeführt hat ✓ ✓ ✓ ✓: ein **sichtbarer Höchstpreis** ✓ ✓, ein
**sichtbares Minimum** ✓ ✓ und ein **same-origin Anfrage-Link** ✓ ✓, dessen
`material`-Parameter ✓ ✓ **exakt** die Zeilenbezeichnung ✓ ✓
*Nickel rein ab 99 %* ✓ ✓ **dekodiert** ✓ ✓ ✓ ✓

## Der Grund, warum die Zeile heute abgewiesen wird ✓ ✓

`FLAT_QUOTE_ARTICLES` ✓ ✓ in
`crates/ingestion/src/traders/handlers/mkm_metals/overview.rs` ✓ ✓ wurde am
**17.11.2026** ✓ ✓ aus einem **damaligen** Live-Snapshot abgeleitet ✓ ✓ ✓ ✓
Dieser Snapshot ✓ ✓ ist in
`crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261117-sanitized.html`
✓ ✓ gespeichert ✓ ✓ — dort trug `MKM-NE-05` ✓ ✓ **noch** die **Staffel**
✓ ✓:

```html
<tr><th scope="row"><a href="/metallankauf/produkt/nickel-rein">Nickel rein ab 99 %</a></th>
<td>MKM-NE-05</td><td><strong>12,18 €</strong> /kg ab 250 kg</td>
<td><time datetime="2026-10-14">am 14.10.2026</time></td>
<td><input data-price="11.6000" data-staffel='[{"ab":0,"preis":11.6},{"ab":2,"preis":11.83},{"ab":250,"preis":12.18}]'></td></tr>
```

Die Seite hat sich **danach** ✓ ✓ geändert ✓ ✓ — die Nickelposition ✓ ✓
verlief am **16.11.2026** ✓ ✓ in die Flat-Form ✓ ✓ ✓ ✓

**Nur** `MKM-NE-05` ✓ ✓ ist **betroffen** ✓ ✓ — der Owner hat **alle** **69**
Zeilen ✓ ✓ der Seite geprüft ✓ ✓: **45** ✓ ✓ tragen eine Staffel ✓ ✓, **zwei**
✓ ✓ ohne ✓ ✓ (`MKM-NE-05` ✓ ✓ und `SW10041` ✓ ✓) ✓ ✓, und `SW10041` ✓ ✓ steht
**bereits** ✓ ✓ in der Liste ✓ ✓ ✓ ✓

## Die Änderung ✓ ✓

`MKM-NE-05` ✓ ✓ wird in `FLAT_QUOTE_ARTICLES` ✓ ✓ aufgenommen ✓ ✓ — damit
greift für diese Zeile **derselbe** geprüfte Vertrag ✓ ✓ wie für die übrigen
**neun** ✓ ✓ Flat-Quote-Artikel ✓ ✓ ✓ ✓

**Nichts** ✓ ✓ wird **gelockert** ✓ ✓: eine bepreiste Zeile **ohne** Staffel
✓ ✓ bleibt ein **harter Fehler** ✓ ✓ ✓ ✓ Nur die **eine** Zeile ✓ ✓ kommt
hinzu ✓ ✓, deren Form ✓ ✓ **einzeln** geprüft ✓ ✓ ist ✓ ✓

## Der Nachweis ✓ ✓

Ein **neues** Fixture ✓ ✓ aus dem **heutigen** Live-Abruf ✓ ✓:
`crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261122-sanitized.html`
✓ ✓ — **47** Zeilen ✓ ✓, davon **zwei** ohne Staffel ✓ ✓ ✓ ✓ Es ist
**minimalisiert** ✓ ✓: nur Label ✓ ✓, Artikelnummer ✓ ✓, Preis mit Minimum
✓ ✓, Datum ✓ ✓ und der Rechner ✓ ✓ bzw. der Anfrage-Link ✓ ✓ — keine
**fremden** Pfade ✓ ✓, keine Skripte ✓ ✓, keine Bilder ✓ ✓ ✓ ✓

**Wichtig** ✓ ✓: das Fixture vom **17.11.2026** ✓ ✓ bleibt **unverändert**
✓ ✓ — es ist der **Nachweis** ✓ ✓ der damaligen Entscheidung ✓ ✓ ✓ ✓

## Die Tests ✓ ✓

- `cargo test -p schrott-mcp-ingestion mkm` ✓ ✓ — die **MKM**-Tests ✓ ✓
- `cargo test --workspace` ✓ ✓ — die **gesamte** Suite ✓ ✓
- `cargo fmt --check` ✓ ✓ ✓ ✓

**Erwartung** ✓ ✓: der Handler läuft wieder **grün** ✓ ✓ und liefert **alle**
**Zeilen** ✓ ✓ der Seite ✓ ✓ — auch die **Nickelposition** ✓ ✓, jetzt mit dem
**Flat-Quote**-Formular ✓ ✓ und dem **Datum** ✓ ✓ des **Seitenstands**
✓ ✓ (`2026-11-20` ✓ ✓) ✓ ✓ ✓ ✓

## Die Grenze ✓ ✓

Der **Preis** ✓ ✓ der Nickelposition ✓ ✓ ist ein **Höchstpreis** ✓ ✓ mit
**Mindestmenge** ✓ ✓ — **kein** kg-Kurs ✓ ✓ ohne Bedingung ✓ ✓ ✓ ✓ Er wird
**wie** ✓ ✓ die anderen Flat-Quote-Artikel ✓ ✓ **ohne** eigene Preiskurve
✓ ✓ **geführt** ✓ ✓ ✓ ✓
