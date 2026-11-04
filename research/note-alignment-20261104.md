# Notiz- und Format-Alignment (04.11.2026) — Builder, Prüfer-Referenz, YAML

## Anlass

Nach dem Release `3ffdf8b` (Geokodierungs-Welle 04.11. 00:00) blieb aus der Pre-Deploy-Prüfung ein
bekannter Restpunkt: der **Seed-Builder** und der **Prüfer** verstanden unter einer „Timeline-Notiz“
nicht dasselbe. Die Prüfung empfahl, diese Abweichung als eigenen, wellenunabhängigen Punkt zu
beheben. Diese Arbeit tut das — und findet dabei **neun verlorene Notizen** in der Produktion.

## Drei Regeln, die der Builder jetzt mit der Prüfer-Referenz teilt

Referenz: `scripts/release/check_note_order.py::timeline_notes` (und
`scripts/release/check_pre_wave_notes.py::timeline_blocks`, dieselbe Semantik).

| Regel | Vorher (Builder) | Nachher | Wirkung |
|---|---|---|---|
| **Eingerückte Zeile** | nach `trim()` wurde „  - …“ als **eigener** Bullet erkannt → zusätzlicher ` \| `-Trenner mitten im Satz | Fortsetzungszeile des vorigen Bullets | im Release `3ffdf8b` bereits korrigiert; Prüferreferenz stimmt jetzt auch im Builder |
| **Zweiter `## Timeline`-Abschnitt** | `break` → die **ältere** Timeline des Dossiers ging verloren | die Sammlung läuft weiter | **7 verlorene Notizen** in 4 Dossiers kehren zurück |
| **Zeile in Spalte 0 ohne Bullet** (Prosa, Tabelle, Platzhalter) | an den vorigen Bullet **angehängt** → Textmatsch in der Notiz | beendet die Notiz, wird nicht angehängt | **4 Dossiers** mit aufgerissenen Notiztexten werden sauber |

Nach der Korrektur gilt für **alle 3945 Dossiers**: `Builder == Prüfer-Referenz == Seed` — **0**
Abweichungen (vorbereitet und unabhängig nachgerechnet).

## Die neun wiederhergestellten Notizen

| Dossier | Wirkung |
|---|---|
| `he-frankfurt-60489-vebeg` | die Notiz „[Korrektur 01.10.2026 (Owner-Gate): HR-Kongruenz per Northdata nachgeholt (VEBEG GmbH, AG Frankfurt HRB 8255, Roedelheimer Bahnweg 23, 60489 Frankfurt) …]“ (451 Zeichen) war **vollständig verloren** — der Bullet trug ein fehlerhaftes `+- `-Präfix |
| `he-frankfurt-fechenheim-frankfurt-fechenheim-carl-benz-str-1` | dito: „[Korrektur 01.10.2026 (Owner-Gate): website `riwald.de` …]“ (400 Zeichen) |
| `th-greiz-greizer-metall-schrott` | **3** ältere Notizen (Recherche 08.10.2026, Import-Stand 30.09.2026, Recherche 01.10.2026) |
| `st-radegast-eks-ankauf-von-katalysatoren-und-batteri` | **2** ältere Notizen |
| `st-gardelegen-ot-jeseritz-herms-recycling-schrott-abbruch-entrumpe` | **1** ältere Notiz (Import-Stand) |
| `st-wanzleben-tannenlaufer-willi-schrotthandel` | **1** ältere Notiz (Import-Stand) |

Der Builder-Fix allein stellt **7** davon wieder her; die beiden `+- `-Bullets brauchen zusätzlich
eine Dossier-Korrektur (siehe unten).

### Warum die beiden `+- `-Notizen ans Timeline-Ende kommen

Die erste Fassung des Kandidaten setzte die beiden wiederhergestellten Notizen **an ihre
ursprüngliche Stelle** zurück. Das verletzt die **Präfix-Regel**: die Vorwellen-Notizen müssen
unverändert **und an unveränderter Position** stehen — eine in der Mitte eingefügte Notiz
verschiebt alles danach. Der Prüfer meldete das zutreffend als RED.

Regelkonform ist deshalb: die fehlerhaften `+- `-Zeilen **entfallen** an ihrer Stelle (sie waren
nie eine Notiz — die Referenz liest nur Bullets in Spalte 0), und die Notiz wird am **Timeline-Ende**
in einem neuen datierten Abschnitt `### Korrektur 04.11.2026` **wortgleich** wiederholt. Die
Vorwellen-Notizen bleiben damit unangetastet ✓.

## Die Gates waren untereinander uneinig

