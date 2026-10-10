# Feedback-Audit 6: Krätzen, Zinkasche/Hartzink und Lötkrätze/Altlötzinn

**Prüfdatum:** 09.10.2026
**Umfang:** Feedback #6071–6073, vorhandener Datenbestand, passende Dossiers und konkrete Händlerangebote.

## Vorgehen und Datenbankgrenze

Die drei Feedbacks wurden in `/var/lib/schrott-mcp/internal.db` gelesen; Händler, Materialien, Annahmezuordnungen und Preise wurden in `/var/lib/schrott-mcp/public.db` ausschließlich mit `sqlite3 -readonly`/`SELECT` gegengeprüft. Es gab keine Datenbankänderungen. Betreiberseiten wurden direkt abgerufen; Branchenverzeichnis-Treffer wurden nicht als Angebotsbeleg gezählt.

Bestandsabgleich:

- Materialkatalog: `zink` (ID 12), `zinn` (ID 14) und `loetzinn` (ID 7999) existieren. Kein eigenes Material für Krätze, Fehlguss, Zinkasche, Hartzink oder Lötkrätze.
- `loetzinn` hat aktuell fünf Händlerzuordnungen: Frisch Recycling (#162), BERNHARD WESTARP (#248), Efrem Gouchev (#403), MK Wertstoffhandel (#406) und HEIN (#434). Das ist generischer Lötzinn-Ankauf; daraus folgt keine Annahme von Dross/Krätze.
- `zink` hat eine aktuelle HEIN-Preisvariante „Hartzink“ mit 0,40 €/kg (Quellseite https://www.hein-schrotthandel.de/schrottpreise/, DB-Beobachtung 09.10.2026). Sie ist weder ein eigenes Material noch ein Zinkasche-Angebot.
- Die Händlerdatensätze Aulfinger (#5), Frankenberg-Metallrecycling/FMR (#350), Feinhütte Halsbrücke (#1973) und Zinnrecycling Ramm (#1198) haben im aktuellen Bestand keine `trader_materials`-Zeilen für diese Spezialfraktionen. Die bestehenden Dossiers bzw. Quellenhinweise führen einzelne Angebote dennoch im Freitext.
- Für die als neue Kandidaten geprüften Namen AS Recycling, S. Kämper, Heinz-Gerhard Nederkorn/ES Metall Trading und deren Domains wurde kein passender Händlerdatensatz oder passendes Dossier gefunden.

## #6071 — Krätze, Fehlguss und Gießerei-Rücklauf

### Belastbare Treffer

- **Jörg Aulfinger GmbH & Co. KG**, bereits im Bestand als `bw-stuttgart-moglingen-jorg-aulfinger` (#5): Die Betreiberseite nimmt „jede Form von Aluminiumguss oder Aluminiumkrätze aus Produktion und Handwerk“ an den Wertstoffhöfen an und nennt eine Vergütung nach Aluminiumbörsen-Tagespreisen. Das belegt Aluminiumkrätze und Aluguss, aber nicht ausdrücklich Fehlguss oder Gießerei-Rücklauf. Keine numerischen Preise veröffentlicht. Quelle: [Aulfinger – Abfälle](https://www.aulfinger.de/de/leistungen/abfaelle) (Abruf 09.10.2026); Betreiberidentität und Standorte siehe bereits verifiziertes Dossier.
- **S. Kämper GmbH & Co. KG**, Sprockhövel: Die Betreiberseite bezeichnet ihre Rückständeaufbereitung als Ankaufsprogramm und führt Aluminium-, Messing-, Sondermessing-, Rotguss- und Kupferkrätze sowie kupferhaltige Schlacke/Stäube auf. Das ist ein starker Materialtreffer für NE-Krätzen, aber keine spezifische Zusage für Fehlguss. Die Website zeigt Copyright 2018 und verlinkt ein Entsorgungsfachbetrieb-Zertifikat, dessen Dateiname eine Gültigkeit bis Mai 2026 ausweist; das PDF ließ sich im Recherchezugriff nicht prüfen. Daher sind Zertifikatsstatus und aktuelle Annahme-/Genehmigungslage vor einer Bestandsaufnahme telefonisch zu klären. North Data führt S. Kämper GmbH & Co. KG, HRA 7568, Gewerbestraße 11, 45549 Sprockhövel und einen Abschluss zum 31.08.2025, publiziert am 21.09.2026. Quellen: [Kämper-Website](https://www.kaemper-metalle.de/) (Rückständeaufbereitung, Impressum und Zertifikatslink); [North Data – HRA 7568](https://www.northdata.de/S%C2%B7%20K%C3%A4mper%20GmbH%20&%20Co%C2%B7%20KG,%20Sprockh%C3%B6vel/Amtsgericht%20Essen%20HRA%207568).

### Abgrenzungen und offene Belege

- Feinhütte Halsbrücke (#1973) nennt Krätzen/Aschen im Kontext zinn-, blei- und antimonhaltiger Stoffe; das ist kein Beleg für Aluminium-, Messing- oder Kupferkrätze. MAIREC (#189) meint „Gekrätze“ im Edelmetallbereich. #1837 „Krätzer“ ist nur eine Namensähnlichkeit, kein Materialtreffer.
- Aulfingers allgemeiner Ankauf von Aluminiumguss darf nicht automatisch als Fehlguss-/Rücklaufannahme ausgelegt werden. Kein geprüfter Betreiberbeleg nennt „Fehlguss“ ausdrücklich oder weist eine konkrete Gießerei-Rücklaufqualität aus. Die Kandidatenlisten aus normalen Gießereischrott-/Gießereibelieferungsseiten wurden nicht als Ankauf von Fehlguss gezählt.
- Die Betreiberseite von elorec (Essen) listet „Krätze und Abschaum“, spezifiziert aber in der Textliste nur Erst-/Zeitschmelze (z. B. Lötzinn); das Foto-/Altattribut „Aluminiumkrätze“ reicht allein nicht für eine sortenklare Zusage und wurde deshalb nicht als bestätigter Aluminiumkrätze-Ankäufer gewertet. Quelle: [elorec – Metalle](https://www.elorec.de/leistungen/metalle).

**Dossierpflege:** In [`bw-stuttgart-moglingen-jorg-aulfinger.md`](../dossiers/bw/bw-stuttgart-moglingen-jorg-aulfinger.md) wurde die belegte Aluminiumkrätze-Annahme mit der Fehlguss-Abgrenzung ergänzt. Kämper hat noch keinen Repo-Datensatz; kein neues Dossier und kein Datenbankeintrag angelegt.

## #6072 — Zinkasche und Hartzink

- Feedback und aktuelle Bestandsabfrage bestätigen: `zink` ist ein allgemeines Material; die konkrete Variante „Hartzink“ steht bei HEIN (#434) mit 0,40 €/kg. Kein eigenes Material für Hartzink/Zinkasche.
- **FMR Frankenberg-Metallrecycling**, bereits vorhanden als `by-emskirchen-frankenberg` (#350): Die Betreiberseite führt sowohl `Zn-Hartzinkblöcke` als auch `Zn-Leg.-Abschöpfblöcke` als Zink-Recycling-Fraktionen. Das ist ein zusätzlicher, konkreter Hartzink-Anbieter im vorhandenen Bestand, derzeit aber ohne `trader_materials`-Zuordnung. Die Seite führt keine Zinkasche ausdrücklich auf und veröffentlicht keine sortenbezogenen Ankaufpreise. Quelle: [FMR – Zink, Zinn, Blei](https://fmr.ag/de/leistungen/zink-zinn-blei) (Abruf 09.10.2026); Betreiber-/Registerbeleg auch im FMR-Dossier.
- **AS Recycling GmbH**, Schkopau: Die Betreiberseite hat eine eigene Zink-Seite mit Blechschrott, Druckguss, Hartzink und „Rückstände & Aschen“. Der Zink-Kontext ist klar, aber die Seite benennt nicht wörtlich „Zinkasche“; genaue Abfallart, Analyse und Annahmegrenzen sind anzufragen. Das Impressum nennt HRB 30756/Stendal; North Data zeigt Adresse Berliner Straße 130, Schkopau, und Registerpublikationen bis 09.02.2026. Kein passender Repo-Datensatz. Quellen: [AS Recycling – Zink](https://as-recycling.eu/zink/), [Impressum](https://as-recycling.eu/impressum/), [North Data – HRB 30756](https://www.northdata.de/AS%20Recycling%20GmbH,%20Schkopau/Amtsgericht%20Stendal%20HRB%2030756).
- **Metals Chemicals Maastricht B.V. (MCM)** in Maastricht kauft laut eigener Produktseite ausdrücklich Zinkasche von Lieferanten. Das ist ein echter Fachmarkt-Treffer, aber kein deutscher Händler-/Standortdatensatz; daher nicht als inländischer Datenbank-Fund gezählt. Quelle: [MCM – Zinkasche](https://www.metalschemicalsgroup.com/de/produkte/zinkasche).

**Dossierpflege:** [`by-emskirchen-frankenberg.md`](../dossiers/by/by-emskirchen-frankenberg.md) ergänzt die spezifischen FMR-Fraktionen und markiert ausdrücklich, dass Zinkasche nicht belegt ist. Keine Annahmezuordnung oder Preiszeile in der Datenbank ergänzt.

## #6073 — Lötkrätze und Altlötzinn

### Vorhandene Händler und Materialgrenze

- `loetzinn` hat fünf vorhandene Händlerzuordnungen (Frisch Recycling, BERNHARD WESTARP, Efrem Gouchev, MK Wertstoffhandel und HEIN); die `current_prices`-Tabelle enthält 60 Varianten-/Preiszeilen über diese fünf Händler, also nicht fünf einzelne Angebote. Diese Kategorie umfasst Lötzinn, nicht automatisch Lötkrätze. Auch Varianten unter `zinn` (Material ID 14) sind keine Dross-/Aschen-Klasse.
- Die Feedback-Suche fand zwei Bestandstexte: **Zinnrecycling Ramm** (#1198) nennt Zinnaschen, die nach Analyse bewertet werden; **Feinhütte Halsbrücke** (#1973) nennt Altlot sowie Krätzen/Aschen. Das sind valide Materialhinweise, aber ihre genaue Reichweite ist unterschiedlich.
- **Feinhütte Halsbrücke:** Die aktuelle Betreiberseite führt zinn-/antimonhaltige Materialien, Krätzen, Aschen/Schlämme und Lotpasten auf. Sie beschreibt Bemusterung/Analyse, LME-basierte Preisfixierung und eine Mindestmenge von 250 kg je Einheit. Das ist ein direkter Beleg für zinnhaltige Krätzen/Altlot im Industriekunden-Ankauf, jedoch kein Ankauf von Al-/Cu-Krätzen. Quelle: [Feinhütte – Ankauf von Zinn](https://www.feinhuette.de/recycling/ankauf-von-metallen/ankauf-von-zinn) (Abruf 09.10.2026).
- **Zinnrecycling Ramm:** Die Betreiberseite nennt Zinnaschen, Zinnausläufer und Zinnschmelzreste, die nach Befund/Analyse bewertet werden. Veröffentlichte Lötzinnpreise gelten ausschließlich für Paketsendungen. Die Seite benennt diese Rückstände nicht wörtlich „Lötkrätze“; keine Gleichsetzung. Der Kundenverkehr ist laut Betreiberhinweis bis einschließlich 16.10.2026 geschlossen (Paketkunden ausgenommen), Wiederöffnung ab 19.10.2026. Quelle: [Ramm – Handel](https://www.zinn-metall.de/metallankauf/handel/) (Abruf 09.10.2026); Preis-/Schließungsdetails sind bereits im Ramm-Dossier dokumentiert.

### Zusätzliche, nicht im Repo geführte Fachanbieter

- **B2B Zinnverwertung Heinz-Gerhard Nederkorn**, Mühlhausen: `metallankauf.org` und `zinnankauf.org` listen beide denselben Inhaber, dieselbe Flügelstraße 1, 99998 Mühlhausen, Telefonnummer und USt-ID. Sie sind deshalb ein Anbieter, nicht zwei. Die Betreiberseiten nennen Lötzinn, Zinnkrätze/Dross aus Sn-, SnAg- und SnPb-Systemen, Lotpasten, verbrauchtes Lot, Zinnasche und komplette Lottiegel. B2B-Anfrage mit Legierung/Menge/Fotos; persönliche Abgabe in Mühlhausen ist nicht möglich (dort nur Büro), Selbstanlieferung bei Partner-Verwertungsbetrieben nur nach Absprache. Kein Repo-Datensatz gefunden. Quellen: [Metallankauf.org – Zinnrecycling](https://www.metallankauf.org/zinnankauf.html), [Impressum](https://www.metallankauf.org/impressum.html), [Logistik](https://www.metallankauf.org/logistik.html); Zweitdomain mit gleichem Betreiber: [Zinnankauf.org – Sorten](https://www.zinnankauf.org/zinnsorten.html), [Impressum](https://www.zinnankauf.org/impressum.html).
- **ES Metall Trading & Recycling**, Inhaber Erhard Seeber, Kirchheilingen: Die Betreiberseite nennt ausdrücklich Lötzinn, Zinnkrätze, Zinnasche, Zinnschlacke, Altlot/Lotbad, Lottiegel mit Inhalt und Lotpasten als Ankaufsfraktionen. Das Impressum nennt den Betreiber und die Adresse Bahnhofsstraße 180b, 99947 Kirchheilingen. Angebot und Identität stammen bislang nur von derselben Betreiber-Domain; vor einem Datenbankeintrag ist ein zweiter unabhängiger Identitäts-/Aktivitätsbeleg erforderlich. Kein Repo-Datensatz gefunden. Quellen: [ES Metall – Zinn-Ankauf](https://metallankauf-recycling.de/startseite/sondermetalle/zinn), [Impressum](https://metallankauf-recycling.de/impressum/).

**Dossierpflege:** [`sn-halsbrucke-feinhutte-halsbrucke.md`](../dossiers/sn/sn-halsbrucke-feinhutte-halsbrucke.md) präzisiert Materialgruppen, Mindestmenge und Preisbildung; [`ni-hambergen-zinnrecycling-und-metallhandel-ramm-zinn.md`](../dossiers/ni/ni-hambergen-zinnrecycling-und-metallhandel-ramm-zinn.md) hält die erneute Prüfung, Paketbedingung und fehlende explizite Lötkrätze-Bezeichnung fest. Keine Preise, Annahmebedingungen oder Materialzuordnungen in der Datenbank angelegt.

## Ergebnis und offene Fragen

1. **#6071 ist teilweise konkretisiert:** Aulfinger belegt Aluminiumkrätze; Kämper führt mehrere NE-Krätzen als Einkaufsprogramm. Fehlguss/Gießerei-Rücklauf bleibt offen; „Aluminiumguss“ oder die Belieferung von Gießereien sind kein automatischer Fehlguss-Ankaufsbeleg.
2. **#6072 hat mindestens einen übersehenen vorhandenen Hartzink-Treffer:** FMR (#350) listet Hartzinkblöcke und Zink-Legierungsabschöpfblöcke, aber keine Zinkasche. AS Recycling ist ein guter neuer Kandidat für Zink-Hartzink und zinkbezogene Aschen/Rückstände; die exakte Fraktion muss bestätigt werden.
3. **#6073 mischt generisches Lötzinn mit metallurgischen Rückständen:** Feinhütte belegt Krätze/Aschen, Ramm Zinnasche/Schmelzreste. Zwei zusätzliche Betreiberauftritte (Nederkorn, ES Metall) nennen Lötkrätze/Dross direkt, sind aber nicht im Repo und benötigen Owner-Gate für einen Neueintrag.
4. Vor einer Katalogerweiterung sind getrennte Sorten/Qualitäten sinnvoll zu prüfen: Al-/Cu-/Messing-/Zinn-Krätze, Zinkasche, Hartzink, Lötkrätze/Dross und Fehlguss sind nicht durch die bestehenden generischen `zink`, `zinn` oder `loetzinn`-Materialien ausreichend auseinanderzuhalten. Rückstandsankauf ist überwiegend analyse-, Legierungs-, Mengen- und Herkunftsabhängig; nicht ungeprüft als pauschaler Preis oder Kundenannahme darstellen.
5. Nächste sinnvolle Klärungen: Kämper-Zertifikats-/Annahmezustand nach Ablauf des EFB-Zertifikats; AS-Recycling-Spezifikation für `Rückstände & Aschen`; konkrete Fehlguss-Qualitäten und Mindestmengen; aktive Annahme-/Transportbedingungen und unabhängige Identitätsprüfung für Nederkorn und ES Metall.
