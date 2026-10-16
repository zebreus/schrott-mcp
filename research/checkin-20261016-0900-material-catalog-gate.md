# Enges Materialkatalog-Gate: 29 Einzelzuordnungen und Refreshzeit

Stand 16.10.2026, geprüfter HEAD `9c140e19e92d404aac2c384da6459ce387f79cbe`.
Abschlussabgleich nach Fortsetzung: HEAD `8df209969acbddfd6142ddc03d28d63f8b801fc8`;
seit dem geprüften Stand ausschließlich fünf Researchberichte hinzugekommen,
kein Unterschied an Code, Katalog, Fixtures oder Dossiers.

**PASS für das eng abgegrenzte fachliche 29-Material-/Repo-Consumergate;
HOLD für Gesamtfreigabe und noch ausstehende Owner-Katalogentscheidung.**
**Kein neuer fachlicher Katalog-/Producerblocker nachgewiesen.** Die unten einzeln
geprüften 29 Ergänzungen sind für ihren belegten Umfang passend getrennt.
`materials.updated_at` ist im tatsächlichen Seedpfad eine **Refreshzeit**, kein
Nachweis neuer Materialrecherche, Händlerannahme oder frischer Preise. Kein
geprüfter implementierter Repo-Consumer benutzt sie als fachliche Preis-Freshness.
Das ist eine begrenzte fachliche Empfehlung, **keine Gesamt-/Deployfreigabe**;
die ausdrückliche Katalog-/Zeitstempeldisposition bleibt beim Owner.

## Tatsächlich geprüft und Evidenzgrenze

`0900-upgrade-impact-review.md` vollständig gelesen. Alle 29 dort aufgeführten
Slugs gegen den echten `MATERIAL_CATALOG` (`crates/ingestion/src/pipeline.rs:20–524`)
und die gespeicherten 29 Insertzeilen aus der abgeschlossenen Probe verglichen:
Slug, Name, Kategorie, Einheit und Beschreibung jeweils identisch. Tatsächlich
83 eindeutige Katalogslugs, keine Dubletten. **Keine Probe wiederholt**, keine DB
geöffnet. Das ist ein Dateivergleich mit bestehender Evidenz, keine neue Entnahme.

Belegkürzel der Einzeltabelle:

* **M:** aktueller `handlers/mkm_metals/overview.rs:89–528`, registrierter typisierter
  Pfad in `traders/mod.rs:312–365`, echte Fixture
  `crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html`, feste
  `mkm-reviewed-mapping-oracle-20261011.json`. Alle **60 Artikel** einzeln nach
  Material **und Variante** gegen aktuellen Mappingcode verglichen: keine Abweichung.
  Für die hier betroffenen **17 Artikel / 16 Materialien** außerdem die vollständigen
  vorhandenen Primärdetailseiten `/tmp/opencode/mkm-detail-<ARTIKEL>.html` nach ihren
  materialrelevanten Detailtexten abgeglichen;
  alle 17 SHA256 stimmen mit `mkm-detail-audit-20261010.json` überein. Quell-URLs:
  `https://mkm-metals.de/metallankauf/produkt/<Deep-Link>` aus jeweiligem Mapping.
  Vorhandener unabhängiger Fachreview: `checkin-20261010-mkm-independent-review.md`,
  insbesondere vollständige Artikeltabelle, Fraktionsabgrenzungen und Re-Review
  zu Kerze/Zinn99; aktueller Owner-Abschluss in
  `checkin-20261011-mkm-owner-handoff.md:179–240` berücksichtigt F1–F6.
* **K:** aktueller `handlers/kernschrott.rs:44–85,154–354`, Primärfixture
  `crates/ingestion/tests/fixtures/kernschrott-elektro-20261008.html` von
  `https://www.kernschrott-recycling.de/elektro`. Alle echten Positionsnamen,
  Beschreibungen und Preise gelesen, mit Mapping und Katalog abgeglichen.
  Bestehender Fachreview `checkin-20261010-kernschrott-independent-review.md`,
  aktueller Follow-up und Volltabelle. Bekannte Sorten behalten Originalnamen als
  Varianten; Geräte/CPUs/ICs werden nicht zu Platinenpreisen zusammengelegt.
