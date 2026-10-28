# Merge-/Split-Recherche vom 28.10.2026 (Owner-Check-in)

Auftrag des Owners (28.10.2026): Recherche und Entwurf fuer vier Faelle aus dem
Owner-Check-in 07:55 UTC (`research/checkin-20261028-0755-owner.md`) und den
Wellenberichten `research/wave-20261028-0000-report.md` und
`research/wave-20261028-0600-report.md`.

- Fall 1: **Split** `hh-billbrook-billbrook-oberwerder-damm-1-veddel`
  (HEG vs. REMEX Nord GmbH)
- Fall 2: **Dublette** `nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger` vs.
  `nw-essen-schrotthandler-traut`
- Fall 3: **Cluster** `he-schaafheim-64850-he-reining-klaus-containerdienst` +
  `he-schaafheim-64850-he-reining-klaus` + `he-schaafheim-64850-reining-klaus-containerdienst`
- Fall 4: **Namenskorrekturen** an sechs Eintraegen

## 0. Rahmen, Regeln, Methodik

### 0.1 Was in diesem Bericht passiert und was nicht

- **Erstellt wurde nur dieser Bericht.** Es wurde **keine** Dossier-Datei angelegt oder
  geaendert, kein Commit, kein Push, kein Deploy. Die Dossiers in Abschnitt 1.5, 2.4,
  3.5 und 4.3 sind **Entwuerfe als Text** zur Owner-Freigabe.
- **Produktionsdatenbank ausschliesslich read-only.** Alle Bestandsabfragen liefen ueber
  `sqlite3 'file:/var/lib/schrott-mcp/public.db?mode=ro'` mit `PRAGMA query_only=ON`
  (Abfrage am 28.10.2026, 3938 Händler). Es wurde nichts geschrieben.
- **Keine erfundenen Werte.** Jeder Wert in den Entwuerfen ist mit einer Quelle belegt, die
  im Bericht genannt ist. Wo eine Quelle nur ein *Aggregat oder ein Portal* ist, steht das
  ausdruecklich dabei.
- **Keine Koordinate ohne zwei unabhaengige Quellen.** Wo nur eine Geometrie-Quelle
  vorlag, bleibt `lat`/`lon` leer und der Kandidat steht als Kandidat im Text.
- **Ankauf / Verkauf / Gebuehren getrennt.** Die amtliche Hamburger Aussage "gebuehrenfreie
  Annahme" ist eine Entsorgung ohne Gegenleistung und wird nirgends als Ankauf gefuehrt.
- **Negativbefunde im Register beweisen keine Nichtexistenz** (freiwillige
  Efb-Anerkennung, Einzelunternehmer ohne HRB).
- **Slugs werden nie geaendert.** Der Split behaelt den Slug der bestehenden Zeile, das
  neue Dossier bekommt einen neuen Slug.
- **Bestehende Timeline-Notizen werden nicht ueberschrieben oder geloescht.** Die
  Entwuerfe setzen jeweils **eine neue** Notiz obendrauf; bestehende `###`-Abschnitte und
  ihre Bullets bleiben unveraendert.

### 0.2 Maszstaebe, die angewandt wurden (README, Stand 26.10.2026)

- **Quellenhierarchie:** Aggregatoren (Das Oertliche, Gelbe Seiten, 11880, GoLocal, Cylex,
  schrottplatz-/schrottradar-Portale) sind **Leads, nie Evidenz**. Evidenz sind
  Betreiber-Websites (Impressum/Kontakt), Handelsregister/NorthData/Creditreform, amtliche
  Register und Veroeffentlichungen oeffentlicher Stellen.
- **Zwei-Quellen-Standard** als Normalfall; eine Betreiber-Primärquelle oder ein amtliches
  Register darf allein nur fuer die ausdruecklich gepflegten eigenen Fakten stehen, wenn
  die Anerkennungskriterien erfuellt sind (Impressum nennt Rechtstraeger **mit**
  Registernummer, Rechtstraeger ist im Register lebend, Standortseiten sind je Standort,
  Inhalt ist aktuell). Faellt ein Kriterium, gilt der Zwei-Quellen-Standard.
- **Owner-Gate-Mechanik** (`scripts/release/wave_gate.py`): neue Zeilen ohne en/em dash,
  keine Markdown-Deep-Links, jede URL mit Protokoll, neue Timeline-Bullets in der Form
  `- [ ... ]`. Die Entwuerfe in diesem Bericht sind bereits gate-konform formuliert
  (Bindestriche statt Gedankenstrichen, URLs mit `https://`).

### 0.3 Neu gelesene Quellen (alle am 28.10.2026 direkt abgerufen)

| Betrieb | Quelle | Art |
|---|---|---|
| HEG | `hamburg.de/branchenbuch/hamburg/eintrag/10220456/` | amtliches Branchenbuch |
| HEG / Stadtreinigung | `files.stadtreinigung.hamburg/SRH_25_028_Info_Broschuere_2026_Web.pdf` (S. 12) | amtliche Broschuere |
| Recyclinghöfe | `hamburg.de/.../recyclinghoefe-hamburg-379168` | amtliche Liste |
| REMEX Nord | `remex-nord.de/` (Startseite, Standort Oberwerder Damm) | Betreiber-Primärquelle |
| REMEX Nord | `remex.de/profil/standorte/` | Betreiber-Gruppe |
| REMEX Nord | `schrottregister.pages.dev/betrieb-remex-nord-gmbh-hamburg(-2)` | amtliches Fachbetrieberegister |
| Traut | `schrottfinder.de/schrotthandler-traut-gelsenkirchen-gelsenkirchen/`, `.../essen/` | Aggregator (Lead) |
| Schaafheim | `containerdienst-schaafheim.de/assets/{Seo-DTvtlKDA,UeberUns-CamdrXxU,Kontakt-u3FhrzKh,Impressum-DA8ZMBv_}.js` | Betreiber-Primärquelle (JS-Module) |
| Schaafheim | 11880-Eintrag (JSON-LD), Gelbe Seiten | Verzeichnis (Lead) |
| Laroche | `laroche-recycling.com/` (Start, Über uns, Impressum, Kontakt) | Betreiber-Primärquelle |
| AC Containerdienst | `ac-containerdienst.de/impressum` | Betreiber-Primärquelle |
| Schubert / Lenhardt / Ballweg | 11880 + Das Oertliche bzw. Gelbe Seiten | Verzeichnis (Lead) |
| Bobzien | `kleinanzeigen.de/pro/Karsten-Bobzien`, `schrottplatz-info.de` | Betreiber-Profil / Verzeichnis |

**Abrufgrenzen, die dokumentiert werden muessen:** `heg.hamburg` liefert HTTP 403
(Deshalb bleibt `website_status: blockiert`); `remex-nord.de` und `remex.de` sind durch ein
JS-Bot-Challenge geschuetzt und nur mit vollstaendigen Browser-Headern lesbar; das
Efb-Zertifikat-PDF `REMEX_Nord_EfB_Zertfikat_14-10-2027.pdf` war nicht abrufbar (403) und
wurde daher **nicht** als gelesen behandelt, die Zertifikatsnummer stammt aus dem
amtlichen Register.

---

## 1. Fall 1 - Split: `hh-billbrook-billbrook-oberwerder-damm-1-veddel`

### 1.1 Befund in einem Satz

Die Zeile hat zwei **verschiedene Rechtstraeger an zwei verschiedenen Orten** in einer
Anschrift vereint; sie wird in zwei Dossiers getrennt, die bestehende Zeile bleibt die
HEG, die REMEX Nord GmbH bekommt ein eigenes Dossier.

### 1.2 Was gehoert zu wem

| Merkmal | HEG Hamburger Entsorgungsgesellschaft mbH | REMEX Nord GmbH, Betriebsstätte Oberwerder Damm |
|---|---|---|
| Anschrift | Bredowstraße 13, 22113 Hamburg (Billbrook) | Oberwerder Damm 1-5 laut amtlichem Register, "Oberwerder Damm 3-5" laut Betreiber, 20539 Hamburg-Veddel |
| Telefon | 040 / 257 620 70, Fax 040 / 257 620 75 | 040 781107-0, Fax 040 781107-14 |
| E-Mail | heg@srhh.de | info-nord@remex.de (Gruppenpostfach der REMEX Nord) |
| Website | heg.hamburg (HTTP 403) | remex-nord.de |
| Rechtsform / Register | GmbH, HRB 53960 AG Hamburg, 100 % Tochter der Stadtreinigung Hamburg | GmbH, HRB 133957 AG Hamburg, Geschäftsanschrift Einsiedeldeich 15, 20539 Hamburg |
| Tätigkeit | Containerdienst (10 bis 35 Kubik), Gewerbeabfälle, Sperrmüllservice, Wertstofftonne | Efb-Betrieb: Zwischenlager und Behandlung mineralischer Abfälle, Umschlag, Konditionierung |
| Efb-Zertifikat | nicht im amtlichen Register (Negativbefund ohne Beweiswert) | **ZZBE00100010**, Tätigkeiten **Lagern** und **Behandeln**, 32 Abfallschlüssel |
| Öffnungszeiten | keine fuer die Geschaeftsanschrift | Mo-Fr 07:00-17:00 (Betreiber-Standortseite) |
| Status der Zeile | Entsorgungs-/Containerdienstleister, **kein** Schrottankauf | Efb-Anlage fuer Gewerbe, Industrie, Kommunen, **keine** oeffentliche Schrottannahme gegen Verguetung |

### 1.3 Belege

**HEG (amtlich).** Das Branchenbuch der Stadt Hamburg fuehrt die HEG mit Eintragsnummer
10220456, "Letzte Aktualisierung: 13.03.2026":

> HEG Hamburger Entsorgungsgesellschaft mbH ... Adresse Bredowstraße 13, 22113 Hamburg
> (Billbrook) ... Telefon 040 / 257 620 70, Telefax 040 / 257 620 75, E-Mail heg@srhh.de,
> Website heg.hamburg ... Kategorie "Container und Entsorgung".

Die Broschuere INFO 2026 der Stadtreinigung (Seite 12, "12 X IN HAMBURG: RECYCLINGHOFE")
legt fest:

> ANLIEFERUNGEN IN HAUSHALTSUEBLICHER MENGE* UND ART SIND GEBUEHRENFREI** FUER:
> ... Schrott (keine Kfz-Teile), Kabel, Buntmetalle ...
> **Nur aus Hamburger Privathaushalten.
> Abgabe nur in haushaltsueblicher Menge und nach vorhandener Annahme-Kapazitaet vor Ort.

Das ist eine **gebuehrenfreie Entsorgung ohne Gegenleistung, also kein Ankauf**. Die
amtliche Liste der Recyclinghoefe weist den **Recyclinghof Billbrook an der
Liebigstraße 66** aus (Mo bis Fr 8 bis 17 Uhr, Sa 8 bis 17 Uhr), nicht an der
Bredowstraße 13. Die HEG wird in der Broschuere fuer die Grosscontainer von 10 bis 35
Kubik (Tel. 040/2576-2011) und fuer die Gewerbeabfaelle genannt.

**REMEX Nord (Betreiber + amtliches Register).** Die Standortseite des Betreibers
(`remex-nord.de`) fuehrt:

> REMEX Nord GmbH, Betriebsstätte Oberwerder Damm, Oberwerder Damm 3-5, 20539 Hamburg,
> T +49 40 781107-0, F +49 40 781107-14 ... Oeffnungszeiten Mo-Fr: 07:00 - 17:00
> Standortbeschreibung: Anlage zur Zwischenlagerung von gefaehrlichen und nicht
> gefaehrlichen Abfallen ... Anlage zur Behandlung ... sowie Konditionierung von
> fluessigen und staubfoermigen Abfallen ...
> Zertifikat Entsorgungsfachbetrieb (PDF)

Derselbe Auftritt nennt die Gespanntschaft "REMEX Nord GmbH (ehemals ETH Umwelttechnik
GmbH)", "Seit 2019 gehoeren unsere Standorte in Hamburg zur REMEX-Gruppe" und weist
fuer die Verwaltung die Geschaeftsfuehrer Guido Bruning (040 781107-33,
guido.bruening@remex.de) und Torsten Kurth (040 781107-28, torsten.kurth@remex.de) aus.
Das amtliche Fachbetrieberegister (Auszug 11.08.2026) fuehrt dieselbe Gesellschaft mit
**Oberwerder Damm 1 - 5** (Lagern, Behandeln; Zertifikatsnummer **ZZBE00100010**) und
zusaetzlich mit **Einsiedeldeich 15** (Handeln, Makeln; dieselbe Nummer).

**Adressabweichung 1-5 vs. 3-5 (offen, dokumentiert).** Das amtliche Register nennt
"1 - 5", der Betreiber "3 - 5". Hausnummer 3 ist laut eigenem Impressum von der ABN
Baustoffueberwachung Nord mbH (Niederlassung Hamburg) belegt. Die frueher dort
ansässige ETH Wertstoffrecycling GmbH (HRB 115679 Hamburg) fuehrte im Efb-Zertifikat
ZZBT005000349003 die Anschrift "Oberwerder Damm 1-5". Die Registerform "1-5" hat den
Vorrang, weil genau unter dieser Anschrift das Zertifikat ZZBE00100010 erteilt ist; die
Betreiberform wird im Text mitgeschrieben.

**Negativbefund.** Im amtlichen Fachbetrieberegister ist fuer die HEG **kein** Eintrag
vorhanden. Das ist ein Negativbefund ohne Beweiswert (Efb-Anerkennung ist freiwillig,
ein Einzelunternehmer ohne Efb wird nicht erfasst) und wurde bereits am 28.10.2026 so
dokumentiert.

### 1.4 Empfehlung an den Owner

- **Die bestehende Zeile bleibt die HEG** (Bredowstraße 13). Das ist die belegte
  Geschaeftsanschrift eines eingetragenen Unternehmens; das Namensfragment
  "Oberwerder Damm 1 Veddel" wird aus dem Namen entfernt.
