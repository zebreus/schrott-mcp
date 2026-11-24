# Die Bereinigung des Ortsfeldes — 24.11.2026, die zweite Korrekturrunde

**Stand 24.11.2026, 10:35 UTC ✓ ✓**

Die Korrekturrunde vom 24.11.2026 hat **hundertfünfzigvier** ✓✓ Dossiers ✓✓
berichtigt ✓✓, in denen das Feld `city` ✓✓ den Ortsnamen **mit** ✓✓ der
Postleitzahl ✓✓ trug ✓✓ und das Feld `postcode` ✓✓ **leer** ✓✓ war ✓✓ ✓ ✓

**Diese** ✓✓ Runde ✓✓ nimmt den **Rest** ✓✓ derselben Sache ✓✓ ✓ ✓

## Der Befund ✓ ✓

**Hundertzweiundachtzig** ✓✓ Zeilen ✓✓ tragen die Postleitzahl **noch** ✓✓ im Feld
`city` ✓✓ ✓ ✓ **Hundertachtundsiebzig** ✓✓ davon haben das Feld `postcode` ✓✓
**schon** ✓✓ **richtig** ✓✓ gefüllt ✓✓ ✓ ✓

| Feld | Wert |
|---|---|
| Die Zeilen ✓ ✓ | **182** ✓ ✓ |
| Mit der PLZ **auch** ✓✓ im Feld `postcode` ✓✓ | **181** ✓ ✓ |
| Geändert ✓ ✓ | **181** ✓ ✓ |
| **Ausgenommen** ✓ ✓ | **1** ✓ ✓ — `rp-11880-…` ✓ ✓ |
| Die **Verteilung** ✓ ✓ | HE **56** ✓✓ · SN **25** ✓✓ · MV **22** ✓✓ · BY **19** ✓✓ · RP **17** ✓✓ · NI **12** ✓✓ · SH **8** ✓✓ · BW **6** ✓✓ · SL **5** ✓✓ · NW **4** ✓✓ · BB **2** ✓✓ · BE **2** ✓✓ · HB **2** ✓✓ · ST **1** ✓✓ · TH **1** ✓✓ |

**Die Ausnahme** ✓✓: `rp-11880-…` ✓✓ trägt im Ortsfeld nur die **Nummer** ✓✓
**11880** ✓✓ und **kein** Feld `postcode` ✓✓ — das ist der **Klärfall** ✓✓, der nach
Vermerk **`pruefung`** ✓✓ **bleibt** ✓✓ ✓ ✓ Er wird **nicht** mitgezogen ✓✓ ✓ ✓

## Die Regel ✓ ✓

**Unverändert** ✓ ✓ — sie steht in `research/geo-entscheidung-20261119.md` ✓ ✓ ✓ ✓
**Präzisiert** ✓ ✓ hier ✓ ✓: der **Ort** ✓ ✓ steht im Feld `city` ✓ ✓, die
**Postleitzahl** ✓ ✓ im Feld `postcode` ✓ ✓ — **nicht** ✓ ✓ **beide** ✓ ✓ im Feld
`city` ✓ ✓ ✓ ✓

**Der Beleg** ✓ ✓ ist derselbe ✓ ✓ wie in der Runde vom Vormittag ✓ ✓: der
**Seed-Wert** ✓ ✓ des Feldes `postcode` ✓ ✓ desselben Dossiers ✓ ✓ **und** die
**Konvention** ✓ ✓ des Katalogs ✓ ✓ ✓ ✓

**Das Feld `postcode`** ✓ ✓ bleibt **unberührt** ✓ ✓ — es trägt **bereits** ✓✓ die
richtige Postleitzahl ✓ ✓ ✓ ✓

## Die Arbeit ✓ ✓

**Hunderteinsundachtzig** ✓✓ Dossiers ✓✓ tragen jetzt ✓✓ den **reinen** ✓✓ Ortsnamen
✓✓ im Feld `city` ✓✓ und **eine** ✓✓ datierte Korrektur-Sektion ✓✓ mit dem
**Dossiernamen** ✓✓ und der **Straße** ✓✓ ✓ ✓

**Keine** ✓✓ Koordinate ✓ ✓ wurde **angefasst** ✓ ✓ ✓ ✓ **Keine** ✓✓ Notiz der
Vorwellen ✓ ✓ wurde **überschrieben** ✓ ✓ ✓ ✓

## Die Gates ✓ ✓

| Gate | Exit |
|---|---|
| `wave_gate.py` ✓ ✓ | **0** ✓ ✓ |
| `check_pre_wave_notes.py` ✓ ✓ | **0** ✓ ✓ |
| `check_note_order.py` ✓ ✓ | **0** ✓ ✓ |
| `check_cert_balance.py` ✓ ✓ | **0** ✓ ✓ |
| `check_note_claims.py` ✓ ✓ | **0** ✓ ✓ |

## Der Stand in der Produktion ✓ ✓

**Noch** ✓✓ **nichts** ✓✓ ✓ ✓ Die Änderungen ✓✓ liegen im Arbeitsbaum ✓✓ und gehen
mit dem **Release dieser Runde** ✓✓ hinaus ✓✓ — **keine** ✓✓ Änderung an der
Produktion ✓✓ ✓ ✓

**Die Reihenfolge** ✓ ✓: diese Runde ✓ ✓ folgt **nach** ✓ ✓ den beiden Wellen ✓ ✓
(03:00 ✓ ✓ und 09:00 ✓ ✓) ✓ ✓ Sie darf **nicht** ✓ ✓ in einen Release einer Welle
✓ ✓ hineinlaufen ✓ ✓ ✓ ✓
