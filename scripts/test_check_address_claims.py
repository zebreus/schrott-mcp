#!/usr/bin/env python3
"""Die **Contract-Tests** des Adress-Gates ✓✓

Das Gate prueft die Adressangaben der Korrektur-Sektionen des Owners live
✓✓ - gegen die OSM-API ✓✓. Diese Tests pruefen den **Textteil** ✓✓ des
Gates ✓✓: die Satzteilung ✓✓, das Lesen der Hausnummern ✓✓ und die
Erkennung der **beiden** Setzformen ✓✓ ✓ ✓

Ohne Netz ✓✓ - die Tests rufen nur die reinen Funktionen ✓✓ ✓ ✓
"""
import importlib.util
import unittest
from pathlib import Path

SPEICHER = Path(__file__).resolve().parent / "release" / "check_address_claims.py"
spez = importlib.util.spec_from_file_location("check_address_claims", SPEICHER)
cac = importlib.util.module_from_spec(spez)
spez.loader.exec_module(cac)


class Satzteilung(unittest.TestCase):
    def test_doppelpunkt_trennt_den_vorspann(self):
        saetze = cac.saetze("Der Beleg für die Adresse: der **Standortfinder** ✓✓")
        self.assertEqual(len(saetze), 1)
        self.assertIn("Standortfinder", saetze[0])

    def test_doppelpunkt_mit_sternen_trennt_den_vorspann(self):
        saetze = cac.saetze("**Reichweite:** Die EfbV ist freiwillig")
        self.assertEqual(len(saetze), 2)

    def test_punkt_trennt(self):
        saetze = cac.saetze("Ein Satz. Der andere Satz.")
        self.assertEqual(len(saetze), 2)

    def test_punkt_in_einer_koordinate_trennt_nicht(self):
        saetze = cac.saetze("Punkt **47.6759079/9.4802246** gesetzt. Der Rest.")
        self.assertEqual(len(saetze), 2)
        self.assertIn("47.6759079/9.4802246", saetze[0])


class Hausnummern(unittest.TestCase):
    def test_hausnummer_als_feld(self):
        self.assertEqual(cac.hausnummern("mit der **Hausnummer 26** ✓✓"), {"26"})

    def test_hausnummer_nach_einem_namen(self):
        self.assertEqual(cac.hausnummern("**An der Pumpe 7** ✓✓"), {"7"})

    def test_hausnummer_nach_einer_strasse(self):
        self.assertEqual(cac.hausnummern("Auf **Marienstraße 26** ✓✓"), {"26"})

    def test_datum_ist_keine_hausnummer(self):
        self.assertEqual(cac.hausnummern("Korrektur 23.11.2026, Abruf 23.11.2026"), set())

    def test_zertifikatsnummer_ist_keine_hausnummer(self):
        self.assertEqual(
            cac.hausnummern("Zertifikat `ZALS00800033` ✓✓"), set())

    def test_menge_ist_keine_hausnummer(self):
        self.assertEqual(cac.hausnummern("ab 23 kg zertifiziert"), set())

    def test_zahl_im_firmierungsnamen_ist_keine_hausnummer(self):
        self.assertEqual(
            cac.hausnummern("(`Autoverwertung Miltenberg 579 GmbH` ✓✓)"), set())

    def test_zahl_im_osm_wert_ist_keine_hausnummer(self):
        self.assertEqual(
            cac.hausnummern("`name`=Autoverwertung Miltenberg 579 ✓✓"), set())

    def test_adresse_mit_leerzeichen(self):
        self.assertEqual(
            cac.hausnummern("**7, Grünwaldstraße, Hördt** ✓✓"), {"7"})

    def test_hausnummer_mit_sternen_davor(self):
        # Die **Sterne** ✓✓ um die Hausnummer ✓✓ sind **Format** ✓✓ - der
        # Befund aus dem Review vom 24.11.2026 ✓✓ (Befund 1) ✓✓
        self.assertEqual(cac.hausnummern("(Obergasse **8D** ✓✓)"), {"8D"})

    def test_hausnummer_mit_grossbuchstabensuffix(self):
        self.assertEqual(
            cac.hausnummern("(Rudolf-Diesel-Straße **17A** ✓✓)"), {"17A"})

    def test_hausnummer_mit_grossbuchstabensuffix_als_feld(self):
        self.assertEqual(cac.hausnummern("**Hausnummer 8D** ✓✓"), {"8D"})

    def test_distanz_ist_keine_hausnummer(self):
        # Eine **Distanz** ✓✓ ist keine **Hausnummer** ✓✓ - der Befund aus
        # dem Review vom 24.11.2026 ✓✓ (Befund 1) ✓✓
        self.assertEqual(
            cac.hausnummern("Dieses Dossiers liegt **893 m** von der "
                            "belegten Betriebsanschrift entfernt"), set())

    def test_distanz_mit_dezimalkomma_ist_keine_hausnummer(self):
        self.assertEqual(
            cac.hausnummern("Der Punkt liegt **0,03 km** ✓✓ vom Adressobjekt "
                            "entfernt ✓✓"), set())