- **Die REMEX Nord GmbH bekommt ein eigenes Dossier** mit neuem Slug
  `hh-veddel-remex-nord-gmbh` (state `hh`, city `veddel`, name `remex-nord-gmbh`).
  Der Slug der bestehenden Zeile bleibt unveraendert.
- Der Recyclinghof Billbrook an der Liebigstraße 66 wird **nicht** in die HEG-Zeile
  uebernommen: er ist eine andere Anschrift und gehoert zur Stadtreinigung, nicht zur
  Geschaeftsanschrift der HEG.
- Offen als Owner-Entscheidung: `trader_type` der HEG-Zeile (`sonstige` -> `containerdienst`
  empfohlen, siehe 1.5) und der Straßenschreibweise (`Bredowstr. 13` -> `Bredowstraße 13`).

### 1.5 Entwurf (a): geaenderte Felder der bestehenden Zeile

Datei: `dossiers/hh/hh-billbrook-billbrook-oberwerder-damm-1-veddel.md`

| Feld | jetzt | neu | Pflicht | Beleg |
|---|---|---|---|---|
| `name` | `Billbrook + Oberwerder Damm 1 Veddel` | `HEG Hamburger Entsorgungsgesellschaft mbH` | ja | Branchenbuch Hamburg Eintragsnummer 10220456 |
| `description` | beschreibt zwei Betriebe | beschreibt nur die HEG, verweist auf das neue Dossier | ja | s. Entwurf |
| `street` | `Bredowstr. 13` | `Bredowstraße 13` | optional (kosmetisch) | Branchenbuch Hamburg |
| `trader_type` | `sonstige` | `containerdienst` | optional (Owner-Typfrage) | Branchenbuch-Kategorie "Container und Entsorgung", Broschuere "Container von 10-35 m3" |
| `city`, `postcode`, `phone`, `email`, `website`, `website_status`, `status`, `provenance_*` | | unveraendert | | |

Unveraendert bleiben bewusst: `status: pruefung` (die HEG ist amtlich belegt, aber an der
Bredowstraße 13 ist **keine** oeffentliche Schrottannahme und kein Ankauf belegt),
`opening_hours: ''` (die Zeiten 8-17 Uhr gelten dem Recyclinghof Liebigstraße 66),
`website_status: blockiert` (HTTP 403 am 28.10.2026).

```markdown
---
slug: hh-billbrook-billbrook-oberwerder-damm-1-veddel
name: HEG Hamburger Entsorgungsgesellschaft mbH
trader_type: containerdienst
state: HH
city: Billbrook
street: Bredowstraße 13
postcode: '22113'
phone: 040 257 620 70
email: heg@srhh.de
opening_hours: ''
website: https://www.heg.hamburg/
website_status: blockiert
status: pruefung
description: 'Geschäftsanschrift der HEG Hamburger Entsorgungsgesellschaft mbH (HRB 53960 AG Hamburg, 100 Prozent Tochter der Stadtreinigung Hamburg, 2021 aus HEG und WERT Wertstoff-Einsammlung verschmolzen) an der Bredowstraße 13, 22113 Hamburg-Billbrook: Containerdienst mit Großcontainern von 10 bis 35 Kubik (Servicetelefon 040/2576-2011), Gewerbeabfälle und Sperrmüllservice für Industrie, Handwerk, Gastronomie und Handel. Die gebührenfreie Annahme von Schrott (keine Kfz-Teile), Kabel und Buntmetallen aus Hamburger Privathaushalten ist eine amtliche Aussage der Stadtreinigung für ihre zwölf Recyclinghöfe; der Recyclinghof Billbrook liegt an der Liebigstraße 66 und damit an einer anderen Anschrift als diese Geschäftsanschrift. Die Annahme erfolgt ohne Gegenleistung, es sind keine Ankaufspreise, keine Verkaufspreise und keine Gebührentarife belegt. Der zuvor mitgeführte zweite Betrieb, die REMEX Nord GmbH mit Betriebsstätte Oberwerder Damm (Efb-Zertifikat ZZBE00100010), ist in das eigene Dossier hh-veddel-remex-nord-gmbh ausgelagert.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 3
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# HEG Hamburger Entsorgungsgesellschaft mbH

## Überblick

Die Zeile fuehrt nach dem Split vom 28.10.2026 nur noch die HEG Hamburger
Entsorgungsgesellschaft mbH an der Bredowstraße 13, 22113 Hamburg-Billbrook. Belege:
amtliches Branchenbuch der Stadt Hamburg (Eintragsnummer 10220456, zuletzt aktualisiert
13.03.2026) mit Anschrift, Telefon 040 / 257 620 70, Telefax 040 / 257 620 75, E-Mail
heg@srhh.de und Website heg.hamburg; Handelsregister HRB 53960 AG Hamburg. Die HEG ist
eine 100 Prozent Tochter der Stadtreinigung Hamburg und aus HEG und WERT
Wertstoff-Einsammlung hervorgegangen (Verschmelzung 17.08.2021). Die eigene Domain
heg.hamburg antwortet weiterhin mit HTTP 403, deshalb bleibt website_status blockiert
und es wurde kein Impressum als gelesen behandelt.

Die gebuehrenfreie Annahme von Schrott, Kabel und Buntmetallen aus Hamburger
Privathaushalten ist eine amtliche Aussage der Stadtreinigung fuer ihre Recyclinghoefe
(Broschuere INFO 2026, Seite 12). Der Recyclinghof Billbrook liegt nach der offiziellen
Liste der Stadtreinigung an der Liebigstraße 66 (Mo bis Fr 8 bis 17 Uhr, Sa 8 bis 17
Uhr), also an einer anderen Anschrift als die Geschaeftsanschrift. Die Offnungszeiten
bleiben deshalb leer.

ANKAUF: nicht belegt. Die Annahme ist eine gebuehrenfreie Entsorgung ohne
Gegenleistung, kein Ankauf. VERKAUF: keine Liste. GEBUEHREN: keine Tarife fuer
Container, Abfall oder Entsorgung veroeffentlicht; die Broschuere nennt Container von
10 bis 35 Kubik und die Servicetelefonnummer 040/2576-2011, aber keine Preise.

Der zweite zuvor mitgefuehrte Betrieb, die REMEX Nord GmbH mit Betriebsstaette
Oberwerder Damm (Efb-Zertifikat ZZBE00100010), ist in das eigene Dossier
hh-veddel-remex-nord-gmbh ausgelagert. Der Slug dieser Zeile bleibt unveraendert.

## Timeline

### Recherche 28.10.2026: Split der Zeile, Teil 1 bleibt die HEG

- [Recherche 28.10.2026: SPLIT entschieden: Die Zeile enthielt zwei Rechtstraeger an zwei Orten. Teil 1 ist die HEG Hamburger Entsorgungsgesellschaft mbH (HRB 53960 AG Hamburg, 100 Prozent Tochter der Stadtreinigung Hamburg, 2021 aus HEG und WERT Wertstoff-Einsammlung verschmolzen); amtlicher Beleg ist das Branchenbuch der Stadt Hamburg mit Eintragsnummer 10220456 und "Letzte Aktualisierung: 13.03.2026": "HEG Hamburger Entsorgungsgesellschaft mbH", "Adresse: Bredowstraße 13, 22113 Hamburg (Billbrook)", "Telefon: 040 / 257 620 70", "Telefax: 040 / 257 620 75", "E-Mail: heg@srhh.de", "Website: heg.hamburg", Kategorie "Container und Entsorgung". Die Betreiberdomain heg.hamburg liefert am 28.10.2026 weiterhin HTTP 403, deshalb bleibt website_status blockiert und es wurde kein Impressum als gelesen behandelt. Teil 2, Oberwerder Damm, ist die REMEX Nord GmbH und wurde in das neue Dossier hh-veddel-remex-nord-gmbh ausgelagert; der Slug dieser Zeile bleibt unveraendert, kein Slug wurde geaendert und keine bestehende Notiz entfernt. ANNAHME UND GEBUEHREN: Die Broschuere INFO 2026 der Stadtreinigung (Seite 12, "12 X IN HAMBURG: RECYCLINGHOFE") legt fest "ANLIEFERUNGEN IN HAUSHALTSUEBLICHER MENGE UND ART SIND GEBUEHRENFREI FUER: ... Schrott (keine Kfz-Teile), Kabel, Buntmetalle", mit der Fussnote "Nur aus Hamburger Privathaushalten" und der Auflage "Abgabe nur in haushaltsueblicher Menge und nach vorhandener Annahme-Kapazitaet vor Ort". Das ist eine gebuehrenfreie Entsorgung ohne Gegenleistung und damit KEIN Ankauf; Ankaufspreise, Verkaufspreise und Gebuehrentarife sind fuer die HEG nicht belegt. Die offizielle Liste der Recyclinghoefe weist den Recyclinghof Billbrook an der Liebigstraße 66 aus (Mo bis Fr 8 bis 17 Uhr, Sa 8 bis 17 Uhr), nicht an der Bredowstraße 13; die Offnungszeiten bleiben deshalb leer. NEGATIVBEFUND: Im amtlichen Fachbetrieberegister (Auszug 11.08.2026) ist fuer die HEG kein Eintrag verzeichnet; da die Efb-Anerkennung freiwillig ist, beweist das keine Nichtexistenz. Die HEG tritt in diesem Dossier als Entsorgungs- und Containerdienstleister auf, nicht als Schrotthaendler. Quelle(n): https://www.hamburg.de/branchenbuch/hamburg/eintrag/10220456/ ; https://files.stadtreinigung.hamburg/SRH_25_028_Info_Broschuere_2026_Web.pdf ; https://www.hamburg.de/leben-in-hamburg/wohnen/stadtreinigung/recyclinghoefe-hamburg-379168 ; https://www.northdata.de/HEG+Hamburger+Entsorgungsgesellschaft+mbH,+Hamburg/HRB+53960 ; https://schrottregister.pages.dev/ort-hamburg ; https://www.heg.hamburg/ (403, Abruf 28.10.2026).]

### Recherche 28.10.2026
... (bestehende drei Bullets unveraendert)

### Importiert (Seed-Stand 2026-09-30)
... (bestehende zwei Bullets unveraendert)

### Recherche 05.10.2026
... (bestehende zwei Bullets unveraendert)
```

*Hinweis zum Entwurf:* das Repo's eigene Schreibweise nutzt Umlaute; die
gate-relevanten Zeilen sind auf Bindestriche umgestellt, damit der Wave-Gate keine
Gedankenstriche in neuen Zeilen meldet. Die Beamtentexte muessen beim Uebertragen auf
die Originalschreibweise (mit Umlauten) zurueckgegeben werden.

### 1.6 Entwurf (b): neues Dossier fuer die REMEX Nord GmbH

Vorgeschlagene Datei: `dossiers/hh/hh-veddel-remex-nord-gmbh.md`
(Slug-Muster `<state>-<city>-<name>`: `hh` / `veddel` / `remex-nord-gmbh`).
Der Slug ist neu und kollidiert nicht.

**Feldentscheidungen mit Beleg:**

| Feld | Wert | Beleg / Regel |
|---|---|---|
| `name` | `REMEX Nord GmbH (Betriebsstätte Oberwerder Damm)` | Betreiber-Standortseite + Register; der Klammerzug benennt den Standort, weil die Gesellschaft vier Standorte in Hamburg hat |
| `trader_type` | `schrottplatz` | Efb-anerkannte Anlage mit 32 Abfallschluesseln, davon 170401-170407, 191001, 191002, 200140 Metalle; **Owner-Typfrage**, Alternative `sonstige` (vgl. `rp-koblenz-tsr-recycling-nl-koblenz` = `sonstige`) |
| `city` | `Veddel` | Bezirk Hamburg-Mitte, Stadtteil Veddel; OSM fuehrt die Oberwerder-Damm-Adressen zum Stadtteil Veddel |
| `street` | `Oberwerder Damm 1-5` | amtliches Fachbetrieberegister (Auszug 11.08.2026), unter dieser Anschrift ist ZZBE00100010 erteilt; die Betreiberform "3-5" steht im Text |
| `postcode` | `20539` | Register und Betreiber |
| `lat` / `lon` | **leer** | Regel "keine Koordinate ohne zwei unabhaengige Quellen": fuer den Oberwerder Damm gibt es nur OSM-Adressknoten (Node 3160813999 Oberwerder Damm 1 = 53.5276477/10.0296576, Node 3160814003 = 53.5275573/10.029583, Node 3160814009 Oberwerder Damm 5 = 53.5274801/10.0295111) und keine zweite unabhaengige Punktquelle (11880 fuehrt nur die Verwaltung Einsiedeldeich 15, Das Oertliche kennt die Gesellschaft nicht). Die Kandidaten stehen als Kandidaten in der Notiz. |
| `phone` | `040 781107-0` | Betreiber-Standortseite; 11880 nennt dieselbe Nummer fuer die Verwaltung |
| `email` | `info-nord@remex.de` | REMEX-Gruppenportal `remex.de/profil/standorte` verlinkt dieses Postfach fuer die Betriebsstaetten der REMEX Nord; auf `remex-nord.de` ist die Adresse hinter einem mailto-Token verborgen, die Domain `@remex.de` steht im HTML |
| `opening_hours` | `Mo-Fr 07:00-17:00` | Betreiber-Standortseite Oberwerder Damm |
| `website` | `https://www.remex-nord.de/` | Betreiber |
| `website_status` | `aktiv` | Abruf am 28.10.2026 mit HTTP 200 (Bot-Challenge nur bei einfachen Abrufen) |
| `status` | `aktiv` | Betreiberauftritt, Registereintrag HRB 133957, Efb-Registereintrag ZZBE00100010 |
| `certifications` | Efb-Zertifikat ZZBE00100010 | amtliches Fachbetrieberegister |
| `provenance_section` | `Recherche Split 28.10.2026` | Konvention der neuen Dossiers dieser Woche |
| `provenance_ankauf_raw` | `unbekannt` | kein Ankaufsangebot des Betreibers |

