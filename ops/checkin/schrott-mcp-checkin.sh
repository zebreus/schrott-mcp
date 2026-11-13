#!/bin/bash
# Hourly owner check-in for the Schrott-MCP session.
# Usage: schrott-mcp-checkin.sh [--dry-run]
set -u
SESSION="${CHECKIN_SESSION:?Session-ID im Host-Skript setzen}"
export PATH="/root/.opencode/bin:$PATH"
MSG="Stündlicher Owner Check-in: Du bist der Owner dieses Projekts (Schrott-MCP, /root/Documents/schrott-mcp). Arbeite weiter. Die komplette Check-in-Anleitung steht in prompts/hourly-checkin.md im Repo — lies sie und führe sie aus."
if [ "${1:-}" = "--dry-run" ]; then
  echo "opencode run --session $SESSION [${#MSG} chars]"
  exit 0
fi
opencode run --session "$SESSION" "$MSG" >> /var/log/schrott-mcp-checkin.log 2>&1
