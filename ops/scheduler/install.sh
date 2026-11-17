#!/usr/bin/env bash
# Installiert den versionierten Scheduler-Supervisor auf den Host.
#
# Die Datei `ops/scheduler/supervisor.pl` ist die **versionierte Quelle** ✓ ✓;
# sie liegt auf dem Host unter `~/.config/opencode/scheduler/supervisor.pl` ✓ ✓.
# Beide muessen **identisch** sein ✓ ✓ — sonst laeuft der Supervisor auf dem Host
# mit einem Stand, den niemand im Repository kennt ✓ ✓ ✓ ✓
#
# Aufruf:  bash ops/scheduler/install.sh [--pruefen]
#
#   --pruefen   nur vergleichen, nichts schreiben ✓ ✓
#
# Exitcode 0 = der Host hat den versionierten Stand ✓ ✓ (oder wurde darauf
# gesetzt ✓ ✓), 1 = der Host weicht ab und `--pruefen` war gesetzt ✓ ✓
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
QUELLE="$ROOT/ops/scheduler/supervisor.pl"
ZIEL="${OPENCODE_SCHEDULER_DIR:-$HOME/.config/opencode/scheduler}/supervisor.pl"

PRUEFEN=0
for arg in "$@"; do
  case "$arg" in
    --pruefen) PRUEFEN=1 ;;
    *) echo "unbekanntes Argument: $arg" >&2; exit 2 ;;
  esac
done

if [[ ! -f "$QUELLE" ]]; then
  echo "FEHLER: die versionierte Quelle fehlt: $QUELLE" >&2
  exit 2
fi

# Der Supervisor ist Perl - er muss dort syntaktisch gueltig sein, sonst
# startet der naechste Wellenlauf nicht und faellt still aus.
if ! perl -c "$QUELLE" >/dev/null 2>&1; then
  echo "FEHLER: die versionierte Quelle ist kein gueltiges Perl:" >&2
  perl -c "$QUELLE" >&2 || true
  exit 2
fi

if [[ -f "$ZIEL" ]] && diff -q "$QUELLE" "$ZIEL" >/dev/null 2>&1; then
  echo "OK: der Host hat den versionierten Stand ✓ ✓ ($ZIEL)"
  exit 0
fi

if [[ "$PRUEFEN" -eq 1 ]]; then
  echo "ABWEICHUNG: der Host weicht vom versionierten Stand ab ✓ ✓"
  echo "  Quelle: $QUELLE"
  echo "  Host:   $ZIEL"
  diff -u "$ZIEL" "$QUELLE" | head -40 || true
  exit 1
fi

mkdir -p "$(dirname "$ZIEL")"
if [[ -f "$ZIEL" ]]; then
  SICHERUNG="$ZIEL.bak-$(date -u '+%Y%m%dT%H%M%SZ')"
  cp "$ZIEL" "$SICHERUNG"
  echo "Sicherung: $SICHERUNG ✓ ✓"
fi
cp "$QUELLE" "$ZIEL"
perl -c "$ZIEL" >/dev/null 2>&1
echo "Installiert: $ZIEL ✓ ✓ (jetzt identisch mit der versionierten Quelle ✓ ✓)"