```markdown
---
slug: hh-veddel-remex-nord-gmbh
name: REMEX Nord GmbH (Betriebsstätte Oberwerder Damm)
trader_type: schrottplatz
state: HH
city: Veddel
street: Oberwerder Damm 1-5
postcode: '20539'
phone: 040 781107-0
email: info-nord@remex.de
opening_hours: Mo-Fr 07:00-17:00
website: https://www.remex-nord.de/
website_status: aktiv
status: aktiv
description: 'Betriebsstätte Oberwerder Damm der REMEX Nord GmbH (HRB 133957 AG Hamburg, Geschäftsanschrift Einsiedeldeich 15, 20539 Hamburg; seit 2019 Tochter der REMEX-Gruppe, zuvor ETH Umwelttechnik GmbH): Entsorgungsfachbetrieb nach § 56 KrWG mit der Zertifikatsnummer ZZBE00100010 und den amtlich zertifizierten Tätigkeiten Lagern und Behandeln. Der Registerauszug umfasst 32 Abfallschlüssel, darunter 170401 Kupfer/Bronze/Messing, 170402 Aluminium, 170403 Blei, 170404 Zink, 170405 Eisen und Stahl, 170406 Zinn, 170407 gemischte Metalle, 191001 Eisen- und Stahlabfälle, 191002 NE-Metall-Abfälle und 200140 Metalle. Die Standortbeschreibung des Betreibers nennt eine Anlage zur Zwischenlagerung sowie zur Behandlung und Konditionierung von gefährlichen und nicht gefährlichen Abfällen (kontaminierte Böden, Bauschutt, Straßenaufbruch, Ofenausbruch, Aschen, Schlacken, Schlämme, Baggergut, Bioabfälle, Grünschnitt, Straßenkehricht) sowie die Konditionierung von Schlämmen mit Flugaschen, Filterstäuben und Rauchgasreinigungsrückständen. Zielgruppe sind Gewerbe, Industrie und Kommunen; eine Annahme von Privatschrott gegen Vergütung wird nicht angeboten und ist nicht belegt. ANKAUF: keine Ankaufspreise veröffentlicht. VERKAUF: keine Preisliste. GEBÜHREN: keine öffentlichen Tarife, Entsorgung und Verwertung werden individuell angeboten. Die amtliche Registeranschrift lautet Oberwerder Damm 1-5, die Standortseite des Betreibers nennt Oberwerder Damm 3-5; die Abweichung ist dokumentiert und nicht aufgelöst. Hausnummer 3 des Oberwerder Damms ist laut eigenem Impressum von der ABN Baustoffüberwachung Nord mbH (Niederlassung Hamburg) belegt. Die zweite Betriebsstätte der Gesellschaft am Einsiedeldeich 15 (Tätigkeiten Handeln und Makeln, gleiche Zertifikatsnummer ZZBE00100010) ist nicht Teil dieses Datensatzes.'
dropoff_json: ''
pickup_json: ''
certifications: '["Entsorgungsfachbetrieb § 56 KrWG, Zertifikatsnummer ZZBE00100010 (amtliches Fachbetrieberegister, Auszug 11.08.2026), zertifizierte Tätigkeiten Lagern und Behandeln"]'
provenance_section: 'Recherche Split 28.10.2026'
provenance_ankauf_raw: unbekannt
provenance_origin: table
---

# REMEX Nord GmbH (Betriebsstätte Oberwerder Damm)

## Überblick

Zertifizierter Entsorgungsfachbetrieb der REMEX Nord GmbH an der Oberwerder Damm in
Hamburg-Veddel. Das amtliche Fachbetrieberegister (Auszug 11.08.2026) fuehrt die
Gesellschaft an "Oberwerder Damm 1 - 5, 20539 Hamburg" mit der Zertifikatsnummer
ZZBE00100010 und den Taetigkeiten Lagern und Behandeln; dieselbe Nummer traegt der
zweite Registereintrag am Einsiedeldeich 15 mit den Taetigkeiten Handeln und Makeln.
Die Standortseite des Betreibers nennt die Betriebsstaette mit "Oberwerder Damm 3-5" und
beschreibt eine Anlage zur Zwischenlagerung und Behandlung gefaehrlicher und nicht
gefaehrlicher mineralischer Abfaelle mit Umschlag- und Konditionierungsfunktion.

Die Gesellschaft ist HRB 133957 AG Hamburg, Geschaeftsanschrift Einsiedeldeich 15,
20539 Hamburg, und gehoert seit 2019 zur REMEX-Gruppe (Dusseldorf); sie fuehrte zuvor
als ETH Umwelttechnik GmbH, das Efb-Zertifikat der Vorgaengerin ETH Wertstoffrecycling
GmbH nannte denselben Standort Oberwerder Damm 1-5.

**Ankauf, Verkauf, Gebuehren:** keine Ankaufspreise, keine Verkaufspreisliste und keine
oeffentlichen Gebuehrentarife. Die Anlage ist ein BImSchG-Standort fuer gewerbliche
und kommunale Abfallstroeme; eine oeffentliche Annahme von Privatschrott gegen
Verguetung wird nicht angeboten und ist nicht belegt.

**GEO:** keine Koordinate gesetzt. Fuer den Oberwerder Damm existieren nur
OSM-Adressknoten (Oberwerder Damm 1 = 53.5276477/10.0296576, Oberwerder Damm 3 =
53.5275573/10.029583, Oberwerder Damm 5 = 53.5274801/10.0295111) und keine zweite
unabhaengige Punktquelle; zusaetzlich ist die Adressform zwischen amtlichem Register
(1-5) und Betreiber (3-5) offen. Die Kandidaten sind nicht als Dossier-Koordinate
uebernommen.

## Timeline

### Recherche 28.10.2026: Efb-Betrieb am Oberwerder Damm als eigenes Dossier

- [Recherche 28.10.2026: AUSGLAGERUNG aus dem Dossier hh-billbrook-billbrook-oberwerder-damm-1-veddel, das zwei Betriebe in einer Zeile gefuehrt hatte. Amtliches Fachbetrieberegister (Auszug 11.08.2026, Spiegel schrottregister.pages.dev): "REMEX Nord GmbH ist im amtlichen Register mit der Anschrift Oberwerder Damm 1 - 5, 20539 Hamburg gefuehrt", Zertifizierte Taetigkeiten "Lagern" und "Behandeln", 32 Abfallschluessel (020110, 120101-120104, 150104, 160109, 160117, 160118, 160120, 160801-160803, 170401-170407, 170409, 170410, 170411, 191001, 191002, 191005, 191006, 191202, 191203, 200135, 200136, 200140), Zertifikatsnummer ZZBE00100010; ein zweiter Registereintrag fuehrt dieselbe Gesellschaft am Einsiedeldeich 15, 20539 Hamburg mit den Taetigkeiten "Handeln" und "Makeln" und 62 Abfallschluesseln. Betreiber-Primärquelle, am 28.10.2026 direkt gelesen (remex-nord.de, Startseite mit der Standortliste): "REMEX Nord GmbH, Betriebsstaette Oberwerder Damm, Oberwerder Damm 3-5, 20539 Hamburg, Deutschland, T +49 40 781107-0, F +49 40 781107-14", Oeffnungszeiten "Mo-Fr: 07:00 - 17:00", Standortbeschreibung "Anlage zur Zwischenlagerung von gefaehrlichen und nicht gefaehrlichen Abfallen: Zwischenlager fuer eine Vielzahl von Abfallen, darunter kontaminierte Boeden, Bauschutt, Strassenaufbruch, Giessereirestande, Ofenausbruch, Aschen, Schlacken, Schlaemme, Dachpappe, Baggergut sowie organische Abfaelle wie Bioabfaelle, Gruenschnitt, Strassenkehricht" und "Anlage zur Behandlung ... sowie Konditionierung von fluessigen und staubfoermigen Abfallen"; Zertifikate: Efb (Zertifikat Entsorgungsfachbetrieb, PDF), DIN EN ISO 45001 und DIN EN ISO 9001. Der Auftritt nennt die Gesellschaft "REMEX Nord GmbH (ehemals ETH Umwelttechnik GmbH)", "Seit 2019 gehoeren unsere Standorte in Hamburg zur REMEX-Gruppe", vier Betriebsplaetze, sechs nach BlmSchG genehmigte Anlagen, 250.000 t/a Anlagenkapazitaet und den Einzugsbereich Bremen, Hamburg, Mecklenburg-Vorpommern, Niedersachsen und Schleswig-Holstein; die Verwaltung am Einsiedeldeich 15 nennt die Geschaeftsfuehrer Guido Bruning (T +49 40 781107-33, guido.bruening@remex.de) und Torsten Kurth (T +49 40 781107-28, torsten.kurth@remex.de). Register: REMEX Nord GmbH, HRB 133957 AG Hamburg, EUID DEK1101R.HRB133957, Geschaeftsanschrift Einsiedeldeich 15, 20539 Hamburg, Stammkapital 310.100,00 EUR (Stand 28.01.2025). Adressabweichung, dokumentiert und nicht aufgeloest: das amtliche Register nennt "Oberwerder Damm 1 - 5" (unter dieser Anschrift ist ZZBE00100010 erteilt; das fruehere Efb-Zertifikat ZZBT005000349003 der ETH Wertstoffrecycling GmbH, HRB 115679 Hamburg, nennt ebenfalls "Oberwerder Damm 1-5"), die Standortseite des Betreibers dagegen "Oberwerder Damm 3-5"; Hausnummer 3 ist laut eigenem Impressum von der ABN Baustoffueberwachung Nord mbH (Niederlassung Hamburg, HRB 11463 HL) belegt. E-Mail: das REMEX-Gruppenportal remex.de/profil/standorte verlinkt fuer die Betriebsstaetten der REMEX Nord das Postfach info-nord@remex.de; auf remex-nord.de ist die Adresse hinter einem mailto-Token verborgen, die Domain @remex.de ist im HTML sichtbar. ANKAUF: keine Ankaufspreise, das Angebot zielt auf Abfallentsorgung und mineralische Verwertung, nicht auf den Ankauf von Schrott durch Private. VERKAUF: keine Preisliste. GEBUEHREN: keine oeffentlichen Tarife. GEO: keine Koordinate gesetzt, weil fuer den Oberwerder Damm nur OSM-Adressknoten und keine zweite unabhaengige Punktquelle vorliegen (Node 3160813999 Oberwerder Damm 1 = 53.5276477/10.0296576, Node 3160814003 = 53.5275573/10.029583, Node 3160814009 Oberwerder Damm 5 = 53.5274801/10.0295111; Way 659212054 = Oberwerder Damm 3, building=office); 11880 fuehrt nur "REMEX Nord GmbH // Verwaltung" am Einsiedeldeich 15. Quelle(n): https://schrottregister.pages.dev/betrieb-remex-nord-gmbh-hamburg-2 ; https://schrottregister.pages.dev/betrieb-remex-nord-gmbh-hamburg ; https://schrottregister.pages.dev/ort-hamburg ; https://www.remex-nord.de/ ; https://www.remex.de/profil/standorte/ ; https://www.northdata.de/REMEX+Nord+GmbH,+Hamburg/HRB+133957 ; https://www.registercheck.de/companies/remex-nord-gmbh ; https://api.openstreetmap.org/api/0.6/map.json?bbox=10.0240,53.5230,10.0330,53.5300 ; https://abn-online.de/impressum ; https://www.eth-umweltservice.de/fileadmin/content/eth_wertstoffrecycling/ZZBT005000349003_EFB-Zertifikat.pdf (Abruf 28.10.2026).]
```

---

## 2. Fall 2 - Dublette: Traut Gelsenkirchen gegen Traut Essen

### 2.1 Befund

**Ein** mobiler Betrieb mit **zwei** Verzeichniseintraegen desselben Aggregators. Die
Gelsenkirchen-Zeile traegt die einzige vorhandene Anschrift, die Essen-Zeile traegt
**keine** Anschrift. Die Essen-Zeile ist die Dublette und bleibt als Querverweis
bestehen; geloescht wird nichts.

### 2.2 Belege

Beide Profile wurden am 28.10.2026 direkt gelesen:

| | `schrottfinder.de/schrotthandler-traut-gelsenkirchen-gelsenkirchen/` | `schrottfinder.de/schrotthandler-traut-essen/` |
|---|---|---|
| Titel | "Schrotthändler Traut - Gelsenkirchen" | "Schrotthändler Traut" |
| Telefon | 491629135397 | 491629135397 |
| Anschrift | "Gelsenkirchen, Kurt-Schumacher-Straße 80" | **leer** ("nimmt Schrott unter der Adresse  an") |
| Leistung | "Abtransport von Schrott" | "Abtransport von Schrott" |
| Metallarten | Kupfer, Aluminium, Edelstahl, Messing, Zink, Blei | Kupfer, Aluminium, Edelstahl, Messing, Zink, Blei |
| Kartenmarker | `L.marker([51.5217378, 7.090119])` | `L.marker([51.4409073, 7.0160182])` (92, Witteringstraße, Wohnobjekt) |

Die Gleichheit von Mobilnummer, Leistungsbeschreibung und Materialiste in zwei
Stadtvarianten desselben Portals ist der Beleg fuer **einen** Betrieb.

Gegenproben (alle negativ, alle ohne Beweiswert fuer eine Nichtexistenz):

- 11880 Rueckwaertssuche fuer `0162 9135397`: "Leider nichts gefunden".
- 11880-Suche "Traut" im Raum Schrotthaendler: keine Traut-Zeile (Treffer "Trautenstein",
  "Traut Fachhandel Rohr- u. Industriemontagen" in Oberaudorf).
- Gelbe Seiten-Suche "Traut" + Schrotthaendler: 0 Treffer.
- Das Oertliche, Schrotthaendler-Regionallisten Gelsenkirchen (Treffer 1-20) und Essen
  (Treffer 1-13): **kein** Traut-Eintrag.
- NorthData "Schrotthandel Traut" / "Traut Gelsenkirchen": kein passendes
  Registerunternehmen; Traut ist Einzelunternehmer ohne HRB, das ist ein Negativbefund
  ohne Beweiswert.
- Eine eigene Betreiberseite existiert nicht.

