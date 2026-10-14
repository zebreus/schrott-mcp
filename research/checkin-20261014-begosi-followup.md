# Feedback 7387: BeGoSi — gezielter Follow-up

Recherche **14.10.2026**, ausschließlich Research, keine Integration.

## Ergebnis / Owner-Grenze

**Neuer, tatsächlich abgefragter unabhängiger amtlicher Standortbeleg gefunden:** Das öffentliche ZSVR-Herstellerregister LUCID führt **Begosi, Ziegeleiweg 5, 04552 Borna, OT Neukirchen**, mit exakt der selbst angegebenen Verpackungsregisternummer **DE4361709152940** und USt-ID **DE453526376**. Nummern-, Namens-/PLZ- und USt-ID-Abfragen liefern denselben einzelnen Datensatz. Das ist mehr als die bisher unbestätigte Nummer im Eigenimpressum.

**Das README-Gate ist damit dennoch nicht als bestanden erklärt.** LUCID ist ein Verpackungsherstellerregister, weder Handelsregister noch kommunales Gewerberegister; es gehört nicht ausdrücklich zur abschließenden Quellenliste in README Zeilen 180–199. Der gelesene Datensatz nennt zudem **Begosi, nicht Philipp Eiker**, und keine angezeigte Gewerbetätigkeit. Er stützt die Unternehmensbezeichnung/Anschrift auf einem unabhängigen amtlichen Veröffentlichungskanal, beweist aber weder die persönliche Inhaberschaft noch tatsächlichen heutigen Kundenankauf. Registrierung beruht auf Herstellerangaben, nicht auf einer hier verifizierten Vor-Ort-Prüfung. **Owner müsste eine Anerkennung außerhalb der dokumentierten Hierarchie ausdrücklich entscheiden; diese Recherche nimmt sie nicht vor.** Nach der unveränderten Hierarchie bleibt nur ein passender qualifizierter Quellenkomplex, nämlich begosi.de.

Der vergütete Gold-/NE-Metallankauf vor Ort und per Post ist **aktuell online als Betreiberangebot ausdrücklich lesbar**. Nicht nur kostenlose Entsorgung/Entrümpelung. Neue wichtige Abgrenzung: Versandkostenausgleich ist laut Ablauf-FAQ an einen erfolgreichen Verkauf gebunden; Beräumung ist eine kostenpflichtige Dienstleistung mit Wertanrechnung, nicht pauschal kostenlose Abholung.

**Konkreter Namensvetter unabhängig aufgeklärt:** Das frühere **BEGOSI Dietmar Beermarm** in Idar-Oberstein gehört laut Northdata zu **Infain OHG, HRA 11502 Bad Kreuznach**. Keine Zuordnung dieser Registeridentität zu Eiker/Borna, keine Übernahme ihres Liquidationsbefunds auf BeGoSi Borna.

## Auftrag und Schutzgrenzen

- Gelesen: `research/feedback-20261014-7387-7389.md`, `research/checkin-20261014-feedback-7385-7389-owner.md`, README „Feedback triage“ und „Operator rename/merge chains“.
- Keine Produktionsdatenbank geöffnet oder geändert; keine Dossiers, Seeds, Preise, GEO oder Feedbackstatus geändert. Den Dublettenabgleich des Vorberichts **nicht neu durchgeführt** und deshalb nicht als eigene aktuelle DB-Prüfung ausgegeben.
- Nur diese Datei erstellt. Keine Kontaktaufnahme, Anmeldung, Anfrage an die Stadt, Bestellung, Händlernachricht oder Umgehung von Login/CAPTCHA. LUCID-POSTs waren öffentliche **lesende Registerabfragen**, keine Registrierungs-/Änderungsanträge.

## 1. Amtlicher ZSVR-Beleg: tatsächlich gelesen und reproduzierbar

### Öffentliche Einstiegsseiten

1. <https://lucid.verpackungsregister.org/> — offizielle Startseite verlinkt „Öffentliche Register“.
2. <https://oeffentliche-register.verpackungsregister.org/> — beschreibt Veröffentlichung registrierter Hersteller und nennt Aktualisierung **14.10.2026 01:10 Uhr**.
3. <https://oeffentliche-register.verpackungsregister.org/Producer> — Herstellerregister mit Feldern Unternehmensname, Registrierungsnummer, PLZ, Ort, Marke und USt-ID/Steuernummer. Erläutert: Erstregistrierungsdatum ist erste Registrierung; Beendigungsdatum ist Marktaustritt. Der reine Textabruf zeigte zunächst „Ergebnisse konnten nicht abgefragt werden“; **das war kein Negativbefund**.

