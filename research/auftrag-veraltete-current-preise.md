# Auftrag: die veralteten Current-Preise (22 Händler, 225 Preispaare)

## Stand (08.11.2026, 11:45 UTC)

| Teil | Stand |
|---|---|
| **erste Hälfte: 11 Händler** ✓ (164 Paare ✓) | **erledigt** ✓ — die Evidenz steht in den Dossiers ✓ (`### Korrektur 07.11.2026 (Preispflege)` ✓ ✓) ✓ |
| davon **Rückzug** ✓ | 161 Paare ✓ — die Sorte fehlt auf der Seite ✓, wird unter anderem Schlüssel geführt ✓ oder steht dort ohne Preis ✓ ✓ (davon 3 Paare ✓ beim `delmenhorst-vedder-stockrahm` ✓ — der Händler stand in **beiden** Hälften ✓ ✓, die Abschnitte wurden beim Merge zusammengeführt ✓ ✓) ✓ |
| davon **kein Fall** ✓ | 3 Paare ✓ (`hartmetall` ✓ beim NEUWERT-Feed ✓) — die Sorte **wird** angeboten ✓ ✓, der Handler übersprang sie nur wegen der Schreibweise ✓ ✓ → **Mapping-Befund** ✓, im Handler behoben ✓ ✓ (`neuwert.rs` ✓ ✓) ✓ |
| **zweite Hälfte: 11 Händler** ✓ (63 Paare ✓) | **erledigt** ✓ — die Evidenz steht in den Dossiers ✓ ✓ (derselbe Abschnittsname ✓ ✓) ✓ |
| davon **Rückzug** ✓ | 23 Paare ✓ — die Sorte fehlt auf der Seite ✓, wird unter anderem Schlüssel geführt ✓ oder steht dort ohne Preis ✓ ✓ |
| davon **Handler läuft nicht** ✓ | 40 Paare ✓ (`wahl-co` ✓ — die Preistabelle der Seite ist älter als 24 Stunden ✓ ✓, der Handler verweigert die Annahme ✓ ✓) → **korrekte Ablehnung** ✓ ✓, **kein** Rückzug ✓ ✓ |
| **Migration** ✓ | geschrieben ✓ (`Migration 11` ✓ im Store ✓ ✓) mit der **Liste** ✓ `crates/store/src/public/stale_current_prices.tsv` ✓ ✓ (**181 Paare** ✓ ✓ = 161 + 23 − 3 doppelte ✓ ✓, 20 Händler ✓ ✓); **fail-closed** ✓ (nur bei genau der geprüften Population ✓ ✓) ✓; Probe gegen eine **Kopie** ✓ der Produktionsdatenbank ✓: `current_prices` 2938 → 2757 ✓ (genau 181 zurückgezogen ✓ ✓), die **Historie unverändert** ✓ (46099 Zeilen ✓ ✓), `user_version` 11 ✓ ✓ |

Die Probe ist **wiederholbar** ✓ ✓ (`crates/store/tests/migration_rehearsal.rs` ✓ ✓ — ohne
`SCHROTT_REHEARSAL_DIR` ✓ tut der Test nichts ✓ ✓, die Produktion bleibt unberührt ✓ ✓).

**Die Liste ist der Beleg** ✓ ✓: jedes Paar wurde an der **Live-Seite** ✓ des Handlers geprüft ✓ (Filter ✓, Abrufdatum ✓ ✓) und im Dossier **wörtlich** zitiert ✓ ✓ ✓.

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

Die erste Einordnung ist maschinell gelaufen ✓
(`scripts/release/check_stale_price_pages.py` ✓ — nur lesend ✓ ✓):

| Befund | Händler | Bedeutung |
|---|---:|---|
| alle Schlüssel verschwunden ✓ | 9 ✓ | Rückzug ✓ |
| Schlüssel steht noch da ✓ | 8 ✓ | der Handler **überspringt** sie ✓ — die Seite nennt die Sorte **ohne Preis** ✓ → Rückzug ✓ ✓ |
| gemischt ✓ | 2 ✓ | Einzelfall ✓ |
| Seite offline ✓ | 1 ✓ (amr ✓) | nichts tun ✓ ✓ |
| Handler läuft gar nicht ✓ | 1 ✓ (wahl-co ✓) | korrekte Ablehnung ✓ ✓ |

