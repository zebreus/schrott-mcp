---
slug: sl-freisen-konig-containerdienst-albano-wei
name: König Containerdienst (Albano Weiß)
trader_type: containerdienst
state: SL
city: Freisen
street: Pfalzstr. 1
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://koenig-containerdienst.de/schrott-metallhandel
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (Stand 2026-09-27)
provenance_ankauf_raw: ja
provenance_origin: table
---

# König Containerdienst (Albano Weiß)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2614 berechtigt als Klärfall (Einzelbeleg, kein HR). Betreiber-Seiten nennen Inh. Albano Weiß, Pfalzstr. 1, Freisen, PLZ 66629, Tel. 06857/9700942 + Mobil 0176/24964453, info@Koenig-Containerdienst.de; Schrottankauf/Abholung Privat/Gewerbe/Industrie. Ohne Register/Zweitbeleg → PLZ/Telefon/E-Mail NUR hier, Frontmatter leer; website_status aktiv (HTTP 200 Schrott/Metallhandel-Seite); Quelle(n): https://koenig-containerdienst.de/impressum + https://koenig-containerdienst.de/schrott-metallhandel (eine Quelle)]

### Importiert (Seed-Stand 2026-09-30)

- alle Metalle zu Tagespreisen, Abholung

### Recherche 31.10.2026

- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=7.1776610,49.5407173,7.2017668,49.5622313 (OSM-Way-BBox des gesamten Straßenverlaufs "Pfalzstraße", way/30094523, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=Pfalzstraße mit addr:housenumber: **21 Objekte** | **21 verschiedene Hausnummern** (roh 1, 2, 7, 8, 9, 10, 10a, 10b, 11, 12, 14, 14a, 15, 16, 17, 18, 19, 20, 22, 24, 26 — keine Bereichsnotation, daher unverändert aufgelöst). Die Dossier-Hausnummer **1 ist erfasst**: way/344569199 mit `addr:housenumber=1`, `addr:street=Pfalzstraße`, `addr:city=Freisen`, `addr:postcode=66629`, `addr:country=DE`, `building=yes` — das Objekt trägt **kein** `name=` und **kein** `operator=`. Von den 21 Adressobjekten der Straße trägt **0** ein `name=`- oder `operator=`-Tag. Im gesamten Raster tragen 96 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling/Edelmetall** sind dort **0** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-Bezug, zusätzlich ein Name mit Containerdienst-Bezug). Der einzige recycling-nah benannte Betrieb im Raster ist relation/3425009 `name=Kläranlage Asweiler-Eitzweiler` mit `operator=EVS` und `man_made=wastewater_plant` — eine **Kläranlage**, kein Schrottbetrieb. **Kein Objekt im Raster trägt `name=König Containerdienst` oder `operator=Albano Weiß`:** der Betrieb ist in OSM an Pfalzstraße 1 **nicht** kartiert. Die Adresse selbst ist durch das OSM-Adressobjekt (PLZ 66629) **bestätigt**, die Firmierung bleibt ohne zweiten Beleg. Quelle(n): https://api.openstreetmap.org/api/0.6/way/344569199.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=7.1776610,49.5407173,7.2017668,49.5622313 (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Die Betreiberseite nennt **keine** Euro-Beträge, sondern nur die Preisformel: „Wir nehmen jede Art von Metallschrott an und zahlen Ihnen den **tagesaktuellen Kilopreis**", Überschrift „**Schrottankauf zu Tageshöchstpreisen**", „Vergütung für Metallschrott stets zu tagesaktuellen Preisen". Angenommene Sorten sind konkret benannt: **Messing** (Wasserhähne, Armaturen, Fittings, Ventile, Leitungen), **Edelstahl/VA**, **Stahl/Eisen** (Armierungseisen, Torstahl, Baustahl, Rohre, Felgen, Draht, Zäune, Bleche, Platten, Stahlspäne von Dreh-, Fräs- und Bohrmaschinen), **Kabel** (mit/ohne Ummantelung, Alu- oder Kupferkabel, Erdungskabel), **Zink** (Dachrinnen, Fallrohre, Zink-Guss-Bauteile, Zinkanoden aus Batterien, Alt-Zink), **Aluminium** (Späne, Rohre, Bleche, Leitungen, Riffelbleche), **Kupfer** (Heizungsrohre, blanke Kabel, Wärmflaschen, Druckluftleitungen) und **Druckguss/Gusseisen** (Abwasserrohre, Kanaldeckel, Grauguss, Motorgehäuse, alte Pfannen, Rohrfittinge aus Temperguss) — damit ist die **Sortenliste** belegt, der **Kurs je Kilogramm** nicht. Die Seed-Angabe „alle Metalle zu Tagespreisen, Abholung" ist damit durch die Betreiberseite **gedeckt**. **Verkauf:** Kein Verkauf von Sekundärrohstoffen ausgewiesen. **Gebühren:** Die **AGB** (https://koenig-containerdienst.de/allgemeine-geschaeftsbedingungen) nennen bezifferte Konditionen der Containerstellung: „Anhand des bei der Wiegung ergebenden Füllgewichts erfolgt die Abrechnung der **Entsorgungskosten pro Tonne**" — der Containerinhalt wird **vor Ort beim Abholen verwogen**, das vom Kunden angegebene Füllgewicht ist ausdrücklich nur ein „Richtwert"; bei **Vermischung von Abfallarten** behält sich der Betrieb vor, „den gesamten Inhalt nach der **teuersten Abfallart** in Rechnung zu stellen"; „Sofern keine abweichende schriftliche Vereinbarung getroffen wurde, ist der vollständige Rechnungsbetrag **spätestens bei Abholung** des Containers fällig"; bei Nichtzahlung ist der Auftragnehmer berechtigt, die Abholung zu verweigern und den Inhalt zu entleeren, wobei „sämtliche hierdurch entstehenden Mehrkosten, insbesondere für An- und Abfahrt, Entleerung sowie eine gegebenenfalls erforderliche erneute Abholung" der Auftraggeber trägt; für nicht transportfähige Container (Haufenbildung, verrutschende Teile) trägt der Kunde „die Kosten für einen zweiten Container sowie die Stellungskosten". Die Seite „Transportkosten" (https://koenig-containerdienst.de/transportkosten/) war beim Abruf am 31.10.2026 **nicht mehr abrufbar (HTTP 404)** und ist als **tote Quelle** zu behandeln; der dort früher sichtbare Hinweis „Transport Arbeits-/ und Scherenbühnen auf Anfrage. Bei Transport mit LKW zzgl. Mautkosten. Alle Preise + MwSt." ist **nicht** mehr am Objekt prüfbar und wird deshalb **nicht** als Beleg geführt. **Die belegbare Gebührentafel steht auf der Seite „Containerdienst"** (https://koenig-containerdienst.de/containerdienst): „**14 Tage Kostenlose Containermiete! Ab dem 15. Tag NUR 1,80 € / Tag** zzgl. 19 % MwSt."; „**Anfahrtspauschale für Containerstellung bis 30 Kilometer = 99,00 €, ab 30 Kilometer = 119,00 €**, alle Preise zzgl. 19 % MwSt."; „**Mindestabnahme 2 Tonnen** pro Container"; Containergrößen **5 m³, 7 m³, 10 m³** (Bestand „von 5 m³ bis 10 m³"); „Container Verwiegung vor Ort" mit digitalem Wiegesystem der Firma „Bark" und Ausdruck des Wiegevorgangs. **Entsorgungspreise (Gebühren) je Abfallart, alle zzgl. 19 % MwSt.:** Altholz/Holz A1 bis A3 **49,00 €/m³**, Altholz/Holz A4 **65,00 €/m³**, Papier/Karton **35,00 €/m³**, Sperrmüll **289,00 €/Tonne**, Grünschnitt **39,00 €/m³**, Erde/Bodenaushub **49,00 €/Tonne** (mit 50 % Steinen **89,00 €/Tonne**), Gipskarton **169,00 €/Tonne**, Bauschutt **59,00 €/Tonne**, Baumischabfälle **289,00 €/Tonne**, Glas **115,00 €/Tonne**, Mischmüll **289,00 €/Tonne**, Bauschutt mineralisch mit Störstoffanteil **119,00 €/Tonne**. **Ankauf (Vergütung für Metallschrott):** weiterhin **kein** Euro-Betrag, nur der „tagesaktuelle Kilopreis" — die Gebührentafel betrifft ausschließlich **Entsorgung**, nicht die Ankaufvergütung. Quelle(n): https://koenig-containerdienst.de/containerdienst ; https://koenig-containerdienst.de/allgemeine-geschaeftsbedingungen ; https://koenig-containerdienst.de/schrott-metallhandel ; https://koenig-containerdienst.de/impressum (Abruf 31.10.2026)]
