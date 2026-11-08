---
slug: nw-munster-schrottdirektor
name: Schrottdirektor
trader_type: schrotthaendler
state: NW
city: Münster
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: http://www.schrottdirektor.de/
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Münsterland / OWL (Münster, Bielefeld, Nottuln)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schrottdirektor

## Überblick

Die direkt gelesene HTTP-Leistungsseite beschreibt mobile Schrottabholung und Containerservice. Wichtig gegenüber bloßem Gratis-Abholdienst: Sie verspricht zusätzlich eine **Vergütung abhängig von Art und Menge**. Das ist Ankauf-/Vergütungswerbung, aber keine numerische Preisliste und keine Garantie eines konkreten Betrags. Kostenfreie Abholung ist davon getrennt zu behandeln.

Betreiber-Einzelquelle ohne hinreichenden unabhängigen Identitäts-/Kontaktbeleg; leere Felder bleiben leer. **Verkauf:** keine Liste auf den gelesenen Seiten. **Gebühren:** Werbung kostenfrei, kein Container-/Demontagetarif. Fritz-Pütter-Str. 13 ist nicht als verifizierter Anlieferhof oder GEO-Punkt freigegeben.

## Timeline

### Recherche 15.10.2026 (00:00-Welle A)

- [Recherche 15.10.2026: Start und leistungen.html direkt per HTTP gelesen. Leistungsseite präzisiert: Schrottabholung kostenfrei und zusätzlich Geld abhängig von Menge/Art des Schrotts; damit nicht fälschlich als ausschließlich kostenlose Entsorgung ohne mögliche Vergütung darstellen. Keine EUR/kg-Werte, kein Veröffentlichungsdatum, keine Verkaufsliste und kein positiver Container-/Demontagetarif. Nur ein Betreiberquellenkomplex, keine neue unabhängige Bestätigung der im Alt-Impressum dokumentierten Identität Uwe Müller/Anschrift. Bestehende HTTPS-URL nicht wegen erfolgreichem HTTP-Abruf umgeschrieben; keine Kontakt-/GEO-Fills. Quellen: http://schrottdirektor.de/ ; http://schrottdirektor.de/leistungen.html .]

### Recherche 04.10.2026

- [Recherche 04.10.2026: Vorhandene Betreiberseite/Impressum und Leistungsangaben erneut gegen externe Treffer geprüft; keine unabhängige, hinreichend belastbare zweite Quelle für Adresse und Kontakt gefunden. Die Betreiberangaben beziehen sich auf ein Einzelunternehmen ohne HRB; Aggregator-Treffer sind nach Quellenhierarchie nur Leads. Daher Straße, PLZ, Telefon, E-Mail, Öffnungszeiten und Beschreibung weiter leer gelassen; website und website_status bleiben unverändert, kein Preis übernommen; Quelle(n): http://schrottdirektor.de/ (Betreiberseite; in der Recherche 02.10.2026 dokumentierte Start-/Leistungs-/Impressum-Angaben); https://www.google.com/search?q=%22Schrottdirektor%22+%22Uwe+M%C3%BCller%22+M%C3%BCnster (externe Suchspur, darunter Aggregator-Leads ohne unabhängige Gegenbelege; Abruf 04.10.2026)]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Betreiber-Website live und tiefgecrawlt (Start, Leistungen, Impressum einzeln abgerufen). Impressum: Schrottdirektor.de, Uwe Müller, Fritz-Pütter-Str. 13, 48153 Münster, Büro 0251/275898762, Mobil 0172/2750796, info@schrottdirektor.de, USt-Nr. 337/5187/2843 (Einzelunternehmen, kein HRB → keine Owner-Ausnahme → Kontaktfelder bleiben leer, Repo-Praxis). Angebot: mobile kostenlose Schrottabholung + Containerservice, alle FE/NE-Sorten. Auffälligkeit: nur http erreichbar, https schlägt fehl (Strato-Hosting) — website_status dennoch aktiv. Frontmatter: website_status aktiv, status aktiv. street/postcode/phone/email laut Impressum: Fritz-Pütter-Str. 13 / 48153 / 0172/2750796 / info@schrottdirektor.de — Klärfall Folgewelle (Zweitbeleg). Quelle(n): schrottdirektor.de (http-Abruf 02.10.2026: index, leistungen, impressum)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (Fritz-Pütter-Str. 13)

### Recherche 09.11.2026

- [Recherche 09.11.2026: **Amtliches Fachbetrieberegister: für den Münsterer Betreiber „Schrottdirektor“ (Fritz-Pütter-Str. 13, 48153 Münster) keine Efb- oder AltfahrzeugV-Anerkennung ausgewiesen; zugleich ist die Betreiberwebsite nur unter `http://` erreichbar und das Impressum liefert erstmals lesbare Kontaktdaten.** Geprüft wurden beide Routen des amtlichen Fachbetrieberegisters (https://fachbetrieberegister.gadsys.de/fachbetrieberegister/), Route „Entsorgungsfachbetriebe“ und Route „Altfahrzeugverwertung“, durchgängig mit dem Filter „Art des Betriebs = Egal“ (`is=1&e=1`); die Standortabfragen liefen mit validiertem `adr`-Parameter um die Dossieradresse (Nominatim-Abgleich 09.11.2026: 51.9366874/7.6282875) in den Radien 3 km und 10 km. **Route „Entsorgungsfachbetriebe“:** die Namenssuche `n=Schrottdirektor` **0 Treffer**; die Standortsuche 3 km um die Dossieradresse → **15 Treffer**, 10 km um 48153 Münster → **59 Treffer** — u. a. „Einbrodt & Schubert GbR“, Dahlweg 122, 48153 Münster (ZZES01100066 / 1, 0,3 km), „REMONDIS GmbH & Co. KG | Behandlungs- und Lageranlage, Betriebsstätte Münster“, Kesslerweg 10, 48155 Münster, „Gaarmann Overhaus Container GmbH & Co. KG“, Loddenheide 30, 48155 Münster, „Abfallwirtschaftsbetriebe Münster | Recyclinghof Münster“, Eulerstraße 8, 48155 Münster, „SRM Schrott & Metallrecycling GmbH“, Kesslerweg 37, 48155 Münster — **kein** Treffer trägt den Namen Schrottdirektor, **keiner** liegt an der Fritz-Pütter-Straße 13, und keiner wurde übernommen. **Route „Altfahrzeugverwertung“:** `n=Schrottdirektor` **0 Treffer**; 3 km um die Dossieradresse → **1 Treffer** („Einbrodt & Schubert GbR“, Dahlweg 122, 48153 Münster, ZAES01100066 / 1, 0,3 km — **fremd**), 10 km um 48153 Münster → **1 Treffer** (derselbe). **Betreiber-Primärquelle (neu, live gelesen):** die Website **http://www.schrottdirektor.de/impressum.html** liefert am 09.11.2026 das Impressum wörtlich „**Schrottdirektor.de**, **Uwe Müller**, Fritz-Pütter-Str. 13, 48153 Münster, **Büro: 0251 / 275898762**, **Mobil: 0172 / 2750796**, **Mail: info@schrottdirektor.de**, USt-Nr. 337/5187/2843“; die Startseite bestätigt die Leistungszusage „Schrottabholung **kostenfrei**“ **plus** „Sie bekommen sogar zusätzlich Geld von uns — wie viel das im Einzelfall ist, hängt von der **Menge** und der **Art** des Schrotts ab“, und die Leistungsseite nennt als Serviceleistungen „Schrottabholung“ und „Containerservice für Schrottabfälle aller Art“ mit der Stoffliste „Eisen, Stahl, Kupfer, Heizungen, Rohre, Aluminium, VA-Schrott, leichter bis schwerer Mischschrott, Handels- und Maschinenguss, schwerer Altschrott, leichter Neuschrott, Elektroschrott, Waschmaschinen, Motoren, Kupferkabel, Autoschrott, Messing, Zink, Aluminiumfenster“. **Website-Status (neu, korrigierend):** die im Dossier geführte Adresse **https://www.schrottdirektor.de** ist am 09.11.2026 **nicht erreichbar** — alle HTTPS-Varianten (`www.` und ohne `www.`, IPv4 und IPv6) scheitern mit einem TLS-Fehler (curl-Exit 35, **kein** HTTP-Status), während **http://www.schrottdirektor.de/** und **http://schrottdirektor.de/** mit **HTTP 200** antworten und die Inhalte ausliefern; das Feld `website` wird deshalb von `https://www.schrottdirektor.de` auf die **erreichbare** Variante `http://www.schrottdirektor.de/` gesetzt, `website_status` bleibt `aktiv`. **Abgrenzung:** der Impressum nennt **keine** Handelsregisternummer — es ist ein **Einzelunternehmen** („Uwe Müller“) mit USt-IdNr., aber ohne Registereintrag; eine Register- oder Efb-Ergänzung existiert daher nicht. **Preise (getrennt geführt):** **Ankauf** — die Zusage „zusätzlich Geld von uns“ ist eine **Vergütungszusage ohne Betrag**; **kein** €/kg-Kurs und **keine** Zahlenliste, der Betrag hängt laut Betreiber von Menge und Art ab; **Verkauf** — keine Verkaufspreisliste; **Gebühren** — „Schrottabholung kostenfrei“ ist eine **bedingte Kostenfreiheitszusage für die Abholung**, **kein** Tarif und **kein** €/kg-Kurs; ein Container- oder Demontagetarif ist nicht genannt. **Reichweite der Aussage:** geprüft wurde die Namensvariante „Schrottdirektor“ in beiden Registerrouten, die Standortsuche in den Radien 3 km und 10 km um die Dossieradresse sowie die Namenssuche der benachbarten Treffer; Abrufdatum 09.11.2026. **Ein Register-Nichteintrag ist kein Nachweis**, dass der Betrieb keine Anerkennung besitzt: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**. **Weiterhin offen:** ob eine öffentliche Annahmestelle besteht (die Website wirbt ausschließlich **Abholung** und **Containerservice**, nicht aber eine Anlieferung für Privatkunden) und ob die Fritz-Pütter-Str. 13 eine Betriebsadresse oder eine Privatanschrift ist — das OSM-Objekt **way/300120578** trägt `addr:street=Fritz-Pütter-Straße`, `addr:housenumber=13`, `addr:postcode=48153`, `addr:city=Münster` mit **`building=apartments`**, `building:levels=2` und **ohne** `name=`, `operator=`, `shop=` oder `craft=`-Tag, also **keinen** Betriebsbezug; der Seed bleibt `status: aktiv` mit der dokumentierten Betreiber-Primärquelle, die Kontaktfelder bleiben **unverändert leer** (das Impressum ist eine **Einzelquelle**), und GEO wird nicht gesetzt.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrottdirektor ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrottdirektor ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Fritz-P%C3%BCtter-Stra%C3%9Fe%2013%2C%2048153%20M%C3%BCnster%2C%20Deutschland;7.6282875;51.9366874;7.649083;51.958507;7.607492;51.914868&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Fritz-P%C3%BCtter-Stra%C3%9Fe%2013%2C%2048153%20M%C3%BCnster%2C%20Deutschland;7.6282875;51.9366874;7.649083;51.958507;7.607492;51.914868&dst=3&is=1&e=1 ; http://www.schrottdirektor.de/impressum.html ; http://www.schrottdirektor.de/ ; http://www.schrottdirektor.de/leistungen.html ; https://api.openstreetmap.org/api/0.6/way/300120578/full.json (Abruf 09.11.2026).]
