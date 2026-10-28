# Dossier-Lücken-Recherche vom 28.10.2026 (Teil B): „Umschlagplatz Parchim“ (GKM, ZZMT00400134) und „AC Containerdienst Eltville“ / „Goldankauf Reingold Eltville“ (Marktstr. 6, Eltville)

**Auftrag:** Owner-Check-in 28.10.2026. Recherche zu **zwei fehlenden Datensätzen** und **Entwurf**
der Dossiers. **Keine Dossier-Datei im Repo wurde angelegt oder geändert**; dieser Bericht ist die
einzige Lieferung. Kein Commit, kein Push, kein Deploy, keine Änderung an Produktionsdatenbanken
(nur `file:…?mode=ro` + `PRAGMA query_only=ON`).

**Methodik:** Betreiberseiten direkt abgerufen (Startseite, Impressum, Standorte, Leistungsseiten,
Sitemap, Wayback/CDX), amtliche bzw. behördliche Quellen (Fachbetrieberegister-Auszug 11.08.2026
über `schrottregister.pages.dev`, Abfallratgeber 2026 der Abfallwirtschaft Ludwigslust-Parchim AöR
auf `alp-lup.de`, EU-VIES), Registerdaten-Dienstleister (NorthData, Creditreform, Implisense —
**keine** amtlichen Register), Verzeichnisse (11880, Gelbe Seiten, Das Örtliche, GoLocal,
branchen-info.net, recyclinghof-wertstoffhof.de) **ausschließlich als Leads**, OSM/Nominatim.
Jede Belegaussage trägt Quelle und Abrufdatum (28.10.2026).

---

## Korrekturen an der Auftragsprämisse (sie tragen die Ergebnisse)

1. **Postleitzahl Parchim: 19370, nicht 17192.** Fünf unabhängige Quellen nennen für
   *Zum Strauchberg 8* die PLZ **19370**: Betreiber-Standortseite (`gkm-mv.de/standorte/`),
   Fachbetrieberegister (Auszug 11.08.2026), Abfallratgeber 2026 der ALP AöR, Das Örtliche und
   OSM/Nominatim („Zum Strauchberg, Parchim, …, 19370“). Die PLZ-Verzeichnisse stützen das:
   **19370 = Parchim** (1 Ort, Landkreis Ludwigslust-Parchim, 17.490 Einwohner), während
   **17192 gar nicht zu Parchim gehört**, sondern zu **Groß Dratow** u. a. im Landkreis Müritz
   (Vorwahl 03991 statt 03871). → **Die Auftragsprämisse ist hier falsch; im Entwurf steht 19370.**
   (Der Repo-Seed „Parchim 19370“ ist insoweit korrekt; OSM ordnet auch Juri-Gagarin-Ring 10 dem
   Bereich 19370 zu.)
2. **Postleitzahl Eltville: 65343, nicht 65326.** 65326 ist **Aarbergen** (Rheingau-Taunus-Kreis);
   Eltville hat die PLZ-Spanne 65343–65347. Vier Verzeichnisse nennen für *Marktstr. 6*
   einhellig **65343 Eltville am Rhein**. → **Auftragsprämisse korrigiert.**
3. **„Zweitstandort der AC Containerdienst UG in Eltville“ ist nicht belegt und nach dem
   Rechercheergebnis unwahrscheinlich.** Die Adresse *Marktstr. 6, Eltville* ist in allen
   Verzeichnissen ein **Goldankauf („Goldankauf Reingold Eltville“)**, kein Containerdienst und
   kein Schrotthandel. Die Umkreissuchen von 11880 und Gelbe Seiten für „AC Containerdienst“ in
   Eltville liefern **jeweils nur den Freigericht-Sitz der AC Containerdienst UG** (72,9 km bzw.
   72,1 km entfernt). → **Es wird kein AC-Containerdienst-Datensatz für Eltville angelegt.**
4. **Ein Schrottankauf am Umschlagplatz Parchim ist nirgends belegt** — weder auf der
   Betreiberseite (dort kommt das Wort „Schrott“ in **keiner** Seite vor), noch im Register
   (das nur Tätigkeiten und Abfallschlüssel kennt) noch in der Behördenliste (die „Annahme …
   Schrott“ nennt, nicht „Ankauf“). Belegt ist die **Annahme** von Metallabfällen (Efb-Schlüssel
   170401/170402/170405 mit „Verwerten“ an genau dieser Anschrift), **nicht** eine Vergütung.
   → **`trader_type: schrottplatz`, nicht `schrotthaendler`; `status: pruefung`.**
5. **Ein Negativbefund im Register beweist keine Nichtexistenz.** Für Eltville existiert im
   Fachbetrieberegister-Auszug gar keine Ortsseite (für Edelmetallhändler besteht keine
   Efb-Anzeigepflicht); für AC Containerdienst UG (gegründet 01.04.2026) findet sich dort kein
   Eintrag — beides ist ohne Aussagekraft, wird aber als Negativbefund mit Positivkontrolle
   dokumentiert.

## Ergebnis in Kürze

| Datensatz | Befund | Empfehlung an den Owner |
|---|---|---|
| **„Umschlagplatz Parchim“, Zum Strauchberg 8, 19370 Parchim** | Echter, betreiberidentifizierter Standort: Betreiber ist die **GKM Güstrower Kies & Mörtel GmbH** (AG Rostock HRB 3832, USt-IdNr. DE137632931 — am 28.10.2026 im EU-VIES gültig, GF Jens Eckhoff, Sitz Krakow am See). Betreiberseite führt den Standort als „**Recyclinghof | Umschlagplatz**“ (Mo–Fr 07:00–16:00, Sa nach Absprache); amtliches Fachbetrieberegister (11.08.2026) führt dort den Eintrag „Umschlagplatz Parchim“, Zertifikat **ZZMT00400134**, Tätigkeiten **Lagern, Verwerten**, Abfallschlüssel **170401/170402/170405**; Abfallratgeber 2026 der ALP AöR (Landkreis) nennt „Annahme von Bauschutt und Gewerbeabfällen, Asbest, Dämmung, Teerpappe, Altholz, **Schrott**“. **Ankauf/Kurse: nicht belegt.** | **Eigenes Dossier `mv-parchim-19370-umschlagplatz-parchim`** (Entwurf unten), `trader_type: schrottplatz`, `status: pruefung`, **ohne Koordinate** (kein adressgenauer Punkt in OSM). Der Seed-Datensatz `mv-parchim-19370-schrotthandel-parchim` bleibt **unverändert** (anderer Betreiber, andere Straße). |
| **„AC Containerdienst Eltville“, Marktstr. 6, 65343 Eltville** | Die Adresse ist **kein** Standort der AC Containerdienst UG (haftungsbeschränkt). Sie gehört zu **„Goldankauf Reingold Eltville“** (Edelmetallhandel: Gold/Silber/Platin zu Tagespreisen, Schmuckservice). AC Containerdienst hat laut Betreiber-Impressum **eine** Anschrift (Im Schwalbengrund 14, 63579 Freigericht) und nennt als Einsatzgebiet Gelnhausen, Freigericht, Wächtersbach, Alzenau; beide Umkreissuchen (11880, Gelbe Seiten) liefern für „AC Containerdienst“/Eltville nur den Freigericht-Sitz. Die Marke **Reingold** ist zwischen Mainz und Eltville durch die gemeinsame Kontaktadresse `ankauf@reingold-eltville.de` und die Markennennung belegt; **Mainz-spezifische Daten (Anschrift, Rufnummer 06131 6367556, Öffnungszeiten 10:00–18:00) dürfen nicht auf Eltville übertragen werden.** Die Betreiberseite existiert nicht mehr (Domain = IONOS-Parkplatz), der jüngste Bestätigungshinweis ist ein Verzeichnis-Eintragscheck vom 18.06.2026, in OSM steht an der Adresse eine Schneiderei. | **Kein AC-Containerdienst-Datensatz für Eltville.** Stattdessen **eigenes Dossier `he-eltville-65343-goldankauf-reingold`** (Entwurf unten) für die Adresse Marktstr. 6, `trader_type: sonstige` (wie das Mainzer Schwesterdossier), `status: pruefung`, `website_status: tot`. Im Mainzer Dossier `rp-mainz-55129-reingold` nur einen **Querverweis** ergänzen, keine Daten migrieren. |

---

# Teil A — „Umschlagplatz Parchim“, Zum Strauchberg 8, 19370 Parchim (GKM Güstrower Kies + Mörtel GmbH)

## A.1 Betreiber, Rechtsform, Registerlage

**Betreiber-Primärquelle (Impressum der Betreiberseite, direkt abgerufen am 28.10.2026,
`https://www.gkm-mv.de/common/info/impressum.html`):**

| Feld | Angabe |
|---|---|
| Firma | GKM Güstrower Kies & Mörtel GmbH |
| Firmensitz | Stellwerkswiese 2, 18292 Krakow (am See) |
| Telefon / Fax | 038457 23606 / 038457 23101 |
| E-Mail | info@gkm-mv.de |
| Website (Impressum) | www.gkm-kies-moertel.de (live: gkm-mv.de) |
| Geschäftsführer | Jens Eckhoff |
| Steuer-Nr. / USt-IdNr. | 52/215/01011 / **DE 137632931** |
| Registergericht / Nr. | **Amtsgericht Rostock, HRB 3832** |

**Unabhängige amtliche Bestätigung der Steuernummer:** Der EU-VIES-Dienst
(`https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/137632931`, Abruf 28.10.2026
09:17 UTC) liefert `isValid: true` / `userError: "VALID"` für **DE137632931**. VIES legt für
Deutschland weder Namen noch Anschrift offen (`"name": "---"`, `"address": "---"`) — bestätigt ist
die Steuernummer, nicht deren Inhaberschaft.

**Registerdaten-Dienstleister (Suchindex-Auszüge, Seiten selbst nicht abrufbar — keine amtlichen
Register):** Creditreform-Firmenauskunft („wird beim Amtsgericht 18057 Rostock unter der
Handelsregister-Nummer HRB 3832 geführt … wirtschaftsaktiv“), NorthData („Amtsgericht Rostock
HRB 3832: Krakow a. See“; Unternehmensgegenstand: „Recycling von **nichtmetallischen**
Altmaterialien und Reststoffen sowie sonstigen Sekundärrohstoffen, hier besonders das Recycling
und die Rückgewinnung von Bauschutt, sowie deren Vertrieb bzw. Einlagerung; ferner die Herstellung
und der Vertrieb von Kiesen, Sanden, Mineralgemischen und anderen Schüttgütern. Die Gesellschaft
darf auch den Hoch- und Tiefbau und sonstige Baugeschäfte betreiben …“).

**Amtliches Registerportal — technische Grenze, ausdrücklich dokumentiert:** Die Suche im
Gemeinsamen Registerportal der Länder (`handelsregister.de`, normale Suche) war am 28.10.2026
technisch nicht durchführbar (POST auf das Suchformular liefert „Fehler bei der Verarbeitung
Ihrer Anfrage“, Fehler-ID ohne Text; `mask.do`-Direktabruf HTTP 400). Es wird deshalb **kein
Register-Negativbefund** konstruiert; die HR-Angabe stützt sich auf Betreiber-Impressum +
VIES + zwei Registerdaten-Dienstleister.

