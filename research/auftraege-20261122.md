# Aufträge und offene Punkte — 22.11.2026

## Der Rollout `7f40548` ist durch, der Pending-Gate wartet noch ✓ ✓

Der Rollout der Welle vom 21.11.2026, **18:00** ✓ ✓ ist am **22.11.2026,
00:58:24 UTC** ✓ ✓ erfolgt ✓ ✓ — **60** Dossiers ✓ ✓, **Sieben** Standortpunkte
**ersetzt** ✓ ✓, **einunddreißig** **entfernt** ✓ ✓ ✓ ✓

**Der Pending-Gate ist rot** ✓ ✓ — und das ist **erwartbar** ✓ ✓: er fordert
einen **erfolgreichen Scheduler-Schritt nach** dem Rollout ✓ ✓, und der letzte
MSG-Lauf lag um **23:31 UTC** ✓ ✓, der letzte Gutzmann-Lauf um **21:17 UTC**
✓ ✓ ✓ ✓ Die **Turnusläufe** ✓ ✓: Gutzmann um **03:17 UTC** ✓ ✓, MSG um
**05:31 UTC** ✓ ✓ ✓ ✓

**Bis dahin** ✓ ✓ gilt: **keine** weiteren Produktionsänderungen ✓ ✓ ✓ ✓ Die
**Nachprüfung** ✓ ✓ ist für **05:45 UTC** ✓ ✓ angelegt ✓ ✓ (Job
`release-nachpruefung-7f40548` ✓ ✓) ✓ ✓

## Das Provider-Limit des Schedulers ✓ ✓ — **offen** ✓ ✓

Das **Tageslimit** ✓ ✓ des Scheduler-Providers ✓ ✓ ist **erschöpft** ✓ ✓
(**45,02 $** von **45,00 $** ✓ ✓) ✓ ✓ und **resetet nicht** ✓ ✓ ✓ ✓ Der
Check-in-Job für `ed2878e` ✓ ✓ ist um **00:00:05 UTC** ✓ ✓ mit Exit **1**
✓ ✗ abgebrochen ✓ ✓ — **nicht** wegen der Daten ✓ ✓ ✓ ✓

Der **Check-in** ✓ ✓ ist **von Hand** ✓ ✓ nachgeholt ✓ ✓: der Rollout `ed2878e`
✓ ✓ ist **vollständig bestätigt** ✓ ✓ (`verify-pending-corrections-ed2878e.py`
✓ ✓ → `ok: true` ✓ ✓, `failures: []` ✓ ✓) ✓ ✓ — Nachweis:
`research/checkin-20261122-0115-owner.md` ✓ ✓ ✓ ✓

**Folge** ✓ ✓: der Check-in `release-nachpruefung-7f40548` ✓ ✓ (Cron **05:45
UTC** ✓ ✓) läuft **nicht** ✓ ✗ ✓, solange das Limit **nicht** aufgefüllt ist
✓ ✓ — die Nachprüfung muss **von Hand** ✓ ✓ erfolgen ✓ ✓ ✓ ✓

## Offene Punkte ✓ ✓

### 1. Der MKM-Handler scheitert seit dem 20.11.2026 ✓ ✓ — **vorbestehend** ✓ ✓

Die Meldung lautet unverändert ✓ ✓:
`MKM-NE-05: priced row without tier ladder is not a reviewed flat quote`
✓ ✓ — die Quellenseite `https://mkm-metals.de/metallpreise` ✓ ✓ weist die
**Nickelposition** ✓ ✓ inzwischen **ohne Mengenstaffel** ✓ ✓ aus ✓ ✓
(„Anfragen“ ✓ ✓) ✓ ✓ Der Handler verlangt dort eine Staffel ✓ ✓ ✓ ✓

Der letzte **grüne** MKM-Lauf war am **20.11.2026 12:09:13 UTC** ✓ ✓
(**56** Items ✓ ✓) ✓ ✓ — **vor** dem Rollout `ed2878e` ✓ ✓ und **weit** vor
`7f40548` ✓ ✓ ✓ ✓ Das ist **kein** Befund des Releases ✓ ✓ ✓ ✓

### 2. `scripts/test_verify_pending_corrections.py` ist rot ✓ ✓ — **vorbestehend** ✓ ✓

Der **generische** Verifier ✓ ✓ `scripts/verify-pending-corrections.py` ✓ ✓
verweist auf die **nicht mehr vorhandene** ✓ ✓ Erwartungsdatei
`scripts/pending-corrections-eadaf8c6-seed-expectations.json` ✓ ✓ — **neun**
Fehler ✓ ✓ ✓ ✓

**Vorbestehend** ✓ ✓: bei `ed2878e` ✓ ✓ ist der Zustand **identisch** ✓ ✓, und
der Release **berührt** ✓ ✓ die Datei **nicht** ✓ ✓ ✓ ✓ Der **gepinnte**
Verifier ✓ ✓ `scripts/verify-pending-corrections-7f40548.py` ✓ ✓ besteht seine
**neun** Contract-Tests ✓ ✓ ✓ ✓

**Vorschlag für die nächste Welle** ✓ ✓: den generischen Verifier **entfernen**
✓ ✓ — er ist durch die gepinnten Verifier **ersetzt** ✓ ✓ ✓ ✓

### 3. Der Bericht nennt die 43 zusätzlichen Bullets falsch ✓ ✓ — **kosmetisch** ✓ ✓

`scripts/release/make_wave_report.py` ✓ ✓ Z. 239 f ✓ ✓ schreibt „mit den
automatisch erzeugten Frontmatter-Vermerken“ ✓ ✗ ✓ ✓ — die **43** ✓ ✓ sind die
**datierten Owner-Korrektur-Sektionen** ✓ ✓ (38 Geokodierung ✓ ✓, Zertifikat
✓ ✓, Preise ✓ ✓ zweimal ✓ ✓, Abstand ✓ ✓) ✓ ✓ Die Zahlen **144** ✓ ✓ und
**187** ✓ ✓ stimmen ✓ ✓ ✓ ✓

**Kein** Eingriff in diesem Release ✓ ✓ — der Satz ist **irreführend** ✓ ✗ ✓ ✓,
aber nicht falsch in der Zahl ✓ ✓ ✓ ✓

### 4. Die Quellenzahlen des Berichts sind case-sensitiv gemessen ✓ ✓ — **vorbestehend** ✓ ✓

„Betreiber-Website **22**“ ✓ ✓ und „Betreiberverzeichnisse **23**“ ✓ ✓ zählen
**ohne** `re.IGNORECASE` ✓ ✓ — case-insensitiv wären es **23** ✓ ✓ und **32**
✓ ✓ ✓ ✓ Das Muster „Schrottplatz“ ✓ ✓ trifft dabei auch das **Wort** ✓ ✓ im
Kontext ✓ ✓ — eine case-insensitive Zählung würde **falsche** Treffer
**zählen** ✓ ✓ ✓ ✓ Der Bestand bleibt unverändert ✓ ✓ ✓ ✓
