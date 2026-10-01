# 6h-Tiefenrecherche-Welle (Schrott-MCP)

60 Händler-Slugs aus dem Pool ziehen (NUR LESEN, nie in die DB schreiben):

```sql
SELECT slug, state FROM traders WHERE website='' OR street='' ORDER BY RANDOM() LIMIT 60;
```

## Redraw-Schutz

Slugs der letzten Wellen per expliziter Commit-Liste ausschließen (Dossier-
Dateinamen aus den Wellen-Commits sammeln, NICHT per Zeitfenster — ein
Vollsplit-Commit vergiftet Zeitfenster). Gezogene Slugs gegen diese Liste
filtern, bis 60 frische Slugs stehen.

## Shards

3 parallele `general`-Subagenten (background), je 20 Slugs, bundesland-
disjunkt. Jedem Agenten seine EXAKTE Slug-Liste in den Prompt schreiben —
NUR diese Liste zählt. Wave-Dateien (`wave*.txt`) und alte Ergebnis-JSONs
unter `/tmp/opencode/` dürfen NICHT für die Auftragsauswahl genutzt werden.

## Recherche-Auftrag pro Agent (Dossiers DIREKT editieren)

- Fundstelle per `rglob <slug>.md`; KEINE DB-Writes, KEINE Commits, KEINE
  Bulk-Skripte, NIE raten, Einheiten nie erfinden/umrechnen, nichts löschen,
  Slugs NIEMALS ändern, `## Überblick` tabu.
- `seed/` ist retired — es wird ausschließlich in `dossiers/` gearbeitet.
- Frontmatter: nur flache Skalare, NUR LEERE Felder füllen (Ausnahme:
  bewiesene Korrektur mit 2 Belegen + Timeline-Note); `website` NUR
  Domain-Root; `phone`/`email` nie `—`.
- Beleg-Standard = 2 UNABHÄNGIGE Belege, sonst Feld leer + Klärfall-Vermerk.
- Quellen-Hierarchie: Aggregatoren (Das Örtliche, Gelbe Seiten, 11880,
  GoLocal, Cylex, city-map, Schrottplatz-/Schrottradar-Portale, Yelp,
  auftragsbank u.ä.) sind NUR Leads, NIEMALS Belege — auch nicht mehrfach.
  Belege: Betreiber-Website, Handelsregister/Northdata/Creditreform,
  kommunales Gewerberegister, Betreiber-Social. Zwei Seiten derselben Website
  = EINE Quelle.
- Owner-Ausnahme: verifizierte Betreiber-Primärquelle (Impressum Name+HRB+Ort,
  HR-kongruent, aktuelle Detailseiten) genügt ALLEIN für deren eigene
  Filial-Fakten — gilt NICHT für Einzelunternehmen ohne HRB und NICHT bei
  gescheitertem Impressum-Abruf.
- Vollcrawl-Pflicht: jede relevante Unterseite EINZELN abrufen (bis ~12).
- City-Mismatch → Straße/PLZ nie überschreiben außer bewiesener Korrektur.
- `website_status` ∈ {aktiv, tot, blockiert, unbekannt}.
- Jede Änderung als `### Recherche DD.MM.YYYY` mit
  `[Recherche …: …; Quelle(n): …]`; Unsicheres als (Einzelbeleg, unsicher)
  NUR in Timeline, NIE in Frontmatter. Preise nur bei vollständiger Angabe.
- Gründlichkeit: Dossier komplett lesen (Timeline!), Website tief crawlen,
  extern gegenrecherchieren — lieber `pruefung` + ehrlicher Vermerk als Fiktion.

## Owner-Gate nach Abschluss aller 3 Shards

Overwrite-Scan, Enum-Validierung, Deep-Link-/Emdash-Check, Notenformat, dann
Seed-Tests (`cargo test -p schrott-mcp-ingestion seed`), ein Commit, Push,
Redeploy (beide Binaries), Prod-Spot-Checks. Kurz berichten.