**Unternehmensgeschichte (Betreiberseite, `unser-unternehmen/`):** 1991 Gründung, 1992 erste
Kiesförderung Charlottenthal, 1994 Übernahme der Kieswerke Zietlitz, Dargelütz und Dobbertin und
**Einstieg in das Recyclinggeschäft**, 2010 Deponieeröffnung Spoitgendorf, 2018
Rekultivierungspreis des Landes. Die Seite beschreibt die GKM als „zertifizierten
Entsorgungsfachbetrieb und marktnahen Komplettanbieter“ (Fußzeile: „Zertifizierung nach EfbV“).

## A.2 Was das amtliche Fachbetrieberegister führt (Auszug 11.08.2026)

Die Detailseite des Registerverzeichnisses
`https://schrottregister.pages.dev/betrieb-umschlagplatz-parchim-parchim` (Registerabruf
11.08.2026) schreibt wörtlich:

> „Umschlagplatz Parchim **ist im amtlichen Register mit der Anschrift Zum Strauchberg 8, 19370
> Parchim geführt. Hauptsitz: GKM Güstrower Kies + Mörtel GmbH.**“
> Zertifizierte Tätigkeiten: **Lagern**, **Verwerten**.
> Abfallschlüssel im Zertifikat (3): **170401** – Kupfer, Bronze, Messing; **170402** –
> Aluminium; **170405** – Eisen und Stahl.
> Zertifikatsnummer im Register: **ZZMT00400134**.
> „Das Register nennt weder Telefon noch E-Mail — und wir veröffentlichen keine.“

**Dieselbe Zertifikatsnummer ZZMT00400134 trägt zwei weitere Standorte desselben Unternehmens:**

| Registereintrag | Anschrift | Tätigkeiten | Abfallschlüssel |
|---|---|---|---|
| Umschlagplatz Parchim | Zum Strauchberg 8, 19370 Parchim | Lagern, Verwerten | 170401, 170402, 170405 |
| GKM Güstrower Kies + Mörtel GmbH | An der Deponie 1, 18276 Plaatz Spoitgendorf | Lagern, Behandeln, Verwerten, Beseitigen | 120102, 170407, 191202, 191203 |
| Vermittlung (Firmensitz) | Stellwerkswiese 2, 18292 Krakow am See | Sammeln, Befördern, Handeln, Makeln | 62 Schlüssel, u. a. 020110 Metallabfälle, 160104/160106 Altfahrzeuge |

Für Parchim ist im Registerauszug **genau ein** Betrieb geführt; ein anerkannter
Altfahrzeug-Demontagebetrieb ist dort **nicht** verzeichnet. (Die Abweichung der Adressangabe des
Sitzbetriebs — Register „An der Deponie 1, Plaatz Spoitgendorf“ vs. Betreiberseite „Glasewitzer
Straße, 18276 Spoitgendorf“ — betrifft **nicht** den Parchimer Datensatz und wird hier nur der
Vollständigkeit halber notiert.)

## A.3 Tätigkeit: was belegt ist — und was nicht

**Belegt (Annahme/Verwertung am Standort):**

* Betreiberseite, Standortseite: der Parchimer Standort ist ein **„Recyclinghof | Umschlagplatz“**.
* Betreiberseite, „Entsorgung & Deponien“: Annahme von **Sperrmüll, Grünschnitt,
  Bauschutt-/Abbruchabfällen, Sonderabfall und schadstoffbelasteten mineralischen Abfällen**;
  Verarbeitung im eANV-Verfahren für gefährliche Bauabfälle; Abtransport des Schüttguts über
  Partner; ausdrücklicher Hinweis: „Vor der Anlieferung sollten Sie sich über die genauen
  Anliefer- und Annahmebedingungen informieren und Verbindung mit uns aufnehmen“; der Betrieb
  „vermittelt … auch zugelassene Entsorgungsfachbetriebe und Containerdienste in Ihrer Nähe“.
* **Behördliche Quelle:** Der **Abfallratgeber 2026** der Abfallwirtschaft Ludwigslust-Parchim
  AöR (Landkreis Ludwigslust-Parchim; PDF-Erstellung 14.11.2025, abgerufen 28.10.2026 über
  `alp-lup.de/media/custom/3626_100_1.PDF?1765378065`, S. 52) listet:
  „**GKM GÜSTROWER KIES + MÖRTEL GmbH, Zum Strauchberg 8, 19370 Parchim, Tel. 03871 451094,
  Fax 03871 451098** — Abfallentsorgung zur Verwertung und Beseitigung, Annahme von Bauschutt
  und Gewerbeabfällen, Asbest, Dämmung, Teerpappe, Altholz, **Schrott**, Verkauf von
  Recyclingmaterial, Kies, Sand, Schotter, Natursteinen und Oberboden, Bodenannahme nach LAGA.“
  Dieselbe Angabe steht unverändert in den Ausgaben 2022 (`3626_143_1.PDF`) und 2024
  (`3626_187_1.PDF`).
* Amtliches Fachbetrieberegister: zertifizierte Tätigkeiten **Lagern und Verwerten** mit den
  Abfallschlüsseln **170401 (Kupfer/Bronze/Messing), 170402 (Aluminium), 170405 (Eisen und
  Stahl)** an genau dieser Anschrift — das sind die klassischen Schrottsorten.
* **Einschränkung der Quellenzuordnung:** In der ALP-Tabelle bezieht sich die Spalte
  „Leistungsangebot“ auf den **gesamten Eintrag GKM** (zwei Anschriftzeilen: Parchim und
  Dargelütz). Welche Fraktion *am Standort Parchim* angenommen wird, ist aus der Tabelle allein
  **nicht eindeutig** ableitbar. Die standortbezogene Schrott-Aussage für Parchim stützt sich
  deshalb auf das **Efb-Zertifikat** (170401/170402/170405 mit „Verwerten“ an exakt dieser
  Anschrift) — die ALP-Liste ist der behördliche Beleg für den **Betrieb** und für die Annahme
  von Schrott im GKM-Angebot.

**Nicht belegt (und im Entwurf ausdrücklich offengelassen):**

* **Schrottankauf / Ankaufskurse: nicht belegt.** Die Betreiberseite wurde vollständig geprüft
  (Startseite, `unser-unternehmen/`, `baustoffe/`, `baustoffe/kies-und-sand/`,
  `baustoffe/strassenbaustoffe/`, `entsorgung-und-deponien/`, `bewertung/`, `kontakt/`,
  `standorte/`, Impressum; Sitemap mit 13 URLs, davon 9 Inhalts- und 4 Infoseiten): das Wort **„Schrott“ kommt auf keiner Seite
  vor, „Ankauf“ ebenfalls nicht**. Es gibt keine Preisliste, keinen Tageskurs, keine
  Mindestmengen- oder Mengenstaffel.
* **Abholung beim Kunden: nicht angeboten** (die Seite nennt nur Annahme am Standort und
  Abtransport von Schüttgut über Partner für die eigene Baustellenlogistik).
* **Gebühren/Entsorgungstarife: keine veröffentlicht.** Die ALP-Liste verweist ausdrücklich
  darauf, sich „hinsichtlich Abgabe, Kosten etc. in Verbindung zu setzen“.
* **Verkauf** ist belegt und betrifft Recycling-Baustoffe (Kies, Sand, Schotter, Naturstein,
  Oberboden, Recyclingmaterial) — das ist **Verkauf**, nicht Ankauf, und gehört nicht in eine
  Ankaufskala.

**Weitere Verzeichnis-Leads (nur als Lead, nicht als Beleg übernommen):** Das Örtliche führt
„Recycling und Umschlagplatz Güstrower Kies und Mörtel GmbH“, Zum Strauchberg 8, 19370 Parchim,
Branche „Sand und Kies in Parchim“, 03871 451094 / Fax 03871 451098 / Mobil 0172 3814105,
info@gkm-mv.de, „gefällt 2 Personen“; recyclinghof-wertstoffhof.de (Stand Juli 2026) nennt
Mo–Fr 07:00–16:00, Sa/So geschlossen, „Angenommene Abfälle: keine geprüfte Liste“ und als weiteren
Standort desselben Betreibers **„GKM Güstrower Kies & Mörtel GmbH in Weitendorf“** (52,9 km) — ein
Standort, der auf der Betreiberseite **nicht** auftaucht (Klärfall, s. A.7).

## A.4 Anschrift, Kontakt, Öffnungszeiten, Website

| Feld | Wert | Beleg (unabhängig) |
|---|---|---|
| Anschrift | Zum Strauchberg 8, 19370 Parchim | Betreiberseite + Fachbetrieberegister + Abfallratgeber 2026 (ALP AöR) + Das Örtliche |
| Telefon | 03871 451094 | Betreiberseite + Abfallratgeber 2026 (ALP AöR) + Das Örtliche + recyclinghof-wertstoffhof.de |
| Fax | 03871 451098 | Betreiberseite + Abfallratgeber 2026 (ALP AöR) + Das Örtliche |
| E-Mail | info@gkm-mv.de | Betreiberseite (Impressum/Fußzeile) + Das Örtliche — **zentrales Unternehmenspostfach** am Sitz Krakow, kein standortspezifisches Postfach |
| Öffnungszeiten | Mo–Fr 07:00–16:00, Sa nach Absprache | Betreiberseite; recyclinghof-wertstoffhof.de nennt abweichend Sa **geschlossen** → Divergenz wird zugunsten der Betreiberseite dokumentiert |
| Website | https://www.gkm-mv.de/ (aktiv), standortbezogene Seite `gkm-mv.de/standorte/` | Betreiberseite, Sitemap-Lastmod 2026-05-12/2026-06-10 (aktuell gepflegt) |

## A.5 Geokodierung — keine Koordinate

* Nominatim-Suche „Zum Strauchberg 8, 19370 Parchim“ und „Zum Strauchberg 8, 17192 Parchim“
  liefert **nur die beiden Straßenobjekte** `way 178018706` (53.4447341/11.8340169) und
  `way 34230297` (53.4423312/11.8300933) der Straße **K 120** — **keinen adressgenauen
  Hausnummernpunkt**.
* Gegenprobe im OSM-API-Auszug (BBox 11,8150/53,4350–11,8600/53,4560): **kein** Objekt mit
  `addr:street=Zum Strauchberg` und **keine** GKM-Tagging; die PLZ 19370 ist dort an den
  Nachbarstraßen (Am Spargelacker, Neuhofer Weiche) hinterlegt.
* Ein Abruf der Google-Maps-Suchseite liefert keine auswertbaren Koordinaten (JS-Shell).
* → **Es wird keine Koordinate gesetzt** (gleiches Vorgehen wie im Schwesterbericht
  `research/dossier-luecken-20261028.md`, Teil A.6, für Dessau). Ein Straßenmittelpunkt wäre kein
  Hof- oder Waagenpunkt und würde die Zwei-Quellen-Regel nicht erfüllen.

## A.6 Abgrenzung zum Seed-Datensatz `mv-parchim-19370-schrotthandel-parchim`

