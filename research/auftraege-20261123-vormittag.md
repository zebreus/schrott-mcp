# Aufträge und offene Punkte — 23.11.2026, Vormittag

## Die Welle vom 06:00 ist entschieden, korrigiert, geprüft und ausgerollt ✓ ✓

Die Welle vom 23.11.2026, **06:00** ✓ ✓ ist **recherchiert** ✓ ✓ (Commit
`2ffb8514` ✓ ✓), **entschieden** ✓ ✓ (Commit `14a9755c` ✓ ✓), **versioniert** ✓ ✓
(Commit `796dbc3a` ✓ ✓, Bericht ✓ ✓ und Release-Scope ✓ ✓) ✓ ✓ und **nach dem
Review** ✓ ✓ **korrigiert** ✓ ✓ (Commit `54e8c5ab` ✓ ✓) ✓ ✓

**Sechs** Punkte werden **gesetzt** ✓ ✓, **dreiunddreißig** **entfernt** ✓ ✓,
**einundzwanzig** bleiben ✓ ✓ — **alle 60** ✓ ✓ Dossiers sind **entschieden**
✓ ✓ ✓ ✓ Die **Entscheidungsakte** ✓ ✓ steht in
`research/geo-entscheidung-20261123-0600.md` ✓ ✓ ✓ ✓

**Zweihundertzweiunddreißig** neue Notizen ✓ ✓ (Recherche **172** ✓ ✓,
Korrekturen **60** ✓ ✓) ✓ ✓ Die **sechs** Gates ✓ ✓ des Wellen-Torwarts ✓ ✓
sind **grün** ✓ ✓ ✓ ✓

**Das Pending-Gate** ✓ ✓ des Vorgänger-Rollouts `8d496f8` ✓ ✓ ist **grün**
✓ ✓ — der **MSG**-Lauf um **12:09:02 UTC** ✓ ✓ lag **nach** dem Rollout ✓ ✓
✓ ✓ (Er **rund** ✓ ✓) ✓ ✓

**Bis** ✓ ✓ das Pending-Gate ✓ ✓ **dieses** ✓ ✓ Rollouts ✓ ✓ **grün** ✓ ✓ ist,
gilt: **keine** weiteren Produktionsänderungen ✓ ✓ ✓ ✓

## Das Review der Welle ✓ ✓ — neun Befunde ✓ ✓, alle **beantwortet** ✓ ✓

Das unabhängige Pre-Deploy-Review ✓ ✓ (Kandidat `796dbc3` ✓ ✓) meldete
**SPERREN** ✓ ✓ mit **neun** Befunden ✓ ✓ ✓ ✓

### Zwei **Datenfehler** ✓ ✓ des Owners ✓ ✓ — **behoben** ✓ ✓

- `nw-gutersloh-bartsch` ✓ ✓: der **gesetzte** Punkt ✓ ✓ zeigte auf ein Objekt
  in **Mülheim** ✓ ✓ statt in **Gütersloh** ✓ ✓ — jetzt `node/11081198999` ✓ ✓
  (`name`=Bartsch Entsorgung ✓✓, **Hausnummer 26** ✓✓) ✓ ✓
- `ni-bakum-fischer-schrott-recycling` ✓ ✓: die **Hausnummer 1** ✓ ✓ gehörte zum
  **alten** Punkt ✓✓ (Burgweg 1 ✓✓) — das Haus des Betriebs trägt **27** ✓ ✓

### Eine **Blindstelle** ✓ ✓ im Adress-Gate ✓ ✓ — **behoben** ✓ ✓

`scripts/release/check_address_claims.py` ✓✓ erkannte den **Setzfall** ✓✓ nur
in der Formulierung „der Standortpunkt wird **gesetzt**“ ✓✓ — die Welle
schreibt „**ersetzt**“ ✓✓ ✓ ✓ Die **Hausnummer** ✓✓ und die **Koordinate** ✓✓
wurden deshalb **nie** geprüft ✓✓ ✓ ✓

