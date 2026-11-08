#!/usr/bin/env python3
"""Wellen-Eintraege fuer Shard 2 (Hessen, Welle 08.11.2026, 12:00 UTC).

Schreibt Frontmatter-Aenderungen und den neuen Timeline-Abschnitt
"### Recherche 08.11.2026" an das Ende des jeweiligen Dossiers.
Vorwellen-Notizen, Slugs und provenance_* bleiben unveraendert.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOSS = ROOT / "dossiers" / "he"

REG = "https://fachbetrieberegister.gadsys.de/fachbetrieberegister"
HRP = "https://www.online-handelsregister.de/suche/"
HR_LIVE = "https://handelsregister.live"
NOMI = "https://nominatim.openstreetmap.org/search?format=jsonv2&limit=5&addressdetails=1&countrycodes=de&q="

# ---------------------------------------------------------------- Frontmatter
FM = {}

FM["he-offenbach-am-main-fred-de-snoo-rohstoffe-schrott-und-metal"] = {
    "street": "Lämmerspieler Weg 158",
    "postcode": "63075",
    "phone": "069 856112",
    "email": "info@freddesnoo.de",
    "opening_hours": "Mo-Do 07:00-12:00 und 13:00-16:15, Fr 07:00-12:00 und 13:00-14:00, Sa 08:00-13:00",
    "lat": "50.1077259",
    "lon": "8.8018644",
    "status": "aktiv",
    "description": (
        "Betreiber-Primärquelle und amtliches Fachbetrieberegister nennen denselben Betrieb an "
        "derselben Anschrift: Fred De Snoo Rohstoffe e.K., Inh. Nicole Macaluso, Lämmerspieler Weg 158, "
        "63075 Offenbach, HRA 42548 Amtsgericht Offenbach am Main. Das 2026 ausgestellte "
        "Entsorgungsfachbetrieb-Zertifikat (TÜV Technische Überwachung Hessen, Zertifikatsnummer "
        "32970032, Vorgangsnummer ZZFT012000239009, gültig bis 07.09.2027) führt beide Anlagen am "
        "Standort Lämmerspieler Weg 158; das amtliche Register listet denselben Standort mit den "
        "Anlagen ZZFT01200023 / 1 (Sammeln, Befördern) und / 2 (Lagern). Die Betriebsstätte ist "
        "damit belegt und geokodiert. Ankaufvergütung wird nach Analyse und Verwiegung vor Ort "
        "zugesagt, numerische Kurse sind nicht veröffentlicht; eine Verkaufspreisliste gibt es nicht, "
        "Container- und Muldengebühren werden nicht beziffert."
    ),
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG (Zertifizierungsorganisation: TÜV Technische "
        "Überwachung Hessen GmbH, Robert-Bosch-Straße 16, 64293 Darmstadt; Zertifikatsnummer 32970032, "
        "Vorgangsnummer ZZFT012000239009; Folgezertifizierung; gültig bis 07.09.2027; Prüfungsdatum "
        "31.03.2026, Ausstellungsdatum 05.06.2026; Zertifikat umfasst 2 Anlagen, beide am Standort "
        "Lämmerspieler Weg 158, 63075 Offenbach; Rechtsträger laut Zertifikat „Fred de Snoo "
        "Rohstoffe“, Lämmerspieler Weg 158, 63075 Offenbach, Registernummer HRA 42548, Registergericht "
        "Offenbach am Main; Anlage 1 Tätigkeit Sammeln, Befördern (Kennnummer nach § 28 NachwV "
        "F34T115545) mit Beschreibung „Sammeln und Befördern von überwiegend Metallschrotten von und "
        "zur Anlage“; Anlage 2 Tätigkeit Lagern (Kennnummer nach § 28 NachwV F34RD00148, zwecks "
        "Verwertung) und Verwerten (vorbereitend) mit Beschreibung „Lagerung der aufgeführten Abfälle "
        "(Elektroschrott im Auftrag der Stadt Offenbach) in geschlossenen Containern“; amtliches "
        "Fachbetrieberegister: Vorgang ZZFT01200023, Standortanlagen / 1 (Sammeln, Befördern) und / 2 "
        "(Lagern); Abruf 08.11.2026)\"]"
    ),
}

FM["he-limburg-65549-scheuer"] = {
    "street": "Im Elbboden 1",
    "postcode": "65549",
    "phone": "06431 93470",
    "email": "info@scheuer-limburg.de",
    "opening_hours": "Mo-Fr 07:30-12:00 und 13:00-17:00, Sa 08:00-12:00",
    "lat": "50.4001769",
    "lon": "8.052916",
    "status": "aktiv",
    "description": (
        "Betreiber-Impressum, amtliches Registerportal und amtliches Fachbetrieberegister nennen "
        "denselben Betrieb an derselben Anschrift: Scheuer GmbH, Im Elbboden 1, 65549 Limburg an der "
        "Lahn, HRB 3767 Amtsgericht Limburg, Geschäftsführer Karl-Heinz Scheuer. Das amtliche "
        "Fachbetrieberegister weist den Standort mit drei Anlagen aus (ZZHT00900091 / 1 Sammeln und "
        "Befördern, / 2 Lagern und Handeln, / 3 Behandeln); der Betreiber beschreibt einen "
        "firmeneigenen Schrottplatz auf 12.000 qm für sämtliche Arten von Metallen und Mischschrott. "
        "Ankaufkurse, Verkaufsliste und Gebührentafel sind nicht veröffentlicht."
    ),
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG (amtliches Fachbetrieberegister, Abruf 08.11.2026: "
        "Vorgangsnummer ZZHT00900091, Standort Im Elbboden 1, 65549 Limburg an der Lahn, "
        "Rechtsträger Scheuer GmbH; Standortanlage / 1 Tätigkeit Sammeln, Befördern; Standortanlage / 2 "
        "Tätigkeit Lagern, Handeln; Standortanlage / 3 Tätigkeit Behandeln; Zertifikatsnummer, "
        "Zertifizierungsorganisation, Ausstellungsdatum und Gültigkeitsdatum sind im Register nicht "
        "ausgewiesen)\"]"
    ),
}

FM["he-messel-64409-schrottler-eddie"] = {
    "street": "Zeilharderstraße 23a",
    "postcode": "64409",
    "description": (
        "Das amtliche Handelsregister identifiziert Schrottler Eddie GmbH, Amtsgericht Darmstadt "
        "HRB 96053, mit Geschäftsanschrift Zeilharderstraße 23a, 64409 Messel, Geschäftsführer Erwin "
        "Hahn; der Unternehmensgegenstand lautet wörtlich „Die Annahme, Abholung, Sortierung und "
        "Verwertung von Eisen und Metallen sowie alle damit zusammenhängende Geschäfte.“ Die "
        "importierte Hausnummer 25a ist damit durch das Register widerlegt. Eine eigene Website, "
        "aktuelle Öffnungszeiten oder eine Preisliste sind nicht belegt; eine Efb- oder "
        "AltfahrzeugV-Anerkennung ist im amtlichen Fachbetrieberegister nicht ausgewiesen. Die "
        "Hausnummer 23a ist in OSM nicht als Adressobjekt erfasst, deshalb wird keine Koordinate "
        "gesetzt."
    ),
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Schrottler Eddie GmbH oder für den Standort Zeilharderstraße 23a, 64409 Messel (Abruf "
        "08.11.2026; Namenssuche „Schrottler“ in der Route „Entsorgungsfachbetriebe“ 0 Treffer und in "
        "der Route „Altfahrzeugverwertung“ 0 Treffer; Radiusabfrage 3 km um die Dossieradresse 1 "
        "Efb-Treffer, das ist die fremde Südhessische Wertstoffrückgewinnungs GmbH, Recyclinganlage Am "
        "Bahnhof 14, 64409 Messel, ZZLT00500077 / 1, 0,6 km, und 0 AltfahrzeugV-Treffer; die "
        "Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, ein "
        "Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

FM["he-wiesbaden-65199-hubinger"] = {
    "street": "Schwarzenbergstraße 9a",
    "postcode": "65189",
    "city": "Wiesbaden",
    "trader_type": "autoverwertung",
    "lat": "50.0575906",
    "lon": "8.2581627",
    "description": (
        "Das amtliche Handelsregister weist die Autoverwertung Hübinger GmbH, Amtsgericht Wiesbaden "
        "HRB 17680, mit Geschäftsanschrift Schwarzenbergstraße 9a, 65189 Wiesbaden und Geschäftsführer "
        "Tobias Kemme aus; der Unternehmensgegenstand lautet wörtlich „Ist die Ausführung von "
        "Abschleppdiensten, Autoverwertung und Entsorgung sowie Ersatzteilverkauf.“ Der bereits "
        "dokumentierte Verzeichnislead „Hübinger Autoverwertung“ nennt dieselbe Anschrift. Die "
        "importierte Postleitzahl 65199 ist durch keine Quelle belegt. Das im Dossier geführte "
        "Abschlepp-Impressum gehört dagegen zur Hübinger Abschlepp- und Bergungsdienst e. K. "
        "(HRA 3884, Holzstraße 48A, 65197 Wiesbaden), also zu einer anderen Gesellschaft; die beiden "
        "Betriebe werden nicht zusammengeführt. Eine Efb- oder AltfahrzeugV-Anerkennung ist im "
        "amtlichen Fachbetrieberegister nicht ausgewiesen; Kontaktdaten und Öffnungszeiten bleiben "
        "unbelegt."
    ),
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "die Autoverwertung Hübinger GmbH, für Hübinger oder für den Standort Schwarzenbergstraße 9a, "
        "65189 Wiesbaden (Abruf 08.11.2026; Namenssuche „Hübinger“ in beiden Routen 0 Treffer; "
        "Radiusabfrage 3 km um die Dossieradresse 58 Efb-Treffer und 0 AltfahrzeugV-Treffer, darunter "
        "die Knettenbrech + Gurdulic Metallrecycling GmbH am Ferdinand-Knettenbrech-Weg 7a, 65205 "
        "Wiesbaden, ZZHT00300724 / 1 bis / 5, und rm-trade GmbH, Mainzer Straße 117, 65189 Wiesbaden, "
        "ZZGT00200043 / 1, sämtlich andere Rechtsträger ohne Bezug zu Hübinger; die "
        "Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, ein "
        "Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

# negative Registerbefunde fuer die uebrigen Dossiers
FM["he-wiesbaden-erbenheim-scholz-rico-schrott"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Scholz Rico Schrott oder für den Standort Rennbahnstr. 1, 65205 Wiesbaden-Erbenheim (Abruf "
        "08.11.2026; Namenssuche „Scholz“ in der Route „Entsorgungsfachbetriebe“ 179 Treffer und in "
        "der Route „Altfahrzeugverwertung“ 8 Treffer, sämtlich andere Rechtsträger ohne Bezug zu "
        "Wiesbaden-Erbenheim; Radiusabfrage 3 km um die Dossieradresse 15 Efb-Treffer, darunter die "
        "ELW Entsorgungsbetriebe der Landeshauptstadt Wiesbaden, Wertstoffhof Nordenstadt, Borsigstraße "
        "32, 65205 Wiesbaden, ZZET01700037 / 9, 1,6 km, sowie die DBW Recycling GmbH & Co. KG, Unterer "
        "Zwerchweg 110, 65205 Wiesbaden, ZZGE00100117 / 1 bis / 5, und 0 AltfahrzeugV-Treffer; die "
        "Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, ein "
        "Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

FM["he-langenselbold-63505-hontsch-maria"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Höntsch Maria, für Höntsch oder für den Standort Hanauer Str. 66b, 63505 Langenselbold (Abruf "
        "08.11.2026; Namenssuche „Höntsch“ in der Route „Entsorgungsfachbetriebe“ 0 Treffer und in der "
        "Route „Altfahrzeugverwertung“ 0 Treffer; Radiusabfrage 3 km um die Dossieradresse 7 Efb-Treffer, "
        "darunter die REMONDIS GmbH & Co. KG, BS Langenselbold, Am Seegraben 30, 63505 Langenselbold, "
        "ZZET01700114 / 9 bis / 11, 1,6 km, und die Weisgerber Umweltservice GmbH, Industriestraße 7, "
        "63505 Langenselbold, ZZEE00100653 / 7 und / 8, 1,9 km, sowie 0 AltfahrzeugV-Treffer; die "
        "Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, ein "
        "Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

FM["he-babenhausen-64832-kohlmann-frank"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Frank Kohlmann Metallhandel, für Frank Kohlmann oder für den Standort Am Fuchsberg 8, 64832 "
        "Babenhausen (Abruf 08.11.2026; Radiusabfrage 3 km um die Dossieradresse 4 Efb-Treffer, "
        "sämtlich die Vepro GmbH, Werk Babenhausen, Aschaffenburger Weg 18, 64832 Babenhausen, "
        "ZZGT00200093 / 1 bis / 4, 2,3 km, und 1 AltfahrzeugV-Treffer, das ist die Höreth "
        "Autoverwertung, Ostheimer Weg 13, 64832 Babenhausen, ZAGS00300011 / 1, 2,3 km; kein Treffer "
        "nennt Kohlmann oder den Am Fuchsberg 8; die Efb-Anerkennung nach § 56 KrWG und die "
        "AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein "
        "Nichtexistenzbeweis)\"]"
    ),
}

FM["he-schaafheim-64850-schrenk"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Schrenk oder für den Standort Daimlerstr. 6, 64850 Schaafheim (Abruf 08.11.2026; Namenssuche "
        "„Schrenk“ in der Route „Entsorgungsfachbetriebe“ 0 Treffer und in der Route "
        "„Altfahrzeugverwertung“ 0 Treffer; Radiusabfrage 3 km um die Dossieradresse 2 Efb-Treffer, "
        "das ist die Gerhard Höfling GmbH, Eichenweg 101, 64850 Schaafheim, ZZFT00300198 / 1 und / 2, "
        "1,9 km, und 0 AltfahrzeugV-Treffer; die Efb-Anerkennung nach § 56 KrWG und die "
        "AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein "
        "Nichtexistenzbeweis)\"]"
    ),
}

FM["he-wetter-firma-bajramaj"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Firma Bajramaj, für Bajramaj oder für den Standort Im Berntal 9, 35083 Wetter (Abruf "
        "08.11.2026; Namenssuche „Bajramaj“ in der Route „Entsorgungsfachbetriebe“ 0 Treffer und in der "
        "Route „Altfahrzeugverwertung“ 0 Treffer; Radiusabfrage 3 km um die Dossieradresse 0 Efb- und "
        "0 AltfahrzeugV-Treffer; die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung "
        "sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

FM["he-hanau-63452-guth"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Autoverwertung Guth GmbH, für Guth oder für den Standort Niddastraße 32, 63452 Hanau (Abruf "
        "08.11.2026; Namenssuche „Guth“ in der Route „Entsorgungsfachbetriebe“ 14 Treffer, sämtlich "
        "Namensvetter aus dem Wortbestandteil „Schüttguthandel“ wie die TSG Transporte & "
        "Schüttguthandel Greußen GmbH, die TSH Tiefbau- und Schüttguthandel GmbH & Co. KG und die "
        "Containerdienst & Schüttguthandel Steudel GmbH, und 0 Treffer in der Route "
        "„Altfahrzeugverwertung“; Radiusabfrage 3 km um die Dossieradresse 17 Efb-Treffer, darunter die "
        "Kurt Steding Entsorgungs-GmbH, Ruhrstraße 29, 63452 Hanau, ZZEE00300144 / 1 bis / 3, 0,3 km, "
        "sowie die Wüst Abschleppdienst GmbH, An der Brückengrube 12 12, 63452 Hanau, ZZHT00700226 / 1, "
        "0,2 km, und 0 AltfahrzeugV-Treffer; die Efb-Anerkennung nach § 56 KrWG und die "
        "AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist kein "
        "Nichtexistenzbeweis)\"]"
    ),
}

FM["he-oberursel-61440-lackner"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Auto Lackner, für Lackner oder für den Standort Pfeiffstr. 13, 61440 Oberursel (Abruf "
        "08.11.2026; Radiusabfrage 3 km um die Dossieradresse 4 Efb-Treffer, darunter die Behälterdienst "
        "Sommerkorn GmbH, An den Drei Hasen 43, 61440 Oberursel, ZZFT01200035 / 1 bis / 3, 1,9 km, und "
        "die Wilhelm Schütz GmbH & Co. KG, Niederurseler Straße 51, 61440 Oberursel, ZZHT00400103 / 1, "
        "2,5 km, sowie 0 AltfahrzeugV-Treffer; auf dem Grundstück der Pfeiffstraße 13 trägt das "
        "Register keinen Eintrag; die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung "
        "sind freiwillig, ein Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

FM["he-wiesbaden-biebrich-wiesbaden-dotzheim-bert-brecht-str-26"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Wintermeyer Romano Entsorgungsbetriebe, für Wintermeyer oder für den Standort Bert-Brecht-Str. "
        "26, 65201 Wiesbaden (Abruf 08.11.2026; Namenssuche „Wintermeyer“ in der Route "
        "„Entsorgungsfachbetriebe“ 0 Treffer und in der Route „Altfahrzeugverwertung“ 0 Treffer; "
        "Radiusabfrage 3 km um die Dossieradresse 9 Efb-Treffer, darunter die ELW Entsorgungsbetriebe "
        "der Landeshauptstadt Wiesbaden, Wertstoffhof Dotzheim, Willi-Werner-Straße 11, 65199 "
        "Wiesbaden-Dotzheim, ZZET01700037 / 7, 1,2 km, sowie 0 AltfahrzeugV-Treffer; die "
        "Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, ein "
        "Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

FM["he-hanau-lamboy-schrott-und-metalle-reinhardt"] = {
    "certifications": (
        "[\"Entsorgungsfachbetrieb nach § 56 KrWG: kein Eintrag im amtlichen Fachbetrieberegister für "
        "Schrott und Metalle Reinhardt, für Reinhardt oder für den Standort Friedrich-Engels-Straße 5, "
        "63452 Hanau-Lamboy (Abruf 08.11.2026; Namenssuche „Reinhardt“ in der Route "
        "„Entsorgungsfachbetriebe“ 7 Treffer, sämtlich andere Rechtsträger wie die R+S Reinhardt & "
        "Sela GmbH u.Co.KG in Ortenberg-Bleichenbach, die VEREVE Inh. Gina Reinhardt in Erdmannhausen "
        "und Ditzingen sowie die Reinhardt Rohstoffe GmbH in Berlin, und 0 Treffer in der Route "
        "„Altfahrzeugverwertung“; Radiusabfrage 3 km um die Dossieradresse 22 Efb-Treffer, darunter die "
        "DB Bahnbau Gruppe GmbH, Bereich Recycling Hanau, Robert-Blum-Straße 1, 63452 Hanau, "
        "ZZLE00200008 / 5 und / 6, 0,2 km, und die Kurt Steding Entsorgungs-GmbH, Ruhrstraße 29, 63452 "
        "Hanau, ZZEE00300144 / 1 bis / 3, 0,8 km, sowie 0 AltfahrzeugV-Treffer; die "
        "Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind freiwillig, ein "
        "Register-Nichteintrag ist kein Nichtexistenzbeweis)\"]"
    ),
}

# ------------------------------------------------------------------ Notizen
N = {}

N["he-wiesbaden-doringer-frank-schrotthandel"] = [
    "- [Recherche 08.11.2026: **Neue Namensvariante im amtlichen Fachbetrieberegister ohne Treffer; im "
    "Handelsregister stehen zwei „Döringer“-Namensvetter in Taunusstein, die nicht zum Dossierstandort "
    "gehören.** Das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/"
    "Chromium bedient) liefert am 08.11.2026 für die bislang ungeprüfte Namensvariante "
    "„Metallentsorgung“ in der Route „Entsorgungsfachbetriebe“ **0 Treffer** und in der Route "
    "„Altfahrzeugverwertung“ **0 Treffer**; die Standortabfrage mit 3 km um Fritz-Erler-Str. 47, 65207 "
    "Wiesbaden (Geocoder-Koordinaten 8.3380887/50.1068188) ergibt in der Route "
    "„Entsorgungsfachbetriebe“ **0 Treffer** und in der Route „Altfahrzeugverwertung“ **0 Treffer**; "
    "der Negativbefund vom 05.11.2026 (0/0 in beiden Routen für die Namensvarianten Döringer, Doeringer, "
    "Frank Döringer und die Rechtsformvarianten, 3/5/10-km-Radien) ist damit reproduziert und um die "
    "Variante „Metallentsorgung“ erweitert. Das amtliche Registerportal (online-handelsregister.de, "
    "Suchbegriff „Döringer“, Abruf 08.11.2026) weist zwei Treffer mit Registergericht Amtsgericht "
    "Wiesbaden aus: „Auto Döringer GmbH“ (HRB 16154) und „Werner Döringer KG“ (HRA 6055). Die "
    "Registerspiegel nennen für die GmbH den Sitz Taunusstein, Aarstraße 219, 65232 Taunusstein, den "
    "Geschäftsführer Harald Deutsch und den Gegenstand „Der An- und Verkauf von Kraftfahrzeugen, die "
    "Reparatur von Kraftfahrzeugen, der Handel mit Ersatzteilen und Zubehör, Vermittlung des An- und "
    "Verkaufs von Kraftfahrzeugen sowie der Betrieb einer Tankstelle“, für die HRA 6055 die heutige "
    "Bezeichnung „Döringer und Besier oHG“ mit Sitz Taunusstein, Schumannstraße 5, 65232 Taunusstein, "
    "und den Gesellschaftern Kai Döringer und Marina Besier. Beide sind **Namensvetter in Taunusstein** "
    "mit anderer Anschrift, anderer Rechtsform und anderem Gegenstand und werden **nicht** übernommen; "
    "eine Registergesellschaft für Fritz-Erler-Str. 47 in Wiesbaden-Medenbach ist nicht ausgewiesen. "
    "**ANKAUF:** keine Preise und keine Materialannahme belegt. **VERKAUF:** keine Preisliste. "
    "**GEBÜHREN:** keine Tarife. Offen bleiben der aktuelle Betriebszustand, ein lesbares Impressum und "
    "die Frage, ob die Mailadresse heute noch bedient wird. Quelle(n): "
    + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Metallentsorgung ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Metallentsorgung ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Fritz-Erler-Str.%2047%2C%2065207%20Wiesbaden%2C%20Deutschland%3B"
    "8.3380887%3B50.1068188%3B8.3801079%3B50.1337681%3B8.2960695%3B50.0798695&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Fritz-Erler-Str.%2047%2C%2065207%20Wiesbaden%2C%20Deutschland%3B"
    "8.3380887%3B50.1068188%3B8.3801079%3B50.1337681%3B8.2960695%3B50.0798695&dst=3&is=1&e=1 ; " + HRP +
    "D%C3%B6ringer ; " + HR_LIVE +
    "/hrb/hessen/ag-wiesbaden/16154/auto-doeringer-gmbh/strukturiert ; " + HR_LIVE +
    "/hra/hessen/ag-wiesbaden/6055/doeringer-und-besier-ohg/strukturiert (Abruf 08.11.2026).]",
]

N["he-wiesbaden-erbenheim-scholz-rico-schrott"] = [
    "- [Recherche 08.11.2026: **Der Namens- und Ortsbefund im amtlichen Fachbetrieberegister und im "
    "Handelsregister ist negativ, die Anschrift selbst ist als Adresse in OSM belegt.** Das amtliche "
    "Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) liefert "
    "am 08.11.2026 für die Namenssuche „Scholz“ in der Route „Entsorgungsfachbetriebe“ **179 Treffer** "
    "und in der Route „Altfahrzeugverwertung“ **8 Treffer**; die Ergebnislisten wurden vollständig "
    "gelesen, und **keine** Zeile nennt „Scholz Rico“ oder die Rennbahnstraße in Wiesbaden-Erbenheim "
    "(die Treffer sind die Scholz Recycling GmbH mit zahlreichen Standorten, die Autoservice Scholz "
    "GmbH in Crivitz, die Scholz Transporte GmbH in Nossen und die Carl-Heinz Scholz Rohstoffhandel "
    "GmbH & Co. KG in Brigachtal). Die Standortabfrage mit 3 km um Rennbahnstr. 1, 65205 "
    "Wiesbaden-Erbenheim (Nominatim-Koordinaten 8.3085013/50.0523699, PLZ-Abgleich 65205) liefert in der "
    "Route „Entsorgungsfachbetriebe“ **15 Treffer**, darunter die ELW Entsorgungsbetriebe der "
    "Landeshauptstadt Wiesbaden, Wertstoffhof Nordenstadt, Borsigstraße 32, 65205 Wiesbaden, "
    "ZZET01700037 / 9, 1,6 km, die ELW Deponie Dyckerhoffbruch, Deponiestraße 15, ZZET01700037 / 2 bis "
    "/ 6, 2,1 km, die DBW Recycling GmbH & Co. KG, Unterer Zwerchweg 110, 65205 Wiesbaden, "
    "ZZGE00100117 / 1 bis / 5, 2,5 km, und die COLLECTA Europe GmbH, Senefelderstraße 3a, ZZEE00100598 "
    "/ 1, 2,6 km; in der Route „Altfahrzeugverwertung“ **0 Treffer**. Das amtliche Registerportal "
    "(online-handelsregister.de, Abruf 08.11.2026) kennt für den Suchbegriff „Scholz“ in Wiesbaden "
    "**einen** Treffer, die Scholz & Volkmer GmbH (HRB 10899), und für „Scholz Rico“ **keinen** "
    "Treffer. OSM führt die Rennbahnstraße in Wiesbaden-Erbenheim mit der Postleitzahl 65205 und der "
    "Hausnummer 1 als eigenes Adressobjekt (way/434316979, `addr:housenumber`=1, "
    "`addr:street`=Rennbahnstraße, `addr:city`=Wiesbaden, `addr:postcode`=65205, live geprüft am "
    "08.11.2026); das Objekt trägt **weder** `name` **noch** `operator` und ordnet den Betrieb damit "
    "**nicht** zu, es bestätigt nur die Anschrift. **ANKAUF:** keine Kurse und keine belastbare "
    "Materialannahme. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Abhol- oder Räumungstarife "
    "belegt; die Portalangabe „kostenlose Schrottabholung“ bleibt Aggregatorinhalt. Die Efb- und die "
    "AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis. "
    "Quelle(n): " + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Scholz ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Scholz ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Rennbahnstr.%201%2C%2065205%20Wiesbaden%2C%20Deutschland%3B"
    "8.3085013%3B50.0523699%3B8.3504728%3B50.0793192%3B8.2665298%3B50.0254206&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Rennbahnstr.%201%2C%2065205%20Wiesbaden%2C%20Deutschland%3B"
    "8.3085013%3B50.0523699%3B8.3504728%3B50.0793192%3B8.2665298%3B50.0254206&dst=3&is=1&e=1 ; " + HRP +
    "Scholz ; " + HRP + "Scholz%20Rico ; " + NOMI +
    "Rennbahnstr.%201%2C%2065205%20Wiesbaden-Erbenheim%2C%20Deutschland (Abruf 08.11.2026).]",
]

N["he-wiesbaden-65199-hubinger"] = [
    "- [Recherche 08.11.2026: **Die Identität ist aufgelöst: das amtliche Handelsregister führt eine "
    "Autoverwertung Hübinger GmbH an der schon dokumentierten Verzeichnisanschrift.** Der Registerspiegel "
    "zum amtlichen Registerauszug (handelsregister.live, strukturierte Registerdaten, Abruf 08.11.2026) "
    "weist aus: **Autoverwertung Hübinger GmbH**, HRB 17680, Amtsgericht Wiesbaden, Sitz Wiesbaden, "
    "Dienst-/Geschäftsanschrift **Schwarzenbergstraße 9a, 65189 Wiesbaden**, Geschäftsführer **Tobias "
    "Kemme** (Wiesbaden), Stammkapital 50.000,00 EUR, Gesellschaftsvertrag vom 21.02.1994, erste "
    "Eintragung 26.07.1995, Sitzverlegung von Niedernhausen nach Wiesbaden durch Gesellschafterbeschluss "
    "vom 24.01.2023, letzte Eintragung 01.02.2023, Unternehmensgegenstand wörtlich „Ist die Ausführung "
    "von Abschleppdiensten, Autoverwertung und Entsorgung sowie Ersatzteilverkauf.“; das amtliche "
    "Registerportal (online-handelsregister.de, Suchbegriff „Hübinger“, Abruf 08.11.2026) bestätigt "
    "diesen Treffer. Der im Dossier geführte Verzeichnislead „Hübinger Autoverwertung“ nennt dieselbe "
    "Anschrift, damit sind Anschrift und Betriebsart durch zwei voneinander unabhängige Quellen belegt. "
    "Die Felder street, postcode und city wurden im Frontmatter auf diese Registeranschrift gesetzt, "
    "ebenso der Typ trader_type auf autoverwertung, weil der Registergegenstand die Autoverwertung "
    "ausdrücklich nennt; die importierte Postleitzahl 65199 ist dagegen durch keine Quelle belegt und "
    "wird als offene Frage festgehalten, ein Umzug ist nicht belegt. **Abgrenzung:** das im Dossier "
    "dokumentierte Abschlepp-Impressum (abschleppdienst-huebinger.de) gehört zur **Hübinger Abschlepp- "
    "und Bergungsdienst e. K.** (HRA 3884, Amtsgericht Wiesbaden, Holzstraße 48A, 65197 Wiesbaden), "
    "also zu einer anderen Gesellschaft; das Registerportal führt zusätzlich die Hübinger Abschlepp- und "
    "Bergungsdienst GmbH (HRB 35797) und die Hübinger Transporte GmbH (HRB 25347); die Gesellschaften "
    "werden nicht zusammengeführt. **Amtliches Fachbetrieberegister** (clientseitig gerenderte "
    "Vaadin-App, per Playwright/Chromium bedient, Abruf 08.11.2026): Namenssuche „Hübinger“ in beiden "
    "Routen **0 Treffer**; Standortabfrage 3 km um die Dossieradresse Schwarzenbergstraße 9a, 65189 "
    "Wiesbaden (Nominatim-Koordinaten 8.2581626/50.0575904) in der Route „Entsorgungsfachbetriebe“ "
    "**58 Treffer** und in der Route „Altfahrzeugverwertung“ **0 Treffer**; die 58 Zeilen gehören "
    "fremden Rechtsträgern, darunter die Knettenbrech + Gurdulic Metallrecycling GmbH am "
    "Ferdinand-Knettenbrech-Weg 7a, 65205 Wiesbaden, ZZHT00300724 / 1 bis / 5, und die rm-trade GmbH, "
    "Mainzer Straße 117, 65189 Wiesbaden, ZZGT00200043 / 1, 0,4 km; **keine** Zeile nennt Hübinger. "
    "OSM führt die Schwarzenbergstraße 9a, 65189 Wiesbaden als eigenes Adressobjekt "
    "(way/1175555938, `addr:housenumber`=9a, `addr:street`=Schwarzenbergstraße, `addr:postcode`=65189, "
    "`addr:city`=Wiesbaden, `building`=yes, live geprüft am 08.11.2026) mit 4 Knoten, Objekt-BBox "
    "8.2580719/50.0575093 bis 8.258252/50.0576723; der gesetzte Punkt 8.2581627/50.0575906 ist der "
    "Mittelwert dieser Knoten und liegt in der BBox; das Objekt trägt **weder** `name` **noch** "
    "`operator`, der Punkt belegt also die Anschrift, nicht den Betrieb. **ANKAUF:** keine Preise und "
    "keine Materialannahme belegt. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. Offen "
    "bleiben die Abgrenzung zur Seed-Postleitzahl 65199, Telefon, E-Mail und Öffnungszeiten; die im "
    "Registerspiegel genannte Telefonnummer gehört zum Registergericht, nicht zum Unternehmen, und "
    "wird deshalb **nicht** übernommen. Quelle(n): " + HR_LIVE +
    "/hrb/hessen/ag-wiesbaden/17680/autoverwertung-huebinger-gmbh/strukturiert ; " + HRP +
    "H%C3%BCbinger ; " + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=H%C3%BCbinger ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=H%C3%BCbinger ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Schwarzenbergstra%C3%9Fe%209a%2C%2065189%20Wiesbaden%2C%20"
    "Deutschland%3B8.2581626%3B50.0575904%3B8.3001820%3B50.0845400%3B8.2161432%3B50.0306408&dst=3&is=1&e=1 ; "
    + REG + "/Altfahrzeugverwertung/adr=Schwarzenbergstra%C3%9Fe%209a%2C%2065189%20Wiesbaden%2C%20"
    "Deutschland%3B8.2581626%3B50.0575904%3B8.3001820%3B50.0845400%3B8.2161432%3B50.0306408&dst=3&is=1&e=1 ; "
    + NOMI + "Schwarzenbergstra%C3%9Fe%209a%2C%2065189%20Wiesbaden%2C%20Deutschland ; "
    "https://api.openstreetmap.org/api/0.6/way/1175555938/full.json (Abruf 08.11.2026).]",
]

N["he-langenselbold-63505-hontsch-maria"] = [
    "- [Recherche 08.11.2026: **Weder im amtlichen Fachbetrieberegister noch im amtlichen Handelsregister "
    "ist ein Betrieb „Höntsch“ in Langenselbold ausgewiesen; die Hansastraße 66b ist als Adresse in OSM "
    "belegt.** Das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/"
    "Chromium bedient, Abruf 08.11.2026) liefert für die Namenssuche „Höntsch“ in der Route "
    "„Entsorgungsfachbetriebe“ **0 Treffer** und in der Route „Altfahrzeugverwertung“ **0 Treffer**; die "
    "Standortabfrage mit 3 km um Hanauer Str. 66b, 63505 Langenselbold (Nominatim-Koordinaten "
    "9.0296385/50.1716806, PLZ-Abgleich 63505) liefert in der Route „Entsorgungsfachbetriebe“ **7 "
    "Treffer**, darunter die REMONDIS GmbH & Co. KG, BS Langenselbold, Am Seegraben 30, 63505 "
    "Langenselbold, ZZET01700114 / 9 bis / 11, 1,6 km, die Weisgerber Umweltservice GmbH, "
    "Industriestraße 7, 63505 Langenselbold, ZZEE00100653 / 7 und / 8, 1,9 km, die LOGIPART Gesellschaft "
    "für Logistik & Dienstleistungen mbH, Hasselbachstraße 2, 63505 Langenselbold, ZZHT01000006 / 1, "
    "2,1 km, und die BWMK gGmbH, Am Bahnhof 5a, 63505 Langenselbold, ZZFT00700029 / 2, 2,2 km; in der "
    "Route „Altfahrzeugverwertung“ **0 Treffer**. Das amtliche Registerportal (online-handelsregister.de, "
    "Suchbegriff „Höntsch“, Abruf 08.11.2026) kennt nur zwei Unternehmen mit dem Namensbestandteil "
    "Höntsch, die **Baudeko Höntsch & Helfmann GmbH** (Hanau, HRB 3796) und die **Optik-Studio Höntsch "
    "GmbH** (Frankfurt am Main, HRB 28061); beide sind andere Branchen und werden **nicht** übernommen. "
    "OSM führt die Hansastraße 66b, 63505 Langenselbold als eigenes Adressobjekt mit Objektart "
    "commercial (way/289809942, live geprüft am 08.11.2026), ohne `name`- oder `operator`-Tag und damit "
    "ohne Betriebszuordnung. **ANKAUF:** keine Kurse und keine belastbare Annahme belegt. **VERKAUF:** "
    "keine Preisliste. **GEBÜHREN:** keine Tarife. Die Efb- und die AltfahrzeugV-Anerkennung sind "
    "freiwillig, ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis. Offen bleibt die Zuordnung "
    "des Namensvektors „Roland Höntsch“ an derselben Adresse. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=H%C3%B6ntsch ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=H%C3%B6ntsch ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Hanauer%20Str.%2066b%2C%2063505%20Langenselbold%2C%20Deutschland%3B"
    "9.0296385%3B50.1716806%3B9.0778561%3B50.1986301%3B8.9814209%3B50.1447311&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Hanauer%20Str.%2066b%2C%2063505%20Langenselbold%2C%20Deutschland%3B"
    "9.0296385%3B50.1716806%3B9.0778561%3B50.1986301%3B8.9814209%3B50.1447311&dst=3&is=1&e=1 ; " + HRP +
    "H%C3%B6ntsch ; " + NOMI +
    "Hanauer%20Stra%C3%9Fe%2066b%2C%2063505%20Langenselbold%2C%20Deutschland (Abruf 08.11.2026).]",
]

N["he-wetter-niederwetter-mattern-mario-metallrecycling"] = [
    "- [Recherche 08.11.2026: **Der Registerbefund vom 07.11.2026 ist reproduziert, der Negativbefund "
    "bleibt mit derselben Reichweite.** Das amtliche Fachbetrieberegister (clientseitig gerenderte "
    "Vaadin-App, per Playwright/Chromium bedient) liefert am 08.11.2026 für die Namenssuche „Mattern“ in "
    "der Route „Entsorgungsfachbetriebe“ **0 Treffer** und in der Route „Altfahrzeugverwertung“ **0 "
    "Treffer**; die Standortabfrage mit 3 km um Mildengrund 1, 35083 Wetter (Hessen)-Niederwetter "
    "(Geocoder-Koordinaten 8.7477540/50.8919439) liefert in der Route „Entsorgungsfachbetriebe“ **6 "
    "Treffer**, und zwar **ausschließlich** die Scholz Recycling GmbH, Standort Lahntal, "
    "Niederwettersche Straße 4, 35094 Lahntal, ZZEE00300308 / 1 bis / 6, 2,1 km (Sammeln und Befördern, "
    "Schredderanlage, Schrottlagerung, Behandlung nicht gefährlicher Abfälle, zeitweilige Lagerung AT 7, "
    "Lagern und Behandeln); in der Route „Altfahrzeugverwertung“ **0 Treffer**. Damit ist der Befund vom "
    "07.11.2026 unveraendert bestaetigt: die Gelbe-Seiten-Adresse Mildengrund 1 ist **kein** "
    "zertifizierter Standort, und **keine** der sechs Registerzeilen nennt Mario Mattern. Reichweite: "
    "die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein "
    "Register-Nichteintrag ist **kein** Nachweis für die Nichtexistenz des Betriebs; die Umzugsfrage "
    "zwischen Wetter (Mildengrund 1) und Marburg-Michelbach (Zum Kalkberg 3) bleibt offen. **ANKAUF:** "
    "kein Preis und kein Auszahlungsmodell verifiziert, die Portalrubriken bleiben Verzeichnisangaben. "
    "**VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Mattern ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Mattern ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Mildengrund%201%2C%2035083%20Wetter%2C%20Deutschland%3B"
    "8.7477540%3B50.8919439%3B8.8377540%3B50.9419439%3B8.6577540%3B50.8419439&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Mildengrund%201%2C%2035083%20Wetter%2C%20Deutschland%3B"
    "8.7477540%3B50.8919439%3B8.8377540%3B50.9419439%3B8.6577540%3B50.8419439&dst=3&is=1&e=1 (Abruf "
    "08.11.2026).]",
]

N["he-babenhausen-64832-kohlmann-frank"] = [
    "- [Recherche 08.11.2026: **Die Standortabfrage im amtlichen Fachbetrieberegister zeigt nur "
    "Fremdbetriebe; die beiden Namensvetter „Frank Kohlmann“ sitzen in Offenbach, nicht in Babenhausen.** "
    "Das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium "
    "bedient) liefert am 08.11.2026 für die Standortabfrage mit 3 km um Am Fuchsberg 8, 64832 "
    "Babenhausen (Nominatim-Koordinaten 8.9643466/49.9749685, PLZ-Abgleich 64832) in der Route "
    "„Entsorgungsfachbetriebe“ **4 Treffer**, das ist die Vepro GmbH, Werk Babenhausen, Aschaffenburger "
    "Weg 18, 64832 Babenhausen, ZZGT00200093 / 1 bis / 4, 2,3 km, und in der Route "
    "„Altfahrzeugverwertung“ **1 Treffer**, das ist die Höreth Autoverwertung, Ostheimer Weg 13, 64832 "
    "Babenhausen, ZAGS00300011 / 1, 2,3 km; **keine** Zeile nennt Kohlmann oder den Am Fuchsberg 8. Das "
    "amtliche Registerportal (online-handelsregister.de, Suchbegriff „Frank Kohlmann“, Abruf 08.11.2026) "
    "führt zwei Unternehmen mit diesem Namen, **beide in Offenbach am Main**: „Frank Kohlmann“ (HRA 1469) "
    "und „Frako - Metall Frank Kohlmann“ (HRA 1042); sie sind **Namensvetter** mit anderer Stadt und "
    "werden **nicht** übernommen, der im Dossier genannte Firmeneintrag „Frank Kohlmann Metallbau "
    "Kohlmann“ (HRA 31927, Amtsgericht Darmstadt) bleibt ohne erreichbare eigene Website. OSM führt den "
    "Am Fuchsberg 8, 64832 Babenhausen als eigenes Adressobjekt (way/208691008, live geprüft am "
    "08.11.2026); es trägt **weder** `name` **noch** `operator` und ordnet den Betrieb damit **nicht** zu. "
    "**ANKAUF:** keine verifizierten Kurse. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. "
    "Die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist **kein** "
    "Nichtexistenzbeweis. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/adr=Am%20Fuchsberg%208%2C%2064832%20Babenhausen%2C%20Deutschland%3B"
    "8.9643466%3B49.9749685%3B9.0125641%3B50.0019180%3B8.9161291%3B49.9480190&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Am%20Fuchsberg%208%2C%2064832%20Babenhausen%2C%20Deutschland%3B"
    "8.9643466%3B49.9749685%3B9.0125641%3B50.0019180%3B8.9161291%3B49.9480190&dst=3&is=1&e=1 ; " + HRP +
    "Frank%20Kohlmann ; " + NOMI +
    "Am%20Fuchsberg%208%2C%2064832%20Babenhausen%2C%20Deutschland (Abruf 08.11.2026).]",
]

N["he-offenbach-am-main-fred-de-snoo-rohstoffe-schrott-und-metal"] = [
    "- [Recherche 08.11.2026: **Das 2026 ausgestellte Efb-Zertifikat wurde im Original gelesen und mit dem "
    "amtlichen Fachbetrieberegister abgeglichen: die Betreiberanschrift und der Standort sind jetzt durch "
    "eine amtliche Quelle belegt, die Stammdaten wurden entsprechend gefüllt.** Das amtliche "
    "Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) liefert am "
    "08.11.2026 für die Namenssuche „de Snoo“ in der Route „Entsorgungsfachbetriebe“ **2 Treffer** und "
    "in der Route „Altfahrzeugverwertung“ **0 Treffer**; die beiden Zeilen lauten wörtlich „Fred de Snoo "
    "Rohstoffe“, **Lämmerspieler Weg 158, 63075 Offenbach**, Vorgang **ZZFT01200023**, Standortanlage "
    "**/ 1** Tätigkeit „Sammeln, Befördern“ und Standortanlage **/ 2** Tätigkeit „Lagern“; die "
    "Standortabfrage mit 3 km um die Dossieradresse (Nominatim-Koordinaten 8.8018503/50.1077178, "
    "PLZ-Abgleich 63075) liefert in der Route „Entsorgungsfachbetriebe“ **18 Treffer**, darunter die "
    "beiden genannten Zeilen mit **0,0 km** Entfernung; die übrigen 16 Zeilen gehören fremden "
    "Rechtsträgern (Robert Zeller GmbH & Co. KG, Im großen Ahl 53, Saubermacher Recycling GmbH, ESO "
    "Stadtservice GmbH, ESO Offenbacher Dienstleistungsgesellschaft mbH, Knettenbrech + Gurdulic "
    "Metallrecycling GmbH, Philipp Blam & Sohn GmbH und pal plast GmbH). **Zertifikat (im Original "
    "gelesen):** Das auf der Betreiberseite veröffentlichte Zertifikat „Zertifikat Nr. 32970032“ der TÜV "
    "Technischen Überwachung Hessen GmbH, Robert-Bosch-Straße 16, 64293 Darmstadt, Vorgangsnummer "
    "**ZZFT012000239009**, Folgezertifizierung, **2 Anlagen**, gültig bis **07.09.2027**, Prüfungsdatum "
    "31.03.2026, Ausstellungsdatum 05.06.2026, Rechtsträger laut Zertifikat „Fred de Snoo Rohstoffe“, "
    "Lämmerspieler Weg 158, 63075 Offenbach, **HRA 42548**, Registergericht Offenbach am Main. "
    "**Anlage 1** (Standort Lämmerspieler Weg 158): Tätigkeit Sammeln (Kennnummer nach § 28 NachwV "
    "F34T115545, nur deutschlandweit) und Befördern (F34T115545, nur deutschlandweit), Beschreibung "
    "„Sammeln und Befördern von überwiegend Metallschrotten von und zur Anlage“. **Anlage 2** (gleicher "
    "Standort): Lagern (F34RD00148, zwecks Verwertung) und Verwerten (vorbereitend), Beschreibung „Lagerung "
    "der aufgeführten Abfälle (Elektroschrott im Auftrag der Stadt Offenbach) in geschlossenen "
    "Containern“. Das Feld certifications wurde im Frontmatter mit diesen Angaben gefüllt, und die "
    "Felder street, postcode, phone, email, opening_hours sowie die Koordinaten lat/lon wurden im "
    "Frontmatter auf die belegten Werte gesetzt; das Impressum nennt wörtlich „Fred De Snoo Rohstoffe "
    "e.K. / Schrott und Metallhandel, Containerdienst, Entsorgungsfachbetrieb / Lämmerspieler Weg 158 / "
    "63075 Offenbach“ mit „Telefon: +49 69 856112“ und „E-Mail: info@freddesnoo.de“ und "
    "„Umsatzsteuer-Identifikationsnummer gemäß § 27 a Umsatzsteuergesetz: DE315650855“. Die "
    "Betreiber-Ausnahme ist damit erfüllt, weil Impressum, Zertifikat und amtliches Register den "
    "Betreiber mit Registernummer und Anschrift übereinstimmend belegen; der `status` wurde auf aktiv "
    "gesetzt. **GEO:** OSM führt die Lämmerspieler Weg 158, 63075 Offenbach am Main als eigenes "
    "Adressobjekt (way/190947391, `addr:housenumber`=158, `addr:street`=Lämmerspieler Weg, "
    "`addr:postcode`=63075, `addr:city`=Offenbach am Main, `building`=yes, live geprüft am 08.11.2026) "
    "mit 6 Knoten, Objekt-BBox 8.8017968/50.1076811 bis 8.8019038/50.1077611; der gesetzte Punkt "
    "8.8018644/50.1077259 ist der Mittelwert dieser Knoten und liegt in der BBox; das Objekt trägt "
    "**weder** `name` **noch** `operator`, der Punkt belegt also die Anschrift, nicht den Betrieb. "
    "**ANKAUF:** die Betreiberseite beschreibt die Vergütung nach Analyse, Verwiegung und Ausladen "
    "„erhalten Sie an der Anmeldung eine Vergütung für Ihre Metalle“, nennt aber **keine** Zahlenkurse; "
    "**VERKAUF:** keine bezifferte Preisliste; **GEBÜHREN:** Abrollcontainer von 11 bis 40 cbm und "
    "Absetzmulden von 5 bis 15 cbm sind Kapazitäten, keine Preise, und es wird kein Tarif genannt. "
    "Quelle(n): " + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=de%20Snoo ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=de%20Snoo ; " + REG +
    "/Entsorgungsfachbetriebe/adr=L%C3%A4mmerspieler%20Weg%20158%2C%2063075%20Offenbach%20am%20Main%2C%20"
    "Deutschland%3B8.8018503%3B50.1077178%3B8.8438690%3B50.1346673%3B8.7598316%3B50.0807683&dst=3&is=1&e=1 ; "
    "https://www.freddesnoo.de/wp-content/uploads/2026/08/"
    "2026-Zertifikat-Entsorgungsfachbetrieb-ZZFT012000239009_EFB-Zertifikat-1.webp ; "
    "https://www.freddesnoo.de/wp-content/uploads/2026/08/"
    "2026-Zertifikat-Entsorgungsfachbetrieb-ZZFT012000239009_EFB-Zertifikat-2.webp ; "
    "https://www.freddesnoo.de/wp-content/uploads/2026/08/"
    "2026-Zertifikat-Entsorgungsfachbetrieb-ZZFT012000239009_EFB-Zertifikat-5.webp ; "
    "https://www.freddesnoo.de/impressum/ ; https://www.freddesnoo.de/schrott-und-metallhandel/ ; "
    "https://api.openstreetmap.org/api/0.6/way/190947391/full.json (Abruf 08.11.2026).]",
]

N["he-messel-64409-schrottler-eddie"] = [
    "- [Recherche 08.11.2026: **Das amtliche Handelsregister bestätigt die Anschrift und den "
    "Unternehmensgegenstand; die importierte Hausnummer 25a ist widerlegt, OSM kennt 23a nicht.** Das "
    "amtliche Registerportal (online-handelsregister.de, Suchbegriff „Schrottler Eddie“, Abruf 08.11.2026) "
    "weist aus: **Schrottler Eddie GmbH**, Amtsgericht Darmstadt, **HRB 96053**. Der Registerspiegel zum "
    "amtlichen Registerauszug (handelsregister.live, strukturierte Registerdaten, Abruf 08.11.2026) nennt "
    "Sitz Messel, **Dienst-/Geschäftsanschrift Zeilharderstraße 23a, 64409 Messel**, Geschäftsführer "
    "**Erwin Hahn** (Weiterstadt), Stammkapital 25.000,00 EUR, Gesellschaftsvertrag vom 25.10.2016, "
    "letzte Eintragung 28.11.2016, und den Unternehmensgegenstand wörtlich „Die Annahme, Abholung, "
    "Sortierung und Verwertung von Eisen und Metallen sowie alle damit zusammenhängende Geschäfte.“; "
    "die Felder street und postcode wurden im Frontmatter auf diese Registeranschrift gesetzt, die "
    "importierte Hausnummer 25a aus dem Seed bleibt als **widerlegt** dokumentiert. **Amtliches "
    "Fachbetrieberegister** (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient, Abruf "
    "08.11.2026): Namenssuche „Schrottler“ in der Route „Entsorgungsfachbetriebe“ **0 Treffer** und in "
    "der Route „Altfahrzeugverwertung“ **0 Treffer**; Standortabfrage 3 km um Zeilharderstraße 23a, "
    "64409 Messel (Nominatim-Koordinaten 8.7457383/49.9243273, PLZ-Abgleich 64409) in der Route "
    "„Entsorgungsfachbetriebe“ **1 Treffer**, das ist die Südhessische Wertstoffrückgewinnungs GmbH, "
    "Recyclinganlage Am Bahnhof 14, 64409 Messel, ZZLT00500077 / 1, 0,6 km, und in der Route "
    "„Altfahrzeugverwertung“ **0 Treffer**; **keine** Zeile nennt Schrottler. **GEO:** die OSM-Abfrage "
    "des Kachelrasters 8.7400/49.9190 bis 8.7520/49.9295 (map.json, live abgerufen am 08.11.2026) listet "
    "für die Zeilharder Straße in Messel die Adressobjekte mit den Hausnummern 2, 2A, 4, 6, 7, 8, 9, 10, "
    "12, 12a, 13, 15, 16, 16a, 17, 18, 19, 20, 21, 22, 23, 24, 25, 29 und 31; **weder „23a“ noch „25a“** "
    "ist als Adressobjekt erfasst, die Register-Hausnummer 23a ist damit in OSM **nicht** adressgenau "
    "belegt, deshalb wird **keine** Koordinate gesetzt. **ANKAUF:** keine Preise, kein "
    "Auszahlungsmodell und keine Annahmegebühren belegt; der Registergegenstand ist eine "
    "Betriebsbeschreibung und **kein** Preisangebot. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine "
    "Tarife. Offen bleiben der aktuelle Betriebszustand, eine Betreiberwebsite und die "
    "Abweichung zwischen Registerhausnummer 23a und der Seed-Angabe 25a. Quelle(n): " + HRP +
    "Schrottler%20Eddie ; " + HR_LIVE +
    "/hrb/hessen/ag-darmstadt/96053/schrottler-eddie-gmbh/strukturiert ; " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrottler ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrottler ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Zeilharderstra%C3%9Fe%2023a%2C%2064409%20Messel%2C%20Deutschland%3B"
    "8.7457383%3B49.9243273%3B8.7877569%3B49.9512768%3B8.7037197%3B49.8973778&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Zeilharderstra%C3%9Fe%2023a%2C%2064409%20Messel%2C%20Deutschland%3B"
    "8.7457383%3B49.9243273%3B8.7877569%3B49.9512768%3B8.7037197%3B49.8973778&dst=3&is=1&e=1 ; "
    "https://api.openstreetmap.org/api/0.6/map.json?bbox=8.7400,49.9190,8.7520,49.9295 ; " + NOMI +
    "Zeilharder%20Stra%C3%9Fe%2023a%2C%2064409%20Messel%2C%20Deutschland (Abruf 08.11.2026).]",
]

N["he-frankfurt-60388-loew-alfred"] = [
    "- [Recherche 08.11.2026: **Der Negativbefund wird mit einer dritten Namensvariante und einer "
    "gemessenen Ortsabfrage reproduziert; ein Bezug zu „Loew Alfred“ ist nicht ausgewiesen.** Das amtliche "
    "Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) liefert "
    "am 08.11.2026 für die Namenssuche „Loew“ in der Route „Entsorgungsfachbetriebe“ **0 Treffer** und in "
    "der Route „Altfahrzeugverwertung“ **0 Treffer**; die Standortabfrage mit 3 km um 60388 Frankfurt am "
    "Main (Geocoder-Koordinaten 8.763249/50.157394) liefert in der Route „Entsorgungsfachbetriebe“ **10 "
    "Treffer**, darunter die Milan Milosavljevic, Rangenbergstraße 38, 60388 Frankfurt, ZZFT00300193 / 1, "
    "1,2 km, die ELVIVAL GmbH, Vilbeler Landstraße 73, 60388 Frankfurt am Main, ZZIT00700515 / 1, 1,6 km, "
    "die FES GmbH BS Sachsenhausen, Seehofstraße 48, 60389 Frankfurt am Main, ZZEE00200089 / 12 und / 13, "
    "2,8 km, die GCR Green Commodity Recycling GmbH, Borsigallee 18, 60388 Frankfurt am Main, "
    "ZZEE00300324 / 1, 2,8 km, und die Helmut Haug GmbH, Gwinnerstraße 15-17, 60388 Frankfurt, "
    "ZZHT00300139 / 1 bis / 3, 2,9 km; in der Route „Altfahrzeugverwertung“ **0 Treffer**. **Keine** Zeile "
    "nennt Loew oder Alfred; die Namenssuche ist eine Teilstring-Suche und wäre zeichengenau nicht "
    "schlagend. Der Negativbefund ist **kein** Nachweis der Nichtexistenz, die Seed-Klassifikation "
    "„Klein-AV?“ bleibt offen. Die im Dossier dokumentierte amtliche Handelsregister-Suche vom 29.10.2026 "
    "(Suchbegriff „Löw“, in Frankfurt am Main nur die Löwen-Apotheke Corinna Pasche-Colloseus e.K., "
    "HRA 23491) bleibt davon unberührt. **ANKAUF:** keine Preise. **VERKAUF:** keine Verkaufsliste. "
    "**GEBÜHREN:** keine Gebührenangaben. **GEO:** der im Dossier dokumentierte Altpunkt "
    "50.119404878048776/8.653973170731708 (Senckenberganlage, way/42681539, außerhalb des PLZ-Gebiets "
    "60388) bleibt unverändert, ein Ersatzpunkt wäre geraten. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Loew ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Loew ; " + REG +
    "/Entsorgungsfachbetriebe/adr=60388%20Frankfurt%20am%20Main%2C%20Deutschland%3B8.763249%3B50.157394"
    "%3B8.8052686%3B50.1843434%3B8.7212294%3B50.1304446&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=60388%20Frankfurt%20am%20Main%2C%20Deutschland%3B8.763249%3B50.157394"
    "%3B8.8052686%3B50.1843434%3B8.7212294%3B50.1304446&dst=3&is=1&e=1 (Abruf 08.11.2026).]",
]

N["he-schaafheim-64850-schrenk"] = [
    "- [Recherche 08.11.2026: **Weder Register noch Fachbetrieberegister führen einen Schrenk-Betrieb in "
    "Schaafheim; die Identitätsfrage bleibt beim Verzeichnislead.** Das amtliche Fachbetrieberegister "
    "(clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) liefert am 08.11.2026 für die "
    "Namenssuche „Schrenk“ in der Route „Entsorgungsfachbetriebe“ **0 Treffer** und in der Route "
    "„Altfahrzeugverwertung“ **0 Treffer**; die Standortabfrage mit 3 km um Daimlerstr. 6, 64850 "
    "Schaafheim (Nominatim-Koordinaten 8.9984064/49.9309340, PLZ-Abgleich 64850) liefert in der Route "
    "„Entsorgungsfachbetriebe“ **2 Treffer**, das ist die Gerhard Höfling GmbH, Unternehmenssitz Eichenweg "
    "101, 64850 Schaafheim, ZZFT00300198 / 1 und / 2, 1,9 km, und in der Route „Altfahrzeugverwertung“ **0 "
    "Treffer**. Das amtliche Registerportal (online-handelsregister.de, Suchbegriff „Schrenk“, Abruf "
    "08.11.2026) listet über 60 Unternehmen; **keines** sitzt in Schaafheim oder im Landkreis "
    "Darmstadt-Dieburg, die nächstliegenden sind die Elektroservice Schrenk GmbH (Offenbach am Main, HRB "
    "40909), die Peter M. Schrenk Import und Export - Marketing GmbH (Frankfurt am Main, HRB 14670) sowie "
    "die Schrenk GmbH in München (HRB 153825) und Stuttgart (HRB 351795); sie sind **Namensvetter** "
    "anderer Branchen und werden **nicht** übernommen. OSM führt die Daimlerstraße 6, 64850 Schaafheim als "
    "eigenes Adressobjekt (way/708250341, live geprüft am 08.11.2026), ohne `name`- oder "
    "`operator`-Tag und damit ohne Betriebszuordnung. Die im Dossier dokumentierte Betreiberseite "
    "asd-schrenk.de (Torsten Schrenk, Mislag 17, 21775 Ihlienworth) bleibt unverändert ein Lead für eine "
    "mögliche Standort- oder Identitätskette. **ANKAUF:** keine Schrottkurse. **VERKAUF:** keine "
    "Teile- oder Materialpreisliste. **GEBÜHREN:** keine schrottbezogenen Tarife. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrenk ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrenk ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Daimlerstra%C3%9Fe%206%2C%2064850%20Schaafheim%2C%20Deutschland%3B"
    "8.9984064%3B49.9309340%3B9.0466240%3B49.9578835%3B8.9501888%3B49.9039845&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Daimlerstra%C3%9Fe%206%2C%2064850%20Schaafheim%2C%20Deutschland%3B"
    "8.9984064%3B49.9309340%3B9.0466240%3B49.9578835%3B8.9501888%3B49.9039845&dst=3&is=1&e=1 ; " + HRP +
    "Schrenk ; " + NOMI +
    "Daimlerstra%C3%9Fe%206%2C%2064850%20Schaafheim%2C%20Deutschland (Abruf 08.11.2026).]",
]

N["he-limburg-65549-scheuer"] = [
    "- [Recherche 08.11.2026: **Das amtliche Fachbetrieberegister weist den Betrieb an genau dieser "
    "Anschrift aus; Betreiber-Impressum, Registerportal und Register stimmen überein, die Stammdaten "
    "wurden gefüllt.** Das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per "
    "Playwright/Chromium bedient) liefert am 08.11.2026 für die Namenssuche „Scheuer“ in der Route "
    "„Entsorgungsfachbetriebe“ **4 Treffer**, davon **3** für **Scheuer GmbH**, **Im Elbboden 1, 65549 "
    "Limburg**, Vorgang **ZZHT00900091**, Standortanlage **/ 1** Tätigkeit „Sammeln, Befördern“, **/ 2** "
    "Tätigkeit „Lagern, Handeln“ und **/ 3** Tätigkeit „Behandeln“; der vierte Treffer ist der Namensvetter "
    "Hausdienst Scheuermann GmbH (Kollaustraße 148, 22453 Hamburg, ZZBT00500040 / 1); in der Route "
    "„Altfahrzeugverwertung“ **0 Treffer**. Die Standortabfrage mit 3 km um die Dossieradresse (Nominatim-"
    "Koordinaten 8.0529371/50.4001871, PLZ-Abgleich 65549) liefert in der Route „Entsorgungsfachbetriebe“ "
    "**18 Treffer**, darunter die drei genannten Scheuer-Zeilen mit **0,0 km** Entfernung, und in der "
    "Route „Altfahrzeugverwertung“ **0 Treffer**. Das amtliche Registerportal (online-handelsregister.de, "
    "Suchbegriff „Scheuer“, Abruf 08.11.2026) bestätigt „Scheuer GmbH, Limburg • HRB 3767“. Das "
    "Betreiber-Impressum (live gelesen am 08.11.2026) nennt wörtlich „Scheuer GmbH / Im Elbboden 1 / 65549 "
    "Limburg an der Lahn / Tel.: +49 (0) 6431 93470 / Fax: +49 (0) 6431 934780 / Mail: "
    "info@scheuer-limburg.de“, „Eingetragen im Handelsregister des Amtsgerichtes Limburg, HRB 3767“, "
    "„Geschäftsführer: Karl-Heinz Scheuer“ und „Umsatzsteuer-Identifikationsnummer gemäß § 27 a "
    "Umsatzsteuergesetz: DE814324216“; die eigene Öffnungszeiten-Seite nennt wörtlich „Die Öffnungszeiten "
    "unseres Schrottplatzes sind Montag bis Freitag von 7.30-12.00 Uhr sowie von 13.00 - 17.00 Uhr und "
    "Samstags von 8.00 - 12.00 Uhr“, die Schrotthandelseite wörtlich „Auf unserem 12.000 qm großen "
    "Firmengelände befindet sich ein firmeneigener Schrottplatz für sämtliche Arten von Metallen und "
    "Mischschrott“. Die Felder street, postcode, phone, email und opening_hours sowie die Koordinaten "
    "lat/lon wurden im Frontmatter auf diese belegten Werte gesetzt und das Feld certifications mit dem "
    "Registerbefund gefüllt; der `status` wurde auf aktiv gesetzt, weil das amtliche Register den Betrieb "
    "an dieser Anschrift mit aktueller Anerkennung führt. **GEO:** OSM führt den Im Elbboden 1, 65549 "
    "Limburg an der Lahn als eigenes Adressobjekt (way/1111723976, `addr:housenumber`=1, "
    "`addr:street`=Im Elbboden, `addr:city`=Limburg an der Lahn, `addr:suburb`=Limburg, "
    "`addr:country`=DE, `building`=yes, live geprüft am 08.11.2026) mit 5 Knoten, Objekt-BBox "
    "8.0527744/50.4000926 bis 8.0530997/50.4002816; der gesetzte Punkt 8.052916/50.4001769 ist der "
    "Mittelwert dieser Knoten und liegt in der BBox; das Objekt trägt **weder** `name` **noch** "
    "`operator`, der Punkt belegt also die Anschrift, nicht den Betrieb. **ANKAUF:** keine numerische "
    "Kursliste und keine ausdrückliche Vergütungszusage. **VERKAUF:** keine Preisliste. **GEBÜHREN:** kein "
    "Tarif; die 24-Stunden-Angabe gilt dem Abschleppdienst, nicht dem Schrottplatz. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Scheuer ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Scheuer ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Im%20Elbboden%201%2C%2065549%20Limburg%20an%20der%20Lahn%2C%20"
    "Deutschland%3B8.0529371%3B50.4001871%3B8.0979547%3B50.4271366%3B8.0079195%3B50.3732376&dst=3&is=1&e=1 ; "
    + REG + "/Altfahrzeugverwertung/adr=Im%20Elbboden%201%2C%2065549%20Limburg%20an%20der%20Lahn%2C%20"
    "Deutschland%3B8.0529371%3B50.4001871%3B8.0979547%3B50.4271366%3B8.0079195%3B50.3732376&dst=3&is=1&e=1 ; "
    + HRP + "Scheuer ; https://www.scheuer-autokrane.de/impressum/impressum.html ; "
    "https://www.scheuer-autokrane.de/leistungen/schrotthandel.html ; "
    "https://www.scheuer-autokrane.de/kontakt/oeffnungszeiten.html ; "
    "https://api.openstreetmap.org/api/0.6/way/1111723976/full.json (Abruf 08.11.2026).]",
]

N["he-bad-homburg-schrott-hg"] = [
    "- [Recherche 08.11.2026: **Der Negativbefund im amtlichen Fachbetrieberegister ist mit der Namenssuche "
    "und einer 3-km-Ortsabfrage reproduziert; die Beleglage ändert sich nicht.** Das amtliche "
    "Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) liefert am "
    "08.11.2026 für die Namenssuche „Schrott-HG“ in der Route „Entsorgungsfachbetriebe“ **0 Treffer** und "
    "in der Route „Altfahrzeugverwertung“ **0 Treffer**; die Standortabfrage mit 3 km um Oberste Gärten 8, "
    "61350 Bad Homburg (Geocoder-Koordinaten 8.600928/50.2419619) liefert in der Route "
    "„Entsorgungsfachbetriebe“ **1 Treffer**, das ist die Onyx Rohr- und Kanal-Service GmbH, "
    "Industriestraße 10, 61440 Oberursel, ZZFT00500126 / 5, 2,5 km, also ein Fremdbetrieb ohne Bezug zur "
    "Bad-Homburger Anschrift, und in der Route „Altfahrzeugverwertung“ **0 Treffer**. Das Ergebnis "
    "entspricht dem Befund vom 07.11.2026 (Namenssuche 0/0, 3-km-Ortsabfrage 1 Efb-Treffer, 22 Efb-Treffer "
    "im 10-km-Radius, 0 AltfahrzeugV-Treffer, Oberurseler Anschrift ebenfalls ohne Eintrag). Zertifikats"
    "nummer, Vorgangsnummer, Zertifizierungsorganisation, Ausstellungsdatum, Gültigkeitsdatum, "
    "Anlagenanzahl und zertifizierte Tätigkeit bleiben **nicht ausgewiesen**; das Feld certifications bleibt "
    "unverändert. Reichweite: die Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind "
    "**freiwillig**, ein Register-Nichteintrag ist **kein** Nachweis für Nichtexistenz, Betriebsschließung "
    "oder fehlende Zertifizierungspflicht. **ANKAUF:** keine Preise, die sichtbaren Kurse auf "
    "schrottradar.de gehören anderen Betrieben. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife, "
    "„kostenlose Abholung“ bleibt historische Betreiberwerbung. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrott-HG ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrott-HG ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Oberste%20G%C3%A4rten%208%2C%2061350%20Bad%20Homburg%2C%20Deutschland%3B"
    "8.600928%3B50.2419619%3B8.6479467%3B50.2689114%3B8.5539093%3B50.2150124&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Oberste%20G%C3%A4rten%208%2C%2061350%20Bad%20Homburg%2C%20Deutschland%3B"
    "8.600928%3B50.2419619%3B8.6479467%3B50.2689114%3B8.5539093%3B50.2150124&dst=3&is=1&e=1 (Abruf "
    "08.11.2026).]",
]

N["he-wetter-firma-bajramaj"] = [
    "- [Recherche 08.11.2026: **Das amtliche Fachbetrieberegister kennt weder die Firma noch den Standort; "
    "der amtliche Handelsregisterbefund vom 29.10.2026 ist reproduziert.** Das amtliche "
    "Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) liefert am "
    "08.11.2026 für die Namenssuche „Bajramaj“ in der Route „Entsorgungsfachbetriebe“ **0 Treffer** und in "
    "der Route „Altfahrzeugverwertung“ **0 Treffer**; die Standortabfrage mit 3 km um Im Berntal 9, 35083 "
    "Wetter (Geocoder-Koordinaten 8.7362641/50.9034911) liefert in **beiden** Routen **0 Treffer**. Das "
    "amtliche Registerportal (online-handelsregister.de, Suchbegriff „Bajramaj“, Abruf 08.11.2026) kennt "
    "weder eine Gesellschaft „Firma Bajramaj“ noch eine Gesellschaft mit Namensbestandteil Bajramaj in "
    "Wetter oder im Landkreis Marburg-Biedenkopf; die einzigen Treffer sind die La Rustika e.K., Inhaber "
    "Iber Bajramaj (Nürnberg, HRA 16730) und die Ingenieurgemeinschaft Bajramaj Halitovic PartG mbB "
    "(Frankfurt am Main, PR 3155), also **Namensvetter** anderer Branchen und Orte, die **nicht** "
    "übernommen werden. Das Feld certifications wurde im Frontmatter als Negativbefund mit der "
    "Reichweitenklausel gefüllt; Zertifikatsnummer, Vorgangsnummer und Gültigkeitsdatum bleiben **nicht "
    "ausgewiesen**. Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein "
    "Register-Nichteintrag ist **kein** Nachweis für die Nichtexistenz des Betriebs. Der bereits im "
    "Dossier gesetzte Adresspunkt 50.9034911/8.7362641 (Mittelwert der eindeutigen Knoten von "
    "way/1084624540, Im Berntal 9, 35083 Wetter) bleibt unverändert; das OSM-Objekt trägt **weder** `name` "
    "**noch** `operator` und belegt die Anschrift, nicht den Betrieb. **ANKAUF:** keine aktuellen "
    "Metall- oder Schrottpreise und keine Ankaufzusage, die Altmetalltätigkeit ist nach eigener Aussage "
    "eine Anfangsphase ab 2004. **VERKAUF:** keine Warenpreisliste. **GEBÜHREN:** Container-, Bau- und "
    "Entsorgungsleistungen ohne Zahlenpreise, Kalkulation nach Besichtigung. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Bajramaj ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Bajramaj ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Im%20Berntal%209%2C%2035083%20Wetter%2C%20Deutschland%3B8.7362641%3B"
    "50.9034911%3B8.7802827%3B50.9304406%3B8.6922455%3B50.8765416&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Im%20Berntal%209%2C%2035083%20Wetter%2C%20Deutschland%3B8.7362641%3B"
    "50.9034911%3B8.7802827%3B50.9304406%3B8.6922455%3B50.8765416&dst=3&is=1&e=1 ; " + HRP +
    "Bajramaj (Abruf 08.11.2026).]",
]

N["he-hochst-64739-sac-recycling"] = [
    "- [Recherche 08.11.2026: **Die Namenssuche und die 3-km-Ortsabfrage bestätigen den Negativbefund vom "
    "05.11.2026 unverändert; die Substring-Suche „SAC Recycling“ trifft nur Sachbetriebe mit "
    "Nebenstandorten.** Das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per "
    "Playwright/Chromium bedient) liefert am 08.11.2026 für die Namenssuche „SAC Recycling“ in der Route "
    "„Entsorgungsfachbetriebe“ **5 Treffer** und in der Route „Altfahrzeugverwertung“ **0 Treffer**; die "
    "fünf Zeilen gehören der BBV Breisacher Bauschuttverwertungs GmbH (Franzosenweg, Am Rimsinger Ei 1, "
    "79206 Breisach-Oberrimsingen, ZZLT00500045 / 1) und der Dahm & Bohnsack Handels- und Recycling GmbH "
    "(Gewerbegebiet Ost 8, 16845 Neustadt/Dosse, ZZHT00300793 / 1 bis / 4), also **anderen** Rechtsträgern "
    "in Baden-Württemberg und Brandenburg; die Standortabfrage mit 3 km um An der Hasselbach 1, 64739 "
    "Höchst (Geocoder-Koordinaten 8.9328112/49.7897033, PLZ-Abgleich 64739) liefert in **beiden** Routen "
    "**0 Treffer**. Die Zahl der Namensvetter ändert sich damit gegenüber dem 05.11.2026 nicht, die "
    "Substring-Suche erklärt die „SAC“-Treffer von damals (Sachsen, Sachse, Aktualisierung). Zertifikats"
    "nummer, Vorgangsnummer, Zertifizierungsorganisation, Ausstellungsdatum, Gültigkeitsdatum, "
    "Anlagenanzahl und zertifizierte Tätigkeit bleiben **nicht ausgewiesen**. Reichweite: die "
    "Efb-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein "
    "Register-Nichteintrag ist **kein** Nachweis für die Nichtexistenz des Betriebs. **ANKAUF:** keine "
    "Preise und keine belegte Materialannahme. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. "
    "Quelle(n): " + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=SAC%20Recycling ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=SAC%20Recycling ; " + REG +
    "/Entsorgungsfachbetriebe/adr=An%20der%20Hasselbach%201%2C%2064739%20H%C3%B6chst%2C%20Deutschland%3B"
    "8.9328112%3B49.7897033%3B8.9528110%3B49.8097033%3B8.9128114%3B49.7697033&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=An%20der%20Hasselbach%201%2C%2064739%20H%C3%B6chst%2C%20Deutschland%3B"
    "8.9328112%3B49.7897033%3B8.9528110%3B49.8097033%3B8.9128114%3B49.7697033&dst=3&is=1&e=1 (Abruf "
    "08.11.2026).]",
]

N["he-hanau-63452-guth"] = [
    "- [Recherche 08.11.2026: **Der Hanauer Lead bleibt unbelegt: das Register kennt nur einen "
    "Guth-Namensvetter in Großkrotzenburg, die Betreiberdomain ist weiterhin eine Platzhalterseite.** Das "
    "amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) "
    "liefert am 08.11.2026 für die Namenssuche „Guth“ in der Route „Entsorgungsfachbetriebe“ **14 "
    "Treffer** und in der Route „Altfahrzeugverwertung“ **0 Treffer**; alle 14 Zeilen sind **Namensvetter** "
    "aus dem Wortbestandteil „Schüttguthandel“ (TSG Transporte & Schüttguthandel Greußen GmbH, TSH "
    "Tiefbau- und Schüttguthandel GmbH & Co. KG mit sechs Standortanlagen, PG Gruppe Containerdienst und "
    "Schüttguthandel GmbH, Containerdienst & Schüttguthandel Steudel GmbH), **keine** Zeile nennt eine "
    "Firma Guth in Hanau; die Standortabfrage mit 3 km um Niddastraße 32, 63452 Hanau (Nominatim-"
    "Koordinaten 8.9398988/50.1496126, PLZ-Abgleich 63452) liefert in der Route „Entsorgungsfachbetriebe“ "
    "**17 Treffer**, darunter die Wüst Abschleppdienst GmbH, An der Brückengrube 12 12, 63452 Hanau, "
    "ZZHT00700226 / 1, 0,2 km, die Kurt Steding Entsorgungs-GmbH, Ruhrstraße 29, 63452 Hanau, "
    "ZZEE00300144 / 1 bis / 3, 0,3 km, die Altöldienst Vogel GmbH, Möhnestraße 19, 63452 Hanau, "
    "ZZFT00400003 / 1 und / 2, 0,4 km, die Mineralölhandel Hans Schmidt GmbH & Co. KG Hanau, Möhnestraße "
    "17 und 19, ZZFT00400002 / 18 und / 19, 0,4 km, und die DB Bahnbau Gruppe GmbH, Bereich Recycling "
    "Hanau, Robert-Blum-Straße 1, 63452 Hanau, ZZLE00200008 / 5 und / 6, 1,0 km; in der Route "
    "„Altfahrzeugverwertung“ **0 Treffer**. Das amtliche Registerportal (online-handelsregister.de, "
    "Suchbegriff „Guth“, Abruf 08.11.2026) kennt für Hanau die **Guth Metallbau GmbH** (HRB 98439, "
    "Amtsgericht Hanau); der Registerspiegel (handelsregister.live, strukturierte Registerdaten, Abruf "
    "08.11.2026) nennt Sitz Großkrotzenburg, Geschäftsanschrift Hanauer Landstraße 80, 63538 "
    "Großkrotzenburg, Geschäftsführer Karsten Guth (Altenstadt), Stammkapital 25.000,00 EUR, "
    "Gesellschaftsvertrag vom 24.11.2021, Eintragung 21.12.2021, Gegenstand „der Vertrieb, die "
    "Herstellung sowie die Montage und Ausführung von Metallbau- und Stahlbauarbeiten und alle "
    "zusammenhängende Tätigkeiten“; das ist ein **Namensvetter** in einer anderen Stadt und wird **nicht** "
    "übernommen. Die Domain autoverwertungguth.de liefert am 08.11.2026 weiterhin nur die Platzhalter"
    "seite „Diese Homepage wird zur Zeit überarbeitet.“, es gibt damit **keine** Betreiber-Primärquelle. "
    "Das Feld certifications wurde im Frontmatter als Negativbefund mit Reichweitenklausel gefüllt. "
    "**ANKAUF:** für den Hanauer Lead nicht bestätigt. **VERKAUF:** keine zuordenbare Preisliste. "
    "**GEBÜHREN:** keine Angaben; die Öffnungszeiten Mo-Fr 9-18 und Sa 9-14 aus dem Verzeichnis sind "
    "Aggregatorinhalt. **GEO:** OSM führt die Niddastraße 32, 63452 Hanau als eigenes Adressobjekt "
    "(way/223030475, live geprüft am 08.11.2026), ohne `name`- oder `operator`-Tag und damit ohne "
    "Betriebszuordnung; die Koordinate bleibt offen, weil die Anschrift nur ein Verzeichnislead ist. "
    "Quelle(n): " + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Guth ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Guth ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Niddastra%C3%9Fe%2032%2C%2063452%20Hanau%2C%20Deutschland%3B8.9398988%3B"
    "50.1496126%3B8.9839174%3B50.1765621%3B8.8958802%3B50.1226631&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Niddastra%C3%9Fe%2032%2C%2063452%20Hanau%2C%20Deutschland%3B8.9398988%3B"
    "50.1496126%3B8.9839174%3B50.1765621%3B8.8958802%3B50.1226631&dst=3&is=1&e=1 ; " + HRP + "Guth ; " +
    HR_LIVE + "/hrb/hessen/ag-hanau/98439/guth-metallbau-gmbh/strukturiert ; "
    "http://www.autoverwertungguth.de/ ; https://api.openstreetmap.org/api/0.6/way/223030475/full.json "
    "(Abruf 08.11.2026).]",
]

N["he-oberursel-61440-lackner"] = [
    "- [Recherche 08.11.2026: **Der Hausnummernkonflikt ist entschieden: die Verzeichnisanschrift "
    "„Pfeiffstr. 11“ ist in OSM ein Penny-Supermarkt, die Betreiberanschrift „Pfeiffstr. 13“ ist die "
    "einzige Adresse mit Betriebsbezug.** Die OSM-Abfrage (live abgerufen am 08.11.2026) weist für die "
    "Pfeiffstraße in Oberursel zwei Objekte aus: **way/25722044** trägt `addr:housenumber`=**9-11**, "
    "`addr:street`=Pfeiffstraße, `addr:postcode`=61440, `addr:city`=Oberursel (Taunus), "
    "`addr:suburb`=Oberursel-Mitte, `name`=**Penny**, `brand`=Penny, `shop`=**supermarket**, "
    "`building`=retail, `website`=https://www.penny.de/markt/oberursel-taunus/230315/"
    "penny-oberursel-pfeiffstrasse-9, `opening_hours`=Mo-Sa 07:00-21:00, 10 Knoten, BBox "
    "8.58634/50.193283 bis 8.5871434/50.1937398, Mittelpunkt 8.5866874/50.1934789; **way/94177541** trägt "
    "`addr:housenumber`=**13**, `addr:street`=Pfeiffstraße, `addr:postcode`=61440, "
    "`addr:city`=Oberursel (Taunus), `building`=yes, **ohne** `name`, `operator` und `shop`, 4 Knoten, "
    "BBox 8.5860964/50.1934741 bis 8.5867648/50.1939115, Mittelpunkt 8.5864306/50.1936928. Die "
    "Hausnummer **11** bezeichnet damit nachweislich ein **Ladengeschäft** und kann den Schrottplatz "
    "nicht tragen; die im Dossier seit dem 01.10.2026 offene Hausnummensfrage ist damit zugunsten der "
    "Betreiberanschrift 13 geklärt. Das Betreiber-Impressum (live gelesen am 08.11.2026) nennt "
    "unverändert „Auto Lackner / Pfeiffstr. 13 / 61440 Oberursel / Deutschland“, „Telefon: 06171 - 57090“, "
    "„E-Mail: info@auto-lackner.de“, „Kontoinhaber: Ralf Lackner“, IBAN DE45 500 100 600 753 222 607 bei "
    "der Postbank Frankfurt sowie die Öffnungszeiten Montag bis Freitag 09:00 - 18:00, Samstag 10:30 - "
    "14:00 und Sonntag geschlossen; die Seiten tragen den Copyright-Vermerk „© 2020 Auto-Lackner“. "
    "**Amtliches Fachbetrieberegister** (clientseitig gerenderte Vaadin-App, per Playwright/Chromium "
    "bedient, Abruf 08.11.2026): die Standortabfrage mit 3 km um Pfeiffstr. 13, 61440 Oberursel "
    "(Geocoder-Koordinaten 8.5864306/50.1936928) liefert in der Route „Entsorgungsfachbetriebe“ **4 "
    "Treffer**, darunter die Behälterdienst Sommerkorn GmbH, An den Drei Hasen 43, 61440 Oberursel, "
    "ZZFT01200035 / 1 bis / 3, 1,9 km, und die Wilhelm Schütz GmbH & Co. KG, Straßen und Tiefbau, "
    "Niederurseler Straße 51, 61440 Oberursel/Weißkirchen, ZZHT00400103 / 1, 2,5 km, und in der Route "
    "„Altfahrzeugverwertung“ **0 Treffer**; auf dem Grundstück der Pfeiffstraße 13 trägt das Register "
    "**keinen** Eintrag. Das Feld certifications wurde im Frontmatter als Negativbefund mit "
    "Reichweitenklausel gefüllt. Die Stammdaten bleiben unverändert, weil die Betreiber-Primärquelle "
    "allein steht und das OSM-Objekt den Betrieb **nicht** zuordnet; die Koordinate wird deshalb **nicht** "
    "gesetzt. **ANKAUF:** die Autoankauf-Seite bietet wörtlich „ein kostenloses und unverbindliches "
    "Ankauf-Angebot“ an, nennt aber **keine** Zahlenpreise und **keine** Metall- oder Schrottkurse; das "
    "kostenlose Angebot ist kein Ankaufspreis. **VERKAUF:** der Shop führt Einzelartikel mit "
    "Artikelpreisen, das sind Einzelstück-Retailpreise und keine Schrottpreisliste. **GEBÜHREN:** keine "
    "Gebührentafel für Schrottannahme oder Verwertung. Quelle(n): "
    "https://api.openstreetmap.org/api/0.6/way/25722044/full.json ; "
    "https://api.openstreetmap.org/api/0.6/way/94177541/full.json ; https://auto-lackner.de/impressum/ ; "
    "https://auto-lackner.de/pkw_kfz_ankauf/ ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Pfeiffstra%C3%9Fe%2013%2C%2061440%20Oberursel%2C%20Deutschland%3B"
    "8.5864306%3B50.1936928%3B8.6304492%3B50.2206423%3B8.5424120%3B50.1667433&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Pfeiffstra%C3%9Fe%2013%2C%2061440%20Oberursel%2C%20Deutschland%3B"
    "8.5864306%3B50.1936928%3B8.6304492%3B50.2206423%3B8.5424120%3B50.1667433&dst=3&is=1&e=1 (Abruf "
    "08.11.2026).]",
]

N["he-wiesbaden-biebrich-wiesbaden-dotzheim-bert-brecht-str-26"] = [
    "- [Recherche 08.11.2026: **Für „Wintermeyer“ führt das amtliche Register nur Elektro- und "
    "Automationsanlagenbetriebe in Wiesbaden; ein Entsorgungsbetrieb ist nicht ausgewiesen.** Das amtliche "
    "Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) liefert am "
    "08.11.2026 für die Namenssuche „Wintermeyer“ in der Route „Entsorgungsfachbetriebe“ **0 Treffer** und "
    "in der Route „Altfahrzeugverwertung“ **0 Treffer**; die Standortabfrage mit 3 km um Bert-Brecht-Str. "
    "26, 65201 Wiesbaden (Nominatim-Koordinaten 8.1950399/50.0577522, PLZ-Abgleich 65201) liefert in der "
    "Route „Entsorgungsfachbetriebe“ **9 Treffer**, darunter die ELW Entsorgungsbetriebe der "
    "Landeshauptstadt Wiesbaden, Wertstoffhof Dotzheim, Willi-Werner-Straße 11, 65199 Wiesbaden-Dotzheim, "
    "ZZET01700037 / 7, 1,2 km, die H.L.R. Herbert Lagerin Rohstoffhandel GmbH, Am Roten Stock 16, 65199 "
    "Wiesbaden, ZZGT00200094 / 1, 1,8 km, die Gauer Daten- u. Aktenvernichtungs GmbH, Paul-Friedländer-"
    "Straße 10, 65203 Wiesbaden, ZZEE00100105 / 1 bis / 3, 2,2 km, und die Gebr. Wehle GmbH, In der "
    "Rehbach 20, 65396 Walluf, ZZFE00100179 / 1 bis / 3, 2,8 km; in der Route „Altfahrzeugverwertung“ **0 "
    "Treffer**. Das amtliche Registerportal (online-handelsregister.de, Suchbegriff „Wintermeyer“, Abruf "
    "08.11.2026) kennt in Wiesbaden ausschließlich Unternehmen anderer Branchen: die ELEKTRO Wintermeyer "
    "GmbH (HRB 9209), das ELEKTRO Wintermeyer Karl Erich Wintermeyer (HRA 3997), die Karl Wintermeyer "
    "Gesellschaft für Herstellung und Vertrieb von Automationsanlagen mit beschränkter Haftung (HRB "
    "4647), das Karl Wintermeyer, Automationsanlagen (HRA 1810) und Wilhelm Wintermeyer & Sohn (HRA 1745); "
    "sie sind **Namensvetter** und werden **nicht** übernommen, ein „Wintermeyer Romano Entsorgungsbetriebe“ "
    "ist nicht eingetragen. OSM führt Bert-Brecht-Straße 26, 65201 Wiesbaden, Ortsteil Schierstein, als "
    "eigenes Adressobjekt (way/99188048, `building`=house, live geprüft am 08.11.2026) mit 6 Knoten, "
    "Mittelpunkt 8.1950152/50.0577931; es trägt **weder** `name` **noch** `operator` und ordnet den Betrieb "
    "damit **nicht** zu. Das Feld certifications wurde im Frontmatter als Negativbefund mit "
    "Reichweitenklausel gefüllt. **ANKAUF:** keine Kurse. **VERKAUF:** keine Liste. **GEBÜHREN:** die "
    "Verzeichnisangabe „kostenlose Schrottabholung“ bleibt Aggregatorinhalt und ist kein verifizierter "
    "Tarif. Die im Dossier dokumentierte Seed-Spur Wiesbaden-Biebrich, Rathausstr. 25 bleibt offen, ebenso "
    "die Lage des Betriebes; die Koordinate wird **nicht** gesetzt. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Wintermeyer ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Wintermeyer ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Bert-Brecht-Stra%C3%9Fe%2026%2C%2065201%20Wiesbaden%2C%20Deutschland%3B"
    "8.1950399%3B50.0577522%3B8.2370594%3B50.0847017%3B8.1530204%3B50.0308027&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Bert-Brecht-Stra%C3%9Fe%2026%2C%2065201%20Wiesbaden%2C%20Deutschland%3B"
    "8.1950399%3B50.0577522%3B8.2370594%3B50.0847017%3B8.1530204%3B50.0308027&dst=3&is=1&e=1 ; " + HRP +
    "Wintermeyer ; https://api.openstreetmap.org/api/0.6/way/99188048/full.json (Abruf 08.11.2026).]",
]

N["he-korbach-34497-heidel-eckhardt"] = [
    "- [Recherche 08.11.2026: **Die 3-km-Ortsabfrage um Am Mühlwege 2 misst den Bestand genauer als der "
    "10-km-Wert vom 07.11.2026: neun Efb-Zeilen und eine AltfahrzeugV-Zeile, keine davon am "
    "Dossierstandort.** Das amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per "
    "Playwright/Chromium bedient) liefert am 08.11.2026 für die Namenssuche „Autodienst Heidel“ in der "
    "Route „Entsorgungsfachbetriebe“ **0 Treffer** und in der Route „Altfahrzeugverwertung“ **0 "
    "Treffer**; die Standortabfrage mit 3 km um Am Mühlwege 2, 34497 Korbach (Geocoder-Koordinaten "
    "8.8596109/51.2894594, PLZ-Abgleich 34497) liefert in der Route „Entsorgungsfachbetriebe“ **9 "
    "Treffer**, davon **6** für die Friedrich Groß GmbH & Co. KG, Hauptsitz, Am Ziegelgrund 5-7, 34497 "
    "Korbach, ZZET00600003 / 1 bis / 6, **0,5 km**, und **3** für die Pohlmann GmbH & Co. KG, Am Kniep 2, "
    "34497 Korbach, ZZEE00100448 / 1 bis / 3, 1,4 km; in der Route „Altfahrzeugverwertung“ **1 Treffer**, "
    "das ist ebenfalls die Friedrich Groß GmbH & Co. KG, Hauptsitz, Am Ziegelgrund 5 - 7, 34497 Korbach, "
    "ZZET00600003 / 5, 0,5 km. **Keine** Zeile nennt Heidel oder den Am Mühlwege 2; auf dem Grundstück "
    "des Dossierstandorts trägt das Register **keinen** Eintrag. Damit ist der Negativbefund vom "
    "07.11.2026 (Namenssuche 0/0, 10-km-Ortsabfrage 16 Efb- und 6 AfV-Zeilen) reproduziert und um die "
    "3-km-Messung mit Distanzen ergänzt. Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind "
    "**freiwillig**, ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung "
    "besitzt; die im Dossier dokumentierte Heidel eGbR in Vöhl bleibt ein **Namensvetter** und wird nicht "
    "übernommen. **ANKAUF:** keine Fahrzeug- oder Materialkurse veröffentlicht. **VERKAUF:** "
    "Ersatzteilhandel über den Online-Shop belegt, **kein** Preisverzeichnis gelesen. **GEBÜHREN:** keine "
    "Verwertungstarife, die Aussage „kostenlos“ ist für den Korbacher Standort **nicht** belegt. "
    "Quelle(n): " + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Autodienst%20Heidel ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Autodienst%20Heidel ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Am%20M%C3%BChlwege%202%2C%2034497%20Korbach%2C%20Deutschland%3B"
    "8.8596109%3B51.2894594%3B8.9036305%3B51.3164090%3B8.8155913%3B51.2625098&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Am%20M%C3%BChlwege%202%2C%2034497%20Korbach%2C%20Deutschland%3B"
    "8.8596109%3B51.2894594%3B8.9036305%3B51.3164090%3B8.8155913%3B51.2625098&dst=3&is=1&e=1 (Abruf "
    "08.11.2026).]",
]

N["he-hanau-lamboy-schrott-und-metalle-reinhardt"] = [
    "- [Recherche 08.11.2026: **Die Namenssuche und die 3-km-Ortsabfrage bestätigen: kein Reinhardt-Betrieb "
    "am Seed-Standort in Hanau-Lamboy.** Das amtliche Fachbetrieberegister (clientseitig gerenderte "
    "Vaadin-App, per Playwright/Chromium bedient) liefert am 08.11.2026 für die Namenssuche „Reinhardt“ in "
    "der Route „Entsorgungsfachbetriebe“ **7 Treffer** und in der Route „Altfahrzeugverwertung“ **0 "
    "Treffer**; die sieben Zeilen gehören **anderen** Rechtsträgern, nämlich der R+S Reinhardt & Sela "
    "GmbH u.Co.KG, Am Bahndamm 2, 63683 Ortenberg-Bleichenbach, ZZFT01300094 / 1 bis / 3, der VEREVE "
    "Inh. Gina Reinhardt (Anlage Robert-Bosch-Strasse 1, 71729 Erdmannhausen, ZZFT00300031 / 1, und "
    "Unternehmenssitz Otto-Hahn-Straße 6, 71254 Ditzingen, ZZFT00300031 / 2) und der Reinhardt Rohstoffe "
    "GmbH, Lichterfelder Weg 1 - 3, 14167 Berlin, ZZLT00400087 / 1 und / 2; die Standortabfrage mit 3 km um "
    "Friedrich-Engels-Straße 5, 63452 Hanau-Lamboy (Nominatim-Koordinaten 8.9272103/50.1437119, "
    "PLZ-Abgleich 63452) liefert in der Route „Entsorgungsfachbetriebe“ **22 Treffer**, darunter die DB "
    "Bahnbau Gruppe GmbH, Bereich Recycling Hanau, Robert-Blum-Straße 1, 63452 Hanau, ZZLE00200008 / 5 und "
    "/ 6, **0,2 km**, die Martrans GmbH, Breslauer Straße 19, 63452 Hanau, ZZFT00300092 / 1, 0,4 km, die "
    "Hanau Infrastruktur Service, Hessen-Homburg-Platz 5, 63452 Hanau, ZZKT00100161 / 1, 0,8 km, und die "
    "Kurt Steding Entsorgungs-GmbH, Ruhrstraße 29, 63452 Hanau, ZZEE00300144 / 1 bis / 3, 0,8 km; in der "
    "Route „Altfahrzeugverwertung“ **0 Treffer**. **Keine** Zeile nennt Reinhardt oder die "
    "Friedrich-Engels-Straße 5; der im selben Kartenausschnitt dokumentierte Kurt Steding bleibt der "
    "namentlich geführte Schrotthandel im selben Postleitzahlengebiet, eine Verbindung oder Abgrenzung "
    "bleibt offen. OSM führt Friedrich-Engels-Straße 5, 63452 Hanau, Ortsteil Lamboy, als eigenes "
    "Adressobjekt (way/223045220, `building`=apartments mit 8 Ebenen, live geprüft am 08.11.2026) mit 14 "
    "Knoten, Mittelpunkt 8.9272221/50.1436993; es trägt **weder** `name` **noch** `operator` und ordnet "
    "den Betrieb damit **nicht** zu. Das Feld certifications wurde im Frontmatter als Negativbefund mit "
    "Reichweitenklausel gefüllt. **ANKAUF:** keine Kurse. **VERKAUF:** keine Preisliste. **GEBÜHREN:** "
    "keine Tarife. Quelle(n): " + REG + "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Reinhardt ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Reinhardt ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Friedrich-Engels-Stra%C3%9Fe%205%2C%2063452%20Hanau%2C%20Deutschland%3B"
    "8.9272103%3B50.1437119%3B8.9712290%3B50.1706614%3B8.8831916%3B50.1167624&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Friedrich-Engels-Stra%C3%9Fe%205%2C%2063452%20Hanau%2C%20Deutschland%3B"
    "8.9272103%3B50.1437119%3B8.9712290%3B50.1706614%3B8.8831916%3B50.1167624&dst=3&is=1&e=1 ; " + NOMI +
    "Friedrich-Engels-Stra%C3%9Fe%205%2C%2063452%20Hanau%2C%20Deutschland ; "
    "https://api.openstreetmap.org/api/0.6/way/223045220/full.json (Abruf 08.11.2026).]",
]

N["he-dillenburg-35688-schmidt-ludwig"] = [
    "- [Recherche 08.11.2026: **Die 3-km-Ortsabfrage um Obere Mühlseite 26 misst neun Efb-Zeilen "
    "fremder Rechtsträger und bestätigt die Abwesenheit eines Registereintrags am Dossierstandort.** Das "
    "amtliche Fachbetrieberegister (clientseitig gerenderte Vaadin-App, per Playwright/Chromium bedient) "
    "liefert am 08.11.2026 für die Namenssuche „Schmidt Metallhandel“ in der Route „Entsorgungsfachbetriebe“ "
    "**0 Treffer** und in der Route „Altfahrzeugverwertung“ **0 Treffer**; die Standortabfrage mit 3 km um "
    "Obere Mühlseite 26, 35688 Dillenburg (Geocoder-Koordinaten 8.3357265/50.7333643, PLZ-Abgleich 35688) "
    "liefert in der Route „Entsorgungsfachbetriebe“ **9 Treffer**, darunter die DESABAU GmbH, "
    "Schelde-Lahn-Straße 2a, 35688 Dillenburg-Oberscheld, ZZFT01300205 / 1 bis / 3, **0,3 km**, die PreZero "
    "Service Mitte-West GmbH & Co. KG, NL Dillenburg, Grube Falkenstein, 35688 Dillenburg, ZZEE00200224 / "
    "22 bis / 24, 0,7 km, und die TSR Deutschland GmbH & Co. KG, Region Südwest I, Niederlassung "
    "Dillenburg, Am Güterbahnhof 53, 35683 Dillenburg, ZZEE00200230 / 4 bis / 6, 3,0 km; in der Route "
    "„Altfahrzeugverwertung“ **0 Treffer**. **Keine** Zeile nennt Schmidt und **keine** trägt die "
    "Obere Mühlseite 26; das Register kennt das amtlich belegte „Schmidt Metallhandel“ mit den Personen "
    "Ludwig Schmidt und Nico Schmidt nicht. Der Negativbefund vom 07.11.2026 (Namenssuche 0/0, "
    "10-km-Ortsabfrage 23 Anlagenzeilen) ist damit reproduziert und um die 3-km-Messung mit Distanzen "
    "ergänzt. Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**, ein "
    "Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt. **ANKAUF:** "
    "keine Annahmebedingungen und keine Kurse. **VERKAUF:** keine Preisliste. **GEBÜHREN:** keine Tarife. "
    "Die im Dossier dokumentierte Anschrift Obere Mühlseite 26, 35688 Dillenburg-Oberscheld und der "
    "Gewerbeidentitäts-Klärfall zwischen den PLZ 35688 und 35687 bleiben unverändert offen; eine "
    "Koordinate wird **nicht** gesetzt. Quelle(n): " + REG +
    "/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schmidt%20Metallhandel ; " + REG +
    "/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schmidt%20Metallhandel ; " + REG +
    "/Entsorgungsfachbetriebe/adr=Obere%20M%C3%BChlseite%2026%2C%2035688%20Dillenburg%2C%20Deutschland%3B"
    "8.3357265%3B50.7333643%3B8.3777451%3B50.7603138%3B8.2937079%3B50.7064148&dst=3&is=1&e=1 ; " + REG +
    "/Altfahrzeugverwertung/adr=Obere%20M%C3%BChlseite%2026%2C%2035688%20Dillenburg%2C%20Deutschland%3B"
    "8.3357265%3B50.7333643%3B8.3777451%3B50.7603138%3B8.2937079%3B50.7064148&dst=3&is=1&e=1 (Abruf "
    "08.11.2026).]",
]


def yaml_value(key, value):
    """Frontmatter-Wert als YAML-Skalar (einfache Anfuehrungszeichen)."""
    if key == "certifications":
        assert value.startswith('["') and value.endswith('"]'), key
        json.loads(value)          # muss gueltiges JSON-Array sein
        return "'" + value + "'"
    if key in ("lat", "lon"):
        return value
    if key in ("status", "trader_type", "website_status", "clear_fields"):
        assert "'" not in value, (key, value)
        return value
    assert "'" not in value, (key, value)
    return "'" + value + "'"


def set_field(text, key, value):
    """Setzt ein Frontmatter-Feld (ersetzt oder ergaenzt es an passender Stelle)."""
    pat = re.compile(rf"^({re.escape(key)}):.*$", re.M)
    m = pat.search(text)
    if m:
        return pat.sub(lambda _m: f"{key}: {value}", text, count=1)
    # Einfuegeposition: Koordinaten hinter dem slug, Zertifikate hinter pickup_json
    if key in ("lat", "lon"):
        anchor = r"^slug:.*$"
    elif key == "certifications":
        anchor = r"^pickup_json:.*$" if re.search(r"^pickup_json:", text, re.M) else r"^dropoff_json:.*$"
    else:
        anchor = r"^slug:.*$"
    m = re.search(anchor, text, re.M)
    assert m, "kein Anker"
    return text[:m.end()] + f"\n{key}: {value}" + text[m.end():]


def main():
    slugs = [l.strip() for l in open("/var/tmp/opencode/welle-s2.txt") if l.strip()]
    changed = 0
    for slug in slugs:
        path = DOSS / f"{slug}.md"
        text = path.read_text(encoding="utf-8")
        fm = FM.get(slug, {})
        for key, value in fm.items():
            text = set_field(text, key, yaml_value(key, value))
        notes = N.get(slug)
        if notes:
            for note in notes:
                assert note.startswith("- [Recherche 08.11.2026: "), slug
                assert note.endswith("]"), slug
                assert note.count("(") == note.count(")"), (slug, note.count("("), note.count(")"))
                assert "\u2013" not in note and "\u2014" not in note, slug
            text = text.rstrip("\n") + "\n\n### Recherche 08.11.2026\n\n" + "\n".join(notes) + "\n"
        path.write_text(text, encoding="utf-8")
        changed += 1
        print(f"{slug}: {len(fm)} Felder, {len(notes or [])} Notizen")
    print(f"\nDossiers bearbeitet: {changed}")


if __name__ == "__main__":
    main()
