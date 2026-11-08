#!/usr/bin/env python3
"""Abfrageliste fuer das amtliche Fachbetrieberegister (Shard S2, Welle 09.11.2026, Slot 00:00 (Recherchetag 08.11.2026)).

Nur lesende oeffentliche Abfragen. Jede Zeile ist eine JSON-Abfrage:
  route: "Entsorgungsfachbetriebe" | "Altfahrzeugverwertung"
  name:  optionale Namenssuche (dst=10)
  adr/lat/lon/dst: Standortabfrage mit Radius
"""
import json

EFB = "Entsorgungsfachbetriebe"
AFV = "Altfahrzeugverwertung"

# slug -> (Namensvarianten, Standortabfragen [(adr, lat, lon, dst)])
PLAN = [
    ("nw-selfkant-kusters", ["Küsters", "Kuesters"],
     [("Hochstraße 36, 52538 Selfkant, Deutschland", 5.9077749, 51.0493805, 3),
      ("52538 Selfkant-Schalbruch, Deutschland", 5.9077749, 51.0493805, 10)]),
    ("nw-minden-moller", ["Möller", "Moeller"],
     [("Schaumburger Weg 1b, 32423 Minden, Deutschland", 8.9664915, 52.2880498, 3),
      ("Teichstraße 1, 32423 Minden, Deutschland", 8.9382590, 52.3057130, 3)]),
    ("nw-oelde-warnecke", ["Warnecke"],
     [("Ennigerloher Straße 84, 59302 Oelde, Deutschland", 8.1261193, 51.8248840, 3),
      ("59302 Oelde, Deutschland", 8.1261193, 51.8248840, 10)]),
    ("nw-munster-robering", ["Robering"],
     [("Feldkamp 19, 48165 Münster, Deutschland", 7.6722918, 51.9037381, 3),
      ("48165 Münster-Hiltrup, Deutschland", 7.6722918, 51.9037381, 10)]),
    ("nw-koln-b-v", ["B&V Schrott", "B & V"],
     [("Horremer Straße 6, 50933 Köln-Müngersdorf, Deutschland", 6.8836382, 50.9423617, 3),
      ("50933 Köln-Müngersdorf, Deutschland", 6.8836382, 50.9423617, 10)]),
    ("nw-solingen-karl-wolfgang-torner", ["Torner"],
     [("Oben zum Holz 60, 42653 Solingen, Deutschland", 7.0944167, 51.2082455, 3),
      ("42653 Solingen-Gräfrath, Deutschland", 7.0944167, 51.2082455, 10)]),
    ("nw-munster-schrottdirektor", ["Schrottdirektor"],
     [("Fritz-Pütter-Straße 13, 48153 Münster, Deutschland", 7.6282875, 51.9366874, 3),
      ("48153 Münster, Deutschland", 7.6282875, 51.9366874, 10)]),
    ("nw-remscheid-friedrich-ernst-wilms", ["Wilms", "Friedrich Ernst Wilms"],
     [("Reinshagener Straße 21, 42857 Remscheid, Deutschland", 7.1589488, 51.1650985, 3),
      ("42857 Remscheid, Deutschland", 7.1589488, 51.1650985, 10)]),
    ("nw-essen-schuttler", ["Schüttler", "Schuettler"],
     [("Stubbenkamp 6, 45309 Essen, Deutschland", 7.0775926, 51.4732855, 3),
      ("45309 Essen-Kray, Deutschland", 7.0775926, 51.4732855, 10)]),
    ("nw-stolberg-motter", ["Motter"],
     [("Auf der Mühle 26, 52222 Stolberg, Deutschland", 6.2210642, 50.7780362, 3),
      ("52222 Stolberg-Münsterbusch, Deutschland", 6.2210642, 50.7780362, 10)]),
    ("nw-wipperfurth-containerdienst-jurgen-keller", ["Keller", "Containerdienst Keller"],
     [("Alte Papiermühle 14, 51688 Wipperfürth, Deutschland", 7.3558502, 51.1289291, 3),
      ("51688 Wipperfürth, Deutschland", 7.3558502, 51.1289291, 10)]),
    ("nw-oberhausen-ebh-schrott-recycling-umwelttechnik", ["EBH", "EBH Schrott"],
     [("Tackstraße 21, 46047 Oberhausen, Deutschland", 6.7898376, 51.5410802, 3),
      ("46047 Oberhausen, Deutschland", 6.7898376, 51.5410802, 10)]),
    ("nw-leichlingen-muller", ["Müller Metallankauf", "Mueller Metallankauf"],
     [("Metzholz 68, 42799 Leichlingen, Deutschland", 7.0848389, 51.1084208, 3),
      ("42799 Leichlingen, Deutschland", 7.0848389, 51.1084208, 10)]),
]


def main():
    out = []
    for slug, names, locs in PLAN:
        for n in names:
            out.append({"slug": slug, "route": EFB, "name": n, "dst": 10})
            out.append({"slug": slug, "route": AFV, "name": n, "dst": 10})
        for adr, lon, lat, dst in locs:
            out.append({"slug": slug, "route": EFB, "adr": adr, "lat": lat, "lon": lon, "dst": dst})
            out.append({"slug": slug, "route": AFV, "adr": adr, "lat": lat, "lon": lon, "dst": dst})
    for o in out:
        print(json.dumps(o, ensure_ascii=False))


if __name__ == "__main__":
    main()
