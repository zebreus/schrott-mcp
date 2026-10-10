# MSG Gotha: Owner-Fallback, Sichtbarkeit und verbleibender Staffelvertrag

## Ausgeführt

Nach Nutzungslimit-Fehler des MSG-Agenten übernimmt der Owner selbst. Aktuellen Parser, Scrape-Mapping, Current-Schreibpfad und Store-Migrationsmuster gelesen. Dieser Commit behebt ausschließlich die Webflow-Sichtbarkeit; **nicht die vollständige Mengenstaffelabdeckung**.

Regression `hidden_cms_prices_never_become_quotes` vor Fix ausgeführt: rot, ausgeblendete erste Alu-Staffel wird als `ab 1 kg:` statt sichtbarer `ab 100 kg:` erfasst. Ursache: Parser wertete alle `pricedisplay`-Elemente unabhängig von `w-condition-invisible` aus. Fix prüft diese Klasse am Element und an seinen Vorfahren; entfernt versteckte Alternativzellen vor Extraktion. Genau drei sichtbare Zellen (Schwelle, Betrag, €) erforderlich, andernfalls diagnostischer Skip. Hidden-Dummypreise werden nicht mehr nur zufällig durch fehlendes Katalogmapping ausgefiltert.

Regressionen: ausgeblendete erste bepreiste Staffel, versteckter Kartenwrapper, verschobene versteckte t-Alternative. Bisherige Tests für leere/Nullpreise, Fensterbegrenzung, unverständliche Schwellen und Mapping bleiben grün. Keine neue allgemeine CSS-Sichtbarkeitsengine beansprucht; der geprüfte Vertrag betrifft Webflows konkrete `w-condition-invisible`-Klasse.

- MSG-Modultests: **6 bestanden**.
- Workspace: **611 bestanden, 0 Fehler, 5 ignoriert**, `/root/Documents/msg-visibility-workspace-20261010.log`.
- Seed-gefilterte Tests: **13 bestanden**, `/root/Documents/msg-visibility-seed-20261010.log`.
- Rustfmt-Check und `git diff --check` grün.
- Echter Live-Handler: HTTP **200**, **45 Preise**, `published_at=None`, `/root/Documents/msg-owner-live-20261010.log`. Indium/Iridium/Kobalt/Molybdän/Niob/Tantal ausdrücklich request-only; keine übernommenen versteckten 10-Euro-Dummywerte. Weiterhin **82 höhere sichtbare Staffeln übersprungen** über alle Karten; diese Summenzahl ist keine Zahl fehlender gemappter Preise.

Keine Dossier-/Katalog-/Storeänderungen, keine Produktionswrites, keine neue Current-Identität in diesem Fix. Push weiterhin fehlende GitHub-Anmeldung; noch nicht deployed.

## Konkrete nächste Implementierung, weiterhin offen

1. **Owned Identität statt Leak:** `ScrapedPrice.variant` ist derzeit `&'static str`. Auf owned `String` oder `Cow<'static, str>` umstellen, sämtliche Konstruktoren explizit anpassen und alle Handler regressionsprüfen. Keine `Box::leak`, keine hardcodierten drei Schwellen. Roh-Händlersorte plus kanonische Mengenschwelle bilden gemeinsam die dynamische Variante; Mengeneinheit bleibt unabhängig von der Preisnormalisierung (100 kg bleibt 100 kg, auch wenn EUR/kg zu EUR/t normalisiert wird).
2. **Preis- und Mengeneinheit trennen:** Der bisherige `unit_of(tier)`-Pfad und die Markt-Plausibilitätskommentare sind kein vollständiger Vertrag. Expliziten €/kg-Seitenhinweis prüfen; sichtbare `ab N kg/t`-Schwellen strikt parsen und kanonisieren, unbekannte/mehrdeutige Formen fail-closed. Nicht aus einer t-Mengenschwelle automatisch EUR/t ableiten. Keine Monotonie erzwingen (Rotguss kann fallende Staffelpreise haben).
3. **Alle sichtbaren Staffeln:** `kept`/`higher_tiers`-Auslassung ersetzen. Alle positiven sichtbaren Preiszeilen behalten, leere CMS-Felder und Request-only getrennt diagnostizieren. Zero-Regel separat fachlich entscheiden, nicht als kostenloser Ankauf oder Ende aller folgenden Angebote voraussetzen. Dedup über Material und vollständige dynamische Identität; identische Beträge verschiedener Schwellen bleiben verschiedene Angebote, widersprüchliche doppelte Identitäten Fehler statt stiller Überschreibung.
4. **Reconciliation vor Veröffentlichung:** Current-Schlüssel ist Händler/Material/Variante (`traders/mod.rs` übergibt Variante unverändert an `NewPrice`). Eine neue Staffelvariante allein lässt alte staffellose Current-Zeilen stehen. Eng begrenzte normale Anwendungsmigration oder atomare Snapshot-Reconciliation benötigt, mit exaktem MSG-Slug/Source und bewiesenen Legacy-Keys; Wasseruhren-Altzuordnung zusätzlich prüfen. Append-only Preisgeschichte erhalten, fremde Quellen/Handler nicht anfassen. Kein direktes Produktions-SQL durch Agenten. Bei Netz-/Parser-/Katalogfehler darf kein Current-Retirement ausgelöst werden. Ohne diese Absicherung keine Staffel-Freigabe.
5. **Tests vor Main-Abnahme:** echter Handler-Outcome-/TempDB-Schreibpfad mit drei Millberry-Staffeln, dynamischer neuer Schwelle (nicht 1/100/1000), erster leerer FE-Staffel, nur einer Bronze-Staffel, fallendem Rotguss, gleichen Beträgen verschiedener Schwellen, ausgeblendeten Dummies/Alternativen/Eltern, unverständlicher Preis-/Mengeneinheit und konflikthaften Duplicate-Keys. Reconciliation muss genau Legacy-Current entfernen, Geschichte bewahren und benachbarte Händler/Quellen behalten; Wiederholung idempotent, Fehler ohne Retirement. Danach gezielte/Workspace-Tests, Live-Inventur einschließlich separater Kataloglücken, koordinierter Redeploy und read-only regulärer Produktions-Scrape.

Die alte Diagnose im Deploy-Worktree bleibt unverändert erhalten. Keine erfolgreiche Agentenabnahme, keine vollständige Staffelumsetzung und keine Produktionskorrektur aus diesem Teilschritt abgeleitet.

## Check-in

Service aktiv, keine Warnungen letzte sechs Stunden; Produktionszahlen 3881 Händler / 54 Materialien / 2426 Current-Preise. Feedback weiterhin maximal ID 6080, kein neuer Bericht. Produktionsdaten nur read-only gelesen. Main war zu Beginn sauber und zehn Commits voraus.