Der Seed-Datensatz beschreibt eine **Verzeichnisanzeige** („Schrotthandel Parchim“, Juri-Gagarin-Ring
10, 19370 Parchim, ohne Inhaber, Kontakt, Website oder Preise). Der hier recherchierte
Umschlagplatz liegt an einer **anderen Straße** (Zum Strauchberg 8), hat einen **anderen
Betreiber** (GKM Güstrower Kies & Mörtel GmbH) und ist **nicht** in der Anzeige genannt. Beide
Adressen werden im neuen Dossier ausdrücklich nebeneinander dokumentiert; **keine Gleichsetzung,
kein Merge, keine Löschung**, und am Seed-Datensatz ändert dieser Bericht nichts.

Ebenfalls getrennt bleiben: `mv-parchim-19370-parchimer-entsorgungs-peg` (historische PEG, HRB 219
Schwerin, 2012 gelöscht) und `mv-parchim-autoverwertung-brandt-u-heye-flag-autove` (Autoverwertung
Heye/Brandt, rund 0,2 km entfernt, eigener Datensatz).

## A.7 Offene Punkte (Owner)

1. **Ankauf oder nur Annahme?** Der Parchimer Standort ist ein Umschlagplatz/Recyclinghof eines
   Kies- und Entsorgungsbetriebs. Ob dort Metallschrott **gegen Vergütung** angekauft wird, ist
   unbelegt (Betreiberseite nennt weder „Schrott“ noch „Ankauf“). Klärweg: Rückfrage beim Betrieb
   (03871 451094) oder Vor-Ort-Anlieferung; eine schriftliche Efb-Bestätigung mit Ankauf-Status
   wäre der saubere Beleg.
2. **Private vs. gewerbliche Anlieferung:** Die ALP-Liste nennt „Gewerbeabfälle“ als eine Fraktion
   neben Bauschutt/Schrott; eine Privatkundenregel ist nirgends veröffentlicht. Der Entwurf setzt
   deshalb `customer_types: ["gewerbe"]` und weist Privatkunden als ungeklärt aus.
3. **Gebühren/Annahmebedingungen** sind nicht veröffentlicht → vor Anfahrt anrufen.
4. **Standort „Weitendorf“** (nur im Verzeichnis recyclinghof-wertstoffhof.de genannt, 52,9 km) ist
   auf der Betreiberseite nicht vorhanden → kein Datensatz, Klärung über den Betreiber.
5. **Abweichende Adressangabe der Deponie Plaatz/Spoitgendorf** (Register „An der Deponie 1“ vs.
   Betreiber „Glasewitzer Straße“) — betrifft einen anderen Standort, gehört aber in die
   Betreiberkette des GKM.
6. **Registerauszug ist vom 11.08.2026** (17 Tage alt); der Efb-Status kann seither geändert
   worden sein.

---

# Teil B — „AC Containerdienst Eltville“ / Marktstr. 6, 65343 Eltville

## B.1 Faktencheck der Auftragsprämisse

Die Auftragsprämisse lautet: *„Die Welle vom 28.10.2026 fand einen Zweitstandort der bereits
erfassten AC Containerdienst UG … Der Portallead nennt ‚Reingold Eltville, Marktstr. 6‘.“* — d. h.
der Lead selbst nennt **„Reingold“**, nicht „AC Containerdienst“. Die Recherche prüft, ob
Marktstr. 6 in Eltville ein Standort der AC Containerdienst UG ist. Ergebnis: **nein.**

| Prüfung | Ergebnis | Quelle |
|---|---|---|
| 11880-Umkreissuche „AC Containerdienst“ / Eltville | **1 Treffer**: AC Containerdienst UG (haftungsbeschränkt), Im Schwalbengrund 14, 63579 Freigericht (Somborn), **72,9 km** entfernt — **kein** Eintrag in Eltville | 11880-Suche (live, 28.10.2026) |
| Gelbe Seiten-Suche „AC Containerdienst“ / Eltville | 9 Treffer bundesweit, bester Treffer: „AC Containerdienst“, Raiffeisenstr. 26, 63579 Freigericht (Somborn), **72,1 km** — **kein** Eltville-Eintrag | Gelbe Seiten-Suche (live, 28.10.2026) |
| Betreiber-Website: Impressum | **eine** Anschrift: Im Schwalbengrund 14, 63579 Freigericht; AG Hanau HRB 101188, GF Merle Federau; **keine** Zweigniederlassung, kein Eltville | ac-containerdienst.de/impressum (live) |
| Betreiber-Website: Leistungsseiten | Einsatzgebiet durchgehend **„Gelnhausen, Freigericht, Wächtersbach und Umgebung“** bzw. „in Freigericht, Gelnhausen, Alzenau und Umgebung“ — **Eltville kommt auf keiner Seite vor** | ac-containerdienst.de Startseite, /containerdienst-abfallentsorgung/, /schrotthandel-schrottankauf/, /kontakt/ |
| Registerprofil (Dienstleister) | AC Containerdienst UG (haftungsbeschränkt), HRB 101188 Hanau, **gegründet 01.04.2026**, Stammkapital 1.500 €, GF Merle Federau, Unternehmensgegenstand „Der Containerdienst, die Abfallentsorgung sowie der Handel mit Schrott und Metall“, Adresse Im Schwalbengrund 14, 63579 Freigericht, WZ 2008: Sammlung nicht gefährlicher Abfälle / Rückgewinnung sortierter Werkstoffe / Großhandel mit Erzen und Metallen — **keine** Eltville-Niederlassung | Implisense-Registerprofil (Dienstleister, kein amtliches Register) |
| Verzeichnis „AC Containerdienst“ (city-map) | Einziger Eintrag: Raiffeisenstraße 26, 63579 Freigericht-Somborn, 06055 8929763 — die **Altadresse/Altnummer der Marke JR-Recycling** vor der Umfirmierung zur UG (Impressum 13.05.2025); der city-map-Treffer „Wächtersbacher Weg 40, Bad Orb“ gehört **SSCT Schmid Spedition**, nicht AC Containerdienst | city-map (live) |
| Weitere Portale (schrottplatz.org, schrottplatz-info.de, schrottfinder.de, lokaleschrottplatz.de, recyclinghof-wertstoffhof.de, recyclingpoint.de, abfallscout.de, branchen-info Eltville) | **kein** AC-Containerdienst-Eintrag in Eltville | Live-Abrufe 28.10.2026 |

→ **Die Marke AC Containerdienst hat in Eltville keinen belegten Standort; die Gesellschaft ist
7 Monate alt und hat eine Anschrift.** Der Wellen-Eintrag „AC Containerdienst Eltville“ ist eine
**Fehlschluss-Identifikation** des Portalleads.

## B.2 Was ist die AC Containerdienst UG (haftungsbeschränkt)?

| Feld | Wert | Beleg |
|---|---|---|
| Rechtsträger | AC Containerdienst UG (haftungsbeschränkt) | Betreiber-Impressum + Registerprofil |
| Register | AG Hanau HRB 101188 (gegründet 01.04.2026, Stammkapital 1.500 €) | Betreiber-Impressum + Implisense |
| Geschäftsführung | Merle Federau | Betreiber-Impressum + Registerprofil |
| Sitz | Im Schwalbengrund 14, 63579 Freigericht | Betreiber-Impressum |
| Kontakt | 01577 3719857, info@ac-containerdienst.de, ac-containerdienst.de | Betreiber-Website |
| Markenhistorie | „JR-Recycling“ (Inhaber Juliano Jährig, Raiffeisenstr. 26, Freigericht) → seit 14.07.2026 „AC Containerdienst UG (haftungsbeschränkt)“, Im Schwalbengrund 14 | Wayback-Kette (13.05.2025–19.04.2026) + Dossier `by-alzenau-jr-recycling` |
| Dossiers im Repo | `by-alzenau-jr-recycling` (Slug mit Altnamen, Name korrigiert) und `by-freigericht-somborn-63579-he-ac-containerdienst` — **zwei Zeilen für dieselbe Gesellschaft** | Repo, 28.10.2026 |

**Hinweis für den Owner (nicht Teil meines Auftrags):** Für die AC Containerdienst UG existieren
zwei Dossierzeilen mit demselben Betreiber (`by-alzenau-jr-recycling` und
`by-freigericht-somborn-63579-he-ac-containerdienst`). Das ist ein Merge-/Dublettenkandidat, der
im Rahmen dieser Recherche nicht angefasst wurde.

## B.3 Was ist Marktstr. 6 in Eltville?

| Quelle | Befund |
|---|---|
| **11880** (live, 28.10.2026) | „**Goldankauf Reingold Eltville**“, Branche: Goldankauf / Juwelier / An- und Verkauf, **Marktstr. 6, 65343 Eltville am Rhein**, Telefon **(06123) 9252982**, E-Mail **ankauf@reingold-eltville.de** (Cloudflare-geschützt im HTML, dekodiert), Öffnungszeiten **Mo–Fr 09:30–18:00, Sa 09:30–13:00**, 2 Bewertungen (5,0), 2 Fotos **vom 13.12.2020**, GoLocal-Bewertungen 21.04.2017 und 22.10.2016, Google 4,7 (18) — **keine Website hinterlegt** („Website hinzufügen“) |
| **Gelbe Seiten** (live) | „**Goldankauf Reingold Goldankauf**“, Branche „An- und Verkauf“, Leistungsumfang „Gold, Silber, Schmuck, Münzen, Goldankauf, Rüdesheim“, **Marktstr. 6, 65343 Eltville am Rhein**, **06123 9 25 29 82**, Öffnungszeiten **Mo–Fr 09:30–18:00, Sa 09:30–13:00**, 2 Bewertungen (golocal 5,0 / google 4,7) |
| **GoLocal** | Adresse/Telefon wie oben, Öffnungszeiten **Mo–Fr 09:00–18:00, Sa 09:00–13:00**, „✓ Überprüfte Adresse“, Bewertung „Sehr netter und fairer Ankäufer … Mein Schmuck wurde in meinem Beisein professionell geprüft. Das Geld habe ich bar erhalten.“ (22.10.2016) |
| **branchen-info.net** | Branchengruppe „Gold- und Silberwaren“, **Marktstr. 6, 65343 Eltville am Rhein (Eltville)**, 06123 9252982, **ankauf@reingold-eltville.de**, **„keine Website“**, Öffnungszeiten **Mo–Fr 09:00–18:00, Sa 09:00–13:00**, So geschlossen; Betriebstext: „Wir kaufen Gold, Silber und andere Edelmetalle aller Art zu aktuellen Tagespreisen. Zusätzlich bieten wir professionelle Schmuckreparaturen und das Wechseln von Batterien für Uhren, sowie das Reinigen und Polieren von Schmuck an.“; Suchindex-Auszug desselben Eintrags: **„GE-Zertifikat … Dieser Eintrag wurde zuletzt am 18.06.2026 überprüft. Das Unternehmen ‚Goldankauf Reingold Eltville‘ bestätigte die Richtigkeit der Adresse. Außerdem wurde auch die Telefonnummer geprüft.“** (im HTML des Live-Abrufs am 28.10.2026 war dieser Hinweisblock nicht mehr enthalten → nur als Index-Auszug zu werten) |
| **stadtbranchenbuch (Eltville)** | Leistungsbeschreibung: „Ankauf von Altgold, Bruchgold, Zahngold, Schmuck, Münzen, Besteck, Barren, Uhren, Silber, Platin zu aktuellen Tagespreisen“ (Live-Abruf am 28.10.2026 durch Cloudflare-Sperre 403 → nur Index-Auszug) |
| **Domain reingold-eltville.de** | HTTP 200 mit `<meta http-equiv="Refresh" content="0;url=defaultsite">` (IONOS-Parkplatzseite), **HTTPS: TLS-Fehler** („tlsv1 alert internal error“), DNS-MX weiterhin `mx00.ionos.de`/`mx01.ionos.de`, **einziger** Wayback-Snapshot 24.05.2023 zeigt bereits dieselbe Parkplatzseite → die Domain war also **mindestens seit Mai 2023 geparkt** |
| **Fachbetrieberegister (Auszug 11.08.2026)** | Für Eltville existiert **keine Ortsseite** — für einen Edelmetallhändler besteht ohnehin keine Efb-Anzeigepflicht; der Negativbefund ist ohne Aussagekraft |
| **OSM** | Gebäude `way 304565515` (addr:housenumber 6, addr:street „Marktstraße“, addr:postcode 65343, addr:city „Eltville am Rhein“) → Adresspunkt **50.0252972 / 8.1196743**; **Gegenindiz:** `node 4582597790` „**Änderungsschneiderei Uz**“ (craft=tailor, Telefon +49 61 23 624 52, Erfassung 16.06.2024) an derselben Adresse; 11880-Kartennadel 50.025475/8.119755 (Abweichung ca. 25 m) |
| **Namensvetter (getrennt zu halten)** | „**Rheingauer Goldankauf**“, Wilhelmstr. 9, 65343 Eltville, 0176 967 33 894; „hk werteshop“ (Eltville); „Reingold“ als Firmierung existiert daneben in Langenfeld (ReinGold), Solingen, Worms u. a. — **keine** Verbindung zum Standort Eltville belegt |