Der HTML-Quelltext wurde danach mit Python 3/urllib in-memory gelesen. Er enthält das Suchformular `publicRegisterManufacturerSearch`, den POST-Endpunkt `/Producer/ManufacturerRead`, den Antiforgery-Token und die Kendo-Datenquelle. Das tatsächlich gelesene Seitenskript
<https://assets.verpackungsregister.org/or/js/manufacturer.min.js?v=2.26.0917.1>
zeigt die Umwandlung der Formularfelder in `contains`-Filter und das Mitsenden des Tokens. Keine geratenen privaten API-Zugänge.

### Erfolgreiche Abfragen

Cookie-Sitzung durch GET `/Producer`, Antiforgery-Token aus dieser Seite; anschließend URL-encoded POST an
<https://oeffentliche-register.verpackungsregister.org/Producer/ManufacturerRead>
mit `__RequestVerificationToken`, `page=1`, `pageSize=10`, `skip=0`, `take=10`, leeren `sort`/`group` und jeweils folgendem Kendo-Filter:

```text
RegisterNumber~contains~'DE4361709152940'
CompanyName~contains~'BeGoSi'
CompanyName~contains~'Begosi'~and~ZipCode~contains~'04552'
TaxNumber~contains~'DE453526376'
```

**Alle vier Abfragen: `Total: 1`, `Errors: null`, derselbe Datensatz.** Tatsächlich gelesene relevante Antwortfelder:

```json
{
  "ManufacturerId": "a0b046b1-c737-f011-8315-005056b8dfd2",
  "CompanyName": "Begosi",
  "RegisterNumber": "DE4361709152940",
  "ZipCode": "04552",
  "Location": "Borna",
  "Country": "Deutschland",
  "RegisterDate": "23.05.2025",
  "RegistrationEndDate": "",
  "Street": "Ziegeleiweg",
  "StreetNumber": "5",
  "TelephoneNumber": "+49017641060712",
  "TaxNumber": "",
  "EuropeanIdentificationNumber": "DE453526376",
  "AddressAddition": "OT Neukirchen",
  "IsForeignProducer": false,
  "AuthorizedRepresentative": null
}
```

Die ungewöhnliche Telefonformatierung ist **Rohwert** des Registers, kein zusätzlicher Kontakt. Sie entspricht nach Entfernung der inländischen Null hinter +49 der Eigenmobilnummer. Das leere Beendigungsfeld heißt nur: im abgefragten Datensatz kein Beendigungsdatum veröffentlicht; **kein Beweis von Öffnung, Ankaufsaktivität oder Gewerberegister-Live-Status**.

Zusätzlich direkt gelesen:
<https://oeffentliche-register.verpackungsregister.org/PackagingInfo?producerId=a0b046b1-c737-f011-8315-005056b8dfd2>.
Text: „Der Hersteller Begosi gibt an“, Verpackungen mit und ohne Systembeteiligungspflicht bereitzustellen. Das betrifft Verpackungen, **keine Schrott-/E-Schrottannahmezulassung** und keine Zahlung an Materiallieferanten.

**Historiengrenze:** Erstregistrierung 23.05.2025 zusammen mit einer heute publizierten Bornaer Anschrift beweist nicht, dass diese Anschrift schon am Erstregistrierungstag galt. Daher kein Widerspruch oder vorgezogener Betriebsbeginn gegenüber der Betreiber-Eröffnungsankündigung 13.12.2025 konstruiert.

### Technische Fehlversuche, nicht als Evidenz gewertet

- Zunächst POST mit verschachtelten `filter[filters][0]`-Parametern: `Data: null`, `Errors: "Error while getting all manufacturer data."`; kein Nulltreffer.
- GET auf den Read-Endpunkt: HTTP 405. Danach gelang POST mit dem ASP.NET/Kendo-Filterstring oben.
- Ergänzende erfolgreiche `CompanyName~contains~'Eiker'`-Suche: 42 unscharfe Treffer, nur erste 10 gelesen (u.a. Peiker/Schweiker/Leiker). **Keine vollständige Eiker-Namensprüfung**, kein behaupteter Ausschluss aller Personen aus LUCID. Die eindeutigen BeGoSi-Nummernabfragen sind maßgeblich.

