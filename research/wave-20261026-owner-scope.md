# Scope der Recherchewelle: 26.10.2026

## Produktionsauswahl (read-only)

Die Welle umfasste 60 eindeutige Händler aus dem aktuellen Pool. Die Auswahl
erfolgte ausschließlich lesend mit der Abfrage aus
[`prompts/tiefenrecherche-welle.md`](../prompts/tiefenrecherche-welle.md):

```sql
SELECT slug, state FROM traders
WHERE website='' OR street=''
ORDER BY RANDOM() LIMIT 60;
```

Der Pool umfasste beim Check 1.933 Datensätze. Die drei disjunkten Shards
bearbeiteten je 20 Dossiers: Hessen (HE), Nordrhein-Westfalen (NW) und
Rheinland-Pfalz (RP). Produktionsdatenbanken blieben read-only.

## Hessen (20)

- `he-altenstadt-63674-ing-norbert-szielasko-kabelzerlegungen-m`
- `he-erlensee-ruckingen-auto-s3-autoverwertung`
- `he-fulda-malkes-krieger-stefan-schrotthandel`
- `he-fulda-mittelrode-bohm-michael-schrotthandel`
- `he-furth-odw-gerhardt-andreas`
- `he-gieen-schrotthandel-muller-andreas`
- `he-grunberg-35305-eller-schrotthandlung-av`
- `he-hanau-j-e-schrott-recycling`
- `he-heusenstamm-63150-sander-altmetall-demontage`
- `he-korbach-34497-durwald-tobias-recycling`
- `he-malsfeld-34323-eysert-karl`
- `he-obertshausen-63179-altwagenabholservice`
- `he-pfungstadt-64319-gotz-joachim`
- `he-riedstadt-af-metall-und-schrotthandel`
- `he-schaafheim-64850-he-sell-oskar-schrotthdl`
- `he-schaafheim-64850-schrenk`
- `he-seligenstadt-d-sattler-schrott-und-metallentsorgung`
- `he-waldbrunn-65620-ellar-seelbach-niedworok`
- `he-wiesbaden-mainz-kostheim-salvatore-vitale-schrott`
- `he-wiesbaden-sauerland-dienstleistung-wiesbaden`

## Nordrhein-Westfalen (20)

- `nw-bruggen-allmeta-altmetallhandel-standort-bracht`
- `nw-duisburg-rodemann`
- `nw-emsdetten-lohmann`
- `nw-erftstadt-liesegang`
- `nw-essen-firma-la-rohrproduktenhandel`
- `nw-gelsenkirchen-west-schrotthandel-sabine-lemke`
- `nw-hamm-hering`
- `nw-herne-altmetall-deniz`
- `nw-herne-schrottfuchse`
- `nw-lennestadt-l-mulaj`
- `nw-monchengladbach-jl-metall-recycling`
- `nw-nrw-weit-schrotthandler24`
- `nw-oelde-warnecke`
- `nw-ruhrgebiet-nrw-schrott-held-0152`
- `nw-solingen-max-bontgen-sohn`
- `nw-stolberg-motter`
- `nw-viersen-dais`
- `nw-wettringen-skibinski`
- `nw-witten-alwidian`
- `nw-wuppertal-elberfeld-ewald-lietmann`

## Rheinland-Pfalz (20)

- `rp-33481-dmr-rohstoffhandel-kl`
- `rp-altenglan-muhlbach-hess-michael-schrotthandel`
- `rp-bad-kreuznach-55543-loritz-sohn`
- `rp-borrstadt-ayan-borrstadt`
- `rp-buchel-heinen-buchel`
- `rp-dernbach-dernbach`
- `rp-gerolstein-gerolstein`
- `rp-hanhofen-67374-jenny-tavernier-schrotthandel`
- `rp-heidesheim-taurus-entsorgungs`
- `rp-kapellen-drusweiler-76889-kieffer`
- `rp-koblenz-56070-strunck-mario`
- `rp-mehlingen-67678-steppan-leon`
- `rp-neustadt-meyer-lachen-speyerdorf`
- `rp-neustadt-s-ohlinger-steel-men`
- `rp-staudt-56424-bausch-henry`
- `rp-tiefenthal-gotz-altwaren-metalle-demontage-team-bod`
- `rp-trier-pfalzel-sascha-kaiser-schrott-metallhandel`
- `rp-trier-welsch-kochann-hutmacher-frankreiter`
- `rp-wittlich-becker-mathilde`
- `rp-zerf-destani-zerf`

## Ergebnisse

- [HE-Bericht](wave-20261026-he-20-dossiers.md)
- [NW-Bericht](wave-20261026-nw-20-dossiers.md)
- [RP-Bericht](wave-20261026-rp-20-dossiers.md)
- [Gemeinsames Owner-Gate](wave-20261026-owner.md)
