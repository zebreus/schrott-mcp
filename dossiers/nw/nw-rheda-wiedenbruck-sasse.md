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