## 2. Betreiberankauf und Kostenabgrenzung

Alle folgenden Seiten **selbst direkt gelesen am 14.10.2026**, zusammen nur **ein Eigenquellenkomplex**:

| Quelle | Tatsächlich gelesener Befund / Grenze |
| --- | --- |
| <https://begosi.de/impressum> | Philipp Eiker / BeGoSi, Ziegeleiweg 5 Borna OT Neukirchen, 0176 41060712, info@begosi.de; USt-ID und LUCID wie oben. Keine Handelsregisternummer. Standortbezogene Zufahrtsbeschreibung/Parkplätze, nicht nur abstrakte Briefanschrift. |
| <https://begosi.de/gold/> | Altgold, Schmuck, Zahngold, Münzen, Barren. Termin vor Ort oder Post, Bewertung, unverbindliches Angebot, Verkäuferzustimmung, Auszahlung via Konto/PayPal; vor Ort auch sofort bar. Keine Mindestmenge. Hausbesuch nur bei größeren Mengen nach Absprache. Keine feste numerische Händler-Ankaufsliste. |
| <https://begosi.de/andere-metalle/> | Zinn/Kupfer/Messing/Doublé, edelmetallbeschichtete Industrieabfälle, Prozessoren/Leiterplatten; Privat und Gewerbe; konkrete Bewertung und Auszahlung. FAQ nennt Bunt-/unedle Metalle ausdrücklich tagesaktuell und bar bei Anlieferung. Kupferqualitäten/Anhaftungen beeinflussen Bewertung. Keine umfassende E-Geräteannahme oder abfallrechtliche Erlaubnis aus diesen Aussagen abgeleitet. |
| <https://begosi.de/und-so-gehts/> | Zwei konkrete Wege „Postankauf“ und „Barankauf“; Geld nach Angebotsannahme, vor Ort bar oder Konto. DHL max. 31,5 kg ist Paketgrenze, **keine Mindest-/Höchstankaufsmenge**. Versandabschnitte: „Die Versandkosten werden mit dem Ankaufswert verrechnet.“ FAQ präzisiert: entstandene Versandkosten werden auf der Gutschrift erfasst, sodass Kunden sie **im Falle eines erfolgreichen Verkaufs nicht zahlen müssen**. Keine Zusage kostenloser Hin-/Rücksendung bei Ablehnung; keine genaue Erstattungsobergrenze verifiziert. Text verwendet sowohl 48 Stunden als auch 2 Werktage, keine eigene harte Laufzeitgarantie. |
| <https://begosi.de/beraeumungsservice/> | Kostenfreie Vorabberatung, dann Kostenangebot anhand Inhalt/Aufwand. Wertgegenstände werden gutgeschrieben bzw. nach Abstimmung angekauft; Differenz wird in Rechnung gestellt. FAQ: **Haushaltsgegenstände und Möbel werden nicht angekauft**. Nachberechnung erheblicher unvorhergesehener Mehrarbeit nach Absprache möglich. Bundesweiter Beräumungsservice nicht in pauschal kostenlose Schrottabholung umdeuten. |
| <https://begosi.de/downloads/> | Ankaufsformular, Beräumungs-/Umzugsformulare, Marktordnung; trotz Einleitung „Preislisten“ kein sichtbarer numerischer Ankaufspreislistenlink. |
| <https://begosi.de/ueber-uns/> | Philipp als heutiger Geschäftsführer; Übergabe durch Thomas „im Laufe des vergangenen Jahres“ und Name als Besteck/Gold/Silber erklärt. Undatierte relative Angabe bleibt ohne verifiziertes Nachfolgejahr; kein Rechtsform-/HR-Nachweis. |
| <https://begosi.de/firmen-news/feierliche-eroeffnung/> | Ankündigung Eröffnung neuer Standort Borna am 13.12.2025, verlinkt Facebook diversitas7 und Instagram be.go.si. Ankündigung und heutiger Webabruf nicht mit unabhängig beobachteter Öffnung gleichsetzen. |

