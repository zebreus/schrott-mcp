# Auftrag: 76 Händler mit stehengebliebenen Preisen (Stand 17.11.2026)

**Befund:** **76** Händler führen `current_prices`, die seit **mehr als sieben Tagen** nicht
mehr erneuert wurden ✓ ✓. Der Seed-Import überschreibt die Preise nicht, wenn der Handler
keine liefert ✓ ✓ — die alten Werte bleiben als „aktueller Preis" stehen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Das ist die Arbeitsliste, an der nichts verloren geht** ✓ ✓: die Seiten dieser Händler
**leben** ✓ ✓ (die zwölf größten wurden am 17.11.2026 live geprüft ✓ ✓ — **alle** antworten
mit HTTP 200 ✓ ✓), und die Handler-URLs im Repo sind aktuell ✓ ✓. Es scheitert also an der
**Extraktion** ✓ ✓: die Seiten wurden umgebaut ✓ ✓, die Preise stehen woanders ✓ ✓.

**Das Kriterium ist der letzte erfolgreiche Lauf** ✓ ✓ (nicht ein Zeit-Alter ✓ ✓) —
`scripts/release/find_stale_current_prices.py` ✓ ✓ liefert die Fälle, bei denen der Handler
läuft ✓ ✓, aber einzelne Preise nicht auffrischt ✓ ✓ (die Seite bietet sie nicht mehr an ✓ ✓):

| Händler | stale | laufender Handler | letzte Aktualisierung |
|---|---|---|---|
| `rp-ludwigshafen-mkm-metals` | 56 | ja (Turnus 11:50 UTC) | 13.11.2026 |
| `ni-braunschweig-neuwert-schrott-altmetallhandel` | 3 | ja | 17.11.2026 |
| `bb-lubben-neuendorf-smr` | 6 | ja | 17.11.2026 |
| `hh-st-georg-schiefer-co-edelmetall-scheideanstalt` | 2 | ja | 17.11.2026 |

**Die übrigen 72** Händler haben einen Handler, der **seit Tagen oder Wochen nicht mehr
durchläuft** ✓ ✓ (30 seit September ✓ ✓, 13 seit Oktober ✓ ✓, 29 seit November ✓ ✓) ✓ ✓

## Die Vorgehensweise je Händler ✓ ✓

1. `live_handlers -- <slug>` laufen lassen ✓ ✓ — das Werkzeug **liest nur** und schreibt
   nichts ✓ ✓ (kein Force-Ingest ✓ ✓).
2. Schlägt er fehl, die Seite holen ✓ ✓ und mit dem **Fixture** des Handlers vergleichen ✓ ✓
   (liegt unter `crates/ingestion/src/traders/handlers/fixtures/` ✓ ✓).
3. Ist die Seite umgebaut ✓ ✓, den **Selektor** anpassen ✓ ✓ und ein **neues Fixture** aus
   dem Live-Capture anlegen ✓ ✓ (sanitisiert ✓ ✓) — die bestehende Vertragsprüfung
   (`overview.rs` ✓ ✓ / `recording.rs` ✓ ✓) bleibt **fail-closed** ✓ ✓.
4. Tests ✓ ✓, Release-Zyklus ✓ ✓, Pre-Deploy-Freigabe ✓ ✓, Rollout ✓ ✓.

**Nicht** die Preise von Hand in die Datenbank schreiben ✓ ✓ — der Preis muss **belegt**
aus der Betreiberseite kommen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Die Liste ✓ ✓

