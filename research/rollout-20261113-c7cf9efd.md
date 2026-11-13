# Rollout `c7cf9efd` — 13.11.2026, 13:24:58 UTC

**Kandidat:** `c7cf9efddf14ebe244d7b287a03be7ce560236da` ✓ — Tree
`424944ce3dfb71e5178788ee70e5dc3f238dd0f7` ✓
**Vorgänger (live):** `fff9176ab7f5d143e1f23d1f2cbd1dd3d4cc88d8` ✓ (Rollout 11.11.2026, 12:20:16 UTC ✓)
**Art:** **Dossier-Release** ✓ ✓ — **60** Dossiers ✓ ✓ geändert ✓ ✓ (HE **20** ✓ · NW **20** ✓ ·
RP **20** ✓), `crates/` ✓ ✓ **byte-identisch** ✓ ✓ zum Vorgänger ✓ ✓
**Rollout:** `scripts/release/rollout.py` ✓ — Neustart **13:24:58 UTC** ✓ ✓, `health` ✓ ✓
(HTTP 200 ✓ ✓), `post_rollout_run` **4557** ✓ ✓ `ok` ✓ ✓ ✓
**Backup:** `rollout-c7cf9efd-20261113T132457Z` ✓ ✓ — `public.db` ✓ ✓ (`cb316956…` ✓ ✓,
76 222 464 Bytes ✓ ✓), `internal.db` ✓ ✓ (`aca18e59…` ✓ ✓) ✓ ✓ + die beiden alten Binaries ✓ ✓

## Warum dieser Release nötig war ✓ ✓

Die **6h-Welle vom 11.11.2026, 12:00 UTC** ✓ ✓ hat **60** Dossiers ✓ ✓ in **HE/NW/RP** ✓ ✓
recherchiert ✓ ✓ (Bericht ✓ ✓: `research/wave-20261111-1200-report.md` ✓ ✓) ✓ ✓. Erfundene Werte
sind **keine** drin ✓ ✓: **0** unbelegte €/kg-Ankaufskurse ✓ ✓, **16** neue Negativbefunde ✓ ✓
mit Reichweitenklausel ✓ ✓, **5** Koordinaten ✓ ✓ mit OSM-Objekt **und** zweiter Quelle ✓ ✓ ✓ ✓.

Die Welle berührt **zwei** Zeilen des release-gepinnten Wave-E-Scopes ✓ ✓
(`he-langenselbold-63505-levy-schrott-metall` ✓ ✓ und `nw-koln-reimann` ✓ ✓) — die **elf Skalare**
beider Zeilen bleiben **unverändert** ✓ ✓ ✓ ✓. Die Basis
`scripts/wave-e-20261028-0600-expectations.json` ✓ ✓ ist deshalb **mechanisch** revidiert ✓ ✓
(`scripts/release/revise_wave_e_pin.py` ✓ ✓, **Text-Patch** ✓ ✓, `selfcheck: ok` ✓ ✓) ✓ ✓ — Befund
**B1** ✓ ✓, **siebter** Fall ✓ ✓ ✓ ✓.

## Unabhängige Pre-Deploy-Prüfung ✓ ✓

**Urteil des Reviewers: BEFUND** ✓ ✓ — **kein** rollout-relevanter Befund **am Kandidaten** ✓ ✓ ✓.

Der Reviewer war ein **eigenständiges Agent** ✓ ✓ ohne Freigabe-Absicht ✓ ✓ — read-only ✓ ✓,
kein Deployment ✓ ✓, keine Änderung im Repo ✓ ✓, kein `cargo build` ✓ ✓ ✓.

