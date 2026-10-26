# Recherchewelle RP: 20 Dossiers, 26.10.2026

## Ergebnis und Beleggrenzen

14 Dossiers erhielten neue Timeline-Einträge oder kuratierte Übersichten.
Sechs wurden nach der Gegenprüfung ohne redundanten Eintrag belassen:
Altenglan-Mühlbach/Hess, Dernbach, Kieffer, Ohlinger/Steel-Men Neustadt,
Becker Wittlich und Destani Zerf. Die vorhandene Timeline-Historie blieb
erhalten. Nur DMR und Ayan erhielten beleggestützte Stammdatenänderungen; es
wurden keine Koordinaten ergänzt. Produktionsdatenbanken blieben read-only.

## Owner-Prüfung der Stammdaten

- **DMR Rohstoffhandel:** Das Impressum nennt DMR Rohstoffhandel GmbH, HRB
  33481 beim Amtsgericht Kaiserslautern und Eselsfürther Bahnhof 4b,
  67657 Kaiserslautern. NorthData führt dieselbe Registerkennung und
  Anschrift; Implisense beschreibt den Unternehmensgegenstand als An-/Verkauf
  von Schrott und NE-Metallen. Der Betreiberauftritt beschreibt ebenfalls
  Schrott-/NE-Metallhandel. Name, Typ,
  Sitz, Straße, PLZ, Website und Kurzbeschreibung wurden entsprechend
  präzisiert. `status: pruefung` bleibt, weil ein öffentlicher Annahmehof nicht
  belegt ist. Telefon, E-Mail und Öffnungszeiten bleiben leer: Die Kontaktseite
  zeigt zwar Werte passend zum Impressum, ihre `tel:`- und `mailto:`-Ziele sind
  aber Vorlagen (`0000000`, `email@example.com`); die Homepage zeigt generische
  Platzhalter bei Geschäftszeiten. Kein `dropoff_json`, kein GEO.
- **Ayan Börrstadt:** Betreiber-Impressum und Leistungsseiten nennen Resul Ayan,
  Kaiserstraße 5, 67725 Börrstadt und Schrott-/Metallankauf, Containerdienst
  sowie Autorecycling. Zwei 2026 ausgestellte SMU-Zertifikate führen denselben
  Betrieb und Ort; das Entsorgungsfachbetriebs-Zertifikat ist bis 23.08.2027
  gültig und führt Sammlung/Transport, das Demontagezertifikat hat als nächste
  Prüfung Januar 2027. Die PDFs liegen auf der Betreiberwebsite und wurden
  nicht in einem Ausstellerregister gegengeprüft. Zusammen mit dem konkreten
  Seedbezug stützen sie Name, Typ, Anschrift, Website und aktuellen Betrieb.
  Die direkt geprüfte Materialankaufseite lädt ausdrücklich zum Besuch am
  Standort ein und nennt Annahme kleiner und großer Mengen; deshalb ist
  `dropoff_json.allowed` gesetzt. Das belegt keine Koordinaten. Telefon,
  E-Mail und Öffnungszeiten bleiben ohne unabhängige Bestätigung leer.

## Identitäts- und Standortgrenzen

- **Heinen Büchel:** Christopher Heinen und Karl Stenzhorn bleiben getrennte
  Verzeichnis-Leads; keine Belege verbinden sie mit dem unvollständigen Seed.
- **Gerolstein:** Auto Wirfs ist an Sarresdorfer Straße 64 als Werkstatt belegt;
  der Seed nennt Nr. 8 und keinen Betreiber. Kein Merge und keine Übertragung.
- **Hanhofen/Tavernier:** ein LinkedIn-Suchtreffer löst die offene Dublettenfrage
  mit dem Schwester-Dossier nicht. Kein Merge.
- **Tiefenthal/Götz:** Bodo Götz' Betreiberanschrift Kirchweg 9A und der
  Verzeichnislead „Götz Altwaren Metalle“, Hauptstraße 1, wurden nicht
  zusammengeführt.
- **Taurus/E&O:** Annahmeschwellen und Leistungen der E&O Entsorgung gehören
  nicht zum Taurus-Dossier, auch wenn ein früherer Registerstandort räumlich
  übereinstimmt.
- **Loritz, Strünck, Steppan und Bausch:** Social-, Betreiber- oder
  Verzeichnisfunde bleiben je nach Quelle als Lead gekennzeichnet. Ungeklärte
  Adressen wurden nicht in Stammdaten oder GEO übernommen.

## Preis- und Leistungsfunde

### Ankauf

- Keine numerischen Ankaufskurse in EUR pro Materialeinheit gefunden.
- Ayan wirbt mit Tagespreisen auf Anfrage und nennt Kupfer, Kabel, Messing,
  Zink, Aluminium, Mischschrott, Eisen, V2A, Batterien und Elektromotoren.
- Strünck bewirbt Ankauf zu Markt-/Tagespreisen ohne Preisliste. E&O nennt
  Tageshöchstpreise, ebenfalls ohne numerische Kurse; diese Betreiberangabe
  wurde ausdrücklich nicht Taurus zugerechnet.

### Verkauf

Keine separate Material-, Fahrzeug- oder Teile-Verkaufspreisliste gefunden.
Ayans Website sagt ausdrücklich, dass keine gebrauchten Kfz-Teile verkauft
werden.

### Gebühren und kostenlose Leistungen

- Bei Ayan sind Containerdienst und Abholung beschrieben, aber ohne
  numerischen Tarif; eine kostenlose Abholung wurde nicht behauptet.
- Strünck nennt Containergrößen von 5 bis 10 m³ und individuelle Abrechnung,
  aber keinen Preis.
- E&O hat betriebsbezogene, mengenabhängige Annahmebedingungen; sie sind keine
  Euro-Preisliste und wurden nicht als Taurus-Bedingungen übernommen.
- Für die übrigen geprüften Seeds wurden keine zuordenbaren numerischen
  Gebührenlisten gefunden.

## Dateien

Die 14 aktualisierten Dossiers:

- `rp-33481-dmr-rohstoffhandel-kl`
- `rp-bad-kreuznach-55543-loritz-sohn`
- `rp-borrstadt-ayan-borrstadt`
- `rp-buchel-heinen-buchel`
- `rp-gerolstein-gerolstein`
- `rp-hanhofen-67374-jenny-tavernier-schrotthandel`
- `rp-heidesheim-taurus-entsorgungs`
- `rp-koblenz-56070-strunck-mario`
- `rp-mehlingen-67678-steppan-leon`
- `rp-neustadt-meyer-lachen-speyerdorf`
- `rp-staudt-56424-bausch-henry`
- `rp-tiefenthal-gotz-altwaren-metalle-demontage-team-bod`
- `rp-trier-pfalzel-sascha-kaiser-schrott-metallhandel`
- `rp-trier-welsch-kochann-hutmacher-frankreiter`

Quellen, Erhebungsgrenzen und offene Fragen stehen in den jeweiligen
[Dossiers](../dossiers/rp/). Es gab keine Produktionsdatenbankänderung,
Koordinatenergänzung, Veröffentlichung oder Bereitstellung.
