# 6h-Tiefenrecherche-Welle (Schrott-MCP)

60 Händler-Slugs aus dem Pool ziehen (NUR LESEN, nie in die DB schreiben):

```sql
SELECT slug, state FROM traders WHERE website='' OR street='' ORDER BY RANDOM() LIMIT 60;
```

## Shards

3 parallele `general`-Subagenten (background), je 20 Slugs, bundesland-
disjunkt. Jedem Agenten seine EXAKTE Slug-Liste in den Prompt schreiben —
NUR diese Liste zählt. Wave-Dateien (`wave*.txt`) unter `/tmp/opencode/`
dürfen NICHT für die Auftragsauswahl genutzt werden.

## Recherche-Auftrag pro Agent (Dossiers DIREKT editieren)

- Fundstelle per `rglob <slug>.md`; KEINE DB-Writes, KEINE Commits, KEINE
  Bulk-Skripte, Slugs NIEMALS ändern, `## Überblick` tabu — es wird
  ausschließlich in `dossiers/` gearbeitet.
- Statt zu raten: recherchieren. Einheiten nie erfinden/umrechnen, nichts
  löschen (Korrekturen als neue Notes, Historie bleibt erhalten).
- Frontmatter: nur flache Skalare; `website` NUR Domain-Root; `phone`/`email`
  nie `—`; `website_status` ∈ {aktiv, tot, blockiert, unbekannt}.
  `dropoff_json`/`pickup_json` nur als gequotete Strings, nie als Mappings
  (sonst Build-Bruch, Welle 31).
- Beleg-Leitlinie (kein absolutes Verbot): Ziele auf 2 UNABHÄNGIGE Belege
  (zwei Seiten derselben Website = EINE Quelle). Bei starker Einzelquelle
  (Betreiber-Impressum, Register, Kommune) darfst du füllen oder korrigieren
  — lege dann Herkunft und Restunsicherheit in der Timeline offen.
  Aggregatoren (Das Örtliche, Gelbe Seiten, 11880, GoLocal, Cylex, city-map,
  Schrottplatz-/Schrottradar-Portale, Yelp, auftragsbank u.ä.) sind Leads;
  als alleiniger Beleg nur in begründeten Ausnahmefällen mit offener
  Dokumentation.
- Owner-Ausnahme: verifizierte Betreiber-Primärquelle (Impressum Name+HRB+Ort,
  HR-kongruent, aktuelle Detailseiten) genügt ALLEIN für deren eigene
  Filial-Fakten — gilt NICHT für Einzelunternehmen ohne HRB und NICHT bei
  gescheitertem Impressum-Abruf.
- Vollcrawl-Pflicht: jede relevante Unterseite EINZELN abrufen (bis ~12).
- Adress-Hinweis: Bei Adressänderung in der Timeline vermerken, dass die
  Koordinaten neu zu geocodieren sind (geocode_cache-Schlüssel ist
  adressbasiert — die Neu-Geocodierung läuft automatisch).
- Jede Änderung als `### Recherche DD.MM.YYYY` mit
  `[Recherche …: …; Quelle(n): …]`; lege zu jedem Fill Quelle und Beleglage
  offen. Preise nur bei vollständiger Angabe.
- Gründlichkeit: Dossier komplett lesen (Timeline!), Website tief crawlen,
  extern gegenrecherchieren — lieber `pruefung` + ehrlicher Vermerk als Fiktion.

## Owner-Gate nach Abschluss aller 3 Shards

Overwrite-Scan, Enum-Validierung, Deep-Link-/Emdash-Check, Notenformat, dann
Seed-Tests (`cargo test -p schrott-mcp-ingestion seed`), ein Commit, Push,
Redeploy (beide Binaries), Prod-Spot-Checks. Kurz berichten.
