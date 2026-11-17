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


if __name__ == "__main__":
    unittest.main(verbosity=2)
