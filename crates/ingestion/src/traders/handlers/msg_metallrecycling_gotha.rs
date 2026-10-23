//! MSG Metallrecycling Gotha: Webflow-CMS-Preisliste (`/preisliste`) als
//! Produktkarten (`a.product4_text-link`) in `section#preiseSchrott`.
//! Alle sichtbaren positiven Mengenstaffeln bleiben erhalten. Händlersorte
//! plus dynamische kanonische Mengenschwelle bildet die Variante. Der sichtbare
//! Seitenhinweis belegt Preise pro Kilogramm, unabhängig vom Mengenmaß kg/t.
//! Versteckte Webflow-Alternativen/Dummywerte sind keine Angebote. Nullwerte
//! sind Nicht-Angebote, unterdrücken aber keine späteren positiven Staffeln.
//! Bekannte Kataloglücken werden diagnostiziert; neue unbekannte bepreiste
//! Sorten und beschädigte Preiszellen verhindern einen Snapshot.

use scraper::{ElementRef, Html, Selector};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "th-gotha-msg-metallrecycling-msg-schrott-und-meta";
/// Bespoke, live-verified price URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const URL: &str = "https://www.msg-metallrecycling.de/preisliste";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.msg-metallrecycling.de/legal/impressum";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (status, html) = fetch_text(client, URL).await?;
    let mut out = prices_from_html(&html)?;
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    out.trader_info = extract_info(&imp_html)?;
    out.website_alive = true;
    out.status_code = status;
    Ok(out)
}

pub(crate) const SNAPSHOT_OWNER: &str = "msg-visible-tiers-v1";

/// Shared only within this handler: live scrape and parser -> record tests.
pub(crate) fn prices_from_html(html: &str) -> Result<HandlerOutcome, IngestError> {
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    let mut seen = std::collections::HashMap::new();
    for row in rows {
        match grade_for(&row.title) {
            Some((material, _)) => {
                let variant = format!("{} / {}", row.title, canonical_tier(&row.tier)?);
                if let Some(previous) =
                    seen.insert((material, variant.clone()), row.price.to_bits())
                {
                    if previous != row.price.to_bits() {
                        return Err(parse_error("widersprüchliche doppelte Preisidentität"));
                    }
                } else {
                    prices.push(ScrapedPrice {
                        material,
                        variant: variant.into(),
                        price: row.price,
                        currency: "EUR",
                        unit: row.unit,
                        price_kind: "exact",
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        // Rohlabel inkl. Staffel → landet als notes.
                        label: format!("{} ({})", row.title, row.tier),
                    });
                }
            }
            None if matches!(
                row.title.as_str(),
                "Alu Cu Kühler"
                    | "Stahl- und Gussspäne"
                    | "Messer versilbert"
                    | "Silberbesteck 800"
                    | "Versilbertes Besteck gestempelt 60/40/20"
                    | "Versilbertes Besteck gestempelt 80/90/100"
            ) =>
            {
                skipped_labels.push(format!(
                    "Kataloglücke: {} ({}; {} EUR/kg)",
                    row.title, row.tier, row.price
                ))
            }
            None => {
                return Err(parse_error(&format!(
                    "unbekannte bepreiste Sorte (Mapping prüfen): {}",
                    row.title
                )))
            }
        }
    }
    if prices.is_empty() {
        return Err(parse_error("keine katalogisierten Preisangebote"));
    }
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
        prices,
        acceptances: vec![],
        trader_info: TraderInfo::default(),
        website_alive: false,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: 200,
        byte_len: html.len(),
        published_at: None,
    })
}

/// One visible positive source price, before explicit catalog mapping.
#[derive(Debug)]
struct Row {
    title: String,
    tier: String,
    price: f64,
    unit: &'static str,
}

/// Publish only a fully resolved MSG snapshot. All fallible enrichment occurs
/// before the final atomic price/retirement transaction. No best-effort price
/// writes or guessed legacy keys in this path.
pub(crate) fn record_snapshot(
    public: &schrott_mcp_store::PublicDb,
    internal: &schrott_mcp_store::InternalDb,
    outcome: &HandlerOutcome,
    now: &chrono::DateTime<chrono::Utc>,
) -> Result<super::super::RecordSummary, IngestError> {
    use super::super::{
        comparable_price_changed, german_calendar_date, normalize_unit, same_mapping_version,
        MAP_VERSION, MATERIAL_FALLBACKS,
    };
    let db_error = |source| IngestError::Catalog {
        what: "MSG snapshot",
        name: SLUG.to_owned(),
        source,
    };
    if outcome.fetch_url != URL
        || !(200..300).contains(&outcome.status_code)
        || outcome.prices.is_empty()
        || !outcome.acceptances.is_empty()
    {
        return Err(parse_error("ungültiger MSG Snapshot"));
    }
    let tid = public
        .find_trader_id(SLUG)
        .map_err(&db_error)?
        .ok_or_else(|| parse_error("MSG Händler fehlt"))?;
    let now_s = now.to_rfc3339();
    let before = public.current_prices_from(tid, URL).map_err(&db_error)?;
    let mut legacy = Vec::new();
    for p in &before {
        if !p.published || p.source_type != "haendler_angabe" {
            continue;
        }
        let Ok(extra) = serde_json::from_str::<serde_json::Value>(&p.extra_json) else {
            continue;
        };
        let compatible_owner = match extra.get("snapshot_owner") {
            None | Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::String(owner)) => owner.is_empty() || owner == SNAPSHOT_OWNER,
            Some(_) => false,
        };
        if !compatible_owner {
            continue;
        }
        if let Some((title, tier)) = p.notes.strip_suffix(')').and_then(|s| s.rsplit_once(" (")) {
            if canonical_tier(tier).is_err() {
                continue;
            }
            if let Some((material, variant)) = grade_for(title) {
                let mid = public.find_material_id(material).map_err(&db_error)?;
                let wrong_water = title == "Wasseruhren"
                    && p.variant == "Wasseruhren"
                    && Some(p.material_id)
                        == public.find_material_id("messing").map_err(&db_error)?;
                if (Some(p.material_id) == mid && p.variant == variant) || wrong_water {
                    legacy.push(p.clone());
                }
            }
        }
    }
    let mut prepared = Vec::new();
    let mut keys = std::collections::HashSet::new();
    let mut canaries = Vec::new();
    for p in &outcome.prices {
        let (title, tier) = p
            .label
            .strip_suffix(')')
            .and_then(|s| s.rsplit_once(" ("))
            .ok_or_else(|| parse_error("Quelllabel ohne Mengenschwelle"))?;
        let tier = canonical_tier(tier)?;
        if grade_for(title).map(|(m, _)| m) != Some(p.material)
            || p.variant != format!("{title} / {tier}")
            || p.unit != "EUR/kg"
            || p.currency != "EUR"
            || p.price_kind != "exact"
            || !p.price.is_finite()
            || p.price <= 0.0
        {
            return Err(parse_error(
                "MSG Mapping-/Identitäts-/Preisvertrag verletzt",
            ));
        }
        let mid = public
            .find_material_id(p.material)
            .map_err(&db_error)?
            .ok_or_else(|| parse_error(&format!("Katalogmaterial fehlt: {}", p.material)))?;
        if !keys.insert((mid, p.variant.as_ref())) {
            return Err(parse_error("doppelte MSG Snapshot-Identität"));
        }
        let (price, min, max, unit) =
            normalize_unit(public, mid, p.price, p.price_min, p.price_max, p.unit)?;
        let previous = before
            .iter()
            .find(|old| old.material_id == mid && old.variant == p.variant);
        let inferred = outcome.published_at.is_none()
            && previous.is_some_and(|old| {
                same_mapping_version(old)
                    && comparable_price_changed(
                        old,
                        price,
                        p.currency,
                        &unit,
                        p.price_kind,
                        min,
                        max,
                    )
            });
        let published = outcome
            .published_at
            .clone()
            .or_else(|| inferred.then(|| german_calendar_date(now)));
        let parts: Vec<_> = tier.split_whitespace().collect();
        let mut extra = serde_json::json!({"map_v": MAP_VERSION, "snapshot_owner": SNAPSHOT_OWNER,
            "source_title": title, "quantity_min": parts[1].parse::<f64>().unwrap(), "quantity_min_text": parts[1], "quantity_unit": parts[2], "source_price_unit": "EUR/kg"});
        if published.is_some() {
            extra["published_at_basis"] = serde_json::json!(if inferred {
                "observed_price_change"
            } else {
                "page_stated"
            });
        }
        if let Some(old) = previous {
            if old.unit == unit && old.currency == p.currency && old.price > 0.0 {
                let ratio = price / old.price;
                if ratio >= super::super::CANARY_JUMP_RATIO
                    || ratio <= 1.0 / super::super::CANARY_JUMP_RATIO
                {
                    canaries.push(format!(
                        "Preissprung {} ({}): {} -> {} {}",
                        p.material, p.variant, old.price, price, unit
                    ));
                }
            }
        }
        prepared.push((mid, price, min, max, unit, published, extra.to_string()));
    }
    // A suspicious truncated/duplicated page is not a safe retirement snapshot.
    if let Some(base) = internal.last_ok_step_items(SLUG).map_err(&db_error)? {
        let n = prepared.len() as f64;
        if base >= super::super::CANARY_MIN_BASELINE
            && (n < base as f64 * super::super::CANARY_DROP_RATIO
                || n > base as f64 * super::super::CANARY_GROWTH_FACTOR)
        {
            return Err(parse_error(
                "MSG Snapshot-Volumen verdächtig; kein Current-Retirement",
            ));
        }
    }
    // Preflight every fallible ancillary write before any Current retirement.
    for (p, (mid, ..)) in outcome.prices.iter().zip(&prepared) {
        public
            .set_acceptance(tid, *mid, true, "", None, None, &now_s, &now_s)
            .map_err(&db_error)?;
        for (_, targets) in MATERIAL_FALLBACKS.iter().filter(|(m, _)| *m == p.material) {
            for target in *targets {
                let fallback = public
                    .find_material_id(target)
                    .map_err(&db_error)?
                    .ok_or_else(|| {
                        parse_error(&format!("Fallback-Katalogmaterial fehlt: {target}"))
                    })?;
                public
                    .set_acceptance(
                        tid,
                        fallback,
                        true,
                        &format!("Annahme via {}-Ankauf", p.material),
                        None,
                        None,
                        &now_s,
                        &now_s,
                    )
                    .map_err(&db_error)?;
            }
        }
    }
    if !outcome.trader_info.is_empty() {
        let info = &outcome.trader_info;
        public
            .set_trader_info(
                SLUG,
                &info.street,
                &info.postcode,
                &info.city,
                &info.phone,
                &info.email,
                outcome.website_alive,
                &now_s,
            )
            .map_err(&db_error)?;
    }
    let prices: Vec<_> = outcome
        .prices
        .iter()
        .zip(&prepared)
        .map(
            |(p, (mid, price, min, max, unit, published, extra))| schrott_mcp_store::NewPrice {
                trader_id: tid,
                material_id: *mid,
                variant: p.variant.as_ref(),
                price: *price,
                currency: p.currency,
                unit,
                price_kind: p.price_kind,
                price_min: *min,
                price_max: *max,
                confidence: p.confidence,
                source_type: "haendler_angabe",
                published: true,
                source_url: URL,
                observed_at: &now_s,
                published_at: published.as_deref(),
                valid_from: None,
                valid_to: None,
                notes: &p.label,
                extra_json: extra,
                ingested_at: &now_s,
            },
        )
        .collect();
    let retired = public
        .record_price_snapshot(&prices, &legacy, SNAPSHOT_OWNER)
        .map_err(&db_error)?;
    tracing::info!(slug = SLUG, retired, "MSG source snapshot reconciled");
    Ok(super::super::RecordSummary {
        recorded: prices.len() as i64,
        skipped: outcome.skipped_labels.clone(),
        canaries,
        accepted: 0,
    })
}

