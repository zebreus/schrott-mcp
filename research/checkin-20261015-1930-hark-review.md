# Unabhängige Hark-Hamm-Prüfung – 15.10.2026

## Integrationsentscheidung

**Keine Umadressierung der bestehenden Hamm-Schrottplatz-Zeile zum Hafen und keine Übernahme der Hafenkoordinaten.** Der ursprüngliche Standort **Am Ortsgüterbahnhof 9, 59063 Hamm** ist durch einen tatsächlich abgerufenen behördlichen Vor-Ort-Bericht und ein separates Betreiber-PDF konkret belegt. Hafenstraße 110 ist ein **anderer Standort**, kein Ersatzname derselben Adresse. Die rechtliche Sitzverlegung nach Dortmund 2026 ist ebenfalls belegt, beweist aber weder Schließung noch Verlegung des Hammer Betriebsplatzes.

**Empfehlung für den Owner: Adresse Am Ortsgüterbahnhof 9 / 59063 nachtragen, `status: pruefung` statt unkommentiert `aktiv`; bestehende Website und zentrale Kontakte behalten.** Die Adresse ist belastbar, der **heutige Weiterbetrieb dieses konkreten Platzes** dagegen nicht ausreichend bestätigt: Er fehlt auf aktueller Betreiberkarte und Anfahrtseite, und die gefundenen standortbezogenen Zertifikate sind seit 31.08.2026 abgelaufen. `geschlossen` wäre ebenso unbelegt. README-Ziffer „delisted branches … otherwise pruefung + Klärfall“ ist hier unmittelbar einschlägig.

Das ist keine Phantom-Zeile: Der Hammer Schrottplatz war real und 2025 behördlich vor Ort überwacht. Ein Unternehmens-Impressum mit Dortmunder Anschrift widerlegt diesen Standort nicht. Umgekehrt macht eine aktive Website den historischen Hammer Platz nicht automatisch heute aktiv.

## Maßstab und Umfang

- README vollständig gelesen, besonders Quellenhierarchie, Betreiber-Primärquellen-Ausnahme, Betreiberketten und Umgang mit ausgelisteten Standorten.
- Ausgangsdossier und vorhandenen Bericht `research/wave-20261013-c.md` gelesen; dortige Behauptungen nicht als neue Webbelege gezählt.
- Ausschließlich diese Review-Datei geschrieben; keine Dossier-, Git-, Code- oder Datenbankänderung. Keine Produktionsdatenbank abgefragt. Die ältere DB-Koordinate wurde daher nicht erneut aus Produktion verifiziert.
- Alle nachfolgend als abgerufen bezeichneten Quellen wurden in dieser Sitzung am **15.10.2026** tatsächlich gelesen. PDF-Downloads und Textauswertung liefen im Speicher (`urllib.request` → `pdftotext -layout - -`), ohne separate Dateien zu schreiben.

## Neue entscheidende Belege

### A. Behördlicher Nachweis des eigentlichen Hammer Schrottplatzes

Quelle: https://www.bra.nrw.de/system/files/media/document/file/heinrich_hark_gmbh_co_kg_hamm_27.05.2025.pdf

Tatsächlicher Download: **HTTP 200**, `application/pdf`, 85.715 Bytes; SHA-256 `0b541a73be1f7e54d29748ce33d90a21e0c87a9964431c64a2b9b443978d6785`.

Seite 1:

> Umweltinspektionsbericht der Bezirksregierung Arnsberg … vom 24.07.2025
>
> Betreiber: Heinrich Hark GmbH & Co. KG / am Standort: Am Ortsgüterbahnhof 9 in 59063 Hamm
>
> Die Firma … betreibt am o. g. Standort eine Anlage zur zeitweiligen Lagerung und Behandlung von Eisen- und Nichteisenschrotten sowie zur zeitweiligen Lagerung und sonstigen Behandlung von Abfällen …
>
> Datum der Überwachung: 27.05.2025

