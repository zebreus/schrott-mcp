# Wave F Süd — 12.10.2026

## Scope and method

Reviewed the 20 requested dossiers, using their existing timelines as leads and re-checking the names independently against operator pages, register information, municipal pages and direct directory pages. Dated notes distinguish direct page reads from search-index snippets and aggregators. Under the README source hierarchy, multiple directories still count as leads rather than qualifying evidence; no status, owner, address, service or fee has been upgraded on directory consensus alone.

No code, shared documentation, coordinates or production database were changed or accessed. Agent-authored edits are limited to the 20 requested dossier files and this report. The shared worktree showed unrelated edits to other dossier files during the task; those were not touched and are not attributed to this wave.

## Key improvements

- **SM Metals:** found the matching current operator domain, `sm-metals.com`, and read its Impressum, contact and services pages. The GmbH, Lampertheim address, phone, email and hours match; the formerly noted `sm-metallhandel.com` belongs to a differently named UG with no proven connection and remains separate. The website advertises individualized daily purchase offers, not fixed prices. The street address is labeled as a postal address; public drop-off terms are not clear, so no `dropoff_json` was added.
- **CR Recycling:** matched the operator Impressum and company pages to Oberderdingen’s municipal business-directory entry. Added the corroborated address, website and a concise description. The municipality and operator site disagree on phone/email, so neither was seeded. The operator’s Konditionen page says each quote depends on material composition, metal markets, processing effort and hazardous-waste disposal; it is not a numeric list or clear statement of payout versus charge.
- **Agosi:** added the operator’s current contact and distinguished the Kanzlerstrasse contact address, Robert-Bauer-Straße customer counter and the NE-metal receiving location at Am Hauptgüterbahnhof 24. A dated operator rate table was read directly (09.10.2026); values and conditions are captured below and in the dossier.
- **Ladner Recycling:** rechecked the municipal Sontra waste page and North Data. The city names the Ladner premises at Brodberg only for batteries/accessories and tyres with rims; North Data lists HRB 2690, Brodberg 15 and the scrap/waste/vehicle-dismantling purpose. The city’s EUR 30/fahrt is for municipal Bauhof collection of electrical devices, **not** a Ladner tariff.
- **Identity/data hygiene:** removed the unqualified Napoleonstr. 19 association from Schrottmeister Rhein-Main: the directly read operator pages identify that address as Schrott Experte (Zohir Boulahoual), while alias/duplicate status is unresolved. Downgraded Dreieich Stefan Schramm and Freiburg Sahin from `aktiv` to `pruefung`; the supporting evidence was directory-only or an unresolved cross-identity. Removed directory-only contact/address values from Schramm, Eibl, Hoffmann, Augsburg Reinhardt, Cimen, Rosel and Kohlmann. Normalized duplicate city/postcode text where applicable. Preserved prior values in the dossier history.
- **Website-status correction:** B. Samuel’s domain returned transport errors for both bare and `www` URLs, not a verifiable deletion response. Changed `website_status` from `tot` to `unbekannt`; this does not establish that the site is live.

## Dossier-by-dossier findings and open questions

