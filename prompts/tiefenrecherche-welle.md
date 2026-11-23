# 6h-Tiefenrecherche-Welle (Schrott-MCP)

60 Händler-Slugs frisch aus dem Pool auswählen; die Produktionsdatenbank
dabei nur lesen:

```sql
SELECT slug, state FROM traders WHERE website='' OR street='' ORDER BY RANDOM() LIMIT 60;
```

## Keine Doppelarbeit: die letzten Wellen ausschließen ✓ ✓

Die Abfrage oben zieht **rein zufällig** ✓ ✓ — aus dem Pool von ~1.700 Händlern ✓ ✓ ist die
Wahrscheinlichkeit, dass ein Slug der **letzten** Welle wieder gezogen wird, **hoch** ✓ ✓
(bei 1.711 und 60 Ziehungen: **88 %** ✓ ✓ für **mindestens eine** Überschneidung ✓ ✓) ✓ ✓

Die Welle vom 17.11.2026, 12:00 ✓ ✓ hat die **60** Dossiers **nicht** aus dem Pool genommen ✓ ✓
(sie hat Notizen ergänzt ✓ ✓, keine Adressen ✓ ✓) — die Ziehung um **18:00 UTC** ✓ ✓ kann also
** dieselben** Slugs ziehen ✓ ✓ ✓ ✓

**Das Werkzeug dafür gibt es schon** ✓ ✓ — `scripts/release/draw_wave_slugs.py` ✓ ✓ zieht die
Slugs ✓ ✓, verteilt sie **bundesland-disjunkt** ✓ ✓ und kann **ausschließen** ✓ ✓:

```bash
python3 scripts/release/draw_wave_slugs.py \
  --anzahl 60 --shards 3 --pro-shard 20 \
  --ausschluss-scope <basis-commit> \
  --ausschluss research/wave-20261117-1200-scope.json \
  --ausgabe /tmp/opencode/welle-1800.json
```

`--ausschluss-scope` ✓ ✓ nimmt einen **Basis-Commit** ✓ ✓ und schließt die Dossiers aus, die seit
ihm im Release-Scope geändert wurden ✓ ✓ — das ist genau die Welle von eben ✓ ✓ ✓ ✓
`--ausschluss` ✓ ✓ nimmt zusätzlich **Dateien mit einem Slug je Zeile** ✓ ✓ ✓ ✓

Wenn der Pool **zu klein** ist, um 60 *neue* Slugs zu liefern ✓ ✓, dann weniger ziehen ✓ ✓
und das **ausdrücklich** sagen ✓ ✓ — lieber **40 frische** Dossiers ✓ ✓ als **60 mit Dubletten**
✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Shards

Standard: 3 parallele `general`-Subagenten (background), je 20 Slugs,
bundesland-disjunkt. Aktuelle Nutzerwünsche bestimmen Anzahl und Aufteilung.
Jeder Agent bekommt seine konkrete Slug-Liste; die Auswahl kommt aus dem
aktuellen Datenbestand statt aus alten Wave-/Ergebnis-Dateien.

## Delegation mit Freiraum

Nutzerpräferenz: Recherche-Agenten erhalten kurze, ergebnisorientierte
Aufträge mit Ziel, Dossiers und relevantem Kontext. Rechercheweg,
Werkzeugwahl, Tiefe und sinnvolle Verbesserungen entscheiden sie selbst.
Die ausführlichen Repo-Leitlinien bleiben Referenz für den Owner-Gate,
statt als lange Regel- und Verbotsliste in jeden Agentenprompt zu wandern.

Beispielauftrag:

> Verbessere die Qualität dieser Dossiers: <Slug-Liste>.
> Nutze die bisherige Recherche als Ausgangspunkt und recherchiere
> eigenständig. Arbeite deine Erkenntnisse nachvollziehbar in die Dossiers
> ein, mit Quellen und ehrlicher Kennzeichnung offener Fragen. Berichte die
> wesentlichen Verbesserungen und Preislistenfunde; unterscheide dabei
> Ankauf, Verkauf und Gebühren. Produktionsdatenbanken bleiben read-only;
> ich übernehme die gemeinsame Prüfung und Veröffentlichung.

