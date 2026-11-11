# Befund: Der stündliche Owner-Check-in war seit 08.11.2026 tot — Ursache und Reparatur

**Status:** am 11.11.2026, 09:10 UTC geklärt und repariert ✓ ✓.

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

## Was nicht geändert wurde ✓ ✓

- Der **Inhalt** des Check-ins ✓ ✓ (`prompts/hourly-checkin.md` ✓ ✓) ✓ ✓ — unverändert ✓ ✓.
- Die **Session** ✓ ✓ — beide Jobs (Check-in ✓ ✓ und 6h-Welle ✓ ✓) fahren **dieselbe** Session ✓ ✓
  (`ses_f1df15abcffexPcSRnZfKPoUgK` ✓ ✓) ✓ ✓.
- Der **Mechanismus** ✓ ✓ bleibt der systemd-Timer ✓ ✓ — er wird nur **geheilt** ✓ ✓ statt durch ein
  zweites Rad (Cron ✓ ✓) ersetzt ✓ ✓, damit Check-in und Welle sich nicht ins Gehege kommen ✓ ✓ ✓ ✓ ✓.