| Slug | Verified finding / change | Open question and price boundary |
|---|---|---|
| `he-heidenrod-schrottmeister-rhein-main` | `schrott-experte.de` directly names Zohir Boulahoual at Napoleonstr. 19, 65321 Heidenrod, phone 0178 161 8017. Removed that street from this dossier because it is evidenced for the differently named Schrott Experte. [Contact](https://schrott-experte.de/kontakt/) · [Impressum](https://schrott-experte.de/impressum/) | Is Schrottmeister an alias/duplicate of Schrott Experte or a distinct entity? No prices from Schrott Experte were transferred. |
| `he-dreieich-stefan-schramm` | The Herrnroth 11 / 06103 322820 details are directory leads (tellows/Cylex), not qualifying operator/register evidence. Cleared street, postcode, phone and `aktiv`. [tellows](https://tellows.de/num/06103322820) | Confirm operator and current activity; no verified purchase, sale or fee prices. |
| `bw-freiburg-manuel-reinhardt` | Search-index snippets associate the name with Auggener Weg 9, but show different phone numbers; direct Cylex returned 403 and Schrottfinder returned 404. No field fill. [Cylex lead](https://web2.cylex.de/firma-home/manuel-reinhardt-firma-schrotthaendler-13861178.html) · [Schrottfinder lead](https://schrottfinder.de/manuel-reinhardt-firma-schrotthaendler-freiburg/) | Obtain an owner/register source; do not merge with other Reinhardt records. No verified prices. |
| `he-lampertheim-68623-sm-metals` | Operator Impressum/contact confirm SM Metals GmbH, Wilhelm-Herz-Ring 2, 68623 Lampertheim, 06206 9510626, info@sm-metals.com and Mo–Fr 08:00–16:30 / Sa 09:00–14:00. [Impressum](https://www.sm-metals.com/impressum/) · [Kontakt](https://www.sm-metals.com/kontakt/) · [Leistungen](https://www.sm-metals.com/leistungen/) | Buys FE/NE metals and cables after analysis for an individual daily quote; no numeric purchase list. No public selling list or tariff schedule. Confirm whether/how private customers may deliver. |
| `he-fulda-mittelrode-bohm-michael-schrotthandel` | Gelbe Seiten directly lists Lerchenstieg 4, 36041 Fulda-Mittelrode and 0661 9015432; it remains a single-source directory lead. Removed phone from frontmatter. [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/277dec5c-d78e-4ff8-b2b8-190008dbf01c) | Confirm operator, duplicate listing and customer acceptance. No price list. |
| `he-sontra-36205-ladner-franz` | City page directly says “Betriebsgelände der Firma Ladner, Brodberg” for batteries/accessories and tyres/rims; North Data lists Ladner Recycling GmbH, HRB 2690, Brodberg 15. [Sontra](https://www.sontra.de/seite/254956/sonstiger-abfall.html) · [North Data](https://www.northdata.com/Ladner%20Recycling%20GmbH,%20Sontra/Amtsgericht%20Eschwege%20HRB%202690) | This does not prove general scrap intake or purchase. No Ladner price list or fees found; operator domain direct fetch failed. Do not attribute the municipal Bauhof’s EUR 30/fahrt to Ladner. |
| `bw-pforzheim-agosi-ag` | Direct operator contact page separates the Kanzlerstrasse contact, Robert-Bauer-Straße customer counter and Am Hauptgüterbahnhof 24 NE-metal site; added general phone/email but left street empty to avoid mixing branches. [Contact](https://agosi.de/kontakt/) | Dated purchase/sale rates below; daily current quote and customer eligibility should be confirmed with Agosi. No fee schedule found. |
| `by-dietersburg-eibl` | Older directories lead to “Eibl Daniel Schrotthandel Containerdienst”, Langlohstraße 9 and 01512 3518337; no qualifying source found. Removed the seed phone. | Distinguish from Alois Eibl KFZ-Handel; confirm the trader and address. No prices/fees. |
| `bw-oberderdingen-cr-recycling-aydemir-cinar` | Operator Impressum/company pages and municipal business listing agree on CR Recycling / Aydemir Cinar at Robert-Bosch-Ring 9, 75038 Oberderdingen and `cr-recycling.de`. [Impressum](https://www.cr-recycling.de/footer-navigation/impressum/) · [Unternehmen](https://www.cr-recycling.de/unternehmen/) · [Stadt Oberderdingen](https://www.oberderdingen.de/website/de/arbeiten/firmenpark/cr_recycling) | Phone/email conflict (operator 07258 47044-0 / service@ vs city 07258 927360 / cinar@), so left blank. No numeric buying or sale list; individualized material-dependent recycling terms only. Confirm private drop-off terms. |
| `by-munchen-hoffmann-karl-heinz` | Das Örtliche and Gelbe Seiten list Rappenweg 162, 81829 München and 089 4391969; removed street as directory-only. [Das Örtliche](https://www.dasoertliche.de/Themen/Hoffmann-Karl-Heinz-M%C3%BCnchen-Trudering-Riem-Rappenweg) | Owner and current operation unverified. No prices/fees. |
| `he-altenstadt-63674-ing-norbert-szielasko-kabelzerlegungen-m` | Search returned the established 11880/address/phone listing only; frontmatter remains clean and `pruefung`. [11880](https://www.11880.com/branchenbuch/altenstadt-hessen/062392768B29597958/ing-norbert-szielasko-kabelzerlegungen-metalle.html) | Seven portal listings are still a directory cluster, not operator/register proof. No price list or confirmed intake terms. |
| `bw-freiburg-sahin-metall-schrotthandel` | Direct Trennwerk pages name Berkan Sahin and Eichstetten post address, say there is no Freiburg receiving site, and describe pickup at customer location. This does not prove the Freiburg Gelbe-Seiten record is Trennwerk; downgraded this dossier to `pruefung`. [Trennwerk](https://schrotthandel-freiburg.de/) · [Impressum](https://schrotthandel-freiburg.de/impressum/) · [Freiburg directory lead](https://www.gelbeseiten.de/gsbiz/c4fa8b20-465c-4e32-8d7a-b35700f9fab8) | Trennwerk (candidate only) advertises copper/cable purchase without numeric prices and says scrap-car collection is chargeable without stating amount. No such terms attributed to Sahin Freiburg. |
| `by-augsburg-reinhardt-robert` | Conflicting directory locations/phones (Kobelweg 78a/0821 47011181 versus Kobelweg 78 or Dr.-Nick-Str. 17/0821 717348); candidate domain transport-failed. Removed street. [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/a6f0cfa9-edd0-455f-aff0-827b71a25792) · [Schrottplatz-Info](https://www.schrottplatz-info.de/schrottplatz/Augsburg/Reinhardt-Robert-SchrottHdl-) | Resolve location/number and possible branch/move through operator or register source. No price evidence. |
| `he-frankfurt-65929-cimen` | 11880 direct page gives Königsteiner Str. 90, 65929 Frankfurt and 0179 4702226, but shows source data from 28.12.2022 despite portal update on 03.10.2026. Removed phone; retained only coarse city. [11880](https://www.11880.com/branchenbuch/frankfurt-am-main/060440092B108945313/cimen-schrotthandel.html) | No operator/register/qualifying second source. No purchase, sale or fee prices. |
| `by-erlbach-barth-markt` | 11880’s “Schrotthandel MT Barth” lists Nürnberger Str. 2, 91459 Markt Erlbach and 0177 2222491; portal entry data date 08.04.2026. Not enough to equate with seed “Barth Markt”. [11880](https://www.11880.com/branchenbuch/markt-erlbach/060440092B113946169/schrotthandel-mt-barth.html) | Confirm alias/identity and whether Erlbach means Markt Erlbach. No prices. |
| `by-bergkirchen-hartmann` | Directly accessible directory pages still disagree on Himmelreichstr. 23 / 23 A and hours; the known domain is not reliably accessible. No frontmatter fill. [Das Örtliche](https://www.dasoertliche.de/Themen/Autoverwertung-Hartmann-Bergkirchen-Neuhimmelreich-Himmelreichstra%C3%9Fe) · [Schrottplatz-Info](https://www.schrottplatz-info.de/schrottplatz/Bergkirchen/Hartmann) | Resolve Fritz/Cornelia identity and any relation to Autoneo; no verified purchase/sale/fee schedule. |
| `by-growallstadt-63868-schrott-rosel` | Gelbe Seiten lists Frankenstr. 30 and phone 06022 22697; Das Örtliche repeats the name/street. Aggregators only; removed street from frontmatter, kept as lead in notes. [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/6c4130c4-4bb0-45d6-92e3-16820c560e9c) · [Das Örtliche street index](https://www.dasoertliche.de/Themen/Frankenstra%C3%9Fe/Gro%C3%9Fwallstadt.htm) | Confirm owner and receiving location. No purchase prices, sale list or fees. |
| `he-babenhausen-64832-kohlmann-frank` | 11880 lists Am Fuchsberg 8, 64832 Babenhausen, 06073 5800, and email, but underlying entry data is 22.03.2025. Removed directory-only street/postcode/phone. [11880](https://www.11880.com/branchenbuch/babenhausen-hessen/120671391B26666241/frank-kohlmann-metallhandel.html) | Confirm business and current contact with operator/register source. No prices/fees. |
| `he-dietzenbach-b-samuel-schrottabholung` | Both direct domain variants returned transport errors; directory/search snippets are not current operator confirmation. Changed `tot` to `unbekannt`, not to `aktiv`. | “Free pickup” remains an unverified historical claim, not an Ankauf price or confirmed service tariff. No sale list. Resolve operator, address and reachability. |
| `he-wetter-niederwetter-mattern-mario-metallrecycling` | Gelbe Seiten lists Mildengrund 1, Niederwetter and 0162 1607999; an embedded GoLocal review from 2017 is Marburg-related and reuses the number. No basis for a verified move or second branch. [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/ef37fa4b-67b5-4e0c-9ca5-75be5d102720) · [GoLocal lead](https://www.golocal.de/marburg/entsorgung/recycling-fuer-schrott-e-geraete-mario-mattern-79Uk2/) | Obtain operator/register confirmation of current address; no verified purchase rates, sale list or fees. |

## Verified price-list findings

### Ankauf

Agosi is the only one of the 20 with a directly readable, numerical operator rate table. The operator’s latest table fetched on 12.10.2026 was dated **09.10.2026**. Its **Eröffnungspreise** were:

| Material | Agosi Ankauf |
|---|---:|
| Gold | 117,51 €/g |
| Silber | 1.691,40 €/kg |
| Platin | 46,70 €/g |
| Palladium | 32,55 €/g |

SM Metals advertises buying many ferrous/non-ferrous metals and cables in small or large lots, but quotes after material analysis at an individual current daily rate; the inspected pages showed no numeric list. CR Recycling’s terms page describes a case-specific assessment and quote, not a published buy-in rate. No verified numerical purchase prices were found for the remaining dossier identities.

### Verkauf

Agosi’s same 09.10.2026 **Eröffnungspreise** distinguish unprocessed and processed sale material:

| Material | Verkauf unverarbeitet | Verkauf verarbeitet |
|---|---:|---:|
| Gold | 126,95 €/g | 132,99 €/g |
| Silber | 1.880,00 €/kg | 2.019,30 €/kg |
| Platin | not listed | 51,65 €/g |
| Palladium | not listed | 36,75 €/g |

Agosi also publishes separate fixing and after-fixing figures. All figures are a dated snapshot, explicitly non-binding/indicative, relate to smaller quantities, can change rapidly, and require a current quote. No other verified numerical resale catalogue was found.

### Gebühren

- Sontra’s **EUR 30 per trip** applies to collection of electrical devices by the municipal Bauhof, not to Ladner Recycling.
- CR Recycling publishes no fixed amount; its individual recycling conditions factor in material composition, current metal values, dismantling/processing and hazardous-material disposal. The page does not reduce this to a simple service fee or Ankauf payout.
- Trennwerk’s separate Freiburg/Eichstetten candidate site calls scrap-car pickup chargeable but gives no amount; because identity with the Freiburg Sahin dossier is unresolved, it is not assigned to that dossier.
- B. Samuel’s “free pickup” persists only in historical/indexed claims and is not verified. No other verifiable tariff schedule or fee amount was found.

Absence of a readable price list is not proof that a trader has no individual quotes, prices or fees. Failed/blocked fetches and search snippets are recorded as such, not treated as source evidence.

## GEO and integration notes

No coordinates were added or changed. Address retractions above are dossier corrections only; under the seed semantics in the README, blank street/postcode fields do not by themselves clear existing database GEO. The owner should read-only check any previously stored coordinates before integrating this wave, especially for Schrottmeister Rhein-Main, Dreieich Schramm, Hoffmann München, Augsburg Reinhardt, Schrott Rosel and Kohlmann. No production query or write was performed here.

## Validation

The scoped dossier diff contains the 20 requested paths only; `git diff --check` and the report trailing-whitespace check are clean. `/root/.cargo/bin/cargo test --offline -p schrott-mcp-ingestion seed_traders` passed (11 passed, 0 failed). No production database was accessed.