Die OSM-Geometrie stuetzt die Gelsenkirchen-Adresse: Way 263862026 traegt
`addr:housenumber 80`, `addr:street Kurt-Schumacher-Straße`, `addr:city Gelsenkirchen`,
`addr:postcode 45881`, `building=yes`; Nominatim loest ihn auf
"80, Kurt-Schumacher-Straße, Schalke, Gelsenkirchen-Mitte" auf 51.5217366/7.0902337.
Die Schrottfinder-Kartennadel liegt rund 9 m daneben. Die **Adresse selbst** ist aber
weiterhin nur ein Aggregator-Lead.

Die Produktionskoordinaten der beiden Zeilen sind nachweislich falsch:

- `nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger`: 51.539169/7.080031 loest auf
  **Alfred-Zingler-Straße 9**, 45881 Gelsenkirchen-Schalke-Nord, rund 2,06 km vom
  Lead-Adressepunkt entfernt.
- `nw-essen-schrotthandler-traut`: 51.450284375/7.015053125 liegt auf der Strasse
  "Freiheit" am **Fernbusbahnhof Essen-Hbf** (Busspur, keine Gewerbeanschrift).

### 2.3 Empfehlung an den Owner

- **Die Gelsenkirchen-Zeile fuehrt den Betrieb.** Sie hat die einzige vorhandene
  Anschrift; die Essen-Zeile ist die Dublette.
- **Die Essen-Zeile bleibt bestehen** (keine Loeschung, kein Merge) und wird als
  Querverweis auf die Gelsenkirchen-Zeile gefuehrt.
- **Anschrift und Koordinate:** Der Adress-Lead ist nur einfach belegt (ein Aggregator).
  Nach der Quellenhierarchie wird `street`/`postcode` deshalb **nicht** gefuellt; die
  Gelsenkirchen-Zeile bleibt bei `street: ''`, `postcode: ''`. Der heute gespeicherte,
  unbelegte Punkt wird mit `clear_fields: geo` zurueckgenommen, der belegte
  OSM-Hauspunkt plus Portalnadel (zwei Quellen, 9 m Abstand) steht als **Kandidat** in
  der Notiz, bis eine zweite Belegquelle fuer die Adresse vorliegt.
  *Owner-Option:* wenn der Owner den Aggregator-Lead als ausreichend akzeptiert, waere
  `street: 'Kurt-Schumacher-Straße 80'`, `postcode: '45881'`,
  `lat: 51.5217366`, `lon: 7.0902337` vertretbar (OSM-Hauspunkt, deckungsgleich mit der
  Portalnadel 51.5217378/7.090119).
- **Der Telefon-Eintrag wird bereinigt:** `0162 9135397)` (Import-Artefakt mit
  Klammer) -> `0162 9135397`. Die Essener Zeile erhaelt dieselbe Nummer (dort bisher
  leer), weil beide Profile des Aggregators dieselbe Mobilnummer nennen und der Seed
  der Essener Zeile die "0162-Nr." ohne Adresse ausweist.
- **`status` bleibt `pruefung`:** es gibt keine Betreiberquelle und kein Register.

### 2.4 Entwurf: Dossier-Aenderungen

**Datei: `dossiers/nw/nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger.md` (LEITZEILE)**

| Feld | jetzt | neu | Pflicht | Beleg |
|---|---|---|---|---|
| `phone` | `0162 9135397)` | `0162 9135397` | ja | Import-Artefakt; Portalprofil nennt 491629135397 |
| `clear_fields` | - | `geo` | ja | gespeicherter Punkt = Alfred-Zingler-Straße 9, 2,06 km daneben, ohne Beleg |
| `description` | leer | Kurztext mit Anschrift-Lead und Querverweis | ja | s. Entwurf |
| `street` / `postcode` | leer | **bleiben leer** (Owner-Option siehe 2.3) | | Adress-Lead ist nur einfach belegt |
| `name` | `Schrotthändler Traut (GE-Ableger)` | `Schrotthändler Traut (Gelsenkirchen)` | optional (Owner) | Ortsangabe der Sache nach |

```markdown
---
slug: nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger
name: Schrotthändler Traut (Gelsenkirchen)
trader_type: schrotthaendler
state: NW
city: Gelsenkirchen-Mitte
street: ''
postcode: ''
phone: 0162 9135397
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Mobiler Schrotthaendler "Schrotthändler Traut", gefuehrt im Schrottfinder-Portal unter der Anschrift Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen mit der Mobilnummer +49 162 9135397 und den angenommenen Metallarten Kupfer, Aluminium, Edelstahl, Messing, Zink und Blei sowie der Leistung "Abtransport von Schrott". Die Adresse ist ein Aggregator-Lead ohne zweite Belegquelle: 11880 fuehrt die Nummer nicht, Gelbe Seiten und Das Oertliche kennen keinen Traut-Eintrag in Gelsenkirchen oder Essen, und es existiert keine eigene Betreiberseite. Die Schwesterzeile nw-essen-schrotthandler-traut ist ein zweites Verzeichnisprofil desselben Aggregators fuer dieselbe Mobilnummer ohne Anschrift und wird als Dublette hierher verwiesen. ANKAUF: keine Traut zuordenbaren Kurse, die auf der Portalseite gezeigten Schrottmetallpreise sind ausdruecklich Portal-Durchschnitte und kein oeffentliches Angebot. VERKAUF: keine Liste. GEBUEHREN: keine Tarife.'
dropoff_json: ''
pickup_json: ''
clear_fields: geo
provenance_section: R4c Gelsenkirchen-Multi-sites (7 Profile aufgelöst)
provenance_ankauf_raw: mobil/unklar
provenance_origin: table
---

# Schrotthändler Traut (Gelsenkirchen)

## Überblick

Die Zeile fuehrt den mobilen Schrotthaendler Traut ueber das Schrottfinder-Portal mit
der Anschrift Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen. Sie ist damit die
**Leitzeile** des Clusters: das zweite Verzeichnisprofil desselben Aggregators
(`nw-essen-schrotthandler-traut`) nennt dieselbe Mobilnummer +49 162 9135397,
dieselbe Leistung ("Abtransport von Schrott") und dieselben Metallarten, laesst die
Anschrift aber leer und wird hierher verwiesen. Beide Zeilen bleiben bestehen, es wird
nichts geloescht und nichts zusammengefuehrt.

Die Anschrift ist ein Aggregator-Lead mit nur einer Quelle. OSM fuehrt zur Hausnummer 80
den Way 263862026 (addr:housenumber 80, addr:street Kurt-Schumacher-Straße,
addr:city Gelsenkirchen, addr:postcode 45881, building=yes; Nominatim 51.5217366 /
7.0902337), die Kartennadel des Portals liegt bei 51.5217378 / 7.090119, rund 9 m
daneben. Weil die Adresse selbst nur einfach belegt ist, bleiben `street` und
`postcode` leer. Der bis dahin gespeicherte, unbelegte Produktionspunkt
(51.539169/7.080031, Alfred-Zingler-Straße 9, rund 2,06 km entfernt) wird mit
`clear_fields: geo` zurueckgenommen.

**Ankauf:** keine Traut zuordenbaren Kurse. **Verkauf:** keine Liste.
**Gebuehren:** keine Tarife.

## Timeline

### Recherche 28.10.2026: Leitzeile des Traut-Clusters, Dublette in Essen

- [Recherche 28.10.2026: DUBLETTEN-/MERGE-ENTSCHEIDUNG: Die beiden Traut-Zeilen sind ein Betrieb mit zwei Verzeichniseintraegen desselben Aggregators, keine zwei Betriebe. Das Gelsenkirchen-Profil des Schrottfinder (am 28.10.2026 direkt gelesen) nennt "Schrotthandler Traut - Gelsenkirchen", die Adresse "Gelsenkirchen, Kurt-Schumacher-Straße 80", die Telefonnummer 491629135397, die Leistung "Abtransport von Schrott" und die Metallarten Kupfer, Aluminium, Edelstahl, Messing, Zink und Blei, mit einem Kartenmarker bei 51.5217378/7.090119; das Essen-Profil nennt dieselbe Nummer 491629135397, dieselbe Leistung, dieselben sechs Metallarten, laesst die Anschrift aber leer ("nimmt Schrott unter der Adresse  an") und setzt den Marker auf 51.4409073/7.0160182 (92, Witteringstraße, ein Wohnobjekt). Gleichheit von Nummer, Leistung und Materialiste in zwei Stadtvarianten desselben Portals belegt einen mobilen Betrieb; diese Zeile fuehrt ihn, weil nur sie eine Anschrift hat. Gegenproben, alle negativ und ohne Beweiswert fuer eine Nichtexistenz: 11880-Rueckwaertssuche fuer 0162 9135397 meldet "Leider nichts gefunden", die 11880-Suche nach Traut im Schrotthaendler-Bereich zeigt keinen Traut-Eintrag, die Gelbe-Seiten-Suche "Traut" + Schrotthaendler liefert 0 Treffer, und die Das-Oertliche-Regionallisten der Schrotthaendler fuer Gelsenkirchen (Treffer 1-20) und Essen (Treffer 1-13) enthalten keinen Traut; NorthData-Suchen "Schrotthandel Traut" und "Traut Gelsenkirchen" zeigen kein passendes Registerunternehmen, was fuer einen Einzelunternehmer ohne HRB ohne Beweiswert ist. Eine eigene Betreiberseite existiert nicht. Die Anschrift ist damit nur ein Aggregator-Lead; OSM fuehrt zur Hausnummer 80 den Way 263862026 (addr:housenumber 80, addr:street Kurt-Schumacher-Straße, addr:city Gelsenkirchen, addr:postcode 45881, building=yes; Nominatim 51.5217366/7.0902337), die Portalkartennadel liegt rund 9 m daneben; zwei unabhaengige Punktquellen, aber nur eine Quellquelle fuer die Adresse, deshalb bleiben street und postcode leer und der Kandidat ist nur dokumentiert. GEO: die read-only Produktionszeile hielt 51.539169/7.080031, was auf Alfred-Zingler-Straße 9, 45881 Gelsenkirchen-Schalke-Nord aufloest, rund 2,06 km vom Lead-Adressepunkt entfernt; dieser unbelegte Punkt wird mit `clear_fields: geo` zurueckgenommen und es wird kein Ersatzpunkt geraten. ANKAUF: keine Traut zuordenbaren Kurse, die auf der Portalseite gezeigten Schrottmetallpreise sind laut Seite "nur zur Orientierung" bzw. "kein oeffentliches Angebot" und damit ausdruecklich KEINE Traut-Kurse. VERKAUF: keine Liste. GEBUEHREN: keine Tarife. Querverweis: Dublette nw-essen-schrotthandler-traut. Quelle(n): https://schrottfinder.de/schrotthandler-traut-gelsenkirchen-gelsenkirchen/ ; https://schrottfinder.de/schrotthandler-traut-essen/ ; https://www.11880.com/rueckwaertssuche/0162+9135397 ; https://www.11880.com/suche/schrotthaendler/traut ; https://www.gelbeseiten.de/suche/traut/schrotthaendler ; https://www.dasoertliche.de/Themen/Schrotth%C3%A4ndler/Gelsenkirchen.html ; https://www.dasoertliche.de/Themen/Schrotth%C3%A4ndler/Essen.html ; https://www.openstreetmap.org/way/263862026 ; https://nominatim.openstreetmap.org/search?q=Kurt-Schumacher-Stra%C3%9Fe+80%2C+45881+Gelsenkirchen&format=jsonv2&addressdetails=1 ; read-only `public.db` (28.10.2026).]
```

**Datei: `dossiers/nw/nw-essen-schrotthandler-traut.md` (DUBLETTE)**

| Feld | jetzt | neu | Pflicht | Beleg |
|---|---|---|---|---|
| `phone` | leer | `0162 9135397` | ja | beide Portalprofile nennen 491629135397; Seed wies die "0162-Nr." aus |
| `clear_fields` | - | `geo` | ja | Punkt = Fernbusbahnhof Essen-Hbf, Busspur, keine Gewerbeanschrift |
| `description` | leer | Dublettentext mit Querverweis auf die Gelsenkirchen-Zeile | ja | s. Entwurf |
| `city` | `Essen` | `Essen` (unveraendert) | | die Zeile bleibt als Essener Verzeichniseintrag sichtbar |
| `street` / `postcode` | leer | **bleiben leer** | | keine eigene Anschrift belegt |

