# Vorschlag: Freshness-Maßstab in den MKM- und MSG-Verifiern (27.10.2026)

**Status: Vorschlag, nicht umgesetzt.** Die Verifier sind Freigabe- und Nachweisinstrumente;
eine Änderung an ihnen braucht ein eigenes Review. Dieser Notiz liegt die exakte Änderung
mit Risikoanalyse bei, damit die Entscheidung belegbar ist.

## Beobachtung

Nach jedem Rollout sind `verify-mkm-release.py` und der MSG-Teil von
`verify-pending-corrections.py` rot, sobald die Rollout-Untergrenze über dem Zeitpunkt der
letzten **Preisänderung** liegt, obwohl die Handler post-rollout erfolgreich gelaufen sind:

- MKM: Handler-Schritt 11:17:42 UTC (HTTP 200, 56 Preise, `ok`), Ledger-Snapshot
  `source_article_snapshots` auf 11:17:43 fortgeschrieben, aber die Preiszeilen tragen
  weiterhin `observed_at` 05:29:47 (vor der Rollout-Untergrenze 09:47 bzw. 16:04).
- MSG: Handler-Schritt 16:19:47 UTC (124 Preise, `ok`), Preiszeilen vom 15:05 — dieselbe
  Situation.

Befund des Verifiers:

```python
if not ... or not fresh(row["observed_at"]) ...
```

mit `fresh(at) = julianday(at) >= julianday(--since)`.

## Ursache

Der Store legt bei **unverändertem** Wert keine neue Preiszeile an: Die bestehende Zeile
bleibt mit ihrem ursprünglichen `observed_at` stehen (`docs/trader-seed.md`: „Unchanged
effective payloads skip writes and preserve `updated_at`“). Der Zeitstempel einer Preiszeile
ist damit der Zeitpunkt der **letzten Änderung**, nicht der **letzten Beobachtung**. Die
Handler beobachten die Preise aber sehr wohl erneut: Sie schreiben bei jedem erfolgreichen
Lauf den Article-Ledger (`source_article_snapshots` mit `observed_at`) und legen die
Artikelmanifeste darin ab.

Der Verifier misst also „Wurde der Preis **geändert** nach dem Rollout?“ und behauptet
dadurch implizit „Wurde der Preis **beobachtet** nach dem Rollout?“. Genau an dieser
Stelle entstehen die zwei roten Befunde.

## Was fachlich richtig wäre

Die belastbare Aussage ist: **Der Handler hat die Preise nach dem Rollout erneut beobachtet,
und die veröffentlichten Werte entsprechen dieser Beobachtung.** Das ist eine Aussage über
zwei Dinge:

1. Der Ledger-Snapshot ist nach der Rollout-Untergrenze (der Handler hat die Seite
   erneut gelesen und das Manifest neu geschrieben).
2. Die Preiszeile, auf die der Ledger zeigt, ist die gültige Wiedergabe dieser Beobachtung
   (Wert, Einheit, Material, Quelle stimmen mit dem Manifest überein).

Punkt 2 wird bereits geprüft (Preiszeile ↔ Manifest-Übereinstimmung, `numeric`/`requests`
= 56/5, Material-Schlüssel, `source_type`, `published`). Fehlt ist nur die Beobachtungszeit
in der richtigen Quelle.

## Vorgeschlagene Änderung (beide Verifier)

 statt

```python
not fresh(row["observed_at"])
```

 eine Prüfung, die die Beobachtungszeit des Ledgers heranzieht:

```python
observed_fresh = fresh(row["observed_at"]) or fresh(ledger["observed_at"])
```

 und in die Fehlerbedingung `observed_fresh` statt `fresh(row["observed_at"])`.

Der Ledger ist im Verifier bereits geladen (`ledger = public.execute(...)` mit
`ORDER BY s.observed_at DESC LIMIT 1`), die Variable `ledger["observed_at"]` ist also
verfügbar; es kommt kein zusätzlicher Datenbankzugriff hinzu.

## Risikoanalyse

| Risiko | Bewertung |
|---|---|
| Der Verifier wird „weicher“ und übersieht veraltete Preise | **Gering.** Die Preise müssen weiterhin im Ledger-Material frisch sein; der Ledger selbst wird nur bei einem erfolgreichen Handler-Lauf fortgeschrieben, und der Verifier prüft weiterhin, dass der Schritt `ok` war, 56 Items enthält und der kanonische Abruf HTTP 200 war. Ein Handler, der gar nicht läuft, liefert keinen frischen Ledger, und der Verifier bleibt rot. |
| Ein Handler schreibt den Ledger, ohne die Preise zu prüfen | Der Ledger entsteht aus derselben Response wie die Preise; ein „grüner“ Ledger bei „roten“ Preisen würde am Material-/Wertabgleich im selben Verifier auffallen. |
| Zwei Verifier unterschiedliche Freshness-Regeln | **Muss vermieden werden:** dieselbe Änderung gehört in `verify-mkm-release.py` und in den MSG-Teil von `verify-pending-corrections.py`, sonst bedeutet dasselbe Signal in beiden Verifiern Unterschiedliches. |
| Erwartungsdateien und Pins | Die Änderung betrifft nur Prüflogik, nicht Erwartungswerte; die `MANIFEST_SHA256`-Pins bleiben gültig, die Prüflogik selbst ist aber Teil des geprüften Artefakts und braucht ein eigenes Review (inklusive der Vertragstests). |

## Vorgehen

1. Beide Verifier um `observed_fresh` erweitern, jeweils mit einem neuen Vertragstest, der
   die zwei Fälle trennt: Ledger frisch + Preiszeile alt **muss** grün sein (das ist der
   Auslöser), Ledger alt + Preiszeile alt **muss** rot sein (der Schritt lief nicht).
2. Die Änderung als eigenen Commit mit eigenem Review behandeln, nicht als Bestandteil
   eines Daten-Releases.
3. Nach dem Review die beiden Verifier erneut gegen die laufende Produktion prüfen und das
   Ergebnis im Check-in berichten.

## Ehrliche Einordnung

Der derzeitige rote Befund ist **kein Datenverlust und keine Produktionsstörung**: Die
Handler laufen, die Ledger sind frisch, die Preise sind unverändert gültig. Der Befund
misst den falschen Zeitbezug. Er ist aber auch kein Grund, den Verifier ungeprüft
umzubauen; deshalb diese Notiz mit exakter Änderung statt einer stillen Anpassung.
