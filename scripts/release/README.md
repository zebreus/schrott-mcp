# Release-Werkzeuge

Die Werkzeuge für den Release-Zyklus liegen hier **versioniert** (vorher nur unversioniert
unter `target/next-release/`, was den Ablauf nicht reproduzierbar machte).

## Ablauf

```bash
# 1) Kandidat bauen: Export des Commit-Baums, Release-Build, Tests, private Probe,
#    Wave-E- und Release-Delta-Erwartungen, Verifier pinnen
bash scripts/release/run_cycle.sh <commit> <short> /tmp/opencode/release-targets-next.json

# 2) Owner-Gate über die geänderten Dossiers (Overwrite-Scan, Enum-, Deep-Link-/
#    Emdash-Check, Notenformat)
python3 scripts/release/wave_gate.py HEAD

# 3) Rollout (Backup, Binärtausch, Neustart, Health-Check, Post-Rollout-Lauf)
#    RELEASE_SHORT ist Pflicht: ohne sie greifen Rollout und Nachprüfung in
#    fremde Verifier-Pins.
RELEASE_SHORT=<short> RELEASE_COMMIT=<commit> RELEASE_TREE=<tree> \
  RELEASE_TARGET_SLUGS=/tmp/opencode/release-targets-next.json \
  python3 scripts/release/rollout.py

# 4) Nachprüfung read-only
python3 scripts/verify-release-delta-<short>.py
python3 scripts/verify-wave-e-postrollout-<short>.py
python3 scripts/release/post_rollout_check.py
```

## Dateien

| Datei | Aufgabe |
|---|---|
| `run_cycle.sh` | Export → Build → Tests → private Probe → Erwartungs-Manifeste → Verifier-Pins |
| `make_wave_e_expectations.py` | release-gepinntes Wave-E-Erwartungs-Manifest aus der privaten Probe |
| `pin_digests.py` | verankert Manifest-/Digest-/Commit-Konstanten im Verifier und liest sie zurück |
| `install_wave_e_verifier.py` | erzeugt den release-gepinnten Wave-E-Verifier und seinen Vertragstest aus den Vorlagen (`scripts/*-SHORT.py`) |
| `rollout.py` | Backup, Binärtausch, Neustart, Health-Check, Post-Rollout-Lauf |
| `post_rollout_check.py` | read-only Nachprüfung nach dem Rollout |
| `wave_gate.py` | Owner-Gate der Recherche-Wellen (Dossier-Form) |
| `revise_wave_e_pin.py` | revidiert die Wave-E-Pin-Basis nach einer Welle, die Scope-Zeilen verbessert hat (mit Selbstprüfung gegen die private Probe) |
| `pin_pending_corrections.py` | pinnt das Gate `verify-pending-corrections.py` auf einen Release (eigene Erwartungsdatei + Gate-Kopie) |

## Grundsätze

- **read-only gegenüber der Produktion:** Die Werkzeuge schreiben nur in `target/` (Build- und
  Probe-Verzeichnisse) und in `scripts/` (Verifikationsartefakte). Produktionsdatenbanken werden
  ausschließlich mit `file:...?mode=ro` und `PRAGMA query_only=ON` gelesen; der Rollout schreibt
  ausschließlich über die regulären Binär- und Schema-Pfade der Anwendung.
- **Fail closed:** Ein Seed-Delta, das nicht vollständig aus den Dossiers erklärbar ist, bricht den
  Zyklus ab (`unexplained_changes`). Store-Migrationen prüfen ihre Population, bevor sie löschen.
- **Determinismus:** Erzeugte Manifeste enthalten absolute Pfade, damit derselbe Aufruf immer
  dasselbe byte-identische Ergebnis liefert.
- **Vor dem Rollout** steht die Pre-Deploy-Freigabe für **exakt** den geprüften Commit, den
  Seed-Bestand und die Release-Binaries. Nach dem Rollout wird der Post-Rollout-Verifier mit dem
  tatsächlichen Rollout-Zeitpunkt read-only ausgeführt.
