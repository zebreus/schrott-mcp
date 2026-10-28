---
slug: st-gerwisch-b-magdeburg-hoffmann-u-hoffmann-elektroschrottrecycl
name: Hoffmann u. Hoffmann GbR Elektroschrottrecycling
trader_type: schrotthaendler
state: ST
city: Gerwisch b. Magdeburg
street: Gartenstr. 1
postcode: '39175'
phone: 039292/9144
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: 'Elektroschrottrecycling am Standort Gartenstr. 1, 39175 Gerwisch (Biederitz). Die Identität ist nur über ein Branchenverzeichnis belegt; im amtlichen Register der Entsorgungsfachbetriebe (Stand 11.08.2026) ist für die Postleitzahl 39175 nur die Gerwischer Rohstoffrecycling & Verwertungs GmbH (Lostauer Straße 5) geführt, ein Eintrag Hoffmann u. Hoffmann fehlt. Annahmebedingungen, Preise und Gebühren sind nicht belegt; die ebenfalls geführte Gerwischer Rohstoffrecycling & Verwertungs GmbH ist ein getrenntes Unternehmen.'
clear_fields: 'geo'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: Ankauf unklar (E-Schrott – prüfen)
provenance_origin: table
---

# Hoffmann u. Hoffmann GbR Elektroschrottrecycling

## Überblick

Die importierte Identität „Hoffmann u. Hoffmann GbR Elektroschrottrecycling“ mit Gartenstraße 1 und Telefonnummer ist weiterhin nicht durch einen aktuellen Betreiber- oder Registerbeleg bestätigt. Ein eigener Recyclingbetrieb existiert in Gerwisch unter dem Namen **Gerwischer Rohstoffrecycling & Verwertungs GmbH**, Lostauer Straße 5 (HRB 1316); es gibt aber keinen Beleg für eine Verbindung zu Hoffmann u. Hoffmann. Die beiden Datensätze dürfen nicht zusammengeführt werden.

**Preise:** **Ankauf** – für den Zielbetrieb Hoffmann u. Hoffmann keine Annahmebedingungen oder Preise belegt. **Verkauf** – keine Produkt-/Preisliste zuordenbar. **Gebühren** – keine E-Schrott-/Entsorgungsgebühren belegt.

**Geokodierung:** Die Seed-Adresse Gartenstraße 1 bleibt unbestätigt; die Lostauer Straße 5 gehört einem anderen, nicht zugeordneten Unternehmen und darf nicht als Standort übernommen/geokodiert werden. Der bisherige Produktionspunkt lag rund 4,2 km entfernt in einem anderen Ortsteil (Königsborn) und wurde mit `clear_fields: 'geo'` zurückgezogen, ohne Ersatz, weil die Hausnummer nur durch einen Aggregator-Lead belegt ist.

## Timeline

### Recherche 28.10.2026 (Shard C, 6h-Welle): Verzeichnisbestand erweitert, Branchenbezeichnung uneinheitlich

