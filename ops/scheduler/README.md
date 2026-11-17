# `ops/scheduler/` — der Supervisor der 6h-Welle

Der Cron-Eintrag des Wellen-Schedulers ruft
`/root/.config/opencode/scheduler/supervisor.pl` auf. Diese Datei ist die
**versionierte Quelle** desselben Skripts — der Host trägt eine **byte-identische**
Kopie (geprüft mit `diff`).

## Was der Supervisor macht

Er startet den Wellen-Run (`opencode run --session <SESSION>`) als Kindprozess
mit eigener Prozessgruppe (`setsid`), hält ein Lockfile, und bricht den Lauf nach
einem 2-h-Timeout ab.

## Der Bug vom 14.11.2026 — und sein Fix

**Symptom:** Die Wellen-Runs seit 14.11.2026 starben im Log mit
**„Alarm clock"**, ohne Finished-Zeile. Die Session zeigte seit 15.11. 19:19 UTC
keine Aktivität mehr, und **zwei** `opencode run`-Prozesse lebten gleichzeitig.

**Ursache** (`supervisor.pl`, Zeilen 184–201 in der alten Fassung): Der
ALRM-Handler war als **`local $SIG{ALRM}`** im `if`-Block definiert. Der Block
endete **vor** `waitpid` — beim Warten war der Handler also schon wieder auf
Default zurückgesetzt. Feuert das Timeout, **stirbt der Supervisor selbst**;
das Kind (eigene Prozessgruppe) bekommt **kein** Signal und läuft als Waise
weiter. Das Lockfile behält die tote Supervisor-PID, der nächste Lauf startet,
wartet auf die Session des Orphans und wird selbst zur Waise. **Kaskade.**

**Fix:** Der Handler ist jetzt eine benannte Subroutine auf **Dateiskope**, die
**vor** `waitpid` gesetzt wird:

```perl
my $timeout_handler = sub { ... kill 'TERM', -$child_pid; sleep 5; kill 'KILL', -$child_pid; };
if (defined($timeout) && $timeout > 0) { $SIG{ALRM} = $timeout_handler; alarm($timeout); }
my $waited = waitpid($child_pid, 0);
alarm(0); $SIG{ALRM} = 'DEFAULT';
```

`my $child_pid` und `my $timed_out` stehen **vor** der Subroutine (Zeilen 178/179),
damit der Lexical Closure greift (`perl -c` bestätigt: syntax OK ✓). Das negative
PID ist nötig, weil `setsid` die Prozessgruppe des Kindes vom Supervisor trennt.

**Nachweis:** Der Lauf vom 17.11.2026, 00:00 UTC endete regulär mit
`status=success exitCode=0 durationMs=3024947` ✓, die vier verwaisten
Geschwister-Runs wurden beendet ✓, und es liegt **kein** Lockfile mehr zurück ✓.

## Nachvollziehbarkeit

```bash
diff ops/scheduler/supervisor.pl /root/.config/opencode/scheduler/supervisor.pl
perl -c ops/scheduler/supervisor.pl
tail -20 /root/.config/opencode/logs/scheduler/schrott-mcp-cba386ff6018/tiefenrecherche-welle.log
ps -eo pid,etimes,args | grep "[o]pencode run"   # darf keine Waisen zeigen
```

Nach jeder Änderung: die Host-Kopie synchronisieren, dann den Cron-Lauf abwarten.
