#!/usr/bin/env python3
"""Abfrageliste fuer das amtliche Fachbetrieberegister (Shard 2, Welle 08.11.2026).

Nur lesende oeffentliche Abfragen. Jede Zeile ist eine JSON-Abfrage:
  route: "Entsorgungsfachbetriebe" | "Altfahrzeugverwertung"
  name:  optionale Namenssuche (dst=10)
  adr/lat/lon/dst: Standortabfrage mit Radius
"""
import json
import sys

EFB = "Entsorgungsfachbetriebe"
AFV = "Altfahrzeugverwertung"

# slug -> (Namensvarianten, Standortabfrage oder None)
# Die Koordinaten sind live geokodiert (Nominatim/OSM, Abruf 08.11.2026).
PLAN = [
    ("he-wiesbaden-doringer-frank-schrotthandel", ["Metallentsorgung"],
     ("Fritz-Erler-Str. 47, 65207 Wiesbaden", 8.3380887, 50.1068188)),
    ("he-wiesbaden-erbenheim-scholz-rico-schrott", ["Scholz"],
     ("Rennbahnstr. 1, 65205 Wiesbaden", 8.3085013, 50.0523699)),
    ("he-wiesbaden-65199-hubinger", ["Hübinger"],
     ("Holzstraße 48A, 65197 Wiesbaden", 8.2134871, 50.0711432)),
    ("he-langenselbold-63505-hontsch-maria", ["Höntsch"],
     ("Hanauer Str. 66b, 63505 Langenselbold", 9.0296385, 50.1716806)),
    ("he-babenhausen-64832-kohlmann-frank", [],
     ("Am Fuchsberg 8, 64832 Babenhausen", 8.9643466, 49.9749685)),
    ("he-offenbach-am-main-fred-de-snoo-rohstoffe-schrott-und-metal", ["de Snoo"],
     ("Lämmerspieler Weg 158, 63075 Offenbach am Main", 8.8018503, 50.1077178)),
    ("he-messel-64409-schrottler-eddie", ["Schrottler"],
     ("Zeilharderstr. 23a, 64409 Messel", 8.7457383, 49.9243273)),
    ("he-schaafheim-64850-schrenk", ["Schrenk"],
     ("Daimlerstr. 6, 64850 Schaafheim", 8.9984064, 49.9309340)),
    ("he-limburg-65549-scheuer", ["Scheuer"],
     ("Im Elbboden 1, 65549 Limburg an der Lahn", 8.0529371, 50.4001871)),
    ("he-wetter-firma-bajramaj", ["Bajramaj"],
     ("Im Berntal 9, 35083 Wetter", 8.7362641, 50.9034911)),
    ("he-hanau-63452-guth", ["Guth"],
     ("Niddastraße 32, 63452 Hanau", 8.9398988, 50.1496126)),
    ("he-oberursel-61440-lackner", [],
     ("Pfeiffstr. 13, 61440 Oberursel", 8.5864306, 50.1936928)),
    ("he-wiesbaden-biebrich-wiesbaden-dotzheim-bert-brecht-str-26", ["Wintermeyer"],
     ("Bert-Brecht-Str. 26, 65201 Wiesbaden", 8.1950399, 50.0577522)),
    ("he-hanau-lamboy-schrott-und-metalle-reinhardt", ["Reinhardt"],
     ("Friedrich-Engels-Str. 5, 63452 Hanau", 8.9272103, 50.1437119)),
]


def main():
    out = []
    for slug, names, loc in PLAN:
        for n in names:
            out.append({"slug": slug, "route": EFB, "name": n, "dst": 10})
            out.append({"slug": slug, "route": AFV, "name": n, "dst": 10})
        if loc:
            adr, lon, lat = loc
            out.append({"slug": slug, "route": EFB, "adr": adr, "lat": lat, "lon": lon, "dst": 3})
            out.append({"slug": slug, "route": AFV, "adr": adr, "lat": lat, "lon": lon, "dst": 3})
    for o in out:
        print(json.dumps(o, ensure_ascii=False))
    print(f"# {len(out)} Abfragen", file=sys.stderr)


if __name__ == "__main__":
    main()
