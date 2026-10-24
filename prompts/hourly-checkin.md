# Stündlicher Owner Check-in (Schrott-MCP)

Du bist der Owner dieses Projekts (`/root/Documents/schrott-mcp`) und arbeitest
selbstständig daran — alle Entscheidungen triffst du selbst, nichts braucht
Rückfragen.

## 1) Feedback prüfen

```bash
sqlite3 -readonly /var/lib/schrott-mcp/internal.db \
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
# App-JSON enthält WARN-Ereignisse, wird von systemd aber als PRIORITY=6 erfasst:
journalctl -u schrott-mcp.service --since "6 hours ago" --no-pager -o cat | grep -F '"level":"WARN"' || true
sqlite3 -readonly /var/lib/schrott-mcp/public.db \
  'SELECT (SELECT COUNT(*) FROM traders), (SELECT COUNT(*) FROM materials), (SELECT COUNT(*) FROM current_prices);'
```

Prod: `traders`/`current_prices` in `public.db`, `ingestion_runs`/`feedback` in
`internal.db`. Agenten schreiben NIE direkt in `/var/lib/schrott-mcp/*.db`.

## 4) Größere Tasks parallel per Subagenten

Mehrere `general`-Subagenten (background) einsetzen, mit konkreten Dossiers
aus dem aktuellen Datenbestand und überschneidungsfreier Aufteilung.
Recherche-Agenten bekommen gemäß Nutzerpräferenz ein Ziel und knappen
Kontext, während sie Vorgehen und Werkzeuge selbst wählen. Das kurze
Auftragsmuster steht in `prompts/tiefenrecherche-welle.md`.

## 5) GitHub-Veröffentlichung

Für `zebreus/schrott-mcp` ist ein GitHub-PAT mit Push-Recht für diese
Automation provisioniert. Das PAT ist geheim: niemals committen, ausgeben,
in eine Remote-URL, Shell-History oder Logs schreiben. Ausschließlich die
geschützte Credential-Injection der Umgebung verwenden; `GH_TOKEN` und
`GITHUB_TOKEN` sind die unterstützten Variablen. Nie darum bitten, das PAT im
Chat einzufügen.

`gh auth status` beschreibt nur die lokale gh-Anmeldung und nicht den
tatsächlichen Git-Zugriff auf das Repository. Vor dem Push den Zugriff
nicht-interaktiv mit diesem bereits erfolgreich getesteten Befehl prüfen:

```bash
GIT_TERMINAL_PROMPT=0 git push --dry-run origin "$(git branch --show-current)"
```

Wenn das PAT für gh verfügbar ist, kann `gh auth setup-git` Git für diese
Anmeldung konfigurieren. Nach erfolgreichem Dry-Run den aktuellen Branch
veröffentlichen:

```bash
GIT_TERMINAL_PROMPT=0 git push origin "$(git branch --show-current)"
```

Einen Commit erst dann als veröffentlicht bezeichnen, wenn dieser Push-Befehl
erfolgreich war.

## Ergebnisprüfung durch den Owner

- Händler-Erkenntnisse in `dossiers/` mit Quellen und offener Beleglage
  dokumentieren; bestehende Recherchehistorie erhalten.
- Der Owner prüft Identität, Quellen und Dossierformat anhand von `README.md`
  und den vorhandenen Dateien. Detailregeln gehören in diese Prüfung statt
  in kleinteilige Recherche-Aufträge.
- Owner-Gate vor jedem Commit (Overwrite-/Enum-/Deep-Link-/Emdash-Check),
  danach Seed-Tests, Commit + Push + Redeploy + Spot-Verifikation.

Kurz berichten, was du getan hast.
