---
slug: th-zella-mehlis-wahl-co-inh-thomas-madeiski
name: Wahl & Co. Inh. Thomas Madeiski e.K.
trader_type: metallhaendler
state: TH
city: Zella-Mehlis
street: Heinrich-Ehrhardt-Str. 47b
postcode: '98544'
phone: '03682 483449'
email: wahl-@t-online.de
opening_hours: Mo-Fr 8:00-14:30, Wochenende geschlossen
website: https://wahlundco.de/
website_status: aktiv
status: aktiv
description: Altmetallankauf und Schrotthandel für Privat- und Gewerbekunden ab 0,5 kg; Container nach Vereinbarung.
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Thüringen (TH) — Recherche
provenance_ankauf_raw: ja (ab 0,5 kg, Bargeld sofort)
provenance_origin: table
---

# Wahl & Co. Inh. Thomas Madeiski e.K.

## Überblick

Der aktuelle Betreiberauftritt bestätigt Wahl & Co. Inhaber Thomas Madeiski e.K., HRA 503773, den Standort Heinrich-Ehrhardt-Straße 47b und den Kontakt. Es werden Metallankauf, Sortierung und Container nach Vereinbarung für Privat- und Gewerbekunden angeboten. Die Homepage zeigte am 21.10.2026 um 16:44 Uhr einen Preisstand; ausgewiesen sind jedoch unverbindliche, tagesabhängige Richtwerte und Preisbereiche, keine garantierten Ankaufskurse. Der Handler-Fix für die neue CMS-Sortenpreisliste und Preisübersicht ist in `main` enthalten, aber noch nicht in Produktion ausgerollt. Die 32 Produktionsbeobachtungen stammen aus 03:30 UTC und liegen vor dem Seitenpreisstand; sie sind nicht als aktuelles Angebot zu behandeln. Keine Produktionsdaten geändert.

## Timeline

### Quellen- und Scrape-Prüfung 21.10.2026