```markdown
---
slug: nw-essen-schrotthandler-traut
name: Schrotthändler Traut (Essen)
trader_type: schrotthaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: 0162 9135397
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: 'Zweiter Verzeichniseintrag desselben mobilen Schrotthaendlers Traut fuer die Stadt Essen: Das Schrottfinder-Profil nennt dieselbe Mobilnummer +49 162 9135397, dieselbe Leistung "Abtransport von Schrott" und dieselben Metallarten (Kupfer, Aluminium, Edelstahl, Messing, Zink, Blei), laesst die Anschrift aber leer und setzt den Kartenmarker auf 92, Witteringstraße, 45130 Essen (Wohnobjekt). Es gibt keine belegte eigene Betriebsanschrift in Essen. Die Zeile ist deshalb die Dublette zur Leitzeile nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger (Anschrift Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen) und bleibt als Querverweis bestehen; nichts wird geloescht oder zusammengefuehrt. ANKAUF: keine Traut zuordenbaren Kurse. VERKAUF: keine Liste. GEBUEHREN: keine Tarife.'
dropoff_json: ''
pickup_json: ''
clear_fields: geo
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: mobil/unklar
provenance_origin: table
---

# Schrotthändler Traut (Essen)

## Überblick

Dublette. Das Essen-Profil des Schrottfinder ist eine Stadtvariante desselben
Portal-Eintrags wie die Gelsenkirchen-Zeile: gleiche Mobilnummer +49 162 9135397, gleiche
Leistung, gleiche Metallarten, aber **keine Anschrift** und ein Marker auf einem
Wohnobjekt in der Witteringstraße. Es gibt keine belegte eigene Betriebsanschrift in
Essen.

Die Zeile bleibt als sichtbarer Verzeichniseintrag bestehen (keine Loeschung, kein
Merge) und verweist auf die Leitzeile
`nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger`. Der Produktionspunkt
51.450284375/7.015053125 lag auf der Busspur des Essener Fernbusbahnhofs und wird mit
`clear_fields: geo` zurueckgenommen; ein Ersatzpunkt wird nicht geraten.

**Ankauf:** keine Traut zuordenbaren Kurse. **Verkauf:** keine Liste.
**Gebuehren:** keine Tarife.

## Timeline

### Recherche 28.10.2026: Dublette zur Gelsenkirchen-Zeile

- [Recherche 28.10.2026: DUBLETTE bestaetigt, KEIN Merge und keine Loeschung. Das am 28.10.2026 direkt gelesene Schrottfinder-Profil "Schrotthaendler Traut" (Essen) nennt dieselbe Mobilnummer 491629135397 (= +49 162 9135397) wie das Gelsenkirchen-Profil, dieselbe Leistung "Abtransport von Schrott" und dieselben sechs Metallarten; die Anschrift ist dort leer ("nimmt Schrott unter der Adresse  an"), der Kartenmarker liegt bei 51.4409073/7.0160182 auf 92, Witteringstraße, 45130 Essen, einem Wohnobjekt. Damit existiert fuer diese Zeile keine eigene belegte Betriebsanschrift in Essen, und die Zeile ist die Stadtvariante desselben mobilen Betriebs. Die Leitzeile ist nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger mit der Anschrift Kurt-Schumacher-Straße 80, 45881 Gelsenkirchen; die Gelsenkirchen-Adresse wird bewusst NICHT in diese Zeile uebernommen, weil sie zum selben Betrieb gehoert und nicht zu einer Essener Zweigstelle. Das Telefonfeld wird mit der in beiden Profilen und im Seed genannten Nummer gefuellt und das Import-Artefakt in der Schwestzeile (trailing Klammer) bereinigt. GEO: die read-only Produktionszeile enthielt 51.450284375/7.015053125, was laut OSM auf der Strasse "Freiheit", 45128 Essen, auf dem Fernbusbahnhof am Essener Hauptbahnhof liegt (way 46961889, highway=service, bus=designated) - eine Busspur ohne Gewerbeanschrift; dieser Platzhalter wird mit `clear_fields: geo` zurueckgenommen, ein Ersatzpunkt wird nicht geraten und keine Adressangabe aus der Gelsenkirchen-Zeile uebernommen. ANKAUF: keine Traut zuordenbaren Kurse; die auf der Portalseite gezeigten Preise sind Portal-Durchschnitte und ausdruecklich kein oeffentliches Angebot. VERKAUF: keine Liste. GEBUEHREN: keine Tarife. Quelle(n): https://schrottfinder.de/schrotthandler-traut-essen/ ; https://schrottfinder.de/schrotthandler-traut-gelsenkirchen-gelsenkirchen/ ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; read-only `public.db` (28.10.2026).]
```

---

## 3. Fall 3 - Cluster Schaafheim (Reining Klaus / Containerdienst Schaafheim)

### 3.1 Befund

Die drei Slugs sind **ein** Betrieb: der 2020 von Salih Gencsoy uebernommene
Containerdienst Schaafheim, Ringstraße 7, 64850 Schaafheim. Zwei der drei Zeilen tragen
den historischen Klaus-Reining-Bestand an der Karlstraße 1 und sind untereinander
Dublette; eine Zeile fuehrt den heutigen Betrieb.

### 3.2 Belege (alle am 28.10.2026 direkt gelesen)

Die Seite `containerdienst-schaafheim.de` ist eine JavaScript-Anwendung; die
ausgelieferten Module wurden einzeln gelesen.

**`assets/Seo-DTvtlKDA.js`, Konfigurationsblock:**

```
name: `Containerdienst Schaafheim`
legalForm: ``
contactPerson: `Salih Gencsoy`
address: {street: `Ringstraße 7`, zip: `64850`, city: `Schaafheim`, country: `Deutschland`}
phoneMobile: `0170 8648439`  phoneMobileTel: `+491708648439`
phoneLandline: `06073 7447301` phoneLandlineTel: `+4960737447301`
email: `info@containerdienst-schaafheim.de`
hours: [{day: `Montag – Freitag`, time: `08:00 – 18:00 Uhr`},
        {day: `Samstag`, time: `geschlossen`}, {day: `Sonntag`, time: `geschlossen`}]
operatingSince: `Mai 2020`
historyPredecessor: ``            <- leer
serviceRadiusKm: 50
serviceAreas: [{name: `Schaafheim`, zip: `64850`, confirmed: true}]
```

**`assets/UeberUns-CamdrXxU.js`:** "Der Containerdienst in Schaafheim wird **laut
bisheriger Website** seit Mai 2020 von Salih Gencsoy betrieben." Dazu ein Kasten:
"Hinweis (vor Launch): Aussagen zur Firmengeschichte und einem moeglichen
Vorgängerbetrieb werden erst nach Freigabe durch den Kunden veroeffentlicht."

**`assets/Impressum-DA8ZMBv_.js`:** "Angaben gemäß § 5 DDG" mit Name, Anschrift,
Vertretung durch Salih Gencsoy, aber **`Rechtsform: [vom Kunden ergänzen]`** und
**`Umsatzsteuer-Identifikationsnummer gemäß § 27a Umsatzsteuergesetz: [vom Kunden
ergänzen]`**. Es gibt also keine Registernummer, und die README-Ausnahme fuer eine
allein tragende Betreiberquelle greift formal nicht.

**`assets/Kontakt-u3FhrzKh.js`:** JSON-LD mit `openingHoursSpecification` Montag bis
Freitag 08:00 bis 18:00.

**Zweite Quelle (Verzeichnis).** 11880 fuehrt "Containerdienst Schaafheim Salih Gencsoy",
Ringstr. 7, 64850 Schaafheim (Radheim), (06073) 7447301, E-Mail
`info@containerdienst-schaafheim.de`, Geo-Koordinate **49.887766 / 9.019157**,
Oeffnungszeiten Mo bis Fr 08:00-17:00, JSON-LD-Status "vom Inhaber bestaetigt".
Gelbe Seiten fuehrt "Gencsoy Salih Containerdienste, Entsorgung", Ringstr. 7, 64850
Schaafheim (Radheim), 06073 7 44 73 01, "Geoeffnet, schliesst um 18:00", Website.

**Dritte Quelle fuer die Geometrie.** OSM-Way 961378576 mit `addr:housenumber 7`,
`addr:street Ringstraße`, `addr:postcode 64850`, `addr:city Schaafheim`,
`building=yes`, Zentrum **49.8877659 / 9.0191569**. Das deckt sich mit der
11880-Kartennadel auf rund 1 m.

**Was der Auftritt *nicht* mehr hergibt.** `historyPredecessor` ist leer, die
Uebernahmeaussage ist auf "laut bisheriger Website" abgeschwaecht, und die frueher
im Suchindex sichtbare Formulierung "Vor mehr als 38 Jahren gruendet Klaus Reining den
Containerdienst Reining, welcher nun unter neuem Namen und neuem Inhaber fuer Sie da
ist" ist im ausgelieferten Modul **nicht mehr nachweisbar**. Die Altdomain
containerdienst-reining.com ist nicht im DNS; der Web-Archiv-Snapshot vom 02.12.2021
zeigt nur eine Joomla-Fehlerseite. Klaus Reining ist in keinem Register erfasst. Das
amtliche Fachbetrieberegister fuehrt fuer Schaafheim (64850) genau einen Eintrag,
Gerhard Hoefling GmbH, Eichenweg 101 (Sammeln, Befoerdern, Handeln, Makeln) - weder
Reining noch Gencsoy; das ist ein Negativbefund ohne Beweiswert und zugleich eine
Namensvetter-Abgrenzung.

**Falscher Punkt.** Alle drei Zeilen tragen in der Produktion 49.9242/9.0094. Das loest
auf **Im Herrngarten 6a**, 64850 Schaafheim, auf - weder die Karlstraße 1 (OSM-Way
799842912, 49.9241338/9.0063758) noch die Ringstraße 7.

### 3.3 Empfehlung an den Owner

- **Leitzeile: `he-schaafheim-64850-he-reining-klaus-containerdienst`.** Diese Zeile
  traegt heute den Betrieb Containerdienst Schaafheim (Salih Gencsoy). Sie ist der
  einzige der drei Slugs, dessen Adressfelder bereits am 17.10.2026 mit
  Owner-Freigabe bereinigt wurden (city normalisiert, street/postcode leer), und ihr
  Namensfragment endet auf "containerdienst", was der heutigen Taetigkeit entspricht.
  *Alternative:* ein **neues** Dossier `he-schaafheim-containerdienst-schaafheim`, wie
  im 60:00-Wellenbericht empfohlen; das haelt die beiden Reining-Zeilen semantisch
  sauber, erzeugt aber eine vierte Zeile im Cluster. Beide Varianten loesen den
  Fall ohne Loeschung; die Empfehlung des Verfassers ist die erstgenannte, weil sie
  ohne neue Zeile auskommt.
- **Die beiden anderen Zeilen bleiben als historische Klaus-Reining-Datensaetze
  bestehen** und verweisen quer auf die Leitzeile. Sie sind untereinander Dublette.
- **Anschrift, Telefon, E-Mail, Zeiten und Website werden gefuellt** (Betreiber +
  11880 + Gelbe Seiten + OSM).
- **Abweichung bei den Oeffnungszeiten:** Betreiber und Gelbe Seiten nennen
  08:00-18:00, 11880 nennt 08:00-17:00. Es gilt die Betreiberangabe 08:00-18:00; die
  Abweichung wird in der Notiz festgehalten.
- **`status` bleibt `pruefung`:** das Impressum fuehrt Rechtsform und USt-IdNr. als
  Platzhalter, der Rechtstraeger ist nicht im Register; die Kriterien der
  Betreiber-Primärquelle-Ausnahme sind damit nicht erfuellt. *Owner-Option:*
  `status: aktiv`, wenn die Existenz eines laufenden Betriebs genuegt.
- **GEO:** Die Leitzeile bekommt `lat: 49.8877659`, `lon: 9.0191569` (OSM-Hauspunkt,
  deckungsgleich mit der 11880-Kartennadel, zwei unabhaengige Quellen). Die beiden
  historischen Zeilen bekommen `clear_fields: geo`, weil ihr gemeinsamer Punkt
  (Im Herrngarten 6a) weder Karlstraße 1 noch Ringstraße 7 ist.

### 3.4 Zuordnung

| Slug | traegt heute | Anschrift | Kontakte | Status |
|---|---|---|---|---|
| `he-schaafheim-64850-he-reining-klaus-containerdienst` | **Containerdienst Schaafheim (Salih Gencsoy)** | Ringstraße 7, 64850 Schaafheim | 0170 8648439 / 06073 7447301, info@containerdienst-schaafheim.de, Mo-Fr 08:00-18:00, containerdienst-schaafheim.de | pruefung |
| `he-schaafheim-64850-he-reining-klaus` | historischer Klaus-Reining-Bestand | Karlstr. 1, 64850 Schaafheim (Verzeichnis-Lead) | keine | pruefung |
| `he-schaafheim-64850-reining-klaus-containerdienst` | historischer Klaus-Reining-Bestand | Karlstr. 1, 64850 Schaafheim (Verzeichnis-Lead) | 06073 9009 (Verzeichnis-Lead) | pruefung |

### 3.5 Entwurf: Dossier-Aenderungen

**Leitzeile `dossiers/he/he-schaafheim-64850-he-reining-klaus-containerdienst.md`**

| Feld | jetzt | neu | Beleg |
|---|---|---|---|
| `name` | `Reining Klaus Containerdienst` | `Containerdienst Schaafheim` | Betreiber-Konfig (`name: Containerdienst Schaafheim`), 11880 |
| `street` | leer | `Ringstraße 7` | Betreiber + 11880 + Gelbe Seiten + OSM |
| `postcode` | leer | `64850` | dito |
| `phone` | leer | `0170 8648439 / 06073 7447301` | Betreiber (beide), 11880 und Gelbe Seiten bestaetigen das Festnetz |
| `email` | leer | `info@containerdienst-schaafheim.de` | Betreiber + 11880-JSON-LD |
| `opening_hours` | leer | `Mo-Fr 08:00-18:00` | Betreiber (Kontakt-Modul + JSON-LD) + Gelbe Seiten; 11880 nennt 08:00-17:00 (abweichend) |
| `website` | leer | `https://www.containerdienst-schaafheim.de/` | Betreiber |
| `website_status` | leer | `aktiv` | Abruf HTTP 200 am 28.10.2026 |
| `lat` / `lon` | (49.9242/9.0094 in Produktion) | `49.8877659` / `9.0191569` | OSM-Way 961378576 + 11880-Kartennadel 49.887766/9.019157 |
| `description` | leer | Kurztext | s. Entwurf |
| `status` | `pruefung` | `pruefung` (unveraendert) | Impressum ohne Registernummer |

