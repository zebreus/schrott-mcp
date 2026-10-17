# Wave 13-A – Ownerkorrekturen 17.10.2026, Check-in 21:00

## Ergebnis / Freigabegrenze

**Genau die 20 alten Wave-13-A-Dossiers sind für den gesonderten Owner-Seedtest und die scoped Integrationsprüfung redaktionell fertiggestellt.** Kein pauschaler Existenzbeweis, kein Abschluss offener Identitäts-/Annahmefragen und keine Produktionsfreigabe. Die vier zwingenden Korrekturen sind in den aktuellen Überblicke übernommen und in neuen datierten Timelinehinweisen gegenüber der alten Historie qualifiziert:

1. **WSH Lauchhammer:** konkreter neuer 11880-Lokal-Lead statt fortgeschriebener aktueller Trefferlosigkeit; kein Betreiber-/Betriebsbeweis, kein Telefon-Fill.
2. **RWR:** `DET2210V.HRB32106` ist die echte EUID unter falscher Überschrift „USt-IdNr.“, keine verstümmelte Kennung und keine bestätigte USt-ID. Keine Weißenthurm-/Bassenheim-Kette daraus abgeleitet.
3. **Eftekhari:** aktuelle Wiesbaden HRB 34887 und frühere Mainz HRB 48328 gehören laut gelesenem Registerprofil zur selben Gesellschaft; das Seedmatch Akazienweg 1 bleibt offen. Liquidation nicht auf ungeklärten Seed übertragen.
4. **Trapp:** unbelegtes „Vergütung nur …“ entfernt; Betreiber bewirbt Vergütung bei größeren Mengen und auch kleine Mengen je nach Materialart. Keine allgemeine Mindestmenge/Gratisgarantie.

Alle weiteren Reviewgrenzen wurden im jeweiligen Überblick und in genau einem neuen Abschnitt `### Ownerkorrektur 17.10.2026` klargestellt. Sämtliche früheren Timelineinhalte einschließlich 13.10. bleiben vollständig und bytegleich erhalten; neue Hinweise begrenzen widersprechende frühere Formulierungen ausdrücklich. Der alte Wellenbericht erhielt einen datierten Ownerhinweis, sein ursprünglicher Inhalt wurde nicht gelöscht.

### Was nicht geändert / ausgeführt wurde

- **Kein Frontmatter-Byte geändert**, keine neuen Fills. Die vom Owner akzeptierten zwei `aktiv → pruefung` (Gramet/Wolf) und elf `website_status: unbekannt` bleiben im vorherigen Arbeitsbaumstand erhalten.
- Keine Code-/Seed-/Cargo-/Build-/sonstigen Tests ausgeführt. Der weiter unten dokumentierte Sicherungs-/Textintegritätsabgleich ist kein bestandener Seedtest und ersetzt ihn nicht.
- Kein Commit, Push, Deployment oder Zugriff auf Produktionsdatenbanken; keine Koordinatenrecherche/-bearbeitung. Historische Standortfehlerhinweise nicht neu bestätigt.
- Keine anderen Dossiers geschrieben. Die Auswahl `/tmp/opencode/wave-20261017-2100-selection.json` wurde weder gelesen noch verändert. Neue 60er-Welle außerhalb dieses Auftrags nicht bearbeitet.
- `research/checkin-20261017-1700-wave13-a-review.md` blieb unverändert; hier keine nachträgliche Umschreibung des Review-Ergebnisses.
- **Keine erneuten Live-Webabrufe in diesem Korrekturschritt.** Quellenbasis sind ausschließlich die eigenen dokumentierten Direktabrufe/Suchgegenproben des 17:00-Reviews vom 17.10. sowie ausdrücklich historisch gekennzeichnete Altbefunde. Neue Timelinehinweise behaupten keine frische Wiederholung alter 02./08./13.10.-Abrufe.

## Sicherung vor der ersten Dossieränderung

Externes Verzeichnis:

`/root/Documents/schrott-wave13-a-20261017-2100-backup-zd5r07yh/`

