---
slug: nw-munster-alpha-recycling
name: Alpha Recycling
trader_type: mobil
state: NW
city: Münster
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
clear_fields: 'geo'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Alpha Recycling

## Überblick

Alpha Recycling bleibt ein mobiler Münsteraner Identitätsklärfall. Vorrecherche: Im Moorhock 63, 48159 Münster und 0171 4069639 sind Verzeichnis-Leads; das damalige Social-Profil liefert keine adresskongruente unabhängige Bestätigung. Die Kandidatendomain ist aktuell technisch nicht auswertbar, Instagram liefert nur eine Seitenschale. Nicht mit ALFA Recycling München oder US-amerikanischen Alpha-Recycling-Betrieben verknüpfen. Ankauf nicht aktuell bestätigt; keine zuordenbaren Ankaufspreise, Verkaufspreise oder Abholgebühren. Vor GEO zuerst Betreiber und Betriebs-/Annahmeadresse klären.

## Timeline

### Recherche 23.10.2026

- [Recherche 23.10.2026: Erneute Suche nach Alpha Recycling Münster/Im Moorhock 63 ergab weiterhin vorwiegend Verzeichnis-/Kartenprofile; diese sind keine neuen unabhängigen Betreiberbelege. recycling-alpha.de lieferte beim Direktabruf einen Transportfehler; der direkte Instagram-Abruf zeigte nur die Login-/Seitenschale, keine auswertbaren Betreiber-, Kontakt- oder Adressangaben. Ein US-amerikanischer Alpha-Recycling-Anbieter ist nicht zugeordnet. Keine Frontmatter-Änderung, keine GEO-Freigabe. **ANKAUF:** keine verifizierte Preisliste; **VERKAUF:** keine Liste; **GEBÜHREN:** keine bestätigten Abhol-/Containerkosten. Quellen/Abrufgrenzen: https://recycling-alpha.de/ ; https://www.instagram.com/alpha_recycling/ .]

### Recherche 15.10.2026 Welle18A

- [Recherche 15.10.2026 Welle18A: recycling-alpha.de direkt mit Transportfehler; Instagram-Profil per HTTP 200, aber nur generischer Instagram-Text ohne auswertbare Profil-/Adressangaben. Kein neuer positiver oder negativer Tätigkeitsnachweis. Gezielte Namens-/Ortssuche lieferte vor allem fremde ALFA-/US-Alpha-Betriebe; deren Kontakte und Ankaufangebote nicht zugeordnet. Historischer Social-Befund vom 01.10. bleibt historisch, kein heutiger Doppelbeleg. Keine Stammdatenfills, status pruefung; Ankauf/Verkauf/Gebühren nicht verifiziert. GeoBedarf: Im Moorhock 63 zunächst als aktuelle Betreiber- und ggf. Annahmeadresse bestätigen, nicht automatisch pinnen. Quellen/Abrufgrenzen: https://recycling-alpha.de/ ; https://www.instagram.com/alpha_recycling/ ; Suchlauf Alpha Recycling Münster/recycling-alpha.de.]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Münster

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — 1 Verzeichnis-Lead + Betreiber-Social ohne Adresskongruenz: Alpha-Recycling, Im Moorhock 63, 48159 Münster, Tel. 0171 4069639, Domain-Angabe recycling-alpha.de (Lead, Abruf nur Bilddaten ohne verwertbaren Inhalt); Instagram @alpha_recycling (Betreiber-Social, Metallrecycling/Containerdienst Münster) bestätigt Identität, aber keine Adresse; kein 2. Adress-Beleg; kein Frontmatter-Fill; Quelle(n): Lead gelbeseiten.de + Betreiber-Instagram]

### Recherche 23.11.2026 (Welle 0300 A)

- [Recherche 23.11.2026: **Die Kandidaten-Domain lebt nur noch als Weiterleitung auf ein gelöschtes Instagram-Profil, und das Instagram-Profil selbst ist entfernt - damit ist die im Dossier vom 01.10.2026 dokumentierte „Betreiber-Social-Quelle“ nicht mehr abrufbar.** Live am 23.11.2026: (1) Der Abruf von `https://recycling-alpha.de/` über HTTPS schlägt mit **`net::ERR_CERT_COMMON_NAME_INVALID`** fehl - das TLS-Zertifikat lautet nach Prüfung mit `openssl s_client` auf `subject=CN=*.kasserver.com` (Aussteller Sectigo Public Server Authentication CA DV E36, gültig 27.10.2026 bis 13.05.2027) und passt **nicht** zur Domain `recycling-alpha.de`; die Domain löst zwar auf **85.13.162.216** auf, das Zertifikat ist aber für eine **andere** Domain ausgestellt (Hoster-Kasserver). (2) Über **HTTP** antwortet die Domain mit **HTTP 301 Moved Permanently** und `Location: https://www.instagram.com/alpha_recycling` - die Domain ist damit **keine Betreiberseite mehr**, sondern eine **nackte Weiterleitung**. (3) Der Direktabruf von `https://www.instagram.com/alpha_recycling/` (headless Browser, 10 Sekunden Wartezeit) liefert die Instagram-Seite mit dem Text **„Page ist nicht verfügbar. Möglicherweise funktioniert der Link nicht oder das Profil wurde entfernt.“** - das Profil ist **entfernt**. Der im Dossier vom 01.10.2026 als „Betreiber-Social“ verbuchte Beleg (@alpha_recycling, „Metallrecycling/Containerdienst Münster“) ist damit **nicht mehr abrufbar** und trägt heute **keinen** Bestätigungswert mehr; die historische Notiz bleibt erhalten, wird aber nicht als heutiger Doppelbeleg geführt. (4) Die Wayback-Abfrage für `recycling-alpha.de` liefert am 23.11.2026 **keinen** Snapshot (`archived_snapshots: {}`). **Verzeichnisstand:** die 11880-Suche „Schrotthändler Münster“ (live am 23.11.2026) führt den Betrieb weiterhin als **„Alpha Recycling“**, Branche **Schrotthandel**, **Im Moorhock 63, 48159 Münster (Kinderhaus-Ost)**, **(0171) 4069639**, **keine Öffnungszeiten**, **keine** Bewertungen; die zugehörige 11880-Firmenseite (live am 23.11.2026, HTTP 200) nennt dieselbe Anschrift und Nummer, bietet **„Website hinzufügen“** an (es ist also **keine** Website hinterlegt) und nennt als Kontaktmöglichkeit **0171 4069639** sowie die E-Mail-Adresse **`info@recycling-alpha.de`** - die damit erstmals als Portalangabe sichtbar wird und die frühere Domain-Angabe stützt, ohne eine Betreiberquelle zu sein. Die 11880-Seite weist ausdrücklich darauf hin, dass die Preisspanne nur angezeigt würde, „sofern uns diese Information vorliegt“ - es wird **keine** angezeigt. **Amtliches Fachbetrieberegister (E-15) am 23.11.2026 geprüft** (beide Routen, Filter „Art des Betriebs = Egal“ `is=1&e=1`): die Namenssuche „Alpha Recycling“ liefert in der Route „Entsorgungsfachbetriebe“ **3 Treffer** - **AKR Alpha Kunststoffrecycling GmbH**, Thenn 18, 85456 Wartenberg, **ZZHT01000319 / 1** (Handeln, Makeln) und **/ 2** (Lagern, Behandeln, Verwerten, Beseitigen) - sowie **AWR Alpha Wertstoff & Recycling GmbH**, Thenn 18, 85456 Wartenberg, **ZZEE00100414 / 1** (Handeln, Makeln); in der Route „Altfahrzeugverwertung“ **0 Treffer**. Die Standortsuche um Im Moorhock 63, 48159 Münster (Nominatim-Punkt 51.9884996/7.6046644) im **3-km-Radius** liefert in **beiden** Routen **0 Treffer**. **Beide Registertreffer sind fremde Rechtsträger in Wartenberg (Landkreis Erding), keine Münsterer Betriebe** und **kein** Bezug zu „Alpha Recycling“ in Münster; sie werden **nicht** übernommen. Die Namenssuche „Alpha“ liefert in der Route „Entsorgungsfachbetriebe“ **6 Treffer**, darunter die beiden genannten - ebenfalls **kein** Münsterer Treffer. **Amtliches Handelsregisterportal** (online-handelsregister.de, live am 23.11.2026): die Suche nach „Alpha Recycling Münster“ liefert **keine Rechtseinheit und keine Person**; bei einem nicht registerpflichtigen Einzelbetrieb ist das ein Negativbefund ohne Beweiswert. **Reichweite der Negativaussage:** die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass ein Betrieb keine Anerkennung besitzt, und **keine** Schließung. `certifications` bleibt leer. **Ankauf:** keine belegten Kurse und keine Preisliste. **Verkauf:** keine Liste. **Gebühren:** keine Abhol-/Containerkosten. **GEO:** Geokodierungsbedarf; die belegte Kandidatenadresse Im Moorhock 63, 48159 Münster ist in OSM das Adressobjekt **way/239486959** (`addr:street`=Im Moorhock, `addr:housenumber`=**63**, `addr:postcode`=48159, `addr:city`=Münster, `building`=yes) - **die Hausnummer 63 steht am Objekt way/239486959**; das Objekt trägt **weder `name` noch `operator`**. Der in der Produktion gespeicherte Punkt **51.6992941176471/8.03888235294118** ist **unverändert und falsch**: der OSM-Rückweg (Nominatim, zoom 18) löst ihn auf „**Frölichweg, Lippborg, Lippetal, Kreis Soest, 59510**“ auf (OSM-Objekt **way/350353336**, `highway=unclassified`) - er liegt damit rund **43,9 km** von der belegten Anschrift entfernt, in einem anderen Landkreis. **Keine Koordinate wird gesetzt** - die Entscheidung über Setzen oder Entfernen des Altpunkts trifft der Owner in einer eigenen datierten Korrektur-Sektion. Quelle(n): https://recycling-alpha.de/ (Zertifikatsfehler ERR_CERT_COMMON_NAME_INVALID, Abruf 23.11.2026) ; http://recycling-alpha.de/ (HTTP 301 auf https://www.instagram.com/alpha_recycling) ; https://www.instagram.com/alpha_recycling/ („Page ist nicht verfügbar“) ; http://archive.org/wayback/available?url=recycling-alpha.de ; https://www.11880.com/suche/schrotthandel/muenster ; https://www.11880.com/branchenbuch/muenster-westfalen/120674719B113391111/alpha-recycling.html ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Alpha%20Recycling ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Alpha%20Recycling ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Alpha ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Im%20Moorhock%2063%2C%2048159%20M%C3%BCnster%2C%20Deutschland;7.6046644;51.9884996;7.6446644;52.0184996;7.5646644;51.9584996&dst=3&is=1&e=1 ; https://www.online-handelsregister.de/suche/Alpha%20Recycling%20M%C3%BCnster ; read-only-Abgleich `public.db` vom 23.11.2026 ; https://api.openstreetmap.org/api/0.6/way/239486959.json ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.6992941176471&lon=8.03888235294118&zoom=18&addressdetails=1 (Abruf 23.11.2026).]

### Korrektur 24.11.2026 (Owner, Geokodierung) — der falsche Standortpunkt wird entfernt

- [Korrektur 24.11.2026: **Der in der Produktion gespeicherte Standortpunkt dieses Dossiers liegt **43,86 km** von **way/239486959** ✓✓ entfernt — er trifft **Frölichweg, Lippborg, Lippetal, Kreis Soest, Nordrhein-Westfalen, 59510, Deutschland** ✓ ✓ — er wird entfernt ✓ ✓  Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓** Der Punkt liegt im **Kreis Soest** ✓✓ ✓ ✓ Die **Domain** ✓✓ lebt nur noch als Weiterleitung ✓✓ auf ein **entferntes** ✓✓ Instagram-Profil ✓✓ — die Adresse ist **nicht** belegt ✓✓ ✓ ✓ Ein Ersatzpunkt entsteht **nicht** ✓ ✓ ✓ ✓ Der Beleg für die Adresse: der **Verzeichniseintrag** ✓✓ oder der **amtliche Registerauszug** ✓✓ **des Dossierbetriebs** ✓ ✓ **Reichweite:** Die EfbV-/AltFahrzeugV-Anerkennung ist **freiwillig** ✓ ✓, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ ✓ ✓ Ein Verzeichnislead ist **kein** belegter Standort des Betriebs ✓ ✓ ✓ ✓ ✓ ✓ **Belege** ✓ ✓: **erstens** ✓ ✓ die Wellennotiz mit der Rückauflösung des gespeicherten Punktes ✓ ✓ (Abruf 23.11.2026 ✓ ✓), **zweitens** ✓ ✓ das **Adressobjekt** ✓ ✓ in OpenStreetMap ✓ ✓ ✓ ✓]
