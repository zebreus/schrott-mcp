# Dossier-Lücken-Recherche vom 28.10.2026: MHG Recycling Dessau, „Sebastian Schrotthändler“ (Dietzenbach), „Emji-Schrott / Gogircea Alexandru“ (Dietzenbach)

**Auftrag:** Owner-Check-in 28.10.2026. Recherche und **Entwurf** von Dossiers für drei fehlende bzw. zu
klärende Datensätze. **Keine Dossier-Datei im Repo wurde angelegt oder geändert**; dieser Bericht ist die
einzige Lieferung. Kein Commit, kein Push, kein Deploy, keine Änderung an Produktionsdatenbanken
(nur `file:…?mode=ro` + `PRAGMA query_only=ON`).

**Methodik:** Betreiberseiten direkt abgerufen (Startseite, Impressum, Kontakt, Leistungs- und
Preisseiten, Sitemap), amtliche Register (Gemeinsames Registerportal der Länder `handelsregister.de`,
Fachbetrieberegister/Register der Entsorgungsfachbetriebe über `schrottregister.pages.dev`, Stand des
Registerabrufs 11.08.2026), EU-VIES-Dienst, OSM/Nominatim, Verzeichnisse (11880, Das Telefonbuch,
lokaleschrottplatz.de) **ausschließlich als Leads**. Jede Belegaussage ist mit Quelle und Abrufdatum
versehen.

**Korrekturen an der Auftragsprämisse (vorab, weil sie die Ergebnisse tragen):**

1. **Die MHG-Preisliste ist nicht mehr vom 20.10.2026.** Die Seite weist am 28.10.2026
   **„letzte Preisänderung 27.10.2026“** aus. Alle Werte der Auftragsliste stimmen, **außer
   V2A Edelstahl (rein): 0,65 €/kg statt 0,70 €/kg**. Der Datumsstand im Auftrag ist einen Tag alt.
2. **Die Domain aus dem Auftrag existiert nicht:** `sebastian-schrotthaendel.de` ist **nicht im DNS
   auflösbar** (ENOTFOUND). Die live Domain der Marke ist **`sebastian-schrotthandel.de`** (mit „d“).
   Die Schreibweise des Auftrags stammt sehr wahrscheinlich aus dem **URL-Slug eines 11880-Eintrags**
   (`…/sebastian-schrotthaendler-schrottabholung.html`) — dort ist die verlinkte Domain korrekt
   `https://sebastian-schrotthandel.de`.
3. **Container: nicht „3–40 m³“.** Das Anfrageformular der MHG nennt als Größen
   **„7–10 m³ Absetzer“, „20 m³ Abroller“, „40 m³ Abroller“** — keine 3-m³-Variante, **keine Tarife**.
4. **Die MHG steht nicht unter eigenem Namen im amtlichen Fachbetrieberegister.** An der Anschrift
   Taubenstraße 37 führt der Registerauszug vom 11.08.2026 weiterhin **„Transporte und Altmetallhandel
   Horst Habermann“** (Zertifikat ZZPT00700034). **MHG Recycling GmbH hat damit nachweislich keine eigene
   Zertifikatsnummer** — ZZPT00700034 gehört der Vorgängerin und darf nicht auf MHG übertragen werden.

## Ergebnis in Kürze

| Datensatz | Befund | Empfehlung an den Owner |
|---|---|---|
| **MHG Recycling (Dessau)** | Eigener, vollständig belegter Standortbetrieb: Betreiberseite mit Impressum (HRB 37461 AG Stendal, USt-IdNr. DE462718707 **VIES-gültig**), datierte Ankaufspreisliste (Stand 27.10.2026), Öffnungszeiten, Containergrößen, Demontage. Im amtlichen Handelsregisterportal **nicht auffindbar** (Negativbefund mit Positivkontrolle). Im Fachbetrieberegister **nicht unter eigenem Namen** geführt. | **Eigenes Dossier `st-dessau-rolau-mhg-recycling`** anlegen (Entwurf unten). Legacy-Datensatz Horst Habermann **unverändert bestehen lassen**, nur um Querverweis ergänzen. |
| **„Sebastian Schrotthändler“ (Dietzenbach)** | Live-Marke mit **eigenem Auftritt, eigenem Impressum, eigener Telefonnummer/E-Mail, eigenen Öffnungszeiten und eigenen Leistungszusagen**; W-IdNr **DE452409549 identisch** mit STALCANT, Leeraumhelden-Impressum nennt „Sebastian Schrotthandel“ → **ein Betrieb, drei Marken**. | **Eigenes Dossier `he-dietzenbach-sebastian-schrotthandler`** (eigene Marke = eigener Datensatz, Präzedenz Autoverwertung Bolte/AIPEX). **Kein Merge**, keine Löschung; Markenkette in beiden Dossiers dokumentieren. |
| **„Emji-Schrott / Gogircea Alexandru“ (Dietzenbach)** | **Nur ein Aggregator-Lead** (11880: Telefon (0178) 1997566, Marktheidenfelder Weg 2, keine Website, keine E-Mail, keine Öffnungszeiten). Kein Impressum, kein Registereintrag, kein zweiter Beleg. | **Eigenes Dossier `he-dietzenbach-emji-schrott`** mit `status: pruefung` und **ohne Frontmatter-Füllungen** (Lead bleibt im Überblick dokumentiert). **Keine** Alias-Kette zu Cristofor (anderer Name, andere Rufnummer, kein Beleg). |

---

## Teil A — MHG Recycling GmbH, Taubenstraße 37, 06847 Dessau(-Roßlau)

### A.1 Betreiberidentität

**Betreiber-Primärquelle (Impressum, direkt abgerufen am 28.10.2026, `https://www.mhg-recycling.de/impressum`):**

| Feld | Angabe |
|---|---|
| Firma | MHG Recycling GmbH |
| Anschrift | Taubenstraße 37, 06847 Dessau-Roßlau |
| Registergericht | Amtsgericht Stendal |
| Registernummer | HRB 37461 |
| Vertretung | Nicolas Gaudefroy (Geschäftsführer) |
| USt-IdNr. | DE462718707 |
| Telefon | +49 340 5210947 |
| E-Mail | info@mhg-recycling.de |
| Hosting/Technik | WIX (www.mhg-recycling.de) |

**Unabhängige Bestätigung der Steuernummer (amtlich):** Der EU-VIES-Dienst
(`https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/462718707`, Abruf 28.10.2026
05:33 UTC) liefert `isValid: true` / `userError: "VALID"` für **DE462718707**. Positivkontrolle
DE276868959 (im Repo bereits dokumentiert) ebenfalls `VALID`. **VIES legt für Deutschland weder Namen
noch Anschrift offen** (`"name": "---"`, `"address": "---"`) — die Prüfung bestätigt die Steuernummer,
nicht deren Inhaberschaft.

**Weitere Registerdaten-Dienstleister (Suchindex-Auszüge, nicht direkt abgerufen — die Seiten liefern
403/CAPTCHA):** Creditreform-Firmenauskunft („wird beim Amtsgericht 39576 Hansestadt Stendal unter der
Handelsregister-Nummer HRB 37461 geführt … wirtschaftsaktiv“), Companyhouse („Gegründet 2026“, „HRB 37461
Amtsgericht Stendal“), Implisense („HRB 37461 (Stendal)“). Das sind **Daten-Dienstleister, keine
amtlichen Register**; die Abrufe waren technisch blockiert, die Angaben stammen aus dem Suchindex.

### A.2 Amtliches Handelsregister — Negativbefund mit Positivkontrolle

Recherche im **Gemeinsamen Registerportal der Länder** (`handelsregister.de`, Normale Suche, Abruf
28.10.2026; Gerichtscodes aus der Gerichtsliste der Seite abgeleitet, AG Stendal = `W1215`):

| Suche | Ergebnis |
|---|---|
| „MHG Recycling“ + AG Stendal + HRB + Nr. 37461 | **0 Treffer** |
| „MHG Recycling“ + **exakter Firmenname** (Suchoption „genauen Firmennamen enthalten“) | **0 Treffer** |
| „MHG“ + AG Stendal, alle Registerarten | **0 Treffer** |
| Registernummer 37461 + HRB + AG Stendal | **0 Treffer** |
| Registernummer 37461 + HRB, **alle** Gerichte | **0 Treffer** |
| „Recycling“ + AG Stendal, alle Registerarten | 2 Treffer (Hoffmann & Ernst Schrotthandel u. Recycling GmbH HRB 22322; URAG Umwelt-, Recycling- und Abwassertechnologie GmbH HRB 209429) — **kein MHG** |
| Sitz „Dessau“ + AG Stendal | 4 Treffer (ANTEA Bestattungen, Diakoniegesellschaft Wohnen und Arbeiten mbH, GfZ-Gesellschaft für Zeitarbeit mbH, Medix Spezialservice GmbH) — **kein MHG** |
| **Positivkontrolle 1:** „Vetter“ + AG Darmstadt | 1 Treffer (Karl Gerber GmbH, HRB 60382) |
| **Positivkontrolle 2:** HRB 209429 + AG Stendal | 1 Treffer (URAG) |
| **Positivkontrolle 3:** HRB 60382 + AG Darmstadt | 1 Treffer (Karl Gerber GmbH) |
| **Positivkontrolle 4:** „Gaudefroy“ alle Gerichte | 1 Treffer, aber „Godefroy H. von Mumm & Co. Sektkellereien GmbH“ (Schreibweise, keine Übereinstimmung) |

**Bewertung:** Die Suchmechanik ist mit drei Positivkontrollen belegt; MHG Recycling GmbH bzw. die
Nummer HRB 37461 sind dort **nicht auffindbar**. Das ist ein **Negativbefund und kein Nichtexistenzbeweis**:
Eine 2026 gegründete GmbH ohne veröffentlichte Registerunterlagen (Anmeldung nach § 2 GmbHG wird nicht
veröffentlicht) taucht in der Portal-Suche nicht auf; auch ein Indexrückstand ist möglich. **Der
Registereintrag ist damit aus öffentlichen, abrufbaren amtlichen Quellen derzeit nicht bestätigt.**
Konsequenz für die Dossier-Füllung: die Ausnahme „authoritative operator primary source“ (README) ist
**nicht erfüllt**, weil ein Kriterium („the entity is live in the register“) nicht nachweisbar ist → es
gilt die 2-Quellen-Regel.

### A.3 Amtliches Fachbetrieberegister (Efb) — Stand 11.08.2026

Ortsseite Dessau-Roßlau (`schrottregister.pages.dev/ort-dessau-rosslau`, 4 Betriebe):

| Betrieb | Anschrift | Tätigkeiten |
|---|---|---|
| G. Schönemann Entsorgung GmbH | Daheimstraße 5 | Sammeln, Befördern, Makeln |
| Schima GmbH & Co. KG Umweltdienste Dessau | Neuer Acker 21 | Lagern, Behandeln, Verwerten |
| TSR Deutschland GmbH & Co. KG, Niederlassung Dessau-Roßlau | Hünefeldstraße 3c | Lagern, Behandeln |
| **Transporte und Altmetallhandel Horst Habermann** | **Taubenstraße 37** | **Lagern, Behandeln** — Zertifikat **ZZPT00700034**, Abfallschlüssel 160106 (Altfahrzeuge) |

**MHG Recycling GmbH ist dort nicht als eigener Betrieb geführt.** Der Auszug datiert vom 11.08.2026 und
liegt damit **zehn Tage nach** der von MHG genannten Übernahme zum 01.08.2026. Drei Lesarten sind mit
öffentlichen Daten nicht unterscheidbar: (a) MHG hat die Efb-Anzeige noch nicht abgegeben / das Register
ist noch nicht nachgeführt, (b) der Auszug ist zu alt, (c) die Anlage läuft weiterhin unter dem Zertifikat
der Vorgängerin. **Belegbar ist nur: MHG hat keine eigene Zertifikatsnummer; ZZPT00700034 gehört
„Transporte und Altmetallhandel Horst Habermann“.** Ein Negativbefund beweist kein Nichtbestehen — für
reine Metallhändler ohne gefährliche Abfälle ist ohnehin keine Efb-Anzeige erforderlich.

