# Rollout `be496cef` — 08.11.2026, 19:42:49 UTC

**Kandidat:** `be496cef00a792ccda93dc84f5c115b99c1e0654` ✓ — Tree `f42c802c1c6c235ba4c6a6d115ed5e9d09e0db24` ✓
**Vorgänger (live):** `f7c962d65a18e5f760715f8e4f1ad3a0e72dc276` ✓ (Rollout 08.11.2026, 17:47:54 UTC ✓)
**Scope:** **76** Dossiers ✓ — **60** aus der 6h-Welle 09.11.2026, 00:00 ✓ ✓ + **16** Geo-Negativbefunde ✓ ✓
**Rollout:** `scripts/release/rollout.py` ✓ ✓ — Neustart **19:42:49 UTC** ✓ ✓

## Unabhängige Pre-Deploy-Freigabe ✓

**Urteil: GRÜN** ✓ ✓ — der Prüfer hat **unabhängig** geprüft ✓ ✓:

| Prüfung | Ergebnis |
|---|---|
| `git diff f7c962d6 be496cef -- crates/` | **leer** ✓ — kein Produktionscode geändert ✓ ✓ |
| Tree-Binding | **6614/6614** Dateien identisch zum Kandidatenbaum ✓ ✓ (git blob-SHA-1 je Datei ✓) |
| Binär-Digests | Server `0bad4ea0…` ✓, Query-Worker `3ab0d051…` ✓ = Manifest ✓ ✓ |
| **Seed-Nachbau** aus den Dossiers | **3945** Zeilen ✓, **0** Feldunterschiede ✓, **0** Notizunterschiede ✓ → **identisch** ✓ ✓ |
| Gates (in detached Worktree ✓) | alle **exit 0** ✓ ✓ (`check_note_order` ✓, `check_pre_wave_notes` ✓, `wave_gate` ✓, `check_cert_balance` ✓, `check_note_claims` ✓) ✓ |
| Workspace-Tests | **835** passed ✓, **0** failed ✓ ✓ (inkl. der Seed-Tests ✓) ✓ |
| Upgrade-Probe (eigene Kopie ✓) | genau **76** Zeilen geändert ✓, **0** außerhalb des Scopes ✓, **0** unerklärte Änderungen ✓ ✓ |
| Historie | `first_seen_at` in **76/76** unverändert ✓ ✓ |
| **Zertifikate** | **4** übernommene ✓ ✓ live gegen das amtliche Register ✓ und das originale Zertifikats-PDF ✓ ✓ geprüft ✓ — Beleg ✓, Vorgangsnummer ✓, Zertifikatsnummer ✓, Gültigkeit ✓ und **ausschließlich** die gekreuzten Anlagenbogen-Tätigkeiten ✓ ✓ stimmen exakt ✓ ✓; die **4** nicht übernommenen ✓ ✓ (Greifswald ✓, Nachfolger ✓, historisch ✓, andere Adresse ✓ ✓) stehen nachweislich **nicht** im Feld ✓ ✓ |
| Abrufdatum | **76/76 = 08.11.2026** ✓ ✓ — kein 09.11.2026 ✓ ✓ |
| Produktion | **unberührt** ✓ ✓ — der Rollout hatte **nicht** stattgefunden ✓ ✓ (Prüfung vor dem Rollout ✓) ✓ |

## Die zwei Befunde des Prüfers ✓ — beide behoben ✓

| # | Schweregrad | Fundstelle | Behandlung |
|---|---|---|---|
| **K1** | kosmetisch ✓ | `research/wave-20261109-0000-report.md` ✓ | Die Zeile zu `nw-oelde-warnecke` ✓ zählte **Behandeln** ✓ zu den gekreuzten Tätigkeiten ✓ und nannte es im selben Klammerteil als **nicht** gekreuzt ✓. **Das Zertifikat `ZZEE003000959.pdf`** ✓ ✓ — vom Prüfer **live** heruntergeladen ✓ ✓ — zeigt: Anlage 1 Sammeln ✓ + Befördern ✓ + Handeln ✓ + Makeln ✓, Anlage 2 Lagern ✓ + Verwerten ✓, Anlage 3 **Behandeln** ✓ + Verwerten ✓ — **nur Beseitigen** ✓ ist **nicht** gekreuzt ✓ ✓. Dossier-Feld ✓ ✓ und Notiz ✓ ✓ waren korrekt ✓ ✓; nur die **eine** Berichtszeile war falsch ✓ ✓ → **korrigiert** ✓ ✓ (Commit `015dc986` ✓ ✓) ✓ |
| **K2** | kosmetisch ✓ | `tools/append_notes.py` ✓, `tools/append_note.py` ✓ | Die Hilfsskripte trugen das Wellen-Slot-Datum **09.11.2026** ✓ ✗, obwohl die Daten korrekt auf **08.11.2026** ✓ ✓ korrigiert waren ✓ ✓ — ein erneuter Einsatz hätte ein **falsches Abrufdatum** ✓ ✗ geschrieben ✓ ✓ → **korrigiert** ✓ ✓ ✓ |

## Was ausgerollt wurde ✓