| Prüfung | Ergebnis |
|---|---|
| **Kandidat = geprüfter Commit** ✓ ✓ | **6723** Blob-Hashes identisch ✓ ✓, `diff -rq` gegen `git archive c7cf9efd` **0** Unterschiede ✓ ✓ ✓ |
| **Exit-Codes** ✓ ✓ (nicht nur die letzte Zeile ✓ ✓) | `build.log` **0** ✓ ✓, `workspace-tests.log` **0** ✓ ✓, `fmt-check.log` **0** ✓ ✓ — **839** passed / **0** failed / 13 ignored ✓ ✓ ✓ |
| **Binär-Digests** ✓ ✓ | server `ec4b47cb…` ✓ ✓, worker `40efa801…` ✓ ✓ — beide **exakt** == Manifest ✓ ✓ ✓ |
| **Manifest-Selbstkonsistenz** ✓ ✓ | Delta **60** ✓ ✓, `unexplained_changes` **leer** ✓ ✓, Clear-Check **269** Zeilen / **0** Verstöße ✓ ✓, zweiter Boot **0** Zeilen ✓ ✓, Worker **60/60** ✓ ✓, `production_untouched` ✓ ✓ |
| **Wave-E-Basis mechanisch revidiert** ✓ ✓ | **0** Skalar-Differenzen über alle **60** Zeilen ✓ ✓, Reihenfolge identisch ✓ ✓, **genau 2** Hash-Abweichungen ✓ ✓ ✓ ✓ |
| **Verifier** ✓ ✓ | Vertragstests **8/OK** ✓ ✓ und **7/OK** ✓ ✓ · **Negativtest** ✓ ✓: verfälschter Erwartungswert → **exit 1** ✓ ✓ (in `/tmp/opencode/` ✓ ✓, keine Repo-Datei geändert ✓ ✓) ✓ ✓ |
| **Die fünf Gates** ✓ ✓ | alle **exit 0** ✓ ✓ gegen `fff9176a` ✓ ✓ — **0** Slug-Änderungen ✓ ✓, **0** angefasste Vorwellen-Notizen ✓ ✓ ✓ ✓ |
| **Inhalt der 60 Dossiers** ✓ ✓ | **0** erfundene Änderungs-Behauptungen ✓ ✓, **0** unbelegte €/kg-Kurse ✓ ✓, **0** Firmen-als-Standort-Nachweis ✓ ✓, **372**/**11** Fundstellen ✓ ✓ exakt wie im Bericht ✓ ✓ ✓ |
| **Produktion unberührt** ✓ ✓ | traders **3945** ✓ ✓, prices **77644** ✓ ✓, current_prices **4070** ✓ ✓, `user_version` **12** ✓ ✓, Fingerprint **exakt** der Manifest-Wert ✓ ✓, `NRestarts=0` ✓ ✓ ✓ |

### Die drei Befunde des Reviewers und die Owner-Entscheidung ✓ ✓

| Befund | Schwere | Owner-Entscheidung |
|---|---|---|
| **B1** — `RELEASE_TARGET_SLUGS` bedeutet in `post_rollout_check.py` die **Geo-Clear-Liste** ✓ ✓, nicht den Release-Scope ✓ ✓ | mittel ✓ ✓ | **Akzeptiert und behoben** ✓ ✓ — die Geo-Liste wird **nicht** aus dem Dossier-Frontmatter (`clear_fields`) abgeleitet ✓ ✓, sondern aus dem **Post-Seed-Rehearsal** ✓ ✓ (dem Release-Delta-Erwartungsmanifest ✓ ✓): **10** Slugs ✓ ✓ mit `geo = null` ✓ ✓. Die aus den Dossiers abgeleitete Liste hätte **9** gehabt ✓ ✓ — ihr fehlte `rp-11880-quellen-lokaleschrottplatz-181-platze-vo` ✓ ✓, eine Zeile, die **nie** eine Koordinate hatte ✓ ✓ ✓ ✓ |
| **B2** — der Rollout selbst braucht den **60er-Scope** ✓ ✓, nicht die Geo-Liste ✓ ✓ | niedrig ✓ ✓ | **Akzeptiert** ✓ ✓ — übergeben wurde `target/predeploy-c7cf9efd/probe-target-slugs.json` ✓ ✓ (**60** Slugs ✓ ✓, **identisch** mit der eigenen 60er-Liste ✓ ✓, Digest **86086e35…** ✓ ✓ == Manifest ✓ ✓) ✓ ✓ |
| **B3** — der **AltfahrzeugV**-Eintrag eines Negativbefunds trägt die Reichweitenklausel nicht **im selben** Eintrag ✓ ✓ | niedrig ✓ ✓ | **Als Auslegungsentscheidung abgewiesen** ✓ ✓ — die Klausel steht im **EfbV**-Eintrag desselben Feldes ✓ ✓ und nennt **beide** Anerkennungen **wörtlich** ✓ ✓ („die Efb- **und die AltfahrzeugV**-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Negativbeweis" ✓ ✓) — in **allen 16** Dossiers ✓ ✓ ✓ ✓. Der Negativbefund ist damit **inhaltlich vollständig** ✓ ✓; eine Textduplizierung in 16 Dossiers wäre eine neue Welle mit eigenem Zyklus und Review ✓ ✓ — dafür gibt es **keinen** Beleg ✓ ✓ |

**Der Kandidat `c7cf9efd` hat damit keinen rollout-relevanten Befund** ✓ ✓ ✓ ✓ — die beiden Befunde
B1/B2 betrafen die **Aufrufparameter** ✓ ✓ und sind behoben ✓ ✓ ✓ ✓ ✓ ✓ ✓.

## Nachprüfung read-only gegen die Produktion ✓ ✓

| Prüfung | Ergebnis |
|---|---|
| `post_rollout_check.py` ✓ | **exit 0** ✓ ✓ — `ok: true` ✓ ✓, `geo_cleared_ok: true` ✓ ✓, Geo-Clear **10**/10 ✓ ✓, `geo_still_present: []` ✓ ✓ ✓ |
| `verify-wave-e-postrollout-c7cf9efd.py` ✓ | **exit 0** ✓ ✓ — **60** Zeilen ✓ ✓, **660** Skalarprüfungen ✓ ✓, `failures: []` ✓ ✓ ✓ |
| `verify-release-delta-c7cf9efd.py` ✓ | **exit 0** ✓ ✓ — **60** Zeilen ✓ ✓, je **12** Skalarprüfungen ✓ ✓, `notes_hash_match` ✓ ✓, `seed_hash_match` ✓ ✓, `geo_match` ✓ ✓, `service_match` ✓ ✓, `failures: []` ✓ ✓ ✓ |
| `verify-pending-corrections-c7cf9efd.py` ✓ | **exit 0** ✓ ✓ — `failures: []` ✓ ✓ ✓ |
| Rollout-Verifikation ✓ | **60** Scope-Zeilen gelesen ✓ ✓, `geo_cleared` **10** ✓ ✓, `geo_still_present` **50** ✓ ✓ (die Zeilen mit Koordinate ✓ ✓), `dropoff_not_allowed`: `rp-heidesheim-taurus-entsorgungs` ✓ ✓ — **bewusst** so ✓ ✓ ✓ |
| Zählwerte ✓ ✓ | traders **3945** ✓ ✓, materials **83** ✓ ✓, prices **77644** ✓ ✓, current_prices **4070** ✓ ✓, trader_materials **1640** ✓ ✓, `user_version` **12** ✓ ✓, **24** FK-Verletzungen ✓ ✓ (**vorbestehend** ✓ ✓) ✓ ✓ |
| Dienst ✓ ✓ | `active` ✓ ✓, `NRestarts=0` ✓ ✓ (Neustart **13:24:58 UTC** ✓ ✓), Journal-WARN (6 h) **keine** ✓ ✓ ✓ |