### A.4 Preisliste, Ankauf / Verkauf / Gebühren (live am 28.10.2026)

Die Preisseite ist die Startseite (`https://www.mhg-recycling.de/`, Navigationspunkt „Preise“ verweist auf
den Anker `#Aktuelle Einkaufspreise`); keine separate Preisseite, kein PDF. Seitenangabe:
**„letzte Preisänderung 27.10.2026“**. Einleitender Text: *„Alle hier genannten Preise entsprechen dem
heutigen Stand und unterliegen marktabhängigen Schwankungen. Wir vergüten tagesaktuell auf Basis der
globalen Warenbörse. Haftung für Preisangaben und Irrtümer ausgeschlossen.“*

**ANKAUF (€/kg, Betreiberangabe „übliche Klein- und Standardmengen bei Anlieferung an unserem Standort in
Dessau“; größere Mengen und Gewerbekunden: individuelle Ankaufkonditionen):**

| Sortenbezeichnung Betreiber | €/kg |
|---|---|
| Schredder-Schrott | 0,08 |
| Mischschrott (leicht) | 0,14 |
| Mischschrott (schwer) | 0,19 |
| Kupfer-Kabel | 3,00 |
| Kupfer Raffinade (max. 5 %) | 8,30 |
| Kupfer Millberry | 9,60 |
| Altzink (rein) | 1,40 |
| Altblei (rein) | 1,00 |
| Messing (rein) | 5,00 |
| Alu-Schrott mit Anhaftungen (max. 5 %) | 0,90 |
| Aluminium alt/neu (rein) | 1,30 |
| V2A Edelstahl (rein) | **0,65** (seit 27.10.2026; zuvor 0,70) |
| Altpapier | 0,09 („Ab 100 kg zahlen wir 0,10 € pro kg“) |

**VERKAUF:** keine Material-, Teile- oder Leistungsverkaufspreise veröffentlicht.

**GEBÜHREN:** **Containerdienst ohne Tarif.** Das Anfrageformular („Container anfragen“) fragt
`Anliegen` (Container anfragen / Containertausch), `Größe` (**7–10 m³ Absetzer, 20 m³ Abroller,
40 m³ Abroller**), `Sorte` (Mischschrott / Baumischabfall / Andere), Wunschtermin und den
**Containerstellort** ab; Preise werden nicht genannt. **Demontage** (Industrieanlagen,
Metallkonstruktionen) wird beworben, ebenfalls ohne Tarif. **Eine Abholung von Schrott beim Kunden wird
nicht angeboten** — die Startseite sagt „Kommen Sie mit Ihren Materialien zu uns“ (Anlieferung).
**Barzahlung oder Sofortüberweisung** sind Zahlungsarten, **keine** Preiszuschläge.

### A.5 Öffnungszeiten, Kontakt, Annahme

* Öffnungszeiten (Startseite, Abschnitt „Öffnungszeiten“): **Montag–Freitag 07:00–16:00 Uhr, Samstag und
  Sonntag geschlossen** — unverändert gegenüber der bereits am 27.10.2026 im Legacy-Dossier dokumentierten
  Angabe; die Öffnungszeiten stehen **nur** auf der Betreiberseite (kein Verzeichnis- oder Registerbeleg).
* Kontakt: Tel. **+49 340 5210947**, **info@mhg-recycling.de**, Kontaktformular; **keine** Postfach-Adresse.
* **Anlieferung** ist am Standort ausdrücklich vorgesehen („bei Anlieferung an unserem Standort in
  Dessau“, „Kommen Sie mit Ihren Materialien zu uns“).
* **Annahmebedingungen** sind nicht als Liste/Ausschlussliste veröffentlicht; es gibt nur den
  Material-Hinweis „Material nicht dabei? Preis anfragen!“ und die Mengen-/Konditionen-Klausel.
* Ein **Eigenbericht über Ausschlüsse** (Gasflaschen, Feuerlöscher, Reifen) fehlt bei MHG — anders als bei
  der Dietzenbach-Marke „Sebastian Schrotthandel“.

### A.6 Geokodierung

Nominatim (Abruf 28.10.2026) liefert für „Taubenstraße 37, 06847 Dessau-Roßlau“ **nur zwei
straßenobjektbezogene Treffer** (`Taubenstraße, West, Dessau` 51.8280136/12.2267451 als residential way
25156926; 51.8246403/12.2273533 als footway 217881636) — **kein adressgenauer Hausnummernpunkt**. Es wird
**keine Koordinate** in den Entwurf übernommen; für den Entwurf gilt dasselbe wie im Legacy-Datensatz
(`clear_fields: 'geo'`). Die bestehende, zurückgezogene Produktions-Koordinate 51.8333/12.25 bleibt
unangetastet.

### A.7 Betreiberkette Habermann → MHG

* **Betreiberaussage (Startseite, 28.10.2026):** *„die MHG Recycling GmbH hat den Entsorgungsfachbetrieb von
  Horst Habermann durch dessen Eintritt in den altersbedingten Ruhestand zum 01.08.2026 übernommen. Die
  gewohnten Abläufe sowie die zuverlässige Betreuung bleiben für Sie erhalten. … Es ist uns eine große
  Freude, Sie an unserem Standort in der Taubenstraße 37 in Dessau begrüßen zu dürfen.“* — gezeichnet
  Nicolas Gaudefroy, Geschäftsführer.
* **Unabhängiger Beleg für den Ort:** Das amtliche Register führt den Betrieb „Transporte und Altmetallhandel
  Horst Habermann“ an **Taubenstraße 37, 06847 Dessau-Roßlau** (ZZPT00700034) — dieselbe Hausnummer, die
  MHG als eigenen Standort nennt, und dieselbe Hausnummer, die im Legacy-Datensatz `street`/`postcode`
  trägt.
* **Was damit belegt ist:** Der *Standort* (Taubenstraße 37) und die *Tätigkeit* (Metallannahme) sind
  doppelt belegt (Betreiberseite + amtliches Register). **Nicht belegt ist:** die gesellschaftsrechtliche
  Kontinuität (kein Register- oder Vertragsdokument), die Fortgeltung der Efb-Erlaubnis auf MHG und die
  Höhe/Form des Übernahmepreises für den Geschäftsbetrieb.
* Die frühere Domain **horst-habermann.de** ist weiterhin **keine Betreiberquelle**: sie leitet (1 Redirect)
  auf eine **Domain-Verkaufsseite** (`expireddomains.com` → ED.com, „Horst-Habermann.de, Premium domain,
  for sale“, USD 100). Das ist ein Domainbefund, kein Schließungsurteil über den Standort.

### A.8 Offene Punkte (Owner)

1. **Registerexistenz MHG Recycling GmbH / HRB 37461 AG Stendal** — im Portal nicht auffindbar.
   Klärwege: (a) kostenpflichtige **Aktuelle Handelsregisterauszug**-Anfrage beim Amtsgericht Stendal
   (Portal-Recherche scheitert an der Auffindbarkeit); (b) **Gewerbescheinweis/Gewerbeanmeldung** der
   Betriebe direkt; (c) MHG um **eigene Efb-Anzeige/Zertifikatsnummer** nach § 56 KrWG bitten.
2. **Efb-Status nach dem Betreiberwechsel** (Zertifikat weiterhin auf Habermann) — Klärung über MHG oder
   über die zuständige Stelle; die amtliche Liste wird erst bei der nächsten Erhebung neu ausgewertet.
3. **Betriebsstätte als Annahmehof** — die Startseite belegt Anlieferung; ein Standortfoto/Bildbeleg für
   die Waage/Kasse fehlt. Für GEO bleibt das irrelevant (kein adressgenauer Punkt).
4. **Materialkatalog**: Für die MHG-Sorten existiert im `materials`-Katalog **kein Altpapier-Eintrag**
   (Abfrage `name_de LIKE '%Papier%'` → 0 Treffer). Ein Handler dürfte keine Materialien erfinden
   (`docs/trader-seed.md`) — für Altpapier wäre **zuerst ein Katalogeintrag** zu beschließen. Außerdem
   steht der Katalog-Slot `stahlschrott-shredder` auf **EUR/t**, die MHG rechnet in **EUR/kg**
   (0,08 €/kg = 80 €/t) — eine Normalisierung, die der Owner explizit freigeben sollte.

---

## Teil B — „Sebastian Schrotthändler“ / Sebastian Schrotthandel (Dietzenbach)

### B.1 Die Marke ist live und hat einen eigenen Auftritt

Direktabrufe am 28.10.2026 (alle HTTP 200, `sebastian-schrotthandel.de`, WordPress, Sitemap
`/page-sitemap.xml`): Startseite, `/impressum/`, `/kontakt/`, `/schrottabholung/`, `/schrottankauf/`,
`/containerdienst/`, `/demontage/`, `/entruempelung/`.

**Impressum (wörtlich):**
> Anschrift — Sebastian Schrotthandel, Geschäftsführer: Caldarar Sebastian Traian, Robert-Koch-Str. 9,
> 63128 Dietzenbach, Tel: +49 160 95471472, Tel: +49 163 0056553, E-Mail: info@sebastian-schrotthandel.de,
> Web: sebastian-schrotthandel.de, **Wirtschafts-ID: DE452409549**.
> Technische Wartung und Pflege: ffm.media e.K., Paul-Ehrlich-Str. 38, 63322 Rödermark.

**Kein HRB, keine USt-IdNr** im Impressum dieser Marke (im Gegensatz zur Leeraumhelden-Fassung, die
„HRB folgt / Umsatzsteuernummer-ID: folgt“ schreibt). Es handelt sich um ein **Einzelunternehmen**.

### B.2 Der Register-/Identitätsbefund: ein Betrieb, drei Marken

| Marke | Domain | Impressum-Kern | Kontakt | Öffnungs-/Beratungszeiten |
|---|---|---|---|---|
| **STALCANT – der Schrotthändler** | stalcant-schrotthaendler.de | Sebastian Traian Caldarar, Robert-Koch-Straße 9, 63128 Dietzenbach, **W-IdNr DE452409549**, „Magistrat der Kreisstadt Dietzenbach“ (Reisegewerbekarte), kein HRB | 0163 5255962, info@stalcant-schrotthaendler.de | Beratung **Mo–Sa 8:00–22:30**, So geschlossen |
| **Sebastian Schrotthandel** | sebastian-schrotthandel.de | „Sebastian Schrotthandel“, Geschäftsführer: Caldarar Sebastian Traian, Robert-Koch-Str. 9, **W-IdNr DE452409549** | +49 160 95471472, +49 163 0056553, info@sebastian-schrotthandel.de | **Mo–Fr 07:00–20:00, Sa 08:00–19:00**, So geschlossen |
| **Leeraumhelden** | leeraumhelden.de | „Anschrift **Sebastian Schrotthandel**, Geschäftsführer: Caldarar Sebastian Traian, Robert-Koch-Str. 9 …, Web sebastian-schrotthandel.de, HRB folgt, UmsatzsteuernummerID: folgt“ | +49 160 95471472, +49 163 0056553, info@sebastian-schrotthandel.de | (Entrümpelungs-/Aufräum-Marke) |

**Die identische Wirtschafts-Identifikationsnummer DE452409549 in zwei unabhängig abgerufenen Impressen
(STALCANT und Sebastian Schrotthandel) ist der entscheidende Beleg:** die W-IdNr nach § 139c AO wird
wirtschaftlichen Einheiten/Einzelunternehmern zugewiesen, nicht Marken. Zusammen mit derselben Anschrift
und demselben Inhaber ergibt das **einen Betrieb mit drei Marken** (Scrap-Handel, Entrümpelung,
Lohnausräumen) — **kein** Verzeichnis-Doppel und keine zwei Betriebe.

**Gegenprobe (Trennschärfe):** Das Schwesterdossier `he-dietzenbach-leonard-schrotthandel` nennt
**Leonard Dorel Caldarar**, 0176 40067307, info@leonard-schrotthandel.de, **ohne** W-IdNr und **ohne**
HRB — andere Person, andere Rufnummer, andere E-Mail, gleiche Hausnummer. Eine Beziehung zwischen
Sebastian Traian und Leonard Dorel Caldarar ist aus öffentlichen Quellen **nicht** belegt (die
„Caldarar-Familie“ bleibt Hypothese). Ebenfalls getrennt: `he-dietzenbach-b-mugurel-schrotthandler`
(Mugurel Braica, **eigene** W-IdNr DE425479280, eigene Hausnummer 11).

