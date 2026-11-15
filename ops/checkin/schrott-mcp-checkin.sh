#!/bin/bash
# Hourly owner check-in for the Schrott-MCP session.
#
# Die Ziel-Session wird **nicht** fest verdrahtet, sondern aus der
# Scheduler-Job-Datei gelesen. Grund: Check-in und 6h-Welle fahren dieselbe
# Session - wenn sie auseinanderlaufen, bekommt der Check-in keinen Kontext
# mehr und die Welle startet in einer leeren Session. Nach einem
# Session-Cleanup (13./15.11.2026) ist die hart verdrahtete Session-ID
# auf eine archivierte Session gesunken: der Check-in lief stuendlich in eine
# Session mit Zustand "interrupted" und blieb wirkungslos, waehrend die Welle
# in die neue Session fuhr. Deshalb wird die ID hier zur Laufzeit aufgeloest.
#
# Usage: schrott-mcp-checkin.sh [--dry-run]
set -u
SESSION=""
JOB_FILE="${CHECKIN_JOB_FILE:-/root/.config/opencode/scheduler/scopes/schrott-mcp-cba386ff6018/jobs/tiefenrecherche-welle.json}"

# 1) Quelle der Wahrheit: die Session, die der Scheduler fuer die Welle fährt.
if [ -r "$JOB_FILE" ]; then
  SESSION=$(python3 -c "
import json, sys
try:
    d = json.load(open('$JOB_FILE'))
    print(d.get('run', {}).get('session', '') or '')
except Exception:
    print('')
" 2>/dev/null || true)
fi

# 2) Rueckfall: eine explizit gesetzte Session (z. B. per Environment-Datei).
if [ -z "$SESSION" ] && [ -n "${CHECKIN_SESSION:-}" ]; then
  SESSION="$CHECKIN_SESSION"
fi

if [ -z "$SESSION" ]; then
  echo "schrott-mcp-checkin: keine Session ermittelt (Job-Datei: $JOB_FILE)" >&2
  exit 2
fi

export PATH="/root/.opencode/bin:$PATH"
MSG="Stündlicher Owner Check-in: Du bist der Owner dieses Projekts (Schrott-MCP, /root/Documents/schrott-mcp). Arbeite weiter. Die komplette Check-in-Anleitung steht in prompts/hourly-checkin.md im Repo — lies sie und führe sie aus."

if [ "${1:-}" = "--dry-run" ]; then
  echo "opencode run --session $SESSION [${#MSG} chars]"
  exit 0
fi

echo "$(date -u +%FT%TZ) check-in startet in Session $SESSION" >> /var/log/schrott-mcp-checkin.log
opencode run --session "$SESSION" "$MSG" >> /var/log/schrott-mcp-checkin.log 2>&1