### Das alte Gate `wave_e_6b3da663` ✓ ✓ ist rot ✓ ✗ ✓ — **vorbestehend** ✓ ✓, kein Befund dieses Rollouts ✓ ✓

Der Post-Rollout-Check führt zusätzlich den **historischen** Pin `6b3da663` ✓ ✓ (Release vom **27.10.**
✓ ✓) mit ✓ ✓ — er ist **rot** ✓ ✗ ✓ (**106** Failures ✓ ✓) ✓ ✓. Das ist **keine** Regression ✓ ✓ ✓:
Der Alt-Pin ist **17 Tage** alt ✓ ✓.

**Objektiv belegt** ✓ ✓: derselbe Verifier ✓ ✓ gegen das **Vor-Rollout-Backup** ✓ ✓ vom
**13.11.2026, 13:24:57 UTC** ✓ ✓ (`rollout-c7cf9efd-20261113T132457Z/data/` ✓ ✓) meldet
**104** Failures ✓ ✓ — er war also **schon vor** diesem Rollout rot ✓ ✓ ✓ ✓ ✓. Die **2** zusätzlichen
Failures ✓ ✓ sind **genau** die von dieser Welle berührte Wave-E-Zeile
`he-langenselbold-63505-levy-schrott-metall` ✓ ✓ (Notiz-Hash ✓ ✓ und Seed-Hash ✓ ✓) — genau das,
was der **neue**, release-gepinnte Verifier `wave_e_c7cf9efd` ✓ ✓ abdeckt ✓ ✓ (**exit 0** ✓ ✓ ✓) ✓ ✓.

Der Alt-Pin zählt **nicht** in das `ok`-Kriterium ✓ ✓ des Checks ✓ ✓; maßgeblich ist der
release-gepinnte Verifier `wave_e_c7cf9efd` ✓ ✓ — **exit 0** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓.

## Was noch zu geschehen hat ✓ ✓

Der **Release-Scope** ✓ ✓ dieser Welle ✓ ✓ umfasst **60** Zeilen ✓ ✓ — **25** davon ändern
Frontmatter-Felder ✓ ✓, **35** tragen **ausschließlich** die neue Recherche-Notiz ✓ ✓ (HE **4** ✓ ·
NW **13** ✓ · RP **18** ✓ ✓ ✓); bei allen **60** hat sich `notes` ✓ ✓ geändert ✓ ✓ ✓ ✓.
(Der Reviewer hatte **4** Dossiers ohne Frontmatter-Änderung genannt ✓ ✓ — **selbst nachgemessen**
✓ ✓ sind es **35** ✓ ✓ ✓ ✓.)

**Offene Owner-Entscheidungen** ✓ ✓ (aus dem Wellen-Bericht ✓ ✓, **nicht** rollout-relevant ✓ ✓):
`he-langenselbold-63505-levy-schrott-metall` ✓ ✓ (geschlossen führen oder mit Anschrift nachziehen ✓ ✓) ·
`he-pohlheim-vonderheid-steffen-schrott` ✓ ✓ (Koordinate **4,3 km** daneben ✓ ✓ nachziehen ✓ ✓) ·
`he-wiesbaden-doringer-frank-schrotthandel` ✓ ✓ (Flash-Domain ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓.

Bericht ✓ ✓: `research/wave-20261111-1200-report.md` ✓ ✓ ·
`research/checkin-20261108-1600-owner.md` ✓ ✓.
