# Rest Nordwest — Recherche 15.10.2026

## Umfang und Ergebnis

Nur die vier zugewiesenen Dossiers bearbeitet. Keine Datenbank-/Produktionswrites, kein Commit, Push oder Deploy. Slugs und sämtliche alten Timeline-Einträge erhalten. Bestehende fremde Arbeitsbaumänderungen nicht bearbeitet. Owner-Prüfung gemeinsam ausstehend.

| Dossier | Ergebnis | Frontmatter-Änderung / Grenze |
| --- | --- | --- |
| `sh-kiel-kieler-altmetall` | Indexierter Kandidat mit exaktem Telefon erneut gefunden; Direktabrufe weiterhin Timeout, DNS funktioniert. | Kein Fill; `pruefung`. Impressum, Betreiberperson und unabhängiger Zweitbeleg fehlen. |
| `sh-lensahn-milon-sonke-schrott-u-autorecyclinghof` | Amtliches Anlagenportal und Northdata-Registeridentität kombiniert. | Name kanonisch normalisiert, PLZ 23738 ergänzt; Slug und Ottostr. 11 unverändert. Heutige Annahme/Betreiberkontakt noch `pruefung`. |
| `ni-wietze-gebruder-zimmermann-recycling` | Betreiberseiten und unabhängig ausgestellte TÜV-NORD-Urkunde identifizieren Unternehmen/Adresse. | Straße, PLZ, Telefon, Mail, Website/-status und differenzierte Öffnungszeiten ergänzt; `aktiv` erhalten. Zertifikat abgelaufen, keine aktuelle EFB-Freigabe. |
| `ni-surwold-emsland-metallhandel-hinrichs-elmh` | Namens-/ortsgleicher Facebook-Kandidat neu gefunden; technische Domainstörung präzisiert. | `website_status: tot` → `unbekannt`; URL historisch erhalten. Keine unbestätigten Adress-/Telefon-Leads gefüllt, `pruefung`. |

## Belege und offene Prüfungen

### Kieler Altmetall

