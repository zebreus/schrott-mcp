# Tiefenrecherche-Welle — Owner-Gate, 22.10.2026

Die drei Recherche-Shards mit zusammen 60 Dossiers sind abgeschlossen. Dieses
Manifest rekonstruiert den Umfang aus dem aktuellen Dossier-Diff und gruppiert
ihn für die Owner-Prüfung in drei bundesland-disjunkte Mengen zu je 20. Eine
frische read-only DB-Prüfung fand alle 60 Slugs weiterhin in Produktion und
weiterhin passend zur Wellenbedingung `website='' OR street=''`.

## Scope-Manifest

### Owner-Gruppe A — BE/BW/BY/NI (20)

`be-berlin-schrott-renate-und-peter`, `be-neukolln-klemm`,
`bw-backnang-albert-gogel-schrott-gogel`,
`bw-freudenstadt-heinzelmann-containerdienst`,
`bw-ohmden-wiedmann-gucklhorn-schrott-und-metallhan`, `bw-pforzheim-agosi-ag`,
`bw-schwabisch-gmund-karus-arnold-schrott-und-metalle`,
`bw-ulm-eselsberg-reinhardt-schrotthandel`, `bw-willstatt-lehmann-willi-schrott`,
`by-bamberg-schrott-florian`, `by-lohr-a-main-97816-digosa-metall`,
`by-munchen-r-muller`, `by-oberasbach-mm-kat`,
`by-oberkotzau-fattigau-95145-grundl-av`, `by-rotal-schubert`,
`by-u-a-munchen-schrottjungs-mobil-bundesweit`, `by-wenzenbach-meier`,
`ni-northeim-remhof-metallgrohandel-auch-duderstadt`,
`ni-nortmoor-schrott-abholservice-m-mettbach`,
`ni-varel-schrottentsorgung-varel-wilhelmshaven`.

### Owner-Gruppe B — HE/NW/SL/TH (20)

`he-dillenburg-herborn-35683-schafer-und-scheffel-sds`,
`he-gieen-schrotthandel-gieen`, `he-gladenbach-35075-ezedelmetall`,
`he-hanau-63456-pfragner-georg`, `he-heidenrod-schrottmeister-rhein-main`,
`he-offenbach-buchhugel-schrotthandel-marco-hombach`,
`he-seligenstadt-63500-juwelier-dittmeier`, `he-wiesbaden-65199-hubinger`,
`nw-bochum-schrotthandler-1`, `nw-detmold-schmidt-peter-tino`,
`nw-essen-seliger`, `nw-ibbenburen-fuhrer`,
`nw-lippstadt-a-r-schrottabholung-lippstadt`, `nw-lippstadt-dorozala`,
`nw-niederkruchten-wilms`, `nw-paderborn-franz-gawor`,
`nw-wuppertal-elberfeld-ewald-lietmann`,
`sl-bexbach-oberbexbach-bexbacher-buntmetallverwertung`,
`th-sonneberg-gertz-altmetallhandel`,
`th-walldorf-b-meiningen-haid-bernd-schrotthandel-und-containerdi`.

### Owner-Gruppe C — HB/MV/RP/SH/SN/ST (20)

`hb-bremen-28215-aurichalum`,
`hb-bremen-woltmershausen-28197-bav-bremer-autohandels-und-verwertungs`,
`mv-schwaan-schrottankauf-schwaan`, `mv-wiek-rossner-joachim`,
`rp-idar-oberstein-55743-edelmetallkontor`,
`rp-koblenz-s-alten-st-maternus-str`, `rp-maxdorf-a-a`, `rp-nassau-stoll-nassau`,
`sh-bad-oldesloe-holt-d-er-mann`,
`sh-kasseedorf-aldenhoff-maximilian-schrott-u-stahlhand`,
`sh-lensahn-milon-sonke-schrott-u-autorecyclinghof`,
`sn-elstertrebnitz-schrott-wetzel-depot-elstertrebnitz`,
`sn-klingenthal-08248-korner-schrott-container`,
`sn-olbernhau-09526-sieber-gerd-metallrecycling-u-containerd`,
`sn-riesa-scholz-recycling-betrieb-riesa`,
`sn-thiendorf-01561-metallhalle-schrottmax-dienstleistungsse`,
`sn-zwickau-gottfried-wunderlich`, `st-lutherstadt-eisleben-rce`,
`st-molau-1-fred-sonnenberg`,
`st-mrr-namensnahe-prufen-mrr-mitteldeutsche-rohstoff-recycling`.

## Owner-Gate und GEO-Entscheidungen

- Die Dossiers wurden auf Quellenbezug, erhaltene Recherchehistorie,
  Frontmatter-/Enum-/Seed-Validität, Preisrichtung, Kontakt- und
  Annahmebedingungen sowie Website-Deep-Links geprüft. Neue Timeline-Einträge
  sind datiert. In Ohmden und Eisleben bleiben nur unterstützte
  `clear_fields`-Tokens; die Seed-Codegenerierung verarbeitet das ganze Korpus.
