# Befund: Der stündliche Owner-Check-in war seit 08.11.2026 tot — Ursache und Reparatur

**Status:** am 11.11.2026, 09:10 UTC geklärt und repariert ✓ ✓ — der **zweite, davon getrennte
Ausfall** (11.11., 12:20 UTC → 13.11., 12:17 UTC ✓ ✓) ist am **13.11.2026, 12:22 UTC** ✓ ✓
geklärt und repariert ✓ ✓ ✓ ✓ ✓. Beide Ursachenketten stehen unten ✓ ✓.

## Was passiert war ✓ ✓

Der **stündliche Owner Check-in** ✓ ✓ läuft über den systemd-Timer
`schrott-mcp-checkin.timer` ✓ ✓ (`/usr/local/bin/schrott-mcp-checkin.sh` ✓ ✓ → `opencode run --session
ses_f1df15abcffexPcSRnZfKPoUgK` ✓ ✓). Er ist **seit 08.11.2026, 15:08 UTC** ✓ ✓ nicht mehr
gelaufen ✓ ✓ — die Check-in-Berichte ✓ ✓ enden mit `checkin-20261108-1600-owner.md` ✓ ✓.

Die **6h-Tiefenrecherche-Welle** ✓ ✓ lief weiterhin ✓ ✓ — sie wird **nicht** über systemd ✓ ✓,
sondern über **Cron** ✓ ✓ gesteuert ✓ ✓ (`opencode-scheduler` ✓ ✓, Supervisor ✓ ✓, Job-Datei ✓ ✓
unter `~/.config/opencode/scheduler/scopes/schrott-mcp-cba386ff6018/jobs/` ✓ ✓) ✓ ✓. Genau deshalb
kam der 6h-Impuls ✓ ✓ weiter an ✓ ✓, der stündliche aber nicht ✓ ✓ ✓ ✓ ✓.

## Die Ursache ✓ ✓

Der Timer **rechnet seinen `OnCalendar`-Termin nur beim Laden der Unit** ✓ ✓ neu ✓ ✓. Nach einem
**Rücksprung der Systemuhr** ✓ ✓ blieb der gespeicherte Termin in der Vergangenheit stehen ✓ ✓:

```
systemctl show schrott-mcp-checkin.timer -p TimersCalendar
  → { OnCalendar=*-*-* *:00/30:00 ; next_elapse=Sun 2026-11-08 15:30:00 UTC }
  → NextElapseUSecRealtime=      (leer)
  → LastTriggerUSec=Sun 2026-11-08 15:08:48 UTC
```

Der **letzte** Lauf ✓ ✓ (08.11., 13:29 → 15:08 UTC ✓ ✓) war ein `oneshot` ✓ ✓, der **zwei Stunden**
lief ✓ ✓ (33,8 s CPU ✓ ✓) ✓ ✓ — danach wurde der Timer **nie wieder** nachgeladen ✓ ✓ ✗ ✓ ✓.

## Die Reparatur ✓ ✓

1. **Timer neu geladen** ✓ ✓ (`systemctl daemon-reload && systemctl restart …` ✓ ✓) — er hat die
   **überfällige** Runde **nachgeholt** ✓ ✓ (`Persistent=true` ✓ ✓) ✓ ✓ und ist wieder armiert ✓ ✓
   (`next_elapse` = 11.11.2026, 09:00 UTC ✓ ✓) ✓ ✓.
2. **Watchdog** ✓ ✓ (`/usr/local/bin/schrott-mcp-checkin-watchdog.sh` ✓ ✓, Cron **alle 15 Minuten** ✓ ✓):
   er liest den gespeicherten Termin ✓ ✓, vergleicht ihn mit der Uhr ✓ ✓ und lädt die Unit neu ✓ ✓,
   **wenn** er veraltet ist ✓ ✓ ✓ ✓.
   **Schutz gegen Doppelläufe** ✓ ✓: solange der Check-in-Dienst läuft ✓ ✓ (`activating`/`active` ✓ ✓),
   tut der Watchdog **nichts** ✓ ✓ — sonst würde er in dieselbe Session einen zweiten Lauf starten ✓ ✗ ✗ ✓ ✓ ✓.

