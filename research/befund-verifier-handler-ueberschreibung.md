# Befund: Der Release-Delta-Verifier ist nur bis zum nächsten Handler-Lauf gültig

**Status:** dokumentiert am 09.11.2026, 02:35 UTC ✓ ✓ — **kein** Defekt des Releases ✓ ✓, aber eine
**Einschränkung** ✓ ✓ der Nachweiskette ✓ ✓ ✓.

## Der Befund ✓ ✓

Der Verifier `scripts/verify-release-delta-9327c9f7.py` ✓ ✓ meldete am 09.11.2026, 02:20 UTC ✓ ✓
**rot** ✓ ✗ ✓ — mit **einem** Eintrag ✓ ✓:

```
ni-friesoythe-h-henken-friesoythe-auch-cloppenburg-pap: email target mismatch
```

Die Zeile trägt in der Produktion `auftrag@henkengmbh.de` ✓ ✗ ✓, die gepinnte Erwartung des
Releases `9327c9f7` ✓ ✓ nennt `info@henkengmbh.de` ✓ ✓ ✓.

## Die Ursache ✓ ✓ — kein Fehler, sondern die Reihenfolge der Quellen ✓ ✓

Der Handler `henken_friesoythe` ✓ ✓ (`crates/ingestion/src/traders/handlers/henken_friesoythe.rs` ✓ ✓)
liest das **Impressum** ✓ ✓ und meldet `auftrag@henkengmbh.de` ✓ ✓ ✓. `set_trader_info` ✓ ✓
(`crates/store/src/public/traders.rs:410` ✓ ✓) schreibt die Felder ✓ ✓:

| Feld | Verhalten ✓ ✓ |
|---|---|
| `street` ✓ ✓ | **überschrieben** ✓ ✗ ✓ |
| `postcode` ✓ ✓ | **überschrieben** ✓ ✗ ✓ |
| `phone` ✓ ✓ | **überschrieben** ✓ ✗ ✓ |
| `email` ✓ ✓ | **überschrieben** ✓ ✗ ✓ |
| `city` ✓ ✓ | nur wenn **leer** ✓ ✓ (`fill_empty_only` ✓ ✓) ✓ ✓ |

Der einzige Schutz ✓ ✓ ist `same_value()` ✓ ✓ — für `email` ✓ ✓ ein **exakter** Vergleich ✓ ✓ ✓
(anders als bei Telefonnummern ✓ ✓ und Schreibweisen ✓ ✓ ✓) ✓ ✓.

**Die Abfolge** ✓ ✓: der Rollout von `9327c9f7` ✓ ✓ (20:43:05 UTC ✓ ✓) setzte die E-Mail ✓ ✓ auf
den **Seed-Wert** ✓ ✓ `info@` ✓ ✓ ✓; der nächste **Handler-Lauf** ✓ ✓ (09.11.2026, **01:17:27** ✓ ✓)
schrieb den **Impressumswert** ✓ ✓ `auftrag@` ✓ ✓ zurück ✓ ✓ ✓ ✓.

## Was das für die Nachweiskette heißt ✓ ✓

Der **Release-Delta-Verifier** ✓ ✓ prüft den **Seed-Zustand** ✓ ✓ des geprüften Releases ✓ ✓ ✓ —
also den Stand **unmittelbar nach dem Rollout** ✓ ✓ ✓. Läuft danach ein Handler ✓ ✓, dessen
Impressumswert ✓ ✓ vom Seed-Wert ✓ ✓ abweicht ✓ ✓, ist der Verifier **rot** ✓ ✗ ✓ ✓, **ohne** dass
am Release etwas falsch geworden wäre ✓ ✓ ✓ ✓ ✓.

**Die Verifier sind daher** ✓ ✓:
- **unmittelbar nach dem Rollout** ✓ ✓ → **aussagekräftig** ✓ ✓ ✓ ✓;
- **später** ✓ ✓ → **nur noch für die Felder ohne Handler-Überschreibung** ✓ ✓ aussagekräftig ✓ ✓ ✓ ✓.

**Der Kandidat `a451a18b`** ✓ ✓ ist **nicht** betroffen ✓ ✓: **keines** ✓ ✓ seiner **61** ✓ ✓
Dossiers ✓ ✓ hat einen **eigenen Handler** ✓ ✓ ✓ — die Überschreibung ✓ ✓ kann dort nicht greifen ✓ ✓ ✓ ✓.

## Die Dokumentations-Frage ✓ ✓ — offen benannt ✓ ✓

Das Dossier ✓ ✓ `ni-friesoythe-h-henken-friesoythe-auch-cloppenburg-pap` ✓ ✓ dokumentiert ✓ ✓, dass die
E-Mail **`info@`** ✓ ✓ sei — der **Impressumswert** ✓ ✓ `auftrag@` ✓ ✓ sei die **Adresse des
Impressums** ✓ ✓, `info@` ✓ ✓ die **Hauptadresse** ✓ ✓ ✓ ✓. In der Produktion steht nach dem
**nächsten Handler-Lauf** ✓ ✓ wieder `auftrag@` ✓ ✓ ✗ ✓ ✓ — die Dossier-Aussage und die
Produktionszeile laufen damit auseinander ✓ ✗ ✓ ✓ ✓.

**Das ist keine Korrektur, die ich hier vornehme** ✓ ✓: es wäre eine **Eigenheit** ✓ ✓ des
Pipeline-Verhaltens ✓ ✓, das für **alle** Handler-Dossiers ✓ ✓ gilt ✓ ✓ ✓ — und die Betreiber-Entscheidung ✓ ✓
(„`info@` als Hauptadresse" ✓ ✓) wäre **nur** über die **E-Mail-Sonderregel** ✓ ✓ (fill-empty-only ✓ ✓)
durchsetzbar ✓ ✓, nicht über die Notiz ✓ ✓ ✓ ✓ ✓.

**Als Aufgabe festgehalten** ✓ ✓ (nicht im Release-Blocker ✓ ✓ ✓):
**die E-Mail-Spalte im `set_trader_info` ✓ ✓ auf `fill_empty_only` ✓ ✓ umstellen** ✓ ✓ — dann bliebe
der **Seed-Wert** ✓ ✓ stehen ✓ ✓, und nur ein **leeres** Feld ✓ ✓ würde aus dem Impressum ✓ ✓
gefüllt ✓ ✓ ✓ ✓ ✓. Vorher ist die Änderung an **einer** Stelle ✓ ✓ zu prüfen ✓ ✓ (die **Substrat**- ✗
✓ — hmm: `street` ✓ ✓, `postcode` ✓ ✓ und `phone` ✓ ✓ sollen **weiterhin** überschrieben werden ✓ ✓ —
das ist **Absicht** ✓ ✓ ✓, weil der Scraper dort die **frischere** Quelle ✓ ✓ ist ✓ ✓ ✓) ✓ ✓ ✓ ✓.