- `bw-backnang-albert-gogel-schrott-gogel`: alter Produktionspunkt
  48.9474/9.4372 ersetzt durch 48.9477513/9.4248526. Der OSM-Knoten 8660018128
  trägt Albert-Gogel-Namen, Betreiber Thomas Auch, Telefon und Betreiberwebsite;
  die Website bestätigt Fridastraße 5 als Werks-/Anlieferadresse. Die
  Punktgenauigkeit (Betriebsobjekt, kein bestätigtes Einfahrtstor) ist im
  Dossier festgehalten.
- Für `by-rotal-schubert`, `by-u-a-munchen-schrottjungs-mobil-bundesweit`,
  `by-lohr-a-main-97816-digosa-metall`, `by-munchen-r-muller` und
  `st-lutherstadt-eisleben-rce` löst Reverse-Geocoding die vorhandenen
  Produktionspunkte auf sachfremde Gebäude/Orte auf. Die Dossiers weisen
  `clear_fields: geo` aus; ein Ersatzpunkt wird nicht aus einer bloßen
  Verzeichnisadresse oder einem getrennten Partner abgeleitet.
- `he-wettenberg-krofdorf-blum-marco-schrottentsorgung`: das alte GEO lag
  574 m vom ungeklärten Adresskandidaten entfernt und reverse-geocodiert auf
  ein anderes Wohngebäude. `clear_fields: geo` wurde bereits mit einer
  lokalen Seed-Regression gegen die alte Koordinate abgesichert.
- Zusätzliche Owner-Korrekturen außerhalb der 60er-Welle: In Aschersleben
  teilten `st-aschersleben-schrott-und-metallhandelsgesellschaft-as`,
  `st-aschersleben-isr-recycling` und `st-aschersleben-lorenz` denselben
  Produktionspunkt 51.7574/11.4608. Reverse-Geocoding zeigt ein Wohnhaus an der
  Herrenbreite 13b. Die belegten Betriebsadressen liegen anderswo; für den
  Metallhandel liefert die Adresssuche nur eine Straßenlinie, für ISR keinen
  adressgenauen Eingang. Alle drei Dossiers ziehen den unpassenden Punkt
  deshalb explizit zurück, ohne Koordinate zu raten.
- `by-aschaffenburg-alzenau-ak-containerdienst`: gespeicherte Koordinate führt
  zum Jugendkulturzentrum, nicht zum Containerdienst. Die Homepage belegt
  Aschaffenburg im Liefergebiet, nicht Alzenau; die AGB nennen den
  Metall-Anlieferplatz in Rödermark (Hessen). Der Datensatz bleibt als
  BY-Servicegebietszeile erhalten, `city` wird auf Aschaffenburg verengt,
  Straße/PLZ bleiben leer und der falsche GEO-Punkt wird explizit gelöscht.
- Der exakte MRR-Datensatz hatte bei der frischen read-only Abfrage bereits
  `lat=NULL, lon=NULL`; deshalb wurde dafür kein unnötiger GEO-Clear ergänzt.

## Verifikation und Veröffentlichung

- `CARGO_BUILD_JOBS=1 cargo test -p schrott-mcp-ingestion`: **685 bestanden,
  0 fehlgeschlagen, 7 ignoriert**.
- `cargo test -p schrott-mcp-ingestion seed -- --nocapture`: **14 bestanden**;
  insbesondere wird die Löschung eines alten GEO-Paars über den normalen
  Seed-Pfad geprüft.
- `rustfmt --edition 2021 --check crates/ingestion/build.rs
  crates/ingestion/src/seed_traders.rs`, `git diff --check` und statische
  Frontmatter-Prüfung der geänderten Dossiers: bestanden.
- `cargo fmt --all --check` ist repo-weit weiterhin nicht sauber, weil
  unveränderte Dateien u.a. in `goldtrans.rs`, `mkm_metals` und
  `crates/store/src/public/prices.rs` Formatierungsdifferenzen aufweisen.
  Diese fremden Baseline-Dateien wurden nicht massenhaft umformatiert.
- Keine Produktionsdatenbank beschrieben. Kein Seed-Lauf gegen Produktion,
  kein Push und kein Redeploy. Vor diesem Changeset lag `main` 36 Commits vor
  `origin/main`; der veröffentlichte Diff umfasst bereits 1.041 Dateien und
  25.586 zusätzliche Zeilen. Push würde diese unabhängige Queue mitveröffent-
  lichen und bleibt daher gesperrt. Die geprüften Änderungen werden lokal in
  einem Commit gesichert; der Arbeitsbaum wird anschließend kontrolliert.