* **L:** `handlers/limekon.rs:50–69`, echte
  `fixtures/limekon-preisliste-20261014.html`, Quelle
  `https://limekon.de/pages/preisliste.php`; bestehender unabhängiger
  `handler-limekon-material-review-20261014.md:33–61`.

Alle Handlerpfade oben relativ zu `crates/ingestion/src/traders/`; alle genannten
Berichte unter `research/`. Quellen sind gespeicherte Betreiberprimärquellen,
keine unabhängigen Markt-/Registerbestätigungen. Keine Behauptung, die Listen
seien am 16.10. unverändert live. Für dieses Definitionsgate ergab sich keine
klärungsbedürftige neue Quellenkollision, daher kein neuer Liveabruf nötig.

## Alle 29 einzeln

Einheit ist die **Katalog-Zieleinheit**. N=nichteisen, E=elektronik, K=kabel,
I=eisen, P=kunststoff. „Passend“ bezieht sich auf genannte Fraktion/Varianten,
nicht auf bedingungsfreie Annahme sämtlicher denkbarer Gegenstände.

| # | Slug; Kategorie/Einheit; Katalogstelle `pipeline.rs` | Konkreter Producer-/Primärbeleg und fachliche Grenze |
|---|---|---|
| 1 | `ablenkeinheiten`; E, EUR/kg; 434–438 | **K** „Ablenkeinheiten“, eigene Zeile/Variante. Ausgebaute Bauteilfraktion, kein ganzer Motor. Quelle ohne Zusatzbeschreibung; Bildröhrenbezug ist technischer Katalogklassentext, keine zusätzliche Händlerzusage. |
| 2 | `aluminium-isoprofile`; N, EUR/kg; 37–38 | **M SW10000** „Alu Iso Profile Ausbau“: thermisch getrennte Profile, Kunststoffsteg darf bleiben; Glas/Beschläge entfernen. Nicht blanke Profile oder bloß farbiges Vollprofil. |
| 3 | `aluminium-profile-beschichtet`; N, EUR/kg; 35–36 | **M MKM-AL-01** „Alu Profile mit Farbe“: Lack/Pulver/Folie; eigene Fraktion gegenüber blanken und Iso-Profilen. Anhaftungen bleiben Bedingungen, keine Verallgemeinerung des Preises auf alle Profile. |
| 4 | `aluminium-spaene`; N, EUR/kg; 33–34 | **M SW10012** „Alu Späne Basis 5%“: Dreh-/Fräs-/Bohrspäne, Quelle nennt Legierung226 und Feuchte-/Ölbasis. Nicht Kupferspäne oder massive Aluteile; Basis bleibt im Rohgrad. |
| 5 | `blei-gemischt`; N, EUR/kg; 47–48 | **M SW10030** „Blei“ umfasst Rohr-/Walz-/Blechblei **und Auswuchtgewichte**. Ganze Quellenfraktion statt künstlicher Aufteilung auf `blei`/`blei-auswucht`; keine Akkus, kein Geschossblei, kein Blei/Zink-Gemisch. |
| 6 | `blei-geschoss`; N, EUR/kg; 49–50 | **M MKM-BL-01** „Geschossblei“: Kugelfang, Schrot, Bleiklumpen; Sand/Erde/Stahl/Nässe beeinflussen Bewertung. Nicht sauberes Walz-/Rohrblei. |
| 7 | `computer`; E, EUR/kg; 406–410 | **K** „Computer komplett unberaubt“, Beschreibung „Computer, Server“. Ganze Geräte mit Vollständigkeitsbedingung, keine einzelnen Mainboards oder daraus erfundenen Platinenpreise. |
| 8 | `cu-ms-kuehler`; N, EUR/kg; 29–30 | **M SW10017** „Kupfer/Messing Kühler ohne Eisen“: Kupfernetz/Messingkästen, eisenfrei/entleert. Nicht bestehende Alu-Cu-Kühlerfraktion, nicht reines Kupfer/Messing. |
| 9 | `elektrobauteile`; E, EUR/kg; 448–452 | **K** gleichnamige Zeile: „Schütze, Schalter, Sicherungsautomaten usw.“. Gemischte ausgebaute Bauteile, keine kompletten Geräte oder Leiterplatten. |
| 10 | `elektronikstecker`; E, EUR/kg; 392–396 | **K** Computerstecker und Flachbandkabelstecker, **getrennte Varianten**; Kabel unmittelbar abtrennen, Computerstecker vergoldet/ohne Ferritring. Keine Kabelquote und keine Übertragung der Vergoldungsbedingung auf alle Stecker. |
| 11 | `integrierte-schaltungen`; E, EUR/kg; 385–389 | **K** „IC's“, Beschreibung „Kunststoff und Keramik“. Lose ICs, weder ganze Platinen noch Zusammenlegung mit den separat geführten CPU-Varianten. |
| 12 | `kabel-alu-cu`; K, EUR/kg; 31–32 | **M SW10035** „Alu Kupfer Kabel“: Detail ausdrücklich Verbundkabel mit beiden Leitermetallen, Fremdanteile begrenzt. Nicht reine Alu-/Cu-Kabel und trotz Herkunft aus Kühlanlagen keine Kühlerfraktion. |
| 13 | `kabel-ohne-stecker`; K, EUR/kg; 322–326 | **K** „Stromkabel aller Art, Haushaltskabel, Installationskabel, Mischkabel usw.“, **Anfrage**. Unbekanntes Leitermetall bleibt unbekannt; kein unbelegtes Kupfer-/Aluminium-Mapping oder numerischer Kabelpreis. |
| 14 | `kupfer-granulat`; N, EUR/kg; 27–28 | **M MKM-CU-02** „Kupfergranulat“ aus Kabelaufbereitung/Trenntisch; Körner statt Draht. Kunststoff-/Alu-/Feuchteanteile Bedingungen. Nicht Millberry und kein isoliertes Kabel. |
| 15 | `kupfer-stanzabfaelle`; N, EUR/kg; 23–24 | **M MKM-CU-01** „Kupfer Kerze“, Variante `Kerze`: Gitter/Butzen/Randstreifen aus blankem Kupferband. **Kein Berry-Litzendraht**. Korrigierte quellengeschützte Aliasgrenze statt globalem Umbenennen anderer Berry-Angebote. |
| 16 | `laufwerke`; E, EUR/kg; 420–424 | **K** „Laufwerke, CD-ROM, Floppy usw.“; HDDs haben separate Festplattenzeile. Keine CDs als Geräte, keine losen Laufwerkplatinen als ganze Laufwerke. |
| 17 | `luefter`; E, EUR/kg; 455–459 | **K** „Lüfter“, echter veröffentlichter **0,00-EUR/kg-Preis** samt Gefahrstoffausschluss. Komplettes ausgebautes Bauteil, kein nackter Elektromotor; Nullpreis ist keine erfundene Nichtannahme. |
| 18 | `metallschrott-gemischt`; N, EUR/kg; 51–52 | **M MKM-SCH-03** „Metallschrott“: **überwiegender Buntmetallanteil**, Begutachtung/**Anfrage**. Keine sortenreine Metallquote und kein eisenbasierter `mischschrott`. EUR/kg erzeugt keinen Preis. |
| 19 | `neodym-magnete`; N, EUR/kg; 43–44 | **M MKM-NE-02** „Neodym-Magnete“, **Anfrage**; Detail NdFeB aus Antrieben/HDDs/Lautsprechern. Kein reines Neodymmetall und kein Preis aus dem Katalogdefault. |
| 20 | `netzteile`; E, EUR/kg; 413–417 | **K** drei getrennte Varianten: mit Kabel, ohne Kabel, Steckernetzteile. „Nur mit Kupferkern“ gehört zur letzten Variante, nicht pauschal zu allen. Keine Zusammenlegung mit kompletten Computern. |
| 21 | `nickel`; N, EUR/kg; 39–40 | **M MKM-NE-05 / MKM-NE-04**: Reinnickel ab99% versus Nickel60–99%, **unterschiedliche Varianten/Preise**. Nickelreiche Legierungen sind keine Edelstahlfraktion; keine bloß vernickelten Stahlteile ableiten. |
| 22 | `notebooks`; E, EUR/kg; 399–403 | **K** „Laptops, Notebooks“: defekt, **unberaubt, ohne Akku**. Ganze Geräte, nicht Laptopplatinen oder positive Akkuannahme. |
| 23 | `optische-datentraeger`; P, EUR/kg; 427–431 | **K** „CDs“: CD/DVD/CD-ROM **ohne Hüllen**. Datenträger, nicht Laufwerke/Platinen. `kunststoff` ist im Schema freier TEXT-Kategoriewert, keine verletzte SQL-Enum-Beschränkung. |
| 24 | `panzersicherungen`; E, EUR/kg; 441–445 | **K** eigene Zeile „Panzersicherungen“, keine Zusatzbeschreibung. Bauteilfraktion, kein reines Metall; NH ist technische Klassenbezeichnung des Katalogs, kein aus der Quelle ergänzter Gehalts-/Annahmebeleg. |
| 25 | `prozessoren`; E, EUR/kg; 378–382 | **K** zehn separat erhaltene CPU-Grade (Keramik/Goldcap/Kunststoff/Slot/Kupferkern und Pinanteil). Kühlkörper-/Anhaftungsbedingungen bleiben zugeordnet; weder IC-/Platinenpreis noch ein pauschaler CPU-Preis. |
| 26 | `schrott-eisen-ne-gemischt`; I, **EUR/t**; 53–54 | **M MKM-SCH-02** „Mischschrott“: ungetrennt Eisen/Stahl und NE, Bewertung nach NE-Anteil, **Anfrage**. Nicht bestehender reiner Eisenmischschrott, nicht überwiegend NE. **EUR/t ist Katalogkonvention, kein heute belegter MKM-Tonnenpreis.** |
| 27 | `wolfram`; N, EUR/kg; 41–42 | **M MKM-NE-06** „Wolfram ab99%“: metallische Elektroden/Heizleiter/Gewichte. Detail grenzt WC-Co-Hartmetall und HSS ausdrücklich ab; kein Wolframgehaltspreis auf diese anderen Stoffklassen. |
| 28 | `zinn-legierungen`; N, EUR/kg; 45–46 | **M SW10039** Zinn90% umfasst Feinlot/legierte Waren, **L** Zinnlegierungen ca.50% erhält eigenen Grad. Nicht Reinzinn, nicht allein Geschirr, nicht automatisch Sn-Pb-Lot aus Prozentzahl. Keine Vereinigung mit `zinn` oder `loetzinn`. |
| 29 | `zinn-rein-formgemischt`; N, EUR/kg; 21–22 | **M SW10040** Zinn99%: Bangka/Anoden/Barren/Blöcke **und hochreines Geschirr/Figuren**, Analyse entscheidet. Ganze nahezu unlegierte Quellenfraktion; enger `zinn` mit Geschirrausschluss und legiertes `zinn-geschirr` bleiben unverändert. Kein Aufteilen desselben Preises auf mehrere Teilmengen. |

Union-Fraktionen wie Altblei/Zinn können Teilmengen anderer Katalogbegriffe
überlappen; sie sind bewusst **keine globale disjunkte Stofftaxonomie**. Ihre
quellengebundene Variante und getrennte Material-ID verhindern die beanstandete
stille Gleichsetzung. Der Katalog erweitert keine alten Handler automatisch:
beispielsweise bleiben Frischs eigene Skips für Geräte/Nickel und dessen frühere
Kupferzuordnungen bestehen (`frisch_recycling.rs:171–181,230–250,269–270`).
`huth_aschaffenburg.rs:124` ist nur Labelübersetzung zu „drives“, kein zusätzlicher
Producer des Slugs `laufwerke`. Keine globale fachliche Harmonisierung behauptet.

## Einheiten, Preise und Annahmen am echten Anwendungspfad

* **28 EUR/kg, genau 1 EUR/t.** K prüft das echte Heading `Preis / kg`
  (`kernschrott.rs:189–193`) und schreibt Quellquotes EUR/kg; M validiert komplette
  Preiszellen/Artikelidentität und EUR/kg-Quellprojektion. L verlangt für den
  50%-Zinngrad konkret €/kg. Keine Stück-/Feingewichtsquote in diese 29 umetikettiert.
* M liest vor Writes für **alle** Artikel die wirkliche Katalogunit, auch bei
  Anfrage, und verwirft unsupported Units (`recording.rs:165–181`). Numerische
  Storeprojektion nutzt `normalize_unit`, einschließlich beider Grenzen
  (`recording.rs:203–230`; `traders/mod.rs:777–806`): kg→t **×1000**, t→kg **÷1000**.
  Artikel-Originalwerte, Staffeln und kg-Schwellen bleiben erhalten. Das beseitigt
  den früheren F6-Konflikt, ohne EUR/t zu EUR/kg umzudefinieren. Vorhandene echte
  HTML→Record-Regressionen für hypothetisch numerischen Eisen-/Mischschrott stehen
  in `recording.rs:901–996`; Unsupported-Unit-Gegenfälle in 837–884.
  F6-Ownerabschluss oben, **hier nicht erneut ausgeführt**.
* `seed_metadata` (`pipeline.rs:543–566`) schreibt Materialdefinitionen und
  Händlerstammdaten, **keine Preis-/Annahmezeilen**. `materials` hat keinen Händler-
  oder Annahmeschlüssel (`materials.rs:13–25`). Echte Annahme entsteht erst aus
  belegtem Outcome/Record; K aggregiert variantenspezifische und globale Bedingungen
  (`kernschrott.rs:345–353`), M erhält Anfrage-/Artikelbedingungen. Die Fallbackliste
  `traders/mod.rs:49–65` enthält keinen der 29 neuen Slugs. Kein implizites
  „alle Händler nehmen alle neuen Katalogmaterialien an“.

## `materials.updated_at`: Schreib- und Consumervertrag

| Tatsächlicher Pfad | Beleg und Bedeutung |
|---|---|
| Materialseed | `pipeline.rs:543–566`: jedes Mal alle Material-Upserts mit aktuellem `now`. `materials.rs:52–75`: Konflikt-Update setzt `updated_at` unbedingt, auch bei identischen Definitionen. **Refresh/Anwendung des statischen Katalogs**, keine quellenbasierte Neubewertung. Der dokumentierte 83er Timestamp-/Sequenzchurn ist damit erklärt, nicht als neue Freshness legitimiert. |
| Materialauflösung/Normierung | `materials.rs:79–100`: liest ID bzw. unit; keine Zeitstempelfilter. M validiert Einheiten/Identität, nicht Katalogalter. |
| Rust `list_materials` | `materials.rs:103–124` liefert den Zeitstempel unverändert als String, sortiert nach Kategorie/Name, ohne Freshnessberechnung. Repo-Aufrufsuche: außer Storetest nur MKM-Unitmutation im Test (`recording.rs:846–858`), **kein eigener MCP-list_materials-Endpunkt**. |
| Preisview/Current | `prices.rs:96–109`: joint Materialname/Kategorie, aber **nicht m.updated_at**; Zeitfelder kommen von `p.observed_at/published_at/valid_*`. Pointerordnung bleibt preisbezogen, nicht Katalogrefresh. |
| Zähler/Web | `public/mod.rs:363–373` zählt Materialien ohne Alter; `web/dashboard.rs:29–33`/`web/pages.rs:424` zeigen die Zahl. 83 bedeutet Katalogumfang, nicht 83 frische Preis-/Annahmebelege. |
| MCP/freies SQL | `mcp/tools.rs:17,24–36,139–155` dokumentiert die Spalte und transportiert frei gewählte SELECT-Ergebnisse ohne fachliche Freshnessauswertung. `query-worker/src/main.rs:60–68` → `PublicDb::open_read_only/query_sql`; `public/mod.rs:307–359` führt die validierte Query aus. **`SELECT updated_at FROM materials` bleibt öffentlich möglich**. |
| Verifier/sonstige Zeitfelder | Die vorhandenen Handler-/MKM-/Kern-Verifier prüfen Preis-/Annahme-`observed_at`, Ledger-/Stepzeiten; kein `materials.updated_at` als Preisalter. Trader-`updated_at` in Kontakt-/Wave-Verifiern ist ein anderer Vertrag. `vhm_hartmetall.rs` liest ein gleichnamiges Feld aus seiner Quell-API, nicht aus `materials`. |

Eigene Suche über Rust-/Scriptquellen nach `updated_at`, Material-SELECTs,
`MaterialRow/list_materials` und Freshness bestätigt diese Pfade. Das MCP-Schema
nennt für Preisalter ausdrücklich published_at bzw. observed_at; es definiert
**materials.updated_at nicht ausdrücklich als Refreshzeit**. Diese öffentliche
Semantik muss der Owner daher bewusst so disponieren. Kein implementierter
Fehlconsumer gefunden ist **kein Beweis**, dass externe SQL-Clients/Agenten das
Feld niemals falsch als fachliche Aktualität interpretieren. Solche Abfragen
werden technisch nicht verhindert. Keine Traffic-/Produktionsprüfung vorgenommen.

## Bindung, Schutz und Ergebnisgrenze

Frisch berechnete SHA256:

| Datei | SHA256 |
|---|---|
| `pipeline.rs` | `456ce9e28ffeb8e9fe9854ed565a94377bc497f803f87620327d789d9a87dfc9` |
| `store/src/public/materials.rs` | `0e4d22ba9d4e4957e1d4ec8e054152f08bc17229a1b436f9bf6e338f9b96bf6c` |
| MKM-Overviewfixture | `dd69560a9f34b226995869ca47eca09d7ffb9d3aa00383d12d337a6ea9c7de1d` |
| feste MKM-Mappingoracle | `bf44a6fefef1dd9a7c30d9bd0e98358356d156b69370756a7b5ce8703c3507e5` |
| Kernschrott-Primärfixture | `b1a313c9cfc2d3dea5845984f9b98190eac7ba07daedddc61252e466a36db532` |
| Limekon-Primärfixture | `11cc32f98229f9d495c9ca8bf076e80a24fe75386df81c6f95a31b63947475d2` |
| bestehendes 60-Detail-Hashmanifest `/tmp/opencode/mkm-detail-audit-20261010.json` | `8952052050a4a67a596bc231b087089a1b643e5a60add6b0efb154e18f12eb31` |

Die 143 Baseline-Dateien frisch hashgeprüft, keine Abweichung. Keine Code-/Dossier-
oder Evidenzänderung, kein Build, Testlauf, DB-Open, Datenkopieren oder erneuter
Ingest; nur Dateilesen/statische Abgleiche und dieser Bericht. Alte Reviewtests
sind zitierte bestehende Evidenz, keine eigenen neuen Ausführungen.

Nach der Fortsetzung vorhandenen Bericht vollständig gelesen und Arbeitsstand
geprüft, keine Doppelprobe begonnen. Die neu vorliegende
`checkin-20261016-0900-owner-disposition.md` vollständig berücksichtigt:
akzeptierte DDL-/Traderseed-/Schrott24-/FK-Entscheidungen werden **nicht** als neue
offene Sachblocker wieder aufgemacht. Unveränderte Code-/Fixturehashes und alle
29 Tabellenzeilen bleiben Grundlage des fachlichen PASS. Nicht belegt sind
unveränderte heutige Liveangebote oder das Verhalten beliebiger externer SQL-Clients.

**Ownerentscheidung:** Die 29 einzeln begründeten Definitionen samt bewussten
Union-/Varianten-/Anfragegrenzen und Materialzeit als Katalogrefresh sind fachlich
vertretbar. Kein Codefix erforderlich oder vorgenommen. Keine neue Reparatur aus
einer bloßen Zahl hergeleitet. Der Owner kann auf dieser Einzelprüfung die bislang
offene Katalog-/Refreshzeit-Disposition treffen; der Bericht trifft sie nicht an
seiner Stelle. Bereits akzeptierte Ausnahmen bleiben in ihrem dokumentierten Scope.
Weiter offen bleiben die gesonderten Writer-/Backup-/Rollback-/Scheduler-/Releasegates.
Kein automatisches PASS aus
„29“, keine Produktions-, Restore- oder Deployfreigabe.
