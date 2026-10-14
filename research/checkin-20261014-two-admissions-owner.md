# Owner-Gate: Hauser und Bissendorf, 14.10.2026

## Lokale Neuaufnahmen

Genau zwei neue Standortdossiers nach unabhängiger Quellen-/Identitätsprüfung:

- [Hauser Bretten-Gölshausen](../dossiers/bw/bw-bretten-hauser-recycling-transporte.md),
  Feedback 6200/6979. [Aufnahmeprüfung](feedback-20261014-hauser-admission.md).
- [Bissendorf Recycling Wedemark](../dossiers/ni/ni-wedemark-bissendorf-recycling.md),
  Feedback 6911/7369. [Aufnahmeprüfung](feedback-20261014-bissendorf-admission.md).

Beide Betreiber bieten ausdrücklich Ankauf am konkret zugeordneten Standort
an; Firmenregistergegenstand allein wäre dafür kein Beleg. Die jeweiligen
Doppelmeldungen erzeugen je ein Dossier, keine zwei Händler.

## Separate Owner-Prüfung

Hausers unabhängiges Registerprofil selbst direkt gelesen: HRB 240545
Mannheim, Gewerbestr. 59, aktuelle Vertreterstruktur und Publikationen 2025/26
kongruent. Kein amtlicher Tagesauszug behauptet. Betreiber-Leistungsseite
selbst gelesen: vergüteter Privat-/Gewerbeankauf von Metallen/Kabeln.
Aktuelles achtseitiges DEKRA-PDF selbst regulär heruntergeladen und Text
geprüft: Standort, HRB, Bring-/Holsystem, Prüfung 29.01.2026, Ausstellung
01.07.2026, Gültigkeit bis 28.07.2027. Zertifizierte Abfallarten sind keine
pauschale Kundenannahmezusage. Service-JSON bewusst unbekannt, AGB beachten.
Quellen und datierte Grenzen in Dossier/Agentenbericht.

Bissendorfer amtlichen AD vollständig gelesen: zwei Seiten, Abruf
14.10.2026 04:03, HRB 215994 Hannover, Firma/Anschrift/Timo Kohsen kongruent,
sonstige Rechtsverhältnisse leer; kein Auflösungs-/Löschungsvermerk.
PDF-SHA-256 bestätigt:
`afe71b94734a72fc33390e0bf8f64b0443fac4ed24843ccbcd81f6399654a9d1`.
Lokaler Beleg `/tmp/opencode/bissendorf-register-ad-20261014.pdf`;
reproduzierbare Suche im Registerportal in Aufnahmebericht beschrieben.
Betreiber-Angebotsseite bereits separat direkt gelesen: Ankauf, Platzbesuch,
Abholung/Container positiv belegt, aber keine Gratis-/Mengen-/Preiszusagen.
Footer-Mailkonflikt ausdrücklich erhalten, keine Dettmer-Nachfolgekette.

## Gemeinsames Abschlussgate

- Betreiber-/Registerfamilien, Namensvetter und breite Dublettenprüfungen
  beider Berichte gesichtet; keine bestehende Identität umbenannt/verschmolzen.
- Neue Slugs jeweils genau einmal im Dossierbestand; Produktions-Slug-
  Gegenprobe read-only ohne Treffer. Feld-/Enum-Codegen erfolgreich.
- Owner normalisiert beide Website-Felder auf separat geprüfte Leistungs-/
  Angebot-Deep-Links. Keine Kontakt-Emdash-Füllung, keine Defaultannahme
  unbekannter Bedingungen, keine unbestätigten Spezialmaterialien.
- Keine Koordinaten oder numerischen Preise. Adressen benötigen noch
  eigenständige GEO-Prüfung; keine Rust-Geocodierung.
- Alle 155 geschützten fremden Dateien per SHA-256 unverändert. Bestehende
  Dossierhistorien nicht verändert; nur zwei neue Dossiers und drei eigene
  Forschungsberichte gehören zum Commit.
- Gemeinsamer `cargo test -p schrott-mcp-ingestion seed` nach allen Owner-
  Korrekturen: 14 bestanden, 0 fehlgeschlagen. Log:
  `/tmp/opencode/checkin-20261014-two-admissions-seed.log`.

## Veröffentlichung bleibt offen

Nur lokale Integration auf Main. Push, Redeploy beider Binaries und
Produktions-Spot-Verifikation bleiben bei fehlender GitHub-Anmeldung offen.
Die Produktionsdatenbanken wurden nicht beschrieben; Feedbacks nicht als
erledigt quittiert. Read-only Bestand weiter 3881 Händler, 54 Materialien,
2426 aktuelle Preise; die neuen Dossiers sind dort noch nicht veröffentlicht.
Ein Release muss aus einem geprüften sauberen Commit gebaut werden, nicht
aus dem Gesamtbaum mit den 155 fremden Änderungen.