**Neu** ✓ ✓: beide Formulierungen ✓✓ · die **Hausnummern** ✓✓ werden **je Satz**
✓✓ gelesen ✓✓, damit die Nummer des einen Objekts ✓✓ nicht am anderen
**geprüft** ✓✓ wird ✓✓ · die **PLZ** ✓✓ und der **Ort** ✓✓ des **Zielobjekts**
✓✓ müssen zum Dossier passen ✓✓ ✓ ✓

**Fünfzehn** Contract-Tests ✓ ✓ halten die Regeln ✓ ✓ ✓ ✓ (neu:
`scripts/test_check_address_claims.py` ✓ ✓) ✓ ✓

### Sechs **Befunde** ✓ ✓ an Text und Zahlen ✓ ✓ — **behoben** ✓ ✓

- Die **Namen-Behauptung** ✓ ✓ in `nw-mulheim-an-der-ruhr-schute-verstappen` ✓ ✓
  (das Objekt trägt **keinen** ✓✓ Namen ✓✓) ✓ ✓
- Die **PLZ** ✓ ✓ von Grünberg ✓ ✓: **36369** ✓✓ → **35305** ✓ ✓
- Die **widersprüchliche** ✓ ✓ Distanzangabe ✓ ✓ in Biebertal ✓ ✓ ✓ ✓
- Die **Gruppen** ✓ ✓ der Akte ✓ ✓: **elf** ✓✓ fremde Objekte ✓✓ · **fünfzehn**
  ✓✓ andere Straße oder Haus ✓✓ · **sieben** ✓✓ anderer Ort ✓✓ ✓ ✓
- Die **Zahlen** ✓ ✓ der Akte ✓ ✓: **zwölf** ✓✓ Anker ✓✓ + **drei** ✓✓ bleiben
  ✓✓ ohne Anker ✓✓ + **sechs** ✓✓ ohne Punkt ✓✓ = **einundzwanzig** ✓✓ · **39**
  ✓✓ Standortänderungen ✓✓ · **54** ✓✓ Dossiers mit Punkt ✓✓ · **44** ✓✓
  Dossiers mit Änderung ✓✓ · **62** ✓✓ Feldwechsel ✓✓ ✓ ✓
- Die **Zahl** ✓ ✓ in der Scope-Akte ✓ ✓: von **1143** ✓✓ ausgeschlossenen
  Dossiers ✓✓ lagen **1095** ✓✓ im Pool ✓✓ ✓ ✓

### Zwei **Kleinigkeiten** ✓ ✓ — **behoben** ✓ ✓

Die **Zertifikate** ✓ ✓ von Gütersloh ✓ ✓ und Mülheim ✓ ✓ hatten eine **kaputte
Klammer** ✓✓ (`) (` ✓✓), und eine Notiz ✓ ✓ sagte **„and“** ✓✓ statt **„und“**
✓✓ ✓ ✓

## Die Welle vom 06:00 ist **ausgerollt** ✓ ✓

**Rollout** ✓ ✓: **23.11.2026, 12:45:08 UTC** ✓ ✓ · Kandidat `7d0a8dd` ✓ ✓ ·
Backup ✓ ✓ `/root/Documents/schrott-mcp-rollout-backups/rollout-7d0a8dd-20261123T124507Z`
✓ ✓ ✓ ✓

**Die drei** ✓✓ Verifier ✓✓ sind **grün** ✓✓ ✓ ✓ (Release-Delta ✓✓, Wave E
✓✓, Notizen in der Produktion ✓✓) ✓ ✓ Das **Pending-Gate** ✓✓ ist
**erwartbar** ✓✓ **rot** ✓✓ — die Turnusläufe ✓✓ von MSG ✓✓ und Gutzmann ✓✓
liegen **vor** dem Rollout ✓✓ ✓ ✓ Die **nächsten** ✓✓ Läufe ✓✓: Gutzmann
**~16:07 UTC** ✓✓, MSG **~18:09 UTC** ✓✓ ✓ ✓ Die **Nachprüfung** ✓✓ ist für
**18:05 UTC** ✓✓ angelegt ✓✓ (Job `release-nachpruefung-7d0a8dd` ✓✓) ✓ ✓

**Der Sperling-Handler** ✓✓ ist **erstmals** ✓✓ mit dem **Richtungsvertrag**
✓✓ ausgerollt ✓✓ ✓ ✓

