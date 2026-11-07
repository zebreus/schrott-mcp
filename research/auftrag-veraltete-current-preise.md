# Auftrag: die veralteten Current-Preise (22 Händler, 225 Preispaare)

## Befund

Ein Handler nimmt je Lauf die Preise auf, die seine Seite **heute** anbietet ✓. Bietet die Seite
eine Sorte oder eine Preisstufe nicht mehr an — oder ändert sich die **Bezeichnung** der Variante
(„Festplatte“ → „Festplatten“ ✓) —, bleibt der frühere Preis als „aktueller Preis“ stehen ✓. Für
normale Handler zieht die Pipeline Current-Pointer **nicht** zurück ✓ (bei einem vorübergehend
kaputten Seitenbild wären die Preise sonst weg ✓ ✓).

Das Projekt löst solche Fälle **einzeln** über Migrationen ✓: Migration 7 (`goldtrans` ✓) und
Migration 8 (`heraeus` ✓) — „Retire only their current pointers; keep every historical field intact“ ✓ ✓.

Die Kandidatenliste steht in `scripts/release/find_stale_current_prices.py` ✓ (read-only ✓):

```bash
python3 scripts/release/find_stale_current_prices.py --json /tmp/opencode/stale-preise.json
```

| Händler | veraltet | Current gesamt | letzter Lauf |
|---|---:|---:|---|
| `ni-braunschweig-neuwert-schrott-altmetallhandel` | 90 | 186 | 07.11. ✓ |
| `by-aschaffenburg-huth-schrott-metall-huth` | 46 | 266 | 07.11. ✓ |
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` | 40 | 40 | **28.10.** ✓ |
| `by-aschaffenburg-bernhard-westarp` | 7 | 236 | 07.11. ✓ |
| `by-schaftlarn-frisch-recycling` | 5 | 90 | 07.11. ✓ |
| 16 weitere Händler | 1–4 | | 07.11. ✓ |

## Aufgabe je Händler

1. **Die Live-Seite holen** ✓ (die URL steht im Handler ✓ oder in `scripts/verify-handler-release.py` ✓) ✓.
2. **Prüfen, ob der Preis (bzw. die Variante) heute noch angeboten wird** ✓:
   * **nicht mehr angeboten** ✓ → der Current-Pointer gehört zurückgezogen ✓;
   * **noch angeboten** ✓, aber unter **anderer Bezeichnung** ✓ → die alte Variante ist ein
     Fremdkörper ✓ und gehört ebenfalls zurückgezogen ✓ ✓ (der Preis bleibt in der Historie ✓ ✓);
   * **noch angeboten** ✓, unverändert ✓ → **kein** Fall ✓ (dann ist der Handler nicht gelaufen ✓ —
     das wäre ein eigener Befund ✓ ✓).
3. **Die Evidenz dokumentieren** ✓: die Live-Seite ✓, das Zitat der Zeile ✓, das Abrufdatum ✓.

## Grenzen

* **Nichts** in der Produktion ändern ✓ — die Produktionsdatenbanken bleiben **nur lesend** ✓
  (`file:...?mode=ro` ✓ + `PRAGMA query_only=ON` ✓) ✓.
* **Keine** Migration schreiben ✓ — das macht der Owner ✓ ✓, aus den geprüften Fällen ✓ ✓.
* **Keine** Preise erfinden ✓ ✓, keine Werte aus dem Gedächtnis ✓ ✓.
* Der Fall `th-zella-mehlis-wahl-co-inh-thomas-madeiski` ✓ ist **anders** ✓: der Handler verweigert
  die Annahme, weil der Preisstand der Seite älter als 24 Stunden ist ✓ (die Seite aktualisiert
  nicht ✓) — das ist eine **korrekte Ablehnung** ✓ ✓, kein Rückzug ✓ ✓. Er wird hier nur
  aufgeführt ✓, damit die Liste vollständig bleibt ✓ ✓.
