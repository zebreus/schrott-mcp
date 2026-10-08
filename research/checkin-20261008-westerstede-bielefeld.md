# Feedback-Leads #5388 / #5390 — Recherche 08.10.2026

## Ergebnis und Datenentscheidungen

Zwei bislang fehlende, voneinander und von bestehenden Händlern getrennte Betreiber recherchiert:

| Feedback | Neues Dossier | Entscheidung |
|---|---|---|
| #5388 Müller Autoverwertung GbR | `dossiers/ni/ni-westerstede-muller-autoverwertung.md` | Website aktiv; `pruefung`, da unabhängiger Zweitbeleg fehlt. Betreiberangaben vollständig mit Quellen in Timeline, Adresse/Kontakte/Zeiten/Annahmebedingungen noch nicht strukturiert freigegeben. |
| #5390 M+M Recycling GmbH / Bielefelder Autoverwertung | `dossiers/nw/nw-bielefeld-mm-recycling.md` | Identität/Adresse durch Betreiber und Northdata gesichert; `aktiv`, Kontakte/Zeiten und ausdrücklich publizierte Bedingungen ergänzt. Keine aktuelle Zertifizierung behauptet. |

Keine bestehenden Händlerfelder überschrieben, keine Händler zusammengeführt, keine Koordinaten erfunden und keine Preisbeobachtung angelegt. Owner übernimmt gemeinsame Prüfung, Commit, Veröffentlichung und Produktionskontrolle.

## Aktuelle DB ausschließlich gelesen

SQLite `/var/lib/schrott-mcp/public.db` und `/var/lib/schrott-mcp/internal.db` mit URI `mode=ro`, nur SELECT. Feedback-Originale #5388/#5390 gelesen und eigenständig nachrecherchiert, insbesondere Register-/Zertifikatsangaben nicht unbesehen übernommen.

Abgleich: Name, Straße, Ort, Postleitzahl und Websites; insbesondere `city LIKE '%Westerstede%'`, `city LIKE '%Bielefeld%'`, `street LIKE '%Grafenheider%' OR street LIKE '%Leerer%'`, Domains und Namen Müller/Autoverwertung, M+M, Bielefelder Autoverwertung, Wiegel. Dazu Dossier-Volltextsuche nach diesen Domains/Straßen/Namen. Keine passenden vorhandenen Zeilen/Dossiers.