- [Owner-Prüfung 21.10.2026: Der hinterlegte Abruf `https://wahlundco.de/` leitet auf `https://www.wahlundco.de/` um; die aktuelle Homepage und das Impressum nennen weiterhin Wahl & Co. Inh. Thomas Madeiski e.K. Der Beispielpreis-Abschnitt ist ausdrücklich mit Preisstand 04.11.2025 versehen. Eine getrennte, undatierte FAQ-Sortenübersicht ist als unverbindlicher, tagesabhängiger Richtwert markiert; verbindliche Tagespreise sollen telefonisch erfragt werden. Im aktuellen HTML fehlt der vom Handler benötigte Abschnitt `Tagesaktuelle Sortenpreise`; der reguläre Lauf #2326 übernahm deshalb null Preise und fail-closed mit „Sortenpreisliste fehlt“. Der erfolgreiche Step #2302 mit 32 Preisen ist nur ein früherer Abruf, kein aktueller Preisnachweis. Keine Listenwerte als heutige Kurse übernommen; weder Handler noch Produktionsdaten wurden geändert. Quellen: [aktuelle Homepage](https://www.wahlundco.de/), [Impressum](https://www.wahlundco.de/impressum.html).]

### Owner-Nachprüfung 21.10.2026 (Preisstand und Handler)

- [Owner-Nachprüfung 21.10.2026: Die aktuelle Betreiberhomepage und das Impressum wurden erneut direkt abgerufen. Das Impressum nennt Wahl & Co. Inhaber Thomas Madeiski e.K., HRA 503773, Heinrich-Ehrhardt-Straße 47b, 98544 Zella-Mehlis, Telefon 03682 483449 und E-Mail wahl-@t-online.de; die Homepage nennt Annahme ab 0,5 kg für Privat und Gewerbe sowie Container nach Vereinbarung. Der sichtbare Preisstand der Richtpreis-Übersicht lautet 21.10.2026, 16:44 Uhr. Sie kennzeichnet ihre Materialpreise als unverbindliche Richtwerte/Bereiche; verbindliche Tagespreise sind telefonisch oder vor Ort zu klären. Beispielsweise zeigt die aktuelle FAQ für Kupfer-Draht Millberry 10,50 €/kg als Richtwert, nicht als garantierten Kurs. Die frühere Einordnung zum Preisstand 04.11.2025 beschreibt somit einen vorherigen Seitenstand. Der Handler sucht weiterhin nach dem alten Marker `Tagesaktuelle Sortenpreise`; im Lauf #2374 scheiterte er mit `Sortenpreisliste fehlt`. Die 32 bestehenden Produktionszeilen haben alle `observed_at=2026-10-21T03:30:55Z`; davon sind 30 als `exact` und 2 als `upto` markiert. Insbesondere steht Millberry dort als exakter Wert mit 10,10 €/kg, gegenüber dem aktuellen Seitenrichtwert 10,50 €/kg. Die Beobachtungen stammen vor dem sichtbaren Seitenpreisstand und sollten nicht als aktuelle Angebote behandelt werden. Der Handler muss die neue Seitenstruktur, Richtwert-/Bereichssemantik und Datumsbasis erst korrekt abbilden und testen; keine Preise manuell in die Produktionsdatenbank geschrieben. Quellen: https://wahlundco.de/ ; https://wahlundco.de/impressum.html .]

### Owner-Nachprüfung und lokaler Handler-Fix 22.10.2026, 04:14 UTC

- [Die Live-Homepage und das Impressum wurden erneut direkt abgerufen. Der Auftritt nennt weiterhin Wahl & Co. Inhaber Thomas Madeiski e.K.; das Impressum bestätigt HRA 503773, Heinrich-Ehrhardt-Straße 47b, 98544 Zella-Mehlis, Telefon 03682 483449 und wahl-@t-online.de. Die Homepage enthält jetzt eine CMS-Sortenpreistabelle mit 31 €/kg-Richtwerten, eine separate Übersichtstabelle mit 14 Preisbereichen und den maschinenlesbaren Preisstand `2026-10-21T14:44:01Z`. Die 31 Sortenpreise sind ausdrücklich unverbindlich und tagesabhängig; sie werden im lokalen Handler deshalb als `approx` mit Konfidenz 0,5 abgebildet. Elf eindeutig zuordenbare Kategorien werden als `range` mit beiden Bounds und Konfidenz 0,5 erfasst. Der einzelne Übersichts-Einzelpreis wird durch die detailliertere Sortenzeile ersetzt; Rotguss (Kollision mit der separat bepreisten Rotguss-Schrott-Sorte), Blei-Akkus und Kabel & E-Schrott bleiben als generische Bereiche unzugeordnet. Der zusammengesetzte Kabelposten wird nicht fälschlich `kabel-kupfer` zugeschlagen. Die Tests des Ingestion-Pakets bestehen (681 bestanden, 7 ignoriert), aber die Änderung ist nicht deployed. Die 32 vorhandenen Produktionspreise sind weiterhin Beobachtungen vom 21.10.2026, 03:30:55 UTC. Der normale Handler-Pfad schreibt Preise per Upsert und entfernt nicht automatisch fehlende Varianten; daher bleiben die frühere `kabel-kupfer`-`upto`-Zeile und andere nicht mehr belegte Schlüssel eine separate, noch zu lösende Datenabgleichsfrage. Keine Produktionsdaten verändert. Quellen: [Homepage](https://wahlundco.de/), [Impressum](https://wahlundco.de/impressum.html).]

### Owner-Nachprüfung 26.10.2026, 11:00 UTC

- [Homepage und Impressum heute direkt erneut abgerufen. Der Betreiberauftritt
  zeigt weiterhin die CMS-Sortenpreistabelle, die Materialpreis-Übersicht,
  Annahme ab 0,5 kg und unverbindliche, tagesabhängige Richtwerte. Der
  maschinenlesbare Preisstand ist unverändert `2026-10-21T14:44:01Z`, also fünf
  Tage alt. Die Werte wurden nicht als aktuelle Tageskurse übernommen. Quellen:
  https://wahlundco.de/ ; https://wahlundco.de/impressum.html.]
- [Read-only-Live-Test des aktuellen Handlers
  `live_handler_parses_the_complete_current_snapshot` bestanden: 40 Preise
  einschließlich der Sorten- und Preisbereichssemantik; der Test liest Quelle
  und Impressum und schreibt keine Daten. Die deterministischen Tests des
  Handler-Moduls bestanden ebenfalls: 16 bestanden, 1 absichtlich ignorierter
  Live-Test.]
- [Produktionslauf **#2806** (10:51 UTC) war `partial`: 75 Preise,
  1 Händlerfehler. Das Produktionsjournal meldet weiterhin den alten Parsertext
  `Sortenpreisliste fehlt`. Der von `schrott-mcp.service` gestartete Pfad ist
  `/usr/local/bin/schrott-mcp-server`; dessen Änderungszeit ist 17.10.2026,
  vor dem aktuellen Handlerstand und dessen Live-Snapshot-Guard
  (`a7ecccdd`, 23.10.2026). Zusammen mit dem erfolgreichen aktuellen Live-Test
  ist die Ursache damit ein nicht ausgerolltes Produktionsbinary, kein
  reproduzierbarer Fehler im aktuellen Handler.]
- [Der read-only-Abgleich der aktuellen Wahl-Preise bestätigt weiterhin 32
  Current-Zeilen, alle `exact`, ohne `published_at`, beobachtet am
  21.10.2026 03:30:55 UTC. Millberry steht weiter als exakter Wert 10,10 EUR/kg.
  Das ist älter als der Betreiberseitenstand von 10,50 EUR/kg und darf nicht als
  aktuelles Betreiberangebot ausgegeben werden. Keine Produktionsdaten geändert.
  Die Pre-Deploy-Freigabe ist laut [Owner-Check-in 10:42](../../research/checkin-20261026-1042-owner.md)
  weiter offen; daher kein Deployment, Force-Ingest oder direkter
  Datenbank-Write.]

### Importiert (Seed-Stand 2026-09-30)

- Buntmetall-Spezialist (Cu, Messing, Rotguss, Alu, V2A/V4A, Zinn, HSS, Hartmetall, Wolfram), Kabel, Container
- Adressbeleg: https://wahlundco.de/
