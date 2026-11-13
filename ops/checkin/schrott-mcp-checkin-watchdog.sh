#!/bin/bash
# Watchdog fuer den stuendlichen Owner Check-in.
#
# Drei Stoerungen traten auf und werden hier behandelt:
#
# 1) Der systemd-Timer rechnet seinen OnCalendar-Termin nur beim Laden der
#    Unit neu. Nach einem Ruecksprung der Systemuhr blieb der gespeicherte
#    next_elapse in der Vergangenheit stehen; der Timer feuerte danach nie
#    wieder (08.11.2026, 15:08 UTC bis zum 11.11.2026).
#
# 2) Der Check-in-Dienst wartet in derselben Session wie die 6h-Welle. Ohne
#    TimeoutStartSec blieb der oneshot am 11.11.2026 zwei Tage im Status
#    "activating" und blockierte den Timer vollstaendig. Der alte Watchdog
#    uebersprang genau diesen Fall ("Dienst laeuft") und heilte ihn nie.
#
# 3) Liegt der komplette OnCalendar-Kalender in der Vergangenheit (etwa
#    weil die Uhr zurueckgesprungen ist), berechnet systemd **keinen**
#    neuen Termin - auch nicht beim Neuladen. next_elapse bleibt dann
#    dauerhaft leer ("(null)") und der Timer ist unwiederbringlich tot.
#    Der Watchdog schreibt in diesem Fall den Kalender auf einen relativen
#    Zeitplan (OnUnitActiveSec) um, der immer weiterfeuert.
#
# Der Watchdog prueft deshalb unabhaengig voneinander:
#   - haengt der Dienst laenger als sein eigenes Zeitlimit, wird er beendet;
#   - steht der Termin in der Vergangenheit, wird die Unit neu geladen;
#   - gibt es gar keinen Termin mehr, wird der Kalender umgestellt.
set -u
TIMER=schrott-mcp-checkin.timer
SERVICE=schrott-mcp-checkin.service
# Zeitlimit des Dienstes in Sekunden (muss zu TimeoutStartSec passen).
MAX_RUN=300
# Einheit, falls der Kalender umgestellt werden muss (relativer Abstand).
FALLBACK_INTERVAL=3600
now_iso() { date -u +%FT%TZ; }
now_epoch() { date -u +%s; }
# Monotone Uhr: Sekunden seit dem Boot. ExecMainStartTimestampMonotonic
# wird ebenfalls relativ zum Boot geliefert - man muss also gegen
# /proc/uptime rechnen und NICHT gegen die Epoch-Zeit. Mit der Epoch-Zeit
# ergaebe sich ein Alter von ~1,8 Milliarden Sekunden und der Watchdog
# wuerde den Check-in-Dienst bei jedem Lauf sofort abwuergen, auch wenn
# er gerade erst (und gesund) gestartet war.
now_mono() { awk '{print int($1)}' /proc/uptime 2>/dev/null || echo 0; }

# (4) Der Timeout des Check-in-Dienstes ist im Normalfall der erwartete
#     Fall (die Session ist durch die 6h-Welle belegt). systemd fuehrt das
#     Ergebnis dauerhaft als "failed"/"timeout" - SuccessExitStatus deckt
#     nur Exit-Codes ab, nicht den Result-Wert. Ein Health-Check auf
#     "is-failed" waere damit dauerhaft rot, obwohl der Timer normal
#     feuert. Der Watchdog setzt den Fehlerstatus deshalb zurueck; das
#     Journal behaelt den Timeout-Vorgang nachvollziehbar erhalten.
reset_failed_if_needed() {
  state=$(systemctl is-active "$SERVICE" 2>/dev/null || true)
  [ "$state" = "active" ] && return 0
  [ "$state" = "activating" ] && return 0
  result=$(systemctl show "$SERVICE" -p Result --value 2>/dev/null || true)
  if [ "$result" = "timeout" ] || [ "$result" = "exit-code" ] || [ "$result" = "core-dump" ] || [ "$result" = "signal" ]; then
    echo "$(now_iso) Dienst $SERVICE steht auf Result=$result - Fehlerstatus wird zurueckgesetzt (Timeout im Normalfall)"
    systemctl reset-failed "$SERVICE" >/dev/null 2>&1 || true
  fi
}

