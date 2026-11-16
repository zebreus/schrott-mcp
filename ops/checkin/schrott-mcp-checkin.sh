#!/bin/bash
# Hourly owner check-in for the Schrott-MCP session.
#
# Der Check-in bekommt **eine eigene Session**. Grund: Check-in und 6h-Welle
# fahren seit dem Session-Cleanup vom 13./15.11.2026 dieselbe Session, und
# das erzeugt zwei Probleme:
#
#   1. Startet der Check-in, während die Welle in derselben Session laeuft,
#      wartet der Check-in - und umgekehrt. Die Welle bricht dann ab, wenn
#      der Check-in per SIGTERM mitten in der Arbeit gekillt wird
#      (Session-Zustand "interrupted").
#   2. Der Check-in sah stundenlang in eine ARCHIVIERTE Session ("interrupted")
#      und blieb dadurch wirkungslos - der stuendliche Check-in hat nichts
#      getan, waehrend die Welle in der aktuellen Session fuhr.
#
# Deshalb: der Check-in startet eine **frische** Session ohne `--continue`.
# Der Projektstand steht ohnehin im Repo (Git, research/*.md, die
# Produktionsdatenbanken read-only) - nicht im Gespraechszustand.
#
# Mit `CHECKIN_SESSION` (Environment) kann eine feste Session erzwungen
# werden; `CHECKIN_CONTINUE=1` laesst sie fortsetzen.
#
# Das Modell wird explizit gepinnt (`CHECKIN_MODEL`, Vorgabe
# openrouter/qwen/qwen4-max). Grund: ohne Pin erbt `opencode run` das
# jeweils zuletzt benutzte Modell. Am 15./16.11.2026 war das
# google/gemini-4-hydrogen mit 256000 max_tokens - mehr als das
# OpenRouter-Key-Budget deckte ("can only afford ~246k"). Jeder Check-in
# von 21:17 bis 06:17 UTC starb daran vor der ersten Aktion (exit 1).
#
# Usage: schrott-mcp-checkin.sh [--dry-run]
set -u
SESSION="${CHECKIN_SESSION:-}"
MODEL="${CHECKIN_MODEL:-openrouter/qwen/qwen4-max}"
JOB_FILE="${CHECKIN_JOB_FILE:-/root/.config/opencode/scheduler/scopes/schrott-mcp-cba386ff6018/jobs/tiefenrecherche-welle.json}"

# Diagnose: welche Session faehrt die Welle? Nur nach der Session-Aufloesung
# darf der Check-in sie nicht benutzen - sonst kollidieren beide.
WAVE_SESSION=""
if [ -r "$JOB_FILE" ]; then
  WAVE_SESSION=$(python3 -c "
import json
try:
    d = json.load(open('$JOB_FILE'))
    print(d.get('run', {}).get('session', '') or '')
except Exception:
    print('')
" 2>/dev/null || true)
fi

export PATH="/root/.opencode/bin:$PATH"
MSG="Stündlicher Owner Check-in: Du bist der Owner dieses Projekts (Schrott-MCP, /root/Documents/schrott-mcp). Arbeite weiter. Die komplette Check-in-Anleitung steht in prompts/hourly-checkin.md im Repo — lies sie und führe sie aus."

if [ -n "$SESSION" ] && [ "${CHECKIN_CONTINUE:-0}" = "1" ]; then
  ARGS=(--session "$SESSION")
  DESC="Session $SESSION (fortgesetzt)"
elif [ -n "$SESSION" ]; then
  ARGS=(--session "$SESSION")
  DESC="Session $SESSION"
else
  ARGS=()
  DESC="frische Session (Welle faehrt ${WAVE_SESSION:-unbekannt})"
fi

if [ "${1:-}" = "--dry-run" ]; then
  echo "opencode run --model $MODEL ${ARGS[*]-} [${#MSG} chars] -> $DESC"
  exit 0
fi

echo "$(date -u +%FT%TZ) check-in startet -> $DESC (modell $MODEL)" >> /var/log/schrott-mcp-checkin.log
opencode run --model "$MODEL" "${ARGS[@]}" "$MSG" >> /var/log/schrott-mcp-checkin.log 2>&1