**Bewertung:** Marktstr. 6 ist ein **Goldankauf/Edelmetallhandel**, kein Schrott- oder Containerstandort.
Ein Schrottankauf ist dort **nicht** belegt; die Tätigkeit ist Gold-/Silber-/Platinankauf zu
Tagespreisen ohne veröffentlichte Kurse, dazu Schmuckservice (Reparaturen, Batteriewechsel,
Reinigung/Polieren) ohne Preisliste. Ein aktueller Betriebsstatus 2026 ist **nicht** belegt:
Parkplatz-Domain, jüngste Verzeichnisbestätigung 18.06.2026, letzte Bewertung 2017, letzte Fotos
13.12.2020, und OSM führt an der Adresse eine Schneiderei.

## B.4 Abgrenzung zum Mainzer Bestandseintrag `rp-mainz-55129-reingold`

| | Mainz (Bestandsdossier) | Eltville (neuer Standort) |
|---|---|---|
| Marke | „Goldankauf Reingold“ | „Goldankauf Reingold Eltville“ |
| Anschrift | Alte Mainzer Str. 21, 55129 Mainz-Hechtsheim | Marktstr. 6, 65343 Eltville am Rhein |
| Telefon | 06131 6367556 | 06123 9252982 |
| E-Mail | ankauf@reingold-eltville.de | ankauf@reingold-eltville.de (**identisch**) |
| Website | reingold-eltville.de (Parkplatz, `website_status: tot`) | reingold-eltville.de (Parkplatz) |
| Öffnungszeiten | Mo–Fr 10:00–18:00, Sa 10:00–13:00 | Mo–Fr 09:00–18:00 (golocal/branchen-info) bzw. 09:30–18:00 (11880/Gelbe Seiten), Sa 09:00–13:00 bzw. 09:30–13:00 |
| Bewertungen | 3 (GS, 2018) / 1 (11880) + Foto 13.12.2020 | 2 (11880/GS) + 2 Fotos 13.12.2020 + GoLocal 2016/2017 |

**Was belegt ist:** Die **Marke** „Reingold“/„Goldankauf Reingold“ und die **gemeinsame
Kontaktadresse** `ankauf@reingold-eltville.de` sind an beiden Standorten belegt (11880-Einträge
Mainz und Eltville, Gelbe Seiten, GoLocal, branchen-info.net). Der Eltville-Standort ist damit ein
**zweiter Standort derselben Marke** und ein **eigener Datensatz** (eigene Anschrift, eigene
Rufnummer, eigene Öffnungszeiten, eigener Datensatz im Repo fehlte bisher).

**Was nicht belegt ist:** Ob beide Standorte **dieselbe Rechtsträgerin/denselben Inhaber** haben,
ist **nicht bestimmbar** — es existiert weder eine Betreiberseite noch ein Impressum noch ein
Registereintrag (für einen kleinen Edelmetallhändler ist keiner zu erwarten). Ebenso wenig ist
eine USt-IdNr. für die Marke auffindbar (weder für Mainz noch für Eltville).

**Was ausdrücklich NICHT übertragen wird:** Anschrift, Rufnummer, Öffnungszeiten, Bewertungen und
GEO des Mainzer Standorts werden **nicht** auf Eltville übertragen; die Mainzer Marken- und
Domainangaben werden **nicht** als Betreiberbeleg für Eltville gewertet. Die Marke „Reingold“ selbst
darf für Eltville verwendet werden (sie ist dort mehrfach als Firmierung belegt) — die
**Mainzer** Daten nicht.

**Zuordnung zur AC Containerdienst UG:** Es gibt **keinen** Beleg, dass die Marke „Reingold“ zur
AC Containerdienst UG gehört oder von ihr betrieben wird: andere Branche (Edelmetall vs.
Container/Schrott), andere Rufnummern, andere Domain, andere Stadt. Der Name „AC Containerdienst
Eltville“ wird deshalb **nicht** in den Datenbestand übernommen.

## B.5 Bewertung der Wellen-Angabe und Empfehlung

* Die Wellenangabe „AC Containerdienst Eltville“ ist **nicht belegt**; sie entsteht aus der
  Gleichsetzung des Leadnamens („Reingold Eltville“) mit dem bereits erfassten
  AC-Containerdienst-Datensatz. Beide Umkreissuchen (11880, Gelbe Seiten) widerlegen sie für den
  Ort.
* **Empfehlung 1:** **Kein** AC-Containerdienst-Datensatz für Eltville anlegen. Der Lead wird im
  Mainzer- bzw. Eltville-Dossier als Abgrenzung dokumentiert.
* **Empfehlung 2:** **Eigenes Dossier `he-eltville-65343-goldankauf-reingold`** für Marktstr. 6
  anlegen (Entwurf unten) — analog zur Repo-Praxis „Schwester-Dossiers statt Zusammenführen“
  (vgl. „Rheinische Scheidestätte Filiale Kaiserslautern“, „Scholz-Betriebsstellen“). `status:
  pruefung`, `website_status: tot`, `trader_type: sonstige` (wie das Mainzer Schwesterdossier).
* **Empfehlung 3:** Im Mainzer Dossier `rp-mainz-55129-reingold` nur einen **Querverweis** auf den
  neuen Eltville-Datensatz und die Markenkette ergänzen — **keine** Feldmigration.
* **Empfehlung 4 (Klärfall Owner):** Ob der Standort 2026 noch betrieben wird, ist offen
  (Parkplatz-Domain, letzte Verzeichnisbestätigung 18.06.2026, OSM-Schneiderei an der Adresse).
  Klärweg: Anruf unter 06123 9252982 oder Gewerberegister-Auskunft der Stadt Eltville; **keine**
  Aufnahme ins amtliche Fachbetrieberegister (für Edelmetallhändler nicht erforderlich).

---

# Teil C — Dossier-Entwürfe (Repo-Format, **nicht angelegt**)

> Format: YAML-Frontmatter mit ausschließlich den im Seed-Codegen erlaubten Keys, dann
> `## Überblick` und `## Timeline` mit datierten Notizen im Format
> `[Recherche 28.10.2026: …; Quelle(n): …]`.
> **Beide Dateien wurden nicht angelegt und nicht geändert.** Sie sind vom Owner zu prüfen,
> ggf. anzupassen und dann durch `crates/ingestion/build.rs` kompiliert zu lassen
> (`cargo test -p schrott-mcp-ingestion seed --locked --offline`).

## C.1 `dossiers/mv/mv-parchim-19370-umschlagplatz-parchim.md`

**Begründung für `trader_type: schrottplatz`:** Der Standort ist im amtlichen Fachbetrieberegister
unter der Bezeichnung „**Umschlagplatz**“ geführt, die Betreiberseite nennt ihn „**Recyclinghof |
Umschlagplatz**“. Er ist eine **Betriebsstätte** (kein eigenständiger Rechtsträger), und für ihn ist
**kein Schrottankauf** belegt — weder auf der Betreiberseite (kein „Schrott“, kein „Ankauf“ auf
irgendeiner Seite) noch im Register (das nur Tätigkeiten/Schlüssel kennt) noch in der Behördenliste
(die „Annahme“ nennt). `schrotthaendler` würde eine vergütete Ankaufbeziehung suggerieren, die nicht
belegt ist (Präzedenz im Repo: „Görs Recycling GmbH“ → `sonstige`/`pruefung`, weil der öffentlich
beworbene Schrottankauf einer anderen Gesellschaft gehörte). Belegt sind dagegen die zertifizierte
**Verwertung** von Metallabfällen (170401/170402/170405, Tätigkeit „Verwerten“) und die Annahme
von Schrott im GKM-Angebot — das ist die Definition eines Schrottplatzes. Alternative
(`wertstoffhaendler`/`metallhaendler`) wurde verworfen, weil das Kerngeschäft ein
Kies-/Mörtel- und Bauschuttbetrieb ist und der Standort kein Sortier-/Handelsbetrieb ist.

**Begründung für `status: pruefung`:** Der Betreiber ist identifiziert und aktiv (Betreiberseite
2026 gepflegt, VIES gültig, Behördenliste 2026), aber die für den Katalog entscheidende Frage —
**zahlt der Standort Schrott an, und unter welchen Bedingungen?** — ist unbelegt: keine
Ankaufskurse, keine Mengengrenzen, keine Privatkundenregel, keine Gebühren, keine Abholung.
Nach README-Definition (`pruefung` = „buying status still needs a check“) ist `pruefung` die
ehrliche Wahl; `aktiv` würde einen nicht belegten Ankauf suggerieren.

