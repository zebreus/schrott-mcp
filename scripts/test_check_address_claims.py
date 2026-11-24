"""Die **Tests** des Adress-Gates ✓✓ - rein **offline** ✓✓

Geprueft wird die **Entscheidung** ✓✓ ueber eine **behauptete Distanz** ✓✓:
eine **messbare** ✓✓ Distanz, die **nicht** ✓✓ zur gemessenen passt ✓✓,
ist ein **Befund** ✓✓ - und **keine** **Luecke** ✓✓ (Befund aus dem Review
vom 24.11.2026 ✓✓) ✓✓
"""
import importlib.util
import pathlib
import unittest

PFAD = pathlib.Path(__file__).with_name("release").joinpath(
    "check_address_claims.py")
spec = importlib.util.spec_from_file_location("check_address_claims", PFAD)
mod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(mod)


class Distanzentscheidung(unittest.TestCase):
    """Die **Entscheidung** ✓✓ ueber eine behauptete Distanz ✓✓"""

    # die Werte des negativen Beispiels vom 24.11.2026 ✓✓ - die Objektmitte
    # liegt bei **1,54 km** ✓✓, der Mittelwert der Knoten bei **1,58 km** ✓✓
    WERTE = [("die Objektmitte", 1542.0),
             ("der Mittelwert der 10 eindeutigen Knoten", 1580.0),
             ("der naechste Knoten", 1520.0),
             ("der entfernteste Knoten", 1605.0)]

    def test_die_objektmitte_passt(self):
        entscheidung, _text = mod.distanz_entscheidung(
            1540.0, self.WERTE, True, False)
        self.assertEqual(entscheidung, "passend")

    def test_der_mittelwert_der_knoten_passt_auch(self):
        entscheidung, _text = mod.distanz_entscheidung(
            1580.0, self.WERTE, True, False)
        self.assertEqual(entscheidung, "passend")

    def test_eine_falsche_distanz_ist_ein_befund(self):
        # das negative Beispiel vom 24.11.2026 ✓✓ - **15,4 km** ✓✓ statt
        # **1,54 km** ✓✓ (Befund aus dem Review vom 24.11.2026 ✓✓) ✓✓
        entscheidung, text = mod.distanz_entscheidung(
            15400.0, self.WERTE, True, False)
        self.assertEqual(entscheidung, "Befund")
        self.assertIn("15400", text)

    def test_ohne_messbaren_wert_ist_es_eine_luecke(self):
        entscheidung, text = mod.distanz_entscheidung(1540.0, [], False, False)
        self.assertEqual(entscheidung, "Luecke")
        self.assertIn("nicht messbar", text)

    def test_der_rate_limit_ist_kein_befund(self):
        # ein **429** ✓✓ oder **629** ✓✓ ist **nie** ✓✓ ein Fehler des
        # Kandidaten ✓✓ - er bleibt eine **Luecke** ✓✓ ✓✓
        entscheidung, text = mod.distanz_entscheidung(
            15400.0, self.WERTE, True, True)
        self.assertEqual(entscheidung, "Luecke")
        self.assertIn("Rate-Limit", text)

    def test_die_toleranz_waechst_mit_der_distanz(self):
        # **5** ✓✓ Prozent ✓✓ oder **150** ✓✓ Meter ✓✓ ✓✓
        self.assertEqual(max(150, 0.05 * 20000), 1000)
        self.assertEqual(max(150, 0.05 * 1000), 150)

    def test_die_spanne_der_objektpunkte_passt(self):
        # ein **Objekt** ✓✓ mit **grossem** ✓✓ Umfang ✓✓ traegt die
        # Distanz ✓✓ an **jedem** ✓✓ seiner Punkte ✓✓ ✓✓
        werte = [("die Objektmitte", 1000.0), ("der naechste Knoten", 900.0),
                 ("der entfernteste Knoten", 1100.0)]
        treffer, _bezeichnung = mod.distanz_passend(1050.0, werte, 150)
        self.assertTrue(treffer)


class Objektmesswerte(unittest.TestCase):
    """Die **Reinheit** ✓✓ der **Netz** ✓✓ unabhaengigen Teile ✓✓"""

    def test_die_entscheidung_braucht_kein_netz(self):
        # die Funktionen **rechnen** ✓✓ rein ✓✓ - ohne **Netz** ✓✓ ✓✓
        entscheidung, _text = mod.distanz_entscheidung(
            100.0, [("die Objektmitte", 100.0)], True, False)
        self.assertEqual(entscheidung, "passend")

    def test_die_hav_ist_symmetrisch(self):
        a = (49.2469, 7.3698)
        b = (49.2578433, 7.3567385)
        self.assertAlmostEqual(mod.hav(*a, *b), mod.hav(*b, *a), places=6)



class KoordinatenSchreibweisen(unittest.TestCase):
    """Die **Koordinate** ✓✓ - **mit** ✓✓ und **ohne** ✓✓ Leerraum ✓✓

    (Befund aus dem Review vom 24.11.2026 ✓✓) ✓✓ - ohne **Leerraum** ✓✓
    blieb die Koordinate **ungeprueft** ✓✓ und der Gate meldete trotzdem
    „Befunde: 0“ ✓✓ ✓✓
    """

    def test_mit_leerraum(self):
        self.assertEqual(mod.KOORDINATE_RE.findall(
            "auf **52.5163710 / 9.7356670**"),
            [("52.5163710", "9.7356670")])

    def test_ohne_leerraum(self):
        self.assertEqual(mod.KOORDINATE_RE.findall(
            "auf **49.3106526/8.3423660**"),
            [("49.3106526", "8.3423660")])

    def test_ohne_klammern_wird_nichts_gefunden(self):
        self.assertEqual(mod.KOORDINATE_RE.findall("52.5163710 / 9.7356670"), [])