### B.3 Leistungen und Konditionen der Marke „Sebastian Schrotthandel“ (eigene Aussagen)

* **Schrottabholung:** Abholung „innerhalb eines Tages“; „Wir sind darauf vorbereitet, Schrott ab einem
  **Gesamtgewicht von über 500 kg** abzuholen“; „Falls erforderlich, stellen wir Ihnen auch einen passenden
  Container zur Verfügung“; Einsatzgebiet Rhein-Main-Gebiet.
* **Schrottankauf:** „tagesaktuelle Preise … nach aktueller Marktpreisen“; **keine Zahlenkurse**;
  „sofortige Barauszahlung“; **Anlieferung** („Ihren Schrott direkt bei uns abzuliefern“) und Abholtermin;
  **Ausschlüsse**: Gasflaschen, Feuerlöscher, alte Reifen.
* **Containerdienst:** **ausschließlich Metallschrott**. Zulässig: Aluminium, Kupfer, Messing, Zink, Zinn,
  Blei, Kabelreste, Edelstahl (V2A), Aluminium-Offsetplatten. Nicht hinein: Fahrräder, Baustahl,
  Stahltüren, alte Nähmaschinengestelle, Heizkörper, Trägerreste, Werkzeuge, Metallmöbel/-gestelle.
  **Keine Größen, keine Tarife.**
* **Demontage:** Industriedemontage, Rückbau, **Asbestsanierung**, Entkernung — ohne Tarif.
* **Entrümpelung:** Komplett-/Teilentrümpelung, Haushaltsauflösung; **ausgeschlossen**: Asbest,
  Chemikalien, Sondermüll.
* **Preise:** **Ankauf** ohne Zahlenblatt; **Verkauf** keine Liste; **Gebühren** keine Tarife.
  **Achtung:** Die Mengen- und Kostenzusagen der Marke weichen von STALCANT ab (hier „über 500 kg“
  **ohne** Kostenfreiheitszusage; dort „**kostenlose** Abholung ab 350 kg“). Die beiden Marken dürfen in
  dieser Hinsicht **nicht** vermischt werden.

### B.4 Registerbefunde (Negativbefunde mit Positivkontrollen)

* **Handelsregisterportal (handelsregister.de, 28.10.2026):** „Caldarar“ → **0 Treffer**;
  „Gogircea“ → **0 Treffer**; „Ciurar“ → 9 Treffer, **alle ortsfremd** (Fuzzy-Treffer: Karrer & Pascu,
  Courier Network, Adler-Apotheke …) — kein Ciurar; „Emji“ → **>100 Treffer** (Suchbegriff unbrauchbar,
  keine Aussage). Positivkontrollen: „Vetter“/AG Darmstadt und HRB 209429/HRB 60382 liefern Treffer;
  „Gaudefroy“ → nur „Godefroy H. von Mumm“ (Schreibweise). **Einzelunternehmer werden nicht ins HRA/HRB
  eingetragen** — ein 0-Treffer-Befund beweist kein Nichtbestehen.
* **Fachbetrieberegister (Stand 11.08.2026), Ortsseite Dietzenbach:** genau zwei Betriebe —
  **Jakupi Transport Inh. Ramiz Jakupi** (Gallische Straße 1, Sammeln/Befördern, ZZFT00300487) und
  **Peter Vetter Abfallverwertung** (Karl-Benz-Straße 4). Weder „Sebastian“, „STALCANT“, „Leonard“,
  „Cristofor“, „Emji“ noch die geprüften Anschriften (Robert-Koch-Str. 3/9/11, Marktheidenfelder Weg 2,
  Max-Planck-Str. 3, Rohrbrunner Weg 2-4, Lohrer Weg, Messenhäuser Str. 2) erscheinen dort.
  **Negativbefund ohne Existenzbeweis** (für reine Metallhändler ohne gefährliche Abfälle ist keine
  Efb-Anzeige erforderlich).
* **Kommunale Gegenprobe:** Die Stadt Dietzenbach erteilt **Gewerberegister-Auskunft** nur auf Anfrage
  (einfache Auskunft 10,00–30,50 EUR; Gruppenauskunft je Person 7,50–15,00 EUR, mindestens 76,50 EUR).
  Das Gewerberegister ist kein öffentliches Register; eine Online-Einzelauskunft gibt es nicht. → **Der
  amtliche Gewerbenachweis für alle drei Caldarar-Marken bleibt ein schriftlicher Folgeschritt.**
* **Verzeichnisse (nur Leads):** 11880 kennt unter „Schrott in Dietzenbach“ nur **Leonard** (Robert-Koch-Str. 9)
  und **Emji-Schrott**. Die Marke selbst hat einen **eigenen 11880-Eintrag** „**Sebastian Schrotthändler /
  Schrottabholung**“ (`…/branchenbuch/dietzenbach/060440092B113866979/sebastian-schrotthaendler-schrottabholung.html`,
  am 28.10.2026 direkt abgerufen, HTTP 200): Branche **Altmetallrecycling**, Adresse **Robert-Koch-Str. 9,
  63128 Dietzenbach**, Telefon **(0163) 0056553**, E-Mail **info@sebastian-schrotthandel.de** (in der
  Seitenausgabe maskiert, im HTML vorhanden), Website **https://sebastian-schrotthandel.de**, keine
  Öffnungszeiten, keine Bewertungen. Das Verzeichnis reproduziert damit **genau die Kontaktspur der
  Betreiberseite** — es bleibt ein Aggregator und damit nach README **ein Einzelbeleg** (keine zweite
  unabhängige Quelle), bestätigt aber Domain-Schreibweise, Markennamen und die Zuordnung der Rufnummer
  (0163) 0056553 **zur Marke**. Daneben führt 11880 die **vierte Namensfassung** „Schrotthändler Dietzenbach
  kostenlose Schrottabholung **Dorel Horvath**“ (Robert-Koch-Str. 9, **(0163) 205242**, Altmetallrecycling,
  2 Bewertungen) mit Verweis auf das **inzwischen 404 liegende** sellwerk-Profil „HD Entrümpelung &
  Haushaltsaufloesung“ (am 28.10.2026 erneut geprüft: HTTP 404) — **andere Rufnummer, anderes (totes)
  Profil**. → **Robert-Koch-Str. 9 ist eine Mehrmarken- und Mehrbetreiberadresse**; „Dorel Horvath“ bleibt
  ungeklärt und wird **nicht** mit „Sebastian Traian Caldarar“ gleichgesetzt.

### B.5 GEO

Nominatim „Robert-Koch-Straße 9, 63128 Dietzenbach“ liefert einen **Hausnummern-Knoten** (`node 534547353`,
50.0050838/8.78789, „Gewerbegebiete Mitte und Süd, Spessartviertel, Altstadt, Dietzenbach“). Photon
(komoot) löst auf **denselben** OSM-Knoten auf — das ist **keine zweite unabhängige Quelle**, sondern
dieselbe Datenbasis. Der in Produktion gespeicherte Punkt der Dietzenbach-Zeilen (50.0098/8.7778) ist per
Reverse-Geocoding das Stadtmitten-Gebäude „6, Hammannsgasse“ und liegt **892 m** von der belegten
Anschrift entfernt (bzw. **798 m** von Marktheidenfelder Weg 2). → **Im Entwurf wird keine Koordinate
gesetzt** (zwei-Quellen-Regel nicht erfüllt; die Schwester-Dossiers halten es ebenso).

### B.6 Empfehlung: eigener Datensatz **oder** Alias-Kette?

**Empfehlung: eigener Datensatz `he-dietzenbach-sebastian-schrotthandler`.** Begründung:

1. **Eigener Auftritt mit eigenem Markenauftritt:** eigenes Impressum, eigene E-Mail-Domain, eigener
   Slogan („Ihr Experte für Schrottentsorgung und Recycling im Herzen des Rhein-Main-Gebiets“), eigene
   Serviceseiten-Struktur, eigenes Impressums-Layout (Paul-Ehrlich-Str. 38 statt Senfelderstraße 1F der
   Agentur ffm.media auf den Cristofor-/Leonard-/Lorenzo-Seiten).
2. **Abweichende Kontaktdaten und Konditionen:** eigene Rufnummern, eigene E-Mail, **andere
   Öffnungszeiten** (Mo–Fr 07:00–20:00 / Sa 08:00–19:00 vs. STALCANT Mo–Sa 8:00–22:30) und **andere
   Mengenschwelle** („über 500 kg“, ohne Kostenfreiheitszusage vs. „kostenlos ab 350 kg“). Eine
   Zusammenführung würde eine der beiden Eigenangaben falsch machen.
3. **Präzedenz im Repo:** Marke mit eigenem Datensatz und dokumentierter Betreiberkette ist etabliert
   (z. B. „Autoverwertung Bolte“ = Marke der AIPEX Transport & Trading GmbH, HRB 15192 AG Bad
   Oeynhausen, `hb-bremen-neustadt-bolte-h-g`; ebenso Hofmann-Metall-Betriebsteile und TSR-Niederlassungen).
4. **Kein Datenverlust, kein Merge:** Ein eigener Datensatz **löscht nichts**, führt nichts zusammen und
   lässt STALCANT unangetastet. Der Owner behält die Entscheidung; die Markenkette steht in beiden
   Dossiers.

**Alternativ (vom Owner zu entscheiden):** Führung als **Marken-/Alias-Kette** im bestehenden Dossier
`he-dietzenbach-stalcant-schrotthandler` — dann stünde die Alias-Kette nur im Fließtext, die
Marken-Website `sebastian-schrotthandel.de` wäre aber weiterhin ohne Datensatz, und eine Agenten-Abfrage
nach „Sebastian Schrotthändler“ liefe ins Leere. **Empfohlen wird Variante 1**, mit Querverweisen in beide
Richtungen.

### B.7 Umgang mit dem Alt-Datensatz `he-dietzenbach-stalcant-schrotthandler`

* **Nicht löschen, nicht umbenennen, nicht mergen.** Slug, Name und `status: pruefung` bleiben.
* Ergänzung: ein **einer Querverweis** auf das neue Dossier sowie die Markenkette (Sebastian Schrotthandel,
  Leeraumhelden) und die **Trennung der Konditionen** (350 kg kostenlos = STALCANT; 500 kg ohne
  Kostenfreiheitszusage = Sebastian Schrotthandel).
* Der gespeicherte Punkt 50.0098/8.7778 (Hammannsgasse 6) ist für **alle** betroffenen Zeilen falsch
  (892 m daneben) und sollte — wie bei `he-dietzenbach-b-mugurel-schrotthandler` bereits geschehen —
  über `clear_fields: 'geo'` **zurückgezogen** werden, **ohne** Ersatzkoordinate. Das betrifft die Zeilen
  `he-dietzenbach-stalcant-schrotthandler`, `he-dietzenbach-leonard-schrotthandel`,
  `he-dietzenbach-cristofor-schrotthandel` und die weiteren Fragmentzeilen mit demselben Punkt
  (`he-dietzenbach-63128-a-w-metallhandel`, `he-dietzenbach-b-samuel-schrottabholung`,
  `he-dietzenbach-miky-schrotthandler`, `he-dietzenbach-schrotthandler-dietzenbach-s-o`,
  `he-dietzenbach-schrotthandler-sabri`). **Eigene Entscheidung des Owners**, im Bericht nur empfohlen.
* **Kontaktskalare bleiben leer** (Einzelunternehmer ohne Registereintrag; Aggregatoren sind nur Leads).

---

## Teil C — „Emji-Schrott / Gogircea Alexandru“ (Marktheidenfelder Weg 2, 63128 Dietzenbach)

### C.1 Was vorliegt

**Einziger Beleg: ein Verzeichniseintrag (Lead, kein Beleg).** 11880-Einzelseite
`https://www.11880.com/branchenbuch/dietzenbach/060444309B113012766/emji-schrott-abholung-schrotthaendler-gogircea-alexandru.html`
(direkt abgerufen am 28.10.2026, HTTP 200):