/// Verified titles only; case/whitespace and parsed quantity annotations do
/// not change meaning. Unverified suffixes/composed materials are NOT aliases.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let title = label
        .strip_suffix(')')
        .and_then(|s| s.rsplit_once(" ("))
        .filter(|(_, tier)| canonical_tier(tier).is_ok())
        .map_or(label, |(title, _)| title);
    let normalized = title
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    Some(match normalized.as_str() {
        "millberry" => ("kupfer-millberry", "Millberry"),
        "kupfer lackdraht" => ("kupfer-berry", "Lackdraht"),
        "kupfer leitschienen blank" => ("kupfer-gemischt", "Leitschienen Blank"),
        "kupfer leicht" => ("kupfer-gemischt", "Leicht"),
        "kupfer schwer" => ("kupfer-gemischt", "Schwer"),
        "kupfer späne" => ("kupfer-gemischt", "Späne"),
        "alu kabel" => ("kabel-alu", ""),
        "kabel 50% kupfer" => ("kabel-kupfer", "50%"),
        "kabel 60% kupfer" => ("kabel-kupfer", "60%"),
        "kabel 80% kupfer" => ("kabel-kupfer", "80%"),
        "kabel" => ("kabel-kupfer", ""),
        "schwermessing" => ("messing", "Schwermessing"),
        "ms 58 abfälle" => ("messing", "Ms 58 Abfälle"),
        "ms erodierdraht" => ("messing", "Ms Erodierdraht"),
        "ms leicht" => ("messing", "Ms leicht"),
        "ms späne gemischt" => ("messing", "Ms Späne gemischt"),
        "wasseruhren" => ("messing-leicht", "Wasseruhren"),
        "rotguss späne" => ("bronze-rotguss", "Rotguss Späne"),
        "rotguss stücke" => ("bronze-rotguss", "Rotguss Stücke"),
        "bronze schrott" => ("bronze-rotguss", "Bronze Schrott"),
        "bronze späne" => ("bronze-rotguss", "Bronze Späne"),
        "alu bleche blank" => ("aluminium-blech", "Bleche blank"),
        "alu bleche farbe" => ("aluminium-blech", "Bleche Farbe"),
        "alu geschirr" => ("aluminium-blech", "Geschirr"),
        "alu felgen" => ("aluminium-guss", "Felgen"),
        "alu guss" => ("aluminium-guss", "Guss"),
        "alu profile blank" => ("aluminium-profile", "Profile blank"),
        "alu profile iso" => ("aluminium-profile", "Profile iso"),
        "alu späne" => ("aluminium-gemischt", "Späne"),
        "altblei" => ("blei", ""),
        "v4a edelstahl späne" => ("edelstahl-v4a", "Späne"),
        "v4a edelstahl" => ("edelstahl-v4a", ""),
        "v2a edelstahl späne" => ("edelstahl-v2a", "Späne"),
        "v2a edelstahl" => ("edelstahl-v2a", ""),
        "elektromotoren" => ("elektromotoren", ""),
        "hartmetall bohrer/fräser/plättchen" => ("hartmetall", "Bohrer/Fräser/Plättchen"),
        "hartmetall stückschrott" => ("hartmetall", "Stückschrott"),
        "hartmetall schlamm" => ("hartmetall", "Schlamm"),
        "bremsscheiben" => ("eisenschrott-gussbruch", "Bremsscheiben"),
        "gusseisen" => ("eisenschrott-gussbruch", "Gusseisen"),
        "mischschrott schwer" => ("mischschrott", "schwer"),
        "mischschrott" => ("mischschrott", ""),
        "schredderschrott" => ("stahlschrott-shredder", ""),
        "zinn geschirr" => ("zinn-geschirr", "Geschirr"),
        "zinkblech neu und alt" => ("zink", "Blech neu und alt"),
        "zinkguss" => ("zink", "Guss"),
        _ => return None,
    })
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` after the
/// `<h1>Impressum</h1>` carries "Angaben gemäß § 5 TMG" plus firm lines,
/// street, "PLZ Ort, DE", "Telefon:" and "Mail:" rows separated by `<br>`.
/// A zero-width joiner (U+200D) clings to some breaks and is stripped.
/// Missing anchors mean the page changed shape → loud error, never a
/// guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let anchor = doc
        .select(&h1)
        .find(|h| h.text().collect::<String>().trim() == "Impressum");
    if anchor.is_none() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Heading fehlt".to_owned(),
        });
    }
    let body = doc.select(&p).find(|e| {
        let t: String = e.text().collect();
        t.contains("Angaben gem") && t.contains("5 TMG")
    });
    let Some(body) = body else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Angaben-Block fehlt".to_owned(),
        });
    };
    // <br-split discipline: discard everything up to the first '>' so no
    // tag remnants ("class=…") parse as text.
    let mut lines = Vec::new();
    for part in body.inner_html().split("<br") {
        let t = strip_tags(part);
        let t: String = t
            .chars()
            .filter(|c| !matches!(c, '\u{200b}' | '\u{200d}' | '\u{feff}'))
            .collect();
        let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
        if !t.is_empty() {
            lines.push(t);
        }
    }
    let mut street = String::new();
    let mut postcode = String::new();
    let mut city = String::new();
    let mut phone = String::new();
    let mut email = String::new();
    for (i, line) in lines.iter().enumerate() {
        // "99867 Gotha, DE" — PLZ zuerst, Stadt dahinter.
        if postcode.is_empty() {
            let mut it = line.split_whitespace();
            if let (Some(pc), Some(_)) = (it.next(), it.next()) {
                if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                    postcode = pc.to_owned();
                    // "99867 Gotha, DE" → Stadt ohne Länderkürzel.
                    let after = line[pc.len()..].trim();
                    let after = after
                        .trim_end_matches(", DE")
                        .trim_end_matches(",DE")
                        .trim();
                    city = after.to_owned();
                    // Straße = Zeile direkt davor, wenn sie eine Nummer trägt.
                    if i > 0 && lines[i - 1].chars().any(|c| c.is_ascii_digit()) {
                        street = lines[i - 1].clone();
                    }
                }
            }
        }
        if let Some(v) = line.strip_prefix("Telefon:") {
            if phone.is_empty() {
                phone = v.trim().to_owned();
            }
        }
        if let Some(v) = line.strip_prefix("Mail:") {
            let v = v.trim().to_owned();
            if email.is_empty() && v.contains('@') {
                email = v;
            }
        }
    }
    if street.is_empty() && postcode.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "keine Kontaktdaten gefunden".to_owned(),
        });
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

/// Strip tags from a fragment (entities are already decoded by html5ever).
fn strip_tags(s: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in s.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            out.push(c);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_error(detail: &str) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.to_owned(),
    }
}

pub(crate) fn canonical_tier(tier: &str) -> Result<String, IngestError> {
    let trimmed = tier.trim();
    let tokens: Vec<_> = trimmed
        .strip_suffix(':')
        .unwrap_or(trimmed)
        .split_whitespace()
        .collect();
    if tokens.len() != 3
        || tokens[0] != "ab"
        || !matches!(tokens[2], "kg" | "t")
        || !tokens[1]
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '.' | ','))
        || !tokens[1].starts_with(|c: char| c.is_ascii_digit())
        || !tokens[1].ends_with(|c: char| c.is_ascii_digit())
    {
        return Err(parse_error(&format!(
            "Mengenschwelle unverständlich: {tier}"
        )));
    }
    // Quantity labels use German notation: optional grouped thousands and
    // decimal comma. Unlike price cells, arbitrary dot decimals are ambiguous.
    let mut decimal = tokens[1].split(',');
    let whole = decimal.next().unwrap();
    let fraction = decimal.next();
    let groups: Vec<_> = whole.split('.').collect();
    if decimal.next().is_some()
        || groups
            .iter()
            .any(|g| g.is_empty() || !g.chars().all(|c| c.is_ascii_digit()))
        || (groups.len() > 1
            && (groups[0].len() > 3 || groups.iter().skip(1).any(|g| g.len() != 3)))
        || fraction.is_some_and(|f| f.is_empty() || !f.chars().all(|c| c.is_ascii_digit()))
    {
        return Err(parse_error(&format!("mehrdeutige Mengenzahl: {tier}")));
    }
    let whole = groups.join("");
    let whole = whole.trim_start_matches('0');
    let whole = if whole.is_empty() { "0" } else { whole };
    let fraction = fraction.unwrap_or("").trim_end_matches('0');
    let quantity = if fraction.is_empty() {
        whole.to_owned()
    } else {
        format!("{whole}.{fraction}")
    };
    quantity
        .parse::<f64>()
        .ok()
        .filter(|n| n.is_finite() && *n > 0.0)
        .ok_or_else(|| parse_error(&format!("Mengenschwelle unverständlich: {tier}")))?;
    Ok(format!("ab {quantity} {}", tokens[2]))
}

/// Webflow's conditional visibility is inherited by descendant elements.
/// Do not infer visibility from whether a material happens to be mapped.
fn conditionally_hidden(element: ElementRef<'_>) -> bool {
    std::iter::once(element)
        .chain(element.ancestors().filter_map(ElementRef::wrap))
        .any(|e| e.value().classes().any(|c| c == "w-condition-invisible"))
}

fn visible_text(element: ElementRef<'_>) -> String {
    element
        .descendants()
        .filter(|node| {
            !node
                .ancestors()
                .filter_map(ElementRef::wrap)
                .any(conditionally_hidden)
        })
        .filter_map(|node| node.value().as_text().map(|text| text.to_string()))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse(html: &str) -> Result<(Vec<Row>, Vec<String>), IngestError> {
    let doc = Html::parse_document(html);
    let hints = Selector::parse(".hinweis").unwrap();
    let hints: Vec<_> = doc
        .select(&hints)
        .filter(|e| !conditionally_hidden(*e))
        .map(visible_text)
        .collect();
    if !hints
        .iter()
        .any(|s| s.contains("freibleibende Preise pro Kilogramm."))
        || hints
            .iter()
            .any(|s| s.contains("Preise pro Tonne") || s.contains("Preise pro Gramm"))
    {
        return Err(parse_error(
            "expliziter Preismaß-Hinweis pro Kilogramm fehlt oder widerspricht",
        ));
    }
    let section = Selector::parse("section#preiseSchrott").unwrap();
    let sections: Vec<_> = doc.select(&section).collect();
    if sections.len() != 1 || conditionally_hidden(sections[0]) {
        return Err(parse_error("Preisbereich fehlt oder mehrdeutig"));
    }
    // The operator splits the list over #preiseSchrott and two #info sections.
    // Keep all product-list sections, never navigation/header/footer cards.
    let card =
        Selector::parse("section.section_product4 a.product4_text-link").expect("valid selector");
    let title = Selector::parse("div.text-weight-semibold").expect("valid selector");
    let tier_sel = Selector::parse("div.pricedisplay").expect("valid selector");
    let cards: Vec<ElementRef> = doc.select(&card).collect();
    if cards.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiskarten".to_owned(),
        });
    }
    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for c in cards {
        if conditionally_hidden(c) {
            continue;
        }
        let Some(t) = c.select(&title).next() else {
            return Err(parse_error("sichtbare Preiskarte ohne Titel"));
        };
        let name = visible_text(t);
        if name.is_empty() || name.len() > 120 {
            return Err(parse_error("sichtbare Preiskarte mit ungültigem Titel"));
        }
        // Account for EVERY visible child, not only rows a selector recognizes.
        // A changed row class with still-visible prices is not request-only.
        let mut request_only = false;
        for node in c.children() {
            if let Some(child) = ElementRef::wrap(node) {
                if conditionally_hidden(child) {
                    continue;
                }
                let text = visible_text(child);
                if child.id() == t.id() || text.is_empty() {
                    continue;
                }
                if child.value().name() == "div"
                    && child.value().classes().any(|class| class == "pricedisplay")
                {
                    continue;
                }
                if child.value().name() == "div" && text == "Preis auf Anfrage" {
                    request_only = true;
                    continue;
                }
                return Err(parse_error(&format!(
                    "{name}: uninterpretierte sichtbare Kartenfelder"
                )));
            } else if node
                .value()
                .as_text()
                .is_some_and(|text| !text.trim().is_empty())
            {
                return Err(parse_error(&format!(
                    "{name}: uninterpretierter sichtbarer Kartentext"
                )));
            }
        }
        let mut kept = false;
        let mut zero_skipped = false;
        let mut empty_fields = 0;
        for tier_el in c.select(&tier_sel) {
            if conditionally_hidden(tier_el) {
                continue;
            }
            // Ignore the hidden kg/t alternative rather than depending on
            // its position. Visible cells are quantity threshold, value, €.
            // Consume the complete row, not only descendants matching div.
            // Otherwise a moved visible value could masquerade as CMS-empty
            // and authorize retirement of a quote that is still offered.
            let mut elements = Vec::new();
            for node in tier_el.children() {
                if let Some(child) = ElementRef::wrap(node) {
                    if conditionally_hidden(child) {
                        continue;
                    }
                    if child.value().name() == "div" {
                        elements.push(child);
                    } else if !visible_text(child).is_empty() {
                        return Err(parse_error(&format!(
                            "{name}: uninterpretierte sichtbare Staffelzellen"
                        )));
                    }
                } else if node
                    .value()
                    .as_text()
                    .is_some_and(|text| !text.trim().is_empty())
                {
                    return Err(parse_error(&format!(
                        "{name}: uninterpretierter sichtbarer Staffeltext"
                    )));
                }
            }
            let cells: Vec<_> = elements.iter().copied().map(visible_text).collect();
            if cells.len() != 3 || cells[2].trim() != "€" {
                return Err(parse_error(&format!(
                    "{name}: sichtbare Preiszellen unverständlich"
                )));
            }
            let tier = cells[0].trim().to_owned();
            let value = cells[1].trim().to_owned();
            canonical_tier(&tier)?;
            if value.is_empty() {
                if !elements[1]
                    .value()
                    .classes()
                    .any(|class| class == "w-dyn-bind-empty")
                {
                    return Err(parse_error(&format!(
                        "{name}: leeres Preisfeld ohne CMS-Kennzeichnung"
                    )));
                }
                empty_fields += 1;
                continue;
            }
            if !value
                .chars()
                .all(|c| c.is_ascii_digit() || matches!(c, '.' | ','))
            {
                return Err(parse_error(&format!(
                    "{name}: kein gültiger Preis: {value}"
                )));
            }
            let price: f64 = value
                .replace(',', ".")
                .parse()
                .map_err(|_| parse_error("Preiszahl unverständlich"))?;
            if !price.is_finite() || price < 0.0 {
                return Err(parse_error("ungültiger Preis"));
            }
            // Zero is not a quote; do not suppress later positive thresholds.
            if price == 0.0 {
                skipped.push(format!("{name} ({tier}: Preis 0,00 — kein Ankaufspreis)"));
                zero_skipped = true;
                continue;
            }
            rows.push(Row {
                title: name.clone(),
                tier,
                price,
                unit: "EUR/kg",
            });
            kept = true;
        }
        if request_only && kept {
            return Err(parse_error(&format!(
                "{name}: Anfrage und sichtbarer positiver Preis widersprechen sich"
            )));
        }
        if !kept {
            if request_only {
                skipped.push(format!("{name} (kein Preis: Preis auf Anfrage)"));
            } else if empty_fields > 0 {
                skipped.push(format!(
                    "{name} (kein Preis: {empty_fields} leere CMS-Preisfelder)"
                ));
            } else if !zero_skipped {
                return Err(parse_error(&format!("{name}: kein verifizierter Angebotszustand (keine sichtbare Anfrage/CMS-Leer-/Nullpreiszeile)")));
            }
        }
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    Ok((rows, skipped))
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse};

    const LIVE: &str = include_str!("fixtures/msg-preisliste-20261010.html");

    fn edit_card(html: &str, title: &str, edit: impl FnOnce(&str) -> String) -> String {
        let marker = format!("class=\"text-size-medium text-weight-semibold\">{title}</div>");
        let title_at = html.find(&marker).expect("real source card title");
        let start = html[..title_at].rfind("<a ").unwrap();
        let end = title_at + html[title_at..].find("</a>").unwrap() + 4;
        let mut changed = html.to_owned();
        changed.replace_range(start..end, &edit(&html[start..end]));
        changed
    }

    fn database_state(public: &schrott_mcp_store::PublicDb) -> serde_json::Value {
        serde_json::json!({
            "current": public.query_sql("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant").unwrap().rows,
            "history": public.query_sql("SELECT * FROM prices ORDER BY id").unwrap().rows,
        })
    }

    async fn seeded_current_snapshot(
        name: &str,
    ) -> (
        crate::test_support::TempDbDir,
        schrott_mcp_store::PublicDb,
        schrott_mcp_store::InternalDb,
        chrono::DateTime<chrono::Utc>,
    ) {
        let dir = crate::test_support::TempDbDir::new(name);
        let public = schrott_mcp_store::PublicDb::open(dir.path()).unwrap();
        let internal = schrott_mcp_store::InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-10T10:00:00+00:00")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let out = super::prices_from_html(LIVE).unwrap();
        super::super::super::record(&public, &internal, super::SLUG, &out, &now)
            .await
            .unwrap();
        let run = internal.create_run(&now.to_rfc3339()).unwrap();
        let step = internal
            .create_step(run, super::SLUG, &now.to_rfc3339())
            .unwrap();
        internal
            .finish_step(
                step,
                "ok",
                124,
                "verified source snapshot",
                &now.to_rfc3339(),
            )
            .unwrap();
        (dir, public, internal, now)
    }

    #[tokio::test]
    async fn unconsumed_visible_tier_fields_cannot_retire_current() {
        let (_dir, public, internal, now) = seeded_current_snapshot("msg-r1b").await;
        let before = database_state(&public);
        for replacement in [
            "<div class=\"text-size-medium w-dyn-bind-empty\"></div><span class=\"changed-price-value\">9.90</span>",
            "<div class=\"text-size-medium w-dyn-bind-empty\"></div>9.90",
        ] {
            let damaged = edit_card(LIVE, "Millberry", |card| {
                let changed = card.replacen(
                    "<div class=\"text-size-medium\">9.90</div>",
                    replacement,
                    1,
                );
                assert_ne!(changed, card, "real price field must be mutated");
                changed
            });
            let result = match super::prices_from_html(&damaged) {
                Ok(out) => super::super::super::record(
                    &public,
                    &internal,
                    super::SLUG,
                    &out,
                    &(now + chrono::Duration::hours(1)),
                )
                .await
                .map(|_| ()),
                Err(e) => Err(e),
            };
            assert!(result.is_err(), "visible unconsumed price must block snapshot");
            assert_eq!(database_state(&public), before, "entire Current/History unchanged");
        }

        // Text children inside the consumed value cell remain supported.
        let wrapped = edit_card(LIVE, "Millberry", |card| {
            card.replacen(">9.90</div>", "><span>9.90</span></div>", 1)
        });
        let out = super::prices_from_html(&wrapped).unwrap();
        assert_eq!(out.prices.len(), 124);
        assert_eq!(
            out.prices
                .iter()
                .find(|p| p.variant == "Millberry / ab 1 kg")
                .unwrap()
                .price,
            9.9
        );
    }

    #[tokio::test]
    async fn damaged_card_structure_cannot_publish_or_retire_any_current_or_history() {
        let (_dir, public, internal, now) = seeded_current_snapshot("msg-r1").await;
        let before = database_state(&public);
        let damaged = edit_card(LIVE, "Millberry", |card| {
            card.replace("class=\"pricedisplay\"", "class=\"changed-pricedisplay\"")
        });
        let result = match super::prices_from_html(&damaged) {
            Ok(out) => super::super::super::record(
                &public,
                &internal,
                super::SLUG,
                &out,
                &(now + chrono::Duration::hours(1)),
            )
            .await
            .map(|_| ()),
            Err(e) => Err(e),
        };
        assert_eq!(
            database_state(&public),
            before,
            "damaged real source must leave entire Current AND History unchanged"
        );
        assert!(
            result.is_err(),
            "uninterpreted price markup must not invent request-only"
        );
    }

    #[tokio::test]
    async fn composed_unknown_material_titles_cannot_publish_or_retire() {
        let (_dir, public, internal, now) = seeded_current_snapshot("msg-r2").await;
        let before = database_state(&public);
        for (title, unknown) in [
            ("Alu Kabel", "Alu Kabel mit Kupfer"),
            ("V2A Edelstahl", "V2A/V4A Edelstahl"),
            ("V4A Edelstahl", "V4A Edelstahl mit Kupfer"),
            ("Millberry", "Millberry mit Aluminium"),
            ("Kabel", "Kabel Alu/Kupfer gemischt"),
        ] {
            let damaged = edit_card(LIVE, title, |card| {
                card.replace(&format!(">{title}</div>"), &format!(">{unknown}</div>"))
            });
            let result = match super::prices_from_html(&damaged) {
                Ok(out) => super::super::super::record(
                    &public,
                    &internal,
                    super::SLUG,
                    &out,
                    &(now + chrono::Duration::hours(1)),
                )
                .await
                .map(|_| ()),
                Err(e) => Err(e),
            };
            assert_eq!(
                database_state(&public),
                before,
                "unknown composed title {unknown}"
            );
            assert!(
                result.is_err(),
                "unknown composed title {unknown} must fail closed"
            );
            let mut forged = super::prices_from_html(LIVE).unwrap();
            let price = forged
                .prices
                .iter_mut()
                .find(|p| p.label.starts_with(&format!("{title} (")))
                .unwrap();
            price.label = price.label.replacen(title, unknown, 1);
            price.variant = price.variant.replacen(title, unknown, 1).into();
            assert!(super::super::super::record(
                &public,
                &internal,
                super::SLUG,
                &forged,
                &(now + chrono::Duration::hours(1))
            )
            .await
            .is_err());
            assert_eq!(
                database_state(&public),
                before,
                "Record rejects composed title even if an outcome bypasses parser"
            );
        }
    }

    #[tokio::test]
    async fn explicit_non_offer_states_and_canonical_duplicate_quantities() {
        let (_dir, public, internal, now) = seeded_current_snapshot("msg-non-offers").await;
        let before = database_state(&public);
        for damaged in [
            edit_card(LIVE, "Millberry", |card| {
                card.replacen(
                    "class=\"pricedisplay\"",
                    "class=\"changed-pricedisplay\"",
                    1,
                )
            }),
            edit_card(LIVE, "Millberry", |card| {
                card.replace(
                    "class=\"pricedisplay\"",
                    "class=\"pricedisplay w-condition-invisible\"",
                )
            }),
            edit_card(LIVE, "Millberry", |card| {
                card.replace("class=\"pricedisplay\"", "class=\"changed-pricedisplay\"")
                    .replace(
                        "text-size-medium w-condition-invisible\">Preis auf Anfrage",
                        "text-size-medium\">Preis auf Anfrage",
                    )
            }),
            edit_card(LIVE, "Millberry", |card| {
                card.replace(">9.90<", "><")
                    .replace(">10.10<", "><")
                    .replace(">11.00<", "><")
            }),
        ] {
            assert!(super::prices_from_html(&damaged).is_err());
            assert_eq!(database_state(&public), before);
        }
        let conflicts = edit_card(LIVE, "Millberry", |card| {
            card.replace("ab 100 kg:", "ab 1000,50 kg:")
                .replace("ab 1000 kg:", "ab 001.000,500 kg:")
        });
        assert!(
            super::prices_from_html(&conflicts).is_err(),
            "equivalent quantities with different prices conflict"
        );
        assert_eq!(database_state(&public), before);
        let equal = edit_card(&conflicts, "Millberry", |card| {
            card.replace(">11.00<", ">10.10<")
        });
        let out = super::prices_from_html(&equal).unwrap();
        assert_eq!(
            out.prices
                .iter()
                .filter(|p| p.variant == "Millberry / ab 1000.5 kg")
                .count(),
            1
        );
        super::super::super::record(
            &public,
            &internal,
            super::SLUG,
            &out,
            &(now + chrono::Duration::minutes(30)),
        )
        .await
        .unwrap();
        let tid = public.find_trader_id(super::SLUG).unwrap().unwrap();
        let mid = public
            .find_material_id("kupfer-millberry")
            .unwrap()
            .unwrap();
        assert_eq!(
            public
                .current_price_for(tid, mid, "Millberry / ab 1000.5 kg")
                .unwrap()
                .unwrap()
                .price,
            10.1
        );
        assert_eq!(
            public
                .price_history(tid, mid, "Millberry / ab 1000.5 kg", 10)
                .unwrap()
                .len(),
            1
        );
        // Visible literal request-only is evidence; hidden request text is not.
        let request = edit_card(LIVE, "Millberry", |card| {
            card.replace(
                "class=\"pricedisplay\"",
                "class=\"pricedisplay w-condition-invisible\"",
            )
            .replace(
                "text-size-medium w-condition-invisible\">Preis auf Anfrage",
                "text-size-medium\">Preis auf Anfrage",
            )
        });
        let empty = edit_card(LIVE, "Millberry", |card| {
            card.replace(
                "text-size-medium\">9.90",
                "text-size-medium w-dyn-bind-empty\">",
            )
            .replace(
                "text-size-medium\">10.10",
                "text-size-medium w-dyn-bind-empty\">",
            )
            .replace(
                "text-size-medium\">11.00",
                "text-size-medium w-dyn-bind-empty\">",
            )
        });
        let zero = edit_card(LIVE, "Millberry", |card| {
            card.replace(">9.90<", ">0.00<")
                .replace(">10.10<", ">0.00<")
                .replace(">11.00<", ">0.00<")
        });
        let hidden = edit_card(LIVE, "Millberry", |card| {
            card.replacen(
                "product4_text-link w-inline-block",
                "product4_text-link w-inline-block w-condition-invisible",
                1,
            )
        });
        for (index, (html, diagnostic)) in [
            (request, "Preis auf Anfrage"),
            (empty, "leere CMS-Preisfelder"),
            (zero, "Preis 0,00"),
            (hidden, ""),
        ]
        .into_iter()
        .enumerate()
        {
            let out = super::prices_from_html(&html).unwrap();
            assert!(!out.prices.iter().any(|p| p.material == "kupfer-millberry"));
            if !diagnostic.is_empty() {
                assert!(out
                    .skipped_labels
                    .iter()
                    .any(|s| s.starts_with("Millberry (") && s.contains(diagnostic)));
            }
            if diagnostic != "Preis auf Anfrage" {
                assert!(!out
                    .skipped_labels
                    .iter()
                    .any(|s| s == "Millberry (kein Preis: Preis auf Anfrage)"));
            }
            let copper_history = public
                .query_sql(&format!(
                    "SELECT * FROM prices WHERE trader_id={tid} AND material_id={mid} ORDER BY id"
                ))
                .unwrap()
                .rows;
            super::super::super::record(
                &public,
                &internal,
                super::SLUG,
                &out,
                &(now + chrono::Duration::hours(index as i64 + 1)),
            )
            .await
            .unwrap();
            assert!(public
                .current_prices_from(tid, super::URL)
                .unwrap()
                .iter()
                .all(|p| p.material_id != mid));
            assert_eq!(public.query_sql(&format!("SELECT * FROM prices WHERE trader_id={tid} AND material_id={mid} ORDER BY id")).unwrap().rows, copper_history, "legitimate non-offer retires pointers, never copper history");
        }
    }

    #[test]
    fn dynamic_quantities_units_zero_and_duplicate_contract() {
        use super::prices_from_html;
        assert_eq!(
            super::canonical_tier("ab 001.000,500 kg:").unwrap(),
            "ab 1000.5 kg"
        );
        assert_ne!(
            super::canonical_tier("ab 9007199254740992 kg:").unwrap(),
            super::canonical_tier("ab 9007199254740993 kg:").unwrap(),
            "identity must not round through f64"
        );
        for malformed in [
            "ab 0 kg:",
            "ab 1.2 kg:",
            "ab 1.2,5 kg:",
            "ab 1, kg:",
            "ab 1 kg::",
        ] {
            assert!(super::canonical_tier(malformed).is_err(), "{malformed}");
        }
        let html = FIXTURE
            .replace("ab 100 kg:", "ab 137,5 t:")
            .replace("1.45", "1.30");
        let out = prices_from_html(&html).unwrap();
        assert_eq!(out.prices.len(), 5);
        assert!(out
            .prices
            .iter()
            .any(|p| p.variant == "Alu Bleche blank / ab 137.5 t"
                && p.price == 1.3
                && p.unit == "EUR/kg"));
        assert!(out
            .prices
            .iter()
            .any(|p| p.variant == "Alu Bleche blank / ab 1 kg" && p.price == 1.3));
        let zero_first = FIXTURE.replacen(">1.30<", ">0.00<", 1);
        let out = prices_from_html(&zero_first).unwrap();
        assert!(out
            .prices
            .iter()
            .any(|p| p.variant == "Alu Bleche blank / ab 100 kg"));
        assert!(!out.prices.iter().any(|p| p.price <= 0.0));
        let duplicate = FIXTURE.replace("Bronze Schrott", "Alu Bleche blank");
        assert!(prices_from_html(&duplicate)
            .unwrap_err()
            .to_string()
            .contains("doppelte"));
        let identical = duplicate.replace("5.30", "1.30");
        let out = prices_from_html(&identical).unwrap();
        assert_eq!(
            out.prices
                .iter()
                .filter(|p| p.variant == "Alu Bleche blank / ab 1 kg")
                .count(),
            1
        );
        let renamed =
            prices_from_html(&FIXTURE.replace("Alu Bleche blank", "ALU BLECHE BLANK")).unwrap();
        assert!(renamed
            .prices
            .iter()
            .any(|p| p.variant == "ALU BLECHE BLANK / ab 1 kg"));
        for (from, to) in [
            ("Alu Bleche blank", "Unbekannte neue Sorte"),
            ("pro Kilogramm.", "pro Tonne."),
            ("ab 100 kg:", "ab 100 Sack:"),
            (">1.30<", ">-1.30<"),
            (">1.30<", ">1.30 USD<"),
        ] {
            assert!(
                prices_from_html(&FIXTURE.replace(from, to)).is_err(),
                "must fail closed: {to}"
            );
        }
    }

    #[tokio::test]
    async fn real_parser_record_reconciles_legacy_and_changed_snapshots() {
        use super::{prices_from_html, SLUG, URL};
        use schrott_mcp_store::{InternalDb, NewPrice, PublicDb};
        let dir = crate::test_support::TempDbDir::new("msg-snapshot");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let tid = public.find_trader_id(SLUG).unwrap().unwrap();
        let copper = public
            .find_material_id("kupfer-millberry")
            .unwrap()
            .unwrap();
        let brass = public.find_material_id("messing").unwrap().unwrap();
        let light = public.find_material_id("messing-leicht").unwrap().unwrap();
        let v4a = public.find_material_id("edelstahl-v4a").unwrap().unwrap();
        let other = public
            .find_trader_id("hb-woltmershausen-vedder-stockrahm")
            .unwrap()
            .unwrap();
        let old = |trader_id, material_id, variant, source_url, notes| NewPrice {
            trader_id,
            material_id,
            variant,
            price: 3.0,
            currency: "EUR",
            unit: "EUR/kg",
            price_kind: "exact",
            price_min: None,
            price_max: None,
            confidence: Some(1.0),
            source_type: "haendler_angabe",
            published: true,
            source_url,
            observed_at: "2026-10-09T10:00:00+00:00",
            published_at: None,
            valid_from: None,
            valid_to: None,
            notes,
            extra_json: "{}",
            ingested_at: "2026-10-09T10:00:00+00:00",
        };
        // Independent production SELECT oracle, never generated by this parser
        // or mapping table. Preserve actual source, dates, units and old map_v.
        let oracle: serde_json::Value =
            serde_json::from_str(include_str!("fixtures/msg-legacy-current-20261010.json"))
                .unwrap();
        assert_eq!(oracle["trader_slug"], SLUG);
        let defaults = &oracle["defaults"];
        assert_eq!(defaults["source_url"], URL);
        let mut legacy = Vec::new();
        let mut original_keys = std::collections::HashSet::new();
        for row in oracle["rows"].as_array().unwrap() {
            let mid = public
                .find_material_id(row[0].as_str().unwrap())
                .unwrap()
                .unwrap();
            let variant = row[1].as_str().unwrap();
            assert!(
                original_keys.insert((row[0].as_str().unwrap(), variant)),
                "independent keys must be unique"
            );
            let observed = row[6]
                .as_str()
                .unwrap_or(defaults["observed_at"].as_str().unwrap());
            legacy.push(
                public
                    .record_price(&NewPrice {
                        trader_id: tid,
                        material_id: mid,
                        variant,
                        price: row[2].as_f64().unwrap(),
                        currency: defaults["currency"].as_str().unwrap(),
                        unit: row[3].as_str().unwrap(),
                        price_kind: defaults["price_kind"].as_str().unwrap(),
                        price_min: defaults["price_min"].as_f64(),
                        price_max: defaults["price_max"].as_f64(),
                        confidence: defaults["confidence"].as_f64(),
                        source_type: defaults["source_type"].as_str().unwrap(),
                        published: defaults["published"].as_bool().unwrap(),
                        source_url: defaults["source_url"].as_str().unwrap(),
                        observed_at: observed,
                        published_at: defaults["published_at"].as_str(),
                        valid_from: defaults["valid_from"].as_str(),
                        valid_to: defaults["valid_to"].as_str(),
                        notes: row[4].as_str().unwrap(),
                        extra_json: row[5].as_str().unwrap(),
                        ingested_at: observed,
                    })
                    .unwrap(),
            );
        }
        assert_eq!(legacy.len(), 46);
        let protected = [
            public
                .record_price(&old(
                    tid,
                    copper,
                    "foreign",
                    "https://other.test/",
                    "Millberry (ab 1 kg:)",
                ))
                .unwrap(),
            public
                .record_price(&old(
                    other,
                    copper,
                    "Millberry",
                    URL,
                    "Millberry (ab 1 kg:)",
                ))
                .unwrap(),
            public
                .record_price(&old(tid, copper, "unproven", URL, "Unbewiesene Notiz"))
                .unwrap(),
        ];
        let now = chrono::DateTime::parse_from_rfc3339(defaults["observed_at"].as_str().unwrap())
            .unwrap()
            .with_timezone(&chrono::Utc)
            + chrono::Duration::hours(1);
        let legacy_query = format!(
            "SELECT * FROM prices WHERE id IN ({}) ORDER BY id",
            legacy
                .iter()
                .map(i64::to_string)
                .collect::<Vec<_>>()
                .join(",")
        );
        let original_history = public.query_sql(&legacy_query).unwrap().rows;
        let out = prices_from_html(LIVE).unwrap();
        assert_eq!(out.prices.len(), 124);
        assert_eq!(
            out.skipped_labels
                .iter()
                .filter(|s| s.starts_with("Kataloglücke:"))
                .count(),
            9
        );
        let first = super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        assert_eq!(first.recorded, 124);
        for p in &out.prices {
            let mid = public.find_material_id(p.material).unwrap().unwrap();
            let stored = public
                .current_price_for(tid, mid, p.variant.as_ref())
                .unwrap()
                .unwrap();
            assert_eq!(stored.notes, p.label);
            assert_eq!(
                stored.price,
                p.price * if stored.unit == "EUR/t" { 1000.0 } else { 1.0 }
            );
            assert_eq!(stored.source_url, URL);
        }
        for (mid, variant) in [
            (copper, "Millberry"),
            (brass, "Wasseruhren"),
            (light, "Wasseruhren"),
            (v4a, ""),
        ] {
            assert!(public
                .current_price_for(tid, mid, variant)
                .unwrap()
                .is_none());
        }
        for id in legacy {
            assert_eq!(
                public
                    .query_sql(&format!(
                        "SELECT COUNT(*) FROM current_prices WHERE price_id={id}"
                    ))
                    .unwrap()
                    .rows[0][0],
                0,
                "every proven legacy pointer retired"
            );
            let history = public
                .query_sql(&format!("SELECT notes,price FROM prices WHERE id={id}"))
                .unwrap();
            assert_eq!(history.rows.len(), 1, "legacy history preserved");
        }
        assert_eq!(
            public.query_sql(&legacy_query).unwrap().rows,
            original_history,
            "all legacy fields stay byte-for-byte unchanged"
        );
        for id in protected {
            assert_eq!(
                public
                    .query_sql(&format!(
                        "SELECT COUNT(*) FROM current_prices WHERE price_id={id}"
                    ))
                    .unwrap()
                    .rows[0][0],
                1
            );
        }
        let fe = public
            .find_material_id("eisenschrott-gussbruch")
            .unwrap()
            .unwrap();
        let brake = public
            .current_price_for(tid, fe, "Bremsscheiben / ab 100 kg")
            .unwrap()
            .unwrap();
        assert_eq!((brake.price, brake.unit.as_str()), (190.0, "EUR/t"));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&brake.extra_json).unwrap()["quantity_unit"],
            "kg"
        );
        assert_eq!(
            public
                .current_price_for(tid, copper, "Millberry / ab 1000 kg")
                .unwrap()
                .unwrap()
                .price,
            11.0
        );
        let count_before = public
            .query_sql("SELECT COUNT(*) FROM prices")
            .unwrap()
            .rows[0][0]
            .clone();
        super::super::super::record(
            &public,
            &internal,
            SLUG,
            &out,
            &(now + chrono::Duration::hours(1)),
        )
        .await
        .unwrap();
        assert_eq!(
            public
                .query_sql("SELECT COUNT(*) FROM prices")
                .unwrap()
                .rows[0][0],
            count_before,
            "repeat idempotent"
        );
        let mut jump = out.clone();
        jump.prices
            .iter_mut()
            .find(|p| p.variant == "Millberry / ab 1 kg")
            .unwrap()
            .price = 40.0;
        let change_at = now + chrono::Duration::minutes(90);
        let summary = super::super::super::record(&public, &internal, SLUG, &jump, &change_at)
            .await
            .unwrap();
        assert!(summary.canaries.iter().any(|s| s.contains("Preissprung")));
        let changed_price = public
            .current_price_for(tid, copper, "Millberry / ab 1 kg")
            .unwrap()
            .unwrap();
        assert_eq!(
            changed_price.published_at.as_deref(),
            Some("2026-10-10T00:00:00+00:00")
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&changed_price.extra_json).unwrap()
                ["published_at_basis"],
            "observed_price_change"
        );
        super::super::super::record(
            &public,
            &internal,
            SLUG,
            &out,
            &(change_at + chrono::Duration::minutes(1)),
        )
        .await
        .unwrap();
        assert_eq!(
            public
                .price_history(tid, copper, "Millberry / ab 1 kg", 10)
                .unwrap()
                .len(),
            3,
            "A -> B -> A price events survive snapshot reconciliation"
        );
        // Actual source parser produces renamed grades and a newly appearing t threshold.
        let changed = LIVE
            .replace(
                "text-weight-semibold\">Millberry<",
                "text-weight-semibold\">MILLBERRY<",
            )
            .replace("ab 1000 kg:", "ab 137,5 t:");
        let changed = prices_from_html(&changed).unwrap();
        super::super::super::record(
            &public,
            &internal,
            SLUG,
            &changed,
            &(now + chrono::Duration::hours(2)),
        )
        .await
        .unwrap();
        assert!(public
            .current_price_for(tid, copper, "Millberry / ab 1 kg")
            .unwrap()
            .is_none());
        let new = public
            .current_price_for(tid, copper, "MILLBERRY / ab 137.5 t")
            .unwrap()
            .unwrap();
        assert_eq!((new.price, new.unit.as_str()), (11.0, "EUR/kg"));
        assert_eq!(
            new.published_at, None,
            "rename is not a comparable price-change event"
        );
        assert_eq!(
            public
                .price_history(tid, copper, "Millberry / ab 1 kg", 10)
                .unwrap()
                .len(),
            3
        );
        // Invalid outcome must not retire any current pointer.
        let snapshot = public
            .query_sql("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant")
            .unwrap()
            .rows;
        let mut bad = changed.clone();
        bad.prices[0].material = "not-in-catalog";
        assert!(super::super::super::record(
            &public,
            &internal,
            SLUG,
            &bad,
            &(now + chrono::Duration::hours(3))
        )
        .await
        .is_err());
        bad = changed.clone();
        bad.status_code = 503;
        assert!(super::super::super::record(
            &public,
            &internal,
            SLUG,
            &bad,
            &(now + chrono::Duration::hours(3))
        )
        .await
        .is_err());
        assert!(super::super::super::record(
            &public,
            &internal,
            SLUG,
            &out,
            &(now - chrono::Duration::hours(1))
        )
        .await
        .is_err());
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant")
                .unwrap()
                .rows,
            snapshot
        );
        // Missing real catalog entry (not merely a forged material label).
        let missing_dir = crate::test_support::TempDbDir::new("msg-missing-catalog");
        let missing = PublicDb::open(missing_dir.path()).unwrap();
        crate::seed_traders(&missing, &now.to_rfc3339()).unwrap();
        missing
            .upsert_material(&schrott_mcp_store::NewMaterial {
                slug: "kupfer-millberry",
                name_de: "Kupfer",
                category: "nichteisen",
                unit: "EUR/kg",
                description: "",
                updated_at: &now.to_rfc3339(),
            })
            .unwrap();
        let missing_tid = missing.find_trader_id(SLUG).unwrap().unwrap();
        let missing_mid = missing
            .find_material_id("kupfer-millberry")
            .unwrap()
            .unwrap();
        let retained = missing
            .record_price(&old(
                missing_tid,
                missing_mid,
                "Millberry",
                URL,
                "Millberry (ab 1 kg:)",
            ))
            .unwrap();
        assert!(super::super::super::record(
            &missing,
            &internal,
            SLUG,
            &out,
            &(now + chrono::Duration::hours(3))
        )
        .await
        .is_err());
        assert_eq!(
            missing
                .current_price_for(missing_tid, missing_mid, "Millberry")
                .unwrap()
                .unwrap()
                .id,
            retained
        );
        assert_eq!(
            missing
                .query_sql("SELECT COUNT(*) FROM prices")
                .unwrap()
                .rows[0][0],
            1
        );
        let readonly = PublicDb::open_read_only(dir.path()).unwrap();
        assert!(
            super::super::super::record(
                &readonly,
                &internal,
                SLUG,
                &out,
                &(now + chrono::Duration::hours(3))
            )
            .await
            .is_err(),
            "enrichment write failure must block retirement"
        );
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant")
                .unwrap()
                .rows,
            snapshot
        );
        assert!(prices_from_html(&LIVE.replace("ab 100 kg:", "ab unbekannt kg:")).is_err());
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant")
                .unwrap()
                .rows,
            snapshot
        );
        let run = internal.create_run(&now.to_rfc3339()).unwrap();
        let step = internal.create_step(run, SLUG, &now.to_rfc3339()).unwrap();
        internal
            .finish_step(step, "ok", 124, "complete snapshot", &now.to_rfc3339())
            .unwrap();
        let partial = prices_from_html(FIXTURE).unwrap();
        assert!(super::super::super::record(
            &public,
            &internal,
            SLUG,
            &partial,
            &(now + chrono::Duration::hours(4))
        )
        .await
        .is_err());
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant")
                .unwrap()
                .rows,
            snapshot
        );
    }

    #[test]
    fn real_fixture_keeps_every_visible_positive_tier() {
        let (rows, _) = parse(LIVE).unwrap();
        let millberry: Vec<_> = rows.iter().filter(|r| r.title == "Millberry").collect();
        assert_eq!(millberry.len(), 3, "all visible thresholds required");
        assert_eq!(rows.len(), 133, "source inventory, before material mapping");
        assert!(!rows.iter().any(|r| r.title == "Indium"));
    }

    // Real excerpt shape of the live page (Webflow CMS cards, 28.09.2026):
    // title + up-to-three pricedisplay tiers (value cell or
    // w-dyn-bind-empty), "0,00" tier, all-empty card, first-tier-empty
    // card, stray euro amounts outside the price window.
    const FIXTURE: &str = "<p class=\"hinweis\">freibleibende Preise pro Kilogramm.</p><p>Versand ab 49 €</p>\
        <section id=\"preiseSchrott\" class=\"section_product4\">\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Alu Bleche blank</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium\">1.30</div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 100 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 100 t:</div>\
        <div class=\"text-size-medium\">1.45</div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1000 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1000 t:</div>\
        <div class=\"text-size-medium\">1.65</div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Bronze Schrott</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium\">5.30</div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 100 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 100 t:</div>\
        <div class=\"text-size-medium w-dyn-bind-empty\"></div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Hartmetall Schlamm</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium\">0.00</div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Bremsscheiben</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium w-dyn-bind-empty\"></div><div class=\"text-size-medium\">€</div></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 100 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 100 t:</div>\
        <div class=\"text-size-medium\">0.19</div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        <div role=\"listitem\" class=\"w-dyn-item\"><div class=\"product4_item\">\
        <a href=\"#\" class=\"product4_text-link w-inline-block\">\
        <div class=\"text-size-medium text-weight-semibold\">Inconell</div>\
        <div class=\"spacer-xxsmall\"></div>\
        <div class=\"pricedisplay\"><div class=\"text-size-medium\">ab 1 kg:</div>\
        <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
        <div class=\"text-size-medium w-dyn-bind-empty\"></div><div class=\"text-size-medium\">€</div></div>\
        </a></div></div>\
        </section><footer><p>Container ab 99 €</p></footer>";

    #[test]
    fn hidden_cms_prices_never_become_quotes() {
        let html = FIXTURE.replacen(
            "class=\"pricedisplay\"",
            "class=\"pricedisplay w-condition-invisible\"",
            1,
        );
        let (rows, _) = parse(&html).expect("other visible quotes remain");
        let aluminium = rows.iter().find(|r| r.title == "Alu Bleche blank").unwrap();
        assert_eq!(aluminium.tier, "ab 100 kg:");
        assert_eq!(aluminium.price, 1.45);

        // Position of a hidden CMS alternative does not define the value.
        let html = FIXTURE.replace(
            "<div class=\"text-size-medium\">ab 1 kg:</div>\
         <div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>",
            "<div class=\"text-size-medium w-condition-invisible\">ab 1 t:</div>\
         <div class=\"text-size-medium\">ab 1 kg:</div>",
        );
        assert_ne!(html, FIXTURE, "fixture must actually reorder the cells");
        let (rows, _) = parse(&html).expect("visible cells independent of hidden position");
        assert_eq!(rows[0].tier, "ab 1 kg:");
        assert_eq!(rows[0].price, 1.3);

        // A hidden wrapper also hides its descendant cards and prices.
        let html = FIXTURE.replace(
            "class=\"product4_item\"",
            "class=\"product4_item w-condition-invisible\"",
        );
        assert!(parse(&html).is_err());
    }

    #[test]
    fn every_tier_and_window_holds() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        // Alu blank (1.30), Bronze Schrott (5.30), Bremsscheiben (0.19 ab
        // 100 kg): stray "49 €" before and "99 €" in the footer pair with
        // nothing. Schlamm-0.00 and Inconell-empty yield no rows.
        assert_eq!(rows.len(), 5, "rows: {rows:?}");
        assert_eq!(rows[0].title, "Alu Bleche blank");
        assert_eq!(rows[0].tier, "ab 1 kg:");
        assert_eq!(rows[0].price, 1.3);
        assert_eq!(rows[0].unit, "EUR/kg");
        assert_eq!(rows[3].title, "Bronze Schrott");
        assert_eq!(rows[3].price, 5.3);
        assert_eq!(rows[4].title, "Bremsscheiben");
        assert_eq!(rows[4].tier, "ab 100 kg:");
        assert_eq!(rows[4].price, 0.19);
        assert!(
            skips
                .iter()
                .any(|s| s.contains("Schlamm") && s.contains("0,00")),
            "{skips:?}"
        );
        assert!(
            skips
                .iter()
                .any(|s| s.contains("Inconell") && s.contains("kein Preis")),
            "{skips:?}"
        );
        assert!(!skips.iter().any(|s| s.contains("höhere Staffeln")));
    }

    #[test]
    fn foreign_tier_fails_closed() {
        let html = FIXTURE.replace("ab 1 kg:", "pro Sack:");
        assert!(parse(&html)
            .unwrap_err()
            .to_string()
            .contains("Mengenschwelle"));
    }

    #[test]
    fn missing_window_and_empty_cards_error() {
        let err = parse("<html><body><p>Neu hier</p></body></html>").expect_err("no window");
        assert!(err.to_string().contains("Preismaß"), "{err}");
        let mut html = FIXTURE.to_owned();
        for v in ["1.30", "1.45", "1.65", "5.30", "0.19", "0.00"] {
            html = html.replace(&format!(">{v}<"), "><");
        }
        let err = parse(&html).expect_err("all empty");
        assert!(err.to_string().contains("leer"), "{err}");
    }

    #[test]
    fn impressum_extracts_contact() {
        // Real fragment shape incl. zero-width joiner after Inhaber line.
        let imp = "<h1 class=\"heading-style-h1\">Impressum</h1>\
            <p class=\"text-size-medium\"><strong>Angaben gemäß § 5 TMG<br/><br/></strong>\
            MSG Recycling<br/>Gleichenstraße 36<br/>99867 Gotha, DE<br/><br/>\
            Inhaber: Helge Göbbel<br/>\u{200d}<br/><strong>Kontakt</strong><br/>\
            Telefon: +49 (0) 3621 3394 851<br/>Mail: info@msg-metallrecycling.de<br/></p>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Gleichenstraße 36");
        assert_eq!(info.postcode, "99867");
        assert_eq!(info.city, "Gotha");
        assert_eq!(info.phone, "+49 (0) 3621 3394 851");
        assert_eq!(info.email, "info@msg-metallrecycling.de");
        // Redesign without anchors fails loudly.
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
        assert!(extract_info("<h1>Impressum</h1><p>leer</p>").is_err());
    }

    #[test]
    fn mapping_covers_live_cards() {
        assert_eq!(
            grade_for("Millberry (ab 1 kg:)"),
            Some(("kupfer-millberry", "Millberry"))
        );
        assert_eq!(
            grade_for("Kupfer Lackdraht (ab 1 kg:)"),
            Some(("kupfer-berry", "Lackdraht"))
        );
        assert_eq!(
            grade_for("Kupfer Leitschienen Blank (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Leitschienen Blank"))
        );
        assert_eq!(
            grade_for("Kupfer Schwer (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Schwer"))
        );
        assert_eq!(
            grade_for("Kupfer Leicht (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Leicht"))
        );
        assert_eq!(
            grade_for("Kupfer Späne (ab 1 kg:)"),
            Some(("kupfer-gemischt", "Späne"))
        );
        assert_eq!(grade_for("Alu Kabel (ab 1 kg:)"), Some(("kabel-alu", "")));
        assert_eq!(
            grade_for("Kabel 50% Kupfer (ab 1 kg:)"),
            Some(("kabel-kupfer", "50%"))
        );
        assert_eq!(
            grade_for("Kabel 60% Kupfer (ab 1 kg:)"),
            Some(("kabel-kupfer", "60%"))
        );
        assert_eq!(
            grade_for("Kabel 80% Kupfer (ab 1 kg:)"),
            Some(("kabel-kupfer", "80%"))
        );
        assert_eq!(grade_for("Kabel (ab 1 kg:)"), Some(("kabel-kupfer", "")));
        assert_eq!(
            grade_for("Ms 58 Abfälle (ab 1 kg:)"),
            Some(("messing", "Ms 58 Abfälle"))
        );
        assert_eq!(
            grade_for("Ms Erodierdraht (ab 1 kg:)"),
            Some(("messing", "Ms Erodierdraht"))
        );
        assert_eq!(
            grade_for("Ms leicht (ab 1 kg:)"),
            Some(("messing", "Ms leicht"))
        );
        assert_eq!(
            grade_for("Ms Späne gemischt (ab 1 kg:)"),
            Some(("messing", "Ms Späne gemischt"))
        );
        assert_eq!(
            grade_for("Schwermessing (ab 1 kg:)"),
            Some(("messing", "Schwermessing"))
        );
        assert_eq!(
            grade_for("Wasseruhren (ab 1 kg:)"),
            Some(("messing-leicht", "Wasseruhren"))
        );
        assert_eq!(
            grade_for("Bronze Schrott (ab 1 kg:)"),
            Some(("bronze-rotguss", "Bronze Schrott"))
        );
        assert_eq!(
            grade_for("Bronze Späne (ab 1 kg:)"),
            Some(("bronze-rotguss", "Bronze Späne"))
        );
        assert_eq!(
            grade_for("Rotguss Späne (ab 1 kg:)"),
            Some(("bronze-rotguss", "Rotguss Späne"))
        );
        assert_eq!(
            grade_for("Rotguss Stücke (ab 1 kg:)"),
            Some(("bronze-rotguss", "Rotguss Stücke"))
        );
        assert_eq!(
            grade_for("Alu Bleche blank (ab 1 kg:)"),
            Some(("aluminium-blech", "Bleche blank"))
        );
        assert_eq!(
            grade_for("Alu Bleche Farbe (ab 1 kg:)"),
            Some(("aluminium-blech", "Bleche Farbe"))
        );
        assert_eq!(
            grade_for("Alu Geschirr (ab 1 kg:)"),
            Some(("aluminium-blech", "Geschirr"))
        );
        assert_eq!(
            grade_for("Alu Felgen (ab 1 kg:)"),
            Some(("aluminium-guss", "Felgen"))
        );
        assert_eq!(
            grade_for("Alu Guss (ab 1 kg:)"),
            Some(("aluminium-guss", "Guss"))
        );
        assert_eq!(
            grade_for("Alu Profile blank (ab 1 kg:)"),
            Some(("aluminium-profile", "Profile blank"))
        );
        assert_eq!(
            grade_for("Alu Profile iso (ab 1 kg:)"),
            Some(("aluminium-profile", "Profile iso"))
        );
        assert_eq!(
            grade_for("Alu Späne (ab 1 kg:)"),
            Some(("aluminium-gemischt", "Späne"))
        );
        assert_eq!(grade_for("Altblei (ab 1 kg:)"), Some(("blei", "")));
        assert_eq!(
            grade_for("V2A Edelstahl (ab 1 kg:)"),
            Some(("edelstahl-v2a", ""))
        );
        assert_eq!(
            grade_for("V2A Edelstahl Späne (ab 1 kg:)"),
            Some(("edelstahl-v2a", "Späne"))
        );
        assert_eq!(
            grade_for("V4A Edelstahl (ab 1 kg:)"),
            Some(("edelstahl-v4a", ""))
        );
        assert_eq!(
            grade_for("V4A Edelstahl Späne (ab 1 kg:)"),
            Some(("edelstahl-v4a", "Späne"))
        );
        assert_eq!(
            grade_for("Elektromotoren (ab 1 kg:)"),
            Some(("elektromotoren", ""))
        );
        assert_eq!(
            grade_for("Hartmetall Bohrer/Fräser/Plättchen (ab 1 kg:)"),
            Some(("hartmetall", "Bohrer/Fräser/Plättchen"))
        );
        assert_eq!(
            grade_for("Hartmetall Stückschrott (ab 1 kg:)"),
            Some(("hartmetall", "Stückschrott"))
        );
        assert_eq!(
            grade_for("Bremsscheiben (ab 100 kg:)"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(
            grade_for("Gusseisen (ab 100 kg:)"),
            Some(("eisenschrott-gussbruch", "Gusseisen"))
        );
        assert_eq!(
            grade_for("Mischschrott (ab 100 kg:)"),
            Some(("mischschrott", ""))
        );
        assert_eq!(
            grade_for("Mischschrott schwer (ab 100 kg:)"),
            Some(("mischschrott", "schwer"))
        );
        assert_eq!(
            grade_for("Schredderschrott (ab 100 kg:)"),
            Some(("stahlschrott-shredder", ""))
        );
        assert_eq!(
            grade_for("Zinn Geschirr (ab 1 kg:)"),
            Some(("zinn-geschirr", "Geschirr"))
        );
        assert_eq!(
            grade_for("Zinkblech neu und alt (ab 1 kg:)"),
            Some(("zink", "Blech neu und alt"))
        );
        assert_eq!(grade_for("Zinkguss (ab 1 kg:)"), Some(("zink", "Guss")));
        // Lauter Skip: Mischmetall, Doppel-Material, kein Katalogmaterial.
        assert_eq!(grade_for("Alu Cu Kühler (ab 1 kg:)"), None);
        assert_eq!(grade_for("Stahl- und Gussspäne (ab 100 kg:)"), None);
        assert_eq!(grade_for("Messer versilbert (ab 1 kg:)"), None);
        assert_eq!(grade_for("Silberbesteck 800 (ab 1 kg:)"), None);
        assert_eq!(
            grade_for("Versilbertes Besteck gestempelt 60/40/20 (ab 1 kg:)"),
            None
        );
        assert_eq!(
            grade_for("Versilbertes Besteck gestempelt 80/90/100 (ab 1 kg:)"),
            None
        );
        assert_eq!(grade_for("Inconell (ab 1 kg:)"), None);
        assert_eq!(grade_for("Nickel (ab 1 kg:)"), None);
        assert_eq!(grade_for("Indium (ab 1 kg:)"), None);
        assert_eq!(grade_for("Wolfram (ab 1 kg:)"), None);
        assert_eq!(grade_for("Gold und silberhaltige Abfälle (ab 1 kg:)"), None);
        assert_eq!(grade_for("Palladium (ab 1 kg:)"), None);
    }
}