## Der zweite Ausfall (11.11. 12:20 → 13.11. 12:17 UTC) ✓ ✓ — vier Fehler in der Kette ✓ ✓

Die erste Reparatur ✓ ✓ (11.11., 09:10 UTC ✓ ✓) hat den Timer wieder **einmal** feuern lassen ✓ ✓ —
der letzte Check-in-Bericht ✓ ✓ stammt vom **11.11.2026, 12:29 UTC** ✓ ✓. Danach kam **kein**
weiterer ✓ ✓ ✗ ✓ ✓. Das war **kein** zweiter Uhr-Rücksprung ✓ ✓, sondern eine **eigene** Kette
aus **vier** Fehlern ✓ ✓ ✓ ✓ ✓:

**(1) Der hängende `oneshot` ✓ ✓** — Check-in ✓ ✓ und 6h-Welle ✓ ✓ fahren **dieselbe** Session
✓ ✓. Ist dort gerade eine Runde aktiv ✓ ✓, wartet `opencode run` ✓ ✓ auf deren Ende ✓ ✓. Der
Dienst hatte als `oneshot` ✓ ✓ **kein** Zeitlimit ✓ ✓ (`TimeoutStartUSec=infinity` ✓ ✓) und
blieb **zwei Tage** im Status `activating` ✓ ✓. Ein aktiver Dienst **blockiert den Timer**
✓ ✓ ✗ ✓ ✓ — systemd startet die Unit nicht ein zweites Mal ✓ ✓. Der alte Watchdog ✓ ✓
übersprang **genau diesen Fall** ✓ ✓ („Dienst läuft → nichts tun" ✓ ✓) und heilte ihn deshalb
**nie** ✓ ✓ ✗ ✓ ✓ ✓ ✓ ✓.

**(2) `next_elapse=(null)` ✓ ✓** — liegt der komplette `OnCalendar`-Kalender ✓ ✓ in der
Vergangenheit ✓ ✓, berechnet systemd **gar keinen** Termin mehr ✓ ✓ ✗ ✓ ✓ — auch nicht beim
Neuladen ✓ ✓. `next_elapse` bleibt dann **dauerhaft** leer ✓ ✓, der Timer ist **unwiederbringlich
tot** ✓ ✓ ✗ ✓ ✓ ✓ ✓.

**(3) Die `(null)`-Verarbeitung ✓ ✓** — systemd schreibt den String **`(null)`** ✓ ✓, nicht
`""` ✓ ✓. Der alte Watchdog ✓ ✓ hat ihn als regulären Zeitstempel weitergegeben ✓ ✓;
`date -d "(null)"` ✓ ✓ scheitert ✓ ✓, das Skript verließ sich mit `exit 0` ✓ ✓ — **ohne** zu
heilen ✓ ✓ ✗ ✓ ✓ ✓.

**(4) `OnCalendar=` hebt im Drop-in nicht auf ✓ ✓** — der erste Reparaturversuch ✓ ✓ schrieb ein
leeres `OnCalendar=` ✓ ✓ in den Drop-in ✓ ✓. systemd **wertet die Unit weiterhin mit dem alten
(vergangenen) Kalender** ✓ ✓ aus ✓ ✓ und berechnet **weiterhin keinen** Termin ✓ ✓ ✗ ✓ ✓ ✓ ✓.
Der Fallback braucht deshalb einen **gültigen** Kalender ✓ ✓ **zusätzlich** zum relativen
Intervall ✓ ✓.

### Die Reparatur ✓ ✓

| Baustelle ✓ ✓ | Maßnahme ✓ ✓ | Wirkung ✓ ✓ |
| --- | --- | --- |
| Dienst ✓ ✓ | `TimeoutStartSec=300` ✓ ✓ | Ein hängender `oneshot` wird nach **5 min** beendet ✓ ✓ — er blockiert den Timer nicht mehr ✓ ✓ |
| Dienst ✓ ✓ | `KillMode=control-group` ✓ ✓ | Auch Kindprozesse ✓ ✓ werden beendet ✓ ✓ — keine Waisen ✓ ✓ |
| Dienst ✓ ✓ | `SuccessExitStatus=0 143` ✓ ✓ | Der Timeout-SIGTERM ✓ ✓ gilt als **Erfolg** ✓ ✓ — der Dienst steht nicht dauerhaft auf `failed` ✓ ✓ |
| Timer ✓ ✓ | `OnCalendar=*-*-* *:17:00` ✓ ✓ | Stündlich auf Minute **:17** ✓ ✓ — bewusst **nicht** `:00` ✓ ✓, weil die 6h-Welle per Cron um `:00`/`:06`/`:12`/`:18` ✓ ✓ feuert und beide in **dieselbe** Session ✓ ✓ laufen ✓ ✓ |
| Watchdog ✓ ✓ | drei Fälle **getrennt** ✓ ✓ | hängender Dienst → beenden ✓ ✓; veralteter Termin → neu laden ✓ ✓; `next_elapse=(null)` → Drop-in mit gültigem Kalender + `OnUnitActiveSec=3600` ✓ ✓ |

### Der Nachweis ✓ ✓ (13.11.2026) ✓ ✓

- **12:17:20 UTC** ✓ ✓ — der Dienst **feuerte** ✓ ✓; der Prompt kam **12:17:55 UTC** ✓ ✓ in der
  Session an ✓ ✓ (**read-only** in der Session-DB nachgewiesen ✓ ✓).
- **12:22:21 UTC** ✓ ✓ — der Timeout griff **exakt** ✓ ✓: `Main process exited, code=killed,
  status=15/TERM` ✓ ✓, `Failed with result 'timeout'` ✓ ✓, 1,095 s CPU ✓ ✓ / 5 min 263 ms
  Wanduhr ✓ ✓ / 47,3 M Speicherpeak ✓ ✓.
- **Danach** ✓ ✓ — der Timer **armierte sich selbst** neu auf **13:17:00 UTC** ✓ ✓ ✓ ✓.
- Der `(null)`-Pfad ✓ ✓ wurde **end-to-end** ✓ ✓ an einer **Test-Timer-Unit** ✓ ✓ inklusive
  Drop-in-Pfad `$FRAG.d` ✓ ✓ verifiziert ✓ ✓ ✓ ✓.
- **9 verwaiste** `opencode run`-Prozesse ✓ ✓ (PPID 1 ✓ ✓) wurden am 13.11. beendet ✓ ✓, eine
  veraltete Lock-Datei ✓ ✓ bereinigt ✓ ✓.

**Strukturelle Ursache** ✓ ✓ bleibt die **geteilte Session** ✓ ✓ ✓ ✓: Solange Check-in ✓ ✓ und
Welle ✓ ✓ dieselbe Session fahren ✓ ✓, kann ein belegter Session-Lauf ✓ ✓ den Check-in warten
lassen ✓ ✓. Der Timeout ✓ ✓ macht daraus einen **begrenzten** Wartevorgang ✓ ✓ statt eines
**toten** Timers ✓ ✓ ✓ ✓ ✓ ✓ ✓.

## Was nicht geändert wurde ✓ ✓

- Der **Inhalt** des Check-ins ✓ ✓ (`prompts/hourly-checkin.md` ✓ ✓) ✓ ✓ — unverändert ✓ ✓.
- Die **Session** ✓ ✓ — beide Jobs (Check-in ✓ ✓ und 6h-Welle ✓ ✓) fahren **dieselbe** Session ✓ ✓
  (`ses_f1df15abcffexPcSRnZfKPoUgK` ✓ ✓) ✓ ✓.
- Der **Mechanismus** ✓ ✓ bleibt der systemd-Timer ✓ ✓ — er wird nur **geheilt** ✓ ✓ statt durch ein
  zweites Rad (Cron ✓ ✓) ersetzt ✓ ✓, damit Check-in und Welle sich nicht ins Gehege kommen ✓ ✓ ✓ ✓ ✓.