```markdown
---
slug: he-schaafheim-64850-he-reining-klaus-containerdienst
name: Containerdienst Schaafheim
trader_type: containerdienst
state: HE
city: Schaafheim
street: Ringstraße 7
postcode: '64850'
lat: 49.8877659
lon: 9.0191569
phone: 0170 8648439 / 06073 7447301
email: info@containerdienst-schaafheim.de
opening_hours: Mo-Fr 08:00-18:00
website: https://www.containerdienst-schaafheim.de/
website_status: aktiv
status: pruefung
description: 'Containerdienst Schaafheim, gefuehrt von Salih Gencsoy, am Standort Ringstraße 7, 64850 Schaafheim (Stadtteil Radheim): regionaler Container-, Schuettgut- und Maschinenservice fuer Privat- und Gewerbekunden aus Bauherren, Handwerk, Gewerbe und Landwirtschaft, laut Betreiberangabe seit Mai 2020 in Betrieb. Leistungen laut Betreiberauftritt: Containerdienst, Schuettgueter, Baumaschinenvermietung, Abfallarten, Entruempelung, Abriss und Demontage; eine Abholung von Schrott und Metall wird beworben, aber ohne Zahlenpreise. ANKAUF: keine Ankaufspreise, kein Materialstaffel, kein Tageskurs. VERKAUF: keine Preisliste. GEBUEHREN: keine Container- oder Entsorgungstarife veroeffentlicht, Angebote werden individuell angefragt. Das Impressum der Betreiberseite fuehrt Rechtsform und Umsatzsteuer-Identifikationsnummer noch als Platzhalter ("vom Kunden ergaenzen"), ein Registereintrag der Gesellschaft liegt nicht vor; der Nachfolgerauftritt nennt die Uebernahme des Containerdienstes Reining (Klaus Reining) selbst nur abgeschwaecht ("laut bisheriger Website"), weshalb die Nachfolgekette nicht als Registertatsache behandelt wird. Die beiden Schwesterzeilen he-schaafheim-64850-he-reining-klaus und he-schaafheim-64850-reining-klaus-containerdienst fuehren den historischen Klaus-Reining-Bestand an der Karlstraße 1 und verweisen hierher; die Anschrift Ringstraße 7 wurde nicht auf sie uebertragen.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Mömlingen-70km (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Containerdienst Schaafheim

## Überblick

Die Zeile fuehrt den heutigen Betrieb **Containerdienst Schaafheim** (Salih Gencsoy) an
der Ringstraße 7, 64850 Schaafheim (Stadtteil Radheim). Der Betreiberauftritt liefert
Anschrift, beide Rufnummern, die E-Mail, die Oeffnungszeiten und das Leistungsspektrum
(Container-, Schuettgut- und Maschinenservice, Abfallarten, Abriss, Demontage,
Entruempelung, Baumaschinenvermietung) sowie den Betriebsbeginn "Mai 2020" und einen
Einzugsradius von 50 km um Schaafheim. Das Impressum fuehrt Rechtsform und
Umsatzsteuer-Identifikationsnummer noch als Platzhalter, ein Registereintrag liegt nicht
vor; `status` bleibt deshalb `pruefung`.

**Ankauf:** eine Abholung von Schrott und Metall wird beworben, aber **ohne
Zahlenpreise**, ohne Materialstaffel und ohne Tageskurs. **Verkauf:** keine
Preisliste. **Gebuehren:** keine Container- oder Entsorgungstarife, Angebote werden
individuell angefragt.

**GEO:** 49.8877659 / 9.0191569 ist der OSM-Hauspunkt (Way 961378576,
addr:housenumber 7, addr:street Ringstraße, addr:postcode 64850, addr:city Schaafheim,
building=yes) und deckt sich auf rund 1 m mit der Kartennadel des 11880-Eintrags
(49.887766 / 9.019157). Es ist ein Adresspunkt, kein belegter Eingangspunkt; der
vorher gespeicherte Punkt (Im Herrngarten 6a) wird dadurch ersetzt.

**Cluster:** Die beiden Schwesterzeilen `he-schaafheim-64850-he-reining-klaus` und
`he-schaafheim-64850-reining-klaus-containerdienst` fuehren den historischen
Klaus-Reining-Bestand an der Karlstraße 1 und verweisen hierher. Sie werden nicht
geloescht und nicht zusammengefuehrt.

## Timeline

### Recherche 28.10.2026: heutiger Betrieb, Cluster aufgeloest

- [Recherche 28.10.2026: CLUSTER AUFGELOEST, die Zeile fuehrt ab jetzt den heutigen Betrieb. Der Nachfolgerauftritt containerdienst-schaafheim.de ist eine JavaScript-Anwendung; die ausgelieferten Module wurden am 28.10.2026 einzeln gelesen. Konfigurationsmodul Seo-DTvtlKDA.js: name "Containerdienst Schaafheim", contactPerson "Salih Gencsoy", address street "Ringstraße 7", zip "64850", city "Schaafheim", phoneMobile "0170 8648439", phoneLandline "06073 7447301", email "info@containerdienst-schaafheim.de", hours Montag bis Freitag "08:00 - 18:00 Uhr" sowie Samstag und Sonntag geschlossen, operatingSince "Mai 2020", historyPredecessor leer, serviceRadiusKm 50, serviceAreas Schaafheim 64850. Kontaktmodul Kontakt-u3FhrzKh.js mit JSON-LD openingHoursSpecification Montag bis Freitag 08:00 bis 18:00. Ueberuns-Modul UeberUns-CamdrXxU.js: "Der Containerdienst in Schaafheim wird laut bisheriger Website seit Mai 2020 von Salih Gencsoy betrieben" mit dem Kasten "Hinweis (vor Launch): Aussagen zur Firmengeschichte und einem moeglichen Vorgaengerbetrieb werden erst nach Freigabe durch den Kunden veroeffentlicht". Impressumsmodul Impressum-DA8ZMBv_.js: Rechtsform und Umsatzsteuer-Identifikationsnummer stehen noch als "[vom Kunden ergaenzen]", es gibt also keine Registernummer, und die README-Ausnahme fuer eine allein tragende Betreiberquelle greift formal nicht; der status bleibt daher pruefung. Unabhaengige zweite Quelle: der 11880-Eintrag "Containerdienst Schaafheim Salih Gencsoy" nennt Ringstr. 7, 64850 Schaafheim (Radheim), (06073) 7447301, info@containerdienst-schaafheim.de, die Geo-Koordinate 49.887766/9.019157, die Oeffnungszeiten Montag bis Freitag 08:00-17:00 und das Label "vom Inhaber bestaetigt"; die Gelbe Seiten fuehrt "Gencsoy Salih Containerdienste, Entsorgung" mit derselben Anschrift, 06073 7 44 73 01 und "schliesst um 18:00". Die Abweichung bei den Oeffnungszeiten (11880: 17:00, Betreiber und Gelbe Seiten: 18:00) ist dokumentiert; es gilt die Betreiberangabe Mo-Fr 08:00-18:00. GEO: OSM-Way 961378576 (addr:housenumber 7, addr:street Ringstraße, addr:postcode 64850, addr:city Schaafheim, building=yes) bei 49.8877659/9.0191569 deckt sich auf rund 1 m mit der 11880-Kartennadel; damit liegen zwei unabhaengige Quellen fuer den Punkt vor, und der bisher gespeicherte Punkt 49.9242/9.0094 (Im Herrngarten 6a, rund 4,4 km daneben und weder Karlstraße 1 noch Ringstraße 7) wird ersetzt. NACHFOLGEKETTE, bewusst nicht als Registertatsache behandelt: historyPredecessor ist im ausgelieferten Modul leer, die Uebernahmeaussage ist auf "laut bisheriger Website" abgeschwaecht, die frueher nur im Suchindex sichtbare Formulierung "Vor mehr als 38 Jahren gruendet Klaus Reining den Containerdienst Reining, welcher nun unter neuem Namen und neuem Inhaber fuer Sie da ist" ist heute im Modul nicht mehr nachweisbar, containerdienst-reining.com ist nicht im DNS und der Archiv-Snapshot vom 02.12.2021 zeigt nur eine Joomla-Fehlerseite; Klaus Reining ist in keinem Register erfasst. NEGATIVBEFUND: Das amtliche Fachbetrieberegister (Auszug 11.08.2026) fuehrt fuer Schaafheim (64850) genau einen Eintrag, "Unternehmenssitz" Eichenweg 101 mit den Taetigkeiten Sammeln, Befoerdern, Handeln und Makeln und Hauptsitz Gerhard Hoefling GmbH (HRB 92026 Darmstadt); weder Reining noch Gencsoy sind erfasst, das ist ein Negativbefund ohne Beweiswert und zugleich eine Namensvetter-Abgrenzung. ANKAUF: der Nachfolger bewirbt die Abholung von Schrott und Metall, aber ohne Zahlenpreise, Materialstaffel oder Tageskurs. VERKAUF: keine Liste. GEBUEHREN: keine Container-, Schuettgut- oder Entsorgungstarife, Angebote werden individuell angefragt. QUERVERWEIS: Die beiden Schwesterzeilen he-schaafheim-64850-he-reining-klaus und he-schaafheim-64850-reining-klaus-containerdienst fuehren den historischen Klaus-Reining-Bestand an der Karlstraße 1 und sind untereinander Dublette; die Ringstraße 7 wurde nicht auf sie uebertragen. Quelle(n): https://www.containerdienst-schaafheim.de/assets/Seo-DTvtlKDA.js ; https://www.containerdienst-schaafheim.de/assets/Kontakt-u3FhrzKh.js ; https://www.containerdienst-schaafheim.de/assets/UeberUns-CamdrXxU.js ; https://www.containerdienst-schaafheim.de/assets/Impressum-DA8ZMBv_.js ; https://www.11880.com/branchenbuch/schaafheim/060371058B112441803/containerdienst-schaafheim-salih-gencsoy.html ; https://www.gelbeseiten.de/suche/gencsoy/schaafheim ; https://www.openstreetmap.org/way/961378576 ; https://nominatim.openstreetmap.org/search?q=Ringstra%C3%9Fe+7%2C+64850+Schaafheim&format=jsonv2&limit=5&addressdetails=1 ; https://schrottregister.pages.dev/ort-schaafheim ; https://www.openstreetmap.org/way/799842912 (Karlstraße 1) ; read-only `public.db` (28.10.2026).]
```

**Historische Zeile 1: `dossiers/he/he-schaafheim-64850-he-reining-klaus.md`**

| Feld | jetzt | neu | Beleg |
|---|---|---|---|
| `city` | `Schaafheim 64850 HE` | `Schaafheim` | Normalisierung (PLZ steht im Feld `postcode`) |
| `street` | `Schaafheim , Karlstr. 1` | `Karlstr. 1` | Parse-Artefakt bereinigt, Verzeichnis-Lead bleibt |
| `clear_fields` | - | `geo` | gespeicherter Punkt = Im Herrngarten 6a, weder Karlstr. 1 noch Ringstr. 7 |
| `description` | leer | Querverweistext | s. Entwurf |
| `name` | `Reining Klaus` | unveraendert (Owner-Option: `Reining Klaus (Altbetrieb)`) | |

**Historische Zeile 2: `dossiers/he/he-schaafheim-64850-reining-klaus-containerdienst.md`**

| Feld | jetzt | neu | Beleg |
|---|---|---|---|
| `city` | `Schaafheim 64850` | `Schaafheim` | Normalisierung |
| `clear_fields` | - | `geo` | siehe oben |
| `description` | leer | Querverweistext | s. Entwurf |
| `street`, `postcode`, `phone` | `Karlstr. 1`, `64850`, `06073 9009` | unveraendert | Verzeichnis-Leads, historisch |
| `name` | `Reining Klaus (+ Containerdienst)` | unveraendert (Owner-Option: `Reining Klaus (+ Containerdienst) (Altbetrieb)`) | |

Die neuen Notizen fuer beide Zeilen (jeweils **eine** neue Notiz, bestehende Notizen
bleiben unveraendert):

```markdown
### Recherche 28.10.2026: historischer Klaus-Reining-Bestand, Querverweis auf die Leitzeile

- [Recherche 28.10.2026: DUBLETTEN- UND NACHFOLGEENTSCHEIDUNG fuer das Reining-Cluster in Schaafheim. Der heute dort betriebene Containerdienst Schaafheim (Salih Gencsoy) fuehrt die Zeile he-schaafheim-64850-he-reining-klaus-containerdienst (Leitzeile): Ringstraße 7, 64850 Schaafheim, 0170 8648439 und 06073 7447301, info@containerdienst-schaafheim.de, Mo-Fr 08:00-18:00, containerdienst-schaafheim.de, belegt durch den am 28.10.2026 vollstaendig gelesenen Betreiberauftritt und den 11880-Eintrag "Containerdienst Schaafheim Salih Gencsoy". Diese Zeile dokumentiert dagegen den historischen Klaus-Reining-Bestand an der Karlstraße 1, 64850 Schaafheim, und bleibt als eigener Datensatz bestehen; die Anschrift Ringstraße 7 wird NICHT auf sie uebertragen und die beiden Reining-Zeilen werden untereinander nicht zusammengefuehrt. Der Nachfolgerauftritt nennt die Uebernahme selbst nur abgeschwaecht ("laut bisheriger Website"), die frueher im Suchindex sichtbare Formulierung zum 38-jaehrigen Bestehen des Containerdienstes Reining ist heute im ausgelieferten Modul nicht mehr nachweisbar, containerdienst-reining.com ist nicht im DNS und Klaus Reining ist in keinem Register erfasst; die Existenz der Klaus-Reining-Entitaet nach dem Wechsel 2020 bleibt deshalb unbewiesen, ein Loesch- oder Schliessungsbeleg fehlt und der status bleibt pruefung. GEO: der in der Produktion gespeicherte Punkt 49.9242/9.0094 loest auf "Im Herrngarten 6a", 64850 Schaafheim, auf und weder auf die Karlstraße 1 (OSM-Way 799842912, 49.9241338/9.0063758) noch auf die Ringstraße 7; er wird mit `clear_fields: geo` zurueckgenommen und kein Ersatzpunkt geraten. NORMALISIERUNG: Das Adressfeld "Schaafheim , Karlstr. 1" ist ein Parse-Artefakt und wird auf "Karlstr. 1" bereinigt, der Ortswert "Schaafheim 64850 HE" auf "Schaafheim" (die PLZ steht bereits im Feld postcode). ANKAUF: fuer den Altbetrieb keine bestaetigten Schrott-Ankaufskurse, keine Verkaufspreise und keine Container- oder Entsorgungstarife. Quelle(n): https://www.containerdienst-schaafheim.de/assets/Seo-DTvtlKDA.js ; https://www.containerdienst-schaafheim.de/assets/UeberUns-CamdrXxU.js ; https://www.containerdienst-schaafheim.de/assets/Impressum-DA8ZMBv_.js ; https://www.11880.com/branchenbuch/schaafheim/060371058B112441803/containerdienst-schaafheim-salih-gencsoy.html ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.9242&lon=9.0094&zoom=18&addressdetails=1 ; https://www.openstreetmap.org/way/799842912 ; https://schrottregister.pages.dev/ort-schaafheim ; read-only `public.db` (28.10.2026).]
```

---

## 4. Fall 4 - Namenskorrekturen: sechs Eintraege

### 4.1 Uebersicht