# (1) Haengenden Dienst beenden. systemd beendet ihn zwar selbst nach
# TimeoutStartSec, aber nur, wenn der Dienst noch dem systemd untersteht.
# Ein von Hand gestarteter oder verwaister Lauf kann sonst ewig haengen.
svc_state=$(systemctl is-active "$SERVICE" 2>/dev/null || true)
if [ "$svc_state" = "activating" ] || [ "$svc_state" = "active" ]; then
  since=$(systemctl show "$SERVICE" -p ExecMainStartTimestampMonotonic --value 2>/dev/null || true)
  # ExecMainStartTimestampMonotonic ist in Mikrosekunden seit Boot.
  if [ -n "$since" ] && [ "$since" != "[not set]" ]; then
    started=$((since / 1000000))
    age=$(( $(now_mono) - started ))
    if [ "$age" -lt 0 ]; then
      # Unsinniger Zeitstempel (Uhr-/Boot-Sprung): nicht beenden, nur melden.
      echo "$(now_iso) Dienst $SERVICE hat einen unplausiblen Startzeitpunkt ($since) - wird nicht beendet"
    elif [ "$age" -gt "$MAX_RUN" ]; then
      echo "$(now_iso) Dienst $SERVICE haengt seit ${age}s (Limit ${MAX_RUN}s) - wird beendet"
      systemctl stop "$SERVICE" >/dev/null 2>&1 || true
      reset_failed_if_needed
    fi
  fi
fi

STATE=$(systemctl show "$TIMER" -p TimersCalendar --value 2>/dev/null | head -1)
NEXT=$(printf '%s' "$STATE" | sed -n 's/.*next_elapse=//p' | sed 's/ *}.*//' | sed 's/ *$//')
# systemd schreibt "(null)", wenn es keinen Termin berechnen konnte - das
# ist kein gueltiger Zeitstempel und wird deshalb wie "leer" behandelt.
if [ "$NEXT" = "(null)" ]; then
  NEXT=""
fi

# (2) Der Timer blockiert gerade (Einheit laeuft) - nichts zu tun.
if [ -z "$NEXT" ]; then
  # (3) next_elapse ist "(null)" oder fehlt: es gibt gar keinen Termin
  # mehr. Ein Neuladen genuegt dann nicht, weil systemd einen Kalender
  # komplett in der Vergangenheit nicht neu berechnet. Der Kalender wird
  # deshalb auf einen relativen Zeitplan umgestellt, der immer feuert.
  if printf '%s' "$STATE" | grep -q 'next_elapse=(null)'; then
    echo "$(now_iso) Timer $TIMER hat keinen Termin mehr (next_elapse=(null)) - Kalender wird auf OnUnitActiveSec umgestellt"
    FRAG=$(systemctl show "$TIMER" -p FragmentPath --value 2>/dev/null)
    DROPIN_DIR="$FRAG.d"
    mkdir -p "$DROPIN_DIR"
    # Wichtig: "OnCalendar=" (leer) hebt den alten Kalender in einem
    # Drop-in NICHT auf - systemd wertet die Unit dann weiterhin mit dem
    # alten (vergangenen) Kalender aus und berechnet keinen Termin.
    # Der Fallback setzt deshalb einen gueltigen Kalender (stuendlich auf
    # Minute 17) zusaetzlich zum relativen Intervall.
    printf '[Timer]\nOnCalendar=*-*-* *:17:00\nOnUnitActiveSec=%s\n' "$FALLBACK_INTERVAL" > "$DROPIN_DIR/watchdog-fallback.conf"
    systemctl daemon-reload
    systemctl restart "$TIMER"
  fi
  reset_failed_if_needed
  exit 0
fi

NEXT_EPOCH=$(date -u -d "$NEXT" +%s 2>/dev/null || true)
if [ -z "$NEXT_EPOCH" ]; then
  exit 0   # Zeitstempel nicht lesbar
fi
if [ "$NEXT_EPOCH" -lt "$(now_epoch)" ]; then
  echo "$(now_iso) Timer $TIMER hat einen veralteten Termin ($NEXT) - Unit wird neu geladen"
  systemctl daemon-reload
  systemctl restart "$TIMER"
fi

# (4) Fehlerstatus des Check-in-Dienstes zuruecksetzen (siehe oben).
reset_failed_if_needed
exit 0