* Firmenname: **„Emji-Schrott Abholung /Schrotthändler Gogircea Alexandru“**
* Branche: **Recycling**; Adresse: **Marktheidenfelder Weg 2, 63128 Dietzenbach**;
  Mobil: **(0178) 1997566**; **keine Website**, **keine E-Mail**, „keine Öffnungszeiten“, 0 Bewertungen.

Derselbe Datensatz taucht in weiteren, datengleichen Verzeichnissen auf (Das Telefonbuch,
stadtbranchenbuch.com, oeffnungszeitenbuch.de) — das ist **eine** Quelle in mehreren Spiegeln, keine
Bestätigung. **Keine Website, keine Impressum, kein Gewerbeeintrag, kein Preis, keine Leistungsbeschreibung
und kein Öffnungszeitbeleg sind auffindbar.** Auch eine Suche nach „Emji-Schrott“ findet keine eigenständige
Website.

### C.2 Abgrenzung zu „Cristofor Schrotthandel“ (gleiche Hausnummer)

| | Cristofor Schrotthandel | Emji-Schrott |
|---|---|---|
| Person laut Quelle | Cristofor Ciurar | Gogircea Alexandru |
| Telefon | +49 163 9392817 | (0178) 1997566 |
| Website | cristofor-schrotthandel.de (Impressum mit „HRB: folgt“, „UmsatzsteuerID: folgt“) | keine |
| Belegqualität | Betreiber-Selbstauskunft (1 Quelle) | Aggregator-Lead (1 Quelle, kein Betreiber) |

→ **Zwei verschiedene Namen, zwei verschiedene Rufnummern, zwei verschiedene Belegqualitäten an derselben
Hausnummer.** Ob zwei Betriebe oder zwei Marken eines Betriebes, ist **nur über ein Impressum oder einen
Registerbeleg zu „Emji-Schrott“** entscheidbar — beides existiert nicht. **Eine Alias-Kette zu Cristofor
ist damit nicht belegt und wird nicht behauptet.** Umgekehrt bleibt Cristofor eine eigenständige Marke mit
eigener Person; die gemeinsame Webagentur ffm.media e.K. (bei Cristofor mit Agenturanschrift Senfelderstraße
1F, 63110 Rodgau) ist **keine** Betreiberverbindung.

### C.3 Registerbefunde

* **Handelsregisterportal (28.10.2026):** „Gogircea“ → **0 Treffer**; „Emji“ → **>100 Treffer**
  (Suchbegriff unbrauchbar, keine Aussage); „Ciurar“ → 9 ortsfremde Fuzzy-Treffer. Positivkontrollen
  belegen die Suchmechanik. Einzelunternehmer werden nicht ins HRA/HRB eingetragen → **Negativbefund ohne
  Existenzbeweis.**
* **Fachbetrieberegister (Stand 11.08.2026):** Marktheidenfelder Weg 2 taucht in Dietzenbach **nicht** auf
  (nur Jakupi Transport, Gallische Straße 1, und Peter Vetter Abfallverwertung, Karl-Benz-Straße 4).
  **Negativbefund ohne Existenzbeweis.**
* **Kommunal:** Gewerberegister-Auskunft der Stadt Dietzenbach nur auf Anfrage (10,00–30,50 EUR);
  das ist der einzige amtliche Weg zu „Emji-Schrott“.

### C.4 GEO

Nominatim „Marktheidenfelder Weg 2, 63128 Dietzenbach“ liefert ein **Adress-Way** (`way 40922223`,
50.0142103/8.7866023, „Spessartviertel, Neue Mitte, Dietzenbach“). Da die Hausnummer **nur aus einem
Aggregator-Lead** stammt und keine zweite unabhängige Quelle vorliegt, wird **keine Koordinate** gesetzt.
Der gespeicherte Produktionspunkt 50.0098/8.7778 (Hammannsgasse 6) liegt **798 m** daneben und ist
falsch → Rücknahme über `clear_fields: 'geo'` empfohlen (siehe B.7).

### C.5 Empfehlung

**Eigenes Dossier `he-dietzenbach-emji-schrott`** mit `status: pruefung`, **ohne** Frontmatter-Füllungen
(keine Straße, keine PLZ, keine Rufnummer, keine Website, keine Öffnungszeiten, keine GEO). Der Lead wird
vollständig im Überblick und in der Timeline dokumentiert, damit der Datensatz nicht verloren geht und der
Owner später entscheiden kann. **Keine Alias-Kette zu Cristofor** (kein Beleg), **keine Löschung**.

---

## Teil D — Dossier-Entwürfe (Repo-Format, nur Entwurf — **nicht** angelegt)

> Format: YAML-Frontmatter mit ausschließlich den im Seed-Codegen erlaubten Keys, dann `## Überblick` und
> `## Timeline` mit datierten Notizen im Format `[Recherche 28.10.2026: …; Quelle(n): …]`.
> **Diese drei Dateien wurden nicht angelegt und nicht geändert.** Sie sind vom Owner zu prüfen,
> ggf. anzupassen und dann durch `crates/ingestion/build.rs` kompiliert zu lassen.

### D.1 `dossiers/st/st-dessau-rolau-mhg-recycling.md` (Vorschlag: **eigenes Dossier — ja**)

**Begründung für einen eigenen Datensatz:** MHG Recycling GmbH ist ein **eigenständiger Rechtsträger**
(eigene GmbH mit eigenem Impressum, eigener USt-IdNr, eigenem Geschäftsführer, eigener Website mit
eigenem Preisblatt) und **kein Umbenennungs- oder Aliasfall** des Legacy-Datensatzes. Der Legacy-Datensatz
dokumentiert einen **historischen** Rechtsträger („Horst Habermann“, Ruhestand 01.08.2026) mit
Eigenheiten im amtlichen Register; MHG ist der **heutige** Betreiber desselben Standorts. Ein einziger
Datensatz müsste entweder den historischen Rechtsträger (dann gehen die MHG-Preise verloren) oder den
heutigen (dann geht die Registerhistorie mit ZZPT00700034 verloren) abbilden — beides ist Datenverlust.
**Die repo-Regel „slugs NEVER change“ und „cross-reference sibling dossiers instead of merging“** spricht
eindeutig für zwei Datensätze.

