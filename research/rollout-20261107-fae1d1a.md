# Rollout 07.11.2026 — Release `fae1d1a` (395 Dossiers)

## Ergebnis: **ausgerollt und verifiziert**

| Feld | Wert |
|---|---|
| Release-Commit | `fae1d1a5c06453cbe469bb3ad8c660d741ecb0c3` |
| Tree | `d2810df572ca60b667d1f79d9a581b5327278ea7` |
| Vorgänger (live) | `20cf7fe` (Rollout 07.11. 08:06:56 UTC) |
| Rollout | 07.11.2026, 09:53:01–09:53:18 UTC |
| Backup-Dir | `/root/Documents/schrott-mcp-rollout-backups/rollout-fae1d1a-20261107T095301Z` |
| Binaries | `f88833cb…` (Server ✓) / `8bc8e199…` (Query-Worker ✓) |
| Neustart | 09:53:02 UTC ✓ — `NRestarts` **0** ✓, `ActiveState` **active** ✓ |
| Post-Rollout-Lauf | `ingestion_runs` **3968** ✓, Status **ok** ✓ |

## Inhalt des Releases

Der Release-Scope umfasst **395** Dossiers ✓ (Basis `27ef26f8` ✓):

| Teil | Umfang | Ergebnis |
|---|---|---|
| **Tiefe** — Zertifikatseinträge feldweise gegen das PDF | 115 Dossiers (Wellen 10:00 und 12:00) | **99** Einträge korrigiert ✓, 16 bestätigt ✓ |
| **6h-Welle 07:00 und 08:30** | 60 + 20 frische Dossiers | Registry-Abfragen, Kettenschlüsse, Negativbefunde mit Namensvettern ✓ |
| **6h-Welle 13:00** | 60 frische Dossiers | 49 Frontmatter-Änderungen ✓, 14 Feldzählungen ✓ |
| **SN-Zweige der Welle 10:00** | 2 × 32 Dossiers | **91 verlorene Notizen** je Zweig wiederhergestellt ✓, **11** neue Zertifikatseinträge ✓ |
| **Altbestand** | 7 Dossiers | unausgeglichene `certifications`-Einträge geheilt ✓ (**nur** Klammern ✓) |

### Die drei Reparaturen vor dem Rollout

1. **Die SN-Zweige** ✓ — beide Zweige der Welle vom 06.11.2026, 10:00 Uhr waren aus einem Merge-Konflikt
   entstanden ✓ und hatten in je **91 Dossiers** eine Notiz verloren ✓ (in beiden Zweigen dieselbe ✓:
   49 × `### Prüfung 06.11.2026` ✓ und 41 × `### Korrektur 06.11.2026` ✓). `scripts/release/restore_lost_notes.py`
   stellt genau diese Sektionen **wortgleich** aus der Basis wieder her ✓; die Dublette in
   `nw-bielefeld-drewer` ist entfernt ✓. Nach der Reparatur meldet `check_wave_branches.py` **0** Notizverlust ✓.
2. **Der Altbestand** ✓ — sieben Dossiers trugen einen `certifications`-Eintrag mit unbalancierter Klammer ✓
   (fünf fehlende schließende ✓, zwei zu früh geschlossene ✓). Kein Wert wurde geändert ✓.
3. **Die Klammer-Zählungen der sieben Notizen** ✓ — die Notizen nannten zunächst für alle fünf Dossiers
   dieselben Zahlen („5 öffnende, 4 schließende“ ✓); die realen Werte unterscheiden sich ✓ (6/5 ✓, 3/2 ✓,
   5/4 ✓, 1/0 ✓, 3/2 ✓). Jede Notiz nennt jetzt die Zahlen **ihres** Dossiers ✓ — vor und nach der Reparatur ✓.

## Pre-Deploy-Freigaben: **zwei** unabhängige Prüfungen

| Prüfung | Kandidat | Urteil |
|---|---|---|
| 1 | `eb8f536` → geprüft und auf **`fae1d1a5`** erweitert ✓ | **GRÜN** ✓ — 403 PDFs geprüft ✓, alle Wellenberichte nachgerechnet ✓, Release-Artefakt byteweise verifiziert ✓ |
| 2 | `139f5af6` / `c504e152` | **ROT** ✓ — zwei Befunde, **beide zu Berichtszahlen** ✓; die **Daten** ausdrücklich in Ordnung ✓ |

**Zur Reihenfolge, offen gesagt ✓:** Die zweite Prüfung lief **noch**, als der Rollout auf der Grundlage der
ersten GRÜN-Freigabe für `fae1d1a5` begann ✓. Ihre Befunde betrafen **nicht** die Daten ✓, sondern die
Zahlen in drei Wellenberichten ✓ — sie sind in einem **Folge-Commit** behoben ✓ (siehe unten ✓). Die
Daten selbst waren nachweislich in Ordnung ✓: kein verlorenes Element ✓, keine erfundenen Werte ✓, keine
geänderten Slugs ✓, keine in place veränderten Basis-Notizen ✓.