`wave_gate.py` benutzte einen **eigenen** Notiz-Parser: eine Zeile in Spalte 0 wurde an den
vorigen Bullet **angehängt**. Dadurch sah das Gate in den beiden `+- `-Dossiers einen
**veränderten Alt-Bullet** (Verhältnis 0,64/0,72 < 0,9 → „bestehender Bullet verändert“), während
`check_note_order` und `check_pre_wave_notes` dieselbe Datei als unverändert bewerteten. Der
Prüfer hat die Inkonsistenz ausdrücklich benannt.

`wave_gate.py` nutzt jetzt **exakt** die Referenz-Semantik. Damit entscheiden die drei Gates über
dieselbe Notizmenge — die Grundlage jeder Welle.

## Sechs Dossiers mit ungültigem YAML-Frontmatter

Der vereinfachte Builder akzeptierte einen **unquotierten** Wert mit `: `, den ein
Standard-YAML-Parser als **Mapping** liest — das Frontmatter war also nicht parsebar:

`bb-velten-kraatz-hartmut-fuhrbetrieb` · `bw-mannheim-seckenheim-specht-recycling-specht-frederik` ·
`bw-schwabisch-hall-74523-secondbuy-second-it-store` · `ni-hildesheim-schrott-handel-miro` ·
`nw-hagen-derda` · `th-muhlhausen-scholz-recycling-standort-muhlhausen`

Die betroffenen `description`-Werte sind jetzt **einfach quotiert** (YAML-Single-Quote-Skalare). Der
generierte Seed ist **byte-identisch** (gleiche md5 über alle 16 Zustandsdateien) — es ist eine
reine Formkorrektur ohne Datenwirkung.

**Wächter im Builder:** ein unquotierter Wert mit `: ` lässt den Seed-Codegen jetzt **loud**
fehlschlagen (wie es der Builder für einen Kommentar in einem Plain-Wert bereits tat). Damit
kann sich der Fehler nicht wieder einschleichen. Der Test deckt beide Richtungen ab.

## Der Degussa-Fall: die Firmierung ist geklärt

Seit dem 31.10.2026 stand „Degussa-Firmierung (Registerbestätigung) ungeklärt“ als offener Punkt
auf der Liste. Ergebnis der Prüfung:

* **Registerfirma:** **Degussa Sonne/Mond Goldhandel GmbH**, HRB **139179**, Amtsgericht Frankfurt am
  Main, USt-IdNr. DE275313528 — belegt durch das **Impressum des Betreibers**
  (`https://www.degussa.com/impressum/`, Abruf 04.11.2026: „Firmensitz: Degussa Sonne/Mond
  Goldhandel GmbH, Friedrich-Ebert-Anlage 35–37, 60327 Frankfurt am Main … HRB-Nummer: 139179“) und
  durch die Wertlagerbedingungen des Betreibers (Sitzverlegung vom Amtsgericht München, HRB 188979,
  zum 26.06.2025).
* **Abgrenzung:** Der Dossier-Name „Degussa Goldhandel GmbH“ ist die **Markenform** (Website und
  OSM), nicht die Registerfirma. **Firmensitz (60327) und Niederlassung (60325) sind zwei
  verschiedene Adressen** — die Frontmatter-Adresse ist die **Niederlassung**, live bestätigt auf
  der Niederlassungsseite des Betreibers („Niederlassung Frankfurt, Kettenhofweg 25, 60325
  Frankfurt“, `+49 69 860068-100`, `frankfurt@degussa.com`, Mo–Fr 9:00–19:00).
* **Hamburg** (`hh-altstadt-degussa-niederlassung-hamburg`): Telefon, E-Mail und Öffnungszeiten
  waren **leer** und sind jetzt belegt (Ballindamm 5, 20095 Hamburg, `+49 (0)40 3290872-0` →
  Inlandform `040 3290872-0`, `hamburg@degussa.com`, Mo–Fr 9:00–19:00).
* **Koordinaten:** Frankfurt über den **betreibereigenen** OSM-Knoten `node/10665610880`
  (`name`=Degussa Goldhandel GmbH, `addr:housenumber`=25, `addr:postcode`=60325, `shop`=gold_buyer)
  bei 50.1150814/8.6673090. Für Hamburg: in OSM ist die Hausnummer 5 **nicht** einzeln getaggt,
  sie gehört zum Adressbereich „4-5“ des Gebäudes `way/43290556` („Propfe Haus“, 7 Geschosse); die
  Koordinate ist der Mittelwert der eindeutigen Knoten dieses Gebäudes (53.5549158/10.0004556) und
  liegt innerhalb der BBox. Für beide Dossiers: die zweite Quelle ist die Niederlassungsseite des
  Betreibers.

Die Live-Datenbank hatte für beide Dossiers bereits Koordinaten (aus einem Handler); der
Dossierstand ist jetzt der dokumentierte, aus dem zitierten Objekt reproduzierbare Wert.