- Enthält die **20 vollständigen Dossierdateien** in ursprünglicher relativer Pfadstruktur, kopiert vor der ersten Änderung.
- `SHA256SUMS`: 20 Vorher-SHA-256; jede Sicherungskopie unmittelbar bytehashgleich zum damaligen Original abgeglichen.
- `manifest.json`: Dateiliste/Vorherhashes, Git-HEAD zum Sicherungszeitpunkt und zusätzlich Vorhertext/-hash des alten Wellenberichts. Die Sicherung überschreibt keine älteren Backupverzeichnisse.
- `after-SHA256SUMS`: nach Abschluss erzeugtes externes Hashmanifest der exakt 22 im Repo geschriebenen Dateien, einschließlich dieser neuen Handoff-Datei. Deren eigener Nachherhash steht bewusst nur extern, um keine selbstreferenzielle Hashangabe zu erzeugen. Ein Vorherhash existiert für die neue Datei nicht.

## Exakte Schreibmenge und Vorher-/Nachher-SHA-256

**22 Repositorydateien:** die folgenden 20 Dossiers, `research/wave-20261013-a.md` und diese neue Datei `research/checkin-20261017-2100-wave13-a-corrections.md`. „Vorher“ meint den extern gesicherten uncommitteten Arbeitsbaum vor diesem Korrekturschritt, nicht HEAD. Die breite offene Arbeitsmenge ist kein erlaubter Commitumfang.

