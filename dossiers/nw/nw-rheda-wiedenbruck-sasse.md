---
slug: nw-rheda-wiedenbruck-sasse
clear_fields: 'geo'
name: Sasse
trader_type: schrotthaendler
state: NW
city: Rheda-Wiedenbrück
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Sasse

## Überblick

Ein lokaler Zeitungsbericht vom 02.01.2020 liefert nun eine belastbare Betriebshistorie: Hermann Sasses Schrottbetrieb am Südring war geräumt; Christina Sasses Betrieb an der Lippstädter Straße und deren Langenberger Zweig hatten bereits einige Monate zuvor den Betrieb eingestellt. Beide gingen unter Hermann Kathöfer in Langenberg auf; Familiennachfolger gab es laut Bericht nicht. Das spricht dafür, dass beide damaligen Sasse-Handelsbetriebe 2019 endeten, belegt aber keine vollständige Rechtsnachfolge zu HRA 5881 und löst die generische Seed-Zuordnung nicht abschließend. Die KG HRA 5881 wird in Registerspiegeln weiterhin als aktive juristische Person geführt; rechtliche Existenz allein beweist keinen aktiven Schrottplatz.

Kein belegter aktueller Annahmeort für diese Seedzeile, keine Ankauf-, Verkaufs- oder Gebührenliste. Die Adressen Lippstädter Str. 61 und Südring 107 sind historische Betriebsorte, nicht zur aktuellen GEO-Geokodierung freigegeben. Status `pruefung` bleibt bis Seed-/Rechtsträgerzuordnung geklärt ist.

## Timeline

### Recherche 26.10.2026

