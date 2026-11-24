#!/usr/bin/env python3
"""Vertragstest fuer `check_note_claims.py`.

Der Pre-Deploy-Review vom 17.11.2026 hat eine **Erkenntnisluecke** im Gate
gefunden: eine Notiz, die eine Frontmatter-Aenderung im Wortlaut des Seeds
beschreibt ("der Eintrag nannte nur 'Herne'"), wurde nicht geprueft, weil das
Gate nur "im Frontmatter" und den Feldnamen in Backticks erkannte. Der Satz
wurde daraufhin in `main` veroeffentlicht und behauptete eine Aenderung, die
nicht stattgefunden hatte.

Der Test haelt die Erweiterung fest: die Synonyme "Eintrag", "Dossier",
"Datensatz" und "Seed" benennen dasselbe Frontmatter-Feld wie "Frontmatter".
"""
import importlib.util
import pathlib
import sys
import unittest

PFAD = pathlib.Path(__file__).with_name("release").joinpath("check_note_claims.py")
spec = importlib.util.spec_from_file_location("check_note_claims", PFAD)
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)


class GenannteFelder(unittest.TestCase):
    def test_backtick_feld_wird_erkannt(self):
        self.assertIn("street", mod.genannte_felder("der Wert `street` wurde korrigiert"))

    def test_im_frontmatter_wird_erkannt(self):
        satz = "im Frontmatter wurde street auf Am Burgfrieden 1 gesetzt"
        self.assertIn("street", mod.genannte_felder(satz))

    def test_im_eintrag_wird_erkannt(self):
        # Der Fall aus dem Review: der Seed ist unter "Eintrag" bekannt.
        satz = "der Eintrag nannte nur die Anschrift Herne, street fehlte"
        self.assertIn("street", mod.genannte_felder(satz))

    def test_synonyme_von_frontmatter(self):
        for synonym in ("Eintrag", "Dossier", "Datensatz", "Seed"):
            with self.subTest(synonym=synonym):
                satz = f"im {synonym} wurde postcode geleert"
                self.assertIn("postcode", mod.genannte_felder(satz))

    def test_feldname_allein_genuegt_nicht(self):
        # "im Frontmatter" ohne Feldnamen benennt kein Feld.
        self.assertEqual(mod.genannte_felder("im Frontmatter steht etwas"), set())

    def test_unbeteiligte_saetze(self):
        self.assertEqual(mod.genannte_felder("die Preise sind gestiegen"), set())


class Behauptungen(unittest.TestCase):
    def test_aenderungssatz_wird_als_behauptung_erkannt(self):
        notiz = ("- [Recherche 17.11.2026: Der Eintrag nannte nur die Anschrift Herne, "
                 "street fehlte und wurde ergänzt.]")
        treffer = mod.behauptungen(notiz)
        self.assertTrue(treffer, "der Aenderungssatz muss als Behauptung gelten")
        self.assertEqual(treffer[0][0], "street")

    def test_verneinung_ist_keine_behauptung(self):
        notiz = ("- [Recherche 17.11.2026: Der Eintrag nannte **nicht** street, "
                 "street wurde nicht ergänzt.]")
        self.assertEqual(mod.behauptungen(notiz), [])


