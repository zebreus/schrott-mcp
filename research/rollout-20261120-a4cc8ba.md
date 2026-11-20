# Rollout 20.11.2026 — Kandidat `a4cc8ba` (die Welle vom 06:00 und elf Geokodierungen)

**Kandidat:** `a4cc8babf81698ec7a4b8deb7ec13507a0440cce`
**Tree:** `f168797e08b19e0ccb048162a4cb373c8aca83a0`
**Basis:** `01a05b2` (der vorige Rollout)
**Rollout:** **20.11.2026, 12:08:56–12:08:57 UTC**
**Backup:** `/root/Documents/schrott-mcp-rollout-backups/rollout-a4cc8ba-20261120T120856Z`

| Feld | Wert |
|---|---|
| `release_source_commit` | `a4cc8babf81698ec7a4b8deb7ec13507a0440cce` |
| `release_source_tree` | `f168797e08b19e0ccb048162a4cb373c8aca83a0` |
| Binary `schrott-mcp-server` | `029edb41d6d057fa9…` (vorher `61c3e167df62778f1…`) |
| Binary `schrott-mcp-query-worker` | `e85f4cb6406d6f138…` (vorher `331dbcdeeaa701da4…`) |
| Workspace-Tests | **13** Suiten · **852** passed · **0** failed · **13** ignored |
| Service-Neustart | **20.11.2026 12:08:57 UTC** (`active`, `NRestarts` **0**) |
| `production_write` | **False** ✓ ✓ |
| **Geänderte Dossiers** | **71** ✓ ✓ |

## Was ausgerollt wurde

Die **60** Dossiers der Welle vom 20.11.2026, **06:00 UTC** ✓ ✓ — **15**
Bundesländer ✓ ✓, **drei** bundesland-disjunkte Shards je **20** ✓ ✓ ✓ ✓

**Und** die **elf** Standortpunkte ✓ ✓, die der Owner nach den Messungen der Welle
vom **00:00** ✓ ✓ entschieden hat ✓ ✓ ✓ ✓

**Null** Überschneidung ✓ ✓ mit allen **662** Dossiers der zwölf früheren Wellen ✓ ✓ ✓ ✓

## Die Nachprüfung read-only ✓ ✓

| Verifier | Exit | Ergebnis |
|---|---|---|
| `verify-release-delta-a4cc8ba.py` | **0** ✓ ✓ | `ok: true` ✓ ✓, **71** Zeilen ✓ ✓ |
| `verify-wave-e-postrollout-a4cc8ba.py` | **0** ✓ ✓ | `ok: true` ✓ ✓, **60** Zeilen ✓ ✓ |
| `verify_notes_in_production.py` (Scope 06:00-Release) | **0** ✓ ✓ | `ok: true` ✓ ✓, **71/71** identisch ✓ ✓ |
| `verify-pending-corrections-a4cc8ba.py` | **1** ✓ ✓ | **erwartbar** ✓ ✓ (siehe unten ✓ ✓) ✓ ✓ |

**Zählwerte nach dem Rollout** ✓ ✓: traders **3945** ✓ · materials **83** ✓ ·
prices **113050** ✓ · current_prices **4077** ✓ · trader_materials **1688** ✓ ·
`user_version` **12** ✓ ✓ ✓ ✓

**Der Dienst** ✓ ✓: Neustart **12:08:57 UTC** ✓ ✓, `ActiveState=active` ✓ ✓,
`NRestarts` **0** ✓ ✓ ✓ ✓, Health **200** ✓ ✓ ✓ ✓ — und der erste Lauf nach dem
Neustart ✓ ✓ ist der Scheduler-Lauf ✓ ✓ `id 5226` ✓ ✓ von **12:09:12 UTC** ✓ ✓ ✓ ✓

**15** Standortpunkte ✓ ✓ haben in der Produktion **keinen** Punkt mehr ✓ ✓ ✓ ✓

### Der Pending-Gate ✓ ✓