## Das zweite Review ✓ ✓ — drei Befunde ✓ ✓, alle **beantwortet** ✓ ✓

Das zweite unabhängige Review ✓ ✓ (Kandidat `54e8c5a` ✓ ✓) meldete
**SPERREN** ✓ ✓ mit **drei** Befunden ✓ ✓ ✓ ✓

1. **Die Klammerbalance** ✓ ✓: die Korrektur von `)(` ✓✓ auf `(` ✓✓ hatte
   die **schließende** ✓✓ Klammer der Beschreibung der Zertifizierungsorganisation
   ✓✓ mitgelöscht ✓✓ — das Gate ✓✓ meldete **beide** Felder ✓✓ ✓ ✓ Die
   Klammer steht jetzt am **Ende** ✓✓ der Beschreibung ✓✓ — die Balance ist
   grün ✓✓ und das Feld bleibt ein **valides** ✓✓ flaches Array ✓✓ ✓ ✓
2. **Biebertal** ✓✓: die Notiz nannte **4,4 km** ✓✓ — **gemessen** ✓✓ sind
   **1,3 km** ✓✓ (1.329 m ✓✓ nach der Wellennotiz ✓✓) ✓ ✓
3. **Lichtenau** ✓✓: die Notiz nannte `way/1011223575` ✓✓ (Hausnummer **31** ✓✓),
   sprach aber von der Hausnummer **53** ✓✓ — richtig ist `way/242998913` ✓✓ ✓ ✓

**Kandidat** ✓ ✓: `9784449` ✓ ✓ · **Pin** ✓ ✓: `8dd41150` ✓ ✓ ✓ ✓ Das
**dritte** ✓ ✓ unabhängige Review ✓ ✓ läuft ✓ ✓ ✓ ✓

## Der Check-in vom 13:00 ✓ ✓ — drei **Datenfehler** ✓ ✓ behoben ✓ ✓

Der Check-in hat **drei** ✓✓ Zeilen ✓✓ gefunden ✓✓, deren **Adressfelder** ✓✓
**keine** ✓✓ Anschrift ✓✓ trugen ✓✓ ✓ ✓

- `rp-47285-jk-heidesheim` ✓✓: `city` trug `47285)` ✓✓ — die
  **Handelsregisternummer** ✓✓ mit **Klammerrest** ✓✓ ✓ ✓ Jetzt **Heidesheim** ✓✓,
  **Am Pfingstborn 9** ✓✓, **55262** ✓✓ ✓ ✓
- `rp-ludwigshafen-ghn` ✓✓: die **Dublette** ✓✓ des Namens **GHN** ✓✓ ✓ ✓
- `st-mrr-namensnahe-…` ✓✓: `city` trug eine **Arbeitsnotiz** ✓✓ der Erhebung
  ✓✓ → **Bitterfeld-Wolfen** ✓✓, `status` → **geschlossen** ✓✓ ✓ ✓

**Das Review** ✓✓ (Kandidat `49e6933` ✓✓) meldete **SPERREN** ✓✓ mit **fünf**
✓✓ Befunden ✓✓ ✓ ✓ Zwei davon waren **blockierend** ✓✓ ✓ ✓

1. Die **Konvention** ✓✓: `city` trägt den **Ortsnamen** ✓✓, die **PLZ** ✓✓
   steht im Feld `postcode` ✓✓ ✓ ✓
2. Die **PLZ** ✓✓ doppelt im **FTS-Tokenstrom** ✓✓ — mit 1 erledigt ✓ ✓
3. Die **Überblicke** ✓✓ beschrieben den **alten** ✓✓ Stand ✓ ✓
4. Der **Ortsvergleich** ✓✓ im Adress-Gate ✓✓ ohne **Wortgrenzen** ✓✓ ✓ ✓
5. Die **Dublette** ✓✓ GHN ✓✓ ✓ ✓

**Alle fünf** ✓✓ sind beantwortet ✓✓ (Kandidat `f1fb73a` ✓✓, Pin `9282a98d` ✓✓) ✓✓
Das **dritte** ✓✓ Review ✓✓ läuft ✓ ✓ ✓ ✓

