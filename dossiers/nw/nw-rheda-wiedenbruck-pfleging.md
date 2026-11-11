---
slug: nw-rheda-wiedenbruck-pfleging
name: Berni Pfleging GmbH
trader_type: schrotthaendler
state: NW
city: Rheda-Wiedenbrück
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: geschlossen
description: 'Berni Pfleging GmbH, HRB 6222 Amtsgericht Gütersloh, Fürst-Bentheim-Straße 36, 33378 Rheda-Wiedenbrück. Das Unternehmen handelte historisch mit Eisen- und Stahlschrott und ist **gelöscht**: das amtliche Handelsregister (online-handelsregister.de) führt den Status "gelöscht", der Registerverlauf nennt die Liquidation (Veröffentlichung 03.04.2020) und die Löschung am 24.10.2022. Zwei unabhängige Registerquellen bestätigen die Auflösung; ein Rechtsnachfolger oder eine Fortführung an dieser oder einer anderen Anschrift ist nicht belegt. Fürst-Bentheim-Str. 36 und 05242 401369 bleiben historische Stammdaten.'
clear_fields: 'geo,phone'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Pfleging

## Überblick

Die Berni Pfleging GmbH (HRB 6222 AG Gütersloh) führte historisch den An- und Verkauf von Eisen-/Stahlschrott. Der nun direkt geprüfte Registerspiegel dokumentiert Liquidator Berni Pfleging und eine Veröffentlichung vom 03.04.2020 sowie die formelle Löschung am 24.10.2022. Damit ist der frühere Status `aktiv` nicht haltbar. Mangels Nachweis einer Fortführung oder eines Rechtsnachfolgers wird der Seed auf `pruefung` gesetzt, nicht als Standort geschlossen behauptet. Fürs Unternehmen sind keine aktuellen Ankauf-/Verkaufsbedingungen oder Gebühren nachweisbar. Fürst-Bentheim-Str. 36 und 05242 401369 bleiben historische Stammdaten, nicht aktuelle Kontakt-/Annahmebestätigung. Der gespeicherte GEO-Punkt löst an einer anderen Straße auf und wird mit `clear_fields: 'geo'` beim nächsten regulären Seed zurückgezogen; keine Ersatzkoordinate.

**Statusentscheidung 07.11.2026:** Die Löschung ist inzwischen durch **zwei unabhängige Registerquellen** belegt (NorthData-Registerverlauf und das amtliche Handelsregister via online-handelsregister.de mit dem Status „gelöscht“), nicht mehr nur über einen Registerspiegel. Der eingetragene Rechtsträger existiert nicht mehr; nach der Repo-Definition ist der Eintrag damit `geschlossen` (gelöschter Rechtsträger), **nicht** `pruefung`. Das ist **keine** Aussage über den Standort: die Fläche an der Fürst-Bentheim-Straße 36 kann weiterhin von einem anderen Betrieb genutzt werden, nur ist für diese Zeile kein Nachfolger dokumentiert. `clear_fields: 'geo,phone'` bleibt bestehen, Adresse und Rufnummer bleiben historische Stammdaten und werden nicht als aktuelle Kontakte geführt.

## Timeline

### Owner-Nachtrag 26.10.2026