Es handelt sich um eine angemeldete Vor-Ort-Revision, nicht um ein Branchenverzeichnis. Seiten 2–3 nennen geringfügige Mängel und Nachträge zu deren Behebung, **keine angeordnete Stilllegung**. Inspektionsdatum 27.05.2025 und Berichtsdatum 24.07.2025 sind auseinanderzuhalten. Der Bericht belegt Betrieb zu diesem Zeitpunkt, nicht eine aktuelle private Schrottannahme am 15.10.2026.

### B. Separates Hamm-Zertifikat auf dem Betreiberhost

Quelle: https://hark-hamm.de/media/pages/unternehmen/zertifikate/2588856863-1746798539/esn-zertifikat_hamm_2025.pdf

Tatsächlicher Download: **HTTP 200**, `application/pdf`, 363.261 Bytes; SHA-256 `697b4416b1546c08c67755fb0f9afdea44ba4d43016405ab1fa9de7c75f6683b`.

- Zertifikat ESN 98-030085(25), Vorgang ZZEE003000828009, ausgestellt 26.03.2025 nach Prüfung 20.03.2025, **gültig bis 31.08.2026** (Seite 1).
- Name Heinrich Hark GmbH & Co. KG, Am Ortsgüterbahnhof 9, 59063 Hamm; seinerzeit HRA 108, Registergericht Hamm.
- Anlagen auf Seiten 2, 5 und 8 führen ausdrücklich **denselben Betriebsstandort Am Ortsgüterbahnhof 9**, nicht nur eine Hauptsitzadresse auf dem Deckblatt.
- Seite 2: Containerdienst im Werkverkehr. Seite 5: Lageranlagen unter anderem für Eisen-/Nichteisenschrotte, Kennnummer E915971847. Seite 8: Behandlungsanlagen, ebenfalls E915971847. Die AVV-Listen umfassen zahlreiche Metallfraktionen.
- Textauswertung bildet grafische Ankreuzfelder nicht zuverlässig ab; keine zusätzlichen Tätigkeitsfreigaben allein aus der gedruckten Standardformularliste abgeleitet.

Das PDF ist ein unabhängiger ESN-Zertifizierungsinhalt auf dem Betreiberhost und stützt zusammen mit A die Standortadresse. **Kein gegenwärtig gültiger EFB-Nachweis**; weder „Zertifizierung aktuell gültig“ noch pauschale Annahme aller gelisteten Abfälle für Privatkunden übernehmen.

### C. Hafen ist getrennt, nicht umbenannter Ortsgüterbahnhof

Quelle: https://hark-hamm.de/media/pages/unternehmen/zertifikate/3343463353-1746798532/esn-zertifikat_hafen_2025.pdf

Tatsächlicher Download: **HTTP 200**, `application/pdf`, 332.908 Bytes; SHA-256 `3e1762824fba46d7a316a1422398787c4774fc4185256670b2693fbc2da76f45`.

- Eigenes Zertifikat ESN 15-030532(25), Vorgang ZZEE003000866008, ebenfalls ausgestellt 26.03.2025 und abgelaufen 31.08.2026.
- Deckblatt: damaliger Hauptsitz Am Ortsgüterbahnhof 9. **Standortanlage Seite 2 dagegen Hafenstraße 110, Hamm**, eigene Kennnummer E915A70700; Beschreibung Lagerung und Behandlung nicht gefährlicher Abfälle/Eisen- und Nichteisenschrotte.
- Damit ist „Hafen nur Warenlager, niemals Schrottverarbeitung“ zu absolut: Das historische Zertifikat weist auch Schrottlager-/Behandlungsanlagen aus. Es beweist trotzdem **keine heutige öffentlich zugängliche Ankaufannahme**.
- PLZ-Widerspruch offen: Hafen-Zertifikat nennt **59063**, aktuelle Betreiberkarte **59067**. Nicht durch Vermischung auflösen; für bestehende Ortsgüterbahnhof-Zeile irrelevant.

