# Unabhängiger adversarial Ownergate-Review — Welle 20261018-1800

## Disposition: keine Gesamtfreigabe

**Recherchegewinn ja; unveränderte Veröffentlichung aller neuen Scalare nein.** Insbesondere Ritter/AaM überschreiten mit Website-/Beschreibungsfills den dokumentierten Belegstandard. Hoffmann hat zwei zulässige Quellenursprünge, aber einen ungelösten Inhaberwiderspruch. Samuel erlaubt eine eng technische Erreichbarkeitsaussage, keine Identitätsauflösung. Die bewusst erhaltenen Altwerte sind nicht automatisch freigegeben.

Nur gelesen und diesen Bericht erstellt: keine Dossier-/Codeänderung, kein Cargo-/Rust-Build, keine DB geöffnet, kein Commit/Deployment. RRW/Hochscheider **nicht extern nachrecherchiert**; deren sachliche Freigabe liegt beim parallel prüfenden Owner. In den lokalen Format-/Backupvergleich aller 60 sind beide eingeschlossen.

## Grundlage und Beweisgrenzen

- Scope: `/tmp/opencode/wave-20261018-1800-scope.json`, 3 × 20 eindeutige Pfade; Backup `/root/Documents/wave-20261018-1800-prechange-backup`. Alle 60 Backup-SHA256 stimmen mit dem Scope überein.
- Alle drei Shardberichte und sämtliche neuen Dossierpassagen gegen Originale gelesen. Python-Leseaudit mit YAML-Parsing und Nachbildung der Timeline-Extraktion aus `crates/ingestion/build.rs:156–193`; **kein Ersatz für den Rust-Gesamttest**.
- Bindung des lokal auditierten 60-Dateien-Stands: SHA256 `b5c51a7b9d26c3a79dc2a84b9746486082ecbe212c1d4a9ee4d187b7f29dc333`, gebildet aus nach Pfad sortierten Zeilen `Pfad + NUL + SHA256(Dateibytes) + LF`. Spätere Owneränderungen erfordern erneuten Vergleich.
- Selbst frisch direkt gelesen: Ritter-Impressum, AaM-Start/Impressum, Hoffmann-Impressum/Northdata, Samuel-Start, Schröder-Preise/Impressum, Mr.-Schrott-Privatkunden. Die übrigen Quellen wurden hinsichtlich Beleglogik/konkreter Verlinkung beurteilt, **nicht alle extern erneut abgerufen**. Keine pauschale Link-Erreichbarkeitszusage. Ritter-PDF hier nicht erneut vollständig ausgewertet; seine Anerkennung als zweite README-Klasse scheitert bereits unabhängig vom Inhalt.
- Maßstab: `README.md:183–202` (zwei unabhängige zulässige Quellen; Betreiber-Ausnahme nur bei sämtlichen Kriterien), `README.md:176–182,237–240` (keine Phantomreste, Korrekturhistorie, Ownergate). Ein Fremdaudit ist wertvolle Zusatzinformation, aber keine zusätzliche Quellenklasse der dortigen Liste. Ein Indexauszug bei 403 ist kein gelesener Registervolltext.

## Priorisierte konkrete Befunde

### P1 — Ritter: gutes neues Indiz, keine formale Zweiquellenfreigabe

Pfad: `dossiers/bw/bw-schutterwald-ritter-recycling-julia-ritter.md:12–15,27,35`.