```yaml
---
slug: mv-parchim-19370-umschlagplatz-parchim
name: Umschlagplatz Parchim (GKM Güstrower Kies + Mörtel GmbH)
trader_type: schrottplatz
state: MV
city: Parchim
street: 'Zum Strauchberg 8'
postcode: '19370'
phone: '03871 451094'
email: info@gkm-mv.de
opening_hours: 'Mo-Fr 07:00-16:00, Sa nach Absprache'
website: https://www.gkm-mv.de/
website_status: aktiv
certifications: '["Entsorgungsfachbetrieb §56 KrWG, Zertifikatsnummer ZZMT00400134 (Fachbetrieberegister, Auszug 11.08.2026); zertifizierte Tätigkeiten Lagern und Verwerten; Abfallschlüssel 170401 Kupfer/Bronze/Messing, 170402 Aluminium, 170405 Eisen und Stahl", "Dieselbe Zertifikatsnummer ZZMT00400134 führt die Betriebsstätte GKM Güstrower Kies + Mörtel GmbH, An der Deponie 1, 18276 Plaatz Spoitgendorf (Lagern, Behandeln, Verwerten, Beseitigen) sowie den Firmensitz Stellwerkswiese 2, 18292 Krakow am See (Sammeln, Befördern, Handeln, Makeln)"]'
status: pruefung
description: 'Betriebsstätte eines Kies-/Mörtel- und Entsorgungsbetriebs: Der amtliche Fachbetrieberegister (Auszug 11.08.2026) führt an der Anschrift Zum Strauchberg 8, 19370 Parchim den Eintrag "Umschlagplatz Parchim" mit Hauptsitz GKM Güstrower Kies + Mörtel GmbH, der Zertifikatsnummer ZZMT00400134, den Tätigkeiten Lagern und Verwerten und den Abfallschlüsseln 170401 (Kupfer/Bronze/Messing), 170402 (Aluminium) und 170405 (Eisen und Stahl). Betreiber: GKM Güstrower Kies & Mörtel GmbH, Impressum Amtsgericht Rostock HRB 3832, USt-IdNr. DE137632931 (am 28.10.2026 über den EU-VIES-Dienst gültig), Geschäftsführer Jens Eckhoff, Firmensitz Stellwerkswiese 2, 18292 Krakow; die Betreiberseite führt den Standort als "Recyclinghof | Umschlagplatz" mit Mo-Fr 07:00-16:00 Uhr und Samstag nach Absprache. Der Abfallratgeber 2026 der Abfallwirtschaft Ludwigslust-Parchim AöR nennt für den Standort "Abfallentsorgung zur Verwertung und Beseitigung, Annahme von Bauschutt und Gewerbeabfällen, Asbest, Dämmung, Teerpappe, Altholz, Schrott". Ankauf: ein Schrottankauf und Ankaufskurse sind nicht belegt - die Betreiberseite nennt auf keiner Seite weder Schrott noch Ankauf und bewirbt die Annahme von Sperrmüll, Grünschnitt, Bauschutt, Sonderabfall und schadstoffbelasteten mineralischen Abfällen; eine Abholung beim Kunden wird nicht angeboten. Verkauf: Recyclingmaterial, Kies, Sand, Schotter, Naturstein, Oberboden, Bodenannahme nach LAGA (Kerngeschäft). Gebühren: keine Tarife veröffentlicht; die Annahmebedingungen sind vor Anlieferung abzustimmen. Nicht gleichzusetzen mit dem Seed-Datensatz mv-parchim-19370-schrotthandel-parchim (Verzeichnisanzeige Juri-Gagarin-Ring 10, anderer und nicht belegter Betreiber) sowie mit mv-parchim-19370-parchimer-entsorgungs-peg (historische PEG, 2012 gelöscht).'
dropoff_json: '{"allowed": true, "customer_types": ["gewerbe"], "days": ["Mo", "Di", "Mi", "Do", "Fr"], "time_windows": ["07:00-16:00"], "conditions": "Annahme am Standort Zum Strauchberg 8 belegt für Bauschutt und Gewerbeabfälle, Asbest, Dämmung, Teerpappe, Altholz und Schrott (Abfallratgeber 2026 der Abfallwirtschaft Ludwigslust-Parchim AöR); das amtliche Fachbetrieberegister (Auszug 11.08.2026) führt für diese Anschrift die zertifizierten Tätigkeiten Lagern und Verwerten mit den Abfallschlüsseln 170401, 170402 und 170405. Gebühren, Mengengrenzen und Annahmebedingungen für Privatkunden sind nicht veröffentlicht und vor Anlieferung zu erfragen; Samstag nur nach Absprache. Eine Abholung von Schrott beim Kunden wird nicht angeboten."}'
pickup_json: ''
provenance_section: 'Recherche Dossier-Lücken 28.10.2026 (Fall 1: Umschlagplatz Parchim)'
provenance_ankauf_raw: 'unklar (Annahme von Schrott belegt, Vergütung/Ankauf nicht belegt)'
provenance_origin: table
---

# Umschlagplatz Parchim (GKM Güstrower Kies + Mörtel GmbH)

## Überblick

Der Standort **Zum Strauchberg 8, 19370 Parchim** ist im amtlichen Fachbetrieberegister (Auszug
11.08.2026) als „**Umschlagplatz Parchim**“ mit dem Hauptsitz **GKM Güstrower Kies + Mörtel GmbH**
geführt, Zertifikatsnummer **ZZMT00400134**, zertifizierte Tätigkeiten **Lagern und Verwerten**,
Abfallschlüssel **170401** (Kupfer/Bronze/Messing), **170402** (Aluminium) und **170405** (Eisen und
Stahl). Der Betreiber ist ein Kies-, Mörtel- und Entsorgungsbetrieb: Impressum **AG Rostock HRB
3832**, USt-IdNr. **DE137632931** (am 28.10.2026 über den EU-VIES-Dienst gültig), Geschäftsführer
**Jens Eckhoff**, Firmensitz Stellwerkswiese 2, 18292 Krakow; die Betreiberseite führt den
Parchimer Standort als „**Recyclinghof | Umschlagplatz**“ mit **Mo–Fr 07:00–16:00 Uhr**, Samstag nach
Absprache. Dieselbe Zertifikatsnummer trägt die Betriebsstätte in **Plaatz Spoitgendorf** und den
Firmensitz in **Krakow am See**.

**Ankauf:** **nicht belegt.** Die Betreiberseite nennt in **keiner** Seite weder „Schrott“ noch
„Ankauf“ und veröffentlicht keine Preise; der Registerauszug kennt nur Tätigkeiten und
Abfallschlüssel; die Behördenliste nennt „Annahme … Schrott“, nicht „Ankauf“. Belegt ist damit die
**Annahme und Verwertung** von Metallabfällen, nicht deren **Vergütung**. **Verkauf:** Recycling-
Baustoffe (Kies, Sand, Schotter, Naturstein, Oberboden, Recyclingmaterial) und Bodenannahme nach
LAGA — Kerngeschäft, keine Schrottankaufskala. **Gebühren:** keine Tarife; die Betreiberseite
verweist auf eine vor Anlieferung abzustimmende Annahmebedingung. **GEO:** für die Hausnummer 8
liefert OSM keinen adressgenauen Punkt (nur die Straßenobjekte K 120), deshalb wird **keine
Koordinate** gesetzt.

**Abgrenzung:** Der Seed-Datensatz `mv-parchim-19370-schrotthandel-parchim` beschreibt eine
Verzeichnisanzeige an anderer Straße (Juri-Gagarin-Ring 10) mit nicht belegtem Betreiber und bleibt
unverändert; ebenso getrennt bleiben `mv-parchim-19370-parchimer-entsorgungs-peg` (historische PEG,
HRB 219 Schwerin, 2012 gelöscht) und `mv-parchim-autoverwertung-brandt-u-heye-flag-autove`.

## Timeline

### Recherche 28.10.2026: Betreiber, Register und Tätigkeit des Umschlagplatzes geklärt

- [Recherche 28.10.2026: **Betreiber und Rechtsform sind über die Betreiberseite belegt.** Das Impressum (gkm-mv.de/common/info/impressum.html) nennt „GKM Güstrower Kies & Mörtel GmbH, Stellwerkswiese 2, 18292 Krakow, Tel. 038457 23606, Fax 038457 23101, info[at]gkm-mv.de, www.gkm-kies-moertel.de, Geschäftsführer Jens Eckhoff, Steuer-Nr. 52/215/01011, Ust-ID.: DE 137632931, Amtsgericht Rostock, HRB: 3832“; die USt-IdNr. DE137632931 wird vom EU-VIES-Dienst am 28.10.2026 09:17 UTC als gültig ausgewiesen (isValid true, userError VALID; VIES legt für Deutschland weder Namen noch Anschrift offen). Registerdaten-Dienstleister (keine amtlichen Register) nennen denselben Registereintrag HRB 3832 AG Rostock (Rostock 18057, wirtschaftsaktiv) und den Unternehmensgegenstand „Recycling von nichtmetallischen Altmaterialien und Reststoffen … insbesondere Recycling und Rückgewinnung von Bauschutt … sowie Herstellung und Vertrieb von Kiesen, Sanden, Mineralgemischen und anderen Schüttgütern“. Die Suche im Gemeinsamen Registerportal der Länder war am 28.10.2026 technisch nicht durchführbar (Fehler-ID, mask.do 400) und wird deshalb nicht als Negativbefund verwendet. Quelle(n): https://www.gkm-mv.de/common/info/impressum.html ; https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/137632931 ; https://www.northdata.de/GKM%20G%C3%BCstrower%20Kies%20+%20M%C3%B6rtel%20GmbH,%20Krakow%20a%C2%B7%20See/Amtsgericht%20Rostock%20HRB%203832 ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Amtliches Fachbetrieberegister (Auszug 11.08.2026).** Die Detailseite des Registerverzeichnisses schreibt: „Umschlagplatz Parchim ist im amtlichen Register mit der Anschrift Zum Strauchberg 8, 19370 Parchim geführt. Hauptsitz: GKM Güstrower Kies + Mörtel GmbH.“ Zertifizierte Tätigkeiten Lagern und Verwerten; Abfallschlüssel 170401 Kupfer/Bronze/Messing, 170402 Aluminium, 170405 Eisen und Stahl; Zertifikatsnummer ZZMT00400134; „Das Register nennt weder Telefon noch E-Mail“. Dieselbe Zertifikatsnummer führt die Betriebsstätte „GKM Güstrower Kies + Mörtel GmbH, An der Deponie 1, 18276 Plaatz Spoitgendorf“ (Lagern, Behandeln, Verwerten, Beseitigen; 120102, 170407, 191202, 191203) sowie den Firmensitz „Vermittlung, Stellwerkswiese 2, 18292 Krakow am See“ (Sammeln, Befördern, Handeln, Makeln; 62 Schlüssel u. a. 020110 Metallabfälle, 160104/160106 Altfahrzeuge). Für Parchim ist genau ein Betrieb verzeichnet; ein anerkannter Altfahrzeug-Demontagebetrieb ist dort nicht verzeichnet. Die Zertifikatsnummer gehört dem Unternehmen, nicht einer einzelnen Niederlassung. Quellen: https://schrottregister.pages.dev/betrieb-umschlagplatz-parchim-parchim ; https://schrottregister.pages.dev/ort-parchim ; https://schrottregister.pages.dev/betrieb-gkm-gustrower-kies-mortel-gmbh-plaatz ; https://schrottregister.pages.dev/betrieb-vermittlung-krakow-am-see ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Annahme von Schrott ist behördlich belegt, Ankauf nicht.** Der Abfallratgeber 2026 der Abfallwirtschaft Ludwigslust-Parchim AöR (Landkreis Ludwigslust-Parchim; PDF vom 14.11.2025, Seite 52) listet: „GKM GÜSTROWER KIES + MÖRTEL GmbH, Zum Strauchberg 8, 19370 Parchim, Tel. 03871 451094, Fax 03871 451098 - Abfallentsorgung zur Verwertung und Beseitigung, Annahme von Bauschutt und Gewerbeabfällen, Asbest, Dämmung, Teerpappe, Altholz, Schrott, Verkauf von Recyclingmaterial, Kies, Sand, Schotter, Natursteinen und Oberboden, Bodenannahme nach LAGA“; dieselbe Angabe steht in den Ausgaben 2022 und 2024. Einschränkung: Die Spalte „Leistungsangebot“ bezieht sich in dieser Tabelle auf den gesamten Eintrag GKM mit zwei Anschriften (Parchim und Dargelütz); die standortbezogene Schrott-Aussage für Parchim stützt sich deshalb auf das Efb-Zertifikat (170401/170402/170405, Verwerten) an exakt dieser Anschrift. ANKAUF: die Betreiberseite wurde vollständig geprüft (Startseite, unser-unternehmen, baustoffe, baustoffe/kies-und-sand, baustoffe/strassenbaustoffe, entsorgung-und-deponien, bewertung, kontakt, standorte, Impressum; Sitemap mit 13 URLs, davon 9 Inhalts- und 4 Infoseiten) - das Wort „Schrott“ und „Ankauf“ kommen auf keiner Seite vor, es gibt keine Preise, keine Mengenstaffel, keine Abholung, keine Privatkundenregel. VERKAUF: Recycling-Baustoffe und Bodenannahme nach LAGA. GEBÜHREN: keine Tarife; die Betreiberseite verweist auf eine vor Anlieferung abzustimmende Annahmebedingung und darauf, dass gefährliche Abfälle in zugelassene Anlagen verbracht werden müssen. Quellen: https://alp-lup.de/media/custom/3626_100_1.PDF?1765378065 ; https://alp-lup.de/media/custom/3626_187_1.PDF?1702547458 ; https://www.gkm-mv.de/entsorgung-und-deponien/ ; https://www.gkm-mv.de/sitemap.xml ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Standortdaten sind mehrfach unabhängig belegt.** Betreiber-Standortseite: „Parchim (Recyclinghof | Umschlagplatz), Zum Strauchberg 8, 19370 Parchim, Tel. 03871 4510 94, Fax 03871 4510 98, Öffnungszeiten Montag bis Freitag 07.00-16.00 Uhr, Samstag nach Absprache“; der Abfallratgeber 2026 der ALP AöR bestätigt Anschrift, Telefon und Fax; Das Örtliche führt „Recycling und Umschlagplatz Güstrower Kies und Mörtel GmbH, Zum Strauchberg 8, 19370 Parchim“ mit denselben Nummern, der Mailadresse info@gkm-mv.de und der Website gkm-mv.de (Verzeichnis-Lead, Mobilnummer 0172 3814105 nur hier belegt und daher nicht übernommen); recyclinghof-wertstoffhof.de (Stand Juli 2026) nennt Mo-Fr 07:00-16:00 und Sa/So geschlossen - die Abweichung am Samstag wird zugunsten der Betreiberseite dokumentiert. Die E-Mail info@gkm-mv.de ist das zentrale Unternehmenspostfach am Sitz Krakow, kein standortspezifisches Postfach. Quellen: https://www.gkm-mv.de/standorte/ ; https://alp-lup.de/media/custom/3626_100_1.PDF?1765378065 ; https://mobil.dasoertliche.de/Themen/Recycling-und-Umschlagplatz-G%C3%BCstrower-Kies-und-M%C3%B6rtel-GmbH-Parchim-Zum-Strauchberg ; https://recyclinghof-wertstoffhof.de/recyclinghof/mecklenburg-vorpommern/parchim/gkm-guestrower-kies-moertel-gmbh-zum-strauchberg-8 ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **GEO.** Für „Zum Strauchberg 8, 19370 Parchim“ liefert Nominatim nur die beiden Straßenobjekte (way 178018706, 53.4447341/11.8340169; way 34230297, 53.4423312/11.8300933 der Straße K 120) und keinen adressgenauen Hausnummernpunkt; im OSM-API-Auszug des umgebenden Bereichs (BBox 11,8150/53,4350 bis 11,8600/53,4560) gibt es kein Objekt mit addr:street „Zum Strauchberg“ und kein Betreiber-Tagging. Die PLZ 19370 ist dort an den Nachbarstraßen hinterlegt, was die PLZ-Angabe aller Quellen stützt; die Auftragsprämisse 17192 ist falsch, denn 17192 gehört zu Groß Dratow u. a. im Landkreis Müritz, nicht zu Parchim. Es wird deshalb keine Koordinate gesetzt und kein Straßenmittelpunkt geraten. Quellen: https://nominatim.openstreetmap.org/search?format=jsonv2&q=Zum%20Strauchberg%208%2C%2019370%20Parchim&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/map?bbox=11.8150,53.4350,11.8400,11.4560 ; https://www.suche-postleitzahl.org/plz-gebiet/19370 ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Abgrenzung zum Seed-Datensatz.** Der Seed-Datensatz mv-parchim-19370-schrotthandel-parchim beschreibt eine Verzeichnisanzeige („Schrotthandel Parchim“, Juri-Gagarin-Ring 10, ohne Inhaber, Kontakt, Website oder Preise; OSM löst Juri-Gagarin-Ring 10 in Parchim 19370 auf). Der Umschlagplatz liegt an anderer Straße und hat einen anderen Betreiber; beide Datensätze bleiben nebeneinander bestehen, es erfolgt keine Gleichsetzung, kein Merge und keine Löschung. Ebenso getrennt: mv-parchim-19370-parchimer-entsorgungs-peg (historische PEG, Löschung 03.12.2012) und mv-parchim-autoverwertung-brandt-u-heye-flag-autove. Quellen: https://www.nordkurier.de/regionale-unternehmen/schrotthandel-parchim-203992 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Juri-Gagarin-Ring%2010%2C%20Parchim ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Offene Punkte für den Owner.** (1) Ankauf oder nur Annahme? Der Standort ist ein Umschlagplatz eines Kies- und Entsorgungsbetriebs; ob Metallschrott gegen Vergütung angekauft wird, ist unbelegt - Klärweg: Rückfrage unter 03871 451094 oder schriftliche Bestätigung des Efb-Status mit Ankauf-Angabe. (2) Privatkunden: die ALP-Liste nennt „Gewerbeabfälle“ neben Bauschutt und Schrott, eine Privatkundenregel ist nicht veröffentlicht; der Entwurf setzt daher customer_types [gewerbe] und weist Privatkunden als ungeklärt aus. (3) Abholung beim Kunden wird nicht angeboten, pickup bleibt leer. (4) Das Verzeichnis recyclinghof-wertstoffhof.de nennt einen weiteren Standort desselben Betreibers in Weitendorf (52,9 km), der auf der Betreiberseite nicht existiert - Klärung über den Betreiber, kein Datensatz. (5) Für die Deponie Plaatz/Spoitgendorf weichen Register („An der Deponie 1“) und Betreiberseite („Glasewitzer Straße“) ab - betrifft einen anderen Standort desselben Betreibers. (6) Der Registerauszug datiert vom 11.08.2026. Quellen: read-only Produktionsabfrage `file:…public.db?mode=ro` + `PRAGMA query_only=ON` (traders, 28.10.2026) ; https://recyclinghof-wertstoffhof.de/recyclinghof/mecklenburg-vorpommern/parchim/gkm-guestrower-kies-moertel-gmbh-zum-strauchberg-8 ; Abruf 28.10.2026.]
```