Aktuelle Betreiberseite https://hark-hamm.de/rohstoffhandel/nutzeisen beschreibt ausdrücklich einen **2007 gegründeten zusätzlichen Standort im Hammer Hafengebiet** für den Handel mit Nutzeisen. „Zusätzlich“ plus separate Standortanlagen und Kennnummern widerlegen die unbelegte Gleichsetzung.

### D. Nachvollziehbare Registerkette: Hamm → Dortmund ist Sitzverlegung

Tatsächlich abgerufen:

- https://www.northdata.de/Heinrich%20Hark%20GmbH%20&%20Co%C2%B7%20KG%20Rohproduktenhandlung,%20Dortmund/HRA%2019998
- https://www.northdata.de/?id=6653878560620544
- https://hark-hamm.de/impressum

Northdata-Firmenseite verknüpft **AG Hamm HRA 108** und **AG Dortmund HRA 19998** bei derselben Heinrich Hark GmbH & Co. KG Rohproduktenhandlung. Die tatsächlich gelesene Veröffentlichung vom **09.03.2026**, Eintragung Nr. 19, lautet:

> Geändert, nunmehr: Dortmund. Geschäftsanschrift: Hartmannstraße 5, 44147 Dortmund. … Der Sitz ist nach Dortmund (Amtsgericht Dortmund HRA 19998) verlegt.

Das aktuelle Impressum nennt Heinrich Hark GmbH & Co. KG, Hartmannstraße 5, 44147 Dortmund, HRA 19998. Die zwei Quellen sind für die Sitz-/Registerkette kongruent. **Keine daraus erfundene physische Verlagerung der Hamm-Anlage.** Kein amtlicher aktueller Registerauszug selbst abgerufen; Registerkette hier über Northdata-Bekanntmachung plus Betreiber-Impressum, nicht als eigene Handelsregistereinsicht ausgeben.

## Gegenwartsbefund, Kontakte und Öffnungszeiten

1. https://hark-hamm.de/unternehmen/standorte tatsächlich als Markdown **und HTML** abgerufen. Markdown unterschlägt die dynamischen Standortdaten. HTML enthält ein `locations`-Array mit genau drei Orten: Dortmund/Hartmannstraße 5, Duisburg/Sympherstraße 96a und **Standort Hafen / Hafenstraße 110, 59067 Hamm / 51.6767, 7.7706**. Hafenbeschreibung: „Rohrlager, Coil- und Spaltbandlager. Bahnanschluss, Schere, Presse.“ **Am Ortsgüterbahnhof 9 fehlt.**
2. https://hark-hamm.de/kontakt/anfahrt als HTML abgerufen: dieselben drei Standortbezeichnungen, kein Ortsgüterbahnhof. Das ist relevante aktuelle Gegenevidenz zur ungeprüften Aktiv-Fortschreibung, aber kein Schließungsbeweis.
3. https://hark-hamm.de/kontakt/kontaktdaten und Impressum erneut abgerufen: **02381/54304-0** und **info@hark-hamm.de** bestätigt. Kontakttabelle führt unter derselben Rufnummer Geschäftsleitung, Ankauf Metall und Schrott, Verkauf Fe-Schrott, Verkauf NE-Metalle und Nutzrohre, sogar Lagerleitung Duisburg. Daher klar **zentrale Unternehmenskontakte**, kein Beleg einer direkt besetzten Hamm-Schrottplatznummer. Bestehende Werte können mit dieser Einordnung bleiben; aus Dortmunder Impressumsadresse nicht als „falsche Hamm-Kontakte“ löschen.
4. Betreiberprofil https://hark-hamm.de/unternehmen/profil nennt Annahme von Eisen-/Metallschrott aus Industrie und Privathaushalten an drei Standorten, ohne Zuordnung jeder Annahmefunktion zu einer Adresse. Diese generische Aussage schließt die standortspezifische Lücke nicht. Altersformulierungen („seit inzwischen 15 Jahren“ zertifiziert) und alte Umweltzahlen zeigen zudem, dass Copyright 2026 allein kein Aktualitätsbeweis ist.
5. Öffnungszeiten bleiben leer: Im HTML von Standort-/Anfahrtseite sind Mo–Fr 07:30–16:30 und Sa 07:00–12:00 **auskommentiert**, während die tatsächlich ausgegebene Tabelle leer ist. Das sind auffindbare historische Templatewerte, keine veröffentlichte aktuelle Annahmezeit. Damit ist die frühere Aussage „keine Fundstelle“ zu präzisieren, ihre Nichtübernahme aber richtig.
6. https://hark-hamm.de/unternehmen/zertifikate liefert im tatsächlichen Abruf **404**. Die beiden direkten PDFs sind trotzdem HTTP 200. Keinen 404 der Übersichtsseite als Beweis interpretieren, dass sämtliche Zertifikate verschwunden seien; ebenso wenig funktionierende alte PDFs als Erneuerung werten.

