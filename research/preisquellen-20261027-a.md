# Preisquellen der großen Schrottbetreiber: wo und in welchem Format Preise veröffentlicht werden (Stand 27.10.2026)

Recherche-Notiz, **read-only**. Abrufdatum aller Live-Abrufe: **27.10.2026** (17:15–17:25 UTC).
Kein Dossier geändert, keine Produktionsdatenbank geschrieben, kein Commit, kein Push.

## Auftrag und Abgrenzung

Der Auftrag war **nicht** „ist die Webseite erreichbar?", sondern: **wo und in welchem Format
veröffentlichen große Betreiber Preise?** Ausgangspunkt ist der Befund aus
[research/handler-backlog-20261027.md](handler-backlog-20261027.md): Für 1.636 Händler mit aktiver
Website gibt es keinen eigenen Preis-Handler, und die Stichprobe fand bei 22 Domains nur **eine**
erkennbare €/kg-Angabe. Die dortige Hypothese war: *der Hebel ist die Preisquelle, nicht der
Handler.*

Diese Notiz prüft die Hypothese an den drei größten Gruppen und liefert je Betreiber:
(a) welche Preisquellen es gibt und wo, (b) ob sie **ohne Anmeldung** maschinenlesbar sind,
(c) welche Formate, (d) was daraus realistisch folgt, (e) was zu klären wäre, wenn eine
B2B-Quelle existiert.

**Methodische Grenzen (ausdrücklich):**

- Alle Abrufe mit klarem User-Agent (nachgebildeter Chrome 128 Desktop). Bei TSR wurden zwei
  Header-Sätze getestet (minimal / vollständige Browser-Header), um die WAF-Sperre von der
  Challenge-Sperre zu trennen.
- **Keine Bot-Challenge gelöst, keine Anmeldung versucht, kein Login-Bereich gescraped.**
  Login-Seiten wurden nur auf Vorhandensein, Zweck und Betreiber geprüft.
- Für TSR wurde die URL-Struktur aus der **Wayback Machine** rekonstruiert, weil die
  Live-Seiten hinter der Challenge liegen. Das sind archivierte Seiten, keine Umgehung.
- Die Produktionsdatenbank wurde ausschließlich read-only abgefragt
  (`file:...?mode=ro` + `PRAGMA query_only=ON`) und nur für Bestandszahlen genutzt.
- **Nicht** geprüft: LME-Datenprodukt (403 in diesem Abruf), kostenpflichtige
  Marktplatzplattformen, Preislisten hinter Anmeldung, Preislisten in App-Stores.

## Mengengerüst (read-only aus `public.db`, 27.10.2026)

Zählkriterium: `status='aktiv' AND website_status='aktiv'`, Gruppierung nach Host.

| Betreiber | Zeilen in Produktion | Anmerkung |
|---|---|---|
| `scholz-recycling.com` | **28** | 48 Zeilen inkl. `pruefung`/`geschlossen` |
| ALBA-Gruppe gesamt | **22** | davon `metall.alba.info` 14, `nisa.alba.info` 3, `www.alba.info` 2, `nord.`/`berlin.`/`sachsen.alba.info` je 1 |
| `tsr-recycling.de` | **16** | plus `tsr.eu` 3 (Konzernzentrale Lünen, MHR Ratingen, TSR Mülheim) |

Die Zahlen des Backlog-Berichts (33 / 21 / 16) sind mit dieser Zählweise reproduzierbar
(Abweichung ±1–5 je nach Zählregel, TSR deckt sich exakt). Für die Preisfrage ist die
Zeilenzahl zweitrangig: **alle Zeilen einer Gruppe teilen sich dieselbe Preisquelle.**

---

# 1. Schrott Recycling GmbH — scholz-recycling.com

## (a) Welche Preisquellen es gibt und wo

**Es gibt keine.** Die Soll-Preise sind im HTML nicht vorhanden — und das ist kein
Stichproben-, sondern ein Struktur-Befund:

| Prüfung | Ergebnis |
|---|---|
| `robots.txt` | HTTP 200, 125 B; verweist auf `https://www.scholz-recycling.com/wp-sitemap.xml` |
| `/sitemap.xml` | HTTP 301 → `/wp-sitemap.xml` (WordPress-Sitemap-**Index**, 6 Untersitemaps) |
| alle Untersitemaps | **273 URLs** gesamt (39 Beiträge, 20 Seiten, ~205 Standorte, Taxonomien, Autoren) — **keine Preis-URL** |
| `/wp-json/wp/v2/pages` | HTTP 200, **JSON**, 20 Seiten — **keine** mit Preis-Bezug |
| `/wp-json/wp/v2/posts` | HTTP 200, **JSON**, 39 Beiträge — **keine** mit Preis-Bezug |
| `/wp-json/wp/v2/types` | HTTP 200, JSON — **kein** Custom-Post-Type für Preise |
| 20 Preis-Pfade (`/preise/`, `/metallpreise/`, `/preisliste/`, `/schrottboerse/`, `/tagespreise/`, `/aktuelle-preise/`, `/preise/metall/`, `/schrottpreise/`, `/de/preise/`, `/de/metallpreise/`, `/de/preisliste/`, `/services/preise/`, `/recycling/preise/`, `/geschaeftsbereiche/preise/`, `/portal/`, `/b2b/`, `/gewerbekunden/`, `/kundenportal/`) | **alle HTTP 500**, 54.863 B, Title „Seite nicht gefunden – Scholz Recycling" → Soft-404 |
| `/login/`, `/wp-login.php`, `/wp-admin/` | HTTP 200, 9.846 B, Title „Anmelden – WordPress" → **WordPress-Admin-Login, kein Kundenportal** |