- Westerstede: nur #1241 `ni-westerstede-schultze-k` (Name Schultze, K., ohne Adresse/Website), kein Beleg einer Verbindung zu Müller.
- Bielefeld: zehn vorhandene Einträge. #1459 `nw-bielefeld-mm-schrotthandel` sitzt laut DB an Adolf-Reichwein-Str.22b, 33615, Domain mmschrott.de. [Northdata HRA17585](https://www.northdata.de/MM%20Schrott%20%26%20Metalle%20e.K.,%20Bielefeld/HRA%2017585) nennt MM Schrott & Metalle e.K. und Martin Mechtold; neue Firma ist dagegen M+M Recycling GmbH, HRB43527 und Denis Matuschok. Ähnlicher Kurzname ist kein Merge-Grund.
- Wolfgang Dehne (#1461, Uferstraße12) teilt 33729, nicht die Anschrift oder Identität.

## Müller Autoverwertung: belegte Betreiberangaben, Grenzen

[Impressum](https://www.mueller-autoverwertung.de/impressum.html) nennt Müller Autoverwertung GbR, Leerer Str.41, 26655 Westerstede, Bernd/Bianca Müller, USt-ID DE184292231, 044882452, Fax04488861091. [Anfahrt](https://www.mueller-autoverwertung.de/anfahrt.html) ordnet den Standort Hollriede zu. Zweittelefon04488861090 im Seitenkontakt. Die verschleierte E-Mail ließ sich aus dem tatsächlichen HTML-Entity-/JavaScript-Code zu `info@mueller-autoverwertung.de` dekodieren; die Feedback-Aussage „nicht lesbar“ ist insofern verbessert.

[Auto-/Metallrecycling](https://www.mueller-autoverwertung.de/leistungen/auto-metallrecycling.html) belegt expliziten Preisvorschlag/Tagespreise, Kleinmengenanlieferung und Abholung für Alt-/Schrott-/Unfallfahrzeuge sowie Misch-/Stahl-/Scherenschrott, Kupfer, Guss, Alu, Blei, Zink, V2A, Messing und Kabel; Wägung auf geeichter50t-Waage oder gewünschter anderer Waage. [Containerdienst](https://www.mueller-autoverwertung.de/leistungen/containerdienst.html): 7–40m³, einmalige/befristete/Dauergestellung. Keine Zahlenpreise, Mindestmengen, Kundengruppen, Abholradien oder Gebühren verifiziert; insbesondere nicht kostenlos behaupten.

Widerspruch: Seitenkontakt Mo–Fr08:00–12:30/14:00–17:00, Sa geschlossen, [AGB-Text](https://www.mueller-autoverwertung.de/agb.html) dagegen Mo–Fr08:00–12:30/13:30–18:00 und Sa08:00–13:00. Zeiten bleiben strukturiert leer, Anruf vor Anfahrt erforderlich. AGB sind für Teileverkauf/Lieferung, nicht automatisch Ankaufbedingungen. [Über uns](https://www.mueller-autoverwertung.de/ueber-uns.html) nennt weitere Familienmitglieder; keine aktuelle Vertretung daraus ableiten.

[Online-Katalog](https://www.mueller-autoverwertung.de/online-katalog.html) laut Betreiber nicht verfügbar. Verlinktes [eBay](https://www.ebay.de/str/autoverwertungmueller) HTTP410, [mobile.de](https://home.mobile.de/MUELLER-AUTOVERWERTUNG) HTTP403. Keine Verkäuferidentität oder Verkaufspreise daraus gelesen.

Unabhängiger Beleg fehlt: [Northdata-Suggest](https://www.northdata.de/suggest.json?query=M%C3%BCller%20Autoverwertung&countries=DE) liefert keine Ergebnisse (kein amtlicher Negativbeweis); GbR-Impressum ohne Registerkennung, also keine README-Ausnahme. Aggregator-Spur Friedrich/Michael Müller, Halsbeker Str.8, bewusst nicht vermischt. Feedback nennt GESA ZAHS01800034/1, aber [GADSYS](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung) liefert nur Vaadin-JS-Hülle ohne Betriebsdatensatz. Anerkennung nach §5Abs.3 bleibt Betreiberangabe, kein aktueller Zertifikatsbeleg.

## M+M Recycling: belastbare Identität, wirklicher Ankauf

[Impressum](https://www.bielefelder-autoverwertung.de/impressum/) nennt GmbH als Betreiberin, Grafenheider Str.105, 33729, AG Bielefeld HRB43527, Denis Matuschok. Gelesenes [Northdata-Profil](https://www.northdata.de/M%2BM+Recycling+GmbH,+Bielefeld/HRB+43527) bestätigt Namen/Adresse/Register/Gegenstand, Eintragung11.09.2019, Publikation Jahresabschluss2024 am09.09.2025; keine Erlöschenskennzeichnung. Kein amtlicher Tagesauszug; Website wegen alter Footer-/Impressumsteile nicht pauschal als aktuell-zertifizierte Ausnahmequelle behandelt. Doppelbeleg für Identität/Adresse, übrige Fakten explizit als Betreiberangaben.

[Startseite](https://www.bielefelder-autoverwertung.de/) und [Schrott & Metalle](https://www.bielefelder-autoverwertung.de/schrott-metalle/) nennen Telefon052132922015, info@bielefelder-autoverwertung.de, Mo–Fr09–16, Sa geschlossen; Impressum zusätzlich info@mm-recycling.de/Fax052132922017. WhatsApp052132922016 ist nicht das Haupttelefon.

Ausdrücklich vergüteter Ankauf von Privat/Gewerbe: Guss/Misch-/Neuschrott/Schredder-Vormaterial/Träger, Alu/Blei/V2A/Kupfer/Messing/Zink, Bleibatterien, E-Motoren und Kabel. Fahrzeugwaage60t mit Außendisplay, Container bis40m³. Überschrift Elektro-Großgeräte genügt nicht für pauschale Vergütungs-/Annahmekonditionen.

**Preis-/Gebührenlage:** Preise orientieren sich an Großhandel/Börse, keine numerische Liste. Bargeld sofort bei Anlieferung möglich; sonst Wochenendabrechnung und Überweisung am ersten Werktag der Folgewoche. Anhaftungen/Müll führen zu Gewichtsabzügen, zusätzliche Entsorgungskosten vorbehalten. [Autoverwertung](https://www.bielefelder-autoverwertung.de/autoverwertung/): Vergütung nach Gewicht, Abholung zum vereinbarten Termin innerhalb30km zum nicht bezifferten Pauschalpreis, separate Reifen gebührenpflichtig, Abmeldung durch Servicepartner auf Wunsch, Verwertungsnachweis. Keine kostenlose Entsorgung/Abholung behauptet. Kein aktueller Ersatzteilverkaufs-Preisstand gelesen.

**Zertifikatsfund wichtig:** [Tatsächlich gelesenes Zertifikatsbild](https://www.bielefelder-autoverwertung.de/wp-content/uploads/2022/05/mm-recycling-bescheinigung-2022.jpg) von ENVIZERT, Nr.2204007, Standort Grafenheider Str.105, Annahme/Rücknahme/Demontage, BetriebsnummerE711E0011, Audit14.03.2022, Ausstellung01.04.2022, **gültig nur bis13.09.2023**. Kein `certifications`-Fill. Feedback-GESA ZAES01100024/1 bleibt unbestätigter Suchschlüssel. [Creditreform](https://firmeneintrag.creditreform.de/33729/4010292065/M_M_RECYCLING_GMBH) Direktabruf403, nicht als gelesener Drittbeleg gezählt.

Offen: aktuelles Zertifikat/GESA-Datensatz, mögliche Vorgängerkette Roger Wiegel (Footer2019 ist kein Übernahmebeweis; since1920/2002 sind nicht GmbH-Gründungsdaten), konkrete Tagespreise und Gebührenbeträge. Gültigkeit der Zeiten vor Anfahrt bestätigen.

## Owner-Gate

- Müller: unabhängigen Register-/Kommunal-/Betreiberprofilbeleg nachholen, GESA-Schlüssel tatsächlich auflösen, Zeitenkonflikt telefonisch/mit datierter Mitteilung klären; erst dann strukturierte Kontakt-/Annahmefelder freigeben.
- Bielefeld: aktuelle Demontagebescheinigung erfragen; älteres2022-Bild keinesfalls als aktuelle Zertifizierung publizieren. Identität und vergüteter Ankauf sind nicht mit dem gleichnamensähnlichen bestehenden MM-Händler zu vermischen.
- Keine numerischen Preise in beiden Quellen; keine erfundenen oder aus Verkauf/Portalen umgedeuteten Ankaufkurse.
- Validierungsergebnis wird unten nach dem Seed-Test ergänzt.

## Owner-Nachtrag nach Nutzungslimit

Agent endete am Nutzungslimit; Bericht und Dossiers sind vorhanden, aber
sein abschließendes Validierungsergebnis fehlt. Owner hat Müller-Ankaufseite
und M+M-Impressum/Registerprofil erneut gelesen. M+M-Adresse/Identität
kongruent; Aktuell-Ausnahme ausdrücklich nicht vollständig erfüllt.
Daher M+M auf pruefung, Kontakt-/Zeit-/Servicefelder vorerst leer,
vollständige gelesene Betreiberbedingungen in Timeline erhalten.
Müller-Website/description ebenfalls nur Timeline bis Zweitbeleg.
Historischer YAML-Fehler im Paralleltest ist inzwischen behoben:
provenance_section steht bereits in Quotes, kein fremdes Zurücksetzen.
Gemeinsame Tests und Deployment übernimmt Owner nach Format-Gate.