### Die beiden Befunde der zweiten Prüfung und ihre Behebung

| Befund | Behebung |
|---|---|
| `research/wave-20261107-0830-report.md:16` — vier Feldzählungen waren falsch ✓ (`phone` 4 statt **6** ✓, `description` 6 statt **7** ✓, `certifications` 3 statt **4** ✓, `lat`/`lon` 1 statt **2** ✓) | Die Zählung ist gegen `a44ee5c1..2bc04406` ✓ neu erhoben ✓ und die **Zählregel** steht jetzt im Bericht ✓ |
| Drei Berichte bezogen ihre Zahlen auf **`HEAD`** ✓ — das ist nach dem Rollout ein anderer Commit ✓ | Die Bezüge nennen jetzt **feste Commits** ✓ (`20cf7fe` ✓ bzw. `544da648` ✓); die zwei überholten Aussagen zum Altbestand ✓ tragen eine **datierte Korrektur** ✓ (der Bestand ist inzwischen geheilt ✓) |

`scripts/release/check_report_figures.py` rechnet die Kennzahlen der fünf Wellenberichte und der
SN-Zweige nach ✓ — **alle stimmen** ✓ (exit 0 ✓).

## Post-Rollout: **alle Verifier grün**

| Prüfung | Ergebnis |
|---|---|
| `verify-release-delta-fae1d1a.py` | **exit 0** ✓ — **79** geänderte Zeilen ✓, **948** Feldprüfungen ✓, 0 Abweichungen ✓ |
| `verify-wave-e-postrollout-fae1d1a.py` | **exit 0** ✓ — **60** Wave-E-Zeilen ✓, **660** Feldprüfungen ✓ |
| `verify-pending-corrections-fae1d1a.py --since 2026-11-07` | **exit 0** ✓ — 0 Abweichungen ✓ |
| `post_rollout_check.py` | **exit 0** ✓ — `ok: true` ✓ |
| Seed beim Start | **79** Zeilen geschrieben ✓ (09:53:03 UTC ✓), danach **0** ✓ (idempotent ✓) |

Der alte Wave-E-Pin `6b3da663` ist rot ✓ — **nur informativ** ✓: er ist ein Snapshot eines früheren
Releases ✓ und die Dossiers haben sich seither geändert ✓. Die **elf Skalare** des Wave-E-Scopes in der
Pin-Basis `wave-e-20261028-0600-expectations.json` ✓ sind unverändert ✓ — deshalb ist der release-gepinnte
Verifier des Releases grün ✓.

## Produktion nach dem Rollout

| Kennzahl | vor | nach | Δ |
|---|---|---|---|
| Händler | 3945 | 3945 | ±0 ✓ |
| Materialien | 83 | 83 | ±0 ✓ |
| `current_prices` | 2938 | 2938 | ±0 ✓ |
| Preise (alle) | 45928 | 45928 | ±0 ✓ |
| Händler mit `street` | 2917 | **2923** | **+6** ✓ |
| Händler mit `postcode` | 2791 | **2799** | **+8** ✓ |
| Händler mit Adresse (`street` oder `postcode`) | 2988 | **2994** | **+6** ✓ |
| Händler mit Koordinate | 3715 | **3716** | **+1** ✓ |
| Fremdschlüsselverletzungen | 24 | 24 | Altbestand ✓ |

Die Werte sind gegen das **Backup vor dem Rollout** gerechnet ✓ (`rollout-fae1d1a-20261107T095301Z/data/public.db` ✓,
nur lesend ✓) — die Zahlen stammen also aus dem echten Vorher-Zustand ✓.

## Was nach dem Rollout noch offen ist

* Der **Handler-Backlog** ✓: **126** Händler haben einen Preisseiten-Handler ✓, **2239** mit Website haben
  keinen ✓ — die nächste eigene Aufgabe ✓.
* Die **Zertifikatsprüfung** ✓: von **405** Dossiers mit nicht leerem `certifications`-Feld ✓ tragen **262**
  einen Zertifikats-PDF-Beleg in der Timeline ✓ — **143** sind noch ohne einen solchen Beleg ✓ (ein fehlender
  Beleg ist **kein** Nachweis der Abwesenheit ✓).
* Der **Wave-E-Pin** ✓: die Basis trägt die elf Skalare ✓; eine Revision ist erst nötig, wenn eine spätere
  Welle eine Scope-Zeile **verbessert** ✓ (`scripts/release/revise_wave_e_pin.py` ✓ + Owner-Review ✓).
