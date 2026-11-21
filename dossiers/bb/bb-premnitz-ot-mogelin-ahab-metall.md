---
slug: bb-premnitz-ot-mogelin-ahab-metall
name: ahab-metall e.K.
trader_type: schrotthaendler
state: BB
city: Premnitz OT Mögelin
street: Carl-Zeiss-Str. 14
postcode: '14727'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: ja
provenance_origin: table
---

# ahab-metall e.K.

## Überblick

„ahab-metall e.K.“ und Carl-Zeiss-Straße 14 sind nur durch den historischen Schrottradar-Lead dokumentiert. Der Direktcheck der NorthData-Suche `ahab-metall` ergab auf der lesbaren ersten Trefferseite keinen orts-/namensgleichen Rechtsträger, nur unpassende Metallfirmen; das ist kein vollständiger Register-Negativnachweis. Für den e.K.-Zusatz fehlen Inhaber und HRA. Deshalb Status auf `pruefung` statt unbelegt `aktiv`; Anschrift als Altangabe erhalten, nicht als bestätigte Annahmestelle ausweisen.

**ANKAUF:** Seed „ja“ ist nicht primär bestätigt, keine Kurse. **VERKAUF / GEBÜHREN:** keine zuordenbare Liste. Offen: korrekte Namensschreibweise, Inhaber/HRA, heutiger Betrieb an der Adresse; Gewerberegister Premnitz abfragen.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 20.10.2026

