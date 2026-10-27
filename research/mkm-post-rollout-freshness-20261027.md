# MKM-Post-Rollout-Gate: Freshness-Prüfung gegen die Rollout-Untergrenze, 27.10.2026

## Befund

Nach dem Rollout `f40d1dbb` (Neustart 27.10.2026 09:47:00 UTC) ist
`scripts/verify-mkm-release.py --since 2026-10-27T09:47:00Z` **rot**, mit genau einem
Befund:

```
"quote provenance/date/freshness mismatch"
```

Mit der vorherigen Rollout-Untergrenze 02:43:15 UTC war dasselbe Gate grün.

## Evidenz (read-only)

- Der MKM-Handler ist post-rollout **erfolgreich gelaufen**:
  `ingestion_steps` für `rp-ludwigshafen-mkm-metals` am 27.10.2026 11:17:42 bis 11:17:43,
  Status `ok`, `items_upserted=56`, Meldung „56 Preise übernommen, 5 Annahmen“.
- Der kanonische Abruf war HTTP 200 (der Verifier meldet keinen fehlenden Fetch).
- Der Article-Ledger wurde post-rollout fortgeschrieben:
  `source_article_snapshots` mit `owner=mkm-article-v1`,
  `source_url=https://mkm-metals.de/metallpreise`, `observed_at=2026-10-27T11:17:43`.
- Die **Preiszeilen** tragen dagegen weiterhin den älteren Zeitstempel:
  `SELECT MAX(observed_at) FROM prices` für den MKM-Händler ergibt
  `2026-10-27T05:29:47` (56 Zeilen), und alle 56 `current_prices`-Zeiger zeigen auf
  genau diese Zeilen.

## Ursache

Die Freshness-Bedingung des Verifiers lautet:

```python
fresh(row["observed_at"])  # julianday(observed_at) >= julianday(--since)
```

Sie prüft also den **Preiszeilen-Zeitstempel** gegen die Rollout-Untergrenze. Der Store
schreibt bei unverändertem Wert keine neue Preiszeile, sondern lässt die bestehende mit
ihrem ursprünglichen `observed_at` stehen (dokumentiertes Idempotenzverhalten: „Unchanged
effective payloads skip writes and preserve `updated_at`“). Weil die MKM-Kurse am 27.10.
2026 unverändert waren, blieb der Zeitstempel 05:29:47 stehen, während der Handler den
Vertrag post-rollout erneut geprüft und den Ledger fortgeschrieben hat.

Das ist **kein Rollout-Defekt und kein Datenverlust**: Der Abruf, die Vertragsprüfung, die
56 Preise und 5 Annahmen sowie der Ledger sind aktuell; nur der Zeitstempel der
unveränderten Preiszeilen liegt vor der Rollout-Untergrenze.

## Einordnung

Der `verify-mkm-release.py` ist laut Wellen-Anleitung ein **Post-Rollout-Nachweis** und
**keine Pre-Deploy-Freigabe**. Sein rotes Ergebnis darf einen notwendigen Rollout nicht
zirkulär blockieren; es autorisiert ihn aber auch nicht. Der Befund wird hier offen
berichtet und nicht durch eine Änderung am Verifier kaschiert.

## Optionen für die Folgeentscheidung (nicht in diesem Zyklus umgesetzt)

1. **Verifier präzisieren:** Freshness auf den Article-Ledger beziehungsweise den
   Step-Zeitstempel beziehen (der die aktuelle Beobachtung trägt) statt auf den
   Preiszeilen-Zeitstempel. Das ist die sachlich richtige Messung, ändert aber einen
   Freigabe-Vertrag und braucht ein eigenes Review.
2. **Unveränderte Preise im Preisvertrag berücksichtigen:** Der MKM-Handler könnte bei
   unverändertem Wert den bestehenden Preiszeilen-Zeitstempel mitziehen („last confirmed
   at“), damit der Zeitstempel die letzte Bestätigung und nicht die letzte Änderung
   abbildet. Auch das ist eine Datenmodell-Änderung mit eigenem Review.
3. **Bis zu einer Preisänderung akzeptieren:** Der Befund verschwindet automatisch, sobald
   sich ein MKM-Kurs ändert. Das ist keine Lösung, sondern nur die Erklärung des
   jetzigen Zustands.

Bis zur Entscheidung bleibt der Befund offen sichtbar; die MKM-Zeilen selbst sind durch
den Handler-Schritt, den HTTP-200-Abruf, den frischen Ledger und die 56 Current-Zeiger
belegt.