| Slug | `name` jetzt | Vorschlag Owner | Befund | Empfehlung |
|---|---|---|---|---|
| `by-rotal-schubert` | `Schubert` | „Schrott & Recycling Ingo Schubert" | **bestaetigt** (zwei unabhaengige Verzeichnisse, keine Registerfirma) | uebernehmen |
| `by-alzenau-jr-recycling` | `JR-Recycling` | „AC Containerdienst UG (haftungsbeschränkt)" | **bestaetigt** (Betreiber-Impressum + Handelsregister) | uebernehmen |
| `rp-lustadt-lenhardt-lustadt` | `Lenhardt Lustadt` | „Lenhardt Heiko Rohstoffentsorgung" | **bestaetigt** (zwei unabhaengige Verzeichnisse, Einzelunternehmer) | uebernehmen |
| `by-miltenberg-ballweg-schrott` | `Ballweg Schrott` | „Schrotthandel Ballweg" | **bestaetigt** (zwei unabhaengige Verzeichnisse) | uebernehmen |
| `mv-neddemin-schrott-bobzien-zweitstandort` | `Schrott Bobzien (Zweitstandort?)` | „(Zweitstandort?)" entfernen | **bestaetigt** (Zweitstandort-Frage geklaert: ein Betrieb, ein Standort) | `Schrott Bobzien` |
| `rp-koblenz-d-laroche-horchheim` | `D. Laroche Horchheim` | „D. Laroche Recycling" | **korrigieren** (Schreibweise des Inhabers, Markenform des Betreibers) | `D. Laroche Schrotthandel` |

### 4.2 Belege je Eintrag

**(1) `by-rotal-schubert` -> „Schrott & Recycling Ingo Schubert"**

- 11880, Eintragsdaten 06.08.2024: **"Schrott & Recycling Ingo Schubert"**, Gewerbestr. 8,
  90574 Roßtal, (0160) 92926505, Branche "Altmetallrecycling", ohne Website, E-Mail und
  Öffnungszeiten.
- Das Örtliche: **"Schubert Ingo Schrott"**, Gewerbestr. 8, 90574 Roßtal,
  0160 92 92 65 05, Branche "Recycling in Roßtal", Kartennadel
  49.39149397157875 / 10.895781865630104 (identisch mit dem OSM-Hauspunkt
  Way 122869014 auf Gewerbestraße 8).
- Keine Betreiberseite, kein Registereintrag (Einzelunternehmer). Die Firmierung ist
  damit eine **Verzeichnisform**, keine amtliche Handelsregister-Firmierung.
- **Abgrenzung (bleibt bestehen):** Die Otto Schubert GmbH (HRB 3993 Fürth,
  Gehaeuse-/Mechanikhersteller) sitzt an derselben Gewerbestraße 8 und ist ein
  eigenstaendiger Betrieb; ihre Herstellerangaben werden nicht uebertragen.
- *Bestaetigt.* Die Form „Schrott & Recycling Ingo Schubert" ist die ausfuehrlichere der
  beiden Verzeichnisformen und traegt die Taetigkeit im Namen.

**(2) `by-alzenau-jr-recycling` -> „AC Containerdienst UG (haftungsbeschränkt)"**

- Betreiber-Impressum (live am 28.10.2026): **"AC Containerdienst UG (haftungsbeschränkt)"**,
  Im Schwalbengrund 14, 63579 Freigericht, vertreten durch die Geschaeftsfuehrerin
  Merle Federau, Registergericht Amtsgericht Hanau, Registernummer **HRB 101188**,
  Telefon 01577-3719857, E-Mail info@ac-containerdienst.de, Fusszeile
  "AC Containerdienst (c)2026".
- NorthData bestaetigt den heutigen Rechtstraeger: AC Containerdienst UG, AG Hanau
  HRB 101188, EUID DEM1502.HRB101188, Im Schwalbengrund 14, 63579 Freigericht,
  Gegenstand "Der Containerdienst, die Abfallentsorgung sowie der Handel mit Schrott
  und Metall", Publikation 13.05.2026.
- Die Wayback-Kette belegt den Namenswechsel: Impressum vom 13.05.2025 nennt noch
  "Juliano Jährig, Raiffeisenstr. 26" mit der Fusszeile "AC Containerdienst (c)2025",
  der Snapshot vom 14.07.2026 erstmals "AC Containerdienst UG (haftungsbeschränkt)".
- **Zu beachten:** der Seitentitel lautet "Impressum | AC Containerdienst
  **Gelnhausen**" und das Navigationsmenue fuehrt "Containerdienst Gelnhausen". Das ist
  ein Leistungsgebiets-/Menue-Etikett, **kein** Sitz der Firma (Sitz ist Freigericht,
  Registergericht Hanau). Die Stadt bleibt `Freigericht`.
- *Bestaetigt, mit Registerbeleg.*

**(3) `rp-lustadt-lenhardt-lustadt` -> „Lenhardt Heiko Rohstoffentsorgung"**

- 11880, Eintragsdaten 17.06.2023: **"Lenhardt Heiko Rohstoffentsorgung"**,
  Roederstr. 12, 67363 Lustadt, (06347) 1240, Branchen "Entsorgung" und "Brennstoff",
  ohne Website, E-Mail und Öffnungszeiten.
- Gelbe Seiten, Suche "Lenhardt" + "Lustadt" (3 Treffer): **"Lenhardt Heiko
  Rohstoffentsorgung"**, Roederstr. 12, 67363 Lustadt, 06347 12 40, Branche "Brennstoffe".
- Einzelunternehmer ohne HRB, keine eigene Betreiberseite. "Rohstoffentsorgung"
  bezeichnet eine Entsorgungsleistung, **keinen** Schrottankauf.
- *Bestaetigt.* Die Namensvetter-Gruppe (Recycling Lenhard Andreas, Rohstoff Recycling
  Lenhard OHG/GB, Robert Lenhart, Lenhart Philipp, Martin Lenhardt) in Lingenfeld bleibt
  getrennt und wird nicht zusammengefuehrt.

**(4) `by-miltenberg-ballweg-schrott` -> „Schrotthandel Ballweg"**

- Gelbe Seiten: **"Schrotthandel Ballweg"**, Branche "Schrott", Berliner Str. 3,
  63916 Amorbach, 09373 9 91 02.
- 11880, Eintragsdaten vom 16.06.2023: **"Schrotthandel Ballweg"**, Berliner Str. 3,
  63916 Amorbach, (09373) 99102, Branchen "Altmaterialhandel", "Schrotthandel",
  "Altmetallrecycling".
- Keine Betreiberseite; ballweg.de ist eine Domain-Parking-Seite. Kein zweiter
  Ballweg-Betrieb im Landkreis; "Miltenberg" im Slug ist eine Regionsangabe, der Sitz
  ist Amorbach.
- *Bestaetigt.*

**(5) `mv-neddemin-schrott-bobzien-zweitstandort` -> „(Zweitstandort?)" entfernen**

- Das Kleinanzeigen-Betreiberprofil "Karsten Bobzien" war am 28.10.2026 lesbar
  (HTTP 200, am 14.10.2026 noch 403): "Gewerblicher Anbieter", "Aktiv seit 01.01.2014",
  15 Anzeigen online, 267 gesamt, 211 Follower, Telefon 0395/4226656, Anschrift
  **Dorf Straße 19, 17039 Neddemin**; die Inserate vom 06.10. bis 25.10.2026 schreiben
  durchgehend "Verkauf erfolgt ab Hof 17039 Neddemin OT Hohenmin, Dorf Straße 19".
- Der Verzeichniseintrag "Schrott Bobzien" (schrottplatz-info.de) nennt dieselbe
  Anschrift Dorfstraße 19, 17039 Neddemin OT Hohenmin, Telefon 03961/229311 und
  "Ankauf von Schrott, abholung mit LKW und Lodeskran"; er liegt im Ordner "Brunn",
  einem **Neubrandenburger Stadtteil rund 11 km entfernt** - ein Ablagefehler des
  Verzeichnisses.
- Der zweite Verzeichniseintrag "Karsten Bobzien" (Otto-Lilienthal-Str. 48, 17036
  Neubrandenburg) nennt 0395/4226656, also **genau die Nummer des Betreiberprofils
  fuer Hohenmin**. Die Anschrift Otto-Lilienthal-Str. 48 hat keine Betreiberquelle.
- Ergebnis: **ein Betrieb, ein Standort.** Das Kennzeichen "(Zweitstandort?)" ist ein
  Verzeichnisartefakt und kann aus dem Namen. Der Name wird `Schrott Bobzien`
  (die Marke des Verzeichniseintrags); der Betriebsprofil-Name waere "Karsten Bobzien".
- *Bestaetigt.* Unveraendert bleiben: `status: pruefung` (im aktuellen Betreiberprofil
  kommt das Wort "Schrott" nicht vor, die Schrott-Zusage stammt allein aus dem undatierten
  Verzeichniseintrag) und `provenance_ankauf_raw: Ankauf unklar`.
- **Strukturdefekt in derselben Datei:** das Dossier enthaelt **zwei** `## Überblick`
  Abschnitte; der zweite ist der aktualisierte Text vom 28.10.2026. Beim Uebertragen
  sollte der erste Platzhalter entfernt und der zweite an seine Stelle gesetzt werden
  (kein Timeline-Bullet, daher kein Verstoß gegen den Overwrite-Scan).

**(6) `rp-koblenz-d-laroche-horchheim` -> Korrektur statt Bestaetigung**

- **Schreibweise des Inhabers:** Das Impressum schreibt "**Dominic Larohe**" (ein "c"
  fehlt), die Fusszeile derselben Seite "© 2025 Dominic **Laroche** - Recyling", der
  Text auf der Ueber-uns-Seite "Dominic **Laroche** gruendet das Unternehmen", das
  Chat-Widget "D. **Laroche** / Inhaber / Dominic" und die 11880-Ueberschrift "D.
  **Laroche** Recycling ...". Vier von fuenf Eigenaussagen der Betreiberseite schreiben
  **Laroche**; das Impressum ist der Tippfehler. Ein Registereintrag existiert nicht
  (Einzelunternehmer ohne HRB), die Namensform ist also betreiberseitig zu belegen, nicht
  amtlich.
- **Markenform:** Der Seitentitel der Startseite lautet "**D. Laroche Schrotthandel**
  Koblenz | Recycling & Container", die H1 "D. Laroche Schrotthandel aus Koblenz", die
  Ueber-uns-Seite "D. Laroche - Schrotthandel & Demontage seit 2008", die Kontaktseite
  "Kontaktieren Sie D. Laroche - Schrott & Recycling".
- Der Vorschlag des Owners „D. Laroche Recycling" ist die **Verzeichnisform von 11880**
  (dort der Titelpraefix eines laengeren Eintragsnamens), **nicht** die Eigenbezeichnung
  der Betreiberseite.
- *Korrektur empfohlen:* `name: D. Laroche Schrotthandel`. Das Namensfragment
  "Horchheim" ist ein Quartier (OSM fuehrt die Mendelssohnstraße 11 dem Stadtteil
  Horchheim in 56076 Koblenz zu) und kann entfallen, da Stadt und PLZ die Zeile bereits
  tragen. „D. Laroche Recycling" bleibt als dokumentierte Verzeichnisform in der Notiz.

### 4.3 Entwurf: Aenderungen am `name`-Feld

Alle sechs Aenderungen sind reine `name`-Aenderungen (kein Feld leer, kein Feld
gefuellt). Sie aendern den `seed_hash` und damit `updated_at`; `first_seen_at` bleibt.

| Datei | alt | neu |
|---|---|---|
| `dossiers/by/by-rotal-schubert.md` | `name: Schubert` | `name: Schrott & Recycling Ingo Schubert` |
| `dossiers/by/by-alzenau-jr-recycling.md` | `name: JR-Recycling` | `name: AC Containerdienst UG (haftungsbeschränkt)` |
| `dossiers/rp/rp-lustadt-lenhardt-lustadt.md` | `name: Lenhardt Lustadt` | `name: Lenhardt Heiko Rohstoffentsorgung` |
| `dossiers/by/by-miltenberg-ballweg-schrott.md` | `name: Ballweg Schrott` | `name: Schrotthandel Ballweg` |
| `dossiers/mv/mv-neddemin-schrott-bobzien-zweitstandort.md` | `name: Schrott Bobzien (Zweitstandort?)` | `name: Schrott Bobzien` |
| `dossiers/rp/rp-koblenz-d-laroche-horchheim.md` | `name: D. Laroche Horchheim` | `name: D. Laroche Schrotthandel` |

**Textbausteine, die in die `description` der betroffenen Zeilen gehoeren** (die
`description` wird derzeit an mehreren Stellen mit dem alten Namen gefuehrt, sonst
widerspricht der Datensatz sich selbst):

- `by-rotal-schubert`: `... Inhaber laut Verzeichnissen Ingo Schubert ...` ist bereits
  richtig; ergaenzend „Die Firmierung lautet in beiden unabhaengigen Verzeichnissen
  „Schrott & Recycling Ingo Schubert" bzw. „Schubert Ingo Schrott"; ein
  Handelsregistereintrag existiert nicht (Einzelunternehmer)."
- `by-alzenau-jr-recycling`: `... Der Seed-Name „JR-Recycling" ist der fruehere
  Geschäftsname (Juliano Jährig, Raiffeisenstr. 26, Freigericht) ...` -> `... Der
  fruehere Geschäftsname „JR-Recycling" (Juliano Jährig, Raiffeisenstr. 26, Freigericht)
  ist seit dem 14.07.2026 durch die Eintragung der AC Containerdienst UG (haftungs-
  beschränkt) ersetzt ...`
- `rp-lustadt-lenhardt-lustadt`: `... (Inhaber laut Verzeichnis Heiko Lenhardt) ...` ->
  `... Die Verzeichnisse fuehren den Betrieb als „Lenhardt Heiko Rohstoffentsorgung";
  ein Registereintrag existiert nicht (Einzelunternehmer) ...`
- `by-miltenberg-ballweg-schrott`: `... Schrotthandel und Altmetallrecycling „Ballweg"
  ...` -> `... Schrotthandel und Altmetallrecycling „Schrotthandel Ballweg" ...`
