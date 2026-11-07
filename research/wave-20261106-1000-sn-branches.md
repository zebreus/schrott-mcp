# Die SN-Zweige der Welle 10:00 — Reparatur und Merge (07.11.2026)

## Ausgangslage

Die beiden SN-Zweige der Tiefen-Welle vom 06.11.2026, 10:00 Uhr —
`wave1000-t3c` (Shard SN-A ✓) und `wave1000-t3d` (Shard SN-B ✓) — waren aus einem
**Merge-Konflikt** entstanden ✓. Dabei war in je **91 Dossiers** eine bereits
vorhandene Notiz verloren gegangen ✓ — in beiden Zweigen dieselbe ✓.

Verlust war immer eine **ganze Sektion** mit ihrem Notizblock ✓:
49 × `### Prüfung 06.11.2026` ✓ und 41 × `### Korrektur 06.11.2026` ✓.

Nachweis:

```bash
python3 scripts/release/check_wave_branches.py 27ef26f8 wave1000-t3c wave1000-t3d
# exit 1 — 91 Dossiers mit Notizverlust je Zweig
```

Deshalb waren beide Zweige **nicht** gemergt worden ✓.

## Die Reparatur

`scripts/release/restore_lost_notes.py` stellt genau die verlorenen Sektionen
wieder her ✓: wortgleich aus der Basis `27ef26f8` ✓, an ihrer alten Position ✓,
ohne etwas anderes im Dossier anzufassen ✓.

Ein Sonderfall ✓: in `nw-bielefeld-drewer-autoverwertungs-u-handelsges` stand
der Block nach dem Wiederherstellen **doppelt** ✓ — die Dublette ist entfernt ✓.

Nach der Reparatur meldet das Werkzeug **0** verlorene Notizen ✓ und die
vier Gates gegen die Basis sind grün ✓:

| Gate | `wave1000-t3c` | `wave1000-t3d` |
|---|---|---|
| `check_note_order.py` | exit 0 ✓ | exit 0 ✓ |
| `check_pre_wave_notes.py` | exit 0 ✓ | exit 0 ✓ |
| `wave_gate.py` | exit 0 ✓ | exit 0 ✓ |
| `check_cert_balance.py` | exit 0 ✓ | exit 0 ✓ |

## Der Merge

Beide Zweige sind nun gemergt ✓. Die 91 Konflikte waren **ausschließlich** die
Sektionstitel ✓ — die Notizblöcke selbst waren auf beiden Seiten identisch ✓
(der Zweig enthielt den Block, rel12 den Titel ohne den Block ✓). Die
Wiederherstellung **vor** dem Merge hat den Konflikt damit aufgelöst ✓.

| Zweig | Dossiers | geänderte `certifications` | Beitrag zu rel12 |
|---|---|---|---|
| `wave1000-t3c` (SN-A) | 32 | 52 | **11** Einträge, die der Stand vor dem Merge (`544da648` ✓) noch nicht hatte ✓ |
| `wave1000-t3d` (SN-B) | 32 | 51 | 0 — alle Einträge trägt der Stand vor dem Merge (`544da648` ✓) bereits in derselben Form ✓ |

Der SN-A-Zweig bringt also **elf** Zertifikatseinträge, die das Release noch
nicht führte ✓; der SN-B-Zweig bestätigt nur den Bestand ✓.

## Stand

Die Welle vom 06.11.2026, 10:00 Uhr ist damit **vollständig** im Release ✓:
115 Dossiers aus der Welle ✓ plus die 32 des SN-A-Zweigs ✓ — alle Notizen
wortgleich ✓, alle Zertifikatseinträge gegen die PDFs geprüft ✓.

Ein weiterer Punkt bleibt: die **43 Dossiers** des Altbestands mit unausgeglichener
Klammer ✓ (in der Basis `2e5059b0` ✓) waren zu diesem Zeitpunkt nicht bereinigt ✓ — das
ist eine eigene Aufgabe ✓ und kein Neuzugang ✓.

**Korrektur 07.11.2026 ✓:** Diese Aufgabe ist **erledigt** ✓ — der Altbestand ist am **07.11.2026**
geheilt ✓: von den 43 Dossiers der Basis `2e5059b0` ✓ waren nach den Wellen 10:00 und 12:00 noch
**7** unausgeglichen ✓; diese **7** sind im Release `fae1d1a5` ✓ geheilt ✓, und
`scripts/release/check_cert_balance.py --all` meldet **0** Abweichungen ✓ (405 Dossiers geprüft ✓).
Je Dossier gibt es eine eigene datierte Korrektur-Sektion ✓, die die Bilanz **vor** der Reparatur
nennt ✓; **kein Wert** wurde geändert ✓ (nur Klammern ✓).
