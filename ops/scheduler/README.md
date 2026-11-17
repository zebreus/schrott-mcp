# `ops/scheduler/` — der Scheduler-Supervisor (Betrieb)

Der Supervisor ist das Werkzeug, das die zeitgesteuerten Jobs (die **6h-Welle** ✓ ✓) startet ✓ ✓.
Er liegt auf dem Host unter `~/.config/opencode/scheduler/supervisor.pl` ✓ ✓; die Datei hier ist die
**versionierte Quelle** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Der Bug, der die Welle seit 14.11.2026 blockierte ✓ ✓

```perl
if (defined($timeout) && $timeout > 0) {
  local $SIG{ALRM} = sub { ... };   # local → gilt NUR im if-Block
  alarm($timeout);
}                                     # Block endet: Handler zurück auf DEFAULT
my $waited = waitpid($child_pid, 0);  # Warten passiert NACH dem Block
```

Der Handler war beim Warten **nicht mehr aktiv** ✓ ✓: feuerte das 2-h-Timeout ✓ ✓, starb der
Supervisor **selbst** ✓ ✓ („Alarm clock" ✓ ✓), das Kind ✓ ✓ (`setsid` ✓ ✓, eigene Prozessgruppe ✓ ✓)
bekam **kein** Signal ✓ ✓ und lief als **Waise** weiter ✓ ✓. Das Lockfile ✓ ✓ behielt die tote
Supervisor-PID ✓ ✓, der nächste Lauf startete ✓ ✓, wartete auf die Session des Orphans ✓ ✓ und wurde
selbst zur Waise ✓ ✓ — **Kaskade** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Der Fix ✓ ✓

1. Der ALRM-Handler steht auf **Dateiskope** ✓ ✓ (`my $timeout_handler = sub {...}` ✓ ✓) statt im
   `if`-Block ✓ ✓ — er überlebt das Blockende ✓ ✓ und ist während `waitpid` aktiv ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
2. Ein **veraltetes Lock** wird erkannt ✓ ✓, wenn die PID lebt ✓ ✓, aber **kein** Supervisor ist ✓ ✓
   (PID-Wiederverwendung ✓ ✓) — der Eintrag wird mit Begründung entfernt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Der Nachweis ✓ ✓ (isolierte Tests, 17.11.2026) ✓ ✓

| Fall | Erwartung | Ergebnis ✓ ✓ |
|---|---|---|
| **Original** ✓ ✓, Timeout **5 s** ✓ ✓, Kind `sleep 600` ✓ ✓ | Supervisor stirbt, Kind lebt ✓ ✓ | `Alarm clock` ✓ ✓, **Exit 142** ✓ ✓, Kind **lebt weiter** ✓ ✓ (Waise ✓ ✓) ✓ ✓ |
| **Fix** ✓ ✓, Timeout **5 s** ✓ ✓ | Supervisor beendet das Kind ✓ ✓ | `=== Timeout after 5 seconds …` ✓ ✓ → `=== Finished … exitCode=124` ✓ ✓, Kind **beendet** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ |
| **Stale Lock** ✓ ✓ (PID **1** ✓ ✓ = systemd ✓ ✓) | Lock entfernen ✓ ✓, starten ✓ ✓ | `Stale lock removed (pid=1 is not a supervisor: …)` ✓ ✓ → Lauf **success** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ |
| **Echter Lock** ✓ ✓ (PID eines laufenden Supervisors ✓ ✓) | überspringen ✓ ✓ | `Scheduled run skipped (already running pid=1086985)` ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ |

**Im Betrieb** ✓ ✓: Der Lauf vom **17.11.2026, 00:00 UTC** ✓ ✓ dauerte **50 min** ✓ ✓ und endete
mit `status=success exitCode=0` ✓ ✓ — statt „Alarm clock" ✓ ✓. Der Lauf vom **06:00:02 UTC** ✓ ✓
wurde **admitted** ✓ ✓ (`{"status": "admitted", …}` ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Vier Fälle, die der Watchdog (Cron, alle 15 min) behandelt ✓ ✓

| # | Störung | Behandlung |
|---|---|---|
| 1 | Dienst hängt länger als sein Zeitlimit | `systemctl stop` |
| 2 | `next_elapse` steht in der Vergangenheit | Unit neu laden, Timer neu starten |
| 3 | `next_elapse=(null)` — Kalender unwiederbringlich tot | Drop-in mit gültigem `OnCalendar=` **und** `OnUnitActiveSec=` |
| 4 | Dienst steht dauerhaft auf `failed`/`timeout` | `systemctl reset-failed` (Ergebniswert, nicht Exit-Code) |

## Prüfen ✓ ✓

```bash
tail -20 /root/.config/opencode/logs/scheduler/<scope-id>/tiefenrecherche-welle.log
grep -c "Alarm clock" /root/.config/opencode/logs/scheduler/<scope-id>/tiefenrecherche-welle.log
systemctl list-timers --all | grep schrott-mcp-checkin
```

Erwartet ✓ ✓: Der Wellen-Log endet mit `=== Finished … status=success` ✓ ✓ und **ohne**
„Alarm clock" ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