- `mv-neddemin-schrott-bobzien-zweitstandort`: `... die Marke „Schrott Bobzien" stammt
  aus einem Verzeichniseintrag ...` bleibt; ergaenzt wird „Der Namensbestandteil
  „(Zweitstandort?)" ist am 28.10.2026 als Verzeichnisartefakt aufgeloest und wurde
  entfernt; der Betrieb unterhaelt nur den Standort Dorfstraße 19, 17039 Neddemin OT
  Hohenmin."
- `rp-koblenz-d-laroche-horchheim`: `... („D. Laroche Recycling", Inhaber laut Impressum
  Dominic Larohe) ...` -> `... (Marke laut Betreiberseite „D. Laroche Schrotthandel";
  das Impressum schreibt den Namen „Dominic Larohe", die Fusszeile und der Text auf der
  Ueber-uns-Seite dagegen „Dominic Laroche" - die korrekte Schreibweise ist Laroche) ...`

**Je eine neue Timeline-Notiz** (bestehende Notizen bleiben unveraendert), Format
`[Recherche 28.10.2026: ...; Quelle(n): ...]`. Beispiel fuer den Laroche-Eintrag:

```markdown
### Recherche 28.10.2026: Namen korrigiert, Schreibweise geklaert

- [Recherche 28.10.2026: NAMENSKORREKTUR, vom Owner am 28.10.2026 zur belegten Umsetzung
  offengelassen. Der Seed-Name „D. Laroche Horchheim" wird auf „D. Laroche Schrotthandel"
  gesetzt. Belege aus der Betreiber-Primärquelle laroche-recycling.com, am 28.10.2026
  vollständig gelesen: Seitentitel der Startseite "D. Laroche Schrotthandel Koblenz |
  Recycling & Container", Ueberschrift "D. Laroche Schrotthandel aus Koblenz",
  Ueber-uns-Titel "D. Laroche - Schrotthandel & Demontage seit 2008", Kontaktseite
  "Kontaktieren Sie D. Laroche - Schrott & Recycling", Fusszeile "© 2025 Dominic Laroche -
  Recyling" und Chat-Widget "D. Laroche / Inhaber / Dominic". Das Impressum schreibt
  dagegen "Dominic Larohe" (ein "c" fehlt) und ist damit der Tippfehler; vier von fuenf
  Eigenaussagen der Betreiberseite schreiben Laroche. Der vom Owner vorgeschlagene
  Firmierungsname „D. Laroche Recycling" ist belegt, aber nur als Praefix des laengeren
  11880-Eintrags "D. Laroche Recycling und Demontagen aller Art sowie Abbruch und
  Rückbau von Industriemaschinen" und nicht als Eigenbezeichnung der Betreiberseite.
  Ein Handelsregistereintrag existiert nicht (Einzelunternehmer ohne HRB), die
  Namensform ist also betreiberseitig belegt, nicht amtlich. Das Namensfragment
  "Horchheim" ist ein Quartier (OSM fuehrt die Mendelssohnstraße 11 dem Stadtteil
  Horchheim in 56076 Koblenz zu) und entfaellt, da Stadt und PLZ die Zeile bereits
  tragen. Die Namensvetter im Raum (R&S Laroche GbR Muelheim-Kaerlich, Nutzfahrzeughandel
  Laroche Kaltenengers, Roxana Laroche, Gianna Laroche, Denise/Macel Laroche) bleiben
  eigenstaendige Betriebe. ANKAUF, VERKAUF und GEBUEHREN unveraendert: keine
  Ankaufspreise, keine Verkaufsliste, Abholung kostenfrei ab 500 kg, Container 7-20 m3
  ohne Tarif. Quelle(n): https://laroche-recycling.com/ ; https://laroche-recycling.com/ueber-uns/ ;
  https://laroche-recycling.com/impressum/ ; https://laroche-recycling.com/kontakt/ ;
  https://www.11880.com/branchenbuch/koblenz-am-rhein/060440092B113564383/d-laroche-recycling-und-demontagen-aller-art-sowie-abbruch-und-rueckbau-von-industriemaschinen.html ;
  https://nominatim.openstreetmap.org/search?q=Mendelssohnstra%C3%9Fe+11%2C+56076+Koblenz&format=jsonv2&limit=5 ;
  read-only `public.db` (28.10.2026).]
```

---

## 5. Kurzfassung der Empfehlung an den Owner

| Fall | Welche Zeile traegt welchen Betrieb | Aktion |
|---|---|---|
| 1 | `hh-billbrook-billbrook-oberwerder-damm-1-veddel` traegt die **HEG** (Bredowstraße 13, HRB 53960) | Zeile auf HEG umbenennen und entschachteln; **neues** Dossier `hh-veddel-remex-nord-gmbh` fuer die REMEX Nord GmbH (Oberwerder Damm 1-5, ZZBE00100010) |
| 2 | `nw-gelsenkirchen-mitte-schrotthandler-traut-ge-ableger` traegt den **einzigen** Traut-Betrieb (Anschrift-Lead Kurt-Schumacher-Straße 80) | `nw-essen-schrotthandler-traut` wird als Dublette mit Querverweis gefuehrt, nicht geloescht; beide Punkte mit `clear_fields: geo` zuruecknehmen |
| 3 | `he-schaafheim-64850-he-reining-klaus-containerdienst` traegt den **heutigen** Betrieb Containerdienst Schaafheim (Salih Gencsoy, Ringstraße 7) | die beiden Reining-Zeilen bleiben als historische Altbetriebe mit Querverweis bestehen und werden untereinander als Dublette gefuehrt; keine Loeschung |
| 4 | - | `name` aendern bei `by-rotal-schubert`, `by-alzenau-jr-recycling`, `rp-lustadt-lenhardt-lustadt`, `by-miltenberg-ballweg-schrott`, `mv-neddemin-schrott-bobzien-zweitstandort`; **korrigiert** `rp-koblenz-d-laroche-horchheim` auf „D. Laroche Schrotthandel" (Schreibweise Laroche) |

### 5.1 Entscheidungen, die der Owner noch treffen muss

1. Fall 1: `trader_type` der HEG-Zeile (`sonstige` -> `containerdienst`?) und
   `trader_type` des neuen REMEX-Dossiers (`schrottplatz` vs. `sonstige`).
2. Fall 1: Adressform des neuen Dossiers - `Oberwerder Damm 1-5` (amtliches Register,
   Empfehlung) oder `Oberwerder Damm 3-5` (Betreiberform). Keine Koordinate, bis die
   Frage geklaert und eine zweite Punktquelle vorhanden ist.
3. Fall 2: ob der Aggregator-Lead als ausreichend gilt. Bei "ja": `street`/`postcode`
   und der OSM-Hauspunkt in der Gelsenkirchen-Zeile; bei "nein" (Empfehlung): leer
   lassen.
4. Fall 2: `name` der beiden Traus-Zeilen (`Schrotthändler Traut (Gelsenkirchen)` /
   `(Essen)`) - oder ohne Namensaenderung nur Beschreibung und Querverweis.
5. Fall 3: Leitzeile im Cluster (Empfehlung `...-he-reining-klaus-containerdienst`) oder
   ein neues Dossier `he-schaafheim-containerdienst-schaafheim` (Variante aus dem
   60:00-Wellenbericht). `status` der Leitzeile `pruefung` (Empfehlung) oder `aktiv`.
6. Fall 3/4: ob die historischen Zeilen zur Kennzeichnung `(Altbetrieb)` umbenannt
   werden.
7. Fall 4: `rp-koblenz-d-laroche-horchheim` - „D. Laroche Schrotthandel" (Empfehlung,
   Betreiberform) statt „D. Laroche Recycling" (11880-Form).

### 5.2 Was bewusst **nicht** getan wurde

- Keine Dossier-Datei angelegt oder geaendert, kein Commit, kein Push, kein Deploy.
- Kein Slug geaendert, keine bestehende Timeline-Notiz entfernt oder ueberschrieben.
- Keine Produktionsdatenbank geschrieben (nur `mode=ro` + `PRAGMA query_only=ON`).
- Keine Koordinate ohne zwei unabhaengige Quellen gesetzt.
- Keine Ankaufskurse aus Portal-Preisen abgeleitet (Schrottfinder, 11880, Gelbe Seiten).
- Die gebuehrenfreie Annahme der Stadtreinigung nicht als Ankauf behandelt.
- Register-Negativbefunde nicht als Nichtexistenz gewertet.
- Kein Beleg fuer den Efb-Zertifikats-PDF der REMEX Nord behauptet (Abruf 403, nicht
  gelesen); die Zertifikatsnummer stammt aus dem amtlichen Register.
- Die Rechtsnachfolge der REMEX Nord ueber die ETH-Umwelttechnik GmbH wurde als
  Betreiberangabe dokumentiert, nicht als Registertatsache.

---

## 6. Quellenverzeichnis (alle am 28.10.2026 abgerufen)

**Amtliche Quellen**

- https://www.hamburg.de/branchenbuch/hamburg/eintrag/10220456/ (HEG, Eintragsnummer 10220456)
- https://files.stadtreinigung.hamburg/SRH_25_028_Info_Broschuere_2026_Web.pdf (Broschuere INFO 2026, S. 12)
- https://www.hamburg.de/leben-in-hamburg/wohnen/stadtreinigung/recyclinghoefe-hamburg-379168 (Recyclinghoefe, Liebigstrasse 66)
- https://schrottregister.pages.dev/betrieb-remex-nord-gmbh-hamburg-2 (Oberwerder Damm 1-5, ZZBE00100010)
- https://schrottregister.pages.dev/betrieb-remex-nord-gmbh-hamburg (Einsiedeldeich 15)
- https://schrottregister.pages.dev/ort-hamburg
- https://schrottregister.pages.dev/ort-schaafheim
- https://www.eth-umweltservice.de/fileadmin/content/eth_wertstoffrecycling/ZZBT005000349003_EFB-Zertifikat.pdf (Vorgaenger ETH Wertstoffrecycling, Oberwerder Damm 1-5)

**Betreiber-Quellen**

- https://www.remex-nord.de/ (Standortseite Oberwerder Damm, Oeffnungszeiten, Standortbeschreibung)
- https://www.remex.de/profil/standorte/
- https://www.containerdienst-schaafheim.de/assets/Seo-DTvtlKDA.js
- https://www.containerdienst-schaafheim.de/assets/UeberUns-CamdrXxU.js
- https://www.containerdienst-schaafheim.de/assets/Kontakt-u3FhrzKh.js
- https://www.containerdienst-schaafheim.de/assets/Impressum-DA8ZMBv_.js
- https://laroche-recycling.com/ , /ueber-uns/ , /impressum/ , /kontakt/
- https://www.ac-containerdienst.de/impressum
- https://www.kleinanzeigen.de/pro/Karsten-Bobzien

**Verzeichnisse (Leads, keine Evidenz)**

- https://schrottfinder.de/schrotthandler-traut-gelsenkirchen-gelsenkirchen/
- https://schrottfinder.de/schrotthandler-traut-essen/
- https://www.11880.com/branchenbuch/schaafheim/060371058B112441803/containerdienst-schaafheim-salih-gencsoy.html
- https://www.11880.com/branchenbuch/rosstal/060440092B113221949/schrott-recycling-ingo-schubert.html
- https://www.11880.com/branchenbuch/lustadt/060441364B106707703/lenhardt-heiko-rohstoffentsorgung.html
- https://www.11880.com/branchenbuch/amorbach/120674719B43369107/schrotthandel-ballweg.html
- https://www.11880.com/branchenbuch/koblenz-am-rhein/060440092B113564383/d-laroche-recycling-und-demontagen-aller-art-sowie-abbruch-und-rueckbau-von-industriemaschinen.html
- https://www.gelbeseiten.de/gsbiz/79e3a92a-c4e8-49b0-808a-8c744db9a169 (Schrotthandel Ballweg)
- https://www.gelbeseiten.de/suche/lenhardt/lustadt
- https://www.gelbeseiten.de/suche/gencsoy/schaafheim
- https://www.gelbeseiten.de/gsbiz/b4179829-6019-4dd1-8b6a-f6af9fcbade9 (JR-Recycling)
- https://www.dasoertliche.de/Themen/Schubert-Ingo-Schrott-Ro%C3%9Ftal-Gewerbestr
- https://www.dasoertliche.de/Themen/Schrotth%C3%A4ndler/Gelsenkirchen.html
- https://www.dasoertliche.de/Themen/Schrotth%C3%A4ndler/Essen.html
- https://www.schrottplatz-info.de/schrottplatz/Brunn/Schrott-Bobzien-

**Register und Geodaten**

- https://www.northdata.de/HEG+Hamburger+Entsorgungsgesellschaft+mbH,+Hamburg/HRB+53960
- https://www.northdata.de/REMEX+Nord+GmbH,+Hamburg/HRB+133957
- https://www.registercheck.de/companies/remex-nord-gmbh
- https://abn-online.de/impressum (ABN Baustoffueberwachung Nord, Oberwerder Damm 3)
- https://api.openstreetmap.org/api/0.6/map.json?bbox=10.0240,53.5230,10.0330,53.5300
- https://www.openstreetmap.org/way/263862026 (Kurt-Schumacher-Strasse 80, Gelsenkirchen)
- https://www.openstreetmap.org/way/961378576 (Ringstrasse 7, Schaafheim)
- https://www.openstreetmap.org/way/799842912 (Karlstrasse 1, Schaafheim)
- https://nominatim.openstreetmap.org/search?q=Ringstra%C3%9Fe+7%2C+64850+Schaafheim&format=jsonv2&limit=5&addressdetails=1
- https://nominatim.openstreetmap.org/search?q=Kurt-Schumacher-Stra%C3%9Fe+80%2C+45881+Gelsenkirchen&format=jsonv2&addressdetails=1
- https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.9242&lon=9.0094&zoom=18&addressdetails=1
- read-only `public.db` (`file:/var/lib/schrott-mcp/public.db?mode=ro`, `PRAGMA query_only=ON`, Abruf 28.10.2026)
