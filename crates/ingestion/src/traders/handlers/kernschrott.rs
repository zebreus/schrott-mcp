//! Kernschrott's electronic purchase list, live verified 08.10.2026.
//! Grades are explicit, prices are read afresh (never embedded). The legacy
//! ?seite=elektro URL redirects to /elektro. All quotes are EUR/kg.
//! Class 3 is quarantined: its hazardous-material description contradicts
//! the validated page-wide exclusion. Explicit material acceptances aggregate
//! all variant conditions, overriding record()'s implicit blank acceptances.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance,
    ScrapedPrice,
};
use crate::IngestError;

pub const SLUG: &str = "by-regensburg-kernschrott-recycling";
pub const URL: &str = "https://www.kernschrott-recycling.de/elektro";
const IMPRESSUM: &str = "https://www.kernschrott-recycling.de/impressum";
const RESERVATION: &str = "Wir kaufen alle hier angegebenen Materialien zum Tageshöchstpreis. Die Preise sind unverbindlich – bei Verkauf bitten wir um kurze Rücksprache.";
const EXCLUSION: &str = "Kein Ankauf von Materialien, die z. B. Kondensatoren ab 25 mm Bauhöhe, PCB oder andere gefährliche Stoffe enthalten. Bei Unsicherheit sprechen Sie uns vorab an.";
const CLASS3: &str = "Leiterplatten Kl. 3";
const CLASS3_DESCRIPTION: &str =
    "Leiterplatten mit Kondensatoren ab 25 mm Bauhöhe, PCB oder gefährliche Stoffe";
const REQUEST_ONLY: &[&str] = &[
    "Kabel ohne Stecker",
    "Kabel mit Stecker",
    "Alu-Cu-Kühler ohne Eisen",
];
// Only these reviewed grades have no description in the unmodified source.
// All other known grades must retain a nonempty description; changed wording
// is preserved, but removing the condition entirely requires re-evaluation.
const DESCRIPTION_OPTIONAL: &[&str] = &[
    "Steckkarten mit Blende",
    "Steckkarten mit Blende und Kühler",
    "Netzteile mit Kabel",
    "Netzteile ohne Kabel",
    "Ablenkeinheiten",
    "Panzersicherungen",
];

// Exact names, not fuzzy category matching: a new or renamed grade is
// reported, never assigned to the nearest existing grade. The original
// name is also its stable variant, so distinct prices cannot collapse.
const GRADES: &[(&str, &str)] = &[
    ("Leiterplatten Kl. 1A", "platinen"),
    ("Leiterplatten Kl. 1B", "platinen"),
    ("Leiterplatten Kl. 2", "platinen"),
    ("Steckkarten", "platinen"),
    ("Steckkarten mit Blende", "platinen"),
    ("Steckkarten mit Blende und Kühler", "platinen"),
    ("Laptopplatinen", "platinen"),
    ("IC's", "integrierte-schaltungen"),
    ("Handyplatinen", "platinen"),
    ("Handys komplett ohne Akku", "handys"),
    ("Smartphones komplett ohne Akku", "handys"),
    ("Prozessor Keramik Intel 286, 386, 486", "prozessoren"),
    ("Prozessor Keramik Goldcap", "prozessoren"),
    ("Prozessor Keramik", "prozessoren"),
    ("Prozessor Kunststoff Schwarz", "prozessoren"),
    ("Prozessor Keramik mit Aludeckel", "prozessoren"),
    ("Prozessor Slot", "prozessoren"),
    ("Prozessor Kunststoff", "prozessoren"),
    ("Prozessor Kupferkern 90 % Pin", "prozessoren"),
    ("Prozessor Kupferkern 50 % Pin", "prozessoren"),
    ("Prozessor Kupferkern ohne Pin", "prozessoren"),
    ("Arbeitsspeicher Goldkante", "ram"),
    ("Arbeitsspeicher Silberkante", "ram"),
    ("Arbeitsspeicher Goldkante mit Aluleiste", "ram"),
    ("Computerstecker", "elektronikstecker"),
    ("Flachbandkabelstecker", "elektronikstecker"),
    ("Kabel ohne Stecker", "kabel-ohne-stecker"),
    ("Kabel mit Stecker", "kabel-mit-stecker"),
    ("Laptops, Notebooks", "notebooks"),
    ("Festplatten", "festplatten"),
    ("Netzteile mit Kabel", "netzteile"),
    ("Computer komplett unberaubt", "computer"),
    ("Laufwerke", "laufwerke"),
    ("Netzteile ohne Kabel", "netzteile"),
    ("CDs", "optische-datentraeger"),
    ("Ablenkeinheiten", "ablenkeinheiten"),
    ("Panzersicherungen", "panzersicherungen"),
    ("Steckernetzteile", "netzteile"),
    ("Elektrobauteile", "elektrobauteile"),
    ("Alu-Cu-Kühler ohne Eisen", "alu-cu-kuehler"),
    ("Lüfter", "luefter"),
];

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |client| Box::pin(scrape(client)),
    }
}

