# `ops/checkin/` — stündlicher Owner Check-in (Betrieb)

Die Dateien hier sind die **versionierte Quelle** der Check-in-Mechanik auf dem
Host. Sie sind keine Dossiers und werden nicht geseedet; sie halten den
Betrieb nachvollziehbar, damit ein Fix nicht nur auf der Maschine existiert.

## Pfade auf dem Host

| Datei im Repo | Pfad auf dem Host |
|---|---|
| `schrott-mcp-checkin.service` | `/etc/systemd/system/schrott-mcp-checkin.service` |
| `schrott-mcp-checkin.timer` | `/etc/systemd/system/schrott-mcp-checkin.timer` |
| `schrott-mcp-checkin.sh` | `/usr/local/bin/schrott-mcp-checkin.sh` |
| `schrott-mcp-checkin-watchdog.sh` | `/usr/local/bin/schrott-mcp-checkin-watchdog.sh` |

Nach jeder Änderung: `systemctl daemon-reload` bzw. `systemctl restart
schrott-mcp-checkin.timer`. Der Watchdog läuft per Cron alle 15 Minuten und
schreibt nach `/var/log/schrott-mcp-checkin-watchdog.log`.

## Aufbau

- **Timer** (`OnCalendar=*-*-* *:17:00`) feuert **stündlich auf Minute 17**.
  Die Minute weicht bewusst von `:00` ab: die 6h-Welle feuert per Cron um
  `:00`, `:06`, `:12`, `:18` und fährt dieselbe Session. Ein Check-in um `:00`
  würde sich hinter der Welle einreihen.
- **Service** ist ein `oneshot` mit `TimeoutStartSec=1800` (**30 min**). Läuft gerade eine
  Welle in derselben Session, wartet der Check-in-Prozess auf deren Ende und
  wird nach 30 Minuten beendet (`status=2/INT`). **Das ist der Normalfall und
  kein Fehler** — der Timer feuert trotzdem wieder.
  - **Warum 30 min statt 5** (Befund vom 15.11.2026): ein Check-in, der mitten in
    der Arbeit per **SIGTERM** gekillt wird, hinterlässt die Session im Zustand
    `interrupted` — und der Welle-Job (`--continue` auf dieselbe Session) bricht
    dann **sofort** ab. So sind am 14.11. bis 15.11.2026 **acht** Wellen-Runs
    hintereinander gescheitert.
  - **Warum `KillSignal=SIGINT`**: damit schließt opencode die Session **sauber** ab,
    statt sie als `interrupted` zu hinterlassen.
- **Watchdog** behandelt vier Fälle getrennt:

| # | Störung | Behandlung |
|---|---|---|
| 1 | Dienst hängt länger als sein Zeitlimit | `systemctl stop` |
| 2 | `next_elapse` steht in der Vergangenheit | Unit neu laden, Timer neu starten |
| 3 | `next_elapse=(null)` — Kalender unwiederbringlich tot | Drop-in mit gültigem `OnCalendar=` **und** `OnUnitActiveSec=` |
| 4 | Dienst steht dauerhaft auf `failed`/`timeout` | `systemctl reset-failed` (Ergebniswert, nicht Exit-Code) |

## Zwei Fallen, die schon einmal zugeschlagen haben

- **`OnCalendar=` (leer) hebt im Drop-in nicht auf.** systemd wertet die Unit
  dann weiterhin mit dem alten Kalender aus und berechnet keinen Termin.
  Der Fallback setzt deshalb zusätzlich einen **gültigen** Kalender.
- **`ExecMainStartTimestampMonotonic` ist Mikrosekunden seit Boot**, nicht
  seit der Epoch. Das Alter des Dienstes muss gegen `/proc/uptime` gerechnet
  werden. Mit der Epoch-Zeit ergäbe sich ein Alter von ~1,8 Milliarden
  Sekunden — der Watchdog würde den Check-in dann bei **jedem** Lauf
  abwürgen, auch wenn der Dienst gerade erst gesund gestartet war.
  (Befund vom 13.11.2026, 18:17 UTC — siehe `research/befund-checkin-timer.md`.)

## Session-ID

`schrott-mcp-checkin.sh` braucht die Ziel-Session. Im Repo steht sie als
Platzhalter `CHECKIN_SESSION`; auf dem Host trägt das Skript die konkrete ID.
Die Session-ID ist kein Geheimnis, wird aber nicht im Repo dupliziert.

## Prüfen

```bash
systemctl list-timers --all | grep schrott-mcp-checkin
systemctl show schrott-mcp-checkin.timer -p TimersCalendar --value
journalctl -u schrott-mcp-checkin.service --since "2 hours ago" --no-pager
tail -20 /var/log/schrott-mcp-checkin-watchdog.log
```

Erwartet: Timer feuert stündlich, der Dienst endet nach 30 min mit
`Result=signal` (Session belegt), der Watchdog-Log bleibt bis zu einer
Störung leer.

## Die Session kommt vom Scheduler

Das Check-in-Skript verdrahtet **keine** Session-ID, sondern liest sie zur Laufzeit
aus der Scheduler-Job-Datei (`run.session`, Vorgabe:
`/root/.config/opencode/scheduler/scopes/schrott-mcp-cba386ff6018/jobs/tiefenrecherche-welle.json`).
Ohne auflösbare Session bricht das Skript mit **Exit 2** ab.

**Warum:** Am 15.11.2026 war die fest verdrahtete Session auf eine **archivierte**
Session gesunken (`idle_outcome=interrupted`), während der Scheduler die Welle in
der **aktuellen** Session fuhr — der stündliche Check-in schrieb also stundenlang in
eine Session, in der **nichts** passierte.

## Speicher

Der Host hat **7,7 GB RAM** und braucht für eine Welle mit drei Browser-Subagenten
rund **5 GB**. Bis zum 15.11.2026 gab es **gar keinen Swap** — der Kernel hat
`opencode` mit **3,1 GB** RSS **gekillt** (`oom-kill`, 15.11. 16:38:13 UTC).
Seitdem: `/swapfile` mit **2 GB** (persistent über `/etc/fstab`) und
`vm.swappiness=10` (persistent über `/etc/sysctl.d/99-schrott-mcp-memory.conf`).

**Regel:** **niemals** einen Release-Build parallel zu drei Browser-Agenten fahren —
der Build braucht bis zu **2 GB**, die Agenten **5 GB**.
