---
slug: bb-lauchhammer-wsh-schrotthandel
name: WSH Schrotthandel
trader_type: schrotthaendler
state: BB
city: Lauchhammer
street: Bockwitzer Str. 97
postcode: 01979
lat: 51.4859757
lon: 13.7486102
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# WSH Schrotthandel

## Überblick

Für „WSH Schrotthandel“ liegt ein konkreter lokaler Verzeichnislead vor: Das im Review vom 17.10.2026 direkt gelesene 11880-Profil nennt Bockwitzer Str. 97, 01979 Lauchhammer und 01511 5603024. Die alten erfolglosen Verzeichnissuchen sind damit kein aktueller Negativbeleg; das Profil ist aber auch kein Nachweis für Betreiberidentität, laufenden Betrieb oder öffentliche Annahme. Die Westpfälzische Schrotthandels GmbH in Kindsbach ist weiterhin nicht mit diesem Lead verknüpft. Status `pruefung`; keine Schließung oder Aktivierung und keine Kontaktfüllung aus dem Portal.

**Preise:** Keine belegten Ankaufspreise, Verkaufsangebote oder Entsorgungs-/Containergebühren.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Ownerkorrektur 17.10.2026

- [Korrektur 17.10.2026: Die früheren Suchläufe ohne WSH-Treffer bleiben als Historie erhalten, sind jedoch durch den Direktabruf im Review vom 17.10. als aktueller Negativbefund überholt. Das konkrete 11880-Profil nennt WSH Schrotthandel, Bockwitzer Str. 97, 01979 Lauchhammer, 01511 5603024, Kategorien Autoverschrottung/Autoverwertung; Seitenangaben: aktualisiert 04.10.2026, Eintragsdaten 11.04.2026. Das sind Portalmetadaten, kein belegtes Betriebsdatum. Die im Review gelesene Kindsbacher Betreiberseite gehört zur Westpfälzischen Schrotthandels GmbH, ohne belegte Verbindung nach Lauchhammer. Kein neuer Live-Abruf im Korrekturschritt, keine Telefonnummer in Frontmatter übertragen. Betreiber, aktueller Betrieb und öffentlicher Annahmepunkt offen; keine lokalen Zahlenpreise verifiziert. Quelle(n): https://www.11880.com/branchenbuch/lauchhammer/060440459B113497545/wsh-schrotthandel.html (im Review direkt gelesen, nur Lead); https://wsh-recycling.de/ (im Review direkt gelesen, Betreibergegenprobe Kindsbach).]

### Recherche 13.10.2026

- [Recherche 13.10.2026: Erneute Gegenprüfung ergab keinen Betreiber- oder Registerbeleg für WSH in Lauchhammer; die in früheren lokalen Branchenabfragen fehlenden Treffer bleiben ein Negativbefund, kein Schließungsnachweis. Die Verwechslungsgefahr mit der Westpfälzischen Schrotthandels GmbH in Kindsbach besteht fort; deren Website/Adresse nicht übertragen. Frontmatter außer dem bereits gesetzten `website_status: unbekannt` unverändert. ANKAUF/VERKAUF/GEBÜHREN: keine Quelle und keine Preise für Lauchhammer belegt. GEO: Bockwitzer Str. 97 ist lediglich Seed-/Verzeichnisangabe, daher kein Pin. Quelle(n): https://wsh-recycling.de/ (anderer Betrieb, Kindsbach); frühere Verzeichnissuchen Gelbe Seiten und 11880 vom 02.10.2026 (kein WSH-Treffer).]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (eigene Adresse ≠ Dymke — separater Eintrag)
- Adresse: Lauchhammer, Bockwitzer Str. 97
- Adressbeleg: Seed-notes (Verzeichnis); PLZ Orts-PLZ

### Recherche 01.10.2026

- [Recherche 01.10.2026: KEINE Belege — zu "WSH Schrotthandel, Bockwitzer Str. 97, 01979 Lauchhammer" kein Betreiber-/Register-Beleg gefunden; einziger WSH-Treffer im Schrottkontext ist Westpfälzische Schrotthandels GmbH Kindsbach (wsh-recycling.de, Kaiserstr. 168, 66862 Kindsbach — anderes Unternehmen, keine Lauchhammer-Filiale belegt). Verwechslungsrisiko beachten. NICHTS übernommen. Status bleibt pruefung, Klärfall: Existenz unbestätigt; Quelle(n): keine (Gegenbeleg wsh-recycling.de für Kindsbach).]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall BESTÄTIGT — Gelbe Seiten (Suche schrotthandel/lauchhammer direkt abgerufen) listet in Lauchhammer nur Wertstoffhandel Schwarzenberger (Finsterwalder Str. 45) und Dymke Timo Schrotthandel (Dolsthaidaer Str. 45), KEIN WSH; 11880-Umkreissuche ebenso ohne WSH-Treffer. Befund vom 01.10.2026 bleibt: NICHTS übernommen, status pruefung. Fix: website_status (leer/ungültig) → unbekannt (Enum-Validierung). Quelle(n): keine (Gegenbelege gelbeseiten.de, 11880.com).]

### Geokodierung 08.11.2026

- [Geokodierung 08.11.2026: **Koordinate der Dossier-Adresse belegt** (`lat`/`lon` = **51.4859757/13.7486102**). Punktgüte: **Hausnummer-Treffer**; der Dossierbetrieb „WSH Schrotthandel“ ist über die Adresse **Bockwitzer Str. 97, 01979 Lauchhammer** adressiert; das OSM-Adressobjekt **w/397282696** trägt `addr:housenumber`=97, `addr:street`=Bockwitzer Straße, `addr:postcode`=01979, `addr:city`=Lauchhammer; das Objekt trägt **weder `name=` noch `operator=`** - die Koordinate belegt die **Anschrift**, nicht die Betreiberzuordnung; der Punkt ist der Mittelwert der 8 eindeutigen Way-Knoten; die Nominatim-Vorwärtssuche nach „Bockwitzer Str. 97, 01979 Lauchhammer“ liefert unter den 1 Treffern dasselbe Objekt **w/397282696**; die Dossiers `bb-lauchhammer-schwarzenberger-kerstin-wertstoffhandel` und `bb-lauchhammer-wsh-schrotthandel` teilen sich dieselbe Anschrift, jeder Dossierbetrieb ist für sich genommen nicht in OSM benannt.]
  Quelle(n): https://nominatim.openstreetmap.org/search?format=jsonv2&extratags=1&namedetails=1&addressdetails=1&countrycodes=de&limit=20&q=Bockwitzer+Str.+97%2C+01979+Lauchhammer ; https://api.openstreetmap.org/api/0.6/way/397282696.json (Abruf 08.11.2026).]

### Recherche 18.11.2026

- [Recherche 18.11.2026: **WSH Schrotthandel und Wertstoffhandel Schwarzenberger sind mit hoher Wahrscheinlichkeit derselbe Betrieb - die beiden Lauchhammer-Dossiers bilden sehr wahrscheinlich eine Doppelzeile (Befund, kein Merge).** Der amtliche Registerabgleich und die Betreiber-Primärquelle schliessen die Lücke: Das archivierte Impressum von https://www.wertstoffhandel-schwarzenberger.de/impressum (Internet-Archiv-Snapshot 12.08.2024) nennt die Fa. Wertstoffhandel Schwarzenberger, Inh. Sascha Schwarzenberger, Bockwitzer Straße 97, 01979 Lauchhammer/Grünewalde, Telefon +49 151-15603024, Mail wertstoffhandel@mail.de, USt-ID 286042575, Steuernummer 057/272/00416; das Geschwisterdossier bb-lauchhammer-schwarzenberger-kerstin-wertstoffhandel dokumentiert dieselbe Firma. Das im Dossier vom 17.10.2026 gelesene 11880-Profil WSH Schrotthandel nennt Bockwitzer Str. 97, 01979 Lauchhammer und die Mobilnummer 01511 5603024 - das ist dieselbe Nummer wie +49 151-15603024 in 15er-Notation (Anschrift und Mobilnummer stimmen überein), und die Abkürzung WSH lässt sich als Wertstoffhandel Schwarzenberger lesen. Für WSH selbst wurde **kein** Betreiberauftritt gefunden; die im Dossier beschriebene Verwechslungsgefahr mit der Westpfälzischen Schrotthandels GmbH (Kindsbach, wsh-recycling.de) bleibt davon unberührt - die Kindsbacher Firma ist ein anderer Rechtsträger und wird **nicht** übernommen. **Amtliches Fachbetrieberegister (live am 18.11.2026, beide Routen, Filter Art des Betriebs = Egal):** die Namenssuche WSH Schrotthandel liefert 0 Treffer in beiden Routen; die Namenssuche WSH liefert 12 Efb-Treffer, die sämtlich keine WSH-Schrotthandlung in Lauchhammer betreffen (team orange - Abfallwirtschaftsbetrieb des Landkreises Würzburg mit Wertstoff-Sammelhallen WSH Bärental, WSH Aalbachtal, WSH Südliches Maintal, WSH Wachtelberg, WSH Schleehof, WSH Matzenhecke, WSH Klingholz, WSH Kiesäcker; Abfallwirtschaft Südholstein GmbH (AWSH), Leineweberring 13, 21493 Elmenhorst; Kilb Vetter Entsorgung GmbH, Halle Außen WSH, Zeilsheimer Weg 4, 65779 Kelkheim) - die Buchstabenfolge WSH wird dort also als Baustein geführt, ein Treffer in Lauchhammer existiert nicht. Die Standortabfrage mit 3 km um Bockwitzer Straße 97 liefert 5 Efb-Zeilen (Scholz Recycling GmbH Betrieb Lauchhammer, Johann-Friedrich-Trautscholdt-Straße 1, 1,7 km; Rubin GmbH, Patschenweg 10, 2,0 km; Integrationswerkstätten gGmbH Niederlausitz, Am Gewerbehof 6, 2,8 km) und 0 AfV-Zeilen, mit 10 km um 01979 Lauchhammer 40 Efb- und 0 AfV-Zeilen - **keine Zeile nennt WSH**. Reichweite: die Anerkennungen sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis und kein Schließungsbeleg. Im amtlichen Handelsregisterportal (online-handelsregister.de, Abruf 18.11.2026) liefert die Namenssuche Wertstoffhandel Schwarzenberger keine Treffer (Reichweite: Ein-Personen- und GbR-Betriebe erscheinen dort nicht). **ANKAUF:** für die Buchstabenbezeichnung WSH ist keine eigene Preis- oder Leistungsangabe belegt; die Leistungsliste des Betreibers (Buntmetall, Schrott, Altpapier, Rückbau/Entkernung, Transporte und Containerdienst) und die Öffnungszeiten stammen aus dem Geschwisterdossier und werden hier nicht als eigene Konditionen dupliziert; **VERKAUF:** keine Preisliste; **GEBÜHREN:** keine Tarife. **Frontmatter unverändert:** name bleibt WSH Schrotthandel, street (Bockwitzer Str. 97), postcode (01979), lat/lon, website_status (unbekannt) und alle übrigen Felder bleiben unverändert, phone, email, website bleiben leer, status bleibt pruefung; **ein Merge der beiden Zeilen wird nicht vorgenommen** - die Deduplizierung ist eine Owner-Entscheidung. Quelle(n): http://web.archive.org/web/20240812182330/https://www.wertstoffhandel-schwarzenberger.de/impressum ; https://www.11880.com/branchenbuch/lauchhammer/060440459B113497545/wsh-schrotthandel.html ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=WSH ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Bockwitzer%20Stra%C3%9Fe%2097%2C%2001979%20Lauchhammer%2C%20Deutschland%3B13.7486809%3B51.4859883%3B13.799...%3B51.504...%3B13.697...%3B51.467...&dst=3&is=1&e=1 (Abruf 18.11.2026)]
