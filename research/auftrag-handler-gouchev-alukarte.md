# Auftrag: Handler `gouchev` liest die Karte „Alu Profile / Alu Sorte1." nicht

**Status:** offen — **bestätigter** Datenbefund ✓ (der Handler läuft ✓, die Seite **bietet** die
Preise ✓, der Handler **nimmt sie nicht** ✓) ✓. Gehört in das **nächste** Release ✓ (Codeänderung ✓).

## Der Befund

Die Preisseite des Händlers
`https://www.schrottankauf-bitterfelderstr23.de/schrottpreise` ✓ hat eine Karte, die **fünf
Aluminiumsorten in einem Block** nennt ✓ und **keine** „ab 1 kg"-Zeile ✓ (Live-Abruf 08.11.2026,
HTTP 200 ✓):

```
Alu Profile 1,90€   Alu Sorte1. 1,10€   Alu Felgen 1,60€   Alu Kupferkühler 3,90€   Alu Guss 1,10€
```

Der Handler überspringt den ganzen Block ✓ — sein Schritt meldet dafür
„**kein ab-1-kg-Preis**" ✓ (die Parse-Stufe erwartet je Karte eine „ab 1 kg"-Zeile ✓ ✓). Die fünf
Sorten stehen damit **nicht** in der Datenbank ✓ ✗.

**Das ist kein Preisfehler**, sondern eine **Lücke der Preisaufnahme** ✓: der Betrieb bietet die
fünf Sorten an ✓, das System führt sie nicht ✓ ✓.

Der Handler hat den **verwandten Fall bereits gelöst** ✓: die Karte „Aluminiumkabel" ✓
(„ab 1 kg ➜ 0,10€ / ALUKABEL DICK 0,35€" ✓) wird in **zwei** Zeilen gesplittet ✓ — der Zweig
`l.contains("alukabel")` ✓ mit dem Zusatz „dick" ✓ ✓
(`crates/ingestion/src/traders/handlers/gouchev.rs` ✓, `grade_for` ✓ ✓).

## Was daraus folgt

| Sorte | Preis | Karte |
|---|---:|---|
| Alu Profile | 1,90 €/kg | `sorte1` ✓ |
| Alu Sorte1. | 1,10 €/kg | `sorte1` ✓ |
| Alu Felgen | 1,60 €/kg | `sorte1` ✓ |
| Alu Kupferkühler | 3,90 €/kg | `sorte1` ✓ |
| Alu Guss | 1,10 €/kg | `sorte1` ✓ |

**Kein** Preis darf geraten werden ✓: die Zuordnung Sortenname → Materialschlüssel und Variante
muss **belegt** sein ✓ (die Materialschlüssel der Datenbank ✓: `aluminium-gemischt` ✓,
`aluminium-felgen` ✓, `alu-kupfer-kuehler` ✓, `aluminium-guss` ✓ ✓ — die vorhandenen Schlüssel ✓
sagen, welche Sorten die Datenbank kennt ✓ ✓).

**Das bestehende Paar** `aluminium-gemischt` / *(leer)* / 1,10 €/kg ✓ (beobachtet 04.11. ✓) **bleibt
unangetastet** ✓ ✓ — der Preis wird weiterhin angeboten ✓ (als „Alu Sorte1." ✓) ✓, er ist **kein**
Rückzug ✓ ✓ (deshalb steht er **nicht** in der Migrationsliste ✓ ✓ und **nicht** in der Migration 12 ✓ ✓).

## Auftragsinhalt

1. **Die Karte parsen** ✓ — die fünf Sorten als je eine Zeile aufnehmen ✓, mit dem Zusatz im
   Variantenfeld ✓ (analog zum „dick"-Zweig ✓ ✓) ✓ — oder, wenn die Zuordnung nicht belegbar ist ✓,
   **laut** überspringen ✓ (nie raten ✓ ✓) ✓.
2. **Die Materialschlüssel prüfen** ✓ — welche Schlüssel die Datenbank für „Alu Profile", „Alu Felgen",
   „Alu Kupferkühler" und „Alu Guss" kennt ✓ ✓ (`materials` ✓, read-only ✓) ✓; neue Schlüssel nur
   mit Beleg ✓ ✓.
3. **Tests** ✓ — ein **Fixture** der heutigen Karte ✓ in
   `crates/ingestion/src/traders/handlers/fixtures/` ✓ ✓; ein Test, der die fünf Sorten **mit**
   den erwarteten Schlüsseln und Varianten **nimmt** ✓, und ein Test, der eine Karte **ohne**
   „ab 1 kg"-Zeile **laut** überspringt ✓ ✓.
4. **Keine DB-Schreibzugriffe** ✓, kein Force-Ingest ✓, kein Deployment ✓ ✓ — der Fix gehört ins
   **nächste** Release ✓ ✓ und durchläuft dort die Pre-Deploy-Prüfung ✓ ✓.

## Abgrenzung

- Die **25 Rückzüge** ✓ desselben Händlers ✓ sind davon **unberührt** ✓ — sie betreffen die
  **Mengenstufen** ✓, die die Seite nicht mehr anbietet ✓ ✓ (Migration 12 ✓ ✓).
- Die übrigen **neunzehn Karten** ✓ der Seite ✓ werden **richtig** gelesen ✓ ✓ (der Handler nimmt
  30 Preise ✓ ✓) ✓ — der Befund betrifft **eine** Karte ✓ ✓.