fn error(detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.into(),
    }
}

fn text(node: scraper::ElementRef<'_>) -> String {
    node.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn quoted_amount(quote: &str) -> Option<f64> {
    let number = quote.strip_suffix('€')?.trim();
    let (integer, decimals) = number.split_once(',')?;
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    if decimals.len() != 2 || !digits(decimals) {
        return None;
    }
    let mut groups = integer.split('.');
    if !digits(groups.next()?) || !groups.all(|g| g.len() == 3 && digits(g)) {
        return None;
    }
    parse_eur(number).filter(|amount| amount.is_finite())
}

fn verify_operator(html: &str) -> Result<(), IngestError> {
    let doc = Html::parse_document(html);
    let body = text(doc.root_element());
    if ![
        "Kernschrott-Recycling UG (haftungsbeschränkt)",
        "HRB 12051",
        "93053 Regensburg",
        "Dieselstr. 7",
    ]
    .iter()
    .all(|needle| body.contains(needle))
    {
        return Err(error("Impressum identity changed: expected Kernschrott UG, HRB 12051, Dieselstr. 7, Regensburg"));
    }
    Ok(())
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let (_, impressum) = fetch_text(client, IMPRESSUM).await?;
    verify_operator(&impressum)?;
    let mut out = parse(&html)?;
    out.status_code = status;
    out.website_alive = true;
    Ok(out)
}

fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let table_sel = Selector::parse("table.prices").unwrap();
    let tables: Vec<_> = doc.select(&table_sel).collect();
    if tables.len() != 1 {
        return Err(error("expected one electronic price table"));
    }
    let table = tables[0];
    let section = table
        .ancestors()
        .filter_map(scraper::ElementRef::wrap)
        .find(|node| node.value().name() == "section")
        .ok_or_else(|| error("price section missing; re-evaluate source"))?;
    let source_text = |selector: &str| -> Result<String, IngestError> {
        let selector = Selector::parse(selector).unwrap();
        let values: Vec<_> = section.select(&selector).map(text).collect();
        if values.len() != 1 {
            return Err(error(
                "missing or ambiguous source conditions/count; re-evaluate source",
            ));
        }
        Ok(values[0].clone())
    };
    let reservation = source_text(".sec-head p")?;
    let exclusion = source_text(".note > div")?;
    if reservation != RESERVATION || exclusion != EXCLUSION {
        return Err(error(
            "global reservation/exclusion changed; re-evaluate source before recording",
        ));
    }
    let declared_count = source_text("#pcount")?
        .strip_suffix(" Positionen")
        .and_then(|n| n.parse::<usize>().ok())
        .ok_or_else(|| error("invalid source position count"))?;
    let global_conditions = format!("{reservation} {exclusion}");
    let heading = Selector::parse("thead th").unwrap();
    let headings: Vec<_> = table.select(&heading).map(text).collect();
    if headings != ["Position", "Preis / kg"] {
        return Err(error("missing verified Preis / kg unit"));
    }
    let body = text(doc.root_element());
    // No date is not an error: shared record() handles unknown dates; never
    // turn copyright years or image filenames into publication dates.
    let dates: Vec<_> = body.split("Stand:").skip(1).collect();
    let published_at = match dates.as_slice() {
        [] => None,
        [d] => {
            let parts: Vec<_> = d
                .split_whitespace()
                .next()
                .unwrap_or("")
                .split('.')
                .collect();
            if parts.len() != 3
                || parts[0].len() != 2
                || parts[1].len() != 2
                || parts[2].len() != 4
                || !parts
                    .iter()
                    .all(|part| part.bytes().all(|b| b.is_ascii_digit()))
            {
                return Err(error("invalid Stand date"));
            }
            Some(
                parse_de_date(parts[0], parts[1], parts[2])
                    .ok_or_else(|| error("invalid Stand date"))?,
            )
        }
        _ => return Err(error("ambiguous Stand dates")),
    };
    let mut out = HandlerOutcome {
        fetch_url: URL.to_owned(),
        byte_len: html.len(),
        published_at,
        ..HandlerOutcome::default()
    };
    let rows = Selector::parse("tbody tr").unwrap();
    let name_sel = Selector::parse(".nm").unwrap();
    let desc_sel = Selector::parse(".ds").unwrap();
    let price_sel = Selector::parse("td.pr").unwrap();
    let mut seen = std::collections::HashSet::new();
    let mut material_conditions: std::collections::BTreeMap<&'static str, Vec<String>> =
        std::collections::BTreeMap::new();
    let mut count = 0;
    for row in table.select(&rows) {
        if row.value().classes().any(|c| c == "cat") {
            let cells = Selector::parse("td").unwrap();
            let cells: Vec<_> = row.select(&cells).collect();
            if row.select(&name_sel).next().is_some()
                || row.select(&price_sel).next().is_some()
                || row.select(&desc_sel).next().is_some()
                || cells.len() != 1
                || cells[0].value().attr("colspan") != Some("2")
                || text(cells[0]).is_empty()
            {
                return Err(error("price row masked as category; re-evaluate source"));
            }
            continue;
        }
        count += 1;
        let names: Vec<_> = row.select(&name_sel).map(text).collect();
        let quotes: Vec<_> = row.select(&price_sel).map(text).collect();
        let descriptions: Vec<_> = row.select(&desc_sel).map(text).collect();
        if names.len() != 1 || names[0].is_empty() || quotes.len() != 1 || descriptions.len() > 1 {
            return Err(error(
                "price row requires exactly one name and quote, at most one description",
            ));
        }
        let name = names[0].clone();
        if !seen.insert(name.clone()) {
            return Err(error(format!("duplicate grade: {name}")));
        }
        let desc = descriptions.into_iter().next().unwrap_or_default();
        let label = if desc.is_empty() {
            name.clone()
        } else {
            format!("{name} — {desc}")
        };
        if name == CLASS3 {
            if desc != CLASS3_DESCRIPTION || quoted_amount(&quotes[0]).is_none() {
                return Err(error(
                    "Class-3 source state changed; re-evaluate quarantine",
                ));
            }
            out.skipped_labels.push(format!(
                "{label} (validated hazardous description contradicts validated page-wide exclusion; quarantined, not an accepted variant)"
            ));
            continue;
        }
        let Some(&(variant, material)) = GRADES.iter().find(|(grade, _)| *grade == name) else {
            out.skipped_labels.push(label);
            continue;
        };
        if desc.is_empty() && !DESCRIPTION_OPTIONAL.contains(&variant) {
            return Err(error(format!(
                "mandatory description lost: {name}; re-evaluate source"
            )));
        }
        let quote = &quotes[0];
        if quote == "Preis auf Anfrage" {
            if !REQUEST_ONLY.contains(&variant) {
                return Err(error(format!("numeric grade became request-only: {name}; no partial snapshot or current-price refresh")));
            }
            material_conditions
                .entry(material)
                .or_default()
                .push(format!("{label}; Preis auf Anfrage"));
        } else if let Some(price) = quoted_amount(quote) {
            if REQUEST_ONLY.contains(&variant) {
                return Err(error(format!(
                    "request-only grade became numeric: {name}; re-evaluate source"
                )));
            }
            material_conditions
                .entry(material)
                .or_default()
                .push(label.clone());
            out.prices.push(ScrapedPrice {
                material,
                variant,
                price,
                currency: "EUR",
                unit: "EUR/kg",
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label: format!("{label}; {global_conditions}"),
            });
        } else {
            return Err(error(format!(
                "unsupported quote for known grade {name}: {quote}; no partial snapshot"
            )));
        }
    }
    if count != declared_count || count == 0 {
        return Err(error(
            "position count does not match source; no partial snapshot",
        ));
    }
    for grade in GRADES
        .iter()
        .map(|(name, _)| *name)
        .chain(std::iter::once(CLASS3))
    {
        if !seen.contains(grade) {
            return Err(error(format!(
                "known grade missing: {grade}; no partial snapshot or current-price refresh"
            )));
        }
    }
    // record() writes prices/implicit acceptances first, then these explicit
    // material-wide aggregates. Never one acceptance per variant (last wins).
    out.acceptances = material_conditions.into_iter().map(|(material, variants)| {
        let mut conditions = format!("Nur die aufgeführten Varianten mit ihren jeweiligen Bedingungen: {}; {global_conditions}", variants.join(" | "));
        if material == "platinen" {
            conditions.push_str("; Leiterplatten Kl. 3 quarantänisiert, keine bestätigte Annahme dieser Variante");
        }
        ScrapedAcceptance { material, label: conditions.clone(), conditions }
    }).collect();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIVE: &str = include_str!("../../../tests/fixtures/kernschrott-elektro-20261008.html");

    // Mutate a copy only: the fetched fixture is never changed to suit tests.
    fn without_grade(html: &str, grade: &str) -> String {
        let name = html
            .find(&format!("<div class=\"nm\">{grade}</div>"))
            .unwrap();
        let start = html[..name].rfind("<tr ").unwrap();
        let end = name + html[name..].find("</tr>").unwrap() + "</tr>".len();
        format!("{}{}", &html[..start], &html[end..])
    }

    fn blocked_source_mutations() -> Vec<String> {
        vec![
            without_grade(LIVE, "Festplatten"),
            without_grade(LIVE, "Festplatten").replace("42 Positionen", "41 Positionen"),
            LIVE.replacen("<tr data-c=\"Leiterplatten\" data-s=", "<tr class=\"cat\" data-c=\"Leiterplatten\" data-s=", 1),
            LIVE.replace("<div class=\"ds\">Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf</div>", ""),
            LIVE.replace("<div class=\"ds\">Defekte Notebooks unberaubt, ohne Akku</div>", ""),
            LIVE.replace("<div class=\"ds\">Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf</div>", "<div class=\"renamed-description\">Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf</div>"),
            without_grade(LIVE, CLASS3).replace("42 Positionen", "41 Positionen"),
            LIVE.replace("Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf</div>", "</div>"),
            LIVE.replace("308,00 €", "Preis auf Anfrage"),
            LIVE.replace("308,00 €", "ab 308,00 €"),
            LIVE.replace("Steckkarten mit Blende und Kühler</div>", "Neue Steckkarte</div>"),
            LIVE.replace("42 Positionen", "41 Positionen"),
            LIVE.replace(RESERVATION, "Die Preise sind verbindlich."),
            LIVE.replace(EXCLUSION, ""),
            LIVE.replace(EXCLUSION, "Wir kaufen auch Gefahrstoffe."),
            LIVE.replace(CLASS3_DESCRIPTION, "Ungefährliche Leiterplatten"),
        ]
    }

    #[test]
    fn source_drift_blocks_instead_of_producing_partial_or_unconditional_snapshots() {
        for (index, html) in blocked_source_mutations().iter().enumerate() {
            assert!(parse(html).is_err(), "mutation {index} must fail closed");
        }
    }

    fn persisted_snapshot(public: &schrott_mcp_store::PublicDb, out: &HandlerOutcome) -> String {
        let tid = public.find_trader_id(SLUG).unwrap().unwrap();
        let mut snapshot = String::new();
        for p in &out.prices {
            let mid = public.find_material_id(p.material).unwrap().unwrap();
            snapshot.push_str(&format!(
                "{:?}{:?}",
                public.current_price_for(tid, mid, p.variant).unwrap(),
                public.price_history(tid, mid, p.variant, 100).unwrap()
            ));
        }
        for a in &out.acceptances {
            let mid = public.find_material_id(a.material).unwrap().unwrap();
            snapshot.push_str(&format!(
                "{:?}",
                public.existing_acceptance(tid, mid).unwrap()
            ));
        }
        snapshot
    }

    #[tokio::test]
    async fn failed_parse_after_record_leaves_all_current_history_and_acceptances_unconfirmed() {
        let dir = crate::test_support::TempDbDir::new("kernschrott-failed-snapshot");
        let public = schrott_mcp_store::PublicDb::open(dir.path()).unwrap();
        let internal = schrott_mcp_store::InternalDb::open(dir.path()).unwrap();
        crate::pipeline::seed_metadata(&public).unwrap();
        let now = chrono::Utc::now();
        let out = parse(LIVE).unwrap();
        super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        let before = persisted_snapshot(&public, &out);
        for (index, html) in blocked_source_mutations().iter().enumerate() {
            // Mirrors scrape's parse? boundary: a failed parse cannot reach
            // record(), including refresh of unchanged current observations.
            let attempted = parse(html);
            assert!(attempted.is_err(), "mutation {index}");
            if let Ok(candidate) = attempted {
                super::super::super::record(
                    &public,
                    &internal,
                    SLUG,
                    &candidate,
                    &(now + chrono::Duration::seconds(1)),
                )
                .await
                .unwrap();
            }
            assert_eq!(
                persisted_snapshot(&public, &out),
                before,
                "mutation {index}: no current/history/acceptance changes"
            );
        }
    }

    #[tokio::test]
    async fn unchanged_amount_changed_conditions_refreshes_existing_row_not_immutable_history() {
        let dir = crate::test_support::TempDbDir::new("kernschrott-dedupe");
        let public = schrott_mcp_store::PublicDb::open(dir.path()).unwrap();
        let internal = schrott_mcp_store::InternalDb::open(dir.path()).unwrap();
        crate::pipeline::seed_metadata(&public).unwrap();
        let now = chrono::Utc::now();
        let out = parse(LIVE).unwrap();
        super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        let tid = public.find_trader_id(SLUG).unwrap().unwrap();
        let mid = public.find_material_id("festplatten").unwrap().unwrap();
        let before = public
            .current_price_for(tid, mid, "Festplatten")
            .unwrap()
            .unwrap();
        let new_condition =
            "Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf; ohne Gehäuseanhaftungen";
        let changed = parse(
            &LIVE
                .replace(
                    "Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf",
                    new_condition,
                )
                .replace("06.10.2026", "10.10.2026"),
        )
        .unwrap();
        super::super::super::record(
            &public,
            &internal,
            SLUG,
            &changed,
            &(now + chrono::Duration::seconds(10)),
        )
        .await
        .unwrap();
        let after = public
            .current_price_for(tid, mid, "Festplatten")
            .unwrap()
            .unwrap();
        let history = public.price_history(tid, mid, "Festplatten", 10).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(after.id, before.id);
        assert_eq!(after.price, before.price);
        assert_ne!(after.observed_at, before.observed_at);
        assert_ne!(after.ingested_at, before.ingested_at);
        assert_eq!(
            after.published_at.as_deref(),
            Some("2026-10-10T00:00:00+00:00")
        );
        assert!(after.notes.contains(new_condition));
        assert!(after.notes.contains(EXCLUSION));
        assert_eq!(history[0].notes, after.notes);
        let acceptance = public.existing_acceptance(tid, mid).unwrap().unwrap();
        assert!(acceptance.0 && acceptance.1.contains(new_condition));
        assert!(acceptance.1.contains(EXCLUSION));
    }

    #[test]
    fn complete_live_snapshot_preserves_grades_conditions_and_zero() {
        let out = parse(LIVE).unwrap();
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-10-06T00:00:00+00:00")
        );
        assert_eq!(
            (
                out.prices.len(),
                out.acceptances.len(),
                out.skipped_labels.len()
            ),
            (38, 19, 1)
        );
        assert!(out.skipped_labels[0].contains("quarantined"));
        assert_eq!(
            out.acceptances
                .iter()
                .filter(|a| a.conditions.contains("Preis auf Anfrage"))
                .count(),
            3
        );
        assert!(out
            .acceptances
            .iter()
            .all(|a| a.conditions.contains(EXCLUSION) && a.conditions.contains(RESERVATION)));
        let unique: std::collections::HashSet<_> =
            out.prices.iter().map(|p| (p.material, p.variant)).collect();
        assert_eq!(unique.len(), out.prices.len());
        let cpu = out
            .prices
            .iter()
            .find(|p| p.variant == "Prozessor Keramik Intel 286, 386, 486")
            .unwrap();
        assert_eq!(
            (cpu.material, cpu.price, cpu.unit),
            ("prozessoren", 308.0, "EUR/kg")
        );
        assert!(cpu.label.contains("Ohne Kühlkörper"));
        assert_eq!(
            out.prices
                .iter()
                .filter(|p| p.material == "prozessoren")
                .count(),
            10
        );
        assert_eq!(
            out.prices
                .iter()
                .filter(|p| p.material == "netzteile")
                .count(),
            3
        );
        assert_eq!(
            out.prices
                .iter()
                .find(|p| p.material == "luefter")
                .unwrap()
                .price,
            0.0
        );
        assert!(out
            .prices
            .iter()
            .find(|p| p.material == "festplatten")
            .unwrap()
            .label
            .contains("Platine muss vorhanden sein"));
        assert!(out
            .acceptances
            .iter()
            .any(|a| a.material == "alu-cu-kuehler"));
        assert!(out
            .acceptances
            .iter()
            .any(|a| a.material == "kabel-ohne-stecker"));
        assert!(!out.acceptances.iter().any(|a| a.material == "kabel-kupfer"));
        assert_eq!(
            out.prices
                .iter()
                .map(|p| (p.variant, p.material, p.price))
                .collect::<Vec<_>>(),
            vec![
                ("Leiterplatten Kl. 1A", "platinen", 15.0),
                ("Leiterplatten Kl. 1B", "platinen", 4.0),
                ("Leiterplatten Kl. 2", "platinen", 8.0),
                ("Steckkarten", "platinen", 15.0),
                ("Steckkarten mit Blende", "platinen", 10.0),
                ("Steckkarten mit Blende und Kühler", "platinen", 6.5),
                ("Laptopplatinen", "platinen", 15.0),
                ("IC's", "integrierte-schaltungen", 18.0),
                ("Handyplatinen", "platinen", 28.0),
                ("Handys komplett ohne Akku", "handys", 18.0),
                ("Smartphones komplett ohne Akku", "handys", 5.0),
                (
                    "Prozessor Keramik Intel 286, 386, 486",
                    "prozessoren",
                    308.0
                ),
                ("Prozessor Keramik Goldcap", "prozessoren", 198.0),
                ("Prozessor Keramik", "prozessoren", 160.0),
                ("Prozessor Kunststoff Schwarz", "prozessoren", 110.0),
                ("Prozessor Keramik mit Aludeckel", "prozessoren", 90.0),
                ("Prozessor Slot", "prozessoren", 78.0),
                ("Prozessor Kunststoff", "prozessoren", 60.0),
                ("Prozessor Kupferkern 90 % Pin", "prozessoren", 10.5),
                ("Prozessor Kupferkern 50 % Pin", "prozessoren", 9.5),
                ("Prozessor Kupferkern ohne Pin", "prozessoren", 6.0),
                ("Arbeitsspeicher Goldkante", "ram", 68.0),
                ("Arbeitsspeicher Silberkante", "ram", 28.0),
                ("Arbeitsspeicher Goldkante mit Aluleiste", "ram", 28.0),
                ("Computerstecker", "elektronikstecker", 1.5),
                ("Flachbandkabelstecker", "elektronikstecker", 1.5),
                ("Laptops, Notebooks", "notebooks", 2.8),
                ("Festplatten", "festplatten", 2.2),
                ("Netzteile mit Kabel", "netzteile", 1.1),
                ("Computer komplett unberaubt", "computer", 0.9),
                ("Laufwerke", "laufwerke", 0.85),
                ("Netzteile ohne Kabel", "netzteile", 0.9),
                ("CDs", "optische-datentraeger", 0.1),
                ("Ablenkeinheiten", "ablenkeinheiten", 2.8),
                ("Panzersicherungen", "panzersicherungen", 1.0),
                ("Steckernetzteile", "netzteile", 0.25),
                ("Elektrobauteile", "elektrobauteile", 0.2),
                ("Lüfter", "luefter", 0.0),
            ]
        );
        assert_eq!(
            out.acceptances
                .iter()
                .filter(|a| a.conditions.contains("Preis auf Anfrage"))
                .map(|a| a.material)
                .collect::<Vec<_>>(),
            vec!["alu-cu-kuehler", "kabel-mit-stecker", "kabel-ohne-stecker"]
        );
        assert!(out.prices.iter().all(|p| p.currency == "EUR"
            && p.unit == "EUR/kg"
            && p.price_kind == "exact"
            && p.label.contains("unverbindlich")));
    }

    #[test]
    fn prices_change_and_new_unknown_grades_are_not_guessed() {
        let changed = LIVE.replace("308,00 €", "321,50 €")
            .replace("42 Positionen", "43 Positionen")
            .replace("</tbody>", "<tr><td><div class=\"nm\">Neue Steckkarte</div></td><td class=\"pr\">1,00 €</td></tr></tbody>");
        let out = parse(&changed).unwrap();
        assert_eq!(
            out.prices
                .iter()
                .find(|p| p.variant == "Prozessor Keramik Intel 286, 386, 486")
                .unwrap()
                .price,
            321.5
        );
        assert!(out
            .skipped_labels
            .iter()
            .any(|s| s.contains("Neue Steckkarte")));
        assert!(out
            .prices
            .iter()
            .any(|p| p.variant == "Steckkarten mit Blende und Kühler"));
        assert_eq!(
            parse(&LIVE.replace("06.10.2026", "09.10.2026"))
                .unwrap()
                .published_at
                .as_deref(),
            Some("2026-10-09T00:00:00+00:00")
        );
        assert!(parse(&LIVE.replace("308,00 €", "Preis auf Anfrage")).is_err());
    }

    #[test]
    fn date_unit_duplicate_and_amount_guards() {
        assert!(parse(&LIVE.replace("Preis / kg", "Preis / Stück")).is_err());
        assert!(parse(&LIVE.replace("Preis / kg", "Preis / kg oder Stück")).is_err());
        assert!(parse(&LIVE.replace("06.10.2026", "31.02.2026")).is_err());
        assert!(parse(&LIVE.replace("06.10.2026", "06.10.+026")).is_err());
        assert!(parse(&LIVE.replace("Stand:", "Stand: 06.10.2026 Stand:")).is_err());
        assert_eq!(
            parse(&LIVE.replace("Stand:", "Aktualisierung:"))
                .unwrap()
                .published_at,
            None
        );
        assert!(
            parse(&LIVE.replace("Leiterplatten Kl. 1B</div>", "Leiterplatten Kl. 1A</div>"))
                .is_err()
        );
        assert!(parse(&LIVE.replace("308,00 €", "ab 308,00 €")).is_err());
        assert!(parse("<html>no prices</html>").is_err());
        assert!(
            parse(&LIVE.replace("<table class=\"prices\">", "<table class=\"other\">")).is_err()
        );
        assert!(parse(&LIVE.replace(
            "<div class=\"nm\">Leiterplatten Kl. 1A</div>",
            "<div class=\"nm\"></div>"
        ))
        .is_err());
        assert!(parse(&LIVE.replace("<div class=\"nm\">Leiterplatten Kl. 1A</div>", "<div class=\"nm\">Leiterplatten Kl. 1A</div><div class=\"nm\">Leiterplatten Kl. 1B</div>")).is_err());
        assert!(parse(&LIVE.replacen("<td class=\"pr\">", "<td>", 1)).is_err());
        assert!(parse(&LIVE.replacen(
            "</tbody>",
            "<tr><td><div class=\"nm\">Neue Sorte</div></td></tr></tbody>",
            1
        ))
        .is_err());
    }

    #[test]
    fn identity_must_match() {
        let imp = "<p>Kernschrott-Recycling UG (haftungsbeschränkt) HRB 12051 Dieselstr. 7, 93053 Regensburg</p>";
        verify_operator(imp).unwrap();
        assert!(verify_operator(&imp.replace("12051", "99999")).is_err());
        assert!(verify_operator(&imp.replace("Regensburg", "Berlin")).is_err());
    }

    #[test]
    fn quote_is_exact_eur_amount_not_model_number_or_range() {
        assert_eq!(quoted_amount("1.234,50 €"), Some(1234.5));
        assert_eq!(quoted_amount("0,00 €"), Some(0.0));
        for quote in [
            "308,00",
            "308,00 USD",
            "ab 308,00 €",
            "-1,00 €",
            "1,00–2,00 €",
            "286, 386, 486",
            "Preis auf Anfrage",
        ] {
            assert_eq!(quoted_amount(quote), None, "{quote}");
        }
        assert_eq!(quoted_amount(&format!("{},00 €", "9".repeat(400))), None);
    }

    #[tokio::test]
    #[ignore = "read-only live network verification; no database access"]
    async fn live_handler_verifies_identity_prices_and_date() {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        let out = (handler().scrape)(&client)
            .await
            .expect("public live handler");
        assert_eq!(out.status_code, 200);
        assert_eq!(out.fetch_url, URL);
        assert!(out.website_alive);
        assert!(out.byte_len > 0);
        assert!(out.published_at.is_some());
        assert_eq!(
            (
                out.prices.len(),
                out.acceptances.len(),
                out.skipped_labels.len()
            ),
            (38, 19, 1)
        );
        assert!(out.skipped_labels[0].contains("quarantined"));
        assert!(out.prices.iter().all(|p| p.currency == "EUR"
            && p.unit == "EUR/kg"
            && p.price.is_finite()
            && p.price >= 0.0));
        assert_eq!(
            out.prices
                .iter()
                .find(|p| p.material == "luefter")
                .unwrap()
                .price,
            0.0
        );
        assert_eq!(
            out.acceptances
                .iter()
                .filter(|a| a.conditions.contains("Preis auf Anfrage"))
                .count(),
            3
        );
        assert!(out
            .acceptances
            .iter()
            .all(|a| a.conditions.contains(EXCLUSION) && a.conditions.contains(RESERVATION)));
        assert!(out
            .prices
            .iter()
            .all(|p| p.label.contains(EXCLUSION) && p.label.contains(RESERVATION)));
        eprintln!("Kernschrott live: {out:?}");
    }

    #[tokio::test]
    async fn seeded_catalog_records_all_conditions_and_amount_change_history() {
        let dir = crate::test_support::TempDbDir::new("kernschrott");
        let public = schrott_mcp_store::PublicDb::open(dir.path()).unwrap();
        let internal = schrott_mcp_store::InternalDb::open(dir.path()).unwrap();
        crate::pipeline::seed_metadata(&public).unwrap();
        let now = chrono::Utc::now();
        let out = parse(LIVE).unwrap();
        let result = super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        assert_eq!(
            (result.recorded, result.accepted, result.skipped.len()),
            (38, 19, 1)
        );
        assert!(result.canaries.is_empty());
        let tid = public.find_trader_id(SLUG).unwrap().unwrap();
        for price in &out.prices {
            let mid = public.find_material_id(price.material).unwrap().unwrap();
            let history = public.price_history(tid, mid, price.variant, 10).unwrap();
            assert_eq!(history.len(), 1, "{}", price.variant);
            let stored = &history[0];
            assert_eq!(stored.price, price.price);
            assert_eq!(stored.currency, "EUR");
            assert_eq!(stored.unit, "EUR/kg");
            assert_eq!(stored.source_url, URL);
            assert_eq!(stored.published_at, out.published_at);
            assert_eq!(stored.notes, price.label);
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&stored.extra_json).unwrap()
                    ["published_at_basis"],
                "page_stated"
            );
        }
        for acceptance in &out.acceptances {
            let mid = public
                .find_material_id(acceptance.material)
                .unwrap()
                .unwrap();
            assert_eq!(
                public.existing_acceptance(tid, mid).unwrap(),
                Some((true, acceptance.conditions.clone()))
            );
            assert!(public.price_history(tid, mid, "", 10).unwrap().is_empty());
        }
        // Verify actual rows after record(), not just parsed labels. Every
        // material includes its global limits and *all* mapped variants.
        for &(variant, material) in GRADES {
            let mid = public.find_material_id(material).unwrap().unwrap();
            let (accepts, conditions) = public.existing_acceptance(tid, mid).unwrap().unwrap();
            assert!(accepts);
            assert!(conditions.contains(variant), "{variant}: {conditions}");
            assert!(conditions.contains(EXCLUSION));
            assert!(conditions.contains(RESERVATION));
        }
        for (material, required) in [
            (
                "festplatten",
                "Platine muss vorhanden sein, ansonsten erfolgt kein Ankauf",
            ),
            ("notebooks", "Defekte Notebooks unberaubt, ohne Akku"),
            ("handys", "Akku muss entfernt sein"),
            ("elektronikstecker", "ohne Ferritring"),
            ("netzteile", "Nur mit Kupferkern"),
            (
                "platinen",
                "Leiterplatten Kl. 3 quarantänisiert, keine bestätigte Annahme",
            ),
        ] {
            let mid = public.find_material_id(material).unwrap().unwrap();
            assert!(
                public
                    .existing_acceptance(tid, mid)
                    .unwrap()
                    .unwrap()
                    .1
                    .contains(required),
                "{material}"
            );
        }
        let changed = parse(&LIVE.replace("308,00 €", "321,50 €")).unwrap();
        super::super::super::record(
            &public,
            &internal,
            SLUG,
            &changed,
            &(now + chrono::Duration::seconds(1)),
        )
        .await
        .unwrap();
        let mid = public.find_material_id("prozessoren").unwrap().unwrap();
        let history = public
            .price_history(tid, mid, "Prozessor Keramik Intel 286, 386, 486", 10)
            .unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!((history[0].price, history[1].price), (321.5, 308.0));
        // A later price record temporarily writes blank implicit acceptances;
        // the explicit aggregates must restore all conditions on every run.
        for acceptance in &out.acceptances {
            let mid = public
                .find_material_id(acceptance.material)
                .unwrap()
                .unwrap();
            assert_eq!(
                public.existing_acceptance(tid, mid).unwrap(),
                Some((true, acceptance.conditions.clone()))
            );
        }
    }
}