```yaml
---
slug: st-dessau-rolau-mhg-recycling
name: MHG Recycling GmbH
trader_type: metallhaendler
state: ST
city: Dessau-Roßlau
street: 'Taubenstraße 37'
postcode: '06847'
phone: ''
email: ''
opening_hours: ''
website: https://www.mhg-recycling.de/
website_status: aktiv
status: pruefung
description: 'Betreiber am Standort Taubenstraße 37, 06847 Dessau-Roßlau: MHG Recycling GmbH, Impressum HRB 37461 Amtsgericht Stendal, USt-IdNr. DE462718707 (am 28.10.2026 über den EU-VIES-Dienst gültig), Geschäftsführer Nicolas Gaudefroy. Die Gesellschaft hat den Entsorgungsfachbetrieb "Transporte und Altmetallhandel Horst Habermann" nach dessen Ruhestand zum 01.08.2026 übernommen und veröffentlicht eine tagesaktuelle Ankaufspreisliste (letzte Preisänderung 27.10.2026) in EUR/kg: Schredder-Schrott 0,08; Mischschrott leicht 0,14 und schwer 0,19; Kupfer-Kabel 3,00; Kupfer Raffinade (max. 5 %) 8,30; Kupfer Millberry 9,60; Altzink rein 1,40; Altblei rein 1,00; Messing rein 5,00; Alu-Schrott mit Anhaftungen (max. 5 %) 0,90; Aluminium alt/neu rein 1,30; V2A Edelstahl rein 0,65; Altpapier 0,09 und ab 100 kg 0,10. Verkauf: keine Verkaufspreise. Gebühren: Containerdienst (7-10 m3 Absetzer, 20 m3 und 40 m3 Abroller) und Demontage ohne veröffentlichte Tarife; eine Schrottabholung wird nicht angeboten, Anlieferung am Standort dagegen ausdrücklich. Im amtlichen Fachbetrieberegister (Stand 11.08.2026) steht an dieser Anschrift weiterhin "Transporte und Altmetallhandel Horst Habermann" (Zertifikat ZZPT00700034, Lagern und Behandeln); MHG Recycling GmbH ist dort nicht unter eigenem Namen eingetragen und hat daher keine eigene Zertifikatsnummer. Der historische Datensatz st-dessau-rolau-horst-habermann-transport-und-altmetallh bleibt unverändert bestehen.'
dropoff_json: '{"allowed": true, "customer_types": ["privat", "gewerbe"], "days": ["Mo", "Di", "Mi", "Do", "Fr"], "time_windows": ["07:00-16:00"], "conditions": "Anlieferung an unserem Standort Taubenstraße 37, 06847 Dessau; Preise für übliche Klein- und Standardmengen, größere Mengen und Gewerbekunden individuell; Barzahlung oder Sofortüberweisung; Containerdienst 7-10 m3 Absetzer, 20 m3 und 40 m3 Abroller ohne Tarif; Demontage ohne Tarif. Betreiberangabe, Abruf 28.10.2026."}'
pickup_json: ''
provenance_section: 'Recherche Dossier-Lücken 28.10.2026'
provenance_ankauf_raw: ja
provenance_origin: table
---

# MHG Recycling GmbH

## Überblick

Die MHG Recycling GmbH betreibt den Standort **Taubenstraße 37, 06847 Dessau(-Roßlau)** und hat laut
Betreiberseite den Entsorgungsfachbetrieb von **Horst Habermann** nach dessen altersbedingtem Ruhestand zum
**01.08.2026** übernommen. Das Impressum nennt **HRB 37461 Amtsgericht Stendal**, USt-IdNr. **DE462718707**
(am 28.10.2026 über den EU-VIES-Dienst gültig), Geschäftsführer **Nicolas Gaudefroy**,
Tel. **+49 340 5210947**, **info@mhg-recycling.de**; Öffnungszeiten **Mo–Fr 07:00–16:00 Uhr**, Sa/So
geschlossen. Die **Adresse ist doppelt belegt**: Betreiberseite und amtliches Fachbetrieberegister, das an
Taubenstraße 37 den Betrieb „Transporte und Altmetallhandel Horst Habermann“ (Zertifikat ZZPT00700034,
Lagern und Behandeln, Abfallschlüssel 160106) führt, Stand des Registerabrufs 11.08.2026.

**Ankauf:** tagesaktuelle Liste, Stand der Seite „letzte Preisänderung 27.10.2026“ (€/kg): Schredder-Schrott
0,08; Mischschrott leicht 0,14 / schwer 0,19; Kupfer-Kabel 3,00; Kupfer Raffinade (max. 5 %) 8,30; Kupfer
Millberry 9,60; Altzink rein 1,40; Altblei rein 1,00; Messing rein 5,00; Alu-Schrott mit Anhaftungen
(max. 5 %) 0,90; Aluminium alt/neu rein 1,30; V2A Edelstahl rein 0,65; Altpapier 0,09 (ab 100 kg 0,10).
Gültigkeit laut Betreiber: übliche Klein- und Standardmengen bei Anlieferung am Standort; größere Mengen und
Gewerbekunden erhalten individuelle Ankaufkonditionen. **Verkauf:** keine Verkaufspreise. **Gebühren:**
Containerdienst und Demontage ohne Tarif; eine Abholung von Schrott wird nicht angeboten, Anlieferung am
Standort dagegen ausdrücklich; Barzahlung oder Sofortüberweisung sind Zahlungsarten, keine Zuschläge.

**Registerlage (offen):** Im Gemeinsamen Registerportal der Länder ist weder „MHG Recycling“ noch
HRB 37461 beim Amtsgericht Stendal auffindbar (Positivkontrollen belegen die Suchmechanik) — ein
Negativbefund ohne Nichtexistenzbeweis; bestätigt ist die Steuernummer (VIES) und durch Registerdaten-
Dienstleister die Registernummer. Im amtlichen Fachbetrieberegister steht an dieser Adresse weiterhin der
Vorgänger; MHG hat **keine eigene Zertifikatsnummer**. **GEO:** für die Hausnummer 37 liefert
OSM/Nominatim keinen adressgenauen Punkt, deshalb wird keine Koordinate gesetzt; der Legacy-Datensatz trägt
dieselbe Anschrift und eine zurückgezogene Koordinate (`clear_fields: 'geo'`).

## Timeline

### Recherche 28.10.2026

- [Recherche 28.10.2026: **Eigenes Dossier angelegt (Entwurf).** Die Betreiberseite
  (Startseite, Impressum, Kontakt, Container-Anfrageformular, Datenschutzerklärung) wurde vollständig
  gelesen; das Sitemap-Register (Stand 2026-10-27) umfasst sechs Seiten. Impressum: MHG Recycling GmbH,
  Taubenstraße 37, 06847 Dessau-Roßlau, „Eingetragen im Handelsregister“, Registergericht Amtsgericht
  Stendal, Registernummer HRB 37461, vertreten durch Geschäftsführer Nicolas Gaudefroy,
  USt-IdNr. DE462718707, Telefon +49 340 5210947, E-Mail info@mhg-recycling.de. Die Startseite nennt
  Öffnungszeiten Mo–Fr 07:00–16:00 Uhr, Sa/So geschlossen, Barzahlung oder Sofortüberweisung, die
  Übernahme des Entsorgungsfachbetriebs von Horst Habermann zum 01.08.2026 und den Standort Taubenstraße 37
  in Dessau. Quellen: https://www.mhg-recycling.de/ ; https://www.mhg-recycling.de/impressum ;
  https://www.mhg-recycling.de/kontakt ; https://www.mhg-recycling.de/container-anfragen-formular ;
  https://www.mhg-recycling.de/sitemap.xml ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Preisliste live, Stand 27.10.2026.** Die Preisseite ist die Startseite
  (Navigationspunkt „Preise“ verweist auf den Anker der Startseite; keine eigene Preisseite, kein PDF).
  Seitenangabe „letzte Preisänderung 27.10.2026“. ANKAUF (€/kg, „übliche Klein- und Standardmengen bei
  Anlieferung an unserem Standort in Dessau“, größere Mengen und Gewerbekunden individuell): Schredder-Schrott
  0,08; Mischschrott leicht 0,14; Mischschrott schwer 0,19; Kupfer-Kabel 3,00; Kupfer Raffinade (max. 5 %)
  8,30; Kupfer Millberry 9,60; Altzink (rein) 1,40; Altblei (rein) 1,00; Messing (rein) 5,00; Alu-Schrott mit
  Anhaftungen (max. 5 %) 0,90; Aluminium alt/neu (rein) 1,30; V2A Edelstahl (rein) 0,65; Altpapier 0,09 und
  „Ab 100 kg zahlen wir 0,10 € pro kg“. Gegenüber der am 20.10.2026 dokumentierten Liste ist **nur V2A von
  0,70 auf 0,65 €/kg gesenkt**; alle übrigen Werte sind unverändert. VERKAUF: keine Verkaufspreise.
  GEBÜHREN: Containerdienst und Demontage ohne Tarif; keine Abholung von Schrott beim Kunden angeboten;
  Barzahlung/Sofortüberweisung sind Zahlungsarten, keine Preiszuschläge. Quelle:
  https://www.mhg-recycling.de/ ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Containergrößen belegt, keine Tarife.** Das Anfrageformular
  „Container anfragen“ fragt „Anliegen“ (Container anfragen / Containertausch), „Größe“ (**7–10 m³
  Absetzer, 20 m³ Abroller, 40 m³ Abroller**), „Sorte“ (Mischschrott / Baumischabfall / Andere), Wunschtermin
  und den Stellort des Containers ab; **Preise werden nicht genannt**. Die Auftragsprämisse „Container
  3–40 m³“ ist damit zu ersetzen. Quellen: https://www.mhg-recycling.de/container-anfragen-formular ;
  https://www.mhg-recycling.de/ ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Identität zweifach belegt, Registereintrag nicht auffindbar.** Die USt-IdNr.
  DE462718707 wird vom EU-VIES-Dienst als gültig ausgewiesen (`isValid: true`, Abruf 28.10.2026 05:33 UTC;
  Positivkontrolle DE276868959 ebenfalls VALID); VIES legt für Deutschland Namen und Anschrift nicht offen.
  Im Gemeinsamen Registerportal der Länder liefern die Suchen „MHG Recycling“ (mit exaktem Firmennamen),
  „MHG“ am Amtsgericht Stendal, HRB 37461 am Amtsgericht Stendal und HRB 37461 gerichtsübergreifend jeweils
  **0 Treffer**; Positivkontrollen („Vetter“ am Amtsgericht Darmstadt, HRB 209429 am Amtsgericht Stendal,
  HRB 60382 am Amtsgericht Darmstadt) liefern Treffer, die Suche funktioniert also. **Negativbefund, kein
  Nichtexistenzbeweis** — eine 2026 gegründte GmbH ohne veröffentlichte Registerunterlagen erscheint in der
  Portalsuche nicht. Handelsregisterdaten-Dienstleister (Creditreform, Companyhouse, Implisense) nennen
  dagegen HRB 37461 Amtsgericht Stendal, Gründung 2026 und „wirtschaftsaktiv“; diese Seiten waren am
  28.10.2026 technisch blockiert, die Angaben stammen aus dem Suchindex. Quellen:
  https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/462718707 ;
  https://www.handelsregister.de/rp_web/normalesuche/welcome.xhtml ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Keine eigene Zertifikatsnummer im amtlichen Fachbetrieberegister.** Die
  Ortsseite Dessau-Roßlau führt vier Betriebe: G. Schönemann Entsorgung GmbH (Daheimstraße 5), Schima GmbH &
  Co. KG Umweltdienste Dessau (Neuer Acker 21), TSR Deutschland Niederlassung Dessau-Roßlau
  (Hünefeldstraße 3c) und **„Transporte und Altmetallhandel Horst Habermann“ (Taubenstraße 37, Lagern und
  Behandeln, Zertifikat ZZPT00700034, Abfallschlüssel 160106)**. **MHG Recycling GmbH ist dort nicht unter
  eigenem Namen eingetragen**; der Auszug (Stand 11.08.2026) liegt zehn Tage nach dem von MHG genannten
  Übernahmedatum 01.08.2026. Für reine Metallhändler ohne gefährliche Abfälle ist ohnehin keine
  Efb-Anzeige erforderlich, der Negativbefund beweist kein Nichtbestehen. Quellen:
  https://schrottregister.pages.dev/ort-dessau-rosslau ;
  https://schrottregister.pages.dev/betrieb-transporte-und-altmetallhandel-horst-habermann-dessau-rosslau ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Betreiberkette und Legacy-Datensatz.** Die Startseite erklärt die Übernahme des
  Entsorgungsfachbetriebs „nach EfvB Horst Habermann“ zum 01.08.2026 (gezeichnet Nicolas Gaudefroy); die
  amtliche Registerführung unter dem Namen Habermann an derselben Hausnummer stützt den **Standort** und die
  **Tätigkeit**, nicht die gesellschaftsrechtliche Kontinuität und nicht die Fortgeltung der Efb-Erlaubnis
  auf MHG. Der Alt-Datensatz `st-dessau-rolau-horst-habermann-transport-und-altmetallh` bleibt unverändert
  bestehen (kein Merge, keine Löschung, Slug und Name bleiben); er trägt die Registerhistorie mit
  ZZPT00700034, dieses Dossier trägt den heutigen Betreiber und seine Preise. Die frühere Domain
  horst-habermann.de leitet auf eine Domain-Verkaufsseite (expireddomains.com/ED.com, „Premium domain for
  sale“), also weiterhin keine Betreiberquelle. Quellen: https://www.mhg-recycling.de/ ;
  https://schrottregister.pages.dev/ort-dessau-rosslau ; https://www.horst-habermann.de/ ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **GEO.** Für „Taubenstraße 37, 06847 Dessau-Roßlau“ liefert OSM/Nominatim nur zwei
  straßenobjektbezogene Treffer (way 25156926 „Taubenstraße, West, Dessau“ 51.8280136/12.2267451 und
  footway 217881636 51.8246403/12.2273533), **keinen adressgenauen Hausnummernpunkt**; es wird keine
  Koordinate gesetzt und keine geraten. Quelle:
  https://nominatim.openstreetmap.org/search?format=jsonv2&q=Taubenstra%C3%9Fe+37%2C+06847+Dessau-Ro%C3%9Flau&addressdetails=1 ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Offene Punkte für den Owner.** (1) Existenz und Sitz der HRB 37461 sind im
  amtlichen Portal nicht bestätigt — Klärung über einen kostenpflichtigen aktuellen Handelsregisterauszug beim
  Amtsgericht Stendal, einen Gewerbescheinweis oder eine Bestätigung des Betreibers; (2) Efb-Status nach dem
  Betreiberwechsel (Zertifikat steht weiterhin auf Habermann) — MHG um die eigene Anzeige nach § 56 KrWG
  bitten; (3) für einen Preis-Handler: der Materialkatalog enthält **keinen Altpapier-Eintrag** (Scraper
  erfinden keine Materialien) und der Slot `stahlschrott-shredder` ist auf EUR/t gepflegt, während MHG in
  EUR/kg rechnet (0,08 €/kg = 80 €/t). Quelle: read-only Produktionsabfrage
  `file:…public.db?mode=ro` + `PRAGMA query_only=ON` (materials), Abruf 28.10.2026.]
```

### D.2 `dossiers/he/he-dietzenbach-sebastian-schrotthandler.md` (Vorschlag: **eigenes Dossier — ja**, Markenkette dokumentiert)

**Begründung:** eigene Website mit eigenem Impressum und eigener Markenidentität, eigene Rufnummern und
E-Mail, **andere Öffnungszeiten** und **andere Mengenschwelle** als die Schwestermarke STALCANT → eine
Zusammenführung würde eine der beiden Eigenangaben falsch machen. Präzedenz im Repo: „Autoverwertung
Bolte“ als Marke der AIPEX Transport & Trading GmbH. **Kein Merge, keine Löschung**; das
STALCANT-Dossier und die Markenkette bleiben unverändert bestehen.

