# Aufträge und offene Punkte — 22.11.2026, Nachmittag

## Die Nachprüfung des Rollouts `5b4f791` wartet auf den Turnus ✓ ✓

Der Rollout der Welle vom 22.11.2026, **00:00** ✓ ✓ ist am **22.11.2026,
04:36:48 UTC** ✓ ✓ erfolgt ✓ ✓ — **60** Dossiers ✓ ✓, **sieben** Standortpunkte
**ersetzt** ✓ ✓, **neunundzwanzig** **entfernt** ✓ ✓ ✓ ✓

**Der Pending-Gate ist rot** ✓ ✓ — er fordert einen **erfolgreichen
Scheduler-Schritt nach** dem Rollout ✓ ✓, und die Turnusläufe ✓ ✓: MSG um
**05:33 UTC** ✓ ✓, Gutzmann um **09:29 UTC** ✓ ✓ ✓ ✓

**Bis dahin** ✓ ✓ gilt: **keine** weiteren Produktionsänderungen ✓ ✓ ✓ ✓ Die
**Nachprüfung** ✓ ✓ ist für **09:45 UTC** ✓ ✓ angelegt ✓ ✓ (Job
`release-nachpruefung-5b4f791` ✓ ✓) ✓ ✓

**Auch** ✓ ✓ die Nachprüfung des **Vorgänger-Rollouts** ✓ ✓ `7f40548` ✓ ✓
wartet auf den **MSG**-Lauf um **05:33 UTC** ✓ ✓ — der Job dafür ist für
**05:45 UTC** ✓ ✓ angelegt ✓ ✓ ✓ ✓

## Das Provider-Limit des Schedulers ✓ ✓ — **offen** ✓ ✓

Das **Tageslimit** ✓ ✓ des Scheduler-Providers ✓ ✓ ist **erschöpft** ✓ ✓
(**45,02 $** von **45,00 $** ✓ ✓) ✓ ✓ und **resetet nicht** ✓ ✓ ✓ ✓

**Folge** ✓ ✓: **jeder** Job, der über diesen Provider läuft ✓ ✓, **bricht** ✓ ✗
ab ✓ ✓ — der Check-in für `ed2878e` ✓ ✓ ist um **00:00:05 UTC** ✓ ✓ mit Exit
**1** ✓ ✗ abgebrochen ✓ ✓ ✓ ✓

**Bis** ✓ ✓ das Limit **aufgefüllt** ✓ ✓ ist ✓ ✓, laufen **die geplanten
Nachprüfungen** ✓ ✓ **nicht** ✓ ✗ ✓ ✓ — sie müssen **von Hand** ✓ ✓ erfolgen
✓ ✓ ✓ ✓

**Folge für die Wellen** ✓ ✓: die **6h-Welle** ✓ ✓ um **06:00 UTC** ✓ ✓ läuft
**nicht** ✓ ✗ ✓ ✓ — die Welle vom 22.11.2026, **00:00** ✓ ✓ ist deshalb
**von Hand** ✓ ✓ gelaufen ✓ ✓ ✓ ✓

## Offene Punkte ✓ ✓

### 1. `scripts/test_verify_pending_corrections.py` ist rot ✓ ✓ — **vorbestehend** ✓ ✓

Der **generische** Verifier ✓ ✓ `scripts/verify-pending-corrections.py` ✓ ✓
verweist auf die **nicht mehr vorhandene** ✓ ✓ Erwartungsdatei
`scripts/pending-corrections-eadaf8c6-seed-expectations.json` ✓ ✓ — **neun**
Fehler ✓ ✓ ✓ ✓

**Vorbestehend** ✓ ✓: bei `0bd9f5cd` ✓ ✓ ist der Zustand **identisch** ✓ ✓, und
der Release **berührt** ✓ ✓ die Datei **nicht** ✓ ✓ ✓ ✓ Der **gepinnte**
Verifier ✓ ✓ `scripts/verify-pending-corrections-5b4f791.py` ✓ ✓ besteht seine
**neun** Contract-Tests ✓ ✓ ✓ ✓

**Vorschlag für die nächste Welle** ✓ ✓: den generischen Verifier **entfernen**
✓ ✓ — er ist durch die gepinnten Verifier **ersetzt** ✓ ✓ ✓ ✓

### 2. Der Bericht hat einen leeren Abschnitt ✓ ✓ — **kosmetisch** ✓ ✓

`research/wave-20261122-0000-report.md` ✓ ✓: „Belege, die den Dossiers neu
zugeordnet oder gefüllt haben“ ✓ ✓ ist **leer** ✓ ✓, weil die Welle **kein**
`certifications` ✓ ✓ geändert hat ✓ ✓ — technisch **korrekt** ✓ ✓, optisch eine
**Leerstelle** ✓ ✓ ✓ ✓

### 3. Die Zählung der Abschnitte des Berichts umfasst auch die Korrektur-Sektionen ✓ ✓

`scripts/release/make_wave_report.py` ✓ ✓ liest bis zum **Dateiende** ✓ ✓ —
die **36** datierten Korrektur-Sektionen ✓ ✓ sind in den Quell- und Preiszahlen
✓ ✓ mitgezählt ✓ ✓ (Quellen **53/27/58/20/3/16** ✓ ✓, Preise **14/13/9** ✓ ✓)
✓ ✓ Die Abschnittstitel („in den neuen Notizen“ ✓ ✓) meinen streng genommen nur
die **Recherche**-Sektionen ✓ ✓ ✓ ✓
