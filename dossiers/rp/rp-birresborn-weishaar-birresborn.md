---
slug: rp-birresborn-weishaar-birresborn
name: Weishaar Birresborn
trader_type: sonstige
state: RP
city: Birresborn
street: 'Rom 3'
postcode: '54574'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Weishaar Birresborn

## Überblick

Der Seed nennt „Weishaar Birresborn" und das Adressfragment „Rom 3". Die Recherche vom 08.11.2026 entschlüsselt das Fragment: In OSM existiert **Rom** als Straße (way/32169027, `highway`=tertiary) und zugleich als **Wohnplatz** (node/294547784, `place`=hamlet, wikidata Q1647176, „de:Rom (Eifel)", 19 Einwohner) in der Gemeinde Birresborn, und die **Hausnummer 3** ist als Adressobjekt way/1558254895 erfasst (50.1622915 / 6.6749125). „Rom 3" ist damit eine reale Anschrift; das Frontmatter führt jetzt `street` = Rom 3 und `postcode` = 54574. Ein **Betriebsbezug** zu „Weishaar" ist damit jedoch **nicht** belegt: die offizielle Handelsregistersuche kennt für „Weishaar" fünf Registerstellen ohne jeden Schrott- oder Recyclingbezug, und das amtliche Fachbetrieberegister war am 08.11.2026 nicht erreichbar. Eine Koordinate wurde deshalb **nicht** gesetzt. „Ankauf:** keine Preise; **Verkauf:** keine Liste; **Gebühren:** keine Tarife. Status `pruefung`.

## Timeline

### Recherche 22.10.2026

- [Recherche 22.10.2026: „Weishaar“/„Schrott“ und das Seed-Fragment „Rom 3“ erneut gegen die öffentlich erreichbaren Orts-/Verwaltungsauftritte von Birresborn und der Verbandsgemeinde Gerolstein geprüft; dort kein zuordenbarer betriebsspezifischer Datensatz oder Betreiberkontakt ermittelt. Diese begrenzte öffentliche Suche ist kein Negativbeweis für Existenz oder Gewerbeanmeldung. Das Fragment „Rom 3“ ist weder als Straße/Hausnummer noch als Annahmestelle verifiziert und bleibt aus den Feldern heraus. Keine belegte Ankaufstätigkeit oder Ankaufskurse; keine Verkaufspreise oder Gebühren gefunden. Status `pruefung`, keine Schließung behauptet. Quellen/Prüfumfang: https://birresborn.de/ ; https://www.gerolstein.de/ .]

### Importiert (Seed-Stand 2026-09-30)

- Rom 3
- urspr. Website-Angabe: keine
- Adresse: Birresborn

### Recherche 02.10.2026

- [Recherche 02.10.2026: Miss — „Weishaar Birresborn" (Seed: Rom 3) ohne Treffer in Websuche (keine Betreiber-Website, kein Register, keine Verzeichnisse); Straßenfragment „Rom 3" nicht verifizierbar. Frontmatter bleibt leer; website_status=unbekannt; Klärfall. Quelle(n): keine (Negativbefund Websuche 02.10.2026).]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Das Seed-Fragment „Rom 3" ist als echte Adresse in Birresborn entschlüsselt - es ist kein unlesbarer Rest.** Die Nominatim-Suche (live am 08.11.2026) nach „Rom 3, Birresborn" liefert genau einen Treffer: **way/1558254895** mit `addr:housenumber`=**3**, `addr:street`=**Rom**, `addr:postcode`=**54574**, `addr:city`=Birresborn, `addr:county`=Vulkaneifel, `building`=**yes**, Mittelpunkt **50.1622915 / 6.6749125**. Der Straßenname selbst ist doppelt belegt: als **Straße** **way/32169027** (`highway`=tertiary, `name`=Rom, `addr:postcode`=54574, `addr:city`=Birresborn) und zusätzlich als **Wohnplatz** **node/294547784** (`place`=**hamlet**, `wikidata`=Q1647176, `wikipedia`=de:Rom (Eifel), Einwohnerzahl 19, Stand 30.06.2011). „Rom" ist damit ein **kleiner Wohnplatz an der Straße zwischen Birresborn und Birresborn-Borler**; das Seed-Fragment ist die **Hausnummer 3 in diesem Wohnplatz**. Aus dem Frontmatter-Klartext wird damit `street` = Rom 3 und `postcode` = 54574; eine Koordinate wurde **nicht** gesetzt, weil ein **Betriebsbezug** zu „Weishaar" damit **nicht** belegt ist und der frühere Klärfall „Phantom-/Karteileiche" dadurch **nicht** ausgeräumt ist. Quelle(n): https://nominatim.openstreetmap.org/search?q=Rom+3%2C+Birresborn&format=jsonv2&limit=5&addressdetails=1&extratags=1 (Abruf 08.11.2026).]
- [Recherche 08.11.2026: **Amtliche Registerprüfung: kein Weishaar-Betrieb mit Schrottbezug.** Die **offizielle** Namenssuche im Handelsregisterportal (handelsregister.de, normale Suche, Modus „alle Schlagwörter enthalten", Abruf 08.11.2026) liefert für „Weishaar" **5 Treffer**, sämtlich ohne Bezug zu Schrott, Metallrecycling oder Birresborn: „Hauck Weishaar eGbR" (AG Wittlich, GsR 556, Saarburg), „KWM Weisshaar GmbH Design und Entwicklung" (AG Mannheim, HRB 441030, Mosbach), „Schuhhaus Weishaar GmbH" (AG Darmstadt, HRB 25240, Heppenheim), „Weishaar Investments UG (haftungsbeschränkt)" (AG Hannover, HRB 229337) sowie eine Historienzeile „Möbel WUM Einrichtungs-GmbH". **Reichweite:** das Registerportal führt ausschließlich Rechtsträger mit Registereintrag; ein **Einzelunternehmer** erscheint dort **nicht** - ein Nichttreffer ist deshalb **kein** Nichtexistenzbeweis, sondern belegt nur: „unter diesem Namen ist kein eingetragener Rechtsträger mit Schrottbezug geführt". Quelle(n): https://www.handelsregister.de/rp_web/auswahl.do (Abruf 08.11.2026, Suchbegriff „Weishaar", 5 Treffer).]
- [Recherche 08.11.2026: **Das amtliche Fachbetrieberegister war am 08.11.2026 technisch nicht erreichbar; die Reichweite dieser Lücke wird festgehalten.** Die clientseitig gerenderte Vaadin-App (`fachbetrieberegister.gadsys.de`) lieferte auf die Namenssuche „Weishaar" in den Routen „Entsorgungsfachbetriebe" und „Altfahrzeugverwertung" nur das Suchformular **ohne** Trefferliste (mehrfach versucht, Wartezeit bis 60 s). Ein Registerbefund ist damit **nicht** erhoben; die EfbV- und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein Register-Nichteintrag wäre also **kein** Nachweis für fehlende Zertifizierung, für Nichtexistenz oder für Betriebsende. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Weishaar ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=Weishaar (Abruf 08.11.2026, jeweils ohne Trefferliste).]
- [Recherche 08.11.2026: **ANKAUF / VERKAUF / GEBÜHREN: keine belegte Preisliste.** Die öffentlich zugänglichen Orts- und Verwaltungsseiten von Birresborn und der Verbandsgemeinde Gerolstein enthalten keinen betriebsspezifischen Datensatz, keine Preise und keine Gebühren. **ANKAUF:** keine Kurse und keine Annahmebedingungen. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife; kommunale Wertstoffhof-Tarife wurden bewusst nicht als Händlergebühren herangezogen. Quelle(n): https://birresborn.de/ ; https://www.gerolstein.de/ (Abruf 08.11.2026).]