## C.2 `dossiers/he/he-eltville-65343-goldankauf-reingold.md`

**Begründung für den Entwurf:** Ein AC-Containerdienst-Datensatz für Eltville ist **nicht** belegt
und wird **nicht** angelegt. Der Standort Marktstr. 6 ist jedoch ein eigenständig adressierbarer
Betrieb mit eigener Marke („Goldankauf Reingold“), eigener Anschrift, eigener Rufnummer und
eigenen Öffnungszeiten; die Marke ist über die in allen Verzeichnissen genannte Kontaktadresse
`ankauf@reingold-eltville.de` mit dem Mainzer Bestandseintrag verbunden. Das Repo führt Filialen
getrennt (Querverweis statt Zusammenführung), deshalb ein eigener Datensatz mit ausdrücklicher
Abgrenzung. `trader_type: sonstige` wie im Mainzer Schwesterdossier (Edelmetallhandel ist kein
Schrott-/Container-Typ). **`status: pruefung`**, weil es keine Betreiberseite gibt (Domain
geparkt), der jüngste Bestätigungshinweis ein Verzeichnis-Eintragscheck vom 18.06.2026 ist, die
letzte Kundenbewertung von 2017 und die letzten Fotos von 2020 stammen und OSM an derselben Adresse
eine Schneiderei ausweist.

