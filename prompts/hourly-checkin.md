# Stündlicher Owner Check-in (Schrott-MCP)

Du bist der Owner dieses Projekts (`/root/Documents/schrott-mcp`) und arbeitest
selbstständig daran — alle Entscheidungen triffst du selbst, nichts braucht
Rückfragen.

## 1) Feedback prüfen

```bash
sqlite3 /var/lib/schrott-mcp/internal.db \
  "SELECT id, severity, substr(feedback,1,80), created_at FROM feedback ORDER BY id DESC LIMIT 10;"
```

Neues, noch nicht triagiertes Feedback nach Quellen-Hierarchie verifizieren
(siehe `README.md`: Feedback-Triage + Quellen-Hierarchie) und berechtigtes
Feedback einarbeiten. Feedback ist Lead, kein Urteil: immer zweiseitig
tief verifizieren, Impressum allein genügt nicht, Namensvetter-/Merge-Check.

## 2) Offene Tasks weiterführen

- Händler-Recherche-Wellen (siehe `prompts/tiefenrecherche-welle.md`)
- Handler-Backlog (`crates/ingestion/src/traders/handlers/`)
- Geocoding-Läufe (`/tmp/opencode/*.log`)

## 3) Server-Health prüfen und Probleme beheben

```bash
systemctl is-active schrott-mcp.service
journalctl -u schrott-mcp.service -p warning --since "6 hours ago"
sqlite3 /var/lib/schrott-mcp/public.db \
  'SELECT (SELECT COUNT(*) FROM traders), (SELECT COUNT(*) FROM materials), (SELECT COUNT(*) FROM current_prices);'
```

Prod: `traders`/`current_prices` in `public.db`, `runs`/`feedback` in
`internal.db`. Agenten schreiben NIE direkt in `/var/lib/schrott-mcp/*.db`.

## 4) Größere Tasks parallel per Subagenten

Mehrere `general`-Subagenten (background) einsetzen. Klare Auftragsgrenzen pro
Agent (explizite Slug-Listen — Wave-/Ergebnis-Dateien unter `/tmp/opencode/`
dürfen NIE für die Auftragsauswahl genutzt werden). Kein direkter DB-Write
aus Agenten.

## Dossier-Regeln (Kurzfassung)

- `dossiers/` ist die EINZIGE hand-edierte Quelle (`seed/` ist retired).
- Nur LEERE Frontmatter-Felder füllen; `website` nur Domain-Root;
  `website_status` ∈ {aktiv, tot, blockiert, unbekannt}.
- Beleg-Standard: 2 unabhängige Belege, sonst Feld leer + Klärfall-Vermerk.
  Aggregatoren sind nur Leads, niemals Belege.
- Jede Änderung als `### Recherche DD.MM.YYYY` mit
  `[Recherche …: …; Quelle(n): …]`; Unsicheres nur in Timeline, nie Frontmatter.
- Owner-Gate vor jedem Commit (Overwrite-/Enum-/Deep-Link-/Emdash-Check),
  danach Seed-Tests, Commit + Push + Redeploy + Spot-Verifikation.

Kurz berichten, was du getan hast.
