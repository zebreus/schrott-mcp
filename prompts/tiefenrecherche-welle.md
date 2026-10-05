# 6h-Tiefenrecherche-Welle (Schrott-MCP)

60 Händler-Slugs frisch aus dem Pool auswählen; die Produktionsdatenbank
dabei nur lesen:

```sql
SELECT slug, state FROM traders WHERE website='' OR street='' ORDER BY RANDOM() LIMIT 60;
```

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

Overwrite-Scan, Enum-Validierung, Deep-Link-/Emdash-Check, Notenformat, dann
Seed-Tests (`cargo test -p schrott-mcp-ingestion seed`), ein Commit, Push,
Redeploy (beide Binaries), Prod-Spot-Checks. Kurz berichten.
