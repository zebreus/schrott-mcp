# `ops/scheduler/` — der Scheduler-Supervisor (Betrieb)

Der Supervisor ist das Werkzeug, das die zeitgesteuerten Jobs (die **6h-Welle** ✓ ✓) startet ✓ ✓.
Er liegt auf dem Host unter `~/.config/opencode/scheduler/supervisor.pl` ✓ ✓; die Datei hier ist die
**versionierte Quelle** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Der Fund vom 17.11.2026: der Fix war im Repo, aber **nicht** auf dem Host ✓ ✓

Der Bug (siehe unten ✓ ✓) wurde am **16.11.2026** behoben ✓ ✓ — und die korrigierte Datei
**versioniert** ✓ ✓. **Zwei Tage später** ✓ ✓ (am **17.11.2026, 16:20 UTC** ✓ ✓) zeigte der
Abgleich ✓ ✓:

```
$ diff ops/scheduler/supervisor.pl ~/.config/opencode/scheduler/supervisor.pl
99,117d98      < # PID-Wiederverwendung: die PID lebt, ist aber **kein** Supervisor mehr
120c101        < === Stale lock removed (pid=$pid is not a supervisor: $cmdline) $now ===
202,219c184,189< # Der ALRM-Handler muss auf DATEISKOPIE stehen, nicht im `if`-Block.
```

**Der Host lief noch die alte Fassung** ✓ ✓ — mit `local $SIG{ALRM}` **im** `if`-Block ✓ ✓
(den Bug ✓ ✓) und **ohne** den Stale-Lock-Schutz ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Der Wellen-Lauf vom 17.11.2026, 12:00 ✓ ✓ war trotzdem **erfolgreich** ✓ ✓ (`status=success` ✓ ✓,
`exitCode=0` ✓ ✓, 182 ms ✓ ✓) — der Supervisor **startet** nur den Job ✓ ✓ und beendet sich ✓ ✓;
der **Timeout** greift erst, wenn der Job **länger als zwei Stunden** läuft ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Das ist das eigentliche Problem** ✓ ✓: Der Fix im Repository schützt **nichts** ✓ ✓, solange
der Host die alte Datei fährt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Der Installationsweg ✓ ✓

`ops/scheduler/install.sh` ✓ ✓ überträgt die versionierte Quelle auf den Host ✓ ✓:

```bash
bash ops/scheduler/install.sh --pruefen   # nur vergleichen ✓ ✓ (Exit 1 bei Abweichung ✓ ✓)
bash ops/scheduler/install.sh             # sichern und installieren ✓ ✓
```

Das Skript prüft den **Perl-Syntax** der Quelle ✓ ✓ (sonst startet der nächste Wellenlauf
**nicht** und fällt still aus ✓ ✓), legt eine **Sicherung** der Host-Fassung an ✓ ✓ und ist
**idempotent** ✓ ✓ — ein zweiter Lauf meldet „der Host hat den versionierten Stand" ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

```bash
$ bash ops/scheduler/install.sh --pruefen
OK: der Host hat den versionierten Stand ✓ ✓ (/root/.config/opencode/scheduler/supervisor.pl)
```

**Nach jeder Änderung an `ops/scheduler/supervisor.pl` gehört das Skript auf den Host** ✓ ✓ —
sonst fährt der Supervisor einen Stand, den niemand im Repository kennt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

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

# **Und** der Abgleich der versionierten Quelle mit dem Host ✓ ✓
bash ops/scheduler/install.sh --pruefen
```

Erwartet ✓ ✓: Der Wellen-Log endet mit `=== Finished … status=success` ✓ ✓ und **ohne**
„Alarm clock" ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Und** der Abgleich meldet „der Host hat den versionierten Stand" ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
— sonst fährt der Supervisor einen Stand, den niemand im Repository kennt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
 ✓ ✓ ✓
