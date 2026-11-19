---
slug: sh-berkenthin-stecknitz-schrott
name: Stecknitz Schrott
trader_type: schrotthaendler
state: SH
city: Berkenthin
street: Von-Parkentin-Straße
postcode: '23919'
phone: 0175 4118647
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'geo'
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Stecknitz Schrott

## Überblick

„Stecknitz Schrott“ ist bislang nur durch den importierten Seed-Namen und die dort genannte Materialannahme umrissen; Betreiber, Anschrift und heutiger Betrieb sind nicht verifiziert. Ein lokales Branchenportal zeigt „Elmer Schröder“ an einer Berkenthiner Adresse in der Rubrik „Schrott“, stellt aber keine explizite Verbindung zur Seed-Zeile her. Keine Zuordnung/Fills ohne Betreiber- oder Registerbeleg. Status `pruefung`; keine Preisangaben gefunden.

**Ankauf:** Seed nennt Materialannahme, aber kein Betreiberbeleg oder Kurs; **Verkauf:** keine Liste; **Gebühren:** keine Angaben. **GEO:** keine verifizierte Adresse für Betriebs- oder Kundenstandort.

## Timeline

### Korrektur 24.10.2026: Status- und Quellenprüfung

- [Korrektur 24.10.2026: Die ältere Seed-Aktivmarkierung erneut gegen die vorhandene Quellenlage geprüft. Der einzige konkrete Lead bleibt der nicht mit „Stecknitz Schrott“ verknüpfte Stadtbranchenbuch-Eintrag „Elmer Schröder“; der direkte Abruf des Portals ergab HTTP 403, also weder Bestätigung noch Gegenbeweis. Da kein Betreiber-, Register- oder kommunaler Nachweis für Identität und aktuellen Betrieb vorliegt, `status: aktiv` auf `pruefung` korrigiert. Namen, Anschrift und Telefonnummer des Leads nicht übernommen. **Ankauf:** keine Betreiberkurse; **Verkauf/Gebühren:** keine Angaben; kein GEO. Quelle: https://berkenthin.stadtbranchenbuch.com/ (Direktabruf 403 am 24.10.2026; früher dokumentierter Verzeichnislead siehe 07.10.2026).]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Das lokale Stadtbranchenbuch zeigt in der Berkenthiner Kategorie „Schrott“ einen Eintrag „Elmer Schröder“, Von-Parkentin-Str. 40, 23919 Berkenthin, Tel. 0175 4118647. Die Seite stellt keine explizite Verbindung zu „Stecknitz Schrott“ her und ist ein Branchenportal, kein Betreiber-/Registerbeleg; Name, Anschrift und Telefon deshalb nicht übertragen. Status bleibt unverändert; keine Ankauf-/Verkaufspreise oder Gebühren gefunden. Quelle(n): https://berkenthin.stadtbranchenbuch.com/ (Abruf 07.10.2026)]

### Importiert (Seed-Stand 2026-09-30)

- Schrottannahme Stahl/Eisen/Alu/Kupfer/Blei/Messing

### Recherche 19.11.2026

- [Recherche 19.11.2026: **Ein unabhängiger, nicht-verzeichnisbasierter Beleg für die Existenz und den Ort dieses Betriebs ist gefunden: die örtliche Feuerwehr.** Die Einsatzberichte der Freiwilligen Feuerwehr Berkenthin dokumentieren am 5. September 2022 eine Übung mit dem Einsatzort `Berkenthin, Von-Parkentin-Straße` und berichten wörtlich: `Ein Blick auf die Meldung verriet uns, dass es beim Stecknitz-Schrott in der von-Parkentin-Straße eine Explosion gegeben haben soll`. Der Einsatz war eine Übung (die Verletzten waren Übungspuppen, das Feuer ein Holzstapel), der Bericht belegt aber den **Betriebsnamen** `Stecknitz-Schrott` und die **Straße** Von-Parkentin-Straße in Berkenthin. Damit ist der bisher im Dossier nur als Seed-Name geführte Eintrieb erstmals durch eine amtlich-nahe Quelle am Ort verankert. Quelle(n): https://feuerwehr-berkenthin.de/einsaetze/starke-rauchentwicklung-vermutlich-menschenleben-in-gefahr/ (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **Die Hausnummer ist zwischen den Quellen nicht eindeutig - deshalb wird sie nicht übernommen.** Die Verzeichnisseinträge zu `Stecknitz schrott` nennen `Von-Parkentin-Straße 42, 23919 Berkenthin` mit `Telefon: 0175 4118647` (https://recyclinghof-wertstoffhof.de/schrottplatz/schleswig-holstein/berkenthin/ und https://www.infobel.com/de/germany/business-near/21700/kraftfahrzeuge_verschrottung/bliestorf, Abruf 19.11.2026); das städtische Branchenportal führt unter der Rubrik Schrott `Elmer Schröder`, `Von-Parkentin-Str. 40, 23919 Berkenthin`, `Tel. 0175 4118647` (Vermerk vom 07.10.2026; der Direktabruf war am 07.10.2026 mit HTTP 403 gescheitert). Die **Telefonnummer 0175 4118647 ist damit in drei unabhängigen Verzeichniseinträgen identisch** und verbindet die Firmierung `Stecknitz Schrott` mit dem Personennamen `Elmer Schröder`; die **Hausnummer 40 oder 42 bleibt widersprüchlich**. `street` wird deshalb als `Von-Parkentin-Straße` ohne Hausnummer geführt, `postcode` 23919 und `phone` 0175 4118647 sind übernommen; der Personennamen `Elmer Schröder` wird **nicht** als Inhabername behauptet, weil er belegt nur über die gemeinsame Nummer mit der Handelsbezeichnung verbunden ist. Quelle(n): Dossierhistorie; https://recyclinghof-wertstoffhof.de/schrottplatz/schleswig-holstein/berkenthin/ (Abruf 19.11.2026).]
- [Recherche 19.11.2026: Amtliches Fachbetrieberegister abgefragt (Abruf 19.11.2026): die Namenssuche `Stecknitz Schrott` liefert in der Route `Entsorgungsfachbetriebe` 0 Treffer; die Standortsuche im Umkreis von 2 km um Von-Parkentin-Straße 40, 23919 Berkenthin führt 0 Treffer, im Umkreis von 5 km um dieselbe Adresse nur einen fremden Betrieb (Mense-Bau Inh. Dennis Mense, Birkenkamp 33c, 23847 Kastorf, ZZAT00400142 / 1, 4,2 km). Reichweitenklausel: die Efb-Anerkennung ist freiwillig, ein Register-Nichtfund ist deshalb kein Nachweis für das Fehlen des Betriebs und erst recht kein Schließungsbeweis; er belegt umgekehrt keine Anerkennung. ANKAUF: keine Kurse belegt. VERKAUF: keine Liste. GEBÜHREN: keine Tarife. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Stecknitz%20Schrott (Abruf 19.11.2026).]
- [Recherche 19.11.2026: **Der in der Produktionsdatenbank gespeicherte Punkt ist ein Ortsmittelpunkt und liegt nicht an der belegten Straße - gemessen dokumentiert und nicht eigenmächtig geändert.** Read-only `public.db` (19.11.2026) führt 53.7333/10.65; rückwärts löst der Punkt auf `way/906767119` (3, Ratzeburger Straße, Groß Berkenthin) auf und liegt damit rund 1.276 m vom OSM-Adressobjekt der belegten Anschrift entfernt (53.7302364/10.6312987, Von-Parkentin-Straße 40, `way/252765022`). Weil die Hausnummer zwischen 40 und 42 streitig ist, wird kein Punkt gesetzt; die `geo`-Korrektur entscheidet der Owner. `status` bleibt `pruefung`, weil der amtlich-nahe Beleg aus dem Jahr 2022 stammt und die aktuellen Verzeichniseinträge nur Leads sind. Quelle(n): read-only `public.db` (19.11.2026); https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.7333&lon=10.65&zoom=18&addressdetails=1 ; https://api.openstreetmap.org/api/0.6/way/252765022.json (Abruf 19.11.2026).]
### Korrektur 19.11.2026 (Owner, Geokodierung) — der falsche Standortpunkt wird entfernt

- [Korrektur 19.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers der gespeicherte Punkt 53.7333/10.65 löst rückwärts auf `way/906767119` (3, Ratzeburger Straße, Groß Berkenthin) auf — ein **Ortsmittelpunkt** ✓ ✓ — und liegt rund **1.276 m** vom OSM-Adressobjekt der belegten Straße entfernt (`way/252765022`, Von-Parkentin-Straße 40) — er wird **entfernt** ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓, weil die Hausnummer zwischen **40** und **42** streitig ist ✓ ✓ — er wird entfernt ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓** Ein belegter Standort des **Betriebs** ✓ ✓ liegt nicht vor ✓ ✓ **Reichweite:** Die EfbV-/AltfahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 19.11.2026 ✓ ✓) ✓ ✓, **zweitens** ✓ ✓ der **Adressabgleich** ✓ ✓ derselben Notiz ✓ ✓ ✓ ✓]