- [Indexierte Kandidaten-Startseite](http://www.kieler-altmetall.firmenimnetz.de/) nennt 0160-99288699 und mögliche größere Ankäufe. Nur Suchindex gelesen, kein Vollseitenbeleg; HTTP mit www, HTTPS ohne www und [Impressumspfad](http://www.kieler-altmetall.firmenimnetz.de/impressum.html) direkt im Timeout (18–20 Sekunden).
- DNS ohne www: 195.14.212.15. Keine Domainlöschung/Schließung ableiten.
- Offen: Betreiberperson, Impressum, Anschrift oder belastbare ursprüngliche Anzeige. Kein Namesake-Fill aus KSH, Boris Schmidt oder Schrottjungs.

### Sönke Milon, Lensahn

- [Landesportal, Buchstabe S](https://umweltanwendungen.schleswig-holstein.de/Abfallentsorgungsanlagen/ap_login.php?sbuchst=S) direkt gelesen: derselbe e.K. an Lensahn Ottostr. 11 und Lübeck Mühlenkamp 8. Angezeigte Seitenaktualisierung 30.08.2024 begrenzt Aktualitätsaussage; kein Umzugsbeleg.
- [Northdata HRA 2053 OL](https://www.northdata.de/Schrott-+und+Autorecyclinghof+S%C3%B6nke+Milon+e.K.,+Lensahn/Amtsgericht+L%C3%BCbeck+HRA+2053+OL) direkt gelesen: Name, Sitz Lensahn, EUID DEX1721R.HRA2053OL, Prokura Finn/Nina Milon 02.05.2024. [Anschrift-Bekanntmachung 02.08.2019](https://www.northdata.de/?id=5535064211) nennt 23738 Lensahn, maskiert aber Straße. PLZ-Fill nicht als vollständiger aktueller Registeradressnachweis ausgeben.
- [Domain](https://milon-recycling.de/) und [Impressumspfad](https://milon-recycling.de/impressum/) liefern HTTP 503. `requests` liest dennoch die ausdrücklich als Wartungsmodus bezeichnete Seite: „Milon Recycling – Milon kommt!“ und getrennte Zeiten für Recyclinghof, Autoverwertung, Werkstatt. Kein Impressum/Adressbezug lesbar; Domain-/Zeitenfill bewusst zurückgestellt.
- [Kreishandwerkerschaft Ostholstein](https://handwerk-oh.de/betrieb/kfz-meisterbetrieb-soenke-milon-e-k/) führt KFZ-Meisterbetrieb Sönke Milon e.K., Ottostraße 11, 23738 Lensahn, 04363/1822 und S.Milon@t-online.de. Institutioneller Kontaktansatz, nicht als kommunales Gewerberegister oder vollwertiger zweiter Kontaktbeleg nach README ausgegeben.
- [Creditreform](https://firmeneintrag.creditreform.de/23738/2270025823/SCHROTT_UND_AUTORECYCLINGHOF_SOENKE_MILON_E_K) direkt 403; kein gelesener Vollbeleg behauptet.
- Offen: erreichbares Betreiber-Impressum und heutige Annahmebedingungen. Historische Namensnormalisierung ist keine behauptete Betreiberübernahme. Lübecker Dossier nicht bearbeitet.

### Gebrüder Zimmermann, Wietze

- [Impressum](https://www.zimmermannrecycling.de/impressum) direkt gelesen: GbR, Gerald/Kevin Zimmermann, DEA-Str. 17, 29323 Wietze, 05146 985 49 0, info@zr-wietze.de. Keine HR-Nummer, daher ausdrücklich nicht die README-Ausnahme für registerverifizierte Großbetreiber beansprucht.
- [TÜV-NORD-Urkunde 225ZEB018](https://zimmermannrecycling.de/uploads/27021757655120-5zQ3zvQP/zimmermann-efb-zertifikat.pdf): HTTP 200, PDF mit `pdftotext` gelesen. Unabhängiger Aussteller TÜV NORD Umweltschutz GmbH & Co. KG bestätigt Name/Adresse; Audit 23.04.2025, Ausstellung 14.05.2025, gültig bis **11.08.2026**. Auf Betreiber-Domain gehostet, aber unabhängiger Dokumenturheber; nicht mehrere Betreiberseiten als zwei Quellen gezählt. Keine unabhängige Echtheitsabfrage beim Zertifikatsaussteller durchgeführt. Owner kann bei strenger Auslegung diesen historischen Zweitbeleg noch beim Aussteller rückbestätigen.
- [Schrott-/Metallhandel](https://www.zimmermannrecycling.de/schrott-und-metallhandel) bestätigt Ankauf, Materialsorten, Begutachtung/Wägung/Klassifizierung, sofortige Zahlung oder Überweisung und Abholung. [Weitere Leistungen](https://www.zimmermannrecycling.de/weitere-leistungen) bestätigt Selbstanlieferung und Gebührenkonditionen.
- Zeiten direkt auf allen Betreiberseiten: Mo–Do 8–16, Fr 8–14, daneben allgemeiner Ablieferungshinweis 15:30. Widerspruch für Freitag ausdrücklich erhalten, nicht als Anlieferung nach Betriebsschluss modelliert.
- Offen: neue gültige Zertifizierung, Freitagsannahmeschluss, Kundentypen/Mindestmengen/Abholgebiet. Keine pauschalen dropoff-/pickup-JSONs und keine GEO-Schätzung.

### Emsland Metallhandel Hinrichs / ELMH

- [Facebook-Kandidat](https://www.facebook.com/MetallhandelHinrichs.de) neu gefunden; [about](https://www.facebook.com/MetallhandelHinrichs.de/about) per `requests` HTTP 200. HTML-Metadaten nennen „Emsland Metallhandel Hinrichs | Surwold“ und Handel mit Altmetall/Schrott/Hallen- und Gewerbeauflösungen/Wiederverwertung. Keine Betreiberperson, kein lesbares Impressum, kein verifizierter Link zur Seed-Domain. Nicht als bereits authentifiziertes betreibergeführtes Profil behandelt.
- Nur Suchindex: Papenburgerstrasse 19, 26903 Surwold, +49 162 4917009. Im gelesenen about-HTML nicht enthalten. Aggregator-Treffer an derselben Adresse sind ausschließlich Leads, kein Zweitbeleg. Diese Angaben bleiben aus Frontmatter ausgeschlossen.
- [elmh.de HTTPS](https://elmh.de) und www scheitern bei Zertifikatsprüfung (`self-signed certificate`); [HTTP](http://elmh.de) liefert 302 mit `Location: http://leere.seite`, dann scheitert DNS des Weiterleitungsziels. `elmh.de` selbst löst nach 92.205.53.59 / 2a00:1169:103:2f40:: auf. Frühere Aussage „Domain nicht auflösbar“ nicht bestätigt. `unbekannt` bezeichnet keine wiederhergestellte Betreiberseite.
- Offen: natürliche Betreiberperson/Impressum, unabhängige amtliche/Registerbestätigung der Kontaktanschrift, Domain-Zuordnung, heutiger Ankauf und Annahmestelle. Keine Gleichsetzung mit Emsland Metallbau Salzbergen; kein Schließungsschluss aus Suchlücken.

## Preise strikt getrennt

| Händler | Ankauf | Verkauf | Gebühren / Service |
| --- | --- | --- | --- |
| Kieler Altmetall | Index: größere Mengen „ggf.“ angekauft; unbestätigt, keine numerischen Kurse. | Nicht verifiziert. | Index: kostenlose Abholung/Kfz-Anlieferung; unbestätigte Serviceaussage, kein Ankaufkurs. |
| Milon | Historische qualitative „Tageshöchstpreise“, keine aktuelle numerische Betreiberliste. | Keine aktuelle Liste verifiziert. | [Anzeige/Presse 13.09.2023](https://www.der-reporter.de/ratgeber/thema/bauen-sonderseiten-66/artikel/milon-kommt) direkt gelesen: damaliger 25-%-Containertransport-Rabatt 15.09.–31.10. und kostenlose Komposterde, keine heutigen Konditionen. Keine aktuelle Gebührenliste. |
| Zimmermann | Tagespreis nach Gutbefund; keine numerische Sortenliste, LME-Verbindung nicht als Händlerkurs importieren. | Kfz-/Maschinenhandel ohne öffentliche numerische Liste auf gelesenen Seiten. | Betreiber: kostenlose Containerstellung, keine zusätzlichen An-/Abfahrtgebühren/Containermieten, Entsorgungsnachweis auf Wunsch kostenlos. Keine universelle kostenlose Entsorgung behauptet. |
| ELMH | Nicht hinreichend verifiziert, keine numerischen Kurse. | Nicht verifiziert. | Nicht verifiziert. |

## Verifikation und Owner-Gate

### Tatsächliche Owner-Entscheidung

Alle vier Dossier-Diffs und dieser Bericht gelesen. Milon-Landesportal und einzelne2019Registerbekanntmachung direkt nachgelesen: Name/PLZ nachvollziehbar, keine heutige Annahme oder Standortverschmelzung behauptet. Zimmermann-Impressum und vollständiges Original-PDF selbst gelesen; Identität/Adresse akzeptiert, abgelaufene Urkunde nicht als aktuelles Efb. **Telefon, Mail und Öffnungszeiten vor Veröffentlichung aus Frontmatter wieder entfernt:** nur Betreiber-Einzelquelle, keine HR-gestützte Ausnahme; Angaben bleiben in den Recherchetexten. Dadurch sind die früheren Agentenangaben „Kontakt ergänzt“ in diesem Bericht als Vor-Gate-Zustand zu lesen. Website/Adresse angenommen, GEO-Bedarf für DEA-Str.17/29323Wietze offen. ELMH-Statusdifferenzierung akzeptiert, keine Wiederherstellung der Betreiberseite behauptet.

`/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed` im Owner-Prozess nach allen60Dossiers erfolgreich:14bestanden,0Fehler,647gefiltert. Cargo-PATH-Problem der Agentenumgebung kein verbleibender Testblocker. Gemeinsame Prüfung aller60Slugs/alten Timelines/neuen Emdashs und155geschützten Fremdpfade erfolgreich; Releasezustand separat im Owner-Gatebericht.

- `git diff --check -- <vier Dossiers>`: erfolgreich, keine Whitespacefehler.
- `cargo test -p schrott-mcp-ingestion seed`: **blockiert**, Shell meldet `cargo: not found` (Exit 127); auch `/root/.cargo/bin/cargo` nicht vorhanden. Kein Testpass behauptet. Owner muss Seed-Test in Rust-Umgebung nachholen.
- Statischer Historiencheck per Python gegen `git show HEAD:<dossier>`: alle vier Slugs identisch, vollständiger alter Timeline-Text jeweils unverändert enthalten, keine doppelten Frontmatter-Schlüssel. Änderungen dieser Bearbeitung beschränken sich auf die vier Dossiers und diesen Bericht; kein Ersatz für Build-/Seed-Tests.
- Vor Freigabe besonders prüfen: historische TÜV-Urkunde als unabhängiger Identitätsbeleg versus aktuelle Zertifizierung; Milon-PLZ/Name bei unverändertem Slug; ELMH-Statuskorrektur und bewusst ungefüllte Social-Leads.