```yaml
---
slug: he-dietzenbach-sebastian-schrotthandler
name: Sebastian Schrotthandel
trader_type: schrotthaendler
state: HE
city: Dietzenbach
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://sebastian-schrotthandel.de/
website_status: aktiv
status: pruefung
description: 'Marke "Sebastian Schrotthandel" mit eigenem Auftritt und eigenem Impressum: "Anschrift Sebastian Schrotthandel, Geschäftsführer: Caldarar Sebastian Traian, Robert-Koch-Str. 9, 63128 Dietzenbach, Tel. +49 160 95471472, Tel. +49 163 0056553, E-Mail info@sebastian-schrotthandel.de, Web sebastian-schrotthandel.de, Wirtschafts-ID DE452409549". Dieselbe Wirtschafts-Identifikationsnummer steht im Impressum der Marke STALCANT (Dossier he-dietzenbach-stalcant-schrotthandler), und die dritte Marke Leeraumhelden (leeraumhelden.de) führt im Impressum ebenfalls "Sebastian Schrotthandel": ein Betrieb, ein Inhaber, drei Marken an der Hausnummer Robert-Koch-Str. 9 - kein Verzeichnis-Doppel. Leistungen: Schrottabholung (Abholung ab einem Gesamtgewicht von über 500 kg, Einsatz im Rhein-Main-Gebiet), Ankauf von Schrott und Altmetall zu tagesaktuellen Marktpreisen mit sofortiger Barauszahlung ohne numerisches Preisblatt, Containerdienst ausschließlich für Metallschrott (Zulassung: Aluminium, Kupfer, Messing, Zink, Zinn, Blei, Kabelreste, Edelstahl V2A, Aluminium-Offsetplatten; nicht zulässig unter anderem Fahrräder, Baustahl, Stahltüren, Heizkörper, Trägerreste, Werkzeuge und Metallmöbel), Demontage von Industrie und Gewerbe einschließlich Asbestsanierung und Entkernung sowie Entrümpelung; beim Ankauf werden Gasflaschen, Feuerlöscher und alte Reifen nicht angenommen. Verkauf: keine Verkaufspreisliste. Gebühren: keine Tarife veröffentlicht. Es liegt kein HRB-Eintrag vor, deshalb bleiben Kontakt- und Adressskalare außen vor.'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Recherche Dossier-Lücken 28.10.2026'
provenance_ankauf_raw: ja
provenance_origin: table
---

# Sebastian Schrotthandel

## Überblick

„Sebastian Schrotthandel“ ist eine **eigenständige Marke mit eigenem Webauftritt**
(`sebastian-schrotthandel.de`), eigenem Impressum und eigenem Leistungsversprechen. Das Impressum nennt
**Caldarar Sebastian Traian**, **Robert-Koch-Str. 9, 63128 Dietzenbach**, Tel. **+49 160 95471472** und
**+49 163 0056553**, **info@sebastian-schrotthandel.de**, **Wirtschafts-ID DE452409549** — **kein HRB, keine
USt-IdNr** (Einzelunternehmen). Die **Wirtschafts-ID ist identisch mit der im Impressum der Marke STALCANT**
(`he-dietzenbach-stalcant-schrotthandler`), und das Impressum der dritten Marke `leeraumhelden.de` führt
ebenfalls „Sebastian Schrotthandel“ als Anschrift: **ein Betrieb, ein Inhaber, drei Marken**.

**Getrennt bleiben:** die Schwestermarke **Leonard Schrotthandel** (`he-dietzenbach-leonard-schrotthandel`,
Leonard Dorel Caldarar, Tel. 0176 40067307, eigene E-Mail, keine W-IdNr) sowie **B. Mugurel
Schrotthändler** (Mugurel Braica, eigene W-IdNr DE425479280, Hausnummer 11). Die gemeinsame Webagentur
ffm.media e.K. ist **keine** Betreiberverbindung.

**Ankauf:** tagesaktuelle Preise ohne Zahlenkurs, sofortige Barauszahlung; Abholung ab einem Gesamtgewicht
von **über 500 kg**, Abholung „innerhalb eines Tages“, Anlieferung möglich; **Ausschlüsse**: Gasflaschen,
Feuerlöscher, alte Reifen. **Verkauf:** keine Verkaufspreisliste. **Gebühren:** Containerdienst (nur
Metallschrott, mit Zulassungs- und Ausschlussliste) und Demontage **ohne Tarif**; die Marke nennt
**keine** kostenlose Abholung — die „kostenlose Abholung ab 350 kg“ gilt für **STALCANT** und darf nicht
hierher übernommen werden. **Öffnungszeiten (Betreiberseite):** Mo–Fr 07:00–20:00, Sa 08:00–19:00, So
geschlossen; die Schwestermarke STALCANT nennt abweichend Mo–Sa 8:00–22:30 (Beratungszeiten).
**GEO:** Robert-Koch-Str. 9 ist eine Mehrmarken- und Mehrbetreiberadresse und keine bestätigte Annahmehof-
Adresse; keine Koordinate gesetzt.

## Timeline

### Recherche 28.10.2026

- [Recherche 28.10.2026: **Marke live, eigener Auftritt.** Die Website
  `https://sebastian-schrotthandel.de/` ist am 28.10.2026 erreichbar (HTTP 200); vollständig gelesen wurden
  Startseite, Impressum, Kontakt, Schrottabholung, Schrottankauf, Containerdienst, Demontage und
  Entrümpelung (alle HTTP 200). Impressum wörtlich: „Anschrift Sebastian Schrotthandel, Geschäftsführer:
  Caldarar Sebastian Traian, Robert-Koch-Str. 9, 63128 Dietzenbach, Tel: +49 160 95471472, Tel: +49 163
  0056553, E-Mail: info@sebastian-schrotthandel.de, Web: sebastian-schrotthandel.de, Wirtschafts-ID:
  DE452409549“; technische Wartung ffm.media e.K., Paul-Ehrlich-Str. 38, 63322 Rödermark. Kein HRB und
  keine USt-IdNr angegeben. Die im Auftrag genannte Domain **sebastian-schrotthaendel.de ist nicht im DNS
  auflösbar** (ENOTFOUND); die live Domain lautet **sebastian-schrotthandel.de**. Quellen:
  https://sebastian-schrotthandel.de/ ; https://sebastian-schrotthandel.de/impressum/ ;
  https://sebastian-schrotthandel.de/kontakt/ ; https://sebastian-schrotthandel.de/schrottabholung/ ;
  https://sebastian-schrotthandel.de/schrottankauf/ ; https://sebastian-schrotthandel.de/containerdienst/ ;
  https://sebastian-schrotthandel.de/demontage/ ; https://sebastian-schrotthandel.de/entruempelung/ ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Betreiberkette belegt (Wirtschafts-ID).** Die **W-IdNr DE452409549** steht sowohl
  im Impressum dieser Marke als auch im Impressum der Marke **STALCANT**
  (https://www.stalcant-schrotthaendler.de/impressum: Sebastian Traian Caldarar, Robert-Koch-Straße 9,
  63128 Dietzenbach, W-IdNr DE452409549, zuständige Behörde für die Reisegewerbekarte: Magistrat der
  Kreisstadt Dietzenbach, Europaplatz 1, Tel. 0163 52 55 962). Die dritte Marke **leeraumhelden.de** führt
  im Impressum „Anschrift: Sebastian Schrotthandel, Geschäftsführer: Caldarar Sebastian Traian, Robert-Koch-Str.
  9, 63128 Dietzenbach, Tel: +49 160 95471472, Tel: +49 163 0056553, E-Mail info@sebastian-schrotthandel.de,
  Web sebastian-schrotthandel.de, HRB folgt, UmsatzsteuernummerID: folgt“ und nennt dieselben Rufnummern.
  → **ein Betrieb, drei Marken** (Scrap-Handel, Entrümpelung, Haushaltsauflösung). Die Zuordnung
  **Leonard Dorel Caldarar** (Dossier he-dietzenbach-leonard-schrotthandel: eigene Rufnummer, eigene E-Mail,
  keine W-IdNr) und **Mugurel Braica** (eigene W-IdNr DE425479280, Hausnummer 11) bleiben getrennt; die
  „Caldarar-Familie“ ist damit **nicht** belegt (kein Register- oder Urkundendokument). Quellen:
  https://www.stalcant-schrotthaendler.de/impressum ; https://leeraumhelden.de/impressum/ ;
  https://sebastian-schrotthandel.de/impressum/ ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Leistungen und Konditionen der Marke.** ANKAUF: „tagesaktuelle Preise … Basierend
  auf den aktuellen Marktpreisen“, sofortige Barauszahlung, **keine Zahlenkurse**; Abholung „ab einem
  Gesamtgewicht von über 500 kg“, Abholung „innerhalb eines Tages“, Anlieferung „direkt bei uns“; nicht
  angenommen werden Gasflaschen, Feuerlöscher und alte Reifen. CONTAINERDIENST: „ausschließlich …
  Metallschrott“; zulässig Aluminium, Kupfer, Messing, Zink, Zinn, Blei, Kabelreste, Edelstahl (V2A),
  Aluminium-Offsetplatten; nicht hinein gehören Fahrräder, Baustahl, Stahltüren, alte Nähmaschinengestelle,
  Heizkörper, Trägerreste, Werkzeuge sowie Metallmöbel und -gestelle; **keine Größen, keine Tarife**.
  DEMONTAGE: Industriedemontage, Rückbau, Asbestsanierung, Entkernung, **ohne Tarif**. ENTRÜMPELUNG:
  Komplett- und Teilentrümpelung, Haushaltsauflösung; ausgeschlossen Asbest, Chemikalien, Sondermüll.
  VERKAUF: keine Verkaufspreisliste. GEBÜHREN: keine Tarife; anders als STALCANT („kostenlose Abholung bereits
  ab einer Menge von 350 kg“) nennt diese Marke **keine Kostenfreiheitszusage**. ÖFFNUNGSZEiten laut
  Betreiberseite: Mo–Fr 07:00–20:00, Sa 08:00–19:00, So geschlossen (STALCANT nennt abweichend Mo–Sa
  8:00–22:30). Quellen: https://sebastian-schrotthandel.de/schrottankauf/ ;
  https://sebastian-schrotthandel.de/schrottabholung/ ;
  https://sebastian-schrotthandel.de/containerdienst/ ; https://sebastian-schrotthandel.de/demontage/ ;
  https://sebastian-schrotthandel.de/entruempelung/ ; https://www.stalcant-schrotthaendler.de/Kontakt ;
  https://www.stalcant-schrotthaendler.de/Schrottabholung ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Registerbefunde (Negativbefunde mit Positivkontrollen).** Im Gemeinsamen
  Registerportal der Länder liefern „Caldarar“ (gerichtsübergreifend) und „Gogircea“ jeweils 0 Treffer;
  „Ciurar“ liefert 9 ausschließlich ortsfremde Fuzzy-Treffer (u. a. Karrer & Pascu GmbH, Courier Network
  Germany GmbH, Adler-Apotheke) — kein Ciurar; „Emji“ liefert mehr als 100 Treffer und ist als Suchbegriff
  unbrauchbar. Positivkontrollen („Vetter“ am Amtsgericht Darmstadt, HRB 209429 am Amtsgericht Stendal,
  HRB 60382 am Amtsgericht Darmstadt) belegen die Suchmechanik; Einzelunternehmer werden nicht ins HRA/HRB
  eingetragen, ein 0-Treffer-Befund beweist daher **kein Nichtbestehen**. Im amtlichen Fachbetrieberegister
  (Stand 11.08.2026) führt die Ortsseite Dietzenbach genau zwei Betriebe (Jakupi Transport Inh. Ramiz Jakupi,
  Gallische Straße 1, ZZFT00300487; Peter Vetter Abfallverwertung, Karl-Benz-Straße 4); weder „Sebastian“
  noch STALCANT noch die geprüften Anschriften erscheinen dort — **Negativbefund ohne Existenzbeweis**, da
  für reine Metallhändler ohne gefährliche Abfälle keine Efb-Anzeige erforderlich ist. Kommunale Gegenprobe:
  Die Stadt Dietzenbach erteilt Gewerberegister-Auskunft nur auf Anfrage (einfache Auskunft 10,00–30,50 EUR,
  Gruppenauskunft je Person 7,50–15,00 EUR, mindestens 76,50 EUR); das Gewerberegister ist kein öffentliches
  Register und es gibt keine Online-Einzelauskunft — der amtliche Gewerbenachweis bleibt ein schriftlicher
  Folgeschritt. Quellen: https://www.handelsregister.de/rp_web/normalesuche/welcome.xhtml ;
  https://schrottregister.pages.dev/ort-dietzenbach ;
  https://www.dietzenbach.de/Stadt-Rathaus/Service/Gewerbeangelegenheiten/Gewerberegister-Auskunft.php ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Eigener Verzeichniseintrag der Marke (Lead).** 11880 führt die Marke als
  „**Sebastian Schrotthändler / Schrottabholung**“ (Branche Altmetallrecycling), Robert-Koch-Str. 9,
  63128 Dietzenbach, Telefon **(0163) 0056553**, E-Mail **info@sebastian-schrotthandel.de** (in der
  Seitenausgabe maskiert, im HTML vorhanden), Website **https://sebastian-schrotthandel.de**, keine
  Öffnungszeiten, keine Bewertungen. Das Verzeichnis reproduziert damit **genau die Kontaktspur der
  Betreiberseite**; es bleibt ein Aggregator und damit nach README ein **Einzelbeleg** (keine zweite
  unabhängige Quelle), bestätigt aber Markennamen, Domain-Schreibweise und die Zuordnung der Rufnummer
  (0163) 0056553 zur Marke. Der URL-Slug des Verzeichniseintrags lautet „sebastian-schrotthaendler-…“ — die
  **verlinkte Domain** lautet dagegen korrekt „sebastian-schrotthandel.de“; die Auftragsschreibweise
  „sebastian-schrotthaendel.de“ ist **nicht im DNS auflösbar**. Quelle:
  https://www.11880.com/branchenbuch/dietzenbach/060440092B113866979/sebastian-schrotthaendler-schrottabholung.html ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Verzeichnisse nur als Leads.** Die 11880-Suche „Schrott in Dietzenbach“ kennt
  weiterhin nur Leonard und Emji-Schrott; die Einzelseite „Schrotthändler dietzenbach kostenlose
  schrottabholung Dorel Horvath“ (Robert-Koch-Str. 9, (0163) 205242, Altmetallrecycling, 2 Bewertungen)
  verlinkt auf das **inzwischen 404 liegende** sellwerk-Profil „HD Entrümpelung & Haushaltsaufloesung“ (am
  28.10.2026 erneut geprüft: HTTP 404). Robert-Koch-Str. 9 ist damit eine **Mehrmarken- und
  Mehrbetreiberadresse**; die vierte Namensfassung „Dorel Horvath“ (andere Rufnummer, totes Profil) bleibt
  ungeklärt und wird nicht mit Caldarar gleichgesetzt. Quellen: https://www.11880.com/suche/schrott/dietzenbach ;
  https://www.11880.com/branchenbuch/dietzenbach/060440092B113511644/schrotthaendler-dietzenbach-kostenlose-schrottabholung-dorel-horvath.html ;
  https://sellwerk.de/firmenprofil/hd-entruempelung--haushaltsaufloesung ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **GEO.** Für „Robert-Koch-Straße 9, 63128 Dietzenbach“ liefert OSM/Nominatim einen
  Hausnummern-Knoten (node 534547353, 50.0050838/8.78789, „Gewerbegebiete Mitte und Süd, Spessartviertel,
  Altstadt, Dietzenbach“); Photon löst auf **denselben** OSM-Knoten auf und ist damit **keine zweite
  unabhängige Quelle**. Der in Produktion gespeicherte Punkt 50.0098/8.7778 ist per Reverse-Geocoding das
  Stadtmitten-Gebäude „6, Hammannsgasse“ und liegt **892 m** von der belegten Anschrift entfernt — er ist
  zurückzuziehen; **keine Ersatzkoordinate** wird gesetzt, weil die Adresse als Annahmehof nicht bestätigt
  ist. Quelle:
  https://nominatim.openstreetmap.org/search?format=jsonv2&q=Robert-Koch-Stra%C3%9Fe+9%2C+63128+Dietzenbach&addressdetails=1 ;
  https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.0098&lon=8.7778&zoom=18&addressdetails=1 ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Keine Frontmatter-Füllungen außer Website/Status.** Einzelunternehmer ohne
  HRB-Eintrag, Aggregatoren sind nur Leads, die Anschrift ist eine Mehrmarkenadresse und nicht als
  Annahmehof bestätigt → Straße, PLZ, Telefon, E-Mail und Öffnungszeiten bleiben leer, `status: pruefung`
  und `website_status: aktiv` bleiben unverändert, keine GEO. Die Marken- und Betreiberkette ist in den
  Dossiers `he-dietzenbach-stalcant-schrotthandler` und `he-dietzenbach-leonard-schrotthandel`
  gegenseitig zu referenzieren (kein Merge). Quellen: README.md, Quellenhierarchie und
  Betreiberkettenregeln; Abruf 28.10.2026.]
