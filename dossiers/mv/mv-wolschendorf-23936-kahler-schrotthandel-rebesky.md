---
slug: mv-wolschendorf-23936-kahler-schrotthandel-rebesky
name: Kähler Schrotthandel Rebesky
trader_type: schrotthaendler
state: MV
city: Wölschendorf 23936
street: Hauptstr. 7
postcode: 23936
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Kähler Schrotthandel Rebesky

## Überblick

Der Seed nennt Hauptstr. 7 in Wölschendorf; die Adresse und PLZ 23936 stammen bisher nur aus Verzeichnissen. Die Telefonnummern widersprechen sich (0171 9414788 vs. 01520 7124594), und ein Betreiberauftritt oder passender Registerbeleg fehlt. Daher kein bestätigter aktueller Ankauf, `status: pruefung` bleibt; beide Nummern sind nur Leads.

## Timeline

### Recherche 21.10.2026

- [Recherche 21.10.2026: Die Grevesmühlen-Übersichtsseite von Schrottplatz-Info direkt erneut gelesen. Sie führt Kähler Schrotthandel Rebesky mit Hauptstr. 7, 23936, ordnet den Treffer aber unter „Grevesmuehlen“ ein; die Frontmatter-Bezeichnung Wölschendorf/Ortsteil bleibt dadurch ein nicht amtlich geklärter Ortsbezug. Der Eintrag ist ein Portalbeleg, keine Betreiberbestätigung; Telefonnummernkonflikt und Status pruefung bleiben. ANKAUF: kein Kurs; VERKAUF: keine Preisliste; GEBÜHREN: keine Anliefer-, Abhol- oder Containergebühr. Keine Korrektur/Geokodierung. Quelle: https://www.schrottplatz-info.de/schrottplatz/Grevesmuehlen .]

### Recherche 07.10.2026

- [Recherche 07.10.2026: ANKAUF: Schrotthandel wird in Verzeichnissen behauptet, aber kein konkreter oder numerischer Ankaufspreis ist belegt.; VERKAUF: Keine Teile-/Materialpreisliste in den dokumentierten Verzeichniseinträgen.; GEBÜHREN: Keine Anliefer-, Abhol- oder Containergebühr belegt.; GEOKODIERUNG: Hauptstr. 7 / 23936 bleibt aggregatorbasiert; wegen ungeklärter Betreiberidentität zunächst nicht geokodieren. Telefonkonflikt separat klären.; Quelle: https://www.schrottplatz-info.de/schrottplatz/Grevesmuehlen ; https://www.schrottplatz.org/bernstorf ; 11880-Eintrag laut Recherche 02.10.2026 (Portal, abweichende Telefonnummer).]

### Importiert (Seed-Stand 2026-09-30)

- Dorf-Schrotthandel, Eisen/NE
- Adresse: Wölschendorf 23936, Hauptstr. 7

### Recherche 02.10.2026

- [Recherche 02.10.2026: Teil-Fill PLZ (2 unabhängige Portal-Quellen, Adress-Konsens) — schrottplatz-info.de + schrottplatz.org (Hauptstr. 7, 23936 Wölschendorf, Tel. 0171 9414788) vs. 11880 (23936 Bernstorf bei Grevesmühlen, Tel. 01520 7124594): ORT konsistent (Wölschendorf ist Ortsteil der Gemeinde Bernstorf — kein Widerspruch), PLZ 23936 daher gefüllt; TELEFON DIVERGIERT (0171 9414788 vs. 01520 7124594) → kein phone-Fill, beide Nummern nur Timeline. Kein Betreiber-Webauftritt, kein Register belegbar → status pruefung UNVERÄNDERT. Quelle(n): schrottplatz-info.de/schrottplatz/Grevesmuehlen; schrottplatz.org/bernstorf; 11880.com/kaehler-schrotthandel-rebesky.]

### Recherche 20.11.2026 (Welle 0600 B)
- [Recherche 20.11.2026: **Ein dritter Adressstandort unter demselben Betriebsnamen und derselben Rufnummer.** Der Brancheneintrag `Kähler Schrotthandel Rebesky` führt `Wölschendorf 13`, `23936 Wölschendorf`, die Telefonnummer `0171 9414788` und die E-Mail-Adresse `kaehlerschrotthandel@googlemail.com` mit den Leistungen `Schrott und Metallankauf frei Haus`, `Stahl und Maschinendemongtage`, `Stahlabbruch und Entkernung`, `Fahrzeugentsorgung aller Art auch Wohnwagen inklusive Abholung`, `Bereitstellung von Containern`, `Kranwagentransporte` und dem Einsatzgebiet `Im Bereich MV, SH und HH`. Die Rufnummer `0171 9414788` ist dieselbe, die Das Örtliche, schrottplatz-info und schrottplatz.org zur Seed-Adresse `Hauptstr. 7, 23936 Wölschendorf` nennen; ein weiterer Eintrag führt denselben Betrieb zusätzlich unter `23936 Bernstorf`. Der Betrieb ist damit über die Nummer stabil, die Anschrift aber zwischen drei Varianten uneinheitlich. Quelle(n): https://bundesland-mecklenburg-vorpommern.stadtbranchenbuch.com/woelschendorf/3075395.html ; https://www.dasoertliche.de/Themen/K%C3%A4hler-Schrotthandel-Rebesky-W%C3%B6lschendorf-Hauptstr ; https://www.schrottplatz.org/bernstorf/schrott-kaehler-schrotthandel-rebesky-aYEPZa.html (Abruf 20.11.2026).]
- [Recherche 20.11.2026: **Die Seed-Adresse ist geografisch falsch zugeordnet.** Das einzige Adressobjekt `Hauptstraße 7` im Kartenwerk ist `node/3982921742` mit `addr:housenumber=7`, `addr:street=Hauptstraße` und `addr:city=Bernstorf` bei **53.8293778/11.1146431**; es liegt rund **2788 m** vom Ortsteil Wölschendorf entfernt, während die Straße im Ortsteil selbst `way/556243221` mit dem Namen `Hauptstraße Wölschendorf` heißt. Die Nummer 13 existiert ebenfalls: `way/318100188` trägt `addr:housenumber=13`, `addr:street=Wölschendorf`, `addr:postcode=23936`, `addr:city=Bernstorf` und `building=yes` bei **53.8138702/11.0812804**. Beide Objekte tragen **weder** `name=` **noch** `operator=`. Die Straße im Frontmatter wird deshalb **nicht** geändert, weil für `Wölschendorf 13` nur ein Verzeichnislead vorliegt und die Kartenobjekte die Betriebszuordnung **nicht** belegen; es wird **keine** Koordinate gesetzt. Quelle(n): https://api.openstreetmap.org/api/0.6/node/3982921742.json ; https://api.openstreetmap.org/api/0.6/way/318100188/full.json ; https://api.openstreetmap.org/api/0.6/way/556243221.json (Abruf 20.11.2026).]
- [Recherche 20.11.2026: **Amtliches Fachbetrieberegister: kein Treffer.** Die Namenssuche in der Route `Entsorgungsfachbetriebe` liefert für `Kähler` und `Kaehler` jeweils **0** Treffer, die Route `Altfahrzeugverwertung` für `Kähler`, `Kaehler` und `Rebesky` jeweils **0** Treffer; die 3-km-Ortsabfrage um `Hauptstraße 7, 23936 Bernstorf` liefert in beiden Routen **0** Zeilen. **Reichweite:** ein Register-Nichteintrag ist **kein** Nachweis für fehlende Zertifizierung, für Nichtexistenz oder für Betriebsende. **Preise 20.11.2026: ANKAUF** - kein Kurs; `Schrott und Metallankauf frei Haus` ist eine Leistungszusage ohne Zahlen; **VERKAUF** - keine Liste; **GEBÜHREN** - keine Tarife, die Containerbereitstellung ist ohne Preisangabe. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Kaehler ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Rebesky (Abruf 20.11.2026).]
