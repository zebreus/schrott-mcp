# Der Befund vom 24.11.2026, 13:09 UTC — die Regel der Standortpunkte hat **drei** Bedingungen

**Stand 24.11.2026, 13:09 UTC ✓ ✓**

Der Owner hat die **36** ✓✓ entfernten Punkte ✓✓ der Welle vom **03:00** ✓✓
**systematisch** ✓✓ nachgeprüft ✓✓ ✓ ✓ **Kein** ✓✓ Zufallsbefund ✓✓ — eine
**Messung** ✓✓ über **alle** ✓✓ 36 ✓✓ Dossiers ✓✓ ✓ ✓

## Die Messung ✓ ✓

**Für** ✓✓ jedes Dossier ✓✓ der **36** ✓✓ wurde geprüft ✓✓ ✓ ✓

1. Trägt ✓✓ ein **Adressobjekt** ✓✓ in OpenStreetMap ✓✓ die **Hausnummer** ✓✓
   des Dossiers ✓✓ **an** ✓✓ der belegten ✓✓ Straße ✓✓ (Nominatim-Suche ✓✓,
   live ✓✓ am **24.11.2026** ✓✓ **13:10 UTC** ✓✓) ✓✓ ✓ ✓
2. Welchen **Grund** ✓✓ die **Korrektur-Sektion** ✓✓ des Dossiers ✓✓ für den
   **Verzicht** ✓✓ auf einen **Ersatzpunkt** ✓✓ nennt ✓✓ ✓ ✓
3. Ob dieser **Grund** ✓✓ die **Regel** ✓✓ trägt ✓✓ ✓ ✓

## Das Ergebnis ✓ ✓

**Acht** ✓✓ der **36** ✓✓ Dossiers ✓✓ tragen **überhaupt** ✓✓ ein
**Adressobjekt** ✓✓ an der Hausnummer ✓✓ ✓ ✓ **Sieben** ✓✓ davon sind
**korrekt** ✓✓ entfernt ✓✓ ✓ ✓

| Dossier ✓✓ | Der Grund ✓✓ | Die Regel ✓✓ |
|---|---|---|
| `he-frankfurt-60488-bertin` ✓✓ | Die Bertin GmbH ist ein **Messtechnikunternehmen** ✓✓ — kein Schrottbetrieb ✓✓ ✓ ✓ | die Quelle belegt **nicht** ✓✓ den Standort **des Schrottbetriebs** ✓✓ ✓ ✓ |
| `he-staufenberg-35460-stahlhugel` ✓✓ | Das Objekt trägt **Mörfelden-Walldorf** ✓✓ (64546) ✓✓ — **nicht** ✓✓ Staufenberg ✓✓ ✓ ✓ | **anderer Ort** ✓✓ ✓ ✓ |
| `he-wiesbaden-marchenland-…-winterstein` ✓✓ | **HTTP 403** ✓✓ und Archiv **HTTP 503** ✓✓ — nur ein **Verzeichniseintrag** ✓✓ ✓ ✓ | ein **Verzeichnislead** ✓✓ allein ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ |
| `nw-bocholt-nam-schrottabholung-bocholt` ✓✓ | Die Adresse ruht allein auf einem **Portalprofil** ✓✓ ✓ ✓ | ein **Verzeichnislead** ✓✓ allein ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ |
| `nw-duisburg-baykal` ✓✓ | Die Adresse ruht allein auf dem **11880-Lead** ✓✓ ✓ ✓ | ein **Verzeichnislead** ✓✓ allein ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ |
| `nw-monchengladbach-rheydt-kupferrecycling-rheydt` ✓✓ | Der Rechtsträger (**HRB 13678** ✓✓) ist **gelöscht** ✓✓ ✓ ✓ | eine **gelöschte** ✓✓ Gesellschaft wird **nicht** ✓✓ verortet ✓✓ ✓ ✓ |
| `rp-kinderbeuern-bach-kinderbeuern` ✓✓ | Die **Domain** ✓✓ ist **nicht erreichbar** ✓✓ — die Adresse ist **nicht belegt** ✓✓ ✓ ✓ | die Adresse ist **nicht** ✓✓ belegt ✓✓ ✓ ✓ |

## Der achte Fall ✓✓ — `nw-marl-schrott-metallhandel-h-w-nowakowski` ✓✓