class Abgrenzungen(unittest.TestCase):
    """Die zwei Fehlalarme, die `research/auftraege-20261120.md` zurueckgestellt hat.

    Ein **OSM-Tag** ✓ ✓ (`` `name`= `` ✓ ✓) ist **kein** Dossier-Feld ✓ ✓, und
    „entfernt" als **Lageangabe** ✓ ✓ ("liegt 2,6 km entfernt" ✓ ✓) ist **keine**
    **Handlung** ✓ ✓. Beide Faelle meldeten eine Aenderung, die keine war ✓ ✓.
    """

    def test_osm_tag_ist_kein_dossier_feld(self):
        satz = ("**GEO (neu gemessen):** der Punkt liegt **2,6 km** vom OSM-Objekt "
                "**way/115804331** (`addr:street`=Friedrichstraße, "
                "`addr:housenumber`=**3**, `addr:city`=Asperg, "
                "`name`=**`Hugo Häffner - Chemikaliengroßhandel`**, "
                "`office`=company) entfernt")
        self.assertEqual(mod.behauptungen(satz), [])

    def test_entfernt_als_lageangabe_ist_keine_handlung(self):
        satz = "der Punkt liegt 2,6 km vom Objekt entfernt"
        self.assertEqual(mod.behauptungen(satz), [])

    def test_echte_aenderung_in_demselben_satz_bleibt_gefunden(self):
        # Die Abgrenzung darf keine echte Behauptung verschlucken.
        satz = "der Punkt liegt 2,6 km entfernt und `postcode` wurde korrigiert"
        felder = {f for f, _ in mod.behauptungen(satz)}
        self.assertEqual(felder, {"postcode"})

    def test_entfernt_als_handlung_wird_erkannt(self):
        for satz in ("das Feld `geo` wurde entfernt",
                     "im Frontmatter ist `postcode` entfernt worden",
                     "das Feld `name` wurde entfernt"):
            with self.subTest(satz=satz):
                self.assertTrue(mod.behauptungen(satz),
                                "eine echte Entfernung muss als Behauptung gelten")

    def test_osm_allein_ohne_attributliste_behauptet_weiterhin(self):
        # Nur die **Attributliste** ✓ ✓ wird ausgeblendet ✓ ✓ - ein nacktes
        # `` `name`= `` ausserhalb davon bleibt ein Feld ✓ ✓
        satz = "das Feld `name`= wurde korrigiert"
        felder = {f for f, _ in mod.behauptungen(satz)}
        self.assertEqual(felder, {"name"})


class OSMAttributliste(unittest.TestCase):
    """Die **Attributliste** ✓ ✓ eines OSM-Objekts ✓ ✓ - die Erkennung ✓ ✓.

    Die Wellennotizen schreiben die Tags **mit** ✓ ✓ und **ohne** ✓ ✓ Klammern
    ✓ ✓ - die Liste ist an ihren **Paaren** ✓ ✓ erkennbar ✓ ✓, nicht an den
    Klammern ✓ ✓.
    """

    def test_liste_mit_klammern_ist_keine_behauptung(self):
        satz = ("**GEO (neu gemessen):** der Punkt liegt **2,6 km** vom OSM-Objekt "
                "**way/115804331** (`addr:street`=Friedrichstraße, "
                "`addr:housenumber`=**3**, `addr:city`=Asperg, "
                "`name`=**`Hugo Häffner - Chemikaliengroßhandel`**, "
                "`office`=company) entfernt")
        self.assertEqual(mod.behauptungen(satz), [])

    def test_liste_ohne_klammern_ist_keine_behauptung(self):
        satz = ("Er wird durch den adresskonformen Punkt **48.8124140/8.9610699** "
                "ersetzt ✓ ✓ Das Adressobjekt trägt `name`=Walter Ott Rohstoff- "
                "Recycling, `operator`=Walter Ott Rohstoff- Recycling GmbH & Co. KG, "
                "`amenity`=recycling ✓ ✓")
        self.assertEqual(mod.behauptungen(satz), [])

    def test_ein_einzelnes_paar_bleibt_eine_behauptung(self):
        satz = "das Feld `name`= wurde korrigiert"
        felder = {f for f, _ in mod.behauptungen(satz)}
        self.assertEqual(felder, {"name"})

    def test_dossier_marke_schuetzt_vor_dem_ausblenden(self):
        # Redet der Satz ueber das Dossier **und** behauptet eine Aenderung ✓ ✓,
        # wird **nichts** ausgeblendet ✓ ✓ - sonst ginge eine echte Behauptung
        # verloren ✓ ✓
        satz = "im Frontmatter wurde `name`= neu gesetzt"
        felder = {f for f, _ in mod.behauptungen(satz)}
        self.assertEqual(felder, {"name"})


class AttributlisteMitDatum(unittest.TestCase):
    def test_ein_datum_zerbricht_die_liste_nicht(self):
        satz = ("**Version 2 (13.06.2017)** setzt `name`=„AWO - Begegnungsstätte "
                "Neckarstraße“, **Version 3 (16.04.2020)** `name`=„AWO Beratungszentrum“ und "
                "**Version 4 (13.09.2024)** `name`=„AWO Beratungszentrum“.")
        felder = {f for f, _ in mod.behauptungen(satz)}
        self.assertEqual(felder, set())

    def test_ohne_datum_wird_die_liste_ebenfalls_ausgeblendet(self):
        satz = "Das Objekt trägt `addr:housenumber`=34 und `addr:street`=Neckarstraße."
        felder = {f for f, _ in mod.behauptungen(satz)}
        self.assertEqual(felder, set())