> **Wichtig für Handler-Logik:** Die Site antwortet auf nicht existierende Pfade mit
> **HTTP 500**, nicht 404. Ein Handler darf 500 nicht als „Seite vorhanden, Abruf defekt"
> deuten, sondern muss den Title „Seite nicht gefunden" als Negativbefund werten.

**Die Preisquelle ist benannt — aber sie ist ein Index, keine Liste.** Auf `/services/`
(Industriekunden-Abschnitt) steht wörtlich:

> „Die Abrechnung erfolgt nach Qualität bzw. Klassifizierung und Gewicht – transparent und fair
> auf Basis der **tagesaktuellen Marktpreise**."

und:

> „Die Abrechnung erfolgt transparent und fair auf Basis der **tagesaktuellen Marktpreise**, die
> mithilfe der branchenüblichen **Preisindizes wie z.B. BDSV oder LME** ermittelt werden."

Scholz nennt also selbst **BDSV und LME** als die Quelle, aus der seine Tagespreise ermittelt
werden. Auf der Einkaufsseite (Selbstanlieferer) steht dieselbe Aussage ohne Index-Nennung.
Damit ist die Frage „wo veröffentlicht Scholz Preise?" beantwortet: **nicht auf der eigenen
Website, sondern indirekt über Marktindizes, die Scholz zur Preisermittlung heranzieht.**

### Unterseiten-Inhalte (wie beauftragt geprüft)

| Seite | Inhalt | Preise? |
|---|---|---|
| `/geschaeftsbereiche/` | Geschäftsfelder; Verkauf deklarierter Stahlprodukte (Coils, Bleche, Rund-/Vierkantknüppel, Träger), „Bindeglied zwischen Stahlerzeugern und Stahlverbrauchern"; je Abschnitt ein „Kontakt"-Button | nein |
| `/recycling/` | Altfahrzeugrecycling, SRW metalfloat GmbH Espenhain, Shredderrückstände, Elektroaltgeräte | nein |
| `/services/` | Anlieferungsablauf (Erstverwiegung → Begutachtung → Ausladung → Abrechnung an Kasse), Ausschlussliste, Pflichtunterlagen; die Index-Aussage oben | nein (narrativ) |
| `/veroeffentlichungen/cbcr-reporting/` | **CbCR = Country-by-Country Reporting** (Steuertransparenz nach Richtlinie (EU) 2022/591): „konsolidierte Umsatzerlöse von über 750 Millionen Euro … verpflichtet, ein Country-by-Country Reporting (CbCR) zu veröffentlichen" | **nein — CbCR ist ein Umsatz-/Länderbericht, kein Preisbericht** |
| `/veroeffentlichungen/` | liefert inhaltlich dieselbe Seite (gleicher Title, gleiche 81.948 B) — die Sektion rendert die CbCR-Unterseite | nein |
| `/standorte/` | Filialfinder (PLZ/Ort) | nein |
| `/standorte-json-nicht-loeschen/` | **öffentlicher JSON-Standortfeed**, 184 Datensätze | nein — siehe unten |
| `/zertifikate/` | Zertifikatsammlung, u. a. `UeZEFB_MAR.pdf` | nein |
| `/kontakt/`, `/agb/`, `/impressum/`, `/karriere/`, `/news/`, `/unternehmen/`, `/sicherheitshinweise/`, `/datenschutz/` | Kontakt, Bedingungen, Rechtliches, News | nein |

### Filialfinder mit Standortpreisen?

**Nein.** Der Standortfeed `/standorte-json-nicht-loeschen/` ist maschinenlesbar (JSON, 184
Datensätze) und trägt **kein Preis-Feld**. Die vollständige Feldliste eines Datensatzes:

```
Zertifikate, Name, Unternehmen, lat, lng, strasse, hausnummer, plz, ort, land,
telefon, fax, email, webseite,
Montag..Sonntag (deutsch) + monday..saturday (englisch), Sonderoeffnungszeiten, special_times, featured
```

Kein `preis`, kein `price`, kein `euro`, kein `kurs` — über alle 184 Datensätze. Der
Standortfeed ist also für **Adress-/Öffnungszeiten-Daten** gut, für Preise nicht.

### RECY-ONLINE: das B2B-Portal

Auf **jeder** Seite steht „**RECY-ONLINE**" als Hauptnavigationspunkt. Der Linkzielpfad ist
`https://recy-online.scholz-group.com/scripts/cgiip.exe/WService=wsROnlineOne/recyonline.p`.

- `https://recy-online.scholz-group.com/` → HTTP 200, 1.109 B, IIS/ASP.NET, `Last-Modified:
  2020-01-02`. Inhalt: „RECY Online ist aufgrund einer Softwareumstellung über diese Seite nicht
  mehr verfügbar. Neue RECY Online steht Ihnen zur Verfügung über
  `https://recy-online.scholz-group.com:4437/RecyOnline`." — mit einem **namentlich benannten
  Ansprechpartner** (Alexander Matscheko, +49 7365/84-134, alexander.matscheko@scholz-recycling.de).
- `https://recy-online.scholz-group.com:4437/RecyOnline` → HTTP 302 auf
  `/RecyOnline/Login/?ReturnUrl=%2FRecyOnline` (die App leitet dabei den Port 4437 weg — die
  Folge-URL 404t, die Login-Seite selbst liegt auf 4437).
- `https://recy-online.scholz-group.com:4437/RecyOnline/Login/` → HTTP 200, Title „Anmeldung -
  Recy Online", Formular mit **Benutzername, Passwort, Firma**; Footer „2026 - Recy Systems
  GmbH", „0.8.2-support-6.5.32.2".