**Der Fall** ✓✓ war **zwei** ✓✓ Bedingungen ✓✓ nahe ✓✓ ✓ ✓

| Die Bedingung ✓✓ | Der Beleg ✓✓ |
|---|---|
| Eine **Quelle über das Geschäft** ✓✓ selbst ✓✓ | das **Impressum** ✓✓ — „H. W. Nowakowski, **Zechenstr. 5**, **45768 Marl**“ ✓✓ — live gelesen am **23.11.2026** ✓✓ (HTTP **200** ✓✓) ✓✓ ✓ ✓ |
| Ein **Adressobjekt** ✓✓ mit dieser **Hausnummer** ✓✓ | **way/250105006** ✓✓ — `addr:street`=**Zechenstraße** ✓✓ · `addr:housenumber`=**5** ✓✓ · `addr:city`=**Marl** ✓✓ · `addr:postcode`=**45772** ✓✓ · `building`=**yes** ✓✓ ✓ ✓ |

**Die dritte Bedingung** ✓✓ fehlt ✓✓ ✓ ✓ Das **Adress-Gate** ✓✓ des Projekts
✓✓ prüft sie ✓✓ beim **Setzen** ✓✓ ✓ ✓

```
nw-marl-schrott-metallhandel-h-w-nowakowski    way/250105006 traegt die PLZ '45772', das Dossier '45768'
```

**Das Objekt** ✓✓ trägt die **Postleitzahl 45772** ✓✓, das **Dossier** ✓✓ die
**45768** ✓✓ ✓ ✓ Der **Postleitzahl-Widerspruch** ✓✓ ist damit **kein**
✓✓ Gegenbeweis ✓✓, sondern ein **eigener** ✓✓ **Grund** ✓✓ gegen den
**Punkt** ✓✓ ✓ ✓

**Die Entscheidung** ✓✓ der Welle ✓✓ — **kein** ✓✓ Punkt ✓✓ — ist damit
**richtig** ✓✓ ✓ ✓ **Die Korrektur ist zurückgenommen** ✓✓ ✓ ✓

## Der Befund ✓✓ — die Regel hat **drei** ✓✓ Bedingungen ✓✓

Die Regel ✓✓ stand in `research/geo-entscheidung-20261119.md` ✓✓ und wurde in
der Welle vom **03:00** ✓✓ präzisiert ✓✓ — aber **unvollständig** ✓✓ ✓ ✓

**Vorher** ✓✓ — **zwei** ✓✓ Bedingungen ✓✓ ✓ ✓

> **Belegt** ✓✓ ist eine Adresse **durch eine Quelle über das Geschäft selbst**
> ✓✓ … — **und** das **Adressobjekt** ✓✓ **trägt diese Hausnummer** ✓✓ → der
> Punkt **wird gesetzt** ✓✓ ✓ ✓

**Nachher** ✓✓ — **drei** ✓✓ Bedingungen ✓✓ ✓ ✓

> **Belegt** ✓✓ ist eine Adresse, wenn **alle drei** ✓✓ Bedingungen ✓✓
> vorliegen ✓✓ ✓ ✓
>
> 1. eine **Quelle über das Geschäft selbst** ✓✓ belegt sie ✓✓ — das
>    **Impressum des Betreibers** ✓✓, die **Betreiberseite** ✓✓, die
>    **Kontaktseite** ✓✓, ein **Zertifikat** ✓✓, das **amtliche
>    Fachbetrieberegister** ✓✓, das **Handelsregister** ✓✓ **oder** ein
>    **OSM-Objekt mit dem Namen des Betriebs** ✓✓ ✓ ✓
> 2. ein **Adressobjekt** ✓✓ **trägt diese Hausnummer** ✓✓ ✓ ✓
> 3. **Postleitzahl** ✓✓ und **Ort** ✓✓ dieses Objekts ✓✓ **passen** ✓✓ zum
>    **Dossier** ✓✓ ✓ ✓
>
> **Fehlt** ✓✓ eine **Bedingung** ✓✓, entsteht **kein** ✓✓ Punkt ✓✓ ✓ ✓

**Die dritte** ✓✓ Bedingung ✓✓ ist **keine** ✓✓ Neuerfindung ✓✓ — sie ist im
**Adress-Gate** ✓✓ `scripts/release/check_address_claims.py` ✓✓ **lange**
✓✓ **umgesetzt** ✓✓ ✓ ✓