- [Recherche 26.10.2026: GLEIF-API-Datensatz LEI 391200DERUQIA2FILR68 frisch gelesen (goldenCopy-Publikation 25.10.2026): Schrott-Sasse GmbH & Co. KG, HRA 5881, Lippstädter Str. 61, 33378 Rheda-Wiedenbrück; Entity ACTIVE, LEI-Status ISSUED, letzte Aktualisierung 18.04.2026, nächste Verlängerung Juni 2027. Das bestätigt den Rechtsträger und dessen Register-/LEI-Status, aber nicht aktuelle Schrottannahme, Betriebsfortführung oder Zuordnung des generischen Seeds gegenüber den historischen Sasse-Betrieben am Südring/Lippstädter Straße. Keine Ankauf-/Verkaufspreise oder Gebühren. `clear_fields: 'geo'` bleibt wirksam; historische/gesellschaftsrechtliche Adresse nicht als gegenwärtigen Annahmepunkt geokodieren. Quelle: https://api.gleif.org/api/v1/lei-records/391200DERUQIA2FILR68 ; Kontext/Betreiberhistorie siehe Recherche 23.10.2026.]

### Owner-GEO-Prüfung 23.10.2026

- [Owner-GEO-Prüfung 23.10.2026: Read-only-Abgleich der Produktionszeile: gespeicherter Altpunkt 51.8497000, 8.3002000. Die aktuelle Recherche belegt für diesen Seed keinen adressgenauen, dem Betreiber und einer öffentlichen Annahmestelle zugeordneten Punkt; ein Ersatz wäre geraten. `clear_fields: 'geo'` entzieht das Alt-Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate“, nicht Schließung oder Annahmeverweigerung. Keine Ersatzkoordinate geraten, Produktionsdatenbank nicht geschrieben. Quellen: read-only-Abgleich `public.db` vom 23.10.2026; Rechercheeintrag 23.10.2026 mit den dort verlinkten Quellen; `docs/trader-seed.md`.]


### Recherche 23.10.2026

- [Recherche 23.10.2026: Lokalen Zeitungsbericht vom 02.01.2020 direkt gelesen. Er berichtet, dass Hermann Sasses Betrieb am Südring seine letzten Lkw abgezogen und das Gelände geräumt hatte; Christina Sasses Betrieb an der Lippstädter Straße samt Langenberger Zweig hatte einige Monate zuvor geschlossen. Ohne Familiennachfolger wurden beide Geschäftsbetriebe unter Hermann Kathöfer in Langenberg vereinigt; Mitarbeiter und Geräte wechselten. Dies ist ein starker historischer Beleg für das Ende der damaligen Sasse-Handelsbetriebe, aber kein Registerbeleg zur Auflösung/Übertragung der weiterhin im Registerspiegel geführten Schrott-Sasse GmbH & Co. KG (HRA 5881) und kein Beleg, welche Sasse der knappe Seed meint. Daher kein aktueller Namens-/Adressfill, `status: pruefung`; beide Adressen bleiben historisch und ungeocodet. **Ankauf:** keine aktuelle Sasse-Annahme/Kurse belegt; **Verkauf:** keine Liste; **Gebühren:** keine Tarife. Quelle: https://www.die-glocke.de/kreis-guetersloh/rheda-wiedenbrueck/artikel/wiedervereinigung-im-schrottgewerbe-1624316601 ; Registergegenprüfung: https://www.northdata.de/Schrott-Sasse%20GmbH%20&%20Co%C2%B7%20KG,%20Rheda-Wiedenbr%C3%BCck/Amtsgericht%20G%C3%BCtersloh%20HRA%205881]

### Recherche 18.10.2026 — Welle 18:00 Shard 2

- [Recherche 18.10.2026: Neuer unabhängiger Identitätsdatenabgleich zur KG: GLEIF-API LEI391200DERUQIA2FILR68 direkt gelesen, Schrott-SasseGmbH&Co.KG/HRA5881/LippstädterStr.61/33378Rheda-Wiedenbrück, entityACTIVE, registrationISSUED, lastUpdate18.04.2026, nextRenewal12.06.2027. Registerdetail live kongruent und Abschluss2024 veröffentlicht02.03.2026. Das stützt den konkreten KG-Kandidaten, löst aber die Seedzuordnung Sasse versus Hermann/Südring107 nicht. Website technisch kontrolliert: HTTP200, exakt119Bytes HTML mit leerem body, kein Impressum oder Kontaktdetail; nicht als aktive Betreiberwebsite bzw. Schließungsbeleg gewertet. GLEIF ist Identitätsdatenquelle, kein zulässiger Ersatz für unabhängige historische Betreiberkette oder Annahmenachweis. Kein Stammdaten-/GEO-Fill, keine Ankauf-/Verkaufs-/Gebührenliste. Quellen: https://api.gleif.org/api/v1/lei-records/391200DERUQIA2FILR68 ; https://www.northdata.de/Schrott-Sasse%20GmbH%20&%20Co%C2%B7%20KG,%20Rheda-Wiedenbr%C3%BCck/Amtsgericht%20G%C3%BCtersloh%20HRA%205881 ; https://www.schrott-sasse.de/ .]

### Recherche 17.10.2026 — Welle 21:00 B

- [Recherche 17.10.2026: Registerspiegel der Schrott-Sasse KG erneut direkt gelesen: HRA 5881, Lippstädter Str. 61/33378, Abschluss 2024 veröffentlicht 02.03.2026. Neu geprüfte Domain schrott-sasse.de leitet auf www weiter und liefert HTTP 200, aber praktisch leeres HTML mit leerem body, kein lesbares Impressum/Kontakt. Das ist kein verifiziert aktiver Betreiberauftritt und ebenso kein Beweis der Betriebsschließung. Knapper Seed Sasse bleibt gegenüber Hermann/Südring 107 ungeklärt; keine Zuordnung der KG ohne Betreiberkette. Ankauf/Verkauf/Gebühren: keine numerischen Listen abrufbar. GEO: beide alten Adresskandidaten gesperrt bis Seedidentität geklärt. Quellen: https://www.northdata.de/Schrott-Sasse%20GmbH%20&%20Co%C2%B7%20KG,%20Rheda-Wiedenbr%C3%BCck/Amtsgericht%20G%C3%BCtersloh%20HRA%205881 ; https://schrott-sasse.de .]

### Recherche 14.10.2026

- [Recherche 14.10.2026: North-Data-Detail direkt gelesen: Schrott-Sasse GmbH & Co. KG, HRA 5881 Gütersloh, Lippstädter Str. 61, 33378 Rheda-Wiedenbrück; Zweck Schrott-/Metallhandel, Containerdienst, Sammlung/Transport von Abfällen; Abschluss 2024 veröffentlicht 02.03.2026. Creditreform-Suchtreffer ID 4130094982 nennt gleiche KG/Adresse und 05242 5217, nicht separat als gelesene Firmenauskunft gewertet. Gegenrecherche findet Sasse Hermann/H. am Südring 107 mit 05242 577515 in Verzeichnissen. Keine belegte Personen-/Betreiberkette zwischen beiden, kein Namens- oder Adressfill. Ankauf/Verkauf/Gebühren und Öffentlichkeit des Platzes nicht aus Registerzweck ableiten. Quellen: https://www.northdata.com/Schrott-Sasse%20GmbH%20%26%20Co%C2%B7%20KG,%20Rheda-Wiedenbr%C3%BCck/Amtsgericht%20G%C3%BCtersloh%20HRA%205881 ; https://firmeneintrag.creditreform.de/33378/4130094982/SCHROTT_SASSE_GMBH_CO_KG (Suchlead); https://www.schrottplatz-info.de/schrottplatz/Rheda-Wiedenbrueck/Sasse-H- ; https://www.dasoertliche.de/Themen/Sasse-Hermann-Schrott-und-Metalle-Rheda-Wiedenbr%C3%BCck-Wiedenbr%C3%BCck-S%C3%BCdring (nur Leads).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Rheda-Wiedenbrück

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Seed-Zeile **„Sasse“** ist **nicht
  eindeutig** einer der im Dossier dokumentierten Sasse-Betriebe zuzuordnen; die im Register geführte **Schrott-Sasse
  GmbH & Co. KG (HRA 5881)** ist zwar aktiv, belegt aber **weder** eine heutige Schrottannahme **noch** die Identität
  mit dem generischen Seed. **Was live geprüft wurde (04.11.2026):** Das Kachelraster über Rheda-Wiedenbrück
  (8.3000/51.8100 bis 8.3300/51.8400, **9 Kacheln von je 0,01 Grad, 0 Kachel-Fehler**, live abgerufen) erfasst die
  **Lippstädter Straße** mit **70** Adressobjekten und **67** verschiedenen Hausnummern und den Hausnummern 3, 4, 5, 6,
  12, 14, 16, 22, 26, 28, 54, 54a, 54b, 56a, **61**, 65, 68, 68c, 69, 69a, 69b, 70, 71, 71a, 71b, 71c, 71d, 71e, 71f,
  73, 74, 75, 75a, 77, 79, 81, 81a, 83, 83a, 84, 85, 87, 100, 103, 104a, 121, 122, 122a, 124, 125, 125a, 125b, 125c,
  126, 127a, 127b, 127c, 127d, 127e, 127f, 128, 129, 129a, 129b, 129c, 130 und 131; die **Dossier-Hausnummer 61
  existiert** als **way/291288595** mit `addr:housenumber`=61, `addr:street`=Lippstädter Straße,
  `addr:city`=Rheda-Wiedenbrück, `addr:postcode`=33378 und `building`=yes - das Objekt trägt **weder** `name=` **noch**
  `operator=`; eine Namenssuche nach **„Sasse“** und **„Kathöfer“** über die Felder name, operator, description,
  website, contact:website, brand, note und addr:housename findet im Raster **keinen** Treffer. Quelle 1
  (Register/Identität): der im Dossier bereits zitierte **GLEIF-Datensatz** LEI **391200DERUQIA2FILR68** (live am
  26.10.2026 gelesen) führt **Schrott-Sasse GmbH & Co. KG, HRA 5881, Lippstädter Str. 61, 33378 Rheda-Wiedenbrück**,
  entity **ACTIVE**, LEI-Status ISSUED, letzte Aktualisierung 18.04.2026; der **lokale Zeitungsbericht vom 02.01.2020**
  (im Dossier zitiert) belegt dagegen, dass **Hermann Sasses** Betrieb am **Südring** und **Christina Sasses** Betrieb
  an der **Lippstädten Straße** bereits 2019 endeten und beide unter **Hermann Kathöfer** in Langenberg vereinigt wurden
  - die **KG** wird also **nicht** durch diese Zeitungsgeschichte als Nachfolgerin belegt, und umgekehrt ist die KG
  **keine** Fortführung der beiden historischen Betriebe. Quelle 2 (OSM): das live abgerufene Kachelraster
  8.3000/51.8100–8.3300/51.8400 mit way/291288595 **ohne** Betriebsbezug. **Warum keine Koordinate:** Die Seed-Zeile ist
  ein generischer Nachname; die im Dossier ausdrücklich als **historische Betriebsorte** und **nicht zur aktuellen
  Geokodierung freigegeben** bezeichneten Adressen **Lippstädter Str. 61** und **Südring 107** dürfen nicht
  stillschweigend zu einem Betriebsstandort werden, und der existierende OSM-Adressknoten trägt **keinen**
  Betriebsbezug. **Was zur Auflösung fehlt:** die Zuordnung des Seeds zu einem bestimmten Sasse-Betrieb **und** ein
  aktueller Beleg für eine öffentliche Schrottannahme an der dann einschlägigen Anschrift. `clear_fields: 'geo'`
  **bleibt** bestehen, `status: pruefung` bleibt unverändert; es wird weder Fortbestand noch Schließung behauptet.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/291288595/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.3000,51.8100,8.3300,51.8400 ;
  https://api.gleif.org/api/v1/lei-records/391200DERUQIA2FILR68 ;
  https://www.die-glocke.de/kreis-guetersloh/rheda-wiedenbrueck/artikel/wiedervereinigung-im-schrottgewerbe-1624316601
  (Abruf 04.11.2026).]

### Recherche 18.11.2026

- [Recherche 18.11.2026: **Der im Zeitungsbericht vom 02.01.2020 genannte Nachfolger Hermann Kathoefer ist amtlich als zertifizierter Schrottplatz-Operator belegt - damit ist die historische Sasse-Kette erstmals mit Registerbeleg unterlegt, ohne dass der Seed-Satz Sasse dadurch zugeordnet waere.** Das amtliche Fachbetrieberegister fuehrt die Kathoefer Wertstoff GmbH, Hauptsitz Druffeler Strasse 105, 33397 Rietberg, HRB 13300 Registergericht Guetersloh, mit dem EfbV-Zertifikat 226ZEB322 der TUEV NORD Umweltschutz GmbH und Co. KG (Vorgangsnummer ZZBT005001736010, 7 Anlagen, Pruefungsdatum 03.02.2026, Ausstellungsdatum 04.03.2026, gueltig bis zum 28.07.2027); die Anlage 5 ist als Schrottplatz, Lippstaedter Strasse 4, 33449 Langenberg, gefuehrt (Behandeln gekreuzt, Kennnummer nach Paragraf 28 NachwV E754770262, Anlagentechnik Schrottplatz, Lagerung und Behandlung von verunreinigten FE- und NE-Metallen und sonstigen FE-Metallen), die uebrigen fuenf Anlagen liegen ebenfalls in Langenberg. Damit ist der im Zeitungsbericht beschriebene Zusammenschluss der beiden Sasse-Betriebe unter Hermann Kathoefer in Langenberg mit einer zweiten, amtlichen Quelle gestuetzt; das Register nennt jedoch keine Sasse-Flaeche, sodass weder die KG Schrott-Sasse GmbH und Co. KG (HRA 5881) noch deren Standort Lippstaedter Str. 61 als Nachfolger oder Fortsetzung belegt sind. Die Namenssuche Sasse im amtlichen Fachbetrieberegister liefert am 18.11.2026 3 Zeilen, saemtlich den Entsorgungsfachbetrieb A. Markowski Hauptsitz Sassenberg, Einsteinstrasse 3, 48336 Sassenberg (ZZHT00200018 / 1 bis / 3), also einen Namensvetter in einem anderen Ort; die Standortabfrage mit 3 km um Lippstaedter Str. 61, 33378 Rheda-Wiedenbrueck (Nominatim 51.8256075/8.3127114) liefert 5 Zeilen fremder Betriebe (Georg Heuer Behaelterhandel und Industriebedarf GmbH 2,4 km, Peterburs GmbH und Co. KG 2,5 km, PreZero Service Westfalen GmbH und Co. KG 2,9 km). Reichweite: die Efb- und die AfV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis; die Aussage gilt fuer die gepruefte Namensvariante, den 3-km-Radius und beide Registerrouten am Abrufdatum. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&p=ZZBT00500173&fd=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Sasse ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Lippst%C3%A4dter%20Stra%C3%9Fe%2061%2C%2033378%20Rheda-Wiedenbr%C3%BCck%2C%20Deutschland%3B8.3127114%3B51.8256075%3B8.3247114%3B51.8386075%3B8.3007114%3B51.8126075&dst=3&is=1&e=1 ; https://www.die-glocke.de/kreis-guetersloh/rheda-wiedenbrueck/artikel/wiedervereinigung-im-schrottgewerbe-1624316601 (Abruf 18.11.2026).]
- [Recherche 18.11.2026: **Im selben Ort betreibt TSR einen eigenen Schrotthaendler an anderer Anschrift - das ist kein Sasse-Nachfolger und loest die Seed-Zuordnung nicht auf.** Die Betreiberseite TSR fuehrt den Standort Schrotthaendler Rheda-Wiedenbrueck mit TSR Deutschland GmbH und Co. KG, Kupferstrasse 40, 33378 Rheda-Wiedenbrueck, Telefon +49 5242 40808-0, Oeffnungszeiten Mo-Do 07:00-13:00 und 13:30-16:00 und Fr 07:00-13:00 und 13:30-14:45 (live am 18.11.2026 abrufbar). Die Anschrift Kupferstrasse 40 liegt nicht an der Dossier- und Registeradresse Lippstaedter Str. 61; eine Ubernahme, Nachfolge oder Standortverlegung zwischen TSR und der KG Schrott-Sasse ist damit nicht belegt und wird nicht behauptet, die Dossiers bleiben getrennt. Im amtlichen Fachbetrieberegister traegt dieser Standort zudem eine eigene AfV-Anerkennung (BLC-The Battery Lifecycle Company GmbH TSR Ostwestfalen GmbH, Kupferstrasse 40, 33378 Rheda-Wiedenbrueck, Vorgang ZAHS01600044 / 1); diese Anerkennung gehoert zu TSR und wird nicht auf den Seed-Satz Sasse uebertragen; die Namenssuche Sasse in der Route Altfahrzeugverwertung liefert am 18.11.2026 0 Treffer, es gibt also auch unter diesem Namen keine Demontage- oder Annahmeanerkennung. ANKAUF: fuer den Seed Sasse keine Kurse; die TSR-Seite nennt nur faire, tagesaktuelle Preise und eine moegliche Barauszahlung. VERKAUF: keine Preisliste. GEBUEHREN: keine Tarife. Quellen: https://www.tsr-recycling.de/standorte/schrotthaendler-rheda-wiedenbrueck/ ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=TSR (Abruf 18.11.2026).]
- [Recherche 18.11.2026: **Registerstand und Webauftritt der KG heute erneut geprueft: LEI aktiv, Website ohne Inhalt, Verzeichnisangaben nur eine Belegklasse.** Der GLEIF-Datensatz LEI 391200DERUQIA2FILR68 fuehrt am 18.11.2026 weiterhin Schrott-Sasse GmbH und Co. KG, Legal Address Lippstaedter Str. 61, 33378 Rheda-Wiedenbrueck, registriert als HRA 5881, Legal Form 8Z6G, Entity Status ACTIVE, LEI Status ISSUED mit Corroboration Level FULLY_CORROBORATED, Erstregistrierung 12.06.2024, letzte Aktualisierung 18.04.2026, naechste Verlaengerung 12.06.2027. Die Domain schrott-sasse.de antwortet am 18.11.2026 mit HTTP 200, aber mit 119 Bytes leerem HTML ohne Impressum oder Kontaktangaben; das ist weder ein verifizierter Betreiberauftritt noch ein Schliessungsbeleg. Die Verzeichniseintraege (Gelbe Seiten, schrottplatz.org, Das Oertliche, Das Telefonbuch) nennen uebereinstimmend Lippstaedter Str. 61, 33378 Rheda-Wiedenbrueck, 05242 5217, Fax 05242 55374, die Internetadresse schrott-sasse.de und die Adresse info@schrott-sasse.de, stammen aber aus einer Verzeichnisfamilie und sind daher nur ein Einzelbeleg. Offen bleiben die Zuordnung des generischen Seed-Namens Sasse zu einem bestimmten Betrieb, eine oeffentliche Schrottannahme an der Lippstaedter Str. 61 und jede Preisauskunft. Quellen: https://api.gleif.org/api/v1/lei-records/391200DERUQIA2FILR68 ; https://schrott-sasse.de/ ; https://www.gelbeseiten.de/gsbiz/65207850-0dd5-4e18-8869-a9c08fe1e7fe (Abruf 18.11.2026).]
