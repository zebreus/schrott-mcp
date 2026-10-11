# Recherchewelle E — 2026-10-11

Status: drei Rechercheagenten gestartet, Ergebnisse noch nicht freigegeben.
Basis `ade525fd`. Keine Dossieränderungen auf Main in diesem Startcheckpoint.

## Auswahl und Isolation

60 verschiedene Slugs am **2026-10-11 06:00:44 UTC** frisch aus Produktionspool
`website='' OR street=''` ausgewählt: je 20 mit `ORDER BY RANDOM()` aus drei
disjunkten Bundeslandgruppen. Datenbank URI mode=ro, query_only=ON. Tatsächliche
Dossierpfade geprüft statt sie aus state zu erraten; Stuhr-Slug im hb-Verzeichnis
hat beispielsweise Produktionsstate NI. Kein Stammdaten-/GEO-Fix daraus geraten.

Auswahlmanifest `/root/Documents/wave-e-20261011-selection.json`, SHA256
`c1260bbc4c4ba1cc7c7f7e3e6fb09227a9a7ce9e935bbe75bb26dff533646f03`.
Der erste Auswahlversuch wurde wegen einer zu engen Pfadannahme vor Delegation
verworfen; die unten dokumentierte Auswahl wurde neu direkt aus Prod gelesen.

Agenten arbeiten ausschließlich in eigenen neuen Worktrees, Dossiers und
jeweiligem Bericht; keine Commits/Push/Deploy/Shared-Code-/Mainwrites, keine
Rust-GEO-Arbeit, Produktion strikt read-only. Rechercheweg bleibt frei, bestehende
Historie und ehrliche Quellen-/Identitätsgrenzen bleiben verbindlich.

### Süd — BY/BW/HE

Session `ses_ed672c93fffesozEpswHK1pI33`.
Worktree `/root/Documents/schrott-mcp-wave-e-sued-20261011`, Branch
`research/wave-e-sued-20261011`. Bericht `research/wave-e-sued-20261011.md`.

- he-hanau-63456-pfragner-georg
- by-peienberg-seiler
- he-bad-emstal-34308-pohl-anita
- he-alsfeld-schrotthandel-entsorgung-delis
- he-limburg-65549-scheuer
- by-konigsberg-anger
- by-neustadt-a-d-aisch-stirling-industrie-metallhandel
- by-wurzburg-versbach-97078-pfeifer-rainer-schrotthandel
- by-munchen-r-muller
- he-nidderau-ab-recycling-bjoern-asal
- he-langenselbold-63505-levy-schrott-metall
- he-spangenberg-34286-heinrich-kleinschmidt-rohprod
- by-ichenhausen-michael-hammerschmidt
- he-dietzenbach-stalcant-schrotthandler
- he-darmstadt-wixhausen-der-zerseeger-schrotthandel
- he-darmstadt-nord-hartmann-recycling
- he-eschborn-65760-pfaff-alexander
- he-wetzlar-jori-recycling-schrott-wetzlar
- he-fulda-dietershan-schrotthandel-stephan-goldbach
- by-sennfeld-97526-mertel-autoverwertung

### Nordwest — NW/NI/HB/HH/SH

Session `ses_ed672c92affeDfY9I0BICANovh`.
Worktree `/root/Documents/schrott-mcp-wave-e-nordwest-20261011`, Branch
`research/wave-e-nordwest-20261011`. Bericht `research/wave-e-nordwest-20261011.md`.

- nw-gummersbach-mar-schrottabholung
- nw-essen-schuttler
- nw-gutersloh-bartsch
- ni-langwedel-auto-recycling-pankowski
- nw-essen-cbr-bracke
- hh-stellingen-georg-lensch-sohne
- ni-hildesheim-autoverschrottung-hildesheim
- nw-koln-reimann
- nw-bochum-wattenscheid-schrotthandler-plus
- sh-lubeck-topschrott
- nw-unna-mairitsch
- nw-essen-kollner
- nw-koln-mulheim-a-f-altleben
- hh-allermohe-van-der-meulen-edelmetaal-b-v
- ni-st-andreasberg-wilfried-joachim-thiermann
- hb-stuhr-thomas-seidel-schrotthandel
- nw-wesel-jack-schrotersk
- hh-billbrook-alt-und-buntmetallhandel-dejan
- nw-lennestadt-l-mulaj
- nw-koln-chorweiler-altmetalle-reinhard

### Ost/Mitte — BE/BB/MV/RP/SL/SN/ST/TH

Session `ses_ed672c923ffex7Jj09e1TFY0Uz`.
Worktree `/root/Documents/schrott-mcp-wave-e-ost-mitte-20261011`, Branch
`research/wave-e-ost-mitte-20261011`. Bericht `research/wave-e-ost-mitte-20261011.md`.

- st-querfurt-tsr-recycling-standort-querfurt
- rp-oberzissen-56651-reinhardt
- th-bad-tennstedt-schrottabholdienst-scheffler-mario
- mv-demmin-oetjen-rohstoffhandel
- rp-mayen-sk-schrotthandel-serviceleistungen
- sl-lebach-autoverwertung-volz-renate-volz
- mv-neubrandenburg-karsten-bobzien-schrotthandel-abbruch-ba
- th-zedlitz-07557-entsorgung-und-recycling-bernd-zauritz
- rp-worms-67549-kampfer
- th-weira-reinisch-u-reinisch
- be-marzahn-hellersdorf-schrotthandel-achtert
- th-gera-stublach-07552-ostthuringer-recycling-und-handels
- sn-weiwasser-uwe-bellmann-schrott-buntmetall
- rp-kaiserslautern-67655-kratz-pfandkredit
- st-kothen-06366-mrr-schrottplatz-kothen
- sn-kirchberg-08107-containerdienst-kaminski-sohn
- st-gardelegen-ot-jeseritz-herms-recycling-schrott-abbruch-entrumpe
- st-sandau-schrotthandel-r-thieke
- sn-grorohrsdorf-ot-bretnig-schrottaufbereitung-rosler
- be-steglitz-zehlendorf-rich-herbig-metallhandel-berlin

## Ausstehender gemeinsamer Owner-Gate

Nach Abschluss aller drei Shards: vollständiger Quellen-/Identitäts- und
Overwrite-/Historien-Scan, Enum-/Dossier-/Deep-Link-/Emdash-/Notenformatprüfung,
Geo-Bedarf dokumentieren; Änderungen geprüft zusammenführen, Seed-Tests,
ein gemeinsamer Wellen-Commit, authentifizierter Push, neu gebaute beide Binaries,
koordiniertes Deployment und read-only Produktions-Spotchecks. Bereits gebaute
Releaseartefakte enthalten diese neuen Dossierergebnisse **nicht**; bei einer
späteren Seedänderung ist ein neuer Build erforderlich.

Check-in beim Start: Service active, sechs Stunden keine Warnungen, Feedback
unverändert bis 6080, Bestand 3,881 Händler / 54 Materialien / 2,426 Current.
Kein laufender Rechercheerfolg oder Veröffentlichungsnachweis aus dem bloßen
Agentenstart abgeleitet. Andere bestehende Worktrees unverändert erhalten.