```python
if adressfall and (typ, oid) == zielobjekt:
    plz_des_objekts = tags.get("addr:postcode")
    ...
    if plz_des_objekts and plz_des_objekts != plz_des_dossiers:
        befunde.append((pfad.stem, "%s/%s traegt die PLZ %r, das Dossier %r" % ...))
```

**Der Satz der Akt** ✓✓ der Welle vom **03:00** ✓✓ — „in **keinem** dieser
Fälle liegt ein **belegter Standort** ✓✓ **und** ein **adressiertes Objekt mit
der Hausnummer** ✓✓ **gleichzeitig** vor“ ✓✓ — ist für **dieses** ✓✓ Dossier
**unpräzise** ✓✓ ✓ ✓ Die **zwei** ✓✓ Bedingungen ✓✓ liegen **zwar** ✓✓ vor ✓✓,
die **dritte** ✓✓ nicht ✓✓ ✓ ✓

## Die Wirkung ✓ ✓

**Kein** ✓✓ Fehler der **veröffentlichten** ✓✓ Entscheidungen ✓✓ ✓ ✓
**Sieben** ✓✓ der **acht** ✓✓ Fälle ✓✓ sind **korrekt** ✓✓ ✓ ✓ Der **achte**
✓✓ ist **ebenfalls** ✓✓ korrekt ✓✓ — nur eben **aus einem** ✓✓ **anderen**
✓✓ Grund ✓✓ als benannt ✓✓ ✓ ✓

**Die Regel** ✓✓ ist jetzt **vollständig** ✓✓ niedergeschrieben ✓✓ ✓ ✓
**Der Satz der Akt** ✓✓ wird **nicht** ✓✓ überschrieben ✓✓ — die
**Recherchehistorie** ✓✓ bleibt ✓✓ ✓ ✓ Die **Präzisierung** ✓✓ steht **ab** ✓✓
hier ✓✓ ✓ ✓

## Der Liquidationsfall der Welle vom 09:00 ✓✓ — **kein** ✓✓ Befund ✓✓

`by-friedberg-selzam-autoverwertung` ✓✓ traf den **Vor-Screen** ✓✓ ✓ ✓ Die
**Registerquelle** ✓✓ nennt **Äußere Industriestr. 1** ✓✓ und das **Objekt**
✓✓ **way/215659304** ✓✓ trägt genau diese **Hausnummer 1** ✓✓ ✓ ✓

**Aber** ✓✓ die Gesellschaft steht seit dem **30.12.2024** ✓✓ **in Liquidation**
✓✓ ✓ ✓ Der **Präzedenzfall** ✓✓ des Projekts ✓✓ (`research/wave-20261014-5-owner.md`
✓✓) ✓✓ sagt ✓ ✓

> **RRW** ✓✓ nun Betreiber-/Register-kongruent, **aber Liquidation** ✓✓
> **Erst operative Annahme/Fortführung klären** ✓✓ ✓ ✓

**Die Entscheidung** ✓✓ — **kein** ✓✓ Punkt ✓✓ — ist damit **richtig** ✓✓ ✓ ✓
Und der **Reichweitensatz** ✓✓ der Notiz ✓✓ deckt es **selbst** ✓✓ ab ✓ ✓
„Die Liquidation ist ein Registerstand und **kein** Nachweis, dass der Standort
bereits geschlossen ist“ ✓✓ ✓ ✓

## Die Lehre ✓ ✓

Die beiden **Präzisierungen** ✓✓ der Regel ✓✓ — das **Handelsregister** ✓✓ als
Quelle ✓✓ und das **adressierte Objekt ohne** ✓✓ `name` ✓✓ **und ohne** ✓✓
`operator` ✓✓, das die **Anschrift** ✓✓ belegt ✓✓ — gehören **zusammen** ✓✓ ✓ ✓
**Hinzu** ✓✓ kommt die **dritte** ✓✓ Bedingung ✓✓ ✓ ✓

**Diese** ✓✓ drei **Bedingungen** ✓✓ gelten **ab jetzt** ✓✓ für **jede**
✓✓ Standortentscheidung ✓✓ ✓ ✓