```yaml
---
slug: he-eltville-65343-goldankauf-reingold
name: Goldankauf Reingold Eltville
trader_type: sonstige
state: HE
city: Eltville
street: 'Marktstr. 6'
postcode: '65343'
lat: 50.0252972
lon: 8.1196743
phone: '06123 9252982'
email: ankauf@reingold-eltville.de
opening_hours: 'Mo-Fr 09:00-18:00, Sa 09:00-13:00'
website: https://reingold-eltville.de/
website_status: tot
status: pruefung
description: 'Gold- und Silberankauf (Edelmetallhandel) am Standort Marktstr. 6, 65343 Eltville am Rhein, laut 11880 und Gelbe Seiten mit der Telefonnummer 06123 9252982 und der Kontaktadresse ankauf@reingold-eltville.de; 11880 führt den Eintrag als "Goldankauf Reingold Eltville" mit den Leistungen Goldankauf, Juwelier und An- und Verkauf, branchen-info.net mit "Wir kaufen Gold, Silber und andere Edelmetalle aller Art zu aktuellen Tagespreisen" sowie Schmuckreparaturen, Batteriewechsel für Uhren, Reinigen und Polieren von Schmuck. Die Marke "Reingold" ist mit dem Mainzer Bestandseintrag rp-mainz-55129-reingold über die gemeinsame Kontaktadresse ankauf@reingold-eltville.de verbunden; Anschrift, Rufnummer, Öffnungszeiten und Bewertungen des Mainzer Standorts werden nicht übernommen. Kein Bezug zur AC Containerdienst UG (haftungsbeschränkt, AG Hanau HRB 101188, Im Schwalbengrund 14, 63579 Freigericht): deren Umkreissuchen in Eltville liefern ausschließlich den Freigericht-Sitz, eine Zweitfiliale in Eltville ist nicht belegt. Ankauf: Gold, Silber, Platin, Altgold, Bruchgold, Zahngold, Schmuck, Münzen, Besteck, Barren und Uhren zu Tagespreisen, aber ohne veröffentlichte Kurse oder Feingehaltsstaffel. Verkauf: Schmuckreparaturen und -service ohne Preisliste. Gebühren: keine Tarife. Eine Betreiberseite existiert nicht: reingold-eltville.de ist am 28.10.2026 eine IONOS-Parkplatzseite (HTTP 200 mit Meta-Refresh auf defaultsite, HTTPS mit TLS-Fehler, nur ein Wayback-Snapshot vom 24.05.2023 mit derselben Parkplatzseite, MX-Einträge bestehen weiterhin); der jüngste Bestätigungshinweis ist ein Verzeichnis-Eintragscheck vom 18.06.2026, jüngste Kundenbewertungen stammen aus 2016/2017 und die letzten Fotos von 2020, in OSM ist an der Adresse eine Schneiderei erfasst. Ein aktueller Betrieb 2026 ist damit nicht belegt; der Status bleibt pruefung. GEO: 50.0252972/8.1196743 ist der Adresspunkt des OSM-Gebäudes way 304565515 (addr:housenumber 6, addr:street Marktstraße, 65343 Eltville am Rhein), gedeckt von der 11880-Kartennadel (50.025475/8.119755, Abweichung rund 25 m); ein Eingangs- oder Waagenpunkt ist nicht belegt.'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Recherche Dossier-Lücken 28.10.2026 (Fall 2: AC Containerdienst Eltville / Goldankauf Reingold Eltville)'
provenance_ankauf_raw: 'ja (Gold-/Silberankauf zu Tagespreisen laut Betriebstext im Verzeichnis; keine Betreiberseite, kein numerischer Kurs)'
provenance_origin: table
---

# Goldankauf Reingold Eltville

## Überblick

„**Goldankauf Reingold Eltville**“ ist ein **Edelmetallhändler** am Standort **Marktstr. 6, 65343
Eltville am Rhein**, geführt von 11880 (Branche Goldankauf, Juwelier, An- und Verkauf; Telefon
**(06123) 9252982**; E-Mail **ankauf@reingold-eltville.de**; 2 Bewertungen mit 5,0; Fotos vom
13.12.2020), Gelbe Seiten (gleiche Anschrift und Nummer, „An- und Verkauf“, Leistungsumfang
Gold/Silber/Schmuck/Münzen), GoLocal (gleiche Anschrift, „✓ Überprüfte Adresse“, Bewertung vom
22.10.2016: „Mein Schmuck wurde in meinem Beisein professionell geprüft. Das Geld habe ich bar
erhalten.“) und branchen-info.net (Betriebstext: „Wir kaufen Gold, Silber und andere Edelmetalle
aller Art zu aktuellen Tagespreisen. Zusätzlich bieten wir professionelle Schmuckreparaturen und
das Wechseln von Batterien für Uhren, sowie das Reinigen und Polieren von Schmuck an.“).
**Öffnungszeiten:** Mo–Fr 09:00–18:00, Sa 09:00–13:00, So geschlossen (golocal/branchen-info);
11880 und Gelbe Seiten nennen abweichend 09:30 als Beginn — die Divergenz wird dokumentiert, nicht
geglättet.

**Ankauf:** Gold, Silber, Platin, Altgold, Bruchgold, Zahngold, Schmuck, Münzen, Besteck, Barren
und Uhren **zu Tagespreisen**, aber **ohne veröffentlichte Kurse oder Feingehaltsstaffel**;
die Bewertung erfolgt laut Kundenstimmen in Anwesenheit des Kunden. **Verkauf:** Schmuckreparaturen,
Batteriewechsel und Reinigung/Polieren, **ohne Preisliste**. **Gebühren:** keine Tarife.

**Betriebsstatus:** Eine **Betreiberseite existiert nicht** — `reingold-eltville.de` ist am
28.10.2026 eine IONOS-Parkplatzseite (HTTP 200 mit Meta-Refresh auf `defaultsite`, HTTPS scheitert
mit TLS-Fehler; der einzige Wayback-Abruf vom 24.05.2023 zeigt dieselbe Parkplatzseite, die Domain
war also mindestens seit Mai 2023 geparkt; die MX-Einträge `mx00.ionos.de`/`mx01.ionos.de` bestehen
weiterhin, die E-Mail-Adresse ist also formal noch zustellbar). Jüngste Bestätigung der Adresse
und der Rufnummer: Verzeichnis-Eintragscheck vom **18.06.2026**; jüngste Kundenbewertungen 2016/2017,
letzte Fotos 13.12.2020; in OSM ist an derselben Adresse eine **Schneiderei** erfasst. Ein aktueller
Betrieb 2026 ist damit **nicht belegt** → `status: pruefung`, `website_status: tot`.

**Abgrenzung (wichtig):** (1) Zur **AC Containerdienst UG (haftungsbeschränkt)** (AG Hanau
HRB 101188, GF Merle Federau, Im Schwalbengrund 14, 63579 Freigericht) besteht **kein** Beleg: Die
Umkreissuchen von 11880 und Gelbe Seiten für „AC Containerdienst“/Eltville liefern ausschließlich den
Freigericht-Sitz (72,9 km bzw. 72,1 km), das Betreiber-Impressum nennt nur die eine Anschrift, und
die Betreiberseite nennt als Einsatzgebiet Gelnhausen, Freigericht, Wächtersbach und Alzenau —
**Eltville kommt dort nicht vor**. (2) Zum Mainzer Bestandseintrag `rp-mainz-55129-reingold` ist die
**Markenkette** über die gemeinsame Kontaktadresse `ankauf@reingold-eltville.de` und die
Firmierung „Goldankauf Reingold“ belegt; die **Mainzer** Anschrift (Alte Mainzer Str. 21),
**Mainzer** Rufnummer (06131 6367556), **Mainzer** Öffnungszeiten (Mo–Fr 10:00–18:00) und die
Mainzer Bewertungen werden **nicht** übernommen. Ob beide Standorte denselben Rechtsträger haben,
ist ohne Betreiberseite, Impressum oder Registereintrag **nicht bestimmbar**. (3) Namensvetter ohne
Übernahme: „Rheingauer Goldankauf“ (Wilhelmstr. 9, 65343 Eltville), „hk werteshop“ (Eltville) sowie
gleichnamige Firmierungen in Langenfeld, Solingen und Worms.

**GEO:** 50.0252972/8.1196743 ist der Adresspunkt des OSM-Gebäudes `way 304565515`
(`addr:housenumber=6`, `addr:street=Marktstraße`, `addr:postcode=65343`, `addr:city=Eltville am
Rhein`), gedeckt von der 11880-Kartennadel (50.025475/8.119755, Abweichung rund 25 m). Punktart:
Gebäude-/Adresspunkt **ohne** Betreiber-Tagging — keine bestätigte Hofeinfahrt, kein Waagenpunkt,
kein Genauigkeitsradius belegt.

## Timeline

### Recherche 28.10.2026: kein AC-Containerdienst-Standort in Eltville; Adresse ist ein Goldankauf

- [Recherche 28.10.2026: **Die Auftragsprämisse ist nicht haltbar: Es gibt keinen AC-Containerdienst-Standort in Eltville.** Die 11880-Umkreissuche „AC Containerdienst“/Eltville liefert genau einen Treffer: AC Containerdienst UG (haftungsbeschränkt), Im Schwalbengrund 14, 63579 Freigericht (Somborn), 72,9 km entfernt. Die Gelbe-Seiten-Suche „AC Containerdienst“/Eltville zeigt neun bundesweite Treffer, der beste ist „AC Containerdienst, Raiffeisenstr. 26, 63579 Freigericht (Somborn)“ mit 72,1 km Entfernung — also die Altadresse der Marke JR-Recycling vor der Umfirmierung, keine Eltville-Adresse. Das Betreiber-Impressum von ac-containerdienst.de nennt nur Im Schwalbengrund 14, 63579 Freigericht (AG Hanau HRB 101188, GF Merle Federau), und die Leistungsseiten nennen als Einsatzgebiet durchgehend Gelnhausen, Freigericht, Wächtersbach, Alzenau und Umgebung. Weitere geprüfte Portale (schrottplatz.org, schrottplatz-info.de, schrottfinder.de, lokaleschrottplatz.de, recyclinghof-wertstoffhof.de, recyclingpoint.de, abfallscout.de, branchen-info Eltville) führen keinen AC-Containerdienst in Eltville. Ein Negativbefund beweist keine Nichtexistenz, aber die positive Gegenprobe (Umkreissuche, Betreiber-Impressum, Betreiber-Leistungsseiten, Registerprofil) ergibt in vier unabhängigen Zugriffswegen denselben Befund: Es gibt nur den Sitz in Freigericht. Quellen: https://www.11880.com/suche/ac%20containerdienst/eltville ; https://www.gelbeseiten.de/suche/ac%20containerdienst/eltville ; https://www.ac-containerdienst.de/impressum ; https://www.ac-containerdienst.de/containerdienst-abfallentsorgung/ ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Der Portallead ist selbst ein Goldankauf.** 11880 führt „Goldankauf Reingold Eltville“, Branche Goldankauf/Juwelier/An- und Verkauf, Marktstr. 6, 65343 Eltville am Rhein, Telefon (06123) 9252982, E-Mail ankauf@reingold-eltville.de (im HTML per Cloudflare-E-Mail-Schutz kodiert und dekodiert), Öffnungszeiten Mo-Fr 09:30-18:00 und Sa 09:30-13:00, 2 Bewertungen (5,0), 2 Fotos vom 13.12.2020, GoLocal-Bewertungen vom 21.04.2017 und 22.10.2016, Google 4,7 (18), keine Website hinterlegt. Gelbe Seiten: „Goldankauf Reingold Goldankauf“, Branche An- und Verkauf, Leistungsumfang Gold, Silber, Schmuck, Münzen, Goldankauf, Rüdesheim, Marktstr. 6, 65343 Eltville am Rhein, 06123 9 25 29 82, Öffnungszeiten Mo-Fr 09:30-18:00, Sa 09:30-13:00, 2 Bewertungen. golocal: gleiche Anschrift und Nummer, „Überprüfte Adresse“, Öffnungszeiten Mo-Fr 09:00-18:00, Sa 09:00-13:00. branchen-info.net: Branchengruppe Gold- und Silberwaren, Marktstr. 6, 65343 Eltville am Rhein, 06123 9252982, ankauf@reingold-eltville.de, „keine Website“, Betriebstext „Wir kaufen Gold, Silber und andere Edelmetalle aller Art zu aktuellen Tagespreisen. Zusätzlich bieten wir professionelle Schmuckreparaturen und das Wechseln von Batterien für Uhren, sowie das Reinigen und Polieren von Schmuck an.“; der Suchindex-Auszug desselben Eintrags weist ein „GE-Zertifikat … Dieser Eintrag wurde zuletzt am 18.06.2026 überprüft. Das Unternehmen ‚Goldankauf Reingold Eltville‘ bestätigte die Richtigkeit der Adresse. Außerdem wurde auch die Telefonnummer geprüft.“ aus — im Live-Abruf war dieser Hinweisblock nicht mehr im HTML enthalten, er wird deshalb nur als Index-Auszug gewertet. Quellen: https://www.11880.com/branchenbuch/eltville-am-rhein/181726177B102456283/goldankauf-reingold-eltville.html ; https://www.gelbeseiten.de/gsbiz/82d5cb27-880b-4276-a1c8-d64386d97445 ; https://www.golocal.de/eltville/ankauf-verkauf/goldankauf-reingold-eltville-Np1OB ; https://eltville-am-rhein.branchen-info.net/goldankauf-reingold-eltville/5585595 ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **DOMAIN-BEFUND: keine Betreiberseite.** reingold-eltville.de liefert über HTTP 200 mit einem Meta-Refresh auf „defaultsite“ (IONOS-Parkplatzseite), HTTPS scheitert mit TLS-Fehler („tlsv1 alert internal error“). Die Wayback-Maschine kennt für die Domain genau einen Abruf (24.05.2023), der ebenfalls die Parkplatzseite zeigt — die Domain war also mindestens seit Mai 2023 geparkt. Die DNS-MX-Einträge mx00.ionos.de und mx01.ionos.de bestehen weiterhin, die in den Verzeichnissen genannte Adresse ankauf@reingold-eltville.de ist formal noch zustellbar. `website` mit `website_status: tot` geführt. Quellen: http://reingold-eltville.de/ ; https://reingold-eltville.de/ (TLS-Fehler) ; http://web.archive.org/cdx/search/cdx?url=reingold-eltville.de ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Abgrenzung zum Mainzer Bestandseint rp-mainz-55129-reingold.** Beide Standorte führen dieselbe Marke und dieselbe Kontaktadresse: Die 11880-Einträge „Goldankauf Reingold“ (Alte Mainzer Str. 21, 55129 Mainz-Hechtsheim, 06131 6367556) und „Goldankauf Reingold Eltville“ (Marktstr. 6, 65343 Eltville, 06123 9252982) nennen beide die per Cloudflare-Schutz kodierte Mailadresse ankauf@reingold-eltville.de; die Mainz-Domäne reingold-eltville.de ist ebenfalls nur noch eine Parkplatzseite. Die Markenkette ist damit belegt, die Anschriften, Rufnummern und Öffnungszeiten sind je Standort verschieden (Mainz Mo-Fr 10:00-18:00 / Sa 10:00-13:00; Eltville Mo-Fr 09:00-18:00 bzw. 09:30-18:00 / Sa 09:00-13:00 bzw. 09:30-13:00) und werden nicht vermischt. Ob beide Standorte denselben Rechtsträger haben, ist ohne Betreiberseite, Impressum oder Registereintrag nicht bestimmbar; eine USt-IdNr. der Marke ist nicht auffindbar. Das Mainzer Dossier bleibt unverändert und soll nur einen Querverweis auf diesen Datensatz erhalten. Quellen: https://www.11880.com/branchenbuch/mainz-am-rhein/181726177B106661197/goldankauf-reingold.html ; https://www.11880.com/branchenbuch/eltville-am-rhein/181726177B102456283/goldankauf-reingold-eltville.html ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **GEO.** Der Adresspunkt des OSM-Gebäudes way 304565515 (addr:housenumber 6, addr:street Marktstraße, addr:postcode 65343, addr:city Eltville am Rhein) liegt bei 50.0252972/8.1196743; die 11880-Kartennadel desselben Eintrags liegt bei 50.025475/8.119755 (Abweichung rund 25 m). Als Gegenindiz ist in OSM an derselben Adresse der Knoten 4582597790 „Änderungsschneiderei Uz“ (craft=tailor, Telefon +49 61 23 624 52, Erfassung 16.06.2024) erfasst; der Punkt ist ein Gebäude-/Adresspunkt ohne Betreiber-Tagging, keine bestätigte Hofeinfahrt und kein Waagenpunkt. Im amtlichen Fachbetrieberegister (Auszug 11.08.2026) existiert für Eltville keine Ortsseite — für Edelmetallhändler besteht keine Efb-Anzeigepflicht, der Negativbefund ist ohne Aussagekraft. Quellen: https://api.openstreetmap.org/api/0.6/way/304565515 ; https://api.openstreetmap.org/api/0.6/node/4582597790 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Marktstr.%206%2C%2065343%20Eltville ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Namensvetter ohne Übernahme.** „Rheingauer Goldankauf“, Wilhelmstr. 9, 65343 Eltville, 0176 967 33 894; „hk werteshop“ (Eltville, 273 m/314 m entfernt gelistet); „ReinGold“-Firmierungen in Langenfeld, Solingen und Worms sind andere Betriebe. ANKAUF: Gold-, Silber- und Platinankauf zu Tagespreisen laut Betriebstext, keine Kurse und keine Feingehaltsstaffel veröffentlicht. VERKAUF: Schmuckreparaturen, Batteriewechsel, Reinigen/Polieren ohne Preisliste. GEBÜHREN: keine Tarife. Offen für den Owner: ob der Standort 2026 noch betrieben wird - Klärweg Anruf 06123 9252982 oder Gewerberegister-Auskunft der Stadt Eltville; ein Herkunftsbeleg aus einem amtlichen Register ist hier nicht zu erwarten. Quellen: https://eltville-am-rhein.stadtbranchenbuch.com/F/firmenverzeichnis.html ; https://www.11880.com/branchenbuch/eltville-am-rhein/181726177B102456283/goldankauf-reingold-eltville.html ; Abruf 28.10.2026.]
```