**Neu vertieftes Dokument:** Die Ablaufseite verlinkt anders als die Downloads-Seite die neuere Datei
<https://begosi.de/wp-content/uploads/2025/12/BeGoSi-Ankaufsformular-1-2.pdf>.
Direkter HTTPS-Abruf plus `pdftotext -layout - -` in-memory ergab auf beiden Seiten **Philipp Eiker, Ziegeleiweg 5, 04552 Borna OT Neukirchen**, außerdem Warenfelder Art/Legierung/Menge in Gramm/sonstige Informationen. Nur der extrahierbare Text ist hier verifiziert; keine vollständige Sicht-/OCR-Prüfung der übrigen Formularfelder. Der ältere Download
<https://begosi.de/wp-content/uploads/2024/07/BeGoSi-Ankaufsformular.pdf>
lieferte bei derselben Textprüfung keinen brauchbaren Text. **Keine alte Anschrift daraus behauptet**, kein unabhängiger Beleg aus beiden PDFs.

Die Gold-FAQ „keine versteckten Gebühren“ ist nicht mit einer allgemeinen Kostenfreiheit von Versand, Rückversand, Umzug oder Beräumung identisch. Börsenlinks auf den Materialseiten sind keine BeGoSi-Auszahlungspreise. Das Beispiel 500 g vergoldetes Konvolut „ab 1000 €“ bleibt Einzelbewertungswerbung, kein €/g-Tarif.

## 3. Namensvetter: BEGOSI Idar-Oberstein ist nicht BeGoSi Borna

**Tatsächlich direkt gelesen:**

- <https://www.northdata.de/?query=BeGoSi> — unscharfe Ergebnisse verschiedener Länder; darunter **Infain OHG, Oberstein (i. L.), Amtsgericht Bad Kreuznach HRA 11502**, auch/vormals „BEGOSI Dietmar Beermarm“. Trefferliste nicht als Borna-Identitätsbeleg verwendet.
- <https://www.northdata.de/Infain+OHG,+Oberstein/Amtsgericht+Bad+Kreuznach+HRA+11502> — Detailseite nennt Namensreihe Infain OHG / Infain e. K. / BEGOSI Dietmar Beermarm, HRA 11502, EUID DET2101V.HRA11502. Publikationsübersicht: 26.01.2006 Eintragung als BEGOSI, 15.03.2006 Name Infain e. K., 04.03.2010 Geschäftsanschrift **Nohweg 31, 55743 Idar-Oberstein**, 10.08.2023 Name Infain OHG, **08.05.2025 Liquidation; Liquidatoren Denise P. Beermann, Patrick Beermann**. Schreibweise „Beermarm“ ist Northdata-Rohtext, keine eigene Namenskorrektur. Die einzelnen Originalpublikationen nicht separat geöffnet; Befund stammt aus gelesener Northdata-Detailübersicht.
- <https://www.fasseratelier-goerg.de/40376/home.html> — Alexander Görg beschreibt eigene Lehre bei „Fa. BEGOSI in Idar-Oberstein“ vor Gründung seines Ateliers im Oktober 1996. Historische Primäraussage eines anderen Betriebs, keine heutige Borna-Bestätigung.

**Abgrenzung:** Historischer BEGOSI-Schmuckbetrieb mit Beermann-Namens-/Registerkette, anderer Stadt/Adresse versus Philipp Eiker/Borna. Keine gelesene gemeinsame Registernummer, Kontaktdaten oder Übertragungskette. **HRA 11502 und Liquidation nicht in einen Bornaer Datensatz übertragen**, keine Fusion/Nachfolge unterstellen. Schreibungsähnlichkeit allein genügt nicht.

<https://www.northdata.de/?query=Philipp%20Eiker> ebenfalls direkt gelesen: erste 15 unscharfe Ergebnisse (u.a. Eike Philipps Frankfurt, Philipp Tom Heikes Hamburg), kein passender Philipp Eiker/Borna auf dieser Seite. **Nicht alle 1.000+ angebotenen unscharfen Treffer gelesen; kein amtlicher Negativauszug.**

## 4. Weitere tatsächlich versuchte Kanäle und ihre Grenzen