| Händler | Preise | letzter Lauf | Handler-Datei |
|---|---|---|---|
| `th-gotha-msg-metallrecycling-msg-schrott-und-meta` | 124 | 2026-10-17 | `msg_metallrecycling_gotha.rs` |
| `ni-braunschweig-neuwert-schrott-altmetallhandel` | 77 | 2026-11-06 | `neuwert.rs` |
| `by-aschaffenburg-bernhard-westarp` | 69 | 2026-11-09 | `westarp.rs` |
| `by-schaftlarn-frisch-recycling` | 68 | 2026-10-26 | `frisch_recycling.rs` |
| `by-aschaffenburg-huth-schrott-metall-huth` | 63 | 2026-11-09 | `huth_aschaffenburg.rs` |
| `ni-friesoythe-h-henken-friesoythe-auch-cloppenburg-pap` | 46 | 2026-09-30 | `henken_friesoythe.rs` |
| `bb-schoneiche-hein-schrotthandel` | 45 | 2026-11-09 | `hein_schrotthandel.rs` |
| `ni-saterland-nfh-zickura` | 40 | 2026-09-30 | `nfh_zickura_saterland.rs` |
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` | 40 | 2026-10-27 | `wahl_co_zella_mehlis.rs` |
| `by-regensburg-kernschrott-recycling` | 38 | 2026-11-04 | `kernschrott.rs` |
| `be-pankow-kiro-schrotthandel-kafedzhiev` | 37 | 2026-09-30 | `kiro.rs` |
| `be-marzahn-hellersdorf-mk-wertstoffhandel-daniel-hartwig` | 34 | 2026-09-29 | `mk_wertstoffhandel.rs` |
| `bb-strausberg-koppe-andreas-schrottplatz-strausberg` | 32 | 2026-10-12 | `koppe_strausberg.rs` |
| `be-neukolln-beller-demontagen-altmetall-schrott` | 31 | 2026-10-09 | `beller.rs` |
| `nw-herdecke-eas-recycling-solution` | 30 | 2026-09-29 | `eas_recycling_herdecke.rs` |
| `sn-leipzig-albus-leipzig` | 30 | 2026-11-05 | `albus_leipzig.rs` |
| `be-spandau-elno-container-und-dienstleistungs` | 28 | 2026-11-05 | `?` |
| `sn-rochlitz-db-recycling-denis-blum` | 28 | 2026-11-05 | `db_recycling.rs` |
| `bb-brandenburg-an-der-havel-mc-schrott-annahmestelle` | 27 | 2026-11-05 | `mc_schrott.rs` |
| `bb-kloster-lehnin-ot-reckahn-mc-schrott-annahmestelle` | 27 | 2026-11-05 | `mc_schrott.rs` |
| `bb-nauen-mc-schrott-annahmestelle` | 27 | 2026-11-05 | `mc_schrott.rs` |
| `bb-rathenow-mc-schrott-annahmestelle` | 27 | 2026-11-05 | `mc_schrott.rs` |
| `bb-werder-ot-plotzin-mc-schrott-annahmestelle` | 27 | 2026-11-05 | `mc_schrott.rs` |
| `mv-rostock-mc-schrott-rostock` | 27 | 2026-11-05 | `mc_schrott.rs` |
| `nw-krefeld-josef-hendrichs-metallhandels` | 26 | 2026-09-28 | `hendrichs_krefeld.rs` |
| `sn-frankenberg-volker-lungwitz-schrotthandel` | 26 | 2026-11-05 | `lungwitz.rs` |
| `sn-zschopau-wertstoff-bauer-inh-micha-bauer` | 26 | 2026-11-06 | `wertstoff_bauer.rs` |
| `nw-bonn-pur-umwelt-p-u-richter` | 25 | 2026-11-05 | `pur_umwelt_bonn.rs` |
| `sl-saarbrucken-66113-katalysator-hai-marke-der-s-b-recycling` | 25 | 2026-09-28 | `katalysator_hai.rs` |
| `by-kulmbach-trapper` | 21 | 2026-11-09 | `?` |
| `hb-woltmershausen-vedder-stockrahm` | 19 | 2026-09-29 | `vedder.rs` |
| `ni-delmenhorst-vedder-stockrahm` | 19 | 2026-09-29 | `vedder_delmenhorst.rs` |
| `nw-dormagen-altmetalle-kraft` | 19 | 2026-11-06 | `altmetalle_kraft.rs` |
| `nw-dusseldorf-bohner-altmetalle` | 19 | 2026-11-06 | `boehner_altmetalle.rs` |
| `rp-bad-kreuznach-geld-fur-gold-dagmar-muller-edelmetallko` | 19 | 2026-11-05 | `geld_fuer_gold.rs` |
| `th-gotha-sdm-gutzmann` | 19 | 2026-09-28 | `gutzmann.rs` |
| `sn-altmittweida-schrott-buntmetallankauf-buntmetallhande` | 18 | 2026-11-07 | `buntmetallhandel_altmittweida.rs` |
| `sn-dresden-2nd-way-recycling` | 18 | 2026-11-06 | `second_way.rs` |
| `be-marzahn-hellersdorf-efrem-gouchev-schrottankauf` | 16 | 2026-11-09 | `gouchev.rs` |
| `nw-koln-niehl-mkr-rothenbucher` | 16 | 2026-11-06 | `mkr_rothenbuecher.rs` |
| `bb-lubben-neuendorf-smr` | 15 | 2026-11-04 | `smr.rs` |
| `ni-braunschweig-cederbaum-privatkunden-annahme` | 15 | 2026-09-28 | `cederbaum.rs` |
| `nw-doerentrup-limekon` | 15 | 2026-11-04 | `limekon.rs` |
| `ni-rotgesbuttel-schrottplatz-rotgesbuttel` | 14 | 2026-11-09 | `schrottplatz_roetgesbuettel.rs` |
| `rp-florsheim-dalsheim-schrott-metallhandel-doring` | 14 | 2026-09-28 | `doering.rs` |
| `st-gerwisch-gerwischer-rohstoffrecycling-verwertungs` | 13 | 2026-09-30 | `gerwischer.rs` |
| `bb-neulewin-oder-metalle-schrotthandel-oderbruch` | 11 | 2026-11-06 | `oder_metalle.rs` |
| `by-u-a-munchen-schrottjungs-mobil-bundesweit` | 11 | 2026-10-17 | `schrottjungs.rs` |
| `nw-koln-wkr` | 11 | 2026-10-27 | `wkr_koln.rs` |
| `he-frankfurt-griesheim-ms-recycling-frankfurt` | 9 | 2026-09-29 | `ms_recycling_frankfurt.rs` |
| `ni-vechelde-uwe-hanusa-schrott-und-metallhandel` | 8 | 2026-09-29 | `hanusa_vechelde.rs` |
| `ni-wangerland-hooksiel-26434-manuel-triebsch-autoverwertung-altmetall` | 8 | 2026-09-28 | `schrott_triebsch.rs` |
| `bb-potsdam-kulisch-co-fahrzeug-handels-und-verwertu` | 7 | 2026-09-29 | `kulisch_co.rs` |
| `he-frankfurt-am-main-schrott-frankfurt-scholz` | 7 | 2026-09-28 | `schrott_frankfurt.rs` |
| `nw-bochum-gerthe-44805-schrottabholung-zentrale` | 7 | 2026-09-28 | `schrottabholung_zentrale.rs` |
| `sn-oschatz-oschatz-recycling-oschatzer-recycling-un` | 7 | 2026-09-28 | `oschatz_recycling.rs` |
| `bb-lauchhammer-ost-lausitz-recycling` | 6 | 2026-10-14 | `lausitz.rs` |
| `bw-rastatt-hofmann` | 6 | 2026-10-21 | `hofmann_rastatt.rs` |
| `he-frankfurt-am-main-schrott-recycle-meikel` | 6 | 2026-11-06 | `schrott_recycle_meikel.rs` |
| `hh-hammerbrook-madi-metall-recycling` | 6 | 2026-09-28 | `madi.rs` |
| `ni-pattensen-klix-recycling` | 6 | 2026-10-22 | `klix_recycling_pattensen.rs` |
| `ni-rosengarten-madi-metall-recycling` | 6 | 2026-09-28 | `madi.rs` |
| `nw-essen-vogelheim-tappe-rohstoffhandel` | 6 | 2026-10-28 | `tappe.rs` |
| `sn-groposna-metcera` | 6 | 2026-09-29 | `metcera.rs` |
| `be-treptow-kopenick-amr-metalle-und-rohstoffe-amr-schrottpla` | 4 | 2026-09-28 | `amr_schrottplatz.rs` |
| `bw-freudenstadt-72250-vana-schrotthandel-entrumpelung` | 4 | 2026-10-28 | `vana.rs` |
| `he-hattersheim-kupferhelden` | 4 | 2026-09-30 | `kupferhelden.rs` |
| `nw-remscheid-vhm-hartmetall-ankauf` | 4 | 2026-11-06 | `vhm_hartmetall.rs` |
| `sn-oelsnitz-vogtl-schrottmobil-inh-ralf-weel` | 4 | 2026-09-28 | `schrottmobil_oelsnitz.rs` |
| `bb-cottbus-mobiler-schrotthandel-cottbus-mobschrott` | 3 | 2026-09-28 | `mobschrott.rs` |
| `be-reinickendorf-antik-art-berlin-an-und-verkauf` | 3 | 2026-11-03 | `antikart.rs` |
| `be-marzahn-hellersdorf-papierfritze-schrott-altpapierankauf` | 2 | 2026-09-30 | `papierfritze.rs` |
| `hh-hohenfelde-trauschmuck-sperling` | 2 | 2026-10-17 | `sperling.rs` |
| `ni-marxen-metallankauf24-andre-owsianski-ne-spezia` | 2 | 2026-11-07 | `metallankauf24.rs` |
| `nw-nettetal-edelcat` | 2 | 2026-09-28 | `edelcat.rs` |
| `bw-karlsruhe-76133-scheideanstalt-karlsruhe` | 1 | 2026-09-28 | `scheideanstalt_ka.rs` |
**Zusammengefasst** ✓ ✓: **76** Händler ✓ ✓ · **1608** stehengebliebene Preiszeilen ✓ ✓ ·
**4** davon mit laufendem Handler ✓ ✓ (davon einer — MKM ✓ ✓ — ist seit dem Release vom
17.11. 07:52 UTC ✓ ✓ gefixt ✓ ✓ und aktualisiert sich zum nächsten 6h-Turnus ✓ ✓ um **11:50
UTC** ✓ ✓) ✓ ✓
