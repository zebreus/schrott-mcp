# Wave-E-Pin-Revision vom 28.10.2026, 00:00-Welle (Owner-Freigabe)

## Anlass

Der release-pinned Wave-E-Verifier prüft die elf Skalare des ursprünglichen Wave-E-Scopes
(60 Zeilen aus der Welle vom 11.10.2026) als **Regressionsschutz**: Die Werte dürfen sich
ohne ausdrückliche Owner-Revision nicht still ändern. Die 6h-Welle vom 28.10.2026, 00:00
hat **zwei** dieser 60 Zeilen verbessert:

| Slug | geänderte Skalare | neuer Wert |
|---|---|---|
| `he-dietzenbach-stalcant-schrotthandler` | `description` | „Einzelunternehmen Sebastian Traian Calda …“ (Handelsregister/Creditreform, Betreiberseite: Ankaufsarten, 350-kg-Regel, kostenlose Abholung) |
| `sn-grorohrsdorf-ot-bretnig-schrottaufbereitung-rosler` | `street`, `postcode`, `description` | „Gewerbering Nord 18“, „01900“ (OSM-Objekt **„Schrott Rösler“**, Efb-Zertifikat ZZPT00400159) |

Die erste Fassung des Pins (Stand 11.10.2026) ließ diese Felder leer, weil die Angaben damals
nicht belegt waren. Die neue Welle belegt sie mit Betreiber- bzw. Registerquellen.

## Owner-Entscheidung

Die neuen Werte sind **besser belegt** als der Pin-Stand vom 11.10.2026. Der Pin wird deshalb
**revidiert**, nicht umgangen:

- Die Revision ist als neue Basisdatei `scripts/wave-e-20261028-expectations.json` abgelegt
  (Fortschreibung von `scripts/wave-e-20261027-expectations.json`).
- `dossier_revision` zeigt auf den Welle-Commit `8fac180c`, `owner_review` auf diese Datei, und
  die Datei trägt einen `revision`-Block mit dem Nachweis je Zeile.
- Der release-pinned Verifier des Releases `8fac180` wird aus dieser revidierten Basis erzeugt.

## Byte-Vergleich der Basis

| Zeilen | Ergebnis |
|---|---|
| 57 | **byte-identisch** zu `scripts/wave-e-20261027-expectations.json` |
| 1 | unterscheidet sich **nur** in `notes_sha256` und `seed_hash` (`he-alsfeld-schrotthandel-entsorgung-delis`) - der Timeline dieser Zeile hat sich geändert, die Hashes werden je Release neu gepinnt |
| 2 | unterscheiden sich zusätzlich in den oben genannten Werten |

Die Basis ist damit **für sich allein** ein gültiges Manifest: alle 60 `notes_sha256` und
`seed_hash` stimmen mit dem Post-Seed-Stand des Releases `8fac180` überein.

## Nachweis

- `python3 scripts/verify-wave-e-postrollout-8fac180.py --data-dir target/predeploy-8fac180/data`
  → `ok: true`, 60 Zeilen, 0 Abweichungen (vor der Revision: 4 Abweichungen in genau diesen zwei
  Zeilen).
- `python3 scripts/test_verify_wave_e_postrollout-8fac180.py` → 8 Tests, OK.
- `python3 scripts/verify-release-delta-8fac180.py --data-dir target/predeploy-8fac180/data`
  → `ok: true`, 60 Zeilen.
- Die Wellen-Dossiers selbst sind in `research/wave-20261028-0000-report.md` und in den
  Dossiers `dossiers/he/he-dietzenbach-stalcant-schrotthandler.md` und
  `dossiers/sn/sn-grorohrsdorf-ot-bretnig-schrottaufbereitung-rosler.md` belegt; die Koordinate
  des zweiten Dossiers wurde unabhängig gegen eine Vorwärtssuche geprüft (0 m Abweichung).

Die Revision ist **kein** Freibrief: Jede weitere Änderung an den Wave-E-Zeilen bleibt rot und
verlangt wiederum eine ausdrückliche Owner-Revision mit Belegnachweis.