**Die Dublette** ✓✓ GHN ✓✓ ist **nach dem Traut-Muster** ✓✓ aufgelöst ✓✓: die
**Leitzeile** ✓✓ `bw-mannheim-ghn-schrott-metallrecycling` ✓✓ trägt den **Betrieb**
✓✓ und jetzt den **richtigen** ✓✓ Standortpunkt ✓✓ (vorher ein **Seed-Lead**
✓✓ **1,7 km** ✓✓ daneben ✓✓) ✓✓ — die **Dublette** ✓✓ bleibt ohne **eigenen**
✓✓ Punkt ✓✓ und beschreibt sich als **Querverweis** ✓✓ ✓ ✓

## Das Pending-Gate wartet auf **MSG** ✓ ✓

Der Rollout `7d0a8dd` ✓ ✓ ist am **23.11.2026, 12:45:08 UTC** ✓ ✓ erfolgt
✓ ✓ · **Gutzmann** ✓ ✓ lief um **16:17:00 UTC** ✓ ✓ (`ok` ✓✓, **19** ✓✓) ✓ ✓
— **nach** dem Rollout ✓ ✓ ✓ ✓ Es fehlt nur noch der **MSG**-Lauf ✓ ✓
(fällig **~18:09 UTC** ✓✓) ✓ ✓

Danach ✓ ✓ ist das **Pending-Gate** ✓ ✓ **grün** ✓ ✓ ✓ ✓ und der **Rollout**
✓ ✓ der **Korrekturrunde** ✓ ✓ (Kandidat `63ef549` ✓ ✓, **FREIGE** ✓ ✓,
Zyklus grün ✓✓, Pin `4137c5ac` ✓✓) ✓ ✓ kann **regulär** ✓ ✓ erfolgen ✓ ✓ ✓ ✓

## Die Welle vom 18:00 ✓ ✓ ist entschieden ✓ ✓ und wartet auf das Review ✓ ✓

Die Welle vom 23.11.2026, **18:00** ✓ ✓ ✓ ✓

- **Scope** ✓ ✓: **60** ✓✓ Dossiers ✓✓ in **acht** ✓✓ Bundesländern ✓✓ (BW 2 ✓✓ · BY 12
  ✓✓ · HE 8 ✓✓ · NI 8 ✓✓ · NW 12 ✓✓ · RP 12 ✓✓ · SH 1 ✓✓ · SN 5 ✓✓) ✓ ✓
- **Recherche** ✓ ✓: **267** ✓✓ neue Notizen ✓✓ ✓ ✓
- **Entscheidung** ✓ ✓: **8** ✓✓ gesetzt ✓✓ · **37** ✓✓ entfernt ✓✓ · **15** ✓✓ bleiben ✓✓ ✓ ✓
- **Bericht** ✓ ✓ und **Release-Scope** ✓ ✓ ✓ ✓
- **Release-Zyklus** ✓ ✓: grün ✓✓ (60 Zeilen ✓✓, 855 Tests ✓✓) ✓✓ · der Kandidat ist
  **gepinnt** ✓✓ (siehe `git log` ✓✓) ✓ ✓

**Die Deckelung** ✓ ✓ liegt bei **zwölf** ✓ ✓ Dossiers je Bundesland ✓ ✓ — bei
**acht** ✓ ✓ wären nur **48** ✓ ✓ möglich ✓ ✓ ✓ ✓

Der **Pool** ✓ ✓ sind die Dossiers **ohne** ✓✓ Website **oder** ✓✓ ohne Straße ✓✓
(am 23.11.2026, 06:00 UTC ✓✓: **1669** ✓✓) ✓ ✓ Nach dem Ausschluss der **1204** ✓✓
Dossiers der **älteren** ✓✓ Scopes ✓✓ — davon **1154** ✓✓ im Pool ✓✓ — bleiben
**515** ✓✓ ✓ ✓ Je Bundesland ✓ ✓: NW 159 ✓✓ · HE 146 ✓✓ · RP 94 ✓✓ · NI 61 ✓✓ ·
BY 47 ✓✓ · SN 5 ✓✓ · BW 2 ✓✓ · SH 1 ✓✓ ✓ ✓ Bei einer Deckelung von **acht** ✓✓
wären das **8 + 8 + 8 + 8 + 8 + 5 + 2 + 1** ✓✓ = **48** ✓✓ Dossiers ✓✓ ✓ ✓