class Adressfall(unittest.TestCase):
    def test_bleibt_wird_geprueft(self):
        abschnitt = ("### Korrektur 24.11.2026 (Owner, Geokodierung) — "
                     "der Standortpunkt bleibt unverändert\n")
        self.assertTrue(cac.adressfall_ist(abschnitt, False))

    def test_entfernung_wird_nicht_geprueft(self):
        abschnitt = ("### Korrektur 24.11.2026 (Owner, Geokodierung) — "
                     "der falsche Standortpunkt wird entfernt\n")
        self.assertFalse(cac.adressfall_ist(abschnitt, False))

    def test_setzen_wird_geprueft(self):
        abschnitt = ("### Korrektur 24.11.2026 (Owner, Geokodierung) — "
                     "der Standortpunkt wird ersetzt\n")
        self.assertTrue(cac.adressfall_ist(abschnitt, True))


class Korrekturabschnitt(unittest.TestCase):
    def test_der_letzte_abschnitt_gilt(self):
        text = ("### Korrektur 22.11.2026 (Owner) — der Standortpunkt wird gesetzt\n"
                "- [Korrektur 22.11.2026: alt]\n"
                "### Korrektur 23.11.2026 (Owner) — der Standortpunkt wird ersetzt\n"
                "- [Korrektur 23.11.2026: neu]\n")
        abschnitt = cac.korrektur_abschnitt(text)
        self.assertIn("23.11.2026", abschnitt)
        self.assertNotIn("alt", abschnitt)

    def test_ohne_korrektur_ist_es_nichts(self):
        self.assertIsNone(cac.korrektur_abschnitt("### Recherche 23.11.2026\n- [x]\n"))


class Ortsvergleich(unittest.TestCase):
    def test_der_ort_des_dossiers_passt_zum_ort_des_objekts(self):
        self.assertTrue(cac.platz_passt("Gütersloh", "Gütersloh"))

    def test_der_stadtteil_des_objekts_passt_auch(self):
        self.assertTrue(cac.platz_passt("Heidesheim", "Heidesheim am Rhein"))

    def test_ein_ortsname_ist_kein_wort_in_einem_anderen(self):
        self.assertFalse(cac.platz_passt("Halle", "Hallerbach"))

    def test_eine_andere_gemeinde_passt_nicht(self):
        self.assertFalse(cac.platz_passt("Gütersloh", "Mülheim an der Ruhr"))

    def test_der_postalische_ort_ohne_plz(self):
        self.assertEqual(cac.ort_ohne_plz("Heidesheim 55262"), "Heidesheim")

    def test_der_ort_ohne_plz_bleibt_der_ort(self):
        self.assertEqual(cac.ort_ohne_plz("Heidesheim"), "Heidesheim")


class RateLimit(unittest.TestCase):
    def test_die_wartezeiten_wachsen(self):
        self.assertEqual(cac.WARTEZEITEN, tuple(sorted(cac.WARTEZEITEN)))
        self.assertGreater(len(cac.WARTEZEITEN), 1)

    def test_ein_429_wird_als_rate_limit_vermerkt(self):
        import urllib.error
        schluessel = ("way", "1")
        cac.TAGS_CACHE.pop(schluessel, None)
        cac.RATE_LIMIT.discard(schluessel)
        klasse = type("F", (Exception,), {})
        fehler = urllib.error.HTTPError("u", 429, "Too Many Requests", None, None)
        # der Abruf wird **simuliert** ✓✓ - der Test greift **nicht** ins
        # Netz ✓✓ und prueft **nur** die **Vermerkung** ✓✓ ✓ ✓
        original = urllib.request.urlopen

        def kaputt(*args, **kwargs):
            raise fehler

        urllib.request.urlopen = kaputt
        try:
            cac.WARTEZEITEN = (0, 0, 0)
            ergebnis = cac.hole_tags("way", "1")
        finally:
            urllib.request.urlopen = original
            cac.WARTEZEITEN = (3, 8, 20)
        self.assertIsNone(ergebnis)
        self.assertIn(schluessel, cac.RATE_LIMIT)
        cac.RATE_LIMIT.discard(schluessel)


class RateLimit629(unittest.TestCase):
    def test_629_wird_als_rate_limit_vermerkt(self):
        import urllib.error
        schluessel = ("way", "2")
        cac.TAGS_CACHE.pop(schluessel, None)
        cac.RATE_LIMIT.discard(schluessel)
        fehler = urllib.error.HTTPError("u", 629, "Too Many Requests", None, None)

        def kaputt(*args, **kwargs):
            raise fehler

        original = urllib.request.urlopen
        urllib.request.urlopen = kaputt
        try:
            cac.WARTEZEITEN = (0, 0, 0)
            ergebnis = cac.hole_tags("way", "2")
        finally:
            urllib.request.urlopen = original
            cac.WARTEZEITEN = (3, 8, 20)
        self.assertIsNone(ergebnis)
        self.assertIn(schluessel, cac.RATE_LIMIT)
        cac.RATE_LIMIT.discard(schluessel)


if __name__ == "__main__":
    unittest.main()
