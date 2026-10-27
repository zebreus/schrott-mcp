# Owner-Disposition - Quellenreviews und Seed-Korrekturen, 27.10.2026

## Reviewumfang

Vier unabhängige Reviews bezogen sich auf den Seed-/Dossierstand `7a431dcb`:
109, 107 und 107 disjunkte Bestandszeilen sowie die 43 neu hinzugefügten Slugs.
Die Bestandsreviews dokumentierten Quellenalter und selektive Live-Prüfungen;
nicht jede historische URL wurde erneut abgerufen. Die 43 neuen Zeilen wurden
einzeln klassifiziert. Keine Review schrieb in Produktion oder in die
Arbeitskopie.

- Shard 1: 109 Zeilen, 365 Felder; 194 gestützt, 165 begründete Clears, sechs
  mehrdeutige Felder.
- Shard 2: 107 Zeilen, 306 Felder; 165 gestützt, 139 begründete Clears, zwei
  mehrdeutige Felder.
- Shard 3: 107 Zeilen; keine weitere beanstandete Änderung. GEO-Clears sind
  Rücknahme eines nicht belegten Punktes, kein Schließungsnachweis.
- 43 neue Slugs: 31 im Umfang der Dossiers gestützt, drei beanstandet und neun
  mit unzureichender Beleglage. Die neun bleiben ohne neue ungesicherte
  Standort-/Leistungsbehauptung; Unsicherheit ist kein Beweis, dass eine Angabe
  falsch oder ein Betrieb geschlossen ist.

## Übernommene Korrekturen

- **Brameier, Schopsdorf:** `status` auf `pruefung` gesetzt. Shop, Impressum,
  Registerspiegel und Stellenanzeige belegen den aktuellen Kfz-Teileverkauf und
  den Betrieb, aber keinen aktuellen Schrott- oder Altfahrzeugankauf.
  `website_status: aktiv` bleibt; keine Schließung behauptet.
- **ALBA Nord, Wismar:** `status` auf `pruefung` gesetzt. Betreiber- und
  Registerseiten belegen Abfallannahme/Containerdienst, nicht vergüteten
  Metallankauf. Website-Status und belegte Betriebsstättendaten bleiben.
- **Tamm, Wuppertal-Ronsdorf:** `status` auf `pruefung` gesetzt, weil der
  aktuelle Betreiberauftritt Entsorgungszahlung, aber keine Vergütung für
  angelieferten Schrott belegt. In `dropoff_json` und `pickup_json` wurden die
  aus einer historischen Passage von 1982 abgeleiteten Kundenarten entfernt;
  aktuell belegte Anliefer- und Containerleistungen bleiben erhalten.
- **Schrotthändler Plus, Bochum/Wattenscheid:** `status` auf `pruefung`, mobile
  Ankaufbeschreibung und `pickup_json` zurückgenommen. Die Website ist aktuell
  erreichbar und `website_status: aktiv` bleibt. Der Betreiber-/Rechtsträger-
  Bezug ist trotz undatiertem, selbstgeführtem XING-Profil nicht unabhängig
  genug bestätigt. `clear_fields: 'description,pickup_json'` stellt sicher,
  dass leere Seed-Felder gespeicherte Claims tatsächlich zurücknehmen. Keine
  Betriebsaufgabe behauptet.
- **DB Recycling, Rochlitz:** Straße wie die bereits geleerte PLZ aus dem
  strukturierten Datensatz entfernt, weil beide Angaben nur auf derselben
  unbestätigten Betreiberquelle beruhten. Die am 27.10. direkt gelesene
  Betreiberpreisliste (gültig ab 03.10.2026) nennt weiterhin aktuellen
  Metallankauf; `status: aktiv` und `website_status: aktiv` bleiben. Kein
  Ersatzstandort geraten.
- **Rheinische Scheidestätte Augsburg und Aachen:** Die Live-Seiten zeigten
  `geschlossen` beim Abruf außerhalb der Öffnungszeit; beide Seiten werben
  gleichzeitig mit Filialbesuchen und nennen normale Werktagszeiten. Das ist
  kein Schließungsbeleg, daher bleiben beide Statuswerte `aktiv`. Die alten
  Samstagszeiten waren jedoch falsch: beide Betreiberseiten nennen Samstag
  `geschlossen`, daher wurden die strukturierten Öffnungszeiten auf Werktage
  korrigiert. Quelle (Abruf ca. 02:16 UTC / 03:16 CET):
  https://rheinische-scheidestaette.de/unternehmen/filialen/augsburg/ und
  https://rheinische-scheidestaette.de/unternehmen/filialen/aachen/.

## Abgelehnte Findings und verbleibende Caveats

- Der Fund „geschlossen“ bei Augsburg/Aachen wurde als momentaner, dynamischer
  Öffnungsstatus falsch zu dauerhaftem Filialstatus ausgelegt. Die direkt
  gelesenen Betreiberseiten widerlegen eine Schließung; korrigiert wurden nur
  die unzutreffenden Samstagsöffnungszeiten.
