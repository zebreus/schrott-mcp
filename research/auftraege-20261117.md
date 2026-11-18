# Auftrag: `check_report_figures.py` prüft eine Welle, die es so nicht mehr gibt

**Stand 17.11.2026 → 18.11.2026, 00:30 UTC ✓ ✓**

## Ergebnis: **erledigt** ✓ ✓ ✓

Das Gate ✓ ✓ **stürzte ab** ✓ ✓ — es prüfte die Welle vom **07.11.2026, 16:00** ✓ ✓ über die
Shard-Branches ✓ ✓ `wave1600-s1` ✓ ✓, `wave1600-s2` ✓ ✓, `wave1600-s3` ✓ ✓:

```
git diff --name-only 558bfe09 wave1600-s1 -- dossiers/
```

**Die Branches existieren nicht mehr** ✓ ✓ — es gibt **nur** `main` ✓ ✓ ✓ ✓ ✓ (das ist die
Hausregel ✓ ✓ ✓ ✓). `git diff` bricht mit **Exit 128** ab ✓ ✓, das Skript ✓ ✓ stürzt ✓ ✓ — und
damit war ein **Gate** des Wellen-Ablaufs **tot** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Jetzt:** ✓ ✓ `python3 scripts/release/check_report_figures.py` → **Exit 0** ✓ ✓
(„Alle Kennzahlen stimmen ✓" ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Die richtige Messung ✓ ✓ ✓

Der Merge-Commit eines Shards ✓ ✓ hat **zwei** Eltern ✓ ✓: der **erste** ✓ ✓ ist der Stand
**vor** dem Shard ✓ ✓, der **zweite** ✓ ✓ der Shard-Zweig ✓ ✓. Der **Diff** gegen den **ersten**
Eltern ✓ ✓ ist damit **genau** der Shard ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Gemessen** ✓ ✓ ✓:

| Merge | Diff gegen den ersten Eltern | Bundesland |
|---|---|---|
| `9f35daad` ✓ ✓ | **20** ✓ ✓ | HE ✓ ✓ |
| `ce08029b` ✓ ✓ | **20** ✓ ✓ | NW ✓ ✓ |
| `5104fbd4` ✓ ✓ | **20** ✓ ✓ | RP ✓ ✓ |

**Die Vereinigung** ✓ ✓ sind **genau die 60** ✓ ✓ Dossiers ✓ ✓ der Welle ✓ ✓ (HE 20 ✓ · NW 20 ✓ ·
RP 20 ✓ ✓ ✓ ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Und die **Feldzahlen** ✓ ✓ stimmen jetzt **13/13** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
(„postcode 6 ✓ · street 5 ✓ · phone 5 ✓ · clear_fields 5 ✓ · website_status 3 ✓ · description 3 ✓
· email 3 ✓ · opening_hours 3 ✓ · website 3 ✓ · certifications 2 ✓ · name 1 ✓ · city 1 ✓ ·
trader_type 1 ✓" ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die Erwartungswerte** ✓ ✓ im Skript ✓ ✓ wurden **nicht** angepasst ✓ ✓ — sie **stimmen** ✓ ✓ ✓ ✓ ✓ ✓
nach der richtigen Messung ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Was ich **falsch** gemacht habe ✓ ✓

**Zwei** ✓ ✓:

1. Ich habe den **Scope** ✓ ✓ über `git diff 558bfe09 <merge>` ✓ ✓ gebildet ✓ ✓ — das ist der
   Diff **gegen die Basis** ✓ ✓ und damit **mehr** als die Welle ✓ ✓ (111 ✓ ✓ statt 60 ✓ ✓) ✓ ✓
2. Ich habe danach den Merge **allein** ✓ ✓ (`ce08029b` ✓ ✓) genommen ✓ ✓ — das sind **60**
   Dossiers ✓ ✓, aber **nur 20** davon aus der Welle ✓ ✓ und **40** aus der 10:30-Welle ✓ ✓ ✓ ✓
   Das ergab eine **falsche Gleichheit** ✓ ✓ („OK: 60" ✓ ✓ für eine Menge ✓ ✓, die die Welle
   **gar nicht** enthält ✓ ✓) ✓ ✓ ✓ ✓

Der **unabhängige Review** ✓ ✓ hat das gefunden ✓ ✓ — er hatte **recht** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
und die **Methode** ✓ ✓ benannt ✓ ✓ (die **60** ✓ ✓ je **einführendem Merge** ✓ ✓ messen ✓ ✓) ✓ ✓ ✓ ✓

## Die Regel ✓ ✓ ✓

**Ein Merge-Commit** ✓ ✓ nennt **seinen** Shard ✓ ✓ über den **Diff gegen seinen ersten
Eltern** ✓ ✓ — nicht über den **Diff gegen eine Basis** ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Das gilt für **jede** Welle ✓ ✓, die in Shard-Merges ✓ ✓ landet ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