## Ankauf / Verkauf / Gebühren – strikt getrennt

| Richtung | Tatsächlich gelesene Betreiberquelle | Ergebnis und Integrationsgrenze |
| --- | --- | --- |
| Ankauf | https://hark-hamm.de/rohstoffhandel/ne-metalle ; https://hark-hamm.de/kontakt/kontaktdaten | Kupfersorten werden „zu täglich marktgerechten Preisen angekauft“, Kupferlegierungen zu handelsüblichen Preisen. Ankaufkontakt vorhanden. **Keine numerischen Ankaufskurse**, keine aktuelle adressgenaue Annahmezusage für Ortsgüterbahnhof oder Hafen. |
| Verkauf | https://hark-hamm.de/rohstoffhandel/nutzeisen ; Kontaktdaten | Rohre/Sonderposten, Bestandsauskunft durch Kontakt; eigener Hafenzusatzstandort. **Keine numerischen Verkaufspreise.** „Tagespreise“ für Ankauf nicht auf Rohrverkauf übertragen. |
| Gebühren/Service | https://hark-hamm.de/service/container | Container-/Behälterangebot bis 40 m³, Sondercontainer auf Anfrage. **Keine Miet-, Transport-, Entsorgungsgebühren oder Kostenfreiheitszusage** in der gelesenen Seite. Volumen und Nutzlast sind keine Preise oder Mindestankaufmengen. |

Kein belastbarer Preisfeed/Handler-Kandidat aus den abgerufenen Seiten. Kein `dropoff_json.allowed: true`, keine Kundentyp-/Zeiten-/Abholpauschalen für diesen Platz aus allgemeiner Unternehmenswerbung erzeugen.

## Feldweises Owner-Handoff

| Feld | Empfehlung | Beleg / Grenze |
| --- | --- | --- |
| slug | unverändert | Identitätsschlüssel, keine kosmetische Umbenennung |
| name/city/type | bestehende Hamm-Schrottplatzidentität erhalten | Historischer Standort durch A+B belegt, kein Anlass zur Dortmunder Umadressierung |
| street/postcode | `Am Ortsgüterbahnhof 9` / `59063` nachtragen | A+B standortspezifisch; zeitliche Grenze in Timeline festhalten |
| status | `pruefung` | aktuelle Auslistung, kein neuer Fortbetriebsnachweis; weder Sitzverlegung noch Ablauf allein ist Schließung |
| website / website_status | bestehende URL / `aktiv` behalten | Website tatsächlich erreichbar; getrennt von Betriebsstatus behandeln |
| phone/email | bestehende Werte behalten, als zentral kennzeichnen | aktuelles Impressum + Kontakttabelle; keine Direktstellenzusage |
| opening_hours | leer lassen | aktive Tabelle leer, alte Zeiten nur HTML-Kommentar |
| dropoff/pickup | nicht befüllen | konkrete aktuelle Annahme-/Abholbedingungen fehlen |
| GEO | keine Hafenkoordinate importieren | falscher Standort; keine neue Ortsgüterbahnhof-Koordinate verifiziert |
| certifications | nicht als aktuell gültig ergänzen | gefundene Zertifikate abgelaufen; historischen Beleg in Timeline verwenden |

