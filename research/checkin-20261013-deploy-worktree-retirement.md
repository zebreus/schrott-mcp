# Letzter Deploy-Worktree: Owner-Abschluss und historische Einordnung

## Entscheidungsstand 13.10.2026

Geprüft gegen Main `a024a7b6`: Worktree `/root/Documents/schrott-mcp-deploy`,
Branch `owner/dossier-quality-2026-10-07`, HEAD `072cf25c`, 63 Commits zurück,
keine eigenen Commits. Ancestry allein genügte nicht: alle 72 uncommittierten
Dateien wurden gesichert, die Dossier-Diffs und sechs einzigartigen Berichte
in drei unabhängigen Read-only-Reviews vollständig geprüft.

- 29 Pfade bytegleich mit Main: kein zusätzlicher Merge erforderlich.
- 43 Pfade abweichend: sechs historische Berichte werden unverändert erhalten;
  die übrigen älteren Dossier-/Codefassungen werden **nicht** pauschal integriert.
- `gutzmann.rs` enthält keinen fehlenden funktionalen Fix. Main verhindert
  bereits Tottlebener Hauptsitz-Enrichment für Gotha, hat zusätzlich den
  Adress-ohne-Kontakt-Regressionstest und die aktuellen Outcome-/Varianttypen.
- Ältere Dossiers würden spätere belegte Korrekturen zurücknehmen: unter anderem
  ALLDEMONT, Klass/Weiler, All-in/Weber/Müller, TSR/Veolia und SchrottJungs.
  Main-Felder, `clear_fields` und spätere Owner-Timelines bleiben unverändert.

## Unverändert erhaltene historische Berichte

Die Berichte beschreiben ihren damaligen Stand, **nicht** den heutigen
Handler-/Produktionszustand. Historische Quellen und Zählwerte wurden nicht
nachträglich umgeschrieben. Spätere Freigaben sind separat zu lesen:

| Originalbericht | Spätere Main-Evidenz |
|---|---|
| [MKM 08.10](checkin-20261008-mkm-price-coverage.md) | [Typed Artikelintegration/Release-Gate](checkin-20261011-mkm-rollout-verifier.md): 60 Artikel, 55 Zahlen-/5 Anfrageartikel; alter 11/55-Gap historisch. |
| [MSG 08.10](checkin-20261008-msg-price-tiers.md) | [Owner-Abnahme](checkin-20261010-msg-tier-implementation.md): sichtbare Mengenstaffeln, 124 gemappte Angebote und scoped Legacy-Reconciliation. |
| [SDM 08.10](checkin-20261008-sdm-price-coverage.md) | [Gotha-Adressfix](checkin-20261010-gutzmann-owner-fallback.md): Standortanreicherung korrigiert; Tottleben bleibt Büro, kein ungeprüft duplizierter Preisstandort. |
| [Welle D Mitte/Nordost](wave-20261009d-mittenordost.md) | [Owner-Gate](checkin-20261010-waveD-mittenordost-owner-gate.md), Commit `2dbf69f4`. |
| [Welle D Nordwest](wave-20261009d-nordwest.md) | [Owner-Gate](checkin-20261010-waveD-nordwest-owner-gate.md), Commit `b72b3869`. |
| [Welle D Südost](wave-20261009d-suedost.md) | [Owner-Gate](checkin-20261010-waveD-suedost-owner-gate.md), Commit `a181e3e1`; später [SchrottJungs-Integration](handler-owner-schrottjungs-integration-20261013.md). |

Die ursprünglichen Dossierfassungen bleiben vollständig im geprüften Archiv,
nicht als operative Frontmatter-Rücksetzungen. Insbesondere erhalten bleiben:
MKM-Transport-/Zahlungsbedingungen und ursprünglicher Coverage-Repro;
MSG-45→124-Diagnose; SDM-Hof-/Büroabgrenzung und damalige Coverage-Zählung;
RSHs ausdrücklich partielle Einkaufs-AGB-Sichtung (NE grundsätzlich 14 Tage,
FE/Gießereischrott 30 Tage des Folgemonats, keine vollständige Verkaufs-AGB-Prüfung);
der historische MSG-Abschlussanhang im Wolf-Bericht. Diese alten Beobachtungen
sind ohne erneute Quellenprüfung keine aktuellen Konditionszusagen.

## Nicht durch Worktree-Cleanup erledigte Folgefragen

- SDM: `Kernschrott S3` und unspezifische `Batterien` weiterhin nicht gemappt.
  Historische 200/100 EUR/t sind keine frisch verifizierten Preise.
  `batterien-blei` existiert inzwischen im Katalog, beweist aber keine Chemie
  der SDM-Position. Kein erratenes Mapping als Voraussetzung fürs Aufräumen.
- MSG: neun positive Staffeln für sechs ungeklärte Sorten bleiben bewusst
  unmapped (Alu-Cu-Kühler, Stahl-/Gussspäne, vier Silber-/versilberte Bestecke).