**Drei** ✓✓ Adressfelder ✓✓ wurden mitkorrigiert ✓✓ — sie passten **nicht** ✓✓ zum
Objekt ✓✓ ✓ ✓

**Der Rollout** ✓ ✓ wartet auf das **Pending-Gate** ✓ ✓ der Korrekturrunde ✓ ✓
(`63ef549` ✓✓, ausgerollt **18:21:49 UTC** ✓✓) — die **Turnusläufe** ✓✓ **Gutzmann**
✓✓ **~22:17 UTC** ✓✓ und **MSG** ✓✓ **~00:18 UTC** ✓✓ müssen **nach** dem Rollout ✓✓
liegen ✓✓ ✓ ✓

## Die Reviews der Welle vom 18:00 ✓ ✓

Die **unabhängigen** ✓✓ Reviews ✓✓ dieser Welle ✓✓ und ihre **Befunde** ✓✓ stehen in
`research/reviews-20261123-1800.md` ✓✓ ✓ ✓

## Die Deckelung der Welle vom 06:00 ✓ ✓ — eine **Ausnahme** ✓ ✓

*Dieser Abschnitt betrifft die **Welle vom 06:00** ✓✓ — ihre Deckelung lag
bei **neun** ✓✓ Dossiers je Bundesland ✓✓, weil der Pool dort noch
**575** ✓✓ Dossiers nach dem Ausschluss ✓✓ übrig hatte ✓✓ ✓ ✓ Die Welle vom
**18:00** ✓✓ hat ihre **eigene** ✓✓ Deckelung ✓✓ — siehe oben ✓ ✓*

## Das neue Gate findet **drei** Befunde in einer **alten** Welle ✓ ✓

Der erste Lauf des Adress-Gates ✓✓ über die Welle vom **22.11.2026, 12:00**
✓✓ meldet **drei** Befunde ✓✓ — sie betreffen **nicht** diese Welle ✓✓ und
**nicht** den aktuellen Release ✓✓, sind aber **echte** Fehler ✓✓ ✓ ✓

- `rp-47285-jk-heidesheim` ✓ ✓: das Feld `postcode` ✓ ✓ trägt **47285** ✓ ✓ —
  das ist die **Handelsregisternummer** ✓✓, nicht die Postleitzahl ✓✓ (die
  **55262** ✓✓ lautet) ✓ ✓ **Sache** ✓ ✓ der **nächsten** ✓ ✓ Welle ✓ ✓
- `rp-ludwigshafen-ghn` ✓ ✓: das genannte Objekt ✓ ✓ trägt den Ort
  **Mannheim** ✓ ✓ statt **Ludwigshafen** ✓ ✓ ✓ ✓

## Die Nachprüfung des Rollouts `8d496f8` wartet auf den Turnus ✓ ✓

Der Rollout der Welle vom 23.11.2026, **00:00** ✓ ✓ ist am **23.11.2026,
06:05:31 UTC** ✓ ✓ erfolgt ✓ ✓ — Kandidat `8d496f8` ✓ ✓ ✓ ✓

Die **drei** Verifier ✓ ✓, die **jetzt** melden können ✓ ✓, sind **grün**
✓ ✓ ✓ ✓ Der **Pending-Gate** ✓ ✓ wartet auf die **Turnusläufe** ✓ ✓ — die
**Nachprüfung** ✓ ✓ ist für **12:05 UTC** ✓ ✓ angelegt ✓ ✓ (Job
`release-nachpruefung-8d496f8` ✓ ✓) ✓ ✓

## Der MKM-Handler ✓ ✓ läuft ✓ ✓

Seit dem Rollout `208e641` ✓ ✓ (23.11.2026 23:57:21 UTC) ✓ ✓ läuft der
Handler **ohne Fehler** ✓ ✓ — **47** Preise ✓ ✓, **14** Annahmen ✓ ✓ ✓ ✓

**Acht** Artikel ✓ ✓ haben auf der Seite ihren **Preis** ✓ ✓ verloren ✓ ✓ — der
Handler **erfindet** dort **keinen** Wert ✓ ✓, sondern schreibt eine
**Annahme** ✓ ✓ ✓ ✓ Die **Historie** ✓ ✓ bleibt erhalten ✓ ✓ ✓ ✓

