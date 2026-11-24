# Rollout 24.11.2026 — Kandidat `5fddbe37` (die Welle vom 20:00)

**Kandidat:** `5fddbe37122aad8077db05799fb0dd22f2b710f8`
**Tree:** `df3bfba16f13eed4b49bcab49a1e0200973d9d8d`
**Basis:** `077c6fba` (der Rollout der Welle vom 18:00)
**Rollout:** **2026-11-24 21:25:17 UTC**
**Backup:** `/root/Documents/schrott-mcp-rollout-backups/rollout-5fddbe37-20261124T212449Z`

| Feld | Wert |
|---|---|
| `release_source_commit` | `5fddbe37122aad8077db05799fb0dd22f2b710f8` |
| `release_source_tree` | `df3bfba16f13eed4b49bcab49a1e0200973d9d8d` |
| Binary `schrott-mcp-server` | `0361257183b722322b87d5241d3d6fe89c1a0e2ae35d599763d2d6430c5904d4` |
| Binary `schrott-mcp-query-worker` | `b5d338da2c4cd96caa206fba33ae1bbde846cacb0529018045fafa92bc358379` |
| Workspace-Tests | **13** Suiten · **858** passed · **0** failed · **13** ignored |
| Der Dienst **vor** dem Rollout | `active` (Start `Tue 2026-11-24 20:00:06 UTC`) |
| `production_write` | **False** |
| Das **Backup** von `public.db` | 115580928 Bytes · sha256 `9a0afe76ec59d1929d7d985534c622e7f2fa294c18b1e8a34c5843ab754b4717` |
| Der **Scheduler-Lauf** ✓✓ nach dem Neustart ✓✓ | **2026-11-24T21:25:06** ✓✓ · `ok` ✓✓ · „Katalog aktuell, 131 Preise übernommen, 0 Händler-Fehler“ ✓✓ |
| Die **Zielzeilen** ✓✓ | **60** ✓✓ |
| Die **Punkte entfernt** ✓✓ | **46** ✓✓ |
| Die **Punkte noch da** ✓✓ | **14** ✓✓ |
| Die **Zählwerte** ✓✓ | traders **3945** ✓ · materials **83** ✓ · current_prices **4054** ✓ · `user_version` **13** ✓ ✓ |
| Die **Fremdschlüssel** ✓✓ | **24** ✓✓ |

## Was ausgerollt wurde ✓ ✓

Die **Welle vom 24.11.2026, 20:00** ✓ ✓ — **sechzig** ✓✓ Dossiers ✓✓ ✓ ✓

| Wirkung ✓ ✓ | Zahl ✓ ✓ |
|---|---|
| **Punkt gesetzt** ✓ ✓ | **2** ✓ ✓ |
| **Punkt ersetzt** ✓ ✓ | **11** ✓ ✓ |
| **Punkt bleibt unverändert** ✓ ✓ | **1** ✓ ✓ |
| **Punkt entfernt** ✓ ✓ | **35** ✓ ✓ |
| **Kein Standortpunkt** ✓ ✓ | **11** ✓ ✓ |

**Die dreizehn** ✓✓ gesetzten **und** ✓✓ ersetzten Punkte ✓✓ stehen **mit**
✓✓ den **gemessenen** ✓✓ Koordinaten ✓✓ in der **Produktion** ✓✓ ✓ ✓

## Der **Handler** ✓✓

**Mit** ✓✓ diesem **Rollout** ✓✓ ging der **Gerwischer**-Handler ✓✓
**live** ✓✓ — er liest die **neue** ✓✓ Seite ✓✓ als **Annahmenquelle** ✓✓
(**neun** ✓✓ Annahmen ✓✓, **null** ✓✓ Preise ✓✓, **fünf** ✓✓ **laut** ✓✓
**übersprungene** ✓✓ Karten ✓✓) ✓ ✓

## Der **Zyklus** ✓✓

**Drei** ✓✓ **Läufe** ✓✓ waren **nötig** ✓✓ ✓ ✓ Der **erste** ✓✓ scheiterte an
einem **vollen** ✓✓ **Temporaerverzeichnis** ✓✓ ✓ ✓ Der **zweite** ✓✓ am
**Format**-**Gate** ✓✓ ✓ ✓ Der **dritte** ✓✓ ist **grün** ✓✓ ✓ ✓