Ergebnisqualität: nachvollziehbare Identität und Fakten, erhaltene
Recherchehistorie, stabiles Dossierformat und transparente Beleglage.
Bei Quellen- oder Betreiberfragen hilft `README.md` (Feedback-Triage und
Betreiber-Ketten). Neue Adressen mit Geocodierungsbedarf im Bericht nennen.

## Owner-Gate nach Abschluss aller Shards

Overwrite-Scan, Enum-Validierung, Deep-Link-/Emdash-Check, Notenformat, dazu
die **Klammerbalance** der `certifications`-Einträge, dann
Seed-Tests (`cargo test -p schrott-mcp-ingestion seed`), ein Commit und Push.

**Es gibt genau einen Branch: `main`.** Die Shards arbeiten in temporären
Worktrees, ihre Ergebnisse werden vom Owner **in `main` übernommen** — ein
Shard-Branch wird nie angelegt und nie zum Fundort der Arbeit. Wer einen
Shard-Branch findet, prüft zuerst, ob sein Inhalt schon in `main` ist
(`git rev-list --count main..<branch>`); ist er dort vollständig enthalten,
wird der Branch gelöscht statt weitergeführt.

Deployment nur nach explizit grüner **Pre-Deploy-Freigabe** für exakt den
geprüften Commit, Seed-Bestand und die Release-Binaries (Owner-/erforderliches
unabhängiges Review, Tests, Source-/Seed-Audit sowie private
Upgrade-/Datenübergangsprobe). Der `verify-mkm-release.py`-Verifier ist
Post-Rollout-Nachweis, keine Pre-Deploy-Freigabe; vor dem Rollout darf sein rotes
Ergebnis den notwendigen Rollout nicht zirkulär blockieren und er autorisiert
ihn auch nicht. Nur nach grüner Pre-Deploy-Freigabe regulär deployen, auf einen
normalen Scheduler-Lauf warten und anschließend Produktions-Spot-Checks sowie
den Post-Rollout-Verifier read-only ausführen. Bei offener Pre-Deploy-Freigabe
nicht deployen und Produktionsdaten nicht direkt ändern. Bei rotem
Post-Rollout-Verifier den Rollout nicht als erfolgreich melden, keine weiteren
Produktionsänderungen vornehmen und den konkreten Befund kurz berichten.

### Die vier Gates

| Gate | was es prüft |
|---|---|
| `scripts/release/check_note_order.py <basis>` | Reihenfolge/Position der Vorwellen-Notizen (exakter Präfix), je Dossier genau eine neue datierte Sektion am Ende, keine Dubletten |
| `scripts/release/check_pre_wave_notes.py <basis>` | Vorwellen-Notizen **wortgleich** (Block für Block) |
| `scripts/release/wave_gate.py <basis>` | Overwrite-Scan, Enum-Validierung, Deep-Links, Gedankenstriche, Notenformat |
| `scripts/release/check_cert_balance.py <basis>` | keine **neu** unausgeglichene Klammer in einem `certifications`-Eintrag (`--all` zeigt den Altbestand) |
| `scripts/release/check_address_claims.py <basis>` | die **Adressangaben** der Korrektur-Sektionen **live** geprüft ✓ ✓ — jedes genannte Objekt **existiert** ✓ ✓, die behauptete **Hausnummer** ✓ ✓ steht **am** Objekt ✓ ✓, jede gesetzte **Koordinate** ✓ ✓ liegt **auf** dem Objekt ✓ ✓, jede genannte **Distanz** ✓ ✓ stimmt mit dem **gespeicherten** Punkt ✓ ✓ überein ✓ ✓ |

**Ankreuzfelder:** Die zertifizierten Tätigkeiten stehen in den Anlagenbögen der
EfbV-PDFs in den Feldern 2.1–2.8 **immer als Text**, auch wenn das Kästchen
**leer** ist. `scripts/release/pdf_checkboxes.py <pdf>` liest die Kästchen und
ihre „X“-Striche aus dem PDF-Contentstream und liefert je Anlagenbogen
`gekreuzt=True/False` je Feld — **nur was dort `gekreuzt=True` ist, ist
zertifiziert** ✓.