- [Owner-Nachtrag 26.10.2026: Eine frische read-only-Abfrage der Produktionszeile bestätigt, dass Fürst-Bentheim-Str. 36, 33378 und 05242 401369 noch in den typisierten Kontaktfeldern standen. Das Dossier bezeichnet diese Werte bereits als historische Stammdaten; die heute erneut gelesene NorthData-Chronik weist die Löschung der Berni Pfleging GmbH am 24.10.2022 aus, liefert aber keinen Nachweis eines aktuellen Betriebs oder Rechtsnachfolgers. Damit sind Adresse und Rufnummer nicht als aktuelle Kontaktdaten kuratiert: Straße/PLZ werden beim nächsten geänderten Seed geleert und `clear_fields: 'phone'` entfernt die alte Rufnummer. Dieser Nachtrag ersetzt die im nachfolgenden Owner-Eintrag festgehaltene frühere Entscheidung, die beiden Werte noch in den strukturierten Feldern zu behalten; die historische Spur bleibt in der Timeline erhalten. Dies ist keine Behauptung, dass die Adresse falsch sei, keine Schließung des Standorts und keine Nachfolgezuordnung; `status: pruefung` bleibt. Der GEO-Altpunkt wird weiterhin separat mit `clear_fields: 'geo'` entzogen. Quellen: read-only `public.db`, 26.10.2026; [NorthData-Chronik HRB 6222](https://www.northdata.de/Berni+Pfleging+GmbH,+Rheda-Wiedenbr%C3%BCck/Amtsgericht+G%C3%BCtersloh+HRB+6222); [Seed-Clear-Semantik](../../docs/trader-seed.md).]

### Korrektur 26.10.2026

- [Korrektur 26.10.2026: North-Data-Detail HRB 6222 AG Gütersloh direkt geprüft. Es führt Berni Pfleging GmbH mit historischem Gegenstand An-/Verkauf von Eisen- und Stahlschrott, Liquidator Berni Pfleging und Veröffentlichung vom 03.04.2020; die Gesellschaft wurde am 24.10.2022 formell gelöscht. Damit ist `status: aktiv` nicht mehr gerechtfertigt. Status auf `pruefung` korrigiert, nicht `geschlossen`, da keine aktuelle Prüfung des Betriebsorts oder belegte Nachfolge vorliegt. Beschreibung auf historischen Gegenstand/Löschdatum präzisiert; Straße und Telefon bleiben im Datensatz als nicht aktuelle Leads, werden nicht als aktive Betreiberkontakte dargestellt. Die read-only Produktionszeile enthielt 51.8497, 8.3002; OSM-Rückwärtssuche löst den Punkt an Zum Eidhagen auf, nicht an der historischen Fürst-Bentheim-Str. 36. Die Adresssuche findet den historischen Hausnummern-Gebäudepunkt rund 1,2 km entfernt, aber keine aktuelle Betreiber-/Annahmestelle. Daher `clear_fields: 'geo'`, ohne Ersatzpin; die gelöschte GmbH wird nicht als heutiger Betrieb geokodiert. Ankauf/Verkauf/Gebühren: keine aktuellen Konditionen, Preislisten oder Tarife; keine Nachfolgezuordnung vorgenommen. Quellen: https://www.northdata.de/Berni+Pfleging+GmbH,+Rheda-Wiedenbr%C3%BCck/Amtsgericht+G%C3%BCtersloh+HRB+6222 ; https://nominatim.openstreetmap.org/reverse?lat=51.8497&lon=8.3002&format=jsonv2&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=F%C3%BCrst-Bentheim-Stra%C3%9Fe%2036%2C%2033378%20Rheda-Wiedenbr%C3%BCck%2C%20Germany&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/copyright ; read-only public.db (26.10.2026).]

### Recherche 04.10.2026

- [Recherche 04.10.2026: Erneute Firmen-/Namenssuche ergab ein widersprüchliches CompanyHouse-Profil, das die Berni Pfleging GmbH als aufgelöst ausweist; im Gegensatz dazu stehen die vorhandenen Registerhinweise zu HRB 6222 und die älteren, konsistenten Firmen-/Adress-Leads. Kein aktueller Betreiberauftritt oder Primärregisterbeleg zur Auflösung bzw. Fortführung nachgewiesen. Bestehenden Status `aktiv` und alle vorhandenen Felder nicht überschrieben; die leeren E-Mail-, Öffnungszeiten- und Website-Felder bleiben mangels belastbarer aktueller Betreiberbelege leer; Quelle(n): https://www.companyhouse.de/en/p/Berni-Pfleging-XUYqzWKw8nWOJYJPB8CRZkhFWms (widersprüchlicher Firmenprofil-Lead); https://www.northdata.de/ (frühere Register-Gegenprobe, siehe Recherche 02.10.2026); https://webvalid.de/ (frühere Register-Gegenprobe, siehe Recherche 02.10.2026); Abruf 04.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Rheda-Wiedenbrück

### Recherche 02.10.2026

- [Recherche 02.10.2026: Registerstark — Berni Pfleging GmbH, HRB 6222 (AG Gütersloh; webvalid.de + northdata.de), gegr. 25.01.1996, Gegenstand „An- und Verkauf von Eisen- und Stahlschrott"; Adresse Fürst-Bentheim-Str. 36, 33378 Rheda-Wiedenbrück (firmania.de + recyclingpool.de + city-advisor + bauenwohnen, konsistent); Tel. 05242/401369 (recyclingpool.de; telefonbuchsuche-Personeneintrag Burgweg 66 gleiche Nr. = Firmenbezug zweitbelegt, Burgweg-Adresse als Privat-/Altadresse in Historie). Fills: name (GmbH), street, postcode, phone, description; status → aktiv (Register + Konsistenz). Quelle(n): webvalid.de + northdata.de (Register, stark); recyclingpool.de + firmania.de (Adress-/Tel.-Konsistenz, Leads)]

### Geokodierung 04.11.2026

- [Geokodierung 04.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Adresse **Fürst-Bentheim-Str. 36,
  33378 Rheda-Wiedenbrück** ist in OSM zwar als Gebäude erfasst, sie ist aber die Anschrift einer **gelöschten**
  Gesellschaft; ein aktueller Betrieb, eine Annahme oder eine Nachfolge sind **nicht** belegt. **Was live geprüft wurde
  (04.11.2026):** Das Kachelraster über Rheda-Wiedenbrück (8.2700/51.8400 bis 8.3000/51.8700, **9 Kacheln von je 0,01
  Grad, 0 Kachel-Fehler**, live abgerufen) erfasst die **Fürst-Bentheim-Straße** mit **73** Adressobjekten und **63**
  verschiedenen Hausnummern und den Hausnummern 2, 3, 4, 4a, 5, 6, 6a, 7, 8, 10, 12, 13, 14, 15, 17, 18, 20, 21, 22, 23,
  24, 25, 26, 27-29, 28, 30, 31, 31a, 32, 33, 33a, 34, 35, 35a, 35b, **36**, 37, 37a, 37b, 38, 39, 39a, 40, 41, 43, 43a,
  45, 51, 55, 56, 57, 58, 59, 60, 60a, 61, 62, 63, 65, 65a, 67, 67a und 69; die **Dossier-Hausnummer 36 existiert** als
  **way/251907087** mit `addr:housenumber`=36, `addr:street`=Fürst-Bentheim-Straße, `addr:city`=Rheda-Wiedenbrück,
  `addr:postcode`=33378 und `building`=yes (`source:building`=OpenData Kreis Gütersloh - NAS) - das Objekt trägt
  **weder** `name=` **noch** `operator=`; eine Namenssuche nach **„Pfleging“** über die Felder name, operator,
  description, website, contact:website, brand, note und addr:housename findet im Raster **keinen** Treffer, es gibt
  also **keine** OSM-Betriebszuordnung. Quelle 1 (Register): der im Dossier bereits zitierte **NorthData-Detailbeleg**
  zu **Berni Pfleging GmbH, HRB 6222 AG Gütersloh** dokumentiert den historischen Unternehmensgegenstand „An- und
  Verkauf von Eisen- und Stahlschrott“, den **Liquidator Berni Pfleging**, eine Veröffentlichung vom 03.04.2020 und die
  **formelle Löschung am 24.10.2022**; ein Beleg für einen Rechtsnachfolger oder eine Fortführung existiert **nicht**.
  Quelle 2 (OSM): das live abgerufene Kachelraster 8.2700/51.8400–8.3000/51.8700 mit way/251907087 **ohne**
  Betriebsbezug. **Warum keine Koordinate:** Die zur Seed-Zeile gehörende Gesellschaft ist seit **24.10.2022 gelöscht**;
  ein Punkt an der historischen Anschrift würde für die **Berni Pfleging GmbH** einen heutigen Betriebsstandort
  behaupten, den weder Register noch OSM tragen - das Dossier selbst hält die Adresse für historische Stammdaten und
  nicht für einen Annahmepunkt. **Was zur Auflösung fehlt:** ein Nachweis einer **Fortführung oder Nachfolge** an dieser
  oder einer anderen Anschrift mit belegter Schrottannahme. `clear_fields: 'geo,phone'` **bleibt** bestehen (der
  Altpunkt wird weiterhin zurückgezogen, die historische Rufnummer weiterhin nicht als aktueller Kontakt geführt),
  `status: pruefung` bleibt unverändert; es wird weder Fortbestand noch Schließung behauptet. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/251907087/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=8.2700,51.8400,8.3000,51.8700 ;
  https://www.northdata.de/Berni+Pfleging+GmbH,+Rheda-Wiedenbr%C3%BCck/Amtsgericht+G%C3%BCtersloh+HRB+6222 (Abruf
  04.11.2026).]

### Recherche 07.11.2026

- [Recherche 07.11.2026: **Zweite unabhaengige Registerquelle bestaetigt die Loeschung.** Das amtliche Handelsregister (Amtsgericht Guetersloh) fuehrt die **Berni Pfleging GmbH, HRB 6222**, unter der Anschrift "Fuerst-Bentheim-Strasse 36, 33378 Rheda-Wiedenbrueck" mit dem Registerstatus **"geloescht"**; die Eintragung erfolgte am 15.02.1996. In Verbindung mit dem bereits im Dossier zitierten NorthData-Registerverlauf (Liquidation mit Veroeffentlichung vom 03.04.2020, Liquidator Berni Pfleging, formelle Loeschung am 24.10.2022) liegt damit die Aufloesung des eingetragenen Rechtstraegers durch **zwei unabhaengige Registerquellen** vor; die fruehere Formulierung "unabhaengige Bestaetigung fehlt" ist damit ueberholt. Quellen: https://www.online-handelsregister.de/handelsregisterauszug/nw/Guetersloh/HRB/6222/Berni-Pfleging-GmbH ; https://www.northdata.de/Berni+Pfleging+GmbH,+Rheda-Wiedenbr%C3%BCck/Amtsgericht+G%C3%BCtersloh+HRB+6222 (Abruf 07.11.2026).]
- [Recherche 07.11.2026: **Statuskorrektur `pruefung` zu `geschlossen`, mit Gegenbeleg.** Nach der Repo-Definition ist `geschlossen` genau der Fall, dass der eingetragene Rechtstraeger des Eintrags geloescht ist; das Unternehmen war ein Schrottbetrieb (Gegenstand "An- und Verkauf von Eisen- und Stahlschrott"), es greift also **nicht** die Ausnahme "kein Schrottbetrieb an diesem Eintrag", sondern der Regelfall eines geloeschten Schrottbetriebs. Der Eintrag wird deshalb auf `status: geschlossen` gesetzt, die Beschreibung praezisiert und der Clear-Token `geo,phone` beibehalten (keine Ersatzkoordinate, historische Rufnummer weiterhin kein aktueller Kontakt). **Abgrenzung und Gegenbeleg:** Der Standort wird **nicht** als geschlossen behauptet - die Flaeche an der Fuerst-Bentheim-Strasse 36 kann von einem anderen Betrieb genutzt werden; ein Nachfolger ist nur nicht belegt. Als Gegenbeleg ist zu nennen, dass das Verzeichnis 11880 den Betrieb weiterhin unter "Berni Pfleging GmbH, Fuerst-Bentheim-Str. 36, 33378 Rheda-Wiedenbrueck" fuehrt (Aggregator, veraltet) und dass die Fruehwelle vom 04.10.2026 ein widerspruechliches CompanyHouse-Profil notiert hatte; die CompanyHouse-Seite ist am 07.11.2026 erneut nicht abrufbar (HTTP 403, Cloudflare-Challenge). **Naechster Schritt fuer den Owner:** pruefen, ob an der Fuerst-Bentheim-Strasse 36 ein Nachfolger taetig ist; dieser gehoert in ein eigenes Dossier, nicht in diese Zeile. Quellen: https://www.11880.com/branchenbuch/rheda-wiedenbrueck/060440090B100458032/berni-pfleging-gmbh ; https://www.companyhouse.de/Berni-Pfleging-GmbH-Rheda-Wiedenbrueck (403, nicht gelesen) (Abruf 07.11.2026).]
- [Recherche 07.11.2026: **Kein Registereintrag und keine Zertifizierung; Umfeld gemessen.** Die Namenssuchen im amtlichen Fachbetrieberegister liefern am 07.11.2026 fuer "Pfleging" und "Berni Pfleging" in beiden Routen jeweils **0 Treffer**. Die Radiusabfrage um die historische Anschrift Fuerst-Bentheim-Strasse 36, 33378 Rheda-Wiedenbrueck liefert in 3 km **4** EfbV-Eintraege (PreZero Service Westfalen GmbH & Co. KG mit Pilgerpatt 3 in drei Anlagen, Peterburs GmbH & Co. KG mit Am Nonenplatz 1) und in 10 km 39 Eintraege, die Route Altfahrzeugverwertung in 3 km 0 und in 10 km 1 (BLC - The Battery Lifecycle Company GmbH / TSR Ostwestfalen GmbH, Kupferstrasse 40) - **keiner** davon nennt Pfleging. `certifications` bleibt leer; Reichweite: EfbV- und AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein Nachweis fehlender Anerkennung und kein eigenstaendiger Schliessungsbeleg. ANKAUF, VERKAUF und GEBUEHREN des geloeschten Unternehmens sind nicht belegbar; es gab und gibt keine Preisliste. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Pfleging ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Berni%20Pfleging ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=F%C3%BCrst-Bentheim-Stra%C3%9Fe%2036%2C%2033378%20Rheda-Wiedenbr%C3%BCck%2C%20Deutschland;8.282619;51.850443;8.302619;51.870443;8.262619;51.830443&dst=3&is=1&e=1 (Abruf 07.11.2026).]

### Recherche 11.11.2026

- [Recherche 11.11.2026: **Die Löschung ist amtlich bestätigt, und an der ehemaligen Anschrift ist kein zertifizierter Betrieb und kein registrierter Nachfolger verzeichnet.** Das amtliche Handelsregisterportal führt „Berni Pfleging GmbH“, Amtsgericht Gütersloh, **HRB 6222**, Status **gelöscht**, letzte Anschrift **Fürst-Bentheim-Straße 36, 33378 Rheda-Wiedenbrück**, Gründung 15.02.1996; die im Dossier dokumentierte Löschung ist damit amtlich und nicht nur über einen Registerspiegel belegt.
  Im amtlichen Fachbetrieberegister (Filter „Art des Betriebs = Egal“) liefern die Namenssuchen „Pfleging“ und „Berni Pfleging“ **je 0 Treffer** in beiden Routen; die Standortsuche um **Fürst-Bentheim-Straße 36, 33378 Rheda-Wiedenbrück** liefert im 3-km-Radius **4 Efb-Treffer** (PreZero Service Westfalen GmbH & Co. KG, Pilgerpatt 3, 2,5 km; Peterburs GmbH & Co. KG, Am Nonenplatz 1, 2,8 km) und im 10-km-Radius **39 Treffer**, die Route „Altfahrzeugverwertung“ liefert im 3-km-Radius **0 Treffer**: **kein** zertifizierter Betrieb an der Hausnummer 36 und **kein** registrierter Nachfolger.
  **Ankauf:** keine Konditionen und keine Preise; **Verkauf:** keine Liste; **Gebühren:** keine Angaben. `status` bleibt `geschlossen` (gelöschter Rechtsträger), `clear_fields: 'geo,phone'` bleibt bestehen, Adresse und Rufnummer bleiben historische Stammdaten; **offen** bleibt, ob die Fläche von einem anderen Betrieb genutzt wird, das ist eine Flächenfrage und keine Aussage über den gelöschten Rechtsträger. Quelle(n): https://www.online-handelsregister.de/handelsregisterauszug/nw/Guetersloh/HRB/6222/Berni-Pfleging-GmbH ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=F%C3%BCrst-Bentheim-Stra%C3%9Fe%2036%2C%2033378%20Rheda-Wiedenbr%C3%BCck…&dst=3&is=1&e=1 (Abruf 11.11.2026).]