class EigenerDossierwert(unittest.TestCase):
    """Die **eigenen** Werte des Dossiers ✓✓ - Name und Straße ✓✓"""

    def test_name_und_strasse_werden_gelesen(self):
        notiz = ("- [Korrektur 24.11.2026: **Dossier** ✓✓: **Nagel** ✓✓, "
                 "Straße **Am Gürtel 3** ✓✓ ✓✓]")
        self.assertEqual(mod.DOSSIER_NAME_RE.search(notiz).group(1), "Nagel")
        self.assertEqual(mod.DOSSIER_STRASSE_RE.search(notiz).group(1), "Am Gürtel 3")

    def test_anfuehrungszeichen_werden_entfernt(self):
        self.assertEqual(mod.ohne_anfuehrungszeichen("'Nagel'"), "Nagel")
        self.assertEqual(mod.ohne_anfuehrungszeichen("'Am Gürtel 3'"), "Am Gürtel 3")
        self.assertEqual(mod.ohne_anfuehrungszeichen(""), "")

    def test_ein_apostroph_zerlegt_den_namen_nicht(self):
        self.assertEqual(mod.ohne_anfuehrungszeichen("Rocky's Schrotthandel"),
                         "Rocky's Schrotthandel")


class Strassenvergleich(unittest.TestCase):
    """Die **Strasse** ✓✓ der Notiz ✓✓ gegen das Frontmatter ✓✓

    (Befund aus dem Review vom 24.11.2026 ✓✓) ✓✓ - die Notiz darf die
    Strasse **ohne** ✓✓ Hausnummer ✓✓ nennen ✓✓, und „Str.“ ✓✓ ist
    „Straße“ ✓✓ ✓ ✓
    """

    def test_name_und_hausnummer_getrennt(self):
        self.assertEqual(mod.strasse_zerlegen("Am Neuen Baum 4"),
                         ("Am Neuen Baum", "4"))
        self.assertEqual(mod.strasse_zerlegen("Stolzenmorgen 80-82"),
                         ("Stolzenmorgen", "80-82"))
        self.assertEqual(mod.strasse_zerlegen("„Obergasse“"),
                         ("Obergasse", ""))
        self.assertEqual(mod.strasse_zerlegen("Bergstr. 6"),
                         ("Bergstraße", "6"))

    def test_die_abkuerzung_ist_die_strasse(self):
        self.assertEqual(mod.strasse_zerlegen("Bergstr. 6")[0].lower(),
                         mod.strasse_zerlegen("Bergstraße")[0].lower())

    def test_ein_gemeinsames_wort(self):
        self.assertTrue(mod.gemeinsames_wort("Am Neuen Baum", "Am Neuen Baum 4"))
        # **normalisiert** ✓✓ - „Bergstr.“ ✓✓ ist „Bergstraße“ ✓✓ ✓ ✓
        self.assertTrue(mod.gemeinsames_wort(mod.strasse_zerlegen("Bergstr. 6")[0],
                                             "Bergstraße"))
        self.assertFalse(mod.gemeinsames_wort("kein", "Am Seiderberg 2"))

    def test_eine_strassenbehauptung(self):
        self.assertTrue(mod.strassenbehauptung("Am Neuen Baum"))
        self.assertTrue(mod.strassenbehauptung("Bergstraße"))
        # ein **Satz** ✓✓ und ein **Funktionswort** ✓✓ sind **keine** ✓✓
        # Strassenbehauptung ✓✓ ✓ ✓
        self.assertFalse(mod.strassenbehauptung(
            ": im Abruf ist weder die Nummer lesbar"))
        self.assertFalse(mod.strassenbehauptung("kein"))

    def test_die_straße_steht_nicht_in_einem_fetten_wort(self):
        # "**Straße** **: im Abruf ist ...**" ✓✓ ist **keine** ✓✓
        # Strassenbehauptung ✓✓ (Befund aus dem Review vom 24.11.2026 ✓✓) ✓✓
        self.assertIsNone(mod.DOSSIER_STRASSE_RE.search(
            "die **Straße** **: im Abruf ist weder die Nummer lesbar**"))

    def test_ein_einzelnes_osm_paar_mit_doppelpunkt_ist_ausgeblendet(self):
        satz = "Das Objekt traegt (`addr:city`=Stuttgart) und das Feld bleibt."
        self.assertNotIn("city", mod.osm_tags_maskieren(satz))



if __name__ == "__main__":
    unittest.main(verbosity=2)
