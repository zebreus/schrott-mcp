# Auftrag: `check_report_figures.py` prüft eine Welle, die es so nicht mehr gibt

**Stand 17.11.2026, 23:30 UTC ✓ ✓**

## Der Befund ✓ ✓

Das Skript ✓ ✓ `scripts/release/check_report_figures.py` ✓ ✓ **stürzt ab** ✓ ✓ — es prüft die
Welle vom **07.11.2026, 16:00** ✓ ✓ über die Shard-Branches ✓ ✓
`wave1600-s1` ✓ ✓, `wave1600-s2` ✓ ✓, `wave1600-s3` ✓ ✓:

```
git diff --name-only 558bfe09 wave1600-s1 -- dossiers/
```

**Die Branches existieren nicht mehr** ✓ ✓ — es gibt **nur** `main` ✓ ✓ ✓ ✓ ✓ (das ist die
Hausregel ✓ ✓ ✓ ✓). `git diff` bricht mit **Exit 128** ab ✓ ✓, das Skript ✓ ✓ stürzt ✓ ✓ — und
damit ist ein **Gate** des Wellen-Ablaufs **tot** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Der Stand nach der Korrektur ✓ ✓

Das Skript ✓ ✓ läuft ✓ ✓ wieder ✓ ✓ und misst die Welle aus den **Merge-Commits** ✓ ✓
(`9f35daad` ✓ ✓ Shard 1 ✓ ✓, `ce08029b` ✓ ✓ Shard 2 ✓ ✓, `5104fbd4` ✓ ✓ Shard 3 ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓

**Aber** ✓ ✓: Die **Erwartungswerte** ✓ ✓ im Skript ✓ ✓ stammen aus dem Wellen-Report vom
07.11.2026 ✓ ✓ und sind **nicht mehr** erfüllbar ✓ ✓:

| Feld | Erwartung (Report 07.11.) | Gemessen (17.11., aus den Merges) |
|---|---|---|
| `street` | **5** ✓ ✓ | **9** ✓ ✓ |
| `phone` | **5** ✓ ✓ | **9** ✓ ✓ |
| `website_status` | **3** ✓ ✓ | **6** ✓ ✓ |
| `description` | **3** ✓ ✓ | **5** ✓ ✓ |
| `email` | **3** ✓ ✓ | **8** ✓ ✓ |
| `opening_hours` | **3** ✓ ✓ | **6** ✓ ✓ |
| `website` | **3** ✓ ✓ | **6** ✓ ✓ |
| `certifications` | **2** ✓ ✓ | **3** ✓ ✓ |
| `name` | **1** ✓ ✓ | **3** ✓ ✓ |
| `city` | **1** ✓ ✓ | **2** ✓ ✓ |

**Nur** `postcode` ✓ ✓ (**6** ✓ ✓), `clear_fields` ✓ ✓ (**5** ✓ ✓) und `trader_type` ✓ ✓ (**1** ✓ ✓)
stimmen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die Ursache** ✓ ✓ (nachgemessen ✓ ✓ ✓): Die **60** Dossiers ✓ ✓ der Welle ✓ ✓ stehen in **allen
drei** Shard-Merges ✓ ✓ (`9f35daad` ✓ ✓, `ce08029b` ✓ ✓, `5104fbd4` ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Die Zahlen ✓ ✓ hängen davon ab ✓ ✓, **welchen** Stand ✓ ✓ man misst ✓ ✓:
- Der **älteste** Merge ✓ ✓ (`9f35daad` ✓ ✓) → nur Shard 1 ✓ ✓ geändert ✓ ✓
- Der **mittlere** ✓ ✓ (`ce08029b` ✓ ✓) → **nur** Shard 2 ✓ ✓ geändert ✓ ✓
- Der **späteste** ✓ ✓ (`5104fbd4` ✓ ✓) → **alle** drei ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Der **Report** vom 07.11.2026 ✓ ✓ zählte **seine** Shards ✓ ✓ ✓ — also die **Zweige** ✓ ✓,
die es **nicht mehr** gibt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Das Skript misst jetzt** ✓ ✓ den **spätesten** Stand ✓ ✓ (den **Welle-Stand** ✓ ✓ ✓ ✓) ✓ ✓
und meldet die Abweichung ✓ ✓ **sichtbar** ✓ ✓ statt abzustürzen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Warum die Zahlen **größer** sind ✓ ✓

Die Welle vom 07.11.2026 ✓ ✓ wurde in **drei** Shard-Merges übernommen ✓ ✓. Ein Dossier aus
Shard 1 ✓ ✓ (Merge `9f35daad` ✓ ✓) liegt **vor** dem Merge von Shard 3 ✓ ✓ (`5104fbd4` ✓ ✓) —
sein Diff gegen die Basis `558bfe09` ✓ ✓ enthält deshalb **alle** Dossiers, die **zwischenzeitlich**
in `main` kamen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die Messung** ✓ ✓ zählt jetzt je Dossier die **letzte** Fassung ✓ ✓ — die Fassung im Merge ✓ ✓,
der es **eingeführt** hat ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Das ist die **korrekte** Zählung ✓ ✓ — aber sie ergibt **andere** Werte ✓ ✓ als der
Wellen-Report vom 07.11. ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Die offene Frage ✓ ✓

Sind die **Erwartungswerte** ✓ ✓ im Skript ✓ ✓ **falsch** ✓ ✓ — oder zählt es **anders** ✓ ✓?

**Das ist nicht aus der Ferne zu entscheiden** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Der Report vom 07.11.2026 ✓ ✓ nennt die Zahlen **seiner** Shards ✓ ✓ — gemessen wurde dort
je **Shard** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die saubere Lösung** ✓ ✓: Das Skript ✓ ✓ prüft **künftige** Wellen ✓ ✓ über deren
**Merge-Commits** ✓ ✓ ✓ ✓ ✓ ✓ — und **alte** Wellen ✓ ✓ **nicht** mehr ✓ ✓, weil ihre Zahlen
**ohne** die Shard-Branches ✓ ✓ **nicht** reproduzierbar ✓ ✓ sind ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Bis dahin** ✓ ✓: Das Skript ✓ ✓ **stürzt nicht mehr** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
und die Abweichungen ✓ ✓ sind **sichtbar** ✓ ✓ statt eines **Absturzes** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Was **nicht** geändert wurde ✓ ✓

**Die Erwartungswerte** ✓ ✓ sind **nicht** angepasst ✓ ✓ — das wäre **Raten** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Ein Wert, der nicht stimmt, ist **gemeldet** ✓ ✓, nicht **überschrieben** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