Das [frisch gelesene Impressum](https://www.ritter-container.de/impressum) nennt Julia Ritter, Am Ziegelplatz 17, dieselbe Telefonnummer und USt-ID, **keine Registerkennung**. Website + dort gehostetes ZN-Audit sind weder die vollständige autoritative Betreiber-Ausnahme noch zwei zulässige README-Quellenklassen. Das Audit kann Existenz/Standort plausibilisieren, den Standard aber nicht stillschweigend erweitern. „Existenz-/Co-Standortzweifel … überholt“ ist als geschlossenes Identitätsgate zu stark; `pruefung` wird im neuen Text nur noch mit fehlender Vergütung begründet.

**Disposition:** `website`, `website_status` und die neue faktische `description` **zurückstellen**, bis eine qualifizierte unabhängige Gewerbe-/Register-/Betreiber-Socialquelle gelesen ist, oder der Owner eine ausdrücklich dokumentierte Regeländerung entscheidet. Domain und Audit als klar qualifizierten Einzelbeleg in der Recherche behalten. Kein Ritter-Bau-Merge, keine Kontakt-/Zeit-/Service-/GEO-Freigabe. Die bisherige Telefonnummer ist Altbestand, nicht durch zwei Klassen neu verifiziert. Vergleichsmaßstab: AutoFritz wird in derselben Welle wegen fehlender Zweitquelle korrekt **nicht** als Websitefill ausgerollt.

### P1 — AaM: Einzelbeleg wird strukturiert, fragwürdiger Alt-Aktivstatus bleibt

Pfad: `dossiers/bw/bw-muggensturm-aam.md:12–15,27,35,39`.

[Impressum](https://www.aam-autorecycling.de/about/) direkt: Mehmet Akif Celik/Heinkelstr. 1, USt-ID, keine HR-Kennung. [Startseite](https://www.aam-autorecycling.de/) trägt Fahrzeugankauf, Teileverkauf und kostenlose Abholung **nach Termin innerhalb 20 km**. Diese Inhalte stimmen mit der neuen Recherche überein. Creditreform ist laut Notiz nur Index/403; damit fehlt die zweite qualifizierte Vollquelle. Die alte Aktivierung beruhte auf `schrottregister.pages.dev`, dessen angebliche amtliche Registerqualität jetzt selbst offengelassen wird.

**Disposition:** neue `website`/`website_status`-Freigabe und als Händlerprofil publizierte Servicebeschreibung **nicht genehmigt**. Im Recherchetext als Betreiber-Einzelbehauptung behalten; bei etwaiger öffentlicher Unsicherheitsbeschreibung ausdrücklich „nur Einzelbeleg; Identität/Aktivfreigabe nicht abschließend qualifiziert“ ergänzen. `status: aktiv` ist ein zusätzliches **offenes Ownergate**; ohne unabhängige Qualifikation ist `pruefung` die sichere Einstufung. Keine pauschale Gratisentsorgung oder aktuell anerkannte Demontage ableiten. Termin/Radius wurden sachgerecht erhalten.

### P1 — Hoffmann: zweite Klasse vorhanden, Gegenevidenz nicht wegzählen

Pfad: `dossiers/by/by-marktoberdorf-hoffmann.md:12–15,27,35`.

[Northdata HRA 11382](https://www.northdata.de/Harald%20Hoffmann%20Schrott%20%26%20Metall%20Recycling%20e%C2%B7%20K%C2%B7,%20Marktoberdorf/Amtsgericht%20Kempten%20HRA%2011382) frisch gelesen: Harald Hoffmann Schrott & Metall Recycling e. K., Marktoberdorf; Publikation 10.11.2023/Inhaber Harald Hoffmann. Keine ausgeschriebene Straße im sichtbaren Profil. [Impressum](https://www.hoffmann-mod.de/impressum.html) frisch gelesen: oben „Harald Hoffmann / Geschäftsführender Inhaber“, unten ausdrücklich „der Inhaber Stefan Kindler“. Keine Registerkennung im Impressum. Deshalb **keine autoritative Alleinquellen-Ausnahme**. Zwei Ursprünge stützen den Kandidaten, beseitigen aber nicht den Widerspruch. Ihn als bloßen Textbaustein zu diagnostizieren wäre eine unbelegte Erklärung.

**Disposition:** keine Namens-/Adress-/Kontakt-/Servicefreigabe; bisheriges `pruefung` richtig. URL ist ein gut gestützter Kandidat, aber die aktuelle definitive `website`-Zuordnung und vor allem `description` **nur nach expliziter Ownerentscheidung zur widersprüchlichen Domainbindung**, nicht automatisch freigeben. Die Beschreibung erwähnt nur EFB-Gültigkeit als offen und unterschlägt den wichtigeren Inhaber-/Kettenkonflikt; für eine vorsichtige Veröffentlichung diesen ausdrücklich nennen. Abgelaufenes PÜG-PDF nicht als Aktualitätsbeleg zählen. Kein Nachweis einer beliebigen Firmenverschmelzung erforderlich, wohl aber eine belastbare Zuordnung des knappen Seeds.

### P2 — Samuel: technischer Status vertretbar, B./R.- und Ortsbindung offen

Pfad: `dossiers/he/he-dietzenbach-b-samuel-schrottabholung.md:13–15,27,35`.

[Frischer Direktinhalt](https://schrottabholung-samuel.de/) bestätigt „Willkommen bei R. Samuel Schrotthändler“ und die Baustellenhülle. Die alte URL selbst wurde nicht neu gefüllt. **`website_status: aktiv` ist eng als technische Eigenschaft dieser bereits eingetragenen URL vertretbar**, nicht als Aktiv-/Identitätsbeleg für B. Samuel. Keine ausreichende Anbieterkennzeichnung, kein B.→R.-Nachweis. Die Seite schreibt selbst „63128 Dietzenbach“; diese Orts-/PLZ-Kombination ist ein zusätzlicher ungeklärter Adresskonflikt und wurde im Dossier nicht als solcher gekennzeichnet. Hier keine amtliche PLZ-Verifikation durchgeführt.

**Disposition:** technische Korrektur/Notiz akzeptierbar, `status: pruefung` und leere Kontakte erhalten. `description` auf reine Domainbeobachtung begrenzen („Seite nennt R. Samuel; Zuordnung zu B. Samuel und Orts-/Adressangabe ungeklärt“), nicht Dietzenbach als neu bestätigten Betreiberort formulieren. Keine Übertragung der Domainkontaktdaten oder der Selbstbehauptung weiterlaufenden Services. Keine Behauptung eines bestätigten Betreiberwechsels.

### P2 — neue descriptions sind Produktionsdaten, keine harmlose Notizablage

20 `description`-Fills einschließlich RRW/Hochscheider; README nennt sie direkte Seedskalare. Auch YAML-leer → gefüllt kann eine **nichtleere bereits gespeicherte DB-Beschreibung ersetzen**; der DB-Vorwert wurde hier nicht gelesen.

- `dossiers/bw/bw-eppingen-mr-schrott.md:15,27–37`: [FAQ](https://www.mr-schrott.de/privatkunden/) frisch bestätigt kostenpflichtige Abholung/Verrechnung, Anlieferung ohne Termin und Ausweis. Sehr guter semantischer Befund, Kundenrezension 0,20 EUR/kg korrekt ausgeschlossen. Trotzdem Betreiber-Einzelbeleg ohne belegte Ausnahme; `description` darf nicht mittels „laut Betreiber“ zur qualifiziert verifizierten Leistungsfreigabe werden. **Als unbestätigte Betreiberbehauptung kennzeichnen oder Servicefill zurückstellen**; alte Aktivfreigabe nicht neu bestätigt. Kontakt-/Samstagskonflikte bleiben korrekt offen.
- `dossiers/he/he-ortenberg-63683-r-s-reinhart-sela.md:15,27,35–37`: KG/Adresse als „registerseitig“ beschrieben, aber vollständige OHG→KG-Kette und heutige Annahme offen. **Als ausdrücklich registerseitigen Kandidaten akzeptierbar, keine rechtlich abschließend aufgelöste Seedidentität**. `status: aktiv` bleibt unbewiesener Altwert. Namenswechsel in `description` darf nicht als Umgehung des fehlenden Zweiquellen-Kettenbelegs erscheinen.
- `dossiers/rp/rp-ludwigshafen-hunke-herd.md:15,27,35–37`: Liquidation als Registeraussage und Trennung von Betriebsschließung sachgerecht. **Recherche-/Unsicherheitsbeschreibung akzeptierbar**, keine Zweiquellenfreigabe einer aktuellen Annahmestelle; Mannheimer Liquidationsanschrift nicht transplantieren.
- ESE/Sterz/Ruß beschreiben eigene Betreiber-/Indexaussagen, aber weiterhin offene Identität bzw. Zertifizierung. **Nur mit ausdrücklich quellengerechter Attribution als Rechercheprofil**, nicht als verifizierte Services lesen. Die anderen neuen Beschreibungen Yavuz/Rayer/Bruch/Dietz/Dechmann/Assaad/NSR/Felbinger markieren überwiegend Nichtwissen/Leads; als solche sinnvoll. Kein aktuell zertifiziertes oder zahlendes Angebot daraus ableiten.

## Nichtleere Overwrites und bewusst erhaltene Altwerte

Lokaler Vorher/Nachhervergleich: **54 Scalaränderungen = 43 Fills + 11 nichtleere Overwrites**, 29 Dossiers mit Frontmatteränderung, 31 unverändert. Shard 1 vollständig unverändertes Frontmatter. Keine `clear_fields` im 60er-Stand.

| Nichtleerer Overwrite | Disposition |
|---|---|
| Schrotthändler24, Boris Schmidt, Strohschein, Assaad: `aktiv → pruefung` (4) | **Akzeptieren** als Rücknahme unzureichender Freigabe, nicht als Schließung. Dafür ist kein positiver Zweiquellen-Schließungsbeleg nötig. Konflikte/Quellenzählfehler sind konkret dokumentiert. |
| AaM: `website_status unbekannt → aktiv` | Neue Domainbindung nicht hinreichend qualifiziert; **zurückstellen**, siehe P1. |
| Samuel: `website_status unbekannt → aktiv` | **Eng technisch akzeptierbar**, keine neue Händleridentität. |
| RRW: `city`, `website_status` (2) | **Owner prüft**, hier nur Format/Backup; keine externe Gegenfreigabe. |
| Hochscheider: `name`, `city`, `status` (3) | **Owner prüft**, hier nur Format/Backup; keine Doppelrecherche. |

Die acht Shard-2-Fills `website_status: unbekannt` sind erlaubte Enums und keine behaupteten Schließungen. Sie können trotzdem gespeicherte technische Statuswerte überschreiben; explizite Unsicherheitsentscheidung, nicht folgenlose Formatkosmetik. Dazu kommen Ritter/Hoffmanns bislang leere Webstatusfelder.

**Offene Legacy-Aktivgates:** `mv-gutzkow-17506-gors-recycling` (HRB 7114 ≠ Ankaufbetreiber HRB 5624), `bb-senftenberg-srf-zweigstelle-senftenberg-schrottrecyc` (kein lesbarer aktueller Zweigstellenbeleg), AaM, R&S und Mr. Schrott. Alle fünf bleiben `aktiv`; neue Unsicherheitsprosa neutralisiert dieses strukturierte Signal nicht. **Owner muss Beibehaltung belegen oder Status korrigieren**, keine stillschweigende Freigabe durch diese Welle. Marienberg ist der abweichende bereits ownergeprüfte Fall: vorhandene bedingte Annahme und Betreiber-GEO nicht ohne Gegenevidenz zurückstufen.

**Alt-Kontakte/GEO/Services:** Yavuz, NSR, Niepars, Dürrhennersdorf und weitere haben ausdrücklich nicht neu qualifizierte Altadresse/-kontakte. Erhaltene Historie ist richtig, aber kein Nachweis heutiger Richtigkeit. `docs/trader-seed.md:73–105` ist entscheidend: leere Servicefelder behalten gespeichertes `allowed:true`; `pruefung` zieht es nicht zurück. Fehlende Koordinaten behalten DB-GEO; nur eine explizite Marienberg-Paarung ist im 60er-Frontmatter vorhanden und unverändert. Eine Notiz „kein GEO freigegeben“ entfernt keinen öffentlichen Altpunkt. Umgekehrt können leere Kernadressen bei geändertem Seed DB-Adressen leeren. **Vor Rollout separater Owner-Abgleich der bestehenden DB-Werte; hier keine Produktionsdaten gelesen, keine unbekannten Werte erfunden.**

## Preise, Ketten und DeepLinks

- **Schröder:** `dossiers/hh/hh-barmbek-nord-gold-silber-ankauf-juwelier-schroder.md:35–36`. [Preise](https://www.barmbek-goldankauf.de/preise) selbst frisch gelesen: alle 16 Werte wie im Shardbericht, EUR/g, Händler-**Ankauf**, Orientierung/verbindlich erst nach Prüfung. Beim Review zeigt die Seite bereits **18.10.2026, 20:11** statt 17:17 bei unveränderten Werten. **Sichtbares Änderungsdatum nicht ungeprüft als echte Kursaktualisierung behandeln**; HTML-/Datumsmechanismus offen. [Impressum](https://www.barmbek-goldankauf.de/impressum) bestätigt Jennifer Balica/Hamburg und Niedersachsen-Text. Websitefill bewusst nicht vorgenommen: richtig. **Handler-/Stammdatenfreigabe bleibt gesperrt** bis unabhängiger Identitäts-/Domainbeleg; gleiches Layout/Inhaberin bei Stade keine zweite Quelle.
- **Leipzig:** zwei tatsächlich verschiedene Höfe nicht in einen Scalar oder Mittelpunkt pressen. Neue Primärinhalte heben alten inhaltsarmen 2017-Abruf auf; Sammelzeilendisposition offen, nicht erneut „kein heutiger Standortbeleg“ behaupten. PDF als Aktualität/Standortinhalt, nicht zweite Klasse: im Dossier korrekt abgegrenzt.
- **Peitzmann, WRG, Drebkau, FMT, Sasse:** neue Registerketten-/Löschungshinweise nicht mit aktuellem Annahmebetrieb verwechseln. Eintragungsdatum, Beschlussdatum, Spiegelstatus und GLEIF-Exportdatum wurden sinnvoll getrennt. Kein unbelegter Nachfolger-/Adresswechsel freigegeben. Northdata + derselbe Registertext anderswo bleiben ein Ursprung; GLEIF ist keine zusätzliche README-Klasse.
- **Besse/Böhm/Lüpcke/Felbinger/Kleinblittersdorf:** keine Udo→Lutz-, Lengenfeld→Auerbach-, Oldenburg→Wardenburg-, Alzenau→Mühlheim- oder Kleinblittersdorf→Kirkel-Transplantation: diese Zurückhaltung ist sachgerecht.
- Alle 71 neuen Notes enthalten konkrete HTTP(S)-Quellen; viele Impressums-/Kontakt-/Leistungs-/Register-/PDF-DeepLinks. Root-URLs bei Einseitern/Startseiten sind nicht automatisch mangelhaft. Suggest-, Such-/Anzeigenindex-, blockierte oder allgemeine Behördenlinks sind **Abrufprotokolle/Leads**, keine individuell gelesenen Betriebsbelege (besonders Schult, JK, Jordan/GADSYS). Ein URL-String allein beweist keine Inhaltsverifikation.

## Alle 60: Notes-/Historien-/Formatprüfung

**60/60** stabile Slugs, genau ein `## Timeline`, neues Datum und mindestens eine neue parserwirksame Bullet; **71 neue Bullets** insgesamt. Neue Bullets sämtlich `- [<Typ> 18.10.2026: …]`, geschlossen, mit URL; **kein U+2014-Emdash im neuen Notizinhalt**, kein reiner Emdash-Scalar als Platzhalter. U+2014 in `### Recherche …`-Überschriften wird nicht als Notizinhalt importiert. En-/Bindestriche in Zeiten sind keine Emdash-Platzhalter.

Alle YAML-Dateien lesbar; Typ-/Status-/Bundesland-/Websitezustands-Enums und URL-Präfixe gültig, JSON-Servicefelder soweit befüllt parsebar. 19 alte leere Websitezustände sind erlaubt, keine neuen Enumfehler. Alle 60 bisherigen **Timeline-Inhalte nach dem Marker** sind als unveränderter zusammenhängender Altblock enthalten. Der komplette alte Text *einschließlich* `## Timeline` ist wegen eingefügter neuer Abschnitte nicht zusammenhängend; die etwas ungenaue Shard-1-Formulierung bedeutet hier keinen historischen Inhaltsverlust. Scope-`protected`-Hashes vollständig unverändert.

**Formatrest nach README:180–182:** Die Statuskorrekturen Schrotthändler24/Boris Schmidt/Strohschein stehen als `[Recherche …]`, nicht als verlangtes `[Korrektur …; Quelle: …]`; ihre Gegenbeweise sind inhaltlich vorhanden. **Owner soll bei diesen drei Triagedispositionen den neuen Eintrag als Korrektur kennzeichnen**, ohne Altgeschichte umzuschreiben. Assaads Statuskorrektur ist bereits korrekt getaggt. Andere Research-/Preis-/Nachprüfungslabels sind parserkompatibel; kein genereller Parserfehler daraus ableiten.

Die folgende Vollständigkeitsmatrix nennt **Dossierpfad:neue Bulletzeilen**; „OK“ gilt nur für Notesformat/Historienerhalt, nicht für Sach-/Produktionsfreigabe.

| Shard | Dossierpfad:Zeilen | Format |
|---|---|---|
| 1 | `dossiers/mv/mv-gutzkow-17506-gors-recycling.md:35` | OK |
| 1 | `dossiers/mv/mv-niepars-wertstoff-u-recyclinghof-niepars.md:36` | OK |
| 1 | `dossiers/be/be-neukolln-psiorz-inh-marius-maciossek.md:33` | OK |
| 1 | `dossiers/st/st-zeitz-wrg-zeitz.md:36–37` | OK |
| 1 | `dossiers/mv/mv-boizenburg-19258-rast-recycling.md:35` | OK |
| 1 | `dossiers/sn/sn-lengenfeld-bohm-wertstoffzentrum.md:35–36` | OK |
| 1 | `dossiers/bb/bb-luckau-hsr-schrotthandel-filiale-lt-website.md:33` | OK |
| 1 | `dossiers/sn/sn-marienberg-scholz-recycling-betrieb-marienberg.md:39` | OK |
| 1 | `dossiers/sn/sn-werdau-euro-metall-korner.md:33` | OK |
| 1 | `dossiers/bb/bb-drebkau-schrott-altmetall-drebkau.md:35` | OK |
| 1 | `dossiers/be/be-friedrichshain-kreuzberg-alt-metallhandel-bernhard-schult.md:33` | OK |
| 1 | `dossiers/sn/sn-leipzig-scholz-recycling-leipzig-nord-ost-sud-we.md:37–38` | OK |
| 1 | `dossiers/sn/sn-reichenbach-08468-jordan-autoverwertung-ahmad-bataineh.md:35` | OK |
| 1 | `dossiers/st/st-ferchland-jk-schrotthandel-ferchland.md:35` | OK |
| 1 | `dossiers/sn/sn-zeithain-01619-auto-fritz-av.md:35–36` | OK |
| 1 | `dossiers/sn/sn-durrhennersdorf-02708-autodienst-durrhennersdorf-av.md:35` | OK |
| 1 | `dossiers/bb/bb-senftenberg-srf-zweigstelle-senftenberg-schrottrecyc.md:35` | OK |
| 1 | `dossiers/bb/bb-peitz-besse-udo-schrotthandel.md:33` | OK |
| 1 | `dossiers/bb/bb-falkensee-wagner-schrott-metallhandel.md:33` | OK |
| 1 | `dossiers/mv/mv-rostock-autoverschrottung-rostock.md:36–37` | OK |
| 2 | `dossiers/nw/nw-zulpich-klaus-schmitz-schrottabholung.md:35` | OK |
| 2 | `dossiers/nw/nw-korschenbroich-baer.md:37` | OK |
| 2 | `dossiers/nw/nw-nrw-weit-schrotthandler24.md:35–36` | Korrektur-Tag fehlt |
| 2 | `dossiers/nw/nw-oberhausen-pickave.md:33` | OK |
| 2 | `dossiers/sh/sh-kiel-schmidt-boris-ne-metallhandel-und-schrot.md:37` | Korrektur-Tag fehlt |
| 2 | `dossiers/nw/nw-troisdorf-pc-schrott-24.md:35` | OK |
| 2 | `dossiers/sh/sh-kasseedorf-aldenhoff-maximilian-schrott-u-stahlhand.md:35` | OK |
| 2 | `dossiers/nw/nw-paderborn-peitzmann.md:35` | OK |
| 2 | `dossiers/ni/ni-eschershausen-37632-rrw-ruckbau-recycling-weserbergland.md:38–39` | OK; Owner-Sachgate |
| 2 | `dossiers/nw/nw-detmold-schmidt-peter-tino.md:33` | OK |
| 2 | `dossiers/hh/hh-billbrook-alt-und-buntmetallhandel-dejan.md:35` | OK |
| 2 | `dossiers/hh/hh-barmbek-nord-gold-silber-ankauf-juwelier-schroder.md:35–36` | OK |
| 2 | `dossiers/hh/hh-altstadt-ferrous-metal-trading-hamburg-fmt.md:35` | OK |
| 2 | `dossiers/sh/sh-mittelangeln-altmetall-schrotthandel-dirk-strohschein.md:33` | Korrektur-Tag fehlt |
| 2 | `dossiers/nw/nw-aachen-ocher-schrottmannchen.md:33` | OK |
| 2 | `dossiers/ni/ni-wathlingen-horst-schmucker-schrotthandel.md:34` | OK |
| 2 | `dossiers/ni/ni-oldenburg-lupcke.md:33` | OK |
| 2 | `dossiers/nw/nw-rheda-wiedenbruck-sasse.md:35` | OK |
| 2 | `dossiers/nw/nw-bochum-wattenscheid-schrotthandler-plus.md:35` | OK |
| 2 | `dossiers/nw/nw-oberhausen-theissen.md:33` | OK |
| 3 | `dossiers/bw/bw-muggensturm-aam.md:35` | OK |
| 3 | `dossiers/bw/bw-schutterwald-ritter-recycling-julia-ritter.md:35` | OK |
| 3 | `dossiers/sl/sl-riegelsberg-e-s-e-dienstleistungen-elektro-system-en.md:33` | OK |
| 3 | `dossiers/rp/rp-ludwigshafen-yavuz.md:33` | OK |
| 3 | `dossiers/by/by-marktoberdorf-hoffmann.md:35` | OK |
| 3 | `dossiers/sl/sl-kleinblittersdorf-66271-kleinblittersdorfer-goldankauf.md:34` | OK |
| 3 | `dossiers/sl/sl-wallerfangen-ittersdorf-rayer-helmut-schrottentsorgung.md:33` | OK |
| 3 | `dossiers/he/he-michelstadt-64720-jochen-sterz.md:35` | OK |
| 3 | `dossiers/he/he-dietzenbach-b-samuel-schrottabholung.md:35` | OK |
| 3 | `dossiers/rp/rp-ludwigshafen-hunke-herd.md:35,37` | OK |
| 3 | `dossiers/by/by-sand-a-main-bruch-susanne.md:35` | OK |
| 3 | `dossiers/he/he-kassel-34123-dietz.md:35` | OK |
| 3 | `dossiers/rp/rp-koblenz-56070-dechmann-koblenz.md:37` | OK |
| 3 | `dossiers/by/by-knetzgau-russ.md:37,39` | OK |
| 3 | `dossiers/sl/sl-beckingen-schrotthandler-f-assaad.md:35` | OK |
| 3 | `dossiers/rp/rp-trier-54293-hochscheider-gondert.md:37` | OK; Owner-Sachgate |
| 3 | `dossiers/he/he-ortenberg-63683-r-s-reinhart-sela.md:35,37` | OK |
| 3 | `dossiers/bw/bw-eppingen-mr-schrott.md:37` | OK |
| 3 | `dossiers/he/he-griesheim-n-s-r-recycling.md:33` | OK |
| 3 | `dossiers/by/by-alzenau-felbinger.md:33` | OK |

## Ownerabschluss vor Veröffentlichung

1. Ritter/AaM-Fills nicht unverändert übernehmen; Hoffmann-Konflikt und beschreibungsseitige Identitätsgrenzen entscheiden. Samuel nur technisch, ohne B./R.-Auflösung freigeben.
2. Die fünf unqualifiziert erhaltenen Aktivgates und gespeicherte Service-/GEO-/Kontaktwerte ausdrücklich disponieren. Keine pauschale Altbestandslöschung; Marienbergs bereits geschlossenes Gate respektieren.
3. Drei neue Statuskorrektur-Notes formal korrigieren; historische Notes nicht nachträglich „säubern“. RRW/Hochscheider durch separate direkte Ownerprüfung abschließen.
4. Anschließend finalen Corpusstand neu prüfen/testen. Die Shard-1/-3-Testmeldungen sind kein in diesem Review frisch ausgeführter, hashgebundener Test des endgültigen Stands; Shard 1 dokumentiert selbst parallele Änderungen als Provenienzgrenze. Produktionsnachweis fehlt hier absichtlich.

**Gesamturteil: teilweise belastbare Recherche, offene materielle Ownergates; keine Blanket-Freigabe.**

## Nachtrag 18.10.2026 — begrenzte Ownerdisposition umgesetzt

Dieser Nachtrag dokumentiert den nachfolgenden **ausdrücklich autorisierten Implementierungsauftrag**; der obige Review bleibt als damaliger Befund erhalten. Seine Zeilenmatrix, Zählungen und sein Hash beziehen sich auf den damaligen Stand, nicht auf die jetzt ergänzten Korrekturen. Keine weitere Livequellenrecherche, kein Build/Testlauf mit Rust/Cargo, kein Commit, keine DB geöffnet oder geschrieben. Der bereits vorhandene Owner-Readonly-Export `/tmp/opencode/wave-20261018-1800-production-impact.json` wurde als Datei gelesen, nicht die Produktion abgefragt.

### Exakte Änderungen: 15 Wellen-Dossiers

| Dossierpfad | Umgesetzte Disposition |
|---|---|
| `dossiers/bw/bw-muggensturm-aam.md` | Neue `website` auf Originalblank `''`, `website_status: unbekannt`, `status: pruefung`; Beschreibung ausschließlich Einzelbeleg-/Identitäts-/Anerkennungsunsicherheit. Eigene neue Korrektur mit Betreiberimpressum, Creditreform-403 und ungeprüftem historischem GSA-Lead; kein Schließungsbefund. |
| `dossiers/bw/bw-schutterwald-ritter-recycling-julia-ritter.md` | Neue `website` auf Originalblank, `website_status: unbekannt`; `pruefung` bleibt. Beschreibung: Betreiber-Einzelbeleg ohne HR, Audit keine zweite Quellenklasse. Eigene Korrektur begrenzt den früher zu stark geschlossenen Existenz-/Co-Standortbefund; keine Ritter-Bau-Gleichsetzung. |
| `dossiers/by/by-marktoberdorf-hoffmann.md` | Neue `website` auf Originalblank, `website_status: unbekannt`, `pruefung` bleibt. Beschreibung nennt Harald Hoffmann versus Stefan Kindler ausdrücklich; eigene Korrektur mit Impressum/Northdata-Gegenevidenz, keine verifizierten Services. |
| `dossiers/he/he-dietzenbach-b-samuel-schrottabholung.md` | Technisches `website_status: aktiv` und alte Domain bleiben; Beschreibung nur Domainbeobachtung mit B./R.- und Orts-/PLZ-Konflikt. Eigene Korrektur trennt Erreichbarkeit von Identität/Aktivbetrieb. |
| `dossiers/bw/bw-eppingen-mr-schrott.md` | `status: aktiv → pruefung`; Beschreibung als nicht abschließend qualifizierte Betreiber-Einzelangaben. Eigene Korrektur mit Creditreform-403 sowie Adress-/Zeiten-/Mailkonflikten; keine Schließung, keine verifizierte Leistungsfreigabe. |
| `dossiers/mv/mv-gutzkow-17506-gors-recycling.md` | `status: aktiv → pruefung`; eigene Korrektur: HRB 7114 versus Ankaufbetreiber HRB 5624 und Baustellendomain; kein Schließungsbefund. Sonstige Scalars unverändert. |
| `dossiers/bb/bb-senftenberg-srf-zweigstelle-senftenberg-schrottrecyc.md` | `status: aktiv → pruefung`; eigene Korrektur mit ungelöster aktueller Zweigstellenidentität, Loading-/Transport-/Zertifikatsproblemen und Register-429, ausdrücklich kein Betriebsaufgabenbeweis. Sonstige Scalars unverändert. |
| `dossiers/he/he-ortenberg-63683-r-s-reinhart-sela.md` | `status: aktiv → pruefung`; Beschreibung ausdrücklich Registerkandidat/offene Seed-/OHG-/KG-Kette, Frontmatter-Name unverändert. Eigene Korrektur mit Registerprofil/Premiumsperre statt Namensfill oder heutiger Annahmebestätigung. |
| `dossiers/sl/sl-riegelsberg-e-s-e-dienstleistungen-elektro-system-en.md` | Beschreibung als nicht abschließend qualifizierte Website-/Socialspur; eigene Korrektur: keine verifizierte Erstbehandlungsanlage/Leistung, Webmaster nicht Inhaber. Sonstige Scalars unverändert. |
| `dossiers/he/he-michelstadt-64720-jochen-sterz.md` | Beschreibung ausdrücklich Index-/Partnerleads bei fehlendem Register-/Socialvolltext; eigene Korrektur, keine verifizierte Demontage-/Werkstattleistung oder Identitäts-/Typänderung. Sonstige Scalars unverändert. |
| `dossiers/by/by-knetzgau-russ.md` | Beschreibung als Betreiber-Einzelbeleg mit offener Seed-/Hammerschmidt-Kette; eigene Korrektur, keine Namensänderung oder verifizierte Leistungsfreigabe. Alte Domain und sonstige Scalars unverändert. |
| `dossiers/nw/nw-nrw-weit-schrotthandler24.md` | Ausschließlich `clear_fields: dropoff_json` ergänzt, `dropoff_json: ''` bleibt; eigene Korrektur zur fehlenden öffentlichen Hofannahme. Zusätzlich nur die neue Wellen-Statusnote formal von Recherche zu Korrektur/Quelle umgetaggt. |
| `dossiers/sh/sh-kiel-schmidt-boris-ne-metallhandel-und-schrot.md` | Ausschließlich `clear_fields: dropoff_json` ergänzt, `dropoff_json: ''` bleibt; eigene Korrektur: historischer Registersitz/Spiegel keine heutige Hofannahme. Zusätzlich nur die neue Wellen-Statusnote formal korrigiert. |
| `dossiers/sh/sh-mittelangeln-altmetall-schrotthandel-dirk-strohschein.md` | Ausschließlich `clear_fields: dropoff_json` ergänzt, `dropoff_json: ''` bleibt; eigene Korrektur: Abholwerbung/gesperrte Zweitquelle belegen keinen öffentlichen Hof. Zusätzlich nur die neue Wellen-Statusnote formal korrigiert. |
| `dossiers/sl/sl-beckingen-schrotthandler-f-assaad.md` | Ausschließlich `clear_fields: dropoff_json` ergänzt, `dropoff_json: ''` bleibt; eigene Korrektur zu F./M.- und Adresskonflikt/fehlender Hofannahme. Vorhandene Statuskorrektur bereits richtig getaggt, nicht verändert. |

**Genau vier gezielte Service-Clears:** Der Owner-Export enthält für Schrotthändler24, Boris Schmidt, Strohschein und Assaad jeweils gespeichertes `{"allowed":true}`. Jeweils eigene Korrektur mit Exportpfad/Slug und vorhandenen konkreten Quellen. Die Direktive soll beim späteren Seed **`{}` = unbekannt**, nicht `{"allowed":false}`, herstellen. Keine Annahmeverweigerung, keine Schließungsbehauptung. Hier nur Dossierdirektiven gesetzt, **noch kein DB-Clear ausgeführt oder als Produktionszustand bestätigt**.

### Frischer begrenzter Leseaudit

- Vor Implementierung eigener 60-Dateien-Snapshot: `/tmp/opencode/wave-1800-independent-before-fix.json`. Dagegen **genau 15 Dossiers geändert**, 45 bytegleich. Außer diesen Dateien nur dieser Berichtsnachtrag durch diesen Auftrag geändert.
- Frontmatterdelta zum Implementierungsstart: **5 Statuskorrekturen, 3 Website-Rückstellungen, 3 Websitezustands-Rückstellungen, 9 Beschreibungsänderungen und 4 Clear-Direktiven = 24 Scalaränderungen**. Kein anderer Frontmatterkey verändert; insbesondere keine Namens-, Adress-, Kontakt-, Pickup- oder GEOänderung.
- **15 eigene neue Korrektur-Bullets**, zusätzlich **3 ausschließlich formal umgetaggte neue Wellen-Bullets**. Neue Korrekturen datiert, geschlossen, mit `Quelle:` und vorhandenen Gegenbelegen; kein Emdash im neuen Notizinhalt. Die bisherigen Wellenbefunde sind damit ausdrücklich nachdisponiert, nicht als unverändert freigegebene aktuelle Scalarentscheidung zu lesen.
- **60/60 ursprüngliche Timeline-Altblöcke nach dem Marker gegenüber dem Original-Wellenbackup unverändert erhalten**, kein Altblock umgetaggt. Überblickstexte gegenüber Implementierungsstart unverändert. Alle 60 YAML-Dateien lesbar; die vier Clear-Direktiven benennen nur `dropoff_json` und stehen jeweils bei leerem Feld. Semantik anhand `docs/trader-seed.md:22–69,73–86` geprüft, kein Rust-Test behauptet.
- **RRW, Hochscheider und Marienberg bytegleich zum Implementierungsstart**; keine erneute Quellenrecherche oder Disposition dieser ausgeschlossenen Fälle.
- Neuer 60-Dateien-Manifesthash (gleiche Methode wie oben): `dfac72e11571957f98cf80e198ff8cd1c2a045de60dbe6aa48db729817c22b56`. Keine Bindung späterer paralleler Owneränderungen behauptet.

### Verbleibende Holds / keine Blanket-Freigabe

1. Die drei zurückgestellten Domains bleiben als recherchierte Kandidaten in der Timeline, nicht als qualifiziert zugeordnete Websitefills. Ritter/AaM benötigen qualifizierte unabhängige Identitätsbelege; Hoffmann die Auflösung des Harald-/Stefan-Kindler-Widerspruchs. Samuel benötigt B./R.-Identitäts- und Orts-/Adressklärung. Die fünf Legacy-Aktivgates sind jetzt konservativ `pruefung`, **nicht** materiell positiv verifiziert.
2. **Keine weiteren Services gelöscht.** Der vorhandene Owner-Export zeigt zusätzlich `dropoff_json allowed:true` bei `sh-kasseedorf-aldenhoff-maximilian-schrott-u-stahlhand` und `bw-eppingen-mr-schrott`. Diese bleiben auf ausdrückliche Begrenzung auf die vier autorisierten Clears **unverändert als separate Rest-Holds**; weder `pruefung` noch vorsichtige Beschreibung ziehen diese gespeicherten Zusagen zurück. Auch andere nicht im Export vertretene Altwerte sind dadurch nicht geprüft.
3. **Kontakte, übrige Services, GEO und übriger Scope bleiben unberührt.** Fehlende GEO-Scalars entfernen keine gespeicherten Punkte. Leere Websitefelder sind Rückstellung der neuen YAML-Fills, **keine zusätzliche Website-Clear-Direktive**; etwaiger gespeicherter URLbestand wird durch Blank allein nicht sicher entfernt. Keine pauschale Datenlöschung aus Quellenunsicherheit.
4. Leipzig-Sammelzeile, weitere Betreiber-/Namensketten, Schröder-Domainbindung/Preisdaterung, historische Kontakte/Annahmebedingungen und unqualifizierte GEO bleiben offene Ownergates aus dem Hauptreview. RRW/Hochscheider bleiben der separaten Ownerprüfung vorbehalten; Marienbergs geschlossenes Gate nicht wieder geöffnet.
5. Abschließender Gesamt-Corpus-/Seedtest, Neubau, Deployment und Kontrolle der vier effektiven DB-Servicewerte **stehen aus** und wurden in diesem begrenzten Auftrag ausdrücklich nicht ausgeführt. Diese konservativen Korrekturen sind keine Produktionsveröffentlichung.