| Datei | SHA-256 vorher | SHA-256 nachher |
|---|---|---|
| `dossiers/bb/bb-eberswalde-seik-automobilrecycling.md` | `b323b5628ad451f382118bc523c59a74ca761b60ab425e4299d00c77fa9289ac` | `15028f5623abfd939f8d3bf366f4976d0e739916b8ec58cd2e708a5ec644262d` |
| `dossiers/bb/bb-gramzow-gramet-schrott-u-metallhandel.md` | `a95cfafa768301386b1d936166e705c0a752bfaa9c623cbd8593320c06ca5624` | `159e660cb5d927bddabd642638cfaaeb6346aed8dc959c4c87768315118304f8` |
| `dossiers/bb/bb-lauchhammer-radke-ulrich.md` | `eeb9b8d65cdd33c86b94c809cac4868f2ac33d1acd0e3e0120204b26688e44ae` | `8ee35bca67dd761c67c8d1818847f04642b6063dc0a43a9858d219458da59868` |
| `dossiers/bb/bb-lauchhammer-wsh-schrotthandel.md` | `2b817ed7994476bdf8ac2e417195d24f6eadf1f8798676967f7728e3ea98369b` | `b37c26ff2b211558532094007892f78fa08d8c2f1954bda74f6aa70f4bb01144` |
| `dossiers/bb/bb-storkow-utmar-koch-ralf.md` | `2549af81fc4367b6c85876cc0dfe34c96a686fc7eb1217d898c37143bf6d4b7b` | `014f7df05a9250a55c32ba8434fece547e73054f6c2ea60126d7e2eac068a398` |
| `dossiers/bb/bb-wandlitz-wolf-abschleppdienst.md` | `af9403c38b1f53628b2f810e1f7e25abb06d25dae24aa3a7bbce0ca599d9b7a4` | `433d6da4bae20d1eb65b13750e1fcacd0c102a48d722ef818cc5e32caba60b83` |
| `dossiers/bb/bb-zossen-autoverwertung-klucke.md` | `ef9b965eea8ff6d3f02164d3fad7624bb12c893e82e77e8c32682146c89032a9` | `ba44c194e8b9efde97b85f213b63bc3f905f52d3666e83e43c0d46c9771efe7e` |
| `dossiers/rp/rp-bechtolsheim-ayan-bechtolsheim.md` | `5b1bba8592864b0299388c4550e02ab974bb8653112840797a3a91cfe37f3ffe` | `2b37bc0f2d83defe96a4829699607ce3cb195ada9938693b66ea14e4307ea3aa` |
| `dossiers/rp/rp-bendorf-albert.md` | `ecafe5ba44744a6a872f511776befc82ce1ed92e0a17579db86ec238be465b50` | `1cf4115c4381d80836c4110054172545e8903abc668baa35cb3debeee780feee` |
| `dossiers/rp/rp-geinsheim-sudpfalz-metall-geinsheim.md` | `9d5ff19c88ac410156e067ddd2b0c28f3fc01ebad25f476270b4ca79a75964dc` | `ddef4e4b0653ecd1fc733be41cbf6a2534fd02159395dd6357cb833495fdc38d` |
| `dossiers/rp/rp-kinderbeuern-bach-kinderbeuern.md` | `8e6f09bf9697534fd7efb6cbf7a3d04364d59339f1c28b5808a5753a0373bd81` | `b7c59298d265bba9fe1c24ed8519bd5a94d346cff82c818b92420ace21cd68a3` |
| `dossiers/rp/rp-koblenz-56073-kreutz-schrotthandel.md` | `8c7f040737d826ff56f7e4a01bf43a23fb8fcb4ebe808548ba2a244527e4c8f0` | `1c0b411f495fbd4165a5bb4f8f34607329a8f67cd1f856deda96671bd8aed4e7` |
| `dossiers/rp/rp-koblenz-karthause-56075-schrotthandel-maik-alten.md` | `a92f80d1d1e77c82f277ec9a14a7c535db38b1e5059282711c71e63ae8a88104` | `c259a3b96c10ccdad22dcd883a69a82df9050134208e6ea2cf55e3f75a872c22` |
| `dossiers/rp/rp-landau-autoverwertung-demand-recycling.md` | `6d7f2c74abd9042c86018eddecfa389494ab6d091cf3c6aea88f2cf25be79791` | `cac0d4ac99623d92fed8bd0e1a04a2b4ba08b5aed285cf81954be1ae43be10e7` |
| `dossiers/rp/rp-limburgerhof-breyer.md` | `980e087adc10077a9d2625056abbe566058879b11c8bbd402c7bc5a77ee39b9b` | `d49f6b6a8e64b5eb14a3c7a4ad078e0980d72287afcb8182570b9e66e634eb3b` |
| `dossiers/rp/rp-mainz-55120-eftekhari.md` | `4bc932085f1819aa5a38521302e600dab62ae352c0aed5bdf9f68a7d128eb0e2` | `c74a8eac7dac60d4a89c3069a94e7e545108b897ed848fb7577ca675cac4cf3d` |
| `dossiers/rp/rp-trier-54294-wenner-david.md` | `5a47d21228d53c14f133fb3310c881e05d1494f6f010e8a420d7cfa6d14d0d7c` | `a32cbb72f966b71ec43ef388fb64b2108a29a766b12bba316620339f847412e0` |
| `dossiers/rp/rp-trier-m-trier-west.md` | `2aad37495484b49024dbd1930b53250755b6fb63fc83ada0838da33c6a834785` | `41d4b7ae78e2a37d5390c663757ac04f8e42d6810aea821f5975a775c623af6c` |
| `dossiers/rp/rp-weienthurm-rwr-weienthurm.md` | `afa45cb1a88e4b58f053d612ab390e79e68555684b28d262073fdd1282322d69` | `867b839c5cbcfbe448c5a9e3180ec7fc16e8a55fb0682d9f185a69d018c86529` |
| `dossiers/rp/rp-westheim-rohstoffe-trapp.md` | `042b999a128330767de47ae817f6c6de6d4aad14610a94527f57a2812c796bc4` | `c5653f3b5d7c3df831091b908281a7d95475d21cc06e0d01487cb8c6ea98f35d` |
| `research/wave-20261013-a.md` | `9d25fd01f3c256f47c3abed40d104c00b6446d759151965687489b0cb158d9da` | `aef04fd06f63b9c686796819df0fc62341719f6d2e1db0ec6d11da0e2d9a6584` |
| `research/checkin-20261017-2100-wave13-a-corrections.md` | — neu angelegt | extern in `after-SHA256SUMS` |

## Quellen, Korrektur und offene Fragen je Dossier