```

### D.3 `dossiers/he/he-dietzenbach-emji-schrott.md` (Vorschlag: **eigenes Dossier — ja**, nur als Lead)

**Begründung:** „Emji-Schrott“ ist ein **eigenständiger Namensbefund an derselben Hausnummer wie
Cristofor** mit eigener Rufnummer; es gibt weder ein Impressum noch einen Registereintrag noch eine zweite
Quelle. Eine Alias-Kette zu Cristofor wäre **ohne Beleg** und würde zwei Namen mit zwei Rufnummern
zusammenführen. Ein eigener Datensatz mit `status: pruefung` **ohne Füllungen** verhindert den Verlust des
Befunds und lässt die Entscheidung offen (Klärung über Gewerberegister-Auskunft oder ein gefundenes
Impressum).

```yaml
---
slug: he-dietzenbach-emji-schrott
name: Emji-Schrott
trader_type: schrotthaendler
state: HE
city: Dietzenbach
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Namensbefund "Emji-Schrott Abholung /Schrotthändler" unter dem Namen Gogircea Alexandru an Marktheidenfelder Weg 2, 63128 Dietzenbach, Mobil (0178) 1997566. Belegt ist dieser Name und diese Rufnummer ausschliesslich durch einen Branchenverzeichniseintrag (11880), der als Lead geführt wird und keine Website, keine E-Mail-Adresse, keine Öffnungszeiten und keine Leistungsbeschreibung enthält; dieselben Daten erscheinen in weiteren Verzeichnissen, die denselben Datensatz spiegeln und deshalb keine zweite Quelle sind. Die Hausnummer teilt sich mit dem Schwesterdossier he-dietzenbach-cristofor-schrotthandel (Cristofor Ciurar, +49 163 9392817, eigene Website) - zwei verschiedene Namen und Rufnummern an einer Anschrift, zwei Betriebe oder zwei Marken eines Betriebes ist ohne Impressum oder Registerbeleg zu "Emji-Schrott" nicht entscheidbar; eine Zusammenführung mit Cristofor wird nicht behauptet. Ankauf: keine Preise belegt. Verkauf: keine Preise belegt. Gebühren: keine Tarife belegt.'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Recherche Dossier-Lücken 28.10.2026'
provenance_ankauf_raw: unbekannt
provenance_origin: table
---

# Emji-Schrott

## Überblick

„**Emji-Schrott Abholung / Schrotthändler**“ unter dem Namen **Gogircea Alexandru** ist ein
**unaufgeklärter Namensbefund an der Hausnummer Marktheidenfelder Weg 2, 63128 Dietzenbach**. Einzige
Quelle ist ein Branchenverzeichniseintrag (11880) mit der Mobilnummer **(0178) 1997566** und der Branche
„Recycling“; **ohne Website, ohne E-Mail, ohne Öffnungszeiten, ohne Leistungsbeschreibung, ohne
Impressum**. Dieselben Daten erscheinen in weiteren Verzeichnissen (Das Telefonbuch, stadtbranchenbuch,
oeffnungszeitenbuch), die denselben Datensatz spiegeln — **das ist keine zweite Quelle**.

Die Anschrift teilt sich mit **Cristofor Schrotthandel** (`he-dietzenbach-cristofor-schrotthandel`,
Cristofor Ciurar, +49 163 9392817, cristofor-schrotthandel.de): **zwei verschiedene Namen, zwei
verschiedene Rufnummern, zwei verschiedene Belegqualitäten**. Ob es sich um zwei Betriebe oder um zwei
Marken eines Betriebes handelt, ist **nur über ein Impressum oder einen Registerbeleg zu „Emji-Schrott“**
entscheidbar — beides existiert nicht. Eine Alias-Kette wird deshalb **nicht** behauptet.

**Ankauf / Verkauf / Gebühren:** keine Preise, keine Verkaufsliste und keine Tarife belegt.
**GEO:** keine Koordinate; die Hausnummer stammt aus einem Aggregator-Lead, und die in Produktion
gespeicherte Punkt 50.0098/8.7778 liegt 798 m daneben.

## Timeline

### Recherche 28.10.2026

- [Recherche 28.10.2026: **Nur ein Verzeichnisbefund (Lead).** Die 11880-Einzelseite
  „Emji-Schrott Abholung /Schrotthändler Gogircea Alexandru“ wurde am 28.10.2026 direkt abgerufen
  (HTTP 200): Branche Recycling, Marktheidenfelder Weg 2, 63128 Dietzenbach, Mobil (0178) 1997566, keine
  Website, keine E-Mail-Adresse, „keine Öffnungszeiten“, keine Bewertungen. Dieselben Daten stehen in
  Das Telefonbuch, stadtbranchenbuch.com und oeffnungszeitenbuch.de — dieselbe Quelle in mehreren Spiegeln,
  **keine Bestätigung**. Eine Suche nach „Emji-Schrott“ findet keine eigenständige Website, kein Impressum
  und keine Registerquelle. Quelle:
  https://www.11880.com/branchenbuch/dietzenbach/060444309B113012766/emji-schrott-abholung-schrotthaendler-gogircea-alexandru.html ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Abgrenzung zu Cristofor.** Am 28.10.2026 bestätigt das Impressum von
  cristofor-schrotthandel.de „Cristofor Ciurar, Marktheidenfelder Weg 2, 63128 Dietzenbach, Tel: +49 163
  9392817, E-Mail info@cristofor-schrotthandel.de, HRB: folgt, UmsatzsteuerID: folgt“ (technische Wartung
  ffm.media e.K., Senfelderstraße 1F, 63110 Rodgau). Damit stehen an derselben Hausnummer **zwei
  verschiedene Namen mit zwei verschiedenen Rufnummern**; die gemeinsame Webagentur ist keine
  Betreiberverbindung. Ob zwei Betriebe oder zwei Marken, ist ohne Impressum oder Registerbeleg zu
  „Emji-Schrott“ nicht entscheidbar — **keine Zusammenführung behauptet**. Quelle:
  https://cristofor-schrotthandel.de/impressum/ ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Registerbefunde (Negativbefunde).** Im Gemeinsamen Registerportal der Länder
  liefert „Gogircea“ **0 Treffer**, „Ciurar“ 9 ortsfremde Fuzzy-Treffer, „Emji“ mehr als 100 Treffer
  (Suchbegriff unbrauchbar); Positivkontrollen belegen die Suchmechanik. Einzelunternehmer werden nicht
  ins HRA/HRB eingetragen — ein 0-Treffer-Befund beweist **kein Nichtbestehen**. Das amtliche
  Fachbetrieberegister (Stand 11.08.2026) führt für 63128 nur Jakupi Transport Inh. Ramiz Jakupi (Gallische
  Straße 1) und Peter Vetter Abfallverwertung (Karl-Benz-Straße 4); Marktheidenfelder Weg 2 erscheint nicht —
  **Negativbefund ohne Existenzbeweis**. Amtlicher Klärweg bleibt die Gewerberegister-Auskunft der Stadt
  Dietzenbach (schriftlich, 10,00–30,50 EUR für die einfache Auskunft). Quellen:
  https://www.handelsregister.de/rp_web/normalesuche/welcome.xhtml ;
  https://schrottregister.pages.dev/ort-dietzenbach ;
  https://www.dietzenbach.de/Stadt-Rathaus/Service/Gewerbeangelegenheiten/Gewerberegister-Auskunft.php ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **GEO.** Nominatim löst „Marktheidenfelder Weg 2, 63128 Dietzenbach“ auf ein
  Adress-Way (way 40922223, 50.0142103/8.7866023). Da die Hausnummer nur aus einem Aggregator-Lead stammt,
  wird **keine Koordinate** gesetzt; der in Produktion gespeicherte Punkt 50.0098/8.7778 (Hammannsgasse 6)
  liegt 798 m daneben und ist zurückzuziehen. Quelle:
  https://nominatim.openstreetmap.org/search?format=jsonv2&q=Marktheidenfelder+Weg+2%2C+63128+Dietzenbach&addressdetails=1 ;
  Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Keine Frontmatter-Füllungen.** Aggregatoren sind nur Leads; Betreiberidentität,
  Tätigkeit und öffentliche Annahme sind nicht belegt → Straße, PLZ, Telefon, E-Mail, Öffnungszeiten und
  Website bleiben leer, `status: pruefung` und `website_status: unbekannt` bleiben unverändert, keine GEO.
  Quellen: README.md, Quellenhierarchie; Abruf 28.10.2026.]