- Offene Händleridentitäts-, Hofannahme-, Kontakt-/Zertifikatsfragen stehen in
  den verlinkten späteren Owner-Gates. Keine neuen Fills aus Aggregatorleads.
- Ein Main-Merge ist kein Deploymentbeweis: MKM-/MSG-Reconciliation,
  Gotha-Adressheilung, `clear_fields` und neuere Handler benötigen separat
  belegten Rollout und reguläre Produktionsläufe. Kein Forced-Ingest und
  keine SQL-Schreibzugriffe auf Produktionsdatenbanken.

## Integrität und Freigabegrenzen

Vollständiger Snapshot der ursprünglichen 72 Dateien plus Working-/Indexpatch:
`/root/Documents/worktree-archive-20261013/schrott-mcp-deploy-precleanup-20261013.tar.gz`.
SHA-256: `e4be7f308346fa0579b77fdd75d5cb951f7282915c60c0540b44fa90c0182ee0`.
Jede archivierte Datei wurde gegen ihren eigenen Manifest-Hash geprüft.
Die sechs übernommenen Berichte sind nochmals bytegleich mit diesem Archiv
geprüft; die übrigen 66 Originaldateien stimmen weiterhin mit dem Manifest
überein. Alle 155 fremden Main-Dateien stimmen mit dem vorherigen
Hashmanifest überein. Nur die sechs Berichte und dieser Index werden staged.

Read-only-Reviews: `ses_ecaf8b819ffegfBYZ4LL40lPvm` (bb–nw),
`ses_ecaf8929cffe6E9GAl9LSTuxxS` (rp–th/Wolf),
`ses_ecaf8750affedr7UnBVelEgKl1` (sechs Originalberichte).
Owner: Main-Gutzmann-Regressionen 4/4, Workspace-rustfmt und 43 Python-Tests
grün. Vollständiger Main-Workspace: 713 passed, 0 failed, 7 ignored, alle
Doc-test-Suiten abgeschlossen. Der erste Lauf überschritt das 120-s-Toollimit
nach dem Build; der erneute vollständige Lauf mit 600-s-Limit war erfolgreich.
Log: `sh_1350b415e001qQClvDpcKOVYDz.out` im OpenCode-Shell-Artefaktverzeichnis.
Alle lokalen Einordnungslinks und `git diff --cached --check` geprüft.
Eine alte Worktree-Gesamtfassung wurde nicht als aktueller Kandidat getestet
oder freigegeben. Entfernen erst nach Sicherungs-/Prozess-/Commitprüfung.

Check-in: Service aktiv; read-only 3881 Händler/54 Materialien/2426 Current.
Feedback unverändert bis ID 7390. App-WARNs: Münster-Preisleiter fail-closed,
AMR HTTP 502. Remote Main `a024a7b6` direkt bestätigt. Kein neues Deployment.

## Tatsächliche Aufräumprüfung

Nach Commit `dcd34339`, erneuter Prüfung aller 72 Original-Hashes und ohne
aktive Prozessreferenzen wurde der Deploy-Worktree entfernt. Ignoriert war
nur sein generierter `target/`-Ordner. Main-Recherche blieb unverändert.

14 unbenutzte, nachweislich vollständig in Main enthaltene lokale Branches
wurden mit exakten Archiv-Tags unter `archive/local-branch-cleanup-20261013/`
gesichert und regulär gelöscht. Der angehängte Deploy-Branch wurde danach
ebenfalls archiviert und gelöscht; seine alte Upstream-Bindung wurde gelöst,
weil Git trotz bewiesener Main-Ancestry sonst die Löschung verweigerte.

Der alte MKM-Branch `ccd4dc79` wurde ausdrücklich **nicht** zurückgemergt:
seine zwei historischen Kandidatencommits sind im exakten Tag
`archive/worktree-cleanup-20261013/schrott-mcp-mkm-full-20261010` erhalten.
Der geprüfte Main-Diff zeigt statt seiner Kupfer-only-Registrierung den später
integrierten typed Artikelpfad, zusätzliche fail-closed Prüfungen und atomare
Recording-Seam. `d9fa958f` und `1c8326e7` sind Main-Vorfahren; der aktuelle
Workspace testet diesen verbesserten Pfad. Der ersetzte Kandidatenbranch
wurde nach Tag-/Ancestry-/Worktreeprüfung gelöscht, nicht dessen Historie.

Ergebnis lokal: genau **ein Worktree und ein Branch, jeweils Main**.
Remote-Inventur ergab zusätzlich zwei vollständig in Main enthaltene Branches:
`copilot/security-audit` (`00663c7b`, 247/0 Divergenz) und
`owner/dossier-quality-2026-10-07` (`530538e7`, 97/0 Divergenz).
Ihre Löschung wird ausschließlich mit gesicherten Tags und exakten
Expected-tip-Leases publiziert; der konkrete Pushausgang ist separat zu prüfen.
Dokumentations-/Ref-Cleanup erfordert keinen neuen Produktionsrollout.