---

# Teil D — Empfehlungen an den Owner (Checkliste)

1. **Fall 1:** `dossiers/mv/mv-parchim-19370-umschlagplatz-parchim.md` nach dem Entwurf C.1
   anlegen; `cargo test -p schrott-mcp-ingestion seed --locked --offline` laufen lassen.
   **Keine** Änderung an `mv-parchim-19370-schrotthandel-parchim` (nur ggf. ein Querverweis auf den
   neuen Datensatz, damit die Verwechslung in beide Richtungen dokumentiert ist).
2. **Fall 2:** **kein** AC-Containerdienst-Datensatz für Eltville. `dossiers/he/he-eltville-65343-goldankauf-reingold.md`
   nach Entwurf C.2 anlegen; im Mainzer Dossier `rp-mainz-55129-reingold` **nur** einen Querverweis
   und die Markenkette ergänzen, **keine** Feldmigration (Mainz-Adresse, -Rufnummer, -Zeiten und
   -GEO bleiben dort).
3. **Namensvetten prüfen:** „Rheingauer Goldankauf“ (Wilhelmstr. 9, Eltville), „Goldstube 24K“
   (Wiesbaden), „ReinGold“ (Langenfeld/Solingen) — nicht mit „Reingold Eltville“ gleichsetzen.
4. **Dublette AC Containerdienst** (nicht Teil dieses Auftrags, aber aufgefallen): zwei Dossierzeilen
   für dieselbe Gesellschaft (`by-alzenau-jr-recycling`, `by-freigericht-somborn-63579-he-ac-containerdienst`).
5. **Nicht vergessen:** Für beide Fälle gilt, dass ein Negativbefund im Register keine Nichtexistenz
   beweist — für Fall 2 (Edelmetallhändler ohne Efb-Pflicht) und für Fall 1 (Kies-/Mörtelbetrieb mit
   Standort-Umschlag) ist das amtliche Fachbetrieberegister nur für die **Standorttätigkeit**
   aussagekräftig, nicht für den Ankauf.

## Belegprüfung / offene Fragen, die nur der Owner entscheiden kann

* Ist die **Annahme am Umschlagplatz Parchim** gewerblich oder auch privat, und wird **Schrott
  vergütet**? (Telefon 03871 451094; alternativ schriftliche Efb-Bestätigung mit Ankauf-Status.)
* Wird der **Standort Weitendorf** des GKM (nur im Verzeichnis) weiter betrieben?
* Betreibt „Goldankauf Reingold“ 2026 noch **Marktstr. 6** (Domain geparkt, OSM-Schneiderei)?

---

## Quellenverzeichnis (alle Abrufe am 28.10.2026)

**Fall 1 – Betreiber/Register/Behörde**
`https://www.gkm-mv.de/` · `https://www.gkm-mv.de/standorte/` · `https://www.gkm-mv.de/unser-unternehmen/` ·
`https://www.gkm-mv.de/entsorgung-und-deponien/` · `https://www.gkm-mv.de/bewertung/` ·
`https://www.gkm-mv.de/kontakt/` · `https://www.gkm-mv.de/common/info/impressum.html` ·
`https://www.gkm-mv.de/sitemap.xml` · `https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/137632931` ·
`https://schrottregister.pages.dev/betrieb-umschlagplatz-parchim-parchim` ·
`https://schrottregister.pages.dev/ort-parchim` ·
`https://schrottregister.pages.dev/betrieb-gkm-gustrower-kies-mortel-gmbh-plaatz` ·
`https://schrottregister.pages.dev/betrieb-vermittlung-krakow-am-see` ·
`https://alp-lup.de/media/custom/3626_100_1.PDF?1765378065` (Abfallratgeber 2026) ·
`https://alp-lup.de/media/custom/3626_187_1.PDF?1702547458` (2024) ·
`https://alp-lup.de/media/custom/3626_143_1.PDF?1652100457` (2022) ·
`https://mobil.dasoertliche.de/Themen/Recycling-und-Umschlagplatz-G%C3%BCstrower-Kies-und-M%C3%B6rtel-GmbH-Parchim-Zum-Strauchberg` ·
`https://recyclinghof-wertstoffhof.de/recyclinghof/mecklenburg-vorpommern/parchim/gkm-guestrower-kies-moertel-gmbh-zum-strauchberg-8` ·
`https://www.northdata.de/GKM%20G%C3%BCstrower%20Kies%20+%20M%C3%B6rtel%20GmbH,%20Krakow%20a%C2%B7%20See/Amtsgericht%20Rostock%20HRB%203832` ·
`https://nominatim.openstreetmap.org/search?format=jsonv2&q=Zum%20Strauchberg%208%2C%2019370%20Parchim&addressdetails=1` ·
`https://api.openstreetmap.org/api/0.6/map?bbox=11.8150,53.4350,11.8400,11.4560` ·
`https://www.suche-postleitzahl.org/plz-gebiet/19370` (PLZ 19370 = Parchim) ·
`https://www.suche-postleitzahl.org/plz-gebiet/17192` (17192 = Groß Dratow u. a., **nicht** Parchim)

**Fall 2 – Eltville / AC Containerdienst**
`https://www.11880.com/suche/ac%20containerdienst/eltville` ·
`https://www.11880.com/branchenbuch/eltville-am-rhein/181726177B102456283/goldankauf-reingold-eltville.html` ·
`https://www.11880.com/branchenbuch/mainz-am-rhein/181726177B106661197/goldankauf-reingold.html` ·
`https://www.gelbeseiten.de/suche/ac%20containerdienst/eltville` ·
`https://www.gelbeseiten.de/suche/reingold/eltville` ·
`https://www.gelbeseiten.de/gsbiz/82d5cb27-880b-4276-a1c8-d64386d97445` ·
`https://www.golocal.de/eltville/ankauf-verkauf/goldankauf-reingold-eltville-Np1OB` ·
`https://eltville-am-rhein.branchen-info.net/goldankauf-reingold-eltville/5585595` ·
`https://www.ac-containerdienst.de/impressum` · `https://www.ac-containerdienst.de/` ·
`https://www.ac-containerdienst.de/kontakt/` · `https://www.ac-containerdienst.de/containerdienst-abfallentsorgung/` ·
`http://reingold-eltville.de/` · `http://web.archive.org/cdx/search/cdx?url=reingold-eltville.de` ·
`https://api.openstreetmap.org/api/0.6/way/304565515` · `https://api.openstreetmap.org/api/0.6/node/4582597790` ·
`https://nominatim.openstreetmap.org/search?format=jsonv2&q=Marktstr.%206%2C%2065343%20Eltville` ·
`https://www.vorwahlen-online.de/plz/65326.html` (65326 = **Aarbergen**, nicht Eltville) ·
`https://www.postleitzahlen.de/plz/bereich-65` (Eltville: 65343–65347)

**Produktionsdatenbank (read-only, `file:…?mode=ro` + `PRAGMA query_only=ON`)**: `traders`-Abfrage
nach „Reingold“, „Eltville“, „Strauchberg“, „Güstrower Kies“, „GKM“ sowie Liste aller Parchimer
Zeilen — 28.10.2026.