- **AVB Autoverwertung Beck, Gerhardshofen:** Das Finding „keine unabhängige
  Zweitquelle“ übersah das aktuelle, extern ausgestellte Inspektionszertifikat.
  Das am 22.07.2026 nach § 5 Abs. 3 AltfahrzeugV ausgestellte Zertifikat nennt
  AVB/Uli Beck, Am Steinbruch 1, 91466 Gerhardshofen und den überwachten
  Demontagebetrieb; es gilt bis 22.08.2028, nächste Überwachung 22.08.2027.
  Unterzeichner ist laut Zertifikat ein öffentlich bestellter und vereidigter
  Sachverständiger der IHK Region Stuttgart. Damit sind Standort und aktuelle
  Fahrzeugverwertung unabhängig gestützt. Das Zertifikat belegt weder
  Metallankaufskurse noch allgemeine Schrottannahme. Der Nummernkonflikt
  09163 995557 / 09163 7272 bleibt offen; die im Betreiber-Impressum geführte
  Nummer wird nicht durch die abweichende Stellennummer ersetzt. Quelle:
  https://www.avb-beck.de/wp-content/uploads/2026/07/AVB_AF-Zertifikat_07-2026.png.
- Die offenen Identitäts-/Kontaktfragen bei Goldsofort Ahlen bleiben als solche
  dokumentiert; die unabhängige Registerkennung und der bevorzugte Kontakt
  gelten nicht als gelöst. Sie sind kein Beleg, dass die aktuelle
  Betreiber-Ankaufsseite geschlossen oder falsch ist.

## Validierung und Freigabegrenze

- Der vollständige Workspace-Test auf dem korrigierten Seed bestand: 778
  bestanden, 11 ignoriert, 0 fehlgeschlagen. Der fokussierte Seed-Test bestand
  mit 14/14; `cargo fmt --all -- --check` und `git diff --check` waren sauber.
- Release-Build aus dem korrigierten Kandidatenbaum war erfolgreich. Vorläufige
  SHA-256 Server `a47b0e461817cee86b01f9bebf9a87d7f52a86161e3306e0fd379722c4ac4eaf`,
  Query-Worker `8fc6429aeb55c4f83b2616d2fd3eb226118abcd8833d231658bd0289110ac67e`.
  Der Server wird nach dem Commit noch einmal gebaut, damit Probe und Binärdatei
  an genau den veröffentlichten Commit gebunden sind. Die private
  Upgrade-/Datenübergangsprobe mit diesem Seed steht noch aus; das vorherige
  Probe-Manifest gehört zum älteren Seedstand `7a431dcb` und ersetzt sie nicht.
- Bis dieser neue Probe-Lauf erfolgreich und an den veröffentlichten Commit
  gebunden ist, bleibt das Pre-Deploy-Gate offen. Kein Produktions-Write,
  Force-Ingest, Service-Neustart oder Deployment wurde ausgeführt.

## Frischer read-only Check-in, 27.10.2026 02:32 UTC

- Feedback-Tabelle endet weiterhin bei ID 7390 vom 13.10.2026; seit dem letzten
  Check-in ist kein neueres Feedback eingegangen. Die Tabelle enthält kein
  Triage-Statusfeld. Die Audit-7-Einträge werden nicht als neue ungeprüfte
  Meldungen ausgegeben.
- `schrott-mcp.service` ist aktiv. systemd meldet den Prozessstart am
  17.10.2026 16:49 UTC; bei dieser Prüfung wurde kein Dienstneustart ausgelöst.
  `journalctl -p warning` ist leer. Die sechs App-JSON-WARNs der letzten sechs
  Stunden betreffen einzelne Upstream-/Scrape-Probleme: Wirkaufendeingold ohne
  validierbare Tageskursreferenzen, Wahl & Co ohne Sortenpreisliste, A-Z mit
  verworfener verdächtiger Preisleiter, AMR mit HTTP 502, MKM mit ungültigem
  Source Contract und VHM mit Requestfehler. Die letzten Läufe #2868 und #2867
  waren `ok` (88 bzw. 116 Preise); #2866 war `partial` (61 Preise, ein
  Händlerfehler).
- Read-only Produktionsbestand: 3.893 Händler, 83 Materialien, 2.715 aktuelle
  Preiszeiger. Bestätigte Korrekturen sind dort noch nicht veröffentlicht:
  Fritsch hat weiter den falschen Punkt 51.90933/7.88825 bei leerer Adresse;
  Elma den alten Punkt 50.0167/10.4667 und die alte Telefonnummer; RoVa den
  alten Punkt 50.86395/12.08895 und die alte Telefonnummer; RVE den alten Punkt
  49.2516/6.8587 sowie `dropoff_json={"allowed":true}`. Wismar enthält weiter
  den falschen Punkt 53.88925/11.44455, und Tamm den alten, mehrere Kilometer
  entfernten Punkt 51.259513/7.164909. Die Seedkorrekturen bleiben bis zur
  abgeschlossenen Gate-Prüfung private Arbeitsdaten. Es gab keinen direkten
  Produktions-DB-Schreibzugriff.
- Freier Speicher: 46 GiB auf `/`, 1.0 GiB auf `/tmp`; ausreichend für die
  privaten Backups/Probe. Nach Commit werden exakte Release-Binaries gebaut und
  der Upgrade-Lauf ausschließlich auf WAL-sicheren privaten Kopien ausgeführt.
