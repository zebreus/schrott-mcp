---
slug: sh-gettorf-auto-center-nemitz
name: Auto Center Nemitz
trader_type: autoverwertung
state: SH
city: Gettorf
street: Ravensberg 21
postcode: '24214'
lat: '54.4158958'
lon: '9.9769499'
phone: ''
email: ''
opening_hours: ''
website: https://autoverwertung-nemitz.de
website_status: blockiert
status: pruefung
description: 'Auto Center Nemitz, Ravensberg 21, 24214 Gettorf (Dänischer Wohld). Die Betreiberdomain autoverwertung-nemitz.de ist derzeit nicht abrufbar (HTTP 403, kein Inhalt lesbar); Anschrift und Lage sind über das Branchenverzeichnis (Eintrag aktualisiert 27.10.2026, Eintragsdaten 11.10.2026) und einen gleichnamigen OSM-Adresspunkt belegt. Ob aktuell Altfahrzeuge oder Metallschrott angenommen werden, ist nicht belegt; Gebrauchtwagenverkauf ist kein Schrottankauf. Ankaufkurse, Verkaufspreise und Entsorgungsgebühren sind nicht veröffentlicht.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja (Autoverwertung-Flag)
provenance_origin: table
---

# Auto Center Nemitz

## Überblick

Die Domain autoverwertung-nemitz.de ist bei direktem Abruf weiterhin blockiert (403, http → 301 → https → 403). Die Anschrift **Ravensberg 21, 24214 Gettorf** ist nun durch zwei voneinander unabhängige Quellen belegt (Branchenverzeichnis, Portalaktualisierung 27.10.2026, und ein betreiberbenannter OSM-Adresspunkt mit praktisch identischer Geokoordinate) und liegt damit im Frontmatter; die frühere Suchindexspur Ravensberg 21 ist damit bestätigt. Impressum und aktueller Rechtsträger sind weiterhin nicht lesbar. Das belegt nicht, dass aktuell Altfahrzeuge oder Metallschrott angenommen werden. Verkauf gebrauchter Fahrzeuge ist nicht mit Ankauf von Schrott gleichzusetzen. Status auf `pruefung` bleibt wegen unbestätigter Zielidentität/Annahme, nicht wegen der Abrufsperre allein. Ankaufkurse, Verkaufs-/Entsorgungsgebühren bleiben offen.

## Timeline

### Recherche 27.10.2026