**Einstufung:** RECY-Online ist ein **echtes B2B-Portal** des Betreibers (ASP.NET, Betreiber
„Recy Systems GmbH") und damit der plausibelste Ort, an dem Scholz Preise führt. Es ist
**ausschließlich mit Anmeldung** zugänglich. Die Login-Seite wurde gelesen, **nicht** umgangen;
ein Login wurde nicht versucht.

### Zweite Scholz-Domain

Der Standortfeed führt für 51 Datensätze `www.scholz-recycling.de` als „webseite". Diese Domain
liefert bei TLS-Verbindung `error:0A000458: tlsv1 unrecognized name` (kein passender SNI-VHost)
— sie ist **nicht** erreichbar und trägt keine Preisquelle.

## (b) Ohne Anmeldung maschinenlesbar?

**Nein.** Was ohne Anmeldung maschinenlesbar ist, ist der **Standort-/Adressfeed (JSON)** — ohne
Preise. Alles, was Preise enthalten *könnte*, ist entweder nicht vorhanden (WordPress-Seiten),
nicht gelistet (URLs) oder hinter Authentifizierung (RECY-Online).

## (c) Formate

| Format | Wo | Preisbezug |
|---|---|---|
| JSON | `/wp-json/wp/v2/{pages,posts,types}`, `/standorte-json-nicht-loeschen/` | **nein** |
| HTML | alle 20 Seiten, 39 Beiträge, 205 Standortseiten | **nein** (nur Index-Nennung „BDSV oder LME") |
| PDF | Zertifikate, `Betriebsordnung-der-Scholz-Recycling-GmbH_28.09.2026.pdf`, `260623_UeZEFB_MAR.pdf` | **nein** |
| JSON (B2B) | RECY-Online hinter Login | **unbekannt — nicht geprüft** |

## (d) Was daraus realistisch folgt

**Kein Handler.** Es fehlt die für einen Handler zwingende Kombination aus *öffentlicher URL +
Preis + Einheit + Datum + eindeutige Materialbezeichnung* an jeder Stelle. Ein Handler auf
`scholz-recycling.com` wäre gegenstandslos.

**Der belastbare Hebel ist die Preisquelle, und sie ist eine andere als die Website:** Scholz
rechnet nach den **tagesaktuellen Marktpreisen** ab, ermittelt über **BDSV- oder LME-Indizes**.
Diese Indizes sind betreiberübergreifend — sie sind genau die Art Quelle, die eine
**Marktinformations-Pipeline** speisen kann (siehe Abschnitt 4).

## (e) Was zu klären wäre, wenn eine B2B-Quelle existiert

Für RECY-Online (potenziell der einzige Scholz-Preispfad):

1. Gibt es einen **Gewerbekunden-/Lieferantenzugang**, und zu welchen Preisen? (Ankaufspreise je
   Sorte, Abholpreise, Standortdifferenzen?)
2. Wird der Preis **maschinell** angeboten (JSON-/CSV-/XLS-Export, API, Spezialaufruf) oder
   nur im HTML der angemeldeten Oberfläche?
3. Ist ein **Preisarchiv** (Historie) zugänglich — für eine Pipeline wertvoller als der
   Tageswert?
4. Werden **Standortpreise** differenziert, und wenn ja, mit welcher Auflösung?
5. Rechtslage der Weiterverwendung (Eigennutzung vs. Veröffentlichung).

---

# 2. ALBA Metall — metall.alba.info

## (a) Welche Preisquellen es gibt und wo

**Keine Preisliste, in keinem Format, an keiner Stelle.** Die Sitemap-Struktur ist vollständig
kartiert:

| Prüfung | Ergebnis |
|---|---|
| `robots.txt` | HTTP 200, 787 B (TYPO3); `Disallow: /typo3/`, `/*?id=*`, `/*/Private/*`, `*.sql` …; Sitemap: `https://metall.alba.info/sitemap.xml` |
| `/sitemap.xml` | Sitemap-**Index** mit 2 Untersitemaps |
| `?sitemap=pages` | **20 Seiten-URLs** — keine Preis-URL |
| `?sitemap=tx_nelocation_locationdetail` | **13 Standort-URLs** |
| Preis-Pfade (`/preise/`, `/metallpreise/`, `/preisliste/`, `/tagespreise/`, `/ankauf/preise/`, `/einkaufspreise/`, `/notierungen/`, `/kundenportal/`, `/login/`, `/portal/`, `/service/preise/`, `/service/notierungen/`, `/downloads/`) | **alle HTTP 404** (77.442 B) — ALBA liefert hier ehrlich 404 |
| `/fileadmin/` | HTTP 403 (kein Listing); **Einzel-PDFs sind abrufbar** |
| Weitere ALBA-Subdomains `nord.`, `berlin.`, `sachsen.`, `nisa.alba.info` | erreichbar, Sitemaps geprüft (15 bzw. 5 Seiten) — ebenfalls **keine** Preis-URL |

**`/ankauf/sorten/`** bestätigt den Backlog-Befund und geht einen Schritt weiter: Die Seite
liefert in **null** von null Fällen eine Preiszahl (geprüft auf `Ziffer+Dezimal+€/t|/kg`). Titel
und Text:

> „Schrottsorten-Übersicht: **Diese Schrotte können Sie bei uns abgeben**"
> „Der Verkauf von altem Metall lohnt sich! Bei uns profitieren Sie von attraktiven Preisen …"
> „Schnell, rechtssicher und zu Bestpreisen."

Das ist Marketingtext ohne Zahlen. Auch `/ankauf/`, `/ankauf/gewerbe/`, `/ankauf/privathaushalte/`
und `/verkauf/` enthalten **null** Preisangaben (`\d+[,.]\d+\s*(€|EUR|Euro|/t|/kg)` → 0 Treffer).

### Der einzige belastbare „Preis-Auftritt": Einkaufsbedingungen als PDF

`/verkauf/` formuliert:

> „Die aktuellen **Einkaufsbedingungen für den Fe-Streckenhandel der ALBA Metall Nord** finden Sie
> hier." → Link auf `/service/downloads-zertifikate/`

Dort stehen **29 Dateien** als PDF (alle HTTP 200, `application/pdf`; 22 Zertifikate und
7 Bedingungsdokumente). Die preisrelevanten sind:

| Datei | Inhalt | Preisangaben? |
|---|---|---|
| `Einkaufsbedingungen_AMN_Strecke.pdf` | „Annahmebedingungen für Stahlschrott-Streckenlieferungen an die ALBA Metall Nord GmbH", Stand **09.05.2022** (Rostock, 02.06.2022): Sorten nach **Europäischer Stahlschrottliste**, Ausschlussliste, Stückgewicht max. 500 kg, chargierfähig/magnetentladbar | **nein** — reine Qualitätsspezifikation, verweist auf die AGB |
| `150323_AEB_Rohstoffe__SaM__ALBA_Metall_Nord_...pdf` | **Allgemeine Einkaufsbestimmungen Rohstoffe**: Ziff. II.2 „Die in der Bestellung bzw. im Einkaufsvertrag ausgewiesenen **Preise** sind bindend und gelten, sofern nichts anderes vereinbart ist, für die Lieferung 'frei Empfangsstelle'"; Ziff. II.3 Abrechnung nach Empfangsgewicht und Qualitätseinschätzung; Zahlungsziel 30 Tage | **nein** — definiert, dass der Preis **pro Bestellung/Vertrag** festgelegt wird |
| `150323_AVB_Rohstoffe__SaM__...pdf` | Allgemeine Verkaufsbestimmungen | nein |
| `140115_AEB_..._englisch_...pdf`, `140115_AVB_..._englisch_...pdf` | englische Fassungen | nein |
| `2024-04-08-ALB-As_fuer_Standardvertraege.pdf`, `2024-05-AEnderungen_der_ALB-As_Mai_2024.pdf` | Allgemeine Leistungsbedingungen Abfallentsorgung (ALB-A) | nein |
| 22 Zertifikate (ISO 9001/14001/50001, WHG/AwSV, EfB, A\|U\|F, QM-Bescheinigungen, Erstbehandlungsanlagen, GHG Protocol, GVP) | Zertifikate | nein |

> **Kernaussage:** ALBA Metall veröffentlicht **keinen Preis**, sondern **Preisregeln**. Der Preis
> entsteht „in der Bestellung bzw. im Einkaufsvertrag". Es gibt auf `metall.alba.info` **keine
> Stelle**, die einen Index oder eine Notierung als Preisquelle benennt.

### Standortfinder mit Standortpreisen?

**Nein.** Alle 13 Standort-Detailseiten wurden einzeln geladen und geprüft. Die einzigen
Preis-Treffer sind Marketingformeln; der einzige inhaltlich interessante Satz steht auf
Wilhelmshaven:

> „Neben unseren Gewerbe- und Industriekunden freuen wir uns auch über die 'Kleinen': Wir bieten
> schnellen Service und ein **sofortiges Entgelt**." / „Auch Kleinstmengen möglich – gegen
> **sofortiges Entgelt**"

„sofortiges Entgelt" ohne Betrag — die Bestätigung des Backlog-Befunds „keine Kurse". Die
Standortseiten sind serverseitig gerendert (damit für Adressdaten maschinenlesbar), enthalten
aber keine Preisfelder.

### ALBAclick — das Kundenportal

`https://www.alba.info/kundenportal/` → HTTP 200, 47.367 B, Title „**ALBAclick - Das Kundenportal**".
Reine JavaScript-Anwendung ohne serverseitigen Text (im HTML nur „ALBAclick - Das Kundenportal").
Die Seite referenziert als Backend **`https://api.albaclick.de`** (Tomcat). Geprüfte
Endpunkte (`/`, `/swagger/index.html`, `/swagger/v1/swagger.json`, `/openapi.json`, `/health`) →
**alle HTTP 404**; es gibt **keine öffentliche API-Dokumentation**. Weitere verlinkte Portale:
`https://portal.albaclick.de/` sowie `https://www.alba.info/unternehmen/verantwortung-engagement/hinweisgeber-portal/`
(Hinweisgeberportal, kein Preisbezug).

Auf der **Gruppen-Seite** `www.alba.info` (134 Seiten-URLs in der Sitemap, dazu
`tx_nefinancialreports_adhoc`, `tx_neappointments_appointments`, `tx_nemediathek_mediathek`,
`tx_nepressfolder_pressfolder`, `tx_nepressinformations_pressinformation`) gibt es **keinen
Preispfad**. Die Rohstoffhandel-Seiten (`/geschaeftsfelder/rohstoffhandel/eisenmetalle/`,
`/geschaeftsfelder/stahl-und-metall/`) beschreiben Sorten, Spezifikationen und Mengengerüst
ohne jede Preisangabe.

## (b) Ohne Anmeldung maschinenlesbar?

**Für Preise: nein.** Maschinenlesbar ohne Anmeldung sind: die Sitemaps, die 20 Seiten, die 13
Standortseiten, die 29 PDF-Dokumente (via `/fileadmin/user_upload/...`) — **alle ohne Preisdaten**.
Das Kundenportal ALBAclick ist JS-basiert und **login-pflichtig**; die Preise dort sind, falls
vorhanden, nur mit Anmeldung sichtbar.

## (c) Formate

| Format | Wo | Preisbezug |
|---|---|---|
| HTML | 20 Seiten, 13 Standorte, 134 Gruppenseiten | **nein** |
| XML | 2 Sitemap-Untersitemaps | **nein** |
| PDF | 29 Dateien unter `/fileadmin/user_upload/ALBA_Metall_Nord/PDF/…` | **nein** (Regeln, keine Werte) |
| JSON/API | `api.albaclick.de` — kein öffentlich dokumentierter Endpunkt | **unbekannt — nicht geprüft** |

## (d) Was daraus realistisch folgt

**Kein Handler.** Selbst wenn `/ankauf/sorten/` eine Sortenliste mit sauberen Bezeichnungen
liefert — was sie als **Annahmeliste** tut — fehlt der Preis. Eine Sortenliste ohne Preis ist
keine Preisquelle und trägt keinen Handler.

**Für die Pipeline ist ALBA der interessanteste Fall**, weil ALBA die einzige der drei Gruppen
mit einer **veröffentlichten Angebots-/Leistungssystematik** ist: die Europäische Stahlschrottliste
und die Annahmebedingungen liefern Materialbezeichnungen und Spezifikationen. Fehlt nur der
Preis. Damit ist ALBA der Kandidat für einen **B2B-Zugang**, nicht für einen Website-Handler.

## (e) Was zu klären wäre, wenn eine B2B-Quelle existiert

1. **Was ist ALBAclick fachlich?** Die Portaloberfläche wirkt auf Abfall-/Containerleistungen
   ausgerichtet. Gibt es darin einen **Metall-/Rohstoff-Bereich** mit Preisanzeige für
   ALBA-Metall-Kunden?
2. Gibt es einen **Preisfinder je Sorte und Standort** für gewerbliche Anlieferer, und in
   welchem Format (HTML-Tabelle, JSON)?
3. Gibt es einen **Zugang zu Einkaufspreisen für Nichtkunden** (etwa ein tagesaktuelles
   Angebots-PDF je Sortenliste)?
4. Werden die **Einkaufsbedingungen (Stand 09.05.2022)** aktuell gehalten? Falls die Preisfindung
   an die Sortenliste plus „Marktpreis des Liefertags" gekoppelt ist, wäre die Preisquelle der
   **Marktpreis des Tages**, nicht ALBA.
5. Die AEB verweist auf „Unsere Preise werden in der Bestellung ausgewiesen" — wer Zugang zu
   den Bestellungen hat, ist die praktische Preisquelle.

---

# 3. TSR Deutschland — tsr-recycling.de

## (a) Welche Preisquellen es gibt und wo

**Keine öffentliche Preisquelle. Die Preisveröffentlichung ist ausdrücklich anfragebasiert** —
das ist bei TSR der klarste Fall der drei, weil der Betreiber es selbst so formuliert.

Auf `/kunden/schrottlieferanten/` (Archivstand 18.06.2026, da live nicht abrufbar):

> „Schrottankauf ist ein fester Bestandteil unseres Tagesgeschäfts. … Bei uns ist jeder willkommen
> und kann sein Material zu fairen Bedingungen verkaufen. … Als Schrottlieferant haben Sie bei uns
> einen festen Ansprechpartner, der bestens mit dem Markt vertraut ist und Sie jederzeit gerne
> über **aktuelle Marktbewegungen** informiert."

> „**Marktgerechte Preise** — Vertrauensvolle Zusammenarbeit ist uns wichtig. Dazu gehört auch
> absolute Transparenz in Sachen Schrottpreis. Bei uns bekommen Sie garantiert das, was der
> jeweilige Schrott wert ist."

und, unter der Überschrift **„Jetzt aktuelle Preisinformationen einholen!"**:

> „Die **Schrottpreise unterliegen täglichen Schwankungen**, da sie sich nach den aktuellen
> Entwicklungen auf den Rohstoffmärkten richten. Deshalb bieten wir Ihnen stets die besten
> **tagesaktuellen** Preise – für Mischschrott genauso wie für wertvolle Metalle wie Kupfer, Eisen,
> Zink, Zinn oder Blei. Besonders lohnenswert ist die Anlieferung sortierter Materialien, da diese
> höher vergütet werden als beispielsweise unsortierter Mischschrott."

**Der Preismechanismus ist damit offengelegt:**

| Kanal | Beleg |
|---|---|
| **Kontaktformular mit Pflichtfeld „Grund der Anfrage" = „Preisanfrage"** | Dropdown-Werte: Kennenlerntermin, **Preisanfrage**, Informationen zu Materialsorten, Lieferanfrage, Dienstleistung, Sonstiges |
| **Hotline Schrottlieferanten** | 0800 8772667 (zusätzlich Beratungstermin Industriekunden) |
| **Altauto-Einzelangebot** | `/standorte/schrotthaendler-dortmund/formular-dortmund/`: „Verkaufen Sie Ihr Altauto … Bitte achten Sie darauf, dass Sie alle Felder vollständig ausfüllen … Andernfalls können wir Ihnen **kein Angebot** schicken." → Einzelfallangebot, kein Preisblatt |
| **Key-Account-Betreuung Verkauf** | `/kunden/industriekunden/`: Key Account je Region über alle Fe-/NE-Fraktionen; „Jeder von uns angenommene Metallrest wird mit **Wiegeschein** erfasst und dokumentiert" — kein Preis |

**Und die Preise selbst sind laut den Allgemeinen Einkaufsbedingungen Festpreise:**
`Allgemeine_Einkaufsbedingungen_TSR_Deutschland.pdf`, Ziff. 4 „Preise, Gewichts- und Mengenermittlung",
4.1: „Die vereinbarten Preise gelten, sofern nichts anderes vereinbart ist, frei Empfangsstelle.
**Sie sind Festpreise.**"

### Was trotz Bot-Challenge per einfachem Abruf erreichbar ist

TSR steht hinter einer **Incapsula/Imperva-WAF**. Zwei getrennte Sperren:

| Ressource | minimale Header | vollständige Browser-Header |
|---|---|---|
| **jede HTML-Seite** (`/`, `/standorte/…`, `/impressum/`, `/bibliothek/`, `/preise/`, …) | HTTP 200, **1.873 B**, „Challenge Validation" | HTTP 200, **1.873 B**, „Challenge Validation" — **identisch** |
| `/robots.txt` | HTTP 403 „Access Denied" | HTTP 403 „Access Denied" |
| `/sitemap.xml`, `/sitemap_index.xml`, `/uploads/`, `/fileadmin/` (Verzeichnisse) | Challenge | Challenge |
| **`/uploads/tx_3slocations/*.pdf`** | HTTP 403 | **HTTP 200, `application/pdf`, 640.472 B** |
| **`/fileadmin/user_upload/…/*.pdf`** | HTTP 403 | **HTTP 200, `application/pdf`, 138.028 B** |

> **Der entscheidende Punkt:** Die JavaScript-Challenge gilt **nur für HTML**. Statische Dateien
> sind mit einem vollständigen Browser-Header-Satz **ohne Challenge** direkt abrufbar. Der
> Backlog-Befund „jede URL liefert eine 1.873-Byte-Challenge-Seite" gilt damit **nur für
> HTML-Seiten** — die Aussage war zu pauschal. **Eine Preisliste als PDF unter `/uploads/` oder
> `/fileadmin/` wäre technisch abrufbar**, ohne dass eine Challenge gelöst wird.

Die Challenge selbst wurde **nicht** gelöst und nicht umgangen; getestet wurden ausschließlich
zwei vollständige, reguläre Browser-Header-Sätze gegen reguläre Pfade.

### URL-Struktur (Wayback-Rekonstruktion)

CDX-Index für `tsr-recycling.de`: **748 Snapshots / 252 eindeutige Nicht-Datei-URLs**, plus die
archivierte Sitemap (17.01.2024, TYPO3) und die Navigation des jüngsten HTML-Snapshots
(18.06.2026).

Top-Level-Segmente: `/standorte` (53), `/fileadmin` (44), `/typo3temp` (62), `/containeranfrage` (8),
`/seiten` (5), `/kunden` (5), `/suche` (4), `/navigation` (2), `/typo3conf` (9), `/.well-known` (5) —
sowie Einzelseiten `/bibliothek/`, `/kontakt/`, `/impressum/`, `/datenschutzhinweise/`,
`/bildnachweise/`, `/lksg/`, `/feed/`, `/sitemap/`, `/unternehmen/`.

Navigation der aktuellen Site (aus dem 18.06.2026-Snapshot): Unternehmen, Standorte, Kunden
(Schrottlieferanten / Industriekunden / Gewerbekunden und Handwerk / Kommunen / Privatkunden),
Bibliothek, Karriere, Containeranfrage, LkSG, Kontakt, Impressum, Datenschutz.

**Von 748 archivierten URLs trägt keine einzige ein Preis-Muster.** Es gibt archiviert
**kein** `/preise/`, `/tagespreise/`, `/metallpreise/` oder `/preisliste/`. Der einzige
Preis-bezogene Treffer im gesamten Archiv ist eine **Suchanfrage-URL**:
`/suche/?tx_kesearch_pi1[sword]=Alu schrottpreis` — also eine vom Suchportal erzeugte Suchanfrage,
kein Preisblatt.

### Die Bibliothek: 244 PDF-Links, 45 Dateien, null Preise

`/bibliothek/` (Archiv 11.03.2026) enthält 244 PDF-Links zu **45 eindeutigen Dateinamen**:

- 32 Zertifikate (EfB-Regionen/Standorte, A|U|F, ISO 9001/14001/50001, QM, EnM, EoW,
  Altfahrzeug-Demontage, GVP-Mitgliedsurkunde, TSR-Group-Compliance-Bericht)
- 7 Bedingungsdokumente (Allgemeine Einkaufsbedingungen TSR Deutschland, MHR Metallhandel Ruhr,
  TSR Resource; Allgemeine Verkaufs- und Zahlungsbedingungen TSR, MHR, TSR Metals, TSR Resource;
  Cablo)
- Datenschutzhinweise

**Kein einziger Dateiname mit Preis-, Notierungs-, Kurs- oder Tagesmuster.** Die beiden
„Einkaufsbedingungen"-PDFs sind dieselbe Gattung wie bei ALBA: Vertragsregeln, keine Werte.

**Kein Marktplatz, kein Portal, keine App:** Über alle Live- und Archivseiten gibt es keinen
Link auf eine B2B-Plattform, ein Kundenportal, einen Login oder einen App-Store. Die einzige
externe Branchenreferenz ist `metall-und-recycling.de` (Branchenmedium) — das selbst hinter
derselben WAF steht (1.964 B Challenge) und daher keine Preisquelle darstellt.

## (b) Ohne Anmeldung maschinenlesbar?

**Für Preise: nein.** Erreichbar ohne Challenge sind (a) statische PDFs via vollständige
Browser-Header — bislang ausschließlich Zertifikate und Bedingungen, und (b) die Wayback
Archive. Die Preisquelle selbst (Kontaktformular, Hotline, Key Account) ist **nicht** maschinenlesbar.

## (c) Formate

| Format | Wo | Preisbezug |
|---|---|---|
| HTML (live) | alle Pfade | **nur Challenge-Seite** (1.873 B) |
| PDF (live) | `/uploads/tx_3slocations/…`, `/fileadmin/user_upload/tsr-local-sites/downloads/…` | **nein** (Zertifikate, Bedingungen) |
| HTML/PDF (Archiv) | Wayback-Snapshots aller Seiten und der Bibliothek | **nein** |
| JSON | keine gefunden | — |
| API | keine gefunden | — |

## (d) Was daraus realistisch folgt

**Kein Handler**, und zwar hier am eindeutigsten: TSR veröffentlicht Preise **nicht**, sondern
**gewährt** sie — auf Anfrage, per Formular, per Hotline, per Key Account. Das ist eine
**Geschäftsbeziehung, keine Datenquelle**. Ein Handler, der die Preisquelle nachahmen müsste,
müsste Anfragen stellen — das ist rechtlich und technisch (Login, Formular, ggf. CAPTCHA) nicht
der Weg.

Der korrigierte Backlog-Satz „jede URL liefert eine Challenge-Seite" gilt nur für HTML. Das ist
für die Strategie relevant, weil es den **billigsten möglichen Test** eröffnet: eine
Preisliste-PDF wäre unter `/uploads/` oder `/fileadmin/` ohne Challenge abrufbar. Gegen die
Existenz einer solchen PDF spricht die **vollständig öffentlich gelistete Bibliothek**: 45
Dateinamen, alle benannt, keiner mit Preischarakter. Die Wahrscheinlichkeit einer
unverlinkten Preisliste ist gering, ein Negativbefund wäre aber mit einem Verzeichnislisting
nicht abschließend beweisbar.

## (e) Was zu klären wäre, wenn eine B2B-Quelle existiert

1. Gibt es ein **Lieferantenportal** (Eingangsanmeldung, Waage, Gutschrift) neben dem
   Kontaktformular? Die Preisanfrage läuft heute über ein **Formularfeld**, nicht über ein Portal
   — das wäre der Hebel.
2. Gibt es eine **Preisanfrage-API** hinter dem Formular (TYPO3-Formular, das JSON statt
   HTML zurückgibt)?
3. Gibt es für **Industriekunden** ein B2B-Portal mit Angebots-/Auftragspreisen je Los?
4. Gibt es eine **Preis-Historie** für die Pipeline (statt Tageswert)?
5. Würde TSR eine **maschinenlesbare Preisschnittstelle** (API/Feed) anbieten? Das ist die
   einzige Variante, die einen Handler tragen würde.

---

# 4. Querschnitt: die tatsächliche Preisquelle und die realen Optionen

## Die eine öffentliche, ohne Anmeldung maschinenlesbare Quelle im ganzen Feld: BDSV

`https://bdsv.de/` (HTTP 200, 88.610 B) enthält auf der **Startseite** öffentlich und ohne
Anmeldung einen Preisblock:

> **„BDSV Durchschnittspreise in Deutschland in €/t (09/2026)"**

| Sorte | Wert |
|---|---|
| Sorte 1 | 254,6 €/t |
| Sorte 2/8 | 294,6 €/t |
| Sorte 3 | 289,0 €/t |
| Sorte 4 | 302,9 €/t |
| Sorte 5 | 246,5 €/t |

**Format und Maschinenlesbarkeit:** stabiles HTML (`div.price-list` → `div.price-list-row.price-types`
mit den Labels, darunter `div.price-list-row` mit je `div.price > span` (Sorte) und `h4`
(Wert)). Deutsches Dezimalkomma, Einheit `€/t`, Monatsstand im Titel. Maschinenlesbar ohne
Anmeldung, ohne JavaScript, ohne API.

**Grenzen dieser Quelle:**

- Es ist ein **Durchschnitts-Index für Deutschland**, kein Betreiber-Angebotspreis. Er beantwortet
  „wo liegt der Markt", nicht „was zahlt ALBA in Wilhelmshaven heute".
- Die Sortenbezeichnungen sind **generisch** („Sorte 1", „Sorte 2/8"). Der Link
  „DETAILS UND HINWEIS ZU AKTUELLEN PREISEN" → `/unser-service/markt-preise/` ist **heute ein
  toter Link (HTTP 404)**. Die Auflösung der Sorten zu Materialien (Europäische Stahlschrottliste)
  ist damit **nicht** vom Angebot abgedeckt.
- **Granularität: monatlich.** Ein 6-Stunden-Handler-Aktualisierungstakt (`Schedule::every_6h()`)
  würde jeden Lauf als „unverändert" behandeln. Das ist kein Defekt, aber es verschiebt die
  Freshness-Diskussion: Der Wert ändert sich monatlich, nicht stündlich.
- **Lizenz/Nutzungsbedingungen sind ungeklärt** — BDSV-Publikationen sind teils mitgliederpflichtig;
  eine Weiterveröffentlichung der Werte braucht eine rechtliche Prüfung.

**LME** wird von Scholz als Indexquelle genannt, ist aber **nicht** frei maschinenlesbar:
`https://www.lme.com/en/metals/non-ferrous/lme-steel-scrap` → HTTP 403 in diesem Abruf; LME-Daten
sind ein **kostenpflichtiges Produkt** (LME Data). In diesem Durchgang **nicht** als frei
zugängliche Quelle verifiziert.

## Realistische Optionen, bewertet

| Option | Begründung | Bewertung |
|---|---|---|
| **Handler je Betreiber-Website** | Keine der drei Gruppen hat eine öffentliche Preisliste; ALBA und TSR dokumentieren Preisermittlung **per Vertrag/Anfrage**, Scholz verweist selbst auf **BDSV/LME** | **Nicht möglich.** Der Backlog-Befund ist bestätigt und verschärft: es gibt nicht nur „keine Preisliste", sondern der Betreiber benennt den Preisindex selbst |
| **Marktinformations-Pipeline (Index statt Betreiberpreis)** | BDSV liefert öffentlich, ohne Anmeldung, maschinenlesbar: €/t, Monatsstand, 5 Sorten; ALBA und TSR verweisen auf Marktpreise/Notierungen; Scholz nennt BDSV/LME explizit | **Machbar, aber als Index, nicht als Händlerpreis.** Die Zuordnung Sortennummer → Material muss aus der Europäischen Stahlschrottliste kommen, weil der BDSV-Detail-Link 404t. Lizenzfrage offen |
| **B2B-Zugang anfragen** | RECY-Online (Scholz, Login), ALBAclick (ALBA, Login), TSR-Preisanfrage (Formular/Hotline) | **Der einzige Weg zu betreiber-echten Preisen.** Konkret zu klären: Zugang, Format, Historie, Standortdifferenzierung, Nutzungsrecht |
| **Gar nichts** | Wenn weder eine öffentliche Preisliste existiert noch ein B2B-Zugang gewährt wird | **Für die Betreiberpreise der faktische Befund.** Der Preis ist bei allen dreien Vertragsgegenstand, nicht Datenquelle |

## Empfehlung

1. **Keinen Handler auf `scholz-recycling.com`, `metall.alba.info` oder `tsr-recycling.de`
   anlegen.** Die Voraussetzung „öffentliche URL + Preis + Einheit + Datum + eindeutige
   Materialbezeichnung" ist bei keinem der drei erfüllt.
2. **Den Backlog-Befund korrigieren:** Bei TSR gilt die Challenge nur für **HTML**. Statische
   PDFs sind mit vollständigen Browser-Headern ohne Challenge abrufbar — das ist der einzige
   verbleibende Ort, an dem eine Preisliste liegen *könnte*, und er ist billig zu prüfen.
3. **Die Preisquelle als Gespräch mit den Betreibern führen** (RECRY-Online-Zugang als
   Lieferant, ALBAclick-Metallzugang, TSR-Preisanfrage-Backend). Das ist die einzige Option, die
   Betreiberpreise statt Indizes liefert.
4. **Parallel die Index-Pipeline auf BDSV prüfen** — als *Marktpreisreferenz*, klar von
   Händlerpreisen getrennt. Vorher klären: Sortenzuordnung, Nutzungsrecht, Monatsgranularität
   gegen den Freshness-Vertrag.

## Anhang: Beleg-URLs (alle Abrufe 27.10.2026)

**Scholz**
`https://www.scholz-recycling.com/robots.txt` · `/wp-sitemap.xml` (+ 6 Untersitemaps) ·
`/wp-json/wp/v2/pages` · `/wp-json/wp/v2/posts` · `/wp-json/wp/v2/types` ·
`/geschaeftsbereiche/` · `/recycling/` · `/services/` · `/veroeffentlichungen/` ·
`/veroeffentlichungen/cbcr-reporting/` · `/standorte/` · `/standorte-json-nicht-loeschen/` ·
`/zertifikate/` · `/login/` · `https://recy-online.scholz-group.com/` ·
`https://recy-online.scholz-group.com:4437/RecyOnline` ·
`https://recy-online.scholz-group.com:4437/RecyOnline/Login/`

**ALBA**
`https://metall.alba.info/robots.txt` · `/sitemap.xml` (+ 2 Untersitemaps) · `/ankauf/` ·
`/ankauf/sorten/` · `/ankauf/gewerbe/` · `/ankauf/privathaushalte/` · `/verkauf/` ·
`/unternehmen/standorte/` + 13 Detailseiten · `/service/downloads-zertifikate/` ·
`/fileadmin/user_upload/ALBA_Metall_Nord/PDF/Einkaufsbedingungen_AMN_Strecke.pdf` ·
`…/150323_AEB_Rohstoffe__SaM__ALBA_Metall_Nord_Finale_Version_nach_Umfirmierung.pdf` ·
`https://www.alba.info/kundenportal/` · `https://api.albaclick.de/` ·
`https://www.alba.info/sitemap.xml` (+ 7 Untersitemaps) · `nord./berlin./sachsen./nisa.alba.info`

**TSR**
`https://www.tsr-recycling.de/robots.txt` (403) · `/` · `/sitemap.xml` · `/standorte/…` ·
`/uploads/tx_3slocations/Efb-Zertifikat-2026_TSR-NordOst1.pdf` (200 PDF) ·
`/fileadmin/user_upload/tsr-local-sites/downloads/Allgemeine_Einkaufsbedingungen_TSR_Deutschland.pdf` (200 PDF) ·
`https://www.tsr.eu/` · Wayback: `web.archive.org` CDX `tsr-recycling.de` (748 Snapshots),
Snapshot 18.06.2026 (`/standorte/schrotthaendler-dortmund/formular-dortmund/`),
Snapshot 18.06.2026 (`/kunden/schrottlieferanten/`), Snapshot 11.03.2026 (`/bibliothek/`),
Snapshot 17.01.2024 (`/sitemap.xml?sitemap=pages…`)

**Indexquelle**
`https://bdsv.de/` (Preisblock 09/2026) · `https://bdsv.de/unser-service/markt-preise/` (404) ·
`https://www.lme.com/en/metals/non-ferrous/lme-steel-scrap` (403) · `https://www.lme.com/en/robots.txt`

**Produktionsbestand (read-only, `mode=ro`, `PRAGMA query_only=ON`)**
`public.db`: `traders` — Zählung nach `website`-Host mit `status='aktiv' AND
website_status='aktiv'`.