- Betreiber-verlinktes <https://www.facebook.com/diversitas7> und `/about`: keine verwertbare Textausgabe. <https://www.instagram.com/be.go.si/>: Login-Seite, kein lesbarer Profil-/Adress-/Inhaberbeleg. Profilführung und Tätigkeit daher nicht unabhängig verifiziert.
- Beräumungsseite verlinkt die eigenen Verkaufsshops <https://www.ebay.de/str/diversitas7> und <https://www.ebay.de/str/artspirit>. Beide Shop-Startseiten direkt gelesen: Antiquitäten-/Sammler-/Schmuckartikel mit Verkaufspreisen; kein gelesener Verkäufername/Ankaufsangebot in dieser Ausgabe. `/str/diversitas7?_tab=about` und `/str/artspirit?_tab=about` lieferten Identitätsbestätigungs-/CAPTCHA-Seite. Kein Umgehen. Eigen-Shop-Verlinkung und Verkäuferbewertungen sind weder unabhängiges Gewerberegister noch Nachweis des heutigen Materialankaufs; **kein zweiter zulässiger Quellenkomplex gezählt**.
- Kommunale Primärseite direkt gelesen: <https://www.borna.de/Stadtverwaltung-und-Buergerservice/Was-erledige-ich-wo.htm/Unterlagen/Gewerberegisterauskunft.html>. Sie stellt den **Antrag auf Gewerberegisterauskunft** bereit, keinen öffentlich gelesenen BeGoSi-Eintrag. Kein Antrag gestellt (Kontaktverbot). Damit ist ein sinnvoller möglicher späterer Beschaffungsweg identifiziert, aber kein Identitätsbeleg beschafft.
- Websuche u.a. `"BeGoSi" "Eiker"`, `"Philipp Eiker" Borna Register`, `"BeGoSi" site:borna.de`, `"BeGoSi" site:creditreform.de`, `"Philipp Eiker" "Borna" -site:begosi.de`, `"BeGoSi" "Handelsregister"`, `"Thomas Eiker" Edelmetall Leipzig`, `"Eiker" "Ziegeleiweg"`: keine passende zusätzlich direkt lesbare HR-/Creditreform-/kommunale Identitätsdetailseite. Suchleads 11880/GoYellow/Exa sind **nicht als Belege gezählt**. Exa „Open“ und Portalstunden keine Aktivitätsbestätigung.
- Thomas-Eiker-Suchlead: <https://leipzig.stadtbranchenbuch.com/4817041.html>, Herrmann-Meyer-Str. 46, 04207 Leipzig, 0341 4929460. **Nur Suchausgabe gelesen, nicht Detailseite; Aggregatorlead.** Nicht als verifizierte frühere BeGoSi-Anschrift oder Thomas→Philipp-Kette übernehmen.

## Nächste beleggebundene Owner-Entscheidung

1. LUCID-Befund als **neu verifizierte amtliche Zusatzinformation** aufnehmen, ohne Handelsregisterqualität, Inhaberbeweis oder heutige Betriebskontrolle zuzuschreiben. Ob er den dokumentierten Quellenstandard ergänzen darf, ist offen; **hier kein erfolgreiches Gate**.
2. Für ein Gate nach unverändertem README fehlt ein tatsächlich gelesener qualifizierter unabhängiger Beleg, der **Philipp Eiker / BeGoSi / heutigen Bornaer Standort** verbindet: kommunale Gewerberegisterauskunft, passender HR-/Northdata-/Creditreform-Beleg oder lesbares verifiziert betreibergeführtes Socialprofil. Eine amtliche Auskunft darf erst nach gesonderter Freigabe zur Kontaktaufnahme beschafft werden.
3. Thomas→Philipp sowie frühere Leipzig-Adresse bleiben unbewiesen; keine Rückdatierung aus LUCID-Erstregistrierung und keine Übernahme der Idar-Obersteiner Registerkette. Aktueller vergüteter Ankauf ist Eigenangebot, unabhängig tatsächliche Auszahlungen nicht beobachtet.
4. Vor späterer Neuanlage erneuter Bestands-/Dublettenabgleich durch Owner. Kein Dossier, kein Seed, keine Preis-/GEO-Integration und keine Feedbackquittierung aus diesem Follow-up.