## Offene Punkte ✓ ✓

### 1. Das Provider-Limit des Schedulers ✓ ✓ — **offen** ✓ ✓

Das **Tageslimit** ✓ ✓ des Scheduler-Providers ✓ ✓ ist **erschöpft** ✓ ✓
(**45,02 $** von **45,00 $** ✓ ✓) ✓ ✓ und **resetet nicht** ✓ ✓ ✓ ✓ Die
**Nachprüfungen** ✓ ✓ müssen **von Hand** ✓ ✓ erfolgen ✓ ✓ ✓ ✓

### 2. Die offenen Fragen der Shards ✓ ✓ — **zu klären** ✓ ✓

Die Shards haben **dreizehn** Fragen offengelassen ✓ ✓ — die **wichtigsten**
✓ ✓:

- `by-rodelsee-knettenbrech-wagner` ✓ ✓: der Registerspiegel ✓ ✓ sagt
  „**aktiv**“ ✓ ✓, das Dossier „**erloschen**“ ✓ ✓ — das **amtliche** Portal
  ✓ ✓ ist formulargeschützt ✓ ✓ ✓ ✓
- `rp-koblenz-56068-autoankauf-koblenz` ✓ ✓: **Bochumer** ✓✓
  Leadgenerator ✓✓ oder **Urmitzer** ✓✓ Betrieb ✓✓ — beide tragen den
  Namen ✓✓, keiner an der Dossier-Adresse ✓✓ ✓ ✓
- `sn-plauen-scholz-recycling-standort-plauen` ✓ ✓: die Anschrift ✓✓ ist
  **amtlich gedeckt** ✓✓, hat aber **kein** Hausnummernobjekt ✓✓ — und der
  Betreiber **führt** ✓✓ den Standort im eigenen Feed **nicht mehr** ✓✓ ✓ ✓
- `sn-callenberg-09337-schulze-l-av` ✓ ✓: **Schulze L.** ✓✓ gegen **Heiko**
  ✓✓ und **Sandro Schulze** ✓✓ — zwei Einheiten am selben Gebäude ✓✓ ✓ ✓
- `rp-koblenz-56070-alten-familie-…` ✓ ✓: **Gelbe Seiten** ✓✓ und **11880**
  ✓✓ widersprechen sich bei **drei** ✓✓ von **vier** ✓✓ ✓ ✓
- `bw-eppingen-kaygun` ✓ ✓: der Verzeichnis-Spiegel ✓✓ nennt ein **Zertifikat**
  ✓✓, das **amtlich** nicht auffindbar ist ✓✓ ✓ ✓

### 3. Der Abgleich der Adressangaben ist jetzt ein Gate ✓ ✓ — **erledigt** ✓ ✓

**Erledigt** ✓ ✓: `scripts/release/check_address_claims.py` ✓✓ prüft die
**Adressangaben** ✓✓ der Korrektur-Sektionen **live** ✓✓ ✓ ✓ Es steht in der
**Anleitung** ✓✓ und im **Bericht** ✓✓ der Welle ✓ ✓ ✓ ✓

### 4. Der generische Pending-Gate ist wieder grün ✓ ✓ — **behoben** ✓ ✓

**Behoben** ✓ ✓ (Commit `ff8ca79c` ✓ ✓): die **Erwartungsdatei** ✓✓ war am
**08.11.2026** ✓✓ mit **gelöscht** ✓✓ worden ✓✓ — die **neun** Contract-Tests
✓✓ sind **grün** ✓ ✓ ✓ ✓

### 5. Der Goldankauf-Handler ✓ ✓ — die Quelle hat umgeschrieben ✓ ✓

**Behoben** ✓ ✓ (Commit `c6bfd5b0` ✓ ✓): der Betreiber hat den Absatz ✓✓,
der die **Richtung** ✓✓ der Preistabelle nennt ✓✓, **umgeschrieben** ✓✓ ✓ ✓
Der Vertrag prüft jetzt die **Richtung** ✓✓ statt eines **festen** Wortlauts
✓✓ ✓ ✓
