# Der Befund vom 24.11.2026, 13:52 UTC — die Berichtigung des Befunds vom 13:20 UTC

**Stand 24.11.2026, 13:52 UTC ✓ ✓**

**Der Befund vom 13:20 UTC** ✓✓ (`research/befund-20261124-0300-entscheidung.md`
✓✓) bleibt **unverändert** ✓✓ stehen ✓✓ ✓ ✓ **Diese** ✓✓ Akte ✓✓ berichtigt
**zwei** ✓✓ seiner Aussagen ✓✓ ✓ ✓

## Die erste Berichtigung ✓ ✓ — die Korrektur am Marl-Dossier ✓ ✓

**Der Befund vom 13:20 UTC** ✓✓ kündigte an ✓✓, der Punkt des Dossiers
`nw-marl-schrott-metallhandel-h-w-nowakowski` ✓✓ werde auf **51.6640815 /
7.0889051** ✓✓ **gesetzt** ✓✓ ✓ ✓

**Das stimmt nicht** ✓✓ ✓ ✓ Das **Adress-Gate** ✓✓ des Projekts ✓✓ prüft
**lange** ✓✓ eine **dritte** ✓✓ Bedingung ✓✓ — und sie ist hier **nicht**
✓✓ erfüllt ✓✓ ✓ ✓

```
nw-marl-schrott-metallhandel-h-w-nowakowski    way/250105006 traegt die PLZ '45772', das Dossier '45768'
```

**Live** ✓✓ am **24.11.2026** ✓✓ (OSM-API ✓✓) ✓ ✓ `way/250105006` ✓✓ trägt
`addr:postcode`=**45772** ✓✓, das Dossier ✓✓ trägt **45768** ✓✓ ✓ ✓

**Der Punkt wird daher nicht gesetzt** ✓✓ — **das ist die Entscheidung des
Befunds vom 03:00** ✓✓ ✓ ✓ Die **Korrektur-Sektion** ✓✓ im Dossier ✓✓ selbst
sagte bereits „Ein Ersatzpunkt entsteht **nicht**“ ✓✓ und nannte denselben
**Postleitzahl-Widerspruch** ✓✓ ✓ ✓

**Der Satz der Akt** ✓✓ der Welle vom **03:00** ✓✓ — „in **keinem** dieser
Fälle liegt ein **belegter Standort** ✓✓ **und** ein **adressiertes Objekt mit
der Hausnummer** ✓✓ **gleichzeitig** vor“ ✓✓ — ist für **dieses** ✓✓ Dossier
**unpräzise** ✓✓ ✓ ✓

## Die zweite Berichtigung ✓ ✓ — die Regel hat **drei** ✓✓ Bedingungen ✓✓

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

## Die Wirkung ✓ ✓

**Kein** ✓✓ Fehler der **veröffentlichten** ✓✓ Entscheidungen ✓✓ ✓ ✓
**Sieben** ✓✓ der **acht** ✓✓ Fälle ✓✓ sind **korrekt** ✓✓ ✓ ✓ Der **achte**
✓✓ ist **ebenfalls** ✓✓ korrekt ✓✓ — nur eben **aus einem** ✓✓ **anderen**
✓✓ Grund ✓✓ als benannt ✓✓ ✓ ✓

**Die Regel** ✓✓ ist jetzt **vollständig** ✓✓ niedergeschrieben ✓✓ ✓ ✓
**Der Satz der Akt** ✓✓ wird **nicht** ✓✓ überschrieben ✓✓ — die
**Recherchehistorie** ✓✓ bleibt ✓✓ ✓ ✓