Die Abrufqualifikation in dieser Tabelle bezieht sich **auf den dokumentierten Review vom 17.10.2026**, nicht auf einen neuen Abruf um 21:00. Vollständige Quellen-/Historienabgrenzungen stehen zusätzlich im jeweiligen neuen Timelinehinweis und im unveränderten [17:00-Review](checkin-20261017-1700-wave13-a-review.md). Aggregatoren bleiben selbst bei Direktabruf Leads; der Direktabruf belegt nur deren Portaltext. NorthData ist der README-zugelassene Registerkanal, kein beglaubigter Auszug.

| Dossier (Kurzname) | Quellenbasis / übernommene Begrenzung | Restfrage für spätere Recherche |
|---|---|---|
| SEIK Eberswalde | [Onlinestreet](https://onlinestreet.de/poi/ZGVBZE9PbnViaC9lN05jSVZuVW0rUT09), [Stadtbranchenbuch](https://eberswalde.stadtbranchenbuch.com/1738216.html): Suchleads 1/1A. [SEIK-News](https://seik.de/news)/Impressum/Unternehmen im Review Transportfehler; Herten-Betriebsende nur historisch vom 03.10. | Eberswalder Rechtsträger, richtige Hausnummer, Annahme; keine Herten-Zuordnung oder Schließungsübertragung. |
| Gramet | [NorthData HRB 10759](https://www.northdata.de/Gramet%20GmbH,%20Gramzow/Amtsgericht%20Neuruppin%20HRB%2010759) gelesen; Löschung 15.07.2015, [1096675191](https://www.northdata.de/?id=1096675191) ist deren Profilverweis. 108975156 ist Eintragung, keine Löschung; Einzeltexte nicht nachgeprüft. | Keine Brücke zu Szymon Mach/Prenzlauer Str. 3; Löschung nicht übertragen, alte Aggregator-Fills nicht aufwerten. |
| Radke | [Das Örtliche](https://www.dasoertliche.de/Themen/Radke-Ulrich-Autoverwertung-Lauchhammer-Windm%C3%BChlenstr): Suchlead statt Portal-Roots als Einzelnachweis. Konkreter 11880-Profilverweis aus WSH-Seite gekennzeichnet, nicht als Radke-Direktabruf ausgegeben. | Betreiber/Aktivität/Ankauf und öffentliche Annahme; Status bereits am 07.10. integriert, keine neue Herabstufung. |
| WSH | [11880-Detailprofil](https://www.11880.com/branchenbuch/lauchhammer/060440459B113497545/wsh-schrotthandel.html) gelesen, Aktualisierung 04.10., Eintragsdaten 11.04.2026, lokale Adresse/Nummer nur Lead. [Kindsbach](https://wsh-recycling.de/) gelesen, keine Brücke. | Lokaler Betreiber und heutiger Betrieb/Annahme; Telefonnummer nicht gefüllt. Alte Trefferlosigkeit ausdrücklich überholt. |
| Utmar-Koch | [11880-Suche](https://www.11880.com/suche/altmaterialhandel/storkow), [Das Örtliche](https://www.dasoertliche.de/Themen/Robert--Koch--Str/Storkow-Mark.htm), [Social-Lead](https://www.facebook.com/p/Utmar-Koch-100007363059780/): Suchgegenproben, keine authentifizierte Betreiberquelle. | Namensformen Ralf Utmar-Koch/Koch Utmar auflösen, Geschäftstätigkeit und öffentliche Annahme bestätigen. |
| Wolf | [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/f711816b-b99a-4e05-8707-98a4f93ae581), [Das Örtliche](https://www.dasoertliche.de/Themen/Autoverwertung-Wolf-Abschleppdienst-Wandlitz-Basdorf-Z%C3%BChlsdorfer-Str), [Places](https://m.facebook.com/pages/Autoverwertung-Wolf-Abschleppdienst/133204130551519): Suchleads; Kategorien nicht stabil, Places nicht als betreibergeführt bewiesen. | Fahrzeugankauf/öffentliche Annahme, Betreiber und Aktivität; Herabstufung kein Schließungsentscheid. |
| Klucke | [Schrottplatz.org](https://www.schrottplatz.org/zossen/autoverwertung-autoverwertung-klucke-aYEPN2.html) gelesen: Nr. 2 eigenes Profil, HEBE Nr. 4 separate Nachbarkarte; alter Domain-410 nur historisch. | Vollständiger Betreiber, aktueller Standort und individueller Anerkennungsnachweis als Altfahrzeugbetrieb. |
| Ayan | [Börrstadt-Impressum](https://www.schrotthandel-ayan.de/impressum) gelesen, Steuerplatzhalter, kein Bechtolsheim-Nachweis. Historische Materialankaufseite nicht neu gelesen. | Bechtolsheim-Seedidentität/Filialbrücke; keine sichere Rechtsträgertrennung aus Ortsverschiedenheit. |
| Albert | [Albert/Cylex](https://web2.cylex.de/firma-home/albert-schrotthandel-15157667.html), [Jasari/Cylex](https://web2.cylex.de/firma-home/schrotthandel-jasari-15460171.html): Suchleads. [Vorstadt-Post](https://vorstadt-post.de/branchenbuch/bendorf/schrotthandel/albert-schrotthandel-32522/): Mail nicht direkt bestätigt. Bewertungsdifferenz kein alleiniger Abhängigkeitsbeweis. | Qualifizierte Betreiber-/Adressbelege; Jasari-Bezug offen, alte Ausnahme keine aktuelle Feldverifizierung. |
| Geinsheim | [AH-Impressum](https://www.ah-drucktechnik.de/impressum.html), [Rohrbach-Impressum](https://suedpfalzmetall-gmbh.de/impressum) gelesen. Fremder Adressnutzer schließt Mit-/Vornutzung nicht aus; Altdomain hier nicht erneut geprüft. | Geinsheimer Betreiber/Standort und mögliche Kette; Rohrbach weder zuordnen noch sichere rechtliche Trennung behaupten. |
| Bach | [Root](https://www.metallankauf-bach.de/)/[Impressum](https://www.metallankauf-bach.de/impressum) Transportfehler; [Bausendorf-Portal](https://www.klick-deinen-schrottplatz.de/schrottplatz-schrotthaendler-bausendorf-rheinland-pfalz-metallankauf-bach-ID232) Suchlead. Domain schon seit 06.10., kein 13.10.-Neufund. | Inhaber/Domainzuordnung, Kinderbeuern/Bausendorf, Annahme; keine dauerhafte Nichterreichbarkeit ableiten. |
| Kreutz | [Konkrete Anzeige](https://www.kleinanzeigen.de/s-anzeige/schrotthandel-schrotthaendler-kostenlose-abholung-metall-ankauf/3517253616-298-5424): Suchlead, Direktabruf 403. Alter Adresskonflikt und Portal-Mindestmengen bleiben unbestätigt. | Urheberschaft/Rechtliche Angaben; richtige Adresse; „geschlossen“ bedeutet nicht belegte Betriebsaufgabe. |
| Maik | [Werkenntdenbesten](https://www.werkenntdenbesten.de/schrotthandel/koblenz) Volltext gelesen; konkrete Profilverweise aus Übersicht gekennzeichnet, nicht als neue Einzelabrufe ausgegeben. | Betreiber-/Annahmenachweis; Lead-Karten Maik/MCR sind keine Identitäts- oder Umzugskette. |
| Demand | [Fahrzeugabgabe](https://www.autoverwertung-demand.de/fahrzeugabgabe), [Impressum](https://www.autoverwertung-demand.de/impressum) gelesen. Bereits am 08.10. integrierte Beträge, kein neuer Fund. HRA statt HRB allein entscheidet README-Ausnahme nicht. | Unabhängige Register-/Nachfolgekette, gültiges Zertifikat; Abmeldegebühr und Abholung außerhalb Landau/SÜW; keine Mindestprämiengarantie jedes Fahrzeugs. |
| Breyer | [Regionales Verzeichnis](https://www.containerdienst-regional.de/containerdienst-container-limburgerhof.html) mit Domainlead, keine Seedzuordnung. Begrenzte erfolglose Suche kein Quellenabschluss. | Qualifizierter Betreiber-/Gewerbe-/Registerbeleg für Limburgerhof; keine Übernahme durch Namensähnlichkeit. |
| Eftekhari | [Aktuelles NorthData-Profil HRB 34887](https://www.northdata.de/Eftekhari%20GmbH,%20Mainz-Kastel/Amtsgericht%20Wiesbaden%20HRB%2034887) gelesen, gleiche Gesellschaft mit früherer HRB 48328, Liquidation 27.09.2024. | Seedmatch Akazienweg 1/55120; keine sichere Filial-/Betriebszuordnung und kein Schließungsstatus für den Seed. |
| Wenner | [Facebook David Wenner](https://www.facebook.com/david.wenner.7503/): Suchselbstdarstellung, Direktabruf ohne Text; [Telefonbuchlead](https://www.dasoertliche.de/Themen/Wenner-David-Schrott-Metallrecycling-Trier-S%C3%BCd-Im-Schammat). Kleinanzeigen-Root kein reproduzierbarer Einzelinseratbeleg. | Authentizität/Aktualität des Social-Kanals, David/Dennis-Kette und richtige Annahmeadresse. |
| Trier-West | Dossier-/Git-Importtext als Primärbeleg des Datenfehlers, kein externer Existenzbeleg. Exakte Suche ohne qualifizierten Treffer; Theo-Steil-Anschrift im Review nicht neu geprüft. | Identität und fehlerhafter strukturierter Straßenwert als gesonderter Owner-Klärfall; hier absichtlich kein Frontmatter-Fix. |
| RWR | [Impressum](https://www.rwr-recycling.de/impressum/), [NorthData](https://www.northdata.de/RWR%20Recycling%20GmbH,%20Bassenheim/Amtsgericht%20Koblenz%20HRB%2032106), [Metallhandel](https://www.rwr-recycling.de/metallhandel/) gelesen. EUID sachlich korrigiert, Bassenheim-Registerabsicherung. | Weißenthurm-/Bassenheim-/Einzelunternehmen-/GmbH-Kette; keine Logistikgratiszusage aus kostenlosem Angebot. |
| Trapp | [Impressum](https://rohstoffe-trapp.de/impressum.html), [Startseite](https://rohstoffe-trapp.de/), [Ankaufseite](https://rohstoffe-trapp.de/schrott-metallankauf.html) gelesen. Startseiten-Root ist hier der tatsächliche zitierte Werbetext, nicht ein Ersatz für fehlenden Deep-Link. | Unabhängige Identitätsabsicherung, Art der Impressumsadresse, einzelfallabhängige Vergütung/Logistik; kein pauschales Mengen- oder Gratisversprechen. |

## Inhaltsintegrität und Owner-Handoff

Read-only-Abgleich gegen die externe Sicherung (keine Anwendungstests):

- 20/20 Sicherungskopien mit Vorher-SHA abgeglichen.
- 20/20 Frontmatter und gesamter Dateipräfix bis `## Überblick` bytegleich zum gesicherten Arbeitsbaum.
- 20/20 bisherige Timelineblöcke bytegleich erhalten; vor jedem Altblock genau ein neuer 17.10.-Ownerhinweis. Keine alte 13.10.- oder frühere Historie gelöscht/umgeschrieben.
- Ursprünglicher Wellenbericht nach dem eingeschobenen Hinweis vollständig unverändert erhalten.
- Nachherhashes der 21 bestehenden Dateien gegen obige Tabelle abgeglichen; Hash der neuen Handoff-Datei extern in `after-SHA256SUMS`.

Nächster Schritt ausschließlich beim Owner: exakte 22-Dateien-Menge gegen diese Sicherung/Hashliste und den bestehenden breiteren HEAD-Diff prüfen, anschließend den ausdrücklich angeforderten Seedtest selbst ausführen und scoped Integration entscheiden. Historische Kontaktdaten-Ausnahmen bleiben Restfragen, keine stillschweigende Freigabe. Die redaktionelle Fertigstellung verlangt keinen pauschalen Existenzbeweis und hebt offene Recherchefälle nicht auf.