- [Recherche 27.10.2026: Branchenverzeichnis-Eintrag neu gelesen (11880, Abruf 27.10.2026): „**Auto Center Nemitz**“, **Ravensberg 21, 24214 Gettorf**, Telefon (04346) 7877, Fax (04346) 410863, E-Mail acn@gmx.net, Verzeichniskategorie RecyclingCenter/Schrottplatz, **Portalaktualisierung 27.10.2026, Eintragsdaten 11.10.2026**, eine Bewertung von 2014. Die Portalseite führt als Geokoordinate **9.97695 / 54.415896**.]
- [Recherche 27.10.2026: Zweite, von der Betreiberseite unabhängige Quelle für die Anschrift: OSM/Nominatim führt in Gettorf-Dänischer Wohld ein **eigenständig benanntes Objekt „Auto Center Nemitz, 21, Ravensberg“** bei **54.4158958 / 9.9769499** — die Koordinaten beider Quellen stimmen auf 1–2 Nachkommastellen (sub-Meter) überein. Damit sind **Anschrift und Lage** durch zwei unabhängige Quellen belegt und wurden in `street`, `postcode`, `lat` und `lon` übernommen. **Telefon und E-Mail stammen ausschließlich aus dem Verzeichnis** (Aggregator = Lead, kein Beleg) und bleiben deshalb leer. Punktart: **Gebäude-/Adresspunkt**, keine bestätigte Anlieferzufahrt.]
- [Recherche 27.10.2026: GEO-Korrektur. Die read-only-Produktionszeile enthielt den Punkt **54.4/9.9833**; die OSM/Nominatim-Rückabfrage löst ihn auf **„8, Erlengrund, 24214 Gettorf“** auf — rund **1,8 km Luftlinie** vom belegten Standort Ravensberg 21 entfernt. Ersetzt durch den adressgenauen, betreiberbenannten OSM-Punkt 54.4158958/9.9769499. Kein `clear_fields: geo` nötig, da die Zeile jetzt eine eigene belegte Ortsangabe trägt und der Punkt korrekt darauf gesetzt ist.]
- [Recherche 27.10.2026: Betreiberdomain weiterhin **nicht abrufbar**: `http://autoverwertung-nemitz.de` → 301 → `https://www.autoverwertung-nemitz.de` → **HTTP 403** (live geprüft 27.10.2026). Die Abrufsperre ist weder Nachweis einer toten Domain noch einer Betriebsschließung; `website_status: blockiert` bleibt. Eine Wayback-Snapshots existiert (19.04.2024, Startseite), der Abruf war am 27.10.2026 jedoch wegen einer Sperre des Archivdienstes nicht möglich — daher wird daraus **kein** Impressum abgeleitet.]
- [Recherche 27.10.2026: Amtliches Fachbetrieberegister (Fachbetrieberegister der Gemeinsamen Stelle Altfahrzeuge der Länder nach § 7 Abs. 2a AltfahrzeugV und Register der Entsorgungsfachbetriebe nach § 56 KrWG, Auszug mit Stand 11.08.2026, gespiegelt über SchrottRegister) kennt **für Gettorf keine Ortsseite** und damit keinen anerkannten Betrieb — ein Demontagebetrieb nach AltfahrzeugV wäre dort zu erwarten. Efb-Anerkennung und Demontagezulassung sind freiwillig bzw. betriebsabhängig, das Fehlen ist **kein** Nichtexistenzbeweis. Ebenfalls neu geprüft: das im gleichen Ort geführte, **andere** Unternehmen „**Autoverwertung Gettorf Inh. R. Jess**“, Hasselrott 86, 24214 Gettorf, Tel. 04346 5355, robert-jess@gmx.de (11880, Eintrag 07.10.2026) — eigener Betrieb, **nicht** zu vermerken und nicht zu verschmelzen.]
- [Preise/Ankauf/Verkauf/Gebühren 27.10.2026: **ANKAUF:** die Verzeichniskategorie „Schrottplatz“ ist keine bestätigte Annahme von Altfahrzeugen oder Metallschrott; keine Kurse belegt. **VERKAUF:** keine aktuellen Fahrzeugpreise (die frühere Gebrauchtwagenwerbung ist nicht live lesbar). **GEBÜHREN:** keine Entsorgungs- oder Abholtarife. Status bleibt `pruefung`. Quellen: https://www.11880.com/branchenbuch/gettorf/060444720B49558891/auto-center-nemitz.html ; https://www.11880.com/branchenbuch/gettorf/120674719B26607076/autoverwertung-gettorf-inh-r-jess.html ; https://nominatim.openstreetmap.org/search?format=jsonv2&q=Auto+Center+Nemitz%2C+Gettorf&addressdetails=1 ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&zoom=18&lat=54.4&lon=9.9833&addressdetails=1 ; https://autoverwertung-nemitz.de/ (403) ; http://web.archive.org/cdx/search/cdx?url=autoverwertung-nemitz.de ; https://schrottregister.pages.dev/ort-gettorf (keine Ortsseite) ; read-only `public.db` (27.10.2026).]

### Recherche 23.10.2026

- [Recherche 23.10.2026: Direkter erneuter Abruf von https://autoverwertung-nemitz.de/ scheitert weiterhin mit HTTP 403; dadurch keine Betreiberinhalte, kein Impressum und kein aktueller Leistungs-/Preistext lesbar. Die Sperre ist weder Nachweis einer toten Domain noch einer Betriebsschließung. Frühere Suchindexspur Ravensberg 21/24214 Gettorf bleibt ein unbestätigter Gebrauchtwagen-Lead, kein belegter Annahmeplatz. **Ankauf:** keine aktuelle Schrott-/Altfahrzeugannahme oder Kurse bestätigt. **Verkauf:** keine aktuellen Fahrzeugpreise. **Gebühren:** keine Tarife. Kein GEO-Punkt ohne verifizierten Betreiber und Annahmestelle. Quelle: https://autoverwertung-nemitz.de/ (Direktabruf HTTP 403, 23.10.2026).]

### Recherche 14.10.2026 (18:00-Welle C)

- [Recherche 14.10.2026: Domain und bereits bekannte Unterseite /consulting/ erneut separat direkt angefragt, beide HTTP 403. Kein Betreibertext/Impressum/Leistungskatalog live lesbar; Abrufblockade bestätigt website_status blockiert, ist weder Website-tot- noch Betriebsschließungsnachweis. Frühere Indexspur Ravensberg 21/24214 Gettorf/Gebrauchtwagen bleibt nur vorläufiger Kandidat; keine frische Adress-/Betreiberbestätigung. ANKAUF: Annahme von Altfahrzeugen/Schrott nicht belegt; VERKAUF: frühere Gebrauchtwagenwerbung ohne frisch gelesene Preise; GEBÜHREN: keine Entsorgungs-/Abholtarife. GEO erst nach Rechtsinhaber/Annahmeplatzbestätigung; Quelle(n): https://autoverwertung-nemitz.de ; https://www.autoverwertung-nemitz.de/consulting/ (403).]
- [Korrektur 14.10.2026: status aktiv → pruefung: Die erhaltenen Recherchen 01./02./07.10. dokumentieren weder zulässigen Zwei-Quellen-Beleg noch aktuelle Schrott-/Autoannahme; Seed-Flag Autoverwertung und Domainname sind dafür nicht genug. Keine Frontmatter-Adresse aus Index ergänzen, Website-Link/Historie erhalten, kein geschlossen. Owner: direkten Betreiber-/Gewerbe- oder aktuellen Zertifikatsnachweis besorgen, Gebrauchtwagenhandel getrennt von Autoverwertung prüfen; Quelle: vorgenannte Abrufe, Dossierhistorie, README.md Quellenhierarchie.]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Suchindexeinträge zur Betreiberdomain zeigen Auto-Center Nemitz, Ravensberg 21, 24214 Gettorf und Gebrauchtwagenangebot für Händler/Export/Privat. Direkter Abruf von Homepage und Unterseite blieb mit HTTP 403 blockiert; deshalb sind die Suchindexangaben kein live gelesener Betreiberbeleg. Die Website-Sperre bleibt dokumentiert, die möglicherweise gebrauchten Fahrzeuge sind Verkauf und keine Schrottpreise; kein Nachweis einer Metallannahme, keine Ankaufspreisliste oder Entsorgungsgebühr gefunden. Keine Frontmatter-Änderung. Quelle(n): https://www.autoverwertung-nemitz.de/ ; https://www.autoverwertung-nemitz.de/consulting/]

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung + Schrottannahme (Verzeichnis)
- (Autoverwertung Nemitz)

### Recherche 02.10.2026

- [Recherche 02.10.2026: Blockade erneut bestätigt (02.10.2026): http + https + www-Variante jeweils 403 (curl + Direktabruf). Domain resolves (Cloudflare-IPs), aber kein Inhalt abrufbar; website_status blockiert beibehalten, keine Fills, weiter Klärfall. Quelle(n): Direktabruf http/https/www 02.10.2026, DNS-Check.]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Domain https://autoverwertung-nemitz.de per Direktabruf (http + https) erneut BLOCKIERT (403, 01.10.2026) — website_status blockiert bestätigt und beibehalten; keine alternativen Belege (Register/Social) auffindbar; alle Adress-/Kontaktfelder bleiben leer; Klärfall für Folgewelle; Quelle(n): Direktabruf-Status 403.]
