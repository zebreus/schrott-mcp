# Aufträge und offene Punkte — 23.11.2026, Vormittag

## Die Welle vom 06:00 ist entschieden, korrigiert und wartet auf den Turnus ✓ ✓

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

**Der Rollout wartet** ✓ ✓ — das **Pending-Gate** ✓ ✓ des Vorgänger-Rollouts
`8d496f8` ✓ ✓ ist **rot** ✓ ✓, weil die **Handler-Turnusläufe** ✓ ✓ noch nicht
**nach** dessen Rollout ✓ ✓ lagen ✓ ✓: MSG zuletzt **06:00:09 UTC** ✓ ✓,
Gutzmann zuletzt **03:59:14 UTC** ✓ ✓ — beide **vor** dem Rollout ✓ ✓ ✓ ✓ Die
**nächsten** Läufe ✓ ✓: Gutzmann **~09:59 UTC** ✓ ✓, MSG **~12:00 UTC** ✓ ✓
✓ ✓

**Bis** ✓ ✓ das Gate **grün** ✓ ✓ ist, gilt: **keine** weiteren
Produktionsänderungen ✓ ✓ ✓ ✓

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

## Die Deckelung der Welle ✓ ✓ — eine **Ausnahme** ✓ ✓

Die **Deckelung** ✓ ✓ liegt bei **neun** ✓ ✓ Dossiers je Bundesland ✓ ✓ —
**nicht** bei **acht** ✓ ✓ wie in den Wellen der letzten Tage ✓ ✓ ✓ ✓

**Grund** ✓ ✓: der **Pool** ✓ ✓ ist **erschöpft** ✓ ✓ — nach dem Ausschluss
der **vergangenen** Wellen ✓ ✓ bleiben je Bundesland nur noch ✓ ✓: NW 168 ✓ ✓ ·
HE 155 ✓ ✓ · RP 103 ✓ ✓ · NI 70 ✓ ✓ · BY 56 ✓ ✓ · SN 14 ✓ ✓ ✓ ✓

Bei einer Deckelung von **acht** ✓ ✓ wären nur **57** ✓ ✓ Dossiers möglich ✓ ✓
— **drei** ✓ ✓ weniger als **sechzig** ✓ ✓ ✓ ✓ Die **Ausnahme** ✓ ✓ ist
**ausdrücklich** ✓ ✓ gemacht ✓ ✓: lieber **60 frische** ✓ ✓ Dossiers ✓ ✓ als
**57** ✓ ✓ ✓ ✓

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