| Größe | Wert |
|---|---|
| Dossiers | **76** ✓ — 60 Welle ✓ ✓ + 16 Geo-Negativ ✓ ✓ ✓ |
| neue Notizen | **76** ✓ ✓ (60 × `### Recherche 08.11.2026` ✓ ✓, 16 × `### Geokodierung 08.11.2026` ✓ ✓) ✓ |
| Zertifikate im Feld | **4** ✓ ✓ (EfbV Oelde ✓ ✓, EfbV Marktoberdorf ✓ ✓, AfV Kroher ✓ ✓, AfV Wollenzhofen ✓ ✓) ✓ |
| Negativbefunde im Feld | **16** ✓ ✓ + **4** weitere ✓ ✓ (Greifswald ✓, Nachfolger ✓, historisch ✓, andere Adresse ✓ ✓) ✓ |
| Koordinaten gesetzt | **4** ✓ ✓ (OSM-Objekt **und** Nominatim/Photon ✓ ✓) ✓ |
| Geo-Punkte **zurückgenommen** | **1** ✓ ✓ — `he-munster-bierhenkel` ✓ ✓ (Lippetal ✓ → **NULL** ✓ ✓) ✓ |
| Anschriften geändert | **13** ✓ ✓ |
| bezifferte Ankaufskurse | **1** ✓ ✓ (Hartmetall 40–60 €/kg ✓ ✓) ✓ |

## Nachprüfung read-only gegen die Produktion ✓

| Prüfung | Ergebnis |
|---|---|
| `verify-release-delta-be496cef.py` ✓ | **exit 0** ✓ — **76** Zeilen ✓, **912** Skalarprüfungen ✓, **76** Notiz-Hashes ✓, **76** Seed-Hashes ✓, **76** Geo- ✓, **76** Service-Prüfungen ✓ ✓ |
| `verify-wave-e-postrollout-be496cef.py` ✓ | **exit 0** ✓ — **60** Wave-E-Zeilen ✓, **660** Skalarprüfungen ✓, **60** Notiz- ✓, **60** Seed-Hashes ✓ ✓ |
| `post_rollout_check.py` ✓ | **exit 0** ✓ — `ok: true` ✓ ✓ (`geo_cleared_ok: true` ✓ ✓, Counts ✓ ✓) ✓ |
| `verify-pending-corrections-be496cef.py --since 2026-10-27` ✓ | **exit 0** ✓ — `ok: true` ✓ ✓ ✓ |
| `verify-pending-corrections-be496cef.py --since 2026-11-08T19:42:49Z` ✓ | **rot** ✗ — **MSG** ✓ ✓ und **Gutzmann** ✓ ✓ müssen **nach** dem Rollout einmal laufen ✓ ✓; ihre Rotation ist **6 Stunden** ✓ ✓ (letzte Läufe 16:18 ✓ ✓ bzw. 14:00 ✓ ✓ → nächste ≈ **22:18** ✓ ✓ bzw. ≈ **20:00** ✓ ✓) ✓ — **reine Rotationsbedingung** ✓ ✓, kein Datenbefund ✓ ✓ |

### Produktionsstand nach dem Rollout ✓

| Größe | Wert |
|---|---|
| `traders` | **3945** ✓ ✓ |
| `current_prices` | **4041** ✓ ✓ |
| `prices` (Historie) | **47450** ✓ ✓ |
| `materials` | **83** ✓ ✓ |
| `trader_materials` | **1636** ✓ ✓ |
| `user_version` | **12** ✓ ✓ |
| FK-Verletzungen | **24** ✓ ✓ — **unverändert** ✓ ✓ (vorbestehend ✓ ✓) ✓ |
| `PRAGMA integrity_check` | **ok** ✓ ✓ |

`he-munster-bierhenkel` ✓ ✓: `lat`/`lon` **51.6992941/8.0388823** ✓ → **NULL** ✓ ✓ ✓, `postcode` `''` ✓ → `'64839'` ✓ ✓ ✓.

## Dienst ✓

- Neustart **19:42:49 UTC** ✓ ✓, `NRestarts` **0** ✓ ✓, Status **active** ✓ ✓
- `/health` ✓ → `{"ok":true,"service":"schrott-mcp"}` ✓ ✓
- Installierte Binaries ✓ = Manifest-Digests ✓ ✓ (`0bad4ea0…` ✓ / `3ab0d051…` ✓) ✓
- Backup ✓: `rollout-be496cef-20261108T194248Z` ✓ ✓ — Backups wieder auf **5** ✓ ✓ getrimmt ✓ ✓

## Anmerkung zur Verwechslung in der Nachprüfung ✓

`post_rollout_check.py` ✓ prüft die **Geo-Clear-Zeilen** ✓ ✓, nicht den Release-Scope ✓ ✓ —
`RELEASE_TARGET_SLUGS` bedeutet dort etwas **anderes** ✓ ✓ als beim Rollout ✓ ✓ ✓. In diesem Release
tragen **zwei** Dossiers `clear_fields` ✓ ✓ mit `geo` ✓ ✓: `by-munchen-autoverwertung-kroher` ✓ ✓
(Geo-Rücknahme ✓ ✓, `phone` ✓ bleibt ✓ ✓) ✓ und `he-munster-bierhenkel` ✓ ✓ — beide stehen in der
Produktion **ohne** Koordinate ✓ ✓ ✓ (`geo_cleared_ok: true` ✓ ✓) ✓ ✓.