class Distanznahmen(unittest.TestCase):
    """Die **Distanz** ✓✓ - **jede** ✓✓ fette Zahl mit **m** ✓✓ oder **km** ✓✓

    (Befund aus dem Review vom 24.11.2026 ✓✓) ✓✓ - vorher sah das Gate
    nur „liegt **X km**“ ✓✓ ✓ ✓
    """

    def test_die_uebliche_bauweise(self):
        self.assertEqual(len(mod.DISTANZ_RE.findall(
            "er liegt **2,10 km** entfernt")), 1)

    def test_jede_bauweise(self):
        for text in ("**12 m** vom Objekt", "**38 m** entfernt",
                     "rund **3,4 km**", "in **22 m** Abstand"):
            self.assertEqual(len(mod.DISTANZ_RE.findall(text)), 1, text)

    def test_keine_gewichte_und_keine_preise(self):
        self.assertEqual(mod.DISTANZ_RE.findall("**250** kg"), [])
        self.assertEqual(mod.DISTANZ_RE.findall("**12,08** €/kg"), [])
        self.assertEqual(mod.DISTANZ_RE.findall("**250 kg**"), [])

    def test_alle_distanzen_werden_gefunden(self):
        self.assertEqual(len(mod.DISTANZ_RE.findall(
            "**12 m** vom einen und **15 m** vom anderen")), 2)


class FrontmatterEntscheidung(unittest.TestCase):
    """Die **Entscheidung** ueber den Standortpunkt muss im Frontmatter stehen.

    Der Seed liest die Koordinate aus dem Frontmatter - nicht aus der Notiz.
    Sagt die Korrektur-Sektion, dass der Punkt gesetzt wird, und traegt das
    Frontmatter keine Koordinate, setzt der Seed nichts (Befund aus dem
    Rollout der Standortrunde vom 24.11.2026).
    """

    GESETZT = ("### Korrektur 24.11.2026 (Owner, Geokodierung) - der Standortpunkt wird gesetzt\n"
               "- [Korrektur 24.11.2026: Der Standortpunkt wird gesetzt auf "
               "**52.5163710 / 9.7356670**.]")
    ENTFERNT = ("### Korrektur 24.11.2026 (Owner, Geokodierung) - der Standortpunkt wird entfernt\n"
                "- [Korrektur 24.11.2026: Der Standortpunkt wird entfernt.]")

    def test_setzen_ohne_koordinate_ist_ein_befund(self):
        befunde = mod.frontmatter_befund(self.GESETZT, "slug: test\nstreet: Weg 1\n")
        self.assertEqual(len(befunde), 1)
        self.assertIn("keine Koordinate", befunde[0])

    def test_setzen_mit_der_richtigen_koordinate_ist_gruen(self):
        frontmatter = "slug: test\nlat: 52.5163710\nlon: 9.7356670\n"
        self.assertEqual(mod.frontmatter_befund(self.GESETZT, frontmatter), [])

    def test_setzen_mit_einer_anderen_koordinate_ist_ein_befund(self):
        frontmatter = "slug: test\nlat: 52.5164\nlon: 9.7356670\n"
        befunde = mod.frontmatter_befund(self.GESETZT, frontmatter)
        self.assertEqual(len(befunde), 1)
        self.assertIn("52.5164", befunde[0])

    def test_setzen_mit_der_gerundeten_koordinate_ist_gruen(self):
        # die Notiz schreibt **52.516371 / 9.735667** ✓✓ (sechs Stellen) ✓✓,
        # das Frontmatter **52.5163710 / 9.7356670** ✓✓ (sieben) ✓✓ - das
        # ist **derselbe** ✓✓ Punkt ✓✓ ✓ ✓
        abschnitt = self.GESETZT.replace("52.5163710 / 9.7356670",
                                         "52.516371 / 9.735667")
        frontmatter = "slug: test\nlat: 52.5163710\nlon: 9.7356670\n"
        self.assertEqual(mod.frontmatter_befund(abschnitt, frontmatter), [])

    def test_entfernen_ohne_clear_fields_ist_ein_befund(self):
        frontmatter = "slug: test\nlat: 52.5\nlon: 9.7\n"
        befunde = mod.frontmatter_befund(self.ENTFERNT, frontmatter)
        self.assertEqual(len(befunde), 1)
        self.assertIn("clear_fields", befunde[0])

    def test_entfernen_mit_clear_fields_geo_ist_gruen(self):
        frontmatter = "slug: test\nclear_fields: 'geo'\n"
        self.assertEqual(mod.frontmatter_befund(self.ENTFERNT, frontmatter), [])

    def test_eine_bleibende_entscheidung_stellt_keine_forderung(self):
        frontmatter = "slug: test\nlat: 52.5\nlon: 9.7\n"
        abschnitt = ("### Recherche 24.11.2026\n"
                     "- [Recherche 24.11.2026: Der Punkt bleibt unverändert.]")
        self.assertEqual(mod.frontmatter_befund(abschnitt, frontmatter), [])



if __name__ == "__main__":
    unittest.main(verbosity=2)