```

---

## Teil E — Empfehlungen an den Owner

### E.1 Umgang mit den Alt-Datensätzen (keine Löschung, keine Zusammenführung ohne Beleg)

| Alt-Datensatz | Empfehlung | Was **nicht** zu tun ist |
|---|---|---|
| `st-dessau-rolau-horst-habermann-transport-und-altmetallh` | **Unverändert bestehen lassen.** Slug, Name „Horst Habermann Transport und Altmetallhandel“, `status: pruefung`, `street`/`postcode` (Taubenstraße 37 / 06847, durch das amtliche Register gedeckt) und `clear_fields: 'geo'` bleiben. **Ergänzung:** ein Timeline-Querverweis auf das neue Dossier `st-dessau-rolau-mhg-recycling` und der Hinweis, dass die Zertifikatsnummer ZZPT00700034 der historischen Zeile gehört. | **Keine** Löschung, **keine** Umbenennung, **kein** Merge mit MHG, **keine** Übernahme der MHG-Preise in die Legacy-Zeile, **keine** Übernahme der MHG-Kontakte in die Legacy-Zeile. |
| `he-dietzenbach-stalcant-schrotthandler` | Unverändert bestehen lassen; **eine** Ergänzung: Querverweis auf das neue Marken-Dossier `he-dietzenbach-sebastian-schrotthandler`, Benennung der Markenkette (STALCANT / Sebastian Schrotthandel / Leeraumhelden — **ein Betrieb, W-IdNr DE452409549**) und **Trennung der Konditionen** (350 kg kostenlos = STALCANT; 500 kg ohne Kostenfreiheitszusage = Sebastian Schrotthandel). | **Keine** Löschung, **kein** Merge der drei Marken zu einer Zeile, **keine** Angleichung der Öffnungszeiten oder Kontaktdaten. |
| `he-dietzenbach-cristofor-schrotthandel` | Unverändert bestehen lassen; Querverweis auf `he-dietzenbach-emji-schrott` mit dem Hinweis „zweiter Name an derselben Hausnummer, keine Belegverbindung“. | **Keine** Zusammenführung mit Emji (anderer Name, andere Rufnummer, kein Beleg), **keine** Löschung. |
| `he-dietzenbach-leonard-schrotthandel`, `he-dietzenbach-b-mugurel-schrotthandler` und die übrigen Fragmentzeilen | Unverändert bestehen lassen; sie sind eigenständige Personen/Betriebe (eigene W-IdNr bzw. eigene Rufnummer, eigene Hausnummer bei Mugurel). | **Keine** „Caldarar-Familie“-Zusammenführung (kein Register- oder Urkundendokument), **keine** Löschung. |

**Generell:** Kein Datensatz wird gelöscht, kein Slug geändert, kein Name umbenannt. Ein Merge ist nur
zulässig, wenn ein **Beleg** die Gleichheit von Rechtsträger **und** Standort trägt — bei
„Horst Habermann → MHG Recycling“ trägt der Beleg nur die **Standortkontinuität** (amtliches Register +
Betreiberaussage), nicht die Rechtsträgeridentität; bei den Dietzenbacher Marken trägt der Beleg die
**Personengleichheit** (identische W-IdNr), nicht die Gleichheit von Marke und Rechtsträger (Einzelunternehmer
ohne HRB).

### E.2 GEO-Punkt für die Dietzenbach-Zeilen (separate Entscheidung, hier nur empfohlen)

Die in Produktion gespeicherte Koordinate **50.0098 / 8.7778** ist per Reverse-Geocoding das
Stadtmitten-Gebäude **„6, Hammannsgasse“** und liegt **892 m** von Robert-Koch-Str. 9 bzw. **798 m** von
Marktheidenfelder Weg 2 entfernt. Sie bezeichnet **keinen** der Clusterbetriebe. Für die betroffenen
Zeilen (`he-dietzenbach-stalcant-schrotthandler`, `he-dietzenbach-leonard-schrotthandel`,
`he-dietzenbach-cristofor-schrotthandel`, `he-dietzenbach-63128-a-w-metallhandel`,
`he-dietzenbach-b-samuel-schrottabholung`, `he-dietzenbach-miky-schrotthandler`,
`he-dietzenbach-schrotthandler-dietzenbach-s-o`, `he-dietzenbach-schrotthandler-sabri`) empfiehlt der
Bericht eine **Rücknahme über `clear_fields: 'geo'` ohne Ersatz** — analog zu
`he-dietzenbach-b-mugurel-schrotthandler`, wo das bereits geschehen ist. **Nicht** gesetzt werden darf
stattdessen ein OSM-Hausnummernknoten: Für Robert-Koch-Str. 9 und Marktheidenfelder Weg 2 existiert jeweils
**nur eine** Geodatenquelle (OSM; Photon liefert denselben Knoten), und die Adressen sind zudem
Mehrmarken-/Mehrbetreiberadressen ohne bestätigten Annahmehof.

### E.3 Konkrete nächste Schritte (Reihenfolge)

1. **Owner-Entscheidung:** je ein eigener Datensatz für MHG, „Sebastian Schrotthandel“ und „Emji-Schrott“
   (Entwürfe D.1–D.3) — oder Variante „Alias-Kette“ für die Sebastian-Marke (Abschnitt B.6). Danach
   `cargo test -p schrott-mcp-ingestion seed --locked --offline` und erst danach ein Commit.
2. **Querverweise** in den drei Alt-Dossiers ergänzen (Abschnitt E.1) — ohne Inhalts- oder Slug-Änderung.
3. **GEO-Rücknahme** der acht Dietzenbach-Zeilen mit dem Stadtmitten-Punkt (E.2), ohne Ersatzkoordinate.
4. **Amtliche Nachprüfung MHG** (Kosten beim Owner): aktueller Handelsregisterauszug HRB 37461 beim Amtsgericht
   Stendal; Gewerbescheinweis; und eine schriftliche Frage an MHG nach der **eigenen Efb-Anzeige/
   Zertifikatsnummer** nach § 56 KrWG. Bis dahin `status: pruefung` für den MHG-Datensatz.
5. **Gewerberegister-Auskunft Dietzenbach** (10,00–30,50 EUR für die einfache Auskunft je Person) für
   „Caldarar“, „Gogircea Alexandru“, „Dorel Horvath“ — das ist der einzige amtliche Weg, um die vierte
   Namensfassung an Robert-Koch-Str. 9 und „Emji-Schrott“ zu klären.
6. **Optional: Preis-Handler für MHG.** Die MHG-Seite ist ein guter Handler-Kandidat: tagesaktuelle Liste mit
   **Seiten-Datum** („letzte Preisänderung 27.10.2026“), maschinenlesbare Preise in €/kg, keine
   Zugangsbarriere. Zwei Voraussetzungen: (a) ein **Altpapier-Eintrag** im Materialkatalog fehlt (Scraper
   erfinden keine Materialien — Katalogeintrag zuerst beschließen), (b) der Slot `stahlschrott-shredder`
   steht auf EUR/t, MHG rechnet in EUR/kg (0,08 €/kg = 80 €/t) — die Normalisierungsregel festlegen.

### E.4 Was dieser Bericht ausdrücklich **nicht** behauptet

* **Keine Nichtexistenz** aus Register-Negativbefunden (MHG/HRB 37461, „Caldarar“, „Gogircea“, „Emji“,
  „Cristofor“): Einzelunternehmer werden nicht ins HRA/HRB eingetragen, und eine 2026 gegründte GmbH ohne
  veröffentlichte Registerunterlagen erscheint in der Portalsuche nicht.
* **Keine Koordinate** ohne zwei unabhängige Quellen; für keinen der drei Datensätze wird eine Koordinate
  vorgeschlagen.
* **Keine Ankaufs-/Verkaufs-/Gebühren-Vermischung**: Ankauf (Zahlenkurs), Verkauf (Preisliste) und Gebühren
  (Tarif) sind je Datensatz getrennt ausgewiesen; „kostenlose Abholung ab 350 kg“ gehört zu **STALCANT**,
  „Abholung über 500 kg“ zur Marke **Sebastian Schrotthandel**, „Barzahlung oder Sofortüberweisung“ ist eine
  Zahlungsart der **MHG**, kein Preis.
* **Keine erfundenen Werte**: alle Zahlen stammen aus direkt abgerufenen Betreiber- oder Registerseiten bzw.
  aus dem VIES-Dienst; die drei Handelsregisterdaten-Dienstleister sind als **Suchindex-Auszüge**
  gekennzeichnet (Direktabruf 403/CAPTCHA).

---

## Anhang — Quellenverzeichnis (alle Abrufe 28.10.2026)

**Betreiberquellen**
* `https://www.mhg-recycling.de/` (Startseite mit Preisliste, Stand „letzte Preisänderung 27.10.2026“)
* `https://www.mhg-recycling.de/impressum` · `/kontakt` · `/container-anfragen-formular` · `/datenschutzerklarung` · `/sitemap.xml`
* `https://sebastian-schrotthandel.de/` · `/impressum/` · `/kontakt/` · `/schrottabholung/` · `/schrottankauf/` · `/containerdienst/` · `/demontage/` · `/entruempelung/`
* `https://leeraumhelden.de/` · `https://leeraumhelden.de/impressum/`
* `https://www.stalcant-schrotthaendler.de/impressum` · `/Kontakt` · `/Schrottabholung` (Direktabruf per
  WebFetch; direkter `curl` erhält 403)
* `https://cristofor-schrotthandel.de/impressum/`
* `https://www.horst-habermann.de/` (→ `expireddomains.com`, Domain-Verkaufsseite)

**Amtliche Register / Behörden**
* `https://www.handelsregister.de/rp_web/normalesuche/welcome.xhtml` (Namens-, Registernummern- und
  Sitzsuchen mit Positivkontrollen)
* `https://schrottregister.pages.dev/ort-dessau-rosslau` · `/ort-dietzenbach` ·
  `/betrieb-transporte-und-altmetallhandel-horst-habermann-dessau-rosslau` · `/sitemap.xml` (6.712 URLs,
  **kein** Treffer „mhg“)
* `https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/462718707` (VIES: `isValid: true`)
* `https://www.dietzenbach.de/Stadt-Rathaus/Service/Gewerbeangelegenheiten/Gewerberegister-Auskunft.php`
  (Gebühren 10,00–30,50 EUR / 7,50–15,00 EUR je Person, mindestens 76,50 EUR)

**GEO**
* `https://nominatim.openstreetmap.org/search?format=jsonv2&q=…` für Taubenstraße 37 Dessau (nur
  Straßenobjekte), Robert-Koch-Straße 9 Dietzenbach (node 534547353), Marktheidenfelder Weg 2 Dietzenbach
  (way 40922223); `…/reverse?lat=50.0098&lon=8.7778` (Hammannsgasse 6); Photon/komoot (derselbe OSM-Knoten)

**Verzeichnisse (nur Leads)**
* `https://www.11880.com/branchenbuch/dietzenbach/060444309B113012766/emji-schrott-abholung-schrotthaendler-gogircea-alexandru.html`
* `https://www.11880.com/branchenbuch/dietzenbach/060440092B113866979/sebastian-schrotthaendler-schrottabholung.html` (Eintrag der Marke „Sebastian Schrotthändler / Schrottabholung“)
* `https://www.11880.com/branchenbuch/dietzenbach/060440092B113511644/schrotthaendler-dietzenbach-kostenlose-schrottabholung-dorel-horvath.html`
* `https://www.11880.com/suche/schrott/dietzenbach` · `https://sellwerk.de/firmenprofil/hd-entruempelung--haushaltsaufloesung` (404)
* `https://www.dastelefonbuch.de/Branchen/Recyclingbetriebe/Dietzenbach` · `https://lokaleschrottplatz.de/search/stahl-eisen-schrotthaendler-dietzenbach/` · `https://www.schrottplatz.org/`
* Suchindex-Auszüge (Direktabruf 403/CAPTCHA): `https://firmeneintrag.creditreform.de/06847/3290275598/MHG_RECYCLING_GMBH` · `https://www.companyhouse.de/MHG-Recycling-GmbH-Dessau-Rosslau` · `https://implisense.com/en/companies/mhg-recycling-gmbh-dessau-rosslau-ot-dessau-DEZG3KBZ5K22`

**Produktionsabfragen (read-only)**
* `file:/var/lib/schrott-mcp/public.db?mode=ro` + `PRAGMA query_only=ON`: `traders` (Zustand der elf
  Dietzenbach-Zeilen und der Zeile `st-dessau-rolau-horst-habermann-transport-und-altmetallh`),
  `materials` (kein Altpapier-Eintrag), `prices`/`current_prices` (keine Preise für MHG/Habermann).
  **Keine Schreibzugriffe, keine Änderung an Produktionsdaten.**

**Nicht durchgeführt (bewusst)**
* Keine Dossier-Datei angelegt oder geändert; kein Commit, kein Push, kein Deploy.
* Kein VIES-Nachweis für die USt-IdNr der Dietzenbach-Marken (sie geben keine an) und keine Anfrage beim
  Magistrat Dietzenbach (nur Verwaltungsleistung, kein Online-Verfahren).