Die im Dossier genannte Produktionskoordinate 51.2629/7.7239 bleibt laut Dossiervorbefund problematisch, wurde hier aber weder aus DB noch kartographisch frisch geprüft. **Wichtig für Owner:** Laut README löschen leere Frontmatter-GEO-Werte vorhandene DB-Koordinaten nicht. Kein stillschweigender „Pin korrigiert“-Erfolg durch bloßes Nichtbefüllen behaupten; dafür separater geprüfter Korrekturweg erforderlich.

## Offene Grenze / nächster entscheidender Nachweis

Für `aktiv` des konkreten Platzes fehlt ein **gegenwärtiger standortspezifischer Betreiberbeleg oder aktueller amtlicher Fortbetriebsnachweis** nach der 2026er Sitzverlegung und aktuellen Auslistung. Zielgerichtete Rückfrage über 02381/54304-0 bzw. info@hark-hamm.de: Wird Am Ortsgüterbahnhof 9 heute weiter betrieben, findet dort Kundenannahme statt, für wen und zu welchen Zeiten, oder wurde der Platz stillgelegt/verlagert? Bei Verlagerung Datum und Zieladresse schriftlich bestätigen lassen. Kein Telefonat und keine E-Mail in dieser Prüfung durchgeführt.

Ein neues Zertifikat könnte die Standortfrage stärken, ersetzt aber nicht automatisch die Zusage öffentlicher Annahme. Der Hafen darf nicht als Ausweichadresse eingetragen werden, solange die Verlagerung gerade dieses Platzes nicht nachgewiesen ist. Eine eigenständige Hafen-Zeile wäre eine separate Owner-Entscheidung mit eigener Funktions-/PLZ-Prüfung.

## Such-/Abrufgrenzen

Eigenständige Websuchen: `"Heinrich Hark" "Ortsgüterbahnhof" Hamm`, `"Heinrich Hark" "HRA 19998"`, `"Heinrich Hark" "2026" "Hamm"`, `"Heinrich Hark" "Ortsgüterbahnhof" geschlossen`, `"Hark" "Ortsgüterbahnhof" "2026" Schließung`. Diese erschlossen insbesondere die PDFs und Registerbekanntmachung. Kein qualifizierter Schließungsbeleg gefunden; ein negativer Suchfund ist kein Fortbetriebsbeweis.

HammWiki, Cylex, 11880, Schrottplatz-Portale, Exa/MapQuest und Jobaggregatoren erschienen in Suchergebnissen. Sie wurden **nicht als Belege** für Adresse, Aktivstatus, Öffnungszeiten oder GEO gezählt; deren widersprüchliche Öffnungszeiten und „geschlossen“-Tageslabels sind keine Betriebsstilllegung. Suchsnippet-Daten wurden nicht als tatsächliche PDF-Lektüre ausgegeben: Die entscheidenden PDFs wurden anschließend separat vollständig heruntergeladen und textlich ausgewertet.

Zusätzlich tatsächlich gelesen: https://hark-hamm.de/unternehmen/umwelt (Umwelttext mit 2018er Kennzahlen und Link zum Nachhaltigkeitsbericht 2024). Der dort verlinkte Nachhaltigkeitsbericht wurde **nicht heruntergeladen** und ist kein tragender Beleg dieser Entscheidung. Ein erster `webfetch`-Versuch auf das Behörden-PDF scheiterte am nicht unterstützten PDF-Inhaltstyp; erst der anschließende HTTP-Download mit Textauswertung lieferte den oben dokumentierten vollständigen Inhalt.

Owner integriert, dokumentiert die Gegenevidenz in der Timeline und testet; diese Review beansprucht keine Dossierintegration, Seedtests oder Produktionsveröffentlichung.
