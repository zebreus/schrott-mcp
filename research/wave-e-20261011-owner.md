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

## Süd-Handoff eingegangen — Prüfung, noch keine Integration

Süd-Session abgeschlossen: 20 Dossiers, Bericht im eigenen Worktree. Owner
gelesen: Rücknahme von Verzeichnisfills, vier Statuswechsel auf `pruefung`,
Primärbefunde und ausdrücklich begrenzte Such-/Abruflage. Die beiden anderen
Wellen-Shards bleiben hiervon unberührt; gemeinsamer Gate noch ausstehend.

Eigener struktureller Check gegen `ade525fd`: exakt die 20 zugeteilten Dossiers
geändert, Slugs/state/city/GEO unverändert, erlaubte status/trader_type-Werte,
alle ursprünglichen Timeline-Suffixe vollständig erhalten, datierte neue Notes,
keine Überblick-Platzhalter. Main-Dossierstände noch bytegleich zur Shardbasis.
Das ist keine Quellen-/Identitätsfreigabe.

Read-only Produktionsvergleich der zurückgenommenen Frontmatterwerte:
**neun Dossiers**, sämtliche dort geleerten Werte weiterhin in Prod vorhanden,
alle neun mit gespeicherter GEO-Paarung:

- Pohl: street/postcode/phone
- Delis: street/postcode/phone/opening_hours
- Pfeifer: street/postcode/phone
- Müller: street/postcode/phone/email/opening_hours/description
- Kleinschmidt: street/postcode/phone
- Hammerschmidt: street/postcode/phone/email
- Zerseeger: street/phone
- Pfaff: street/postcode/phone
- Mertel: street/postcode/phone

Audit außerhalb git: `/root/Documents/wave-e-sued-owner-clear-audit-20261011.json`.
**Keine Clears oder GEO-Änderungen angewendet.** Leere Dossierfelder lassen
gespeicherte Anreicherungen/Adresse/GEO bestehen; `clear_fields` erlaubt nur
bestimmte Enrichmentfelder, nicht street/postcode/GEO. Owner entscheidet nach
Quellenreview ausdrücklich über zulässige Clears. Kein falscher Claim, dass
eine spätere Dossierintegration diese Altwerte automatisch entfernen würde.

Unabhängiges Quellen-/Overwrite-Review für Süd gestartet:
`ses_ed6691d04ffeAxw6LMOhYuD3AJ`, eigener externer Bericht
`/root/Documents/wave-e-sued-independent-review-20261011.md`.
Scope: alle 20 Quellen-/Identitäts-/Overwritefälle, insbesondere Scheuer-
Vergütungsbehauptung, STALCANT-Termin/350kg, Seiler-Postanschrift und Müller-
Rechtsträgertrennung. Reviewer schreibt keine Dossiers/Main/Prod. Owner arbeitet
separat an Seed-/Clear-/GEO- und Strukturprüfung; keine Duplikatrecherche.
Bis Abschluss aller Shards und Gates keine gemeinsame Commit-/Deployfreigabe.

## Fortsetzung nach Nutzungslimits

Nordwest, Ost/Mitte und das unabhängige Süd-Quellenreview endeten mit
`usage limit reached`, nicht mit einer abschließenden Freigabe. Erhaltene
Nordwest-/Ostberichte gelesen: beide dokumentieren bereits alle 20 Dossiers
und Seedtests; das ersetzt weder saubere Übergabe noch Owner-Quellenprüfung.
Alle drei **gleichen Sessions** wurden fortgesetzt und haben die Fortsetzung
angenommen. Auftrag: vorhandene Artefakte/Fehler prüfen, nur fehlende Gates
abschließen, keine Neuerhebung oder Wiederholung bereits erledigter Arbeit.
Die beiden Rechercheberichte erwähnen README-Änderungen; Scope offenlegen
und prüfen lassen, nichts pauschal verwerfen oder alte Main-Dateien ersetzen.

Server-Check bei dieser Fortsetzung: active, sechs Stunden keine Warnungen,
Feedback unverändert bis ID 6080, Bestand 3,881 / 54 / 2,426. Read-only
Sechs-Handler-Prüfer erneut vollständig grün (112 Current):

| Handler | Preise | Letzter erfolgreicher Schritt UTC, 11.10.2026 |
|---|---:|---|
| ELNO | 36 | 04:20:22 |
| Trapper | 21 | 05:20:39 |
| AGH | 8 | 08:22:10 |
| Saxonia | 4 | 06:06:06 |
| Hofmann Rastatt | 23 | 08:52:23 |
| reGOLD | 20 | 06:21:11 |

Keine Quellen-/Einheiten-/Datums-/Canaryfehler im bounded Releaseprüfer.
GitHub-Anmeldung weiterhin fehlend, keine passenden GEO-Logs. Die geprüften
pending Releaseartefakte wurden nicht installiert; Produktion unverändert.
Keine Wellen-Dossiers auf Main übernommen, keine Produktions-Clears angewendet.
