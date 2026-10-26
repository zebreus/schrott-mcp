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

### Bestätigte Datenfehler beheben, nicht nur melden

- Ein bestätigter falscher Produktionswert ist ein offener Korrekturauftrag,
  kein abgeschlossener Check-in-Befund. Den Sachverhalt und die betroffene
  Zeile zweiseitig prüfen, die belegte Korrektur im Dossier/Seed festhalten
  und die Recherchehistorie erhalten. Keine Ersatzwerte raten.
- Die Korrektur bis zur sicheren Übernahme weiterverfolgen: offene
  Pre-Deploy-Prüfungen konkret bearbeiten und den freigegebenen regulären
  Rollout anstreben. Einen Datenfehler erst dann als behoben bezeichnen, wenn
  ein read-only Produktionsabgleich nach dem Rollout den erwarteten Wert
  bestätigt.
- Ein offenes Deployment-Gate ist kein Grund, die Korrektur aufzugeben oder
  nur wiederholt zu melden. Es autorisiert aber auch keinen Bypass: Solange die
  explizite Pre-Deploy-Freigabe fehlt, weder direkt in Produktionsdatenbanken
  schreiben noch deployen. Im Check-in die konkrete Gate-Arbeit fortsetzen und
  verbleibende Freigabeschritte benennen.

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
in eine Remote-URL, Shell-History oder Logs schreiben. Auf dem aktuellen
Linux-Host wird `GH_TOKEN` über `/root/.bashrc` bereitgestellt; den Wert nie
anzeigen oder kopieren. Nie darum bitten, das PAT im Chat einzufügen.

Den tatsächlichen nicht-interaktiven Git-Zugriff mit diesem getesteten Befehl
prüfen. Er lädt die geschützte Umgebungsvariable, konfiguriert Gits GitHub-CLI-
Credential-Helper und führt nur einen Dry-Run aus:

```bash
HOME=/root bash -ic 'gh auth setup-git && GIT_TERMINAL_PROMPT=0 git push --dry-run origin "$(git branch --show-current)"'
```

Nach erfolgreichem Dry-Run den aktuellen Branch veröffentlichen:

```bash
HOME=/root bash -ic 'gh auth setup-git && GIT_TERMINAL_PROMPT=0 git push origin "$(git branch --show-current)"'
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
  danach Seed-Tests, Commit und Push.
- Deployment-Gates strikt trennen: Vor einem Deployment muss die
  **Pre-Deploy-Freigabe** für exakt den geprüften Commit, Seed-Bestand und die
  Release-Binaries grün sein (Owner-/erforderliches unabhängiges Review,
  Tests, Source-/Seed-Audit und private Upgrade-/Datenübergangsprobe). Ein
  `production_ready: false`-Inventory oder ein rotes
  `scripts/verify-mkm-release.py` ist keine Pre-Deploy-Freigabe und kein Grund,
  eine notwendige Produktionsmigration dauerhaft zu blockieren: Dieser
  Verifier prüft den **Post-Rollout-Zustand** und ist vor dem Deployment bei
  altem Schema/Manifest erwartungsgemäß rot. Das rote Ergebnis autorisiert
  umgekehrt kein Deployment.
- Nur nach explizit grüner Pre-Deploy-Freigabe regulär deployen; danach auf
  einen normalen Scheduler-Lauf warten (kein Force-Ingest, keine direkten
  Produktions-DB-Änderungen) und den Post-Rollout-Verifier read-only mit dem
  tatsächlichen Rollout-Zeitpunkt ausführen. Bleibt er rot, Rollout nicht als
  erfolgreich melden, keine weiteren Produktionsänderungen vornehmen und den
  konkreten Befund dokumentieren. Ist die Pre-Deploy-Freigabe offen, weder
  deployen noch Produktionsdaten direkt ändern.

Kurz berichten, was du getan hast.
