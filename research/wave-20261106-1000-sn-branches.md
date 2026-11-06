# Nicht gemergte Wellen-Zweige (SN) — Stand 06.11.2026

Die Zweige `wave1000-t3c` (SN-A) und `wave1000-t3d` (SN-B) enthalten die
Tiefenprüfung der **SN-Dossiers** ✓ (Zertifikatseinträge feldweise gegen die
PDFs geprüft ✓). Sie sind **nicht** in `rel12` gemergt — und dürfen es auch
noch nicht sein.

## Warum nicht

Beide Zweige sind aus einem **Merge-Konflikt** entstanden ✓. Dabei ist in
je **91 Dossiers** eine bereits vorhandene Notiz verloren gegangen ✓ — die
Notiz mit dem Vermerk zu den §-28-NachwV-Kennnummern ✓ (Beispiel
`by-abenberg-veg` ✓: „**Die §-28-NachwV-Kennnummern der Tätigkeiten „Handeln“
und „Makeln" (Anlage 1) …“ ✓).

Ein Merge würde diese Notizen **löschen** ✓ — das verletzt die Hausregel
(Vorwellen-Notizen bleiben wortgleich ✓) und würde 91 Dossiers ihrer
Recherchehistorie berauben ✓. Deshalb sind die Zweige nicht gemergt worden ✓.

Nachweis:

```bash
python3 scripts/release/check_wave_branches.py 27ef26f8 wave1000-t3c wave1000-t3d
# exit 1 — 91 Dossiers mit Notizverlust je Zweig
```

## Was die Zweige sonst enthalten

| Zweig | Dossiers | `certifications` geändert | neue Notizen |
|---|---|---|---|
| `wave1000-t3c` (SN-A) | 159 | 52 | 13 |
| `wave1000-t3d` (SN-B) | 159 | 51 | 12 |

Die Zertifikatseinträge selbst sind **nicht** im Konflikt — sie sind geprüft
✓. Der Verlust betrifft ausschließlich die **Notizzeilen** ✓.

## Wie sie weitergehen

1. Die verlorenen Notizen aus `27ef26f8` in den Zweigen wiederherstellen ✓ —
   ohne die übrigen Änderungen anzutasten ✓.
2. `check_wave_branches.py` muss **exit 0** liefern ✓.
3. Die Wellen-Sektion muss je Dossier am Timeline-Ende stehen ✓ (Hausregel ✓).
4. Erst dann mergen ✓ — mit einem eigenen Wellen-Commit ✓, nicht in den
   laufenden Release ✓ (ein Release ist ein eigener Commit ✓).
5. Für den Merge ist ein eigener Pre-Deploy-Durchlauf nötig ✓.

Bis dahin bleiben die Zweige liegen ✓ — die Dossiers des Pools sind damit
nicht verloren ✓: sie stehen im Repo ✓ und die 12 SN-Dossiers ohne
Register-Permalink sind ohnehin nur über die teure Namenssuche erreichbar ✓.