- [Recherche 20.10.2026: Erneute Suche nach der exakten Schreibweise „ahab-metall e.K.“ und der Carl-Zeiss-Str. 14 ergab keine aktuelle Betreiberseite, belastbare Registerauskunft oder Kundenannahmestelle. Das ist kein Negativnachweis für eine Firma oder Schließung; der bisherige begrenzte NorthData-Nichtfund bleibt nur ein Recherchelimit. Name, Adresse und e.K.-Zusatz bleiben unbestätigte Seed-Leads; Status pruefung. ANKAUF/VERKAUF/GEBÜHREN: keine konkrete Preisliste. GEO bis zur Auflösung von Rechtsträger und Standort nicht durchführen. Quelle/Abgleich: https://www.northdata.de/?query=ahab-metall (Suchgegenprobe 20.10.2026; keine passende Betreiberbestätigung auf der lesbaren Trefferseite).]

### Recherche 18.10.2026 (Welle 1202 Nordost)

- [Recherche 18.10.2026: Exakte NorthData-Suggest-Abfrage ahab-metall heute HTTP 429/Zu viele Abfragen; kein aktueller Registerbefund und insbesondere keine Bestätigung des e.K.-Zusatzes, Inhabers oder HRA. Der frühere beschränkte Nichtfund bleibt historische Recherche, keine Negativauskunft. Carl-Zeiss-Str. 14 weiter ungesicherter Seed-Lead, kein öffentlicher Ankaufhof nachgewiesen; status pruefung und Bestandswerte erhalten. Ankauf/Verkauf/Gebühren ohne verifizierte Zahlen. Weiterer Belegweg kommunale Gewerbeauskunft Premnitz plus registergenauer Inhaber-/Firmenschreibweisenabgleich, danach Betriebsstätte/GEO. Quelle/Abrufversuch: https://www.northdata.de/suggest.json?query=ahab-metall&countries=DE .]

### Recherche 08.10.2026

- [Korrektur 08.10.2026: Belegaudit: bisheriger Aktivstatus/Ankauf-ja beruhen ausschließlich auf Schrottradar-Lead. NorthData-Suche ahab-metall direkt lesbar, erste Trefferseite ohne eindeutig passendes Premnitzer Unternehmen; keine vollständige Registerauskunft und kein Schließungsbeleg. status pruefung gesetzt, vorhandene Anschrift erhalten; keine Kontakte/Website gefüllt. Für e.K. insbesondere Inhaber und HRA ermitteln. ANKAUF/VERKAUF/GEBÜHREN ohne verifizierte Zahlen. Quelle(n): https://www.northdata.de/?query=ahab-metall ; historische Seednotiz Schrottradar (kein Primärbeleg)]

### Importiert (Seed-Stand 2026-09-30)

- Metall/Schrott
- Adressbeleg: schrottradar-Beleg via Runde 3

### Recherche 21.11.2026 (Welle 0000 B)

- [Recherche 21.11.2026: Die offene Frage nach Inhaber und Rechtsform des `ahab-metall e.K.` lässt sich erstmals mit einer von der Dossier-Adresse unabhängigen Geodatenquelle beantworten, die Betriebsseite selbst bleibt aber unbelegt. OSM führt unter way/474699025 genau die Dossier-Adresse `Carl-Zeiss-Straße 14, 14727 Premnitz OT Mögelin` mit den Tags `name=ahab-metall - Inhaber Michael Müller e.K.`, `addr:city=Premnitz`, `addr:suburb=Mögelin`, `addr:housenumber=14`, `addr:postcode=14727`, `addr:street=Carl-Zeiss-Straße`, `building=commercial`; der Eintrag nennt damit sowohl die Rechtsform e.K. als auch den Inhaber. Das ist ein Geodaten-Eintrag, kein Betreiber- oder Registerbeleg. Die Verzeichnisse bestätigen den Namen unabhängig voneinander, aber nicht unabhängig in der Sache: `branchen-info.net` führt `ahab-metall Inhaber: Michael Müller e. K.` mit Carl-Zeiss-Straße 14, 14727 Premnitz OT Mögelin, Telefon 03386 210288 und Oeffnungszeiten Mo-Fr 09:00-19:00 sowie Sa 09:00-13:00; `stadtbranchenbuch.com` nennt dieselben Daten und verweist als "Homepage" auf `ahab-metall.de`, `branchen-info.net` verweist auf `ahab-buntmetall.de`. Beide Domains sind am 21.11.2026 gemessen tot: die DNS-Auflösung über dns.google/resolve liefert für `ahab-metall.de` NXDOMAIN (Status 3) für A, NS und MX, `ahab-buntmetall.de` läuft auf ns1/ns2.redirectdom.com und liefert beim Abruf eine Parking-Weiterleitung auf p2-1.afp24.net, also eine Redirect-Domain ohne Inhalte. Ein lebender Betreiberauftritt ist damit nicht vorhanden. Das amtliche Fachbetrieberegister führt den Betrieb nicht: die Namenssuche `ahab` liefert acht Treffer (AHAB-Akademie GmbH Berlin, ahab IT + Consulting GmbH, ahab IT + Software UG, AHAB GmbH Frankfurt, AHAB Ingenieurleistungen UG & Co. KG, AHAB Verwaltungs-UG, AHAB-Akademie/ESUNA GmbH und die Person Ahab Abu Maileg), die Standortsuche um Carl-Zeiss-Straße 14 (dst=3) vier Anlagen fremder Betriebe (`RELUX Rohstoffe GmbH & Co. KG`, Arthur-Wilke-Straße 4, 2,1 km, ZZEE00200304 / 9 bis 11; `EM & ES Transporte GmbH`, Fabrikenstraße 11, 2,3 km, ZZLT00300007 / 1) und die Route Altfahrzeugverwertung in 10 km nur `H & S Abschleppdienst GmbH`, Rhinower Straße 35, 14712 Rathenow. Für ein Einzelunternehmen der Rechtsform e.K. ist ein Registereintrag nicht zwingend, ein Register-Nichteintrag ist daher kein Nichtexistenzbeweis; eine HRA ist weiterhin nicht belegt. Die Portaleinträge sind ein Datenbild (`schrottplatz.org` führt den Betrieb als `Schrottplatz Brandenburg Ahab Metall` und in einer zweiten Fassung mit der falschen PLZ 14715), der Containerdienst-Portal-Eintrag wiederholt nur Anschrift und Branche. ANKAUF: keine Kursliste und kein Zahlenwert. VERKAUF: keine Preisliste. GEBÜHREN: keine Tarife. Keine Frontmatter-Änderung: Telefonnummer, Oeffnungszeiten, Website und Inhaber bleiben unbelegt im Bestand, der Inhabername ist nur eine Kandidatur; Status `pruefung` bleibt.
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/474699025.json ; https://premnitz.branchen-info.net/ahab-metall-inhaber-michael-mueller-e-k/564100/ ; https://premnitz.stadtbranchenbuch.com/5228751.html ; https://dns.google/resolve?name=ahab-metall.de&type=A ; https://dns.google/resolve?name=ahab-buntmetall.de&type=NS ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=ahab (Abruf 21.11.2026).]
- [Recherche 21.11.2026: Die beiden Portal-Websites des Betriebs sind gemessen tot, was die Annahme "keine Betreiberseite gefunden" (Notiz vom 18.10.2026) präzisiert: es gab einmal eine Domain-Angabe, sie existiert heute nicht mehr. `ahab-metall.de` liefert NXDOMAIN (dns.google/resolve, Status 3) für die Abfrage nach A, NS und MX, die Domain ist damit nicht mehr vergeben; `ahab-buntmetall.de` ist auf die Nameserver ns1/ns2.redirectdom.com delegiert und antwortet beim Abruf mit einer Sicherheitsprüfung des Redirect-Dienstes (p2-1.afp24.net), also mit Park- statt Inhaltsseite. Ob der Betrieb zunächst unter `ahab-buntmetall.de` gelaufen ist, lässt sich aus den Verzeichnisangaben nicht belegen; die Zuordnung bleibt eine Vermutung. Bewertet wird das nur als Lebenszeichen der Betriebswebsite, nicht als Betriebs- oder Schließungsbeleg: eine nicht erreichbare Domain beweist weder das Fortbestehen noch eine Aufgabe, geschweige eine Liquidation. Die Leistungsliste der Portale (Blech, Blei, Eisen, Kupfer, Metall, Zinn) ist eine Schlagwortliste ohne Preise und ohne bestätigte Annahmebedingungen; Mindestmengen sind nirgends genannt. ANKAUF: keine Kurse, keine Zahlen. VERKAUF: keine Liste. GEBÜHREN: keine Tarife; Containerbereitstellung wird nur als Branche vermerkt. Status `pruefung`, keine Feld- oder Geokodierungsanderung, keine Schließungsbehauptung.
  Quelle(n): https://dns.google/resolve?name=ahab-metall.de&type=NS ; https://dns.google/resolve?name=ahab-buntmetall.de&type=A ; https://www.schrottplatz.org/schrottplatz-in-premnitz ; https://containerdienst-portal.de/containerdienst/premnitz (Abruf 21.11.2026).]