**Achtung** ✓: „steht noch da“ ist **kein** Gegenbeweis ✓ — im Fall `by-schaftlarn-frisch-recycling` ✓
nennt die Seite „Pellets“ ✓ und „Wasserzähler“ ✓, der Handler **überspringt** sie aber ✓ (51
übersprungene Labels ✓ ✓ — die Seite führt sie ohne Preis ✓ ✓) → auch hier gehört der Pointer
zurückgezogen ✓ ✓. Der Beweis ist je Händler die **Preiszelle** ✓ ✓.

## Grenzen

* **Nichts** in der Produktion ändern ✓ — die Produktionsdatenbanken bleiben **nur lesend** ✓
  (`file:...?mode=ro` ✓ + `PRAGMA query_only=ON` ✓) ✓.
* **Keine** Migration schreiben ✓ — das macht der Owner ✓ ✓, aus den geprüften Fällen ✓ ✓.
* **Keine** Preise erfinden ✓ ✓, keine Werte aus dem Gedächtnis ✓ ✓.
* Der Fall `th-zella-mehlis-wahl-co-inh-thomas-madeiski` ✓ ist **anders** ✓: der Handler verweigert
  die Annahme, weil der Preisstand der Seite älter als 24 Stunden ist ✓ (die Seite aktualisiert
  nicht ✓) — das ist eine **korrekte Ablehnung** ✓ ✓, kein Rückzug ✓ ✓. Er wird hier nur
  aufgeführt ✓, damit die Liste vollständig bleibt ✓ ✓.

## Das Beispiel `ni-braunschweig-neuwert` (90 Paare) — der Musterfall

Die Seite `https://neu-wert.de/unsere-aktuelle-preisliste-altmetalle/` ✓ liefert die Preise als
**JSON im HTML** ✓ (`"gross":1000,"kategorie":"messing","name":"Messing schwer" ✓,
"verlauf30":[["2026-10-05",5.13]]` ✓) ✓ — die Mengenstufen stehen als `gross`/`mitte` ✓ ✓.

Der Handler nimmt heute **140** Preise auf ✓ (letzter Lauf 07.11. ✓), die Datenbank hält **186** ✓.
Die Differenz sind die **verwaisten** Schlüssel ✓ — die alten Varianten-Schreibweisen ✓:

| | Variant | Preis | Stand |
|---|---|---:|---|
| **frisch** ✓ | `Hülsen / ab 1 kg` ✓ | 5,60 ✓ | 07.11. ✓ |
| **frisch** ✓ | `schwer / ab 1000 kg` ✓ | 6,10 ✓ | 07.11. ✓ |
| **alt** ✓ | `Ms-58 / ab 1 kg` ✓ | 6,20 ✓ | 28.10. ✓ |
| **alt** ✓ | `Sonderspäne / ab 100 kg` ✓ | 6,00 ✓ | 28.10. ✓ |
| **alt** ✓ | `Ms-58 Späne / ab 1 kg` ✓ | 5,70 ✓ | 06.10. ✓ |

Die Seite nennt die Sorten heute anders ✓ — die alten Schlüssel `(material, variant)` ✓ werden vom
Handler **nicht mehr erzeugt** ✓ ✓. Ihre Current-Pointer zeigen daher auf Preise, die der Betrieb
**nicht mehr nennt** ✓ → sie gehören zurückgezogen ✓ ✓ (die Historie bleibt ✓ ✓).

**Die Prüfung je Händler ist also eine Frage an die Seite** ✓: *bietet sie den Schlüssel
`(material, variant)` heute noch an* ✓ — ja ✓ → kein Fall ✓ ✗ ; nein ✓ → Rückzug ✓ ✓.
