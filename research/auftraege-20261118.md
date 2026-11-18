# Nachprüfung d129bec – 18.11.2026, 10:25 UTC: rot

Rollout: 08:05:17 UTC, Wellen 00:00 und 06:00 UTC (je 60 Dossiers) und drei Zertifikatskorrekturen.

**Gutzmann:** `verify-pending-corrections-d129bec.py --data-dir /var/lib/schrott-mcp --since 2026-11-18T08:05:17Z` lieferte `"ok": false` (Exit 1). Einzige Fehlermeldung:

> th-gotha-sdm-gutzmann: successful post-rollout step missing or changed contract

Ursache für das rote Gate: In `internal.db` fehlt der nach dem Rollout erwartete Gutzmann-Schritt (~08:08 UTC). Der letzte belegte Schritt war erfolgreich, lag aber vor dem Rollout. Warum kein neuer Schritt erfasst wurde, ist nicht belegt. Auftrag: Ausführung und Protokollierung des Handlers prüfen und das Gate nach dem nächsten Lauf erneut ausführen.

| Handler | Letzter `started_at` (UTC) | `status` | `items_upserted` | Bewertung |
|---|---|---|---:|---|
| `th-gotha-sdm-gutzmann` | 2026-11-18T02:08:45.247367456Z | ok | 19 | Vor Rollout; fälliger Nachweis fehlt |
| `th-gotha-msg-metallrecycling-msg-schrott-und-meta` | 2026-11-18T10:21:36.673813773Z | ok | 124 | Nach Rollout bestätigt |
| `rp-ludwigshafen-mkm-metals` | 2026-11-18T05:25:09.596504701Z | ok | 56 | Vor Rollout; nach Fälligkeit (~11:25 UTC) erneut prüfen |

**Übrige Prüfungen:** Release-Delta, Wave-E-Postrollout und Produktionsnotizen (Scope `research/release-20261118-0000-0600-scope.json`) meldeten jeweils `"ok": true`. `public.db`: 3945 Händler, 98406 Preise, `user_version=12`.

| Zertifikat (`slug`) | `length(certifications)` |
|---|---:|
| `hh-schnelsen-walter-behncke-inh-bernd-behncke` | 681 |
| `st-dessau-rolau-horst-habermann-transport-und-altmetallh` | 445 |
| `th-schmalkalden-jager-jurgen` | 242 |

`schrott-mcp`: `ActiveState=active`, `SubState=running`, `NRestarts=0`, `ExecMainStartTimestamp=Wed 2026-11-18 08:05:17 UTC`.