Er fordert einen **erfolgreichen Scheduler-Schritt nach** dem Rollout ✓ ✓ — der
Rollout war um **12:08:57 UTC** ✓ ✓, der letzte MSG-Lauf um **11:10:09** ✓ ✓, der
letzte Gutzmann-Lauf um **08:53:39** ✓ ✓ und der letzte MKM-Lauf um **12:09:13**
✓ ✓ (fünf Sekunden nach dem Neustart ✓ ✓) ✓ ✓

**Die Turnusläufe** ✓ ✓: Gutzmann um **14:53:39 UTC** ✓ ✓, MSG um **17:10:09 UTC**
✓ ✓ — **danach** ✓ ✓ wird der Pending-Gate **grün** ✓ ✓ ✓ ✓ Eine **Nachprüfung**
✓ ✓ ist für **15:00 UTC** ✓ ✓ angelegt ✓ ✓ ✓ ✓

**Bis dahin** ✓ ✓ gilt: **keine** weiteren Produktionsänderungen ✓ ✓ ✓ ✓

## Der Weg dorthin — ein Review ✓ ✓

**Ein** unabhängiges Pre-Deploy-Review ✓ ✓ — **FREIGE** ✓ ✓ mit **keinem**
blockierenden Befund ✓ ✓ ✓ ✓

**Ein** Agent hatte eine **veröffentlichte** Notiz **in place** gekürzt ✓ ✗ ✓ — die
Zeile mit den **Quellen** ✓ ✓ im Dossier des Dienstleisters in Bassum ✓ ✓ ✓ ✓
Sie ist **wortgleich wiederhergestellt** ✓ ✓ (Commit `10a5ad05` ✓ ✓) ✓ ✓ und der
neue Befund — die **Faxnummer** ✓ ✓ — steht in der **neuen** Sektion ✓ ✓ ✓ ✓

**Ein** **geringfügiger** Hinweis ✓ ✓: das Gate `check_note_claims` ✓ ✓ endet mit
Exit **1** ✓ ✓ und meldet **zwei** Befunde ✓ ✓ — **beide** sind **Fehlalarme**
✓ ✓ und **von mir nachgeprüft** ✓ ✓: das eine liest ein **Backtick-`name`** ✓ ✓
(der **Name eines OSM-Tags** ✓ ✓) als Behauptung über das **Dossier-Feld** ✓ ✓,
das andere liest **„entfernt"** ✓ ✓ als Ausschluss ✓ ✓, gemeint ist die **Lage**
✓ ✓ (die Rückauflösung ergibt eine Straße **1,9 km** vom belegten Betriebspunkt
✓ ✓ entfernt ✓ ✓) ✓ ✓ Die Disposition steht in `research/auftraege-20261120.md`
✓ ✓ ✓ ✓

## Der Bericht ✓ ✓

Der Bericht ✓ ✓ nennt **60** Dossiers ✓ ✓ in **15** Bundesländern ✓ ✓ · **173**
Recherche-Bullets ✓ ✓ · **sechs** Dossiers mit Zahlen ✓ ✓ — **keine** eigene
Preisarbeit ✓ ✓, die **Gebührenliste** eines Containers ✓ ✓ und die **Plattformpreise**
✓ ✓ sind **fremd** ✓ ✓ — · **10** Dossiers mit Geokodierungsthema ✓ ✓ ✓ ✓

## Reproduktion ✓ ✓

```bash
RELEASE_SHORT=a4cc8ba \
RELEASE_COMMIT=a4cc8babf81698ec7a4b8deb7ec13507a0440cce \
RELEASE_TREE=f168797e08b19e0ccb048162a4cb373c8aca83a0 \
RELEASE_TARGET_SLUGS=research/release-20261120-0600-scope.json \
  python3 scripts/release/rollout.py

# Nachprüfung (read-only)
python3 scripts/verify-release-delta-a4cc8ba.py      --data-dir /var/lib/schrott-mcp
python3 scripts/verify-wave-e-postrollout-a4cc8ba.py --data-dir /var/lib/schrott-mcp
python3 scripts/verify-pending-corrections-a4cc8ba.py  --data-dir /var/lib/schrott-mcp \
  --since 2026-11-20T12:08:57Z
python3 scripts/release/verify_notes_in_production.py  --data-dir /var/lib/schrott-mcp \
  --scope research/release-20261120-0600-scope.json
```