- [Recherche 28.10.2026: Der Bestand an Verzeichnis-Leads für **Hoffmann u. Hoffmann GbR Elektroschrottrecycling**, Gartenstr. 1, 39175 Gerwisch, Tel. 039292 9144 wurde erneut geprüft und ist in sich **stimmig**: das Branchenbuch der Gemeinde Gerwisch („Hoffmann u. Hoffmann GbR Transport“), ITSBETTER, anrufer.info, defirmenkataloge, finderr (Spedition), schrottplatz.org („Anbieter für E-Schrott“), öffnungszeitenbuch und 11880 („Recycling, Entsorgung & Altmaterialhandel“) nennen dieselbe Anschrift und dieselbe Nummer. Es bleibt ein **Verzeichniskonsens ohne Betreiberquelle**: keine eigene Website, kein Impressum, kein Inhabername, kein Registereintrag. Die Recherche vom 27.10.2026 bleibt maßgeblich — im amtlichen Fachbetrieberegister (Auszug 11.08.2026) ist für Biederitz/Gerwisch (39175) **nur** die „Gerwischer Rohstoffrecycling und Verwertung GmbH“ (Lostauer Straße 5) geführt, ein Eintrag „Hoffmann u. Hoffmann“ fehlt (Negativbefund ohne Beweiswert, Efb-Anerkennung ist freiwillig); die beiden Datensätze bleiben getrennt. Quellen: https://gerwisch.stadtbranchenbuch.com/545843.html ; https://itsbetter.com/firmeneintrag/hoffmann-u-hoffmann-gbr-elektroschrottrecycling/ ; https://www.finderr.de/spedition-gerwisch-hoffmann-gbr-270240 ; https://www.schrottplatz.org/biederitz/schrott-hoffmann-u-hoffmann-gbr-elektroschrottrecycling-aYEPQC.html ; https://schrottregister.pages.dev/ort-biederitz (Abruf 28.10.2026).]
- [Recherche 28.10.2026: **Branchenbezeichnung uneinheitlich** — die Verzeichnisse führen den Betrieb je nach Quelle als „Elektroschrottrecycling“ (11880, schrottplatz.org), „Transport“ (Stadtbranchenbuch Gerwisch, defirmenkataloge) oder „Spedition“ (finderr); die 11880-Leistungsangabe „Recycling, Entsorgung & Altmaterialhandel“ ist eine Kategorie, kein Leistungsnachweis. Auch die Öffnungszeitenangaben widersprechen sich (defirmenkataloge: Mo 11–17, Di 9–17, Mi 11–18, Do 8–17, Fr 9–16; finderr nennt ausdrücklich „branchenübliche Zeiten … keine Angabe des Unternehmens“ Mo–Fr 08:00–17:00) — deshalb wurden **keine** Öffnungszeiten übernommen und die Frontmatter unverändert gelassen. Quellen: https://gerwisch.defirmenkataloge.com/company/hoffmann-u-hoffmann-gbr/ ; https://www.finderr.de/spedition-gerwisch-hoffmann-gbr-270240 (Abruf 28.10.2026).]
- [GEO 28.10.2026: Für „Gartenstraße 1, 39175 Gerwisch“ liefert OSM **keinen** adressgenauen Hausnummernpunkt (Nominatim löst nur die Straßenobjekte way 741651466 / way 647453705 / way 525173996 auf) — die in der Recherche 27.10.2026 als Recherchehinweis vermerkte Nominatim-Auflösung 52.1787073/11.7456129 ist ein Adresspunkt derselben Straße, wurde aber nicht gesetzt, weil die Hausnummer nur durch einen Aggregator-Lead belegt ist (Zwei-Quellen-Regel nicht erfüllt). Die `clear_fields: 'geo'`-Direktive bleibt unverändert bestehen, damit der zurückgezogene Punkt in der Produktion nicht wiederkehrt. Quellen: https://nominatim.openstreetmap.org/search?format=jsonv2&q=Gartenstra%C3%9Fe+1%2C+39175+Gerwisch&addressdetails=1&extratags=1 (Abruf 28.10.2026).]
- [Preise/Ankauf/Verkauf/Gebühren 28.10.2026: **Ankauf** — nicht belegt; die Verzeichnisse nennen die Branche „Elektroschrottrecycling“, aber keine Annahmebedingungen, keinen Mindestmengenwert und keinen Kurs. **Verkauf** — nicht belegt. **Gebühren** — nicht belegt; das 11880-Profil führt ausdrücklich „keine Öffnungszeiten“ und keine Preisangaben. Quellen: https://www.11880.com/branchenbuch/gerwisch/060441364B27807836/hoffmann-u-hoffmann-gbr-elektroschrottrecycling.html ; https://www.oeffnungszeitenbuch.de/filiale/Biederitz-Hoffmann%2520u.%2520Hoffmann%2520GbR%2520Elektroschrottrecycling-1889340L.html (Abruf 28.10.2026).]
- [Status und offene Punkte 28.10.2026: `status` bleibt `pruefung`; Frontmatter unverändert (Adresse und Telefon bleiben der Aggregator-Lead vom 27.10.2026, die Beschreibung bleibt bestehen). Offen für den Owner: (1) Inhaber/Rechtsform der GbR über das Handelsregister prüfen — eine GbR ist seit 2023 registerpflichtig, ein Registerabzug wurde bisher nicht herangezogen; (2) den Telefon 039292 9144 verifizieren; (3) klären, ob der Betrieb Elektroschrott tatsächlich annimmt (die Branche steht nur im Verzeichnistext). Quellen: https://www.11880.com/branchenbuch/gerwisch/060441364B27807836/hoffmann-u-hoffmann-gbr-elektroschrottrecycling.html ; https://schrottregister.pages.dev/ort-biederitz (Abruf 28.10.2026).]

### Recherche 27.10.2026

- [Recherche 27.10.2026: Amtliches Fachbetrieberegister neu ausgewertet (Fachbetrieberegister der Gemeinsamen Stelle Altfahrzeuge der Länder nach § 7 Abs. 2a AltfahrzeugV und Register der Entsorgungsfachbetriebe nach § 56 KrWG, Auszug mit Stand 11.08.2026, gespiegelt über SchrottRegister): Für **Biederitz (39175)** ist **genau ein Betrieb** anerkannt — **„Gerwischer Rohstoffrecycling und Verwertung GmbH“, Lostauer Straße 5** (Lagern, Behandeln, Verwerten). Ein Eintrag **„Hoffmann u. Hoffmann“ fehlt**. Die Postleitzahl der Seed-Adresse Gartenstr. 1 (39175) liegt innerhalb der abgedeckten PLZ; der Negativbefund ist deshalb aussagekräftiger als sonst, bleibt aber ein Negativbefund (Efb-Anerkennung ist freiwillig). Die beiden Unternehmen werden ausdrücklich **nicht** zusammengeführt.]
- [Recherche 27.10.2026: Branchenverzeichnis-Eintrag erneut live gelesen (11880, Abruf 27.10.2026): „Hoffmann u. Hoffmann GbR Elektroschrottrecycling“, **Gartenstr. 1, 39175 Gerwisch**, E-Mail hoffmann-gerwisch@t-online.de, Branchenbuch-Eintragsdaten vom 10.05.2026, Portalaktualisierung 03.10.2026; die im Portal verlinkte Geokoordinate lautet 11.745613 / 52.1787073. Es bleibt **ein Aggregator-Lead** (Einzelbeleg) — keine zweite unabhängige Quelle, keine Betreiber-Website, kein Registertreffer; Telefon 039292/9144 bleibt deshalb unbestätigter Altwert.]
- [Recherche 27.10.2026: GEO-Korrektur. Die read-only-Produktionszeile enthielt den Punkt **52.14175/11.7565125**; die OSM/Nominatim-Rückabfrage löst ihn auf **„Hauptparkplatz BBG, Am Fuchsberg, 39175 Königsborn (Biederitz)“** auf — das ist ein **anderer Ortsteil** als die in der Zeile geführte Adresse Gartenstr. 1 in Gerwisch und liegt rund **4,2 km Luftlinie** davon entfernt. Da die Zeile selbst eine eigene Ortsangabe (Gartenstr. 1, 39175 Gerwisch) trägt, die dem gespeicherten Punkt widerspricht, wird der Punkt mit `clear_fields: 'geo'` **zurückgezogen**; eine Ersatzkoordinate wird **nicht** gesetzt, weil die Hausnummer nur durch einen Aggregator-Lead belegt ist und die Two-Source-Regel für die Adresse nicht erfüllt ist. Der Adresspunkt von OSM/Nominatim zu „Gartenstraße 1, 39175 Gerwisch“ (52.1787073/11.7456129) ist als Recherchehinweis vermerkt und deckt sich mit der Portal-Geokoordinate.]
- [Preise/Ankauf/Verkauf/Gebühren 27.10.2026: **Ankauf:** für den Zielbetrieb keine Annahmebedingungen oder Preise belegt; das amtliche Register enthält keine Preise. **Verkauf:** keine Produkt-/Preisliste zuordenbar. **Gebühren:** keine E-Schrott-/Entsorgungsgebühren belegt. Status bleibt `pruefung`. Quellen: https://schrottregister.pages.dev/ort-biederitz ; https://schrottregister.pages.dev/betrieb-gerwischer-rohstoffrecycling-und-verwertungs-gmbh-biederitz ; https://www.11880.com/branchenbuch/gerwisch/060441364B27807836/hoffmann-u-hoffmann-gbr-elektroschrottrecycling.html ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=52.14175&lon=11.7565125&addressdetails=1 ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Gartenstra%C3%9Fe+1%2C+39175+Gerwisch&addressdetails=1 ; read-only `public.db` (27.10.2026).]

### Importiert (Seed-Stand 2026-09-30)

- Elektroschrottrecycling (lt. 11880)

### Recherche 07.10.2026

- [Recherche 07.10.2026: Die offizielle Website von Gerwischer Rohstoffrecycling & Verwertungs GmbH nennt deren Standort Lostauer Straße 5 in Gerwisch; das Impressum führt HRB 1316. Die Leistungen betreffen Recycling/Verwertung, doch weder Website noch Registerbezug weisen eine Verbindung zur im Seed genannten Hoffmann u. Hoffmann GbR, Gartenstraße 1 oder 039292/9144 aus. Das sind daher getrennte Unternehmen/Kandidaten; keine Frontmatter-Felder übertragen. **Ankauf:** keine zuordenbaren Kurse; **Verkauf:** keine Preisliste; **Gebühren:** keine Tarife. Keine Geokodierung für den Seed; Quelle: https://gerwischer-recycling.de/ ; https://gerwischer-recycling.de/impressum/]

### Recherche 04.10.2026

- [Recherche 04.10.2026: Klärfall bleibt offen. Die Gemeindeseite Biederitz liefert für die Suche nach „Hoffmann“ keine Treffer. Das belegt nicht, dass kein Betrieb besteht, und verifiziert weder die importierte Identität noch Gartenstr. 1, 39175 oder 039292/9144; die geprüfte Gemeindesuche liefert keinen ergänzenden Beleg zu den Seed-Angaben. Die vorhandenen, nichtleeren Felder bleiben unangetastet; keine Schließungsbehauptung, keine Annahme-/Abholangaben ergänzt und keine Geocodierung. Restunsicherheit: Zuordnung der GbR zu dieser Adresse/Telefonnummer, aktueller Betrieb und E-Schrott-Annahme; Quelle(n): https://www.biederitz.de/?s=Hoffmann (Abruf 04.10.2026; offizielle Gemeindeseite, keine Übereinstimmung auf der Website-Suche).]
