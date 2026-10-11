//! MKM complete typed Article→transaction bridge. No global Legacy remapping.
use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, Utc};
use schrott_mcp_store::{NewPrice, PublicDb};

use super::{
    overview::{quote_cell, Snapshot, GROUPS, MAPPINGS},
    SLUG, URL,
};
use crate::traders::{RecordSummary, MAP_VERSION};
use crate::IngestError;

const OWNER: &str = "mkm-article-v1";
// Only these ten canonical identities are independently evidenced in the
// pre-typed read-only Current fixture. Not all 60 newly mapped materials are
// automatically legacy retirement candidates. Exact key + label + purchase
// contract are checked below; Store additionally protects source/producer/time.
const PROVEN_LEGACY_ARTICLES: &[&str] = &[
    "MET-KU-MILL",
    "MKM-CU-03",
    "SW10013",
    "SW10014",
    "MET-KU-RAFF",
    "MKM-CU-04",
    "MET-KA-70",
    "MET-KA-60",
    "SW10015",
    "SW10016",
];
fn fail(detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.into(),
    }
}

/// Revalidate the full typed contract at the write boundary, not a mutable
/// `complete=true` flag or a matching count. All fallible resolution is before
/// the Store transaction; requests/metadata/quotes/retirement commit together.
pub fn record_articles(
    public: &PublicDb,
    snapshot: &Snapshot,
    now: &DateTime<Utc>,
) -> Result<RecordSummary, IngestError> {
    let out = &snapshot.outcome;
    if !snapshot.diagnostics.is_empty()
        || !out.skipped_labels.is_empty()
        || out.fetch_url != URL
        || out.status_code != 200
        || !out.website_alive
        || out.published_at.is_some()
        || snapshot.articles.len() != MAPPINGS.len()
        || out.trader_info.street != "Industriestraße 4"
        || out.trader_info.postcode != "67063"
    {
        return Err(fail("incomplete or invalid MKM source contract"));
    }
    if snapshot
        .page_change_date
        .as_ref()
        .is_some_and(|d| DateTime::parse_from_rfc3339(d).is_err())
    {
        return Err(fail("invalid page change evidence"));
    }
    let tid = public
        .find_trader_id(SLUG)
        .map_err(|e| fail(e.to_string()))?
        .ok_or_else(|| fail("MKM trader missing"))?;
    let mut seen = BTreeSet::new();
    let mut request_count = 0;
    let mut metadata = BTreeMap::new();
    let mut catalog_units = BTreeMap::new();
    let mut accepted: BTreeMap<i64, Vec<serde_json::Value>> = BTreeMap::new();
    for article in &snapshot.articles {
        let m = MAPPINGS
            .iter()
            .find(|m| m.0 == article.article)
            .ok_or_else(|| fail("unknown MKM article"))?;
        let variant = m.4.unwrap_or(m.1);
        if !seen.insert(article.article.as_str())
            || article.label != m.1
            || article.group != GROUPS[m.5]
            || article.url != format!("https://mkm-metals.de/metallankauf/produkt/{}", m.2)
            || article.material != Some(m.3)
            || article.variant != variant
        {
            return Err(fail(format!(
                "changed or duplicate article identity: {}",
                article.article
            )));
        }
        if article
            .product_change_date
            .as_ref()
            .is_some_and(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").is_err())
        {
            return Err(fail("invalid product change evidence"));
        }
        if article.request_only {
            request_count += 1;
            if article.price_cell != "auf Anfrage"
                || article.price_eur_per_kg.is_some()
                || article.minimum_for_upper_bound_kg.is_some()
                || !article.tiers.is_empty()
            {
                return Err(fail("request article with numeric contract"));
            }
            let matching: Vec<_> = out
                .acceptances
                .iter()
                .filter(|a| a.material == m.3 && a.label == article.label)
                .collect();
            if matching.len() != 1
                || matching[0].conditions
                    != format!(
                        "{}; auf Anfrage; Artikel {}; {}",
                        article.label, article.article, article.url
                    )
            {
                return Err(fail("request acceptance diverges from article"));
            }
        } else {
            let (price, minimum) =
                quote_cell(&article.price_cell).ok_or_else(|| fail("unknown quote contract"))?;
            if article.price_eur_per_kg != Some(price)
                || article.minimum_for_upper_bound_kg != Some(minimum)
                || article.product_change_date.is_none()
                || article.tiers.first().is_none_or(|t| t.ab != 0.0)
                || article
                    .tiers
                    .last()
                    .is_none_or(|t| t.ab != minimum || (t.preis - price).abs() > 0.000001)
                || article.tiers.iter().any(|t| {
                    !t.ab.is_finite() || t.ab < 0.0 || !t.preis.is_finite() || t.preis <= 0.0
                })
                || article
                    .tiers
                    .windows(2)
                    .any(|t| t[0].ab >= t[1].ab || t[0].preis > t[1].preis)
            {
                return Err(fail("invalid article quote/tier metadata"));
            }
            let matching: Vec<_> = out
                .prices
                .iter()
                .filter(|p| p.material == m.3 && p.variant == variant)
                .collect();
            if matching.len() != 1 {
                return Err(fail("missing/duplicate article price key"));
            }
            let p = matching[0];
            if p.label != article.label
                || p.price != price
                || p.currency != "EUR"
                || p.unit != "EUR/kg"
                || p.price_kind != "upto"
                || p.price_min.is_some()
                || p.price_max != Some(price)
                || p.confidence != Some(0.5)
            {
                return Err(fail("price projection diverges from typed article"));
            }
        }
        let mid = public
            .find_material_id(m.3)
            .map_err(|e| fail(e.to_string()))?
            .ok_or_else(|| fail(format!("catalog material missing: {}", m.3)))?;
        let unit = public
            .material_unit(mid)
            .map_err(|e| fail(e.to_string()))?
            .ok_or_else(|| fail(format!("catalog unit missing: {}", m.3)))?;
        // The shared legacy helper deliberately preserves unsupported units.
        // A complete MKM snapshot must instead fail closed, including its
        // request-only catalog entries, before any quote/acceptance retirement.
        if !matches!(unit.as_str(), "EUR/kg" | "EUR/t") {
            return Err(fail(format!(
                "unsupported MKM catalog unit: {} / {unit}",
                m.3
            )));
        }
        catalog_units.insert(mid, unit);
        let mut data = serde_json::to_value(article).map_err(|e| fail(e.to_string()))?;
        // Relative UI date words are retained in the immutable manifest but
        // don't change the semantic price evidence every midnight.
        data.as_object_mut().unwrap().remove("product_change_text");
        accepted.entry(mid).or_default().push(data.clone());
        metadata.insert((m.3, variant), (mid, data));
    }
    if out.acceptances.len() != request_count || out.prices.len() != seen.len() - request_count {
        return Err(fail("unconsumed prices/acceptances in MKM outcome"));
    }
    let mut articles: Vec<_> = snapshot.articles.iter().collect();
    articles.sort_by_key(|a| &a.article);
    let now_s = now.to_rfc3339_opts(chrono::SecondsFormat::Nanos, false);
    let manifest = serde_json::json!({"owner":OWNER,"source_url":URL,"trader_id":tid,
        "observed_at":now_s,"page_change_date":snapshot.page_change_date,
        "page_change_precision":"calendar_day; UTC midnight is normalization, not source time","articles":articles}).to_string();
    let extras: Vec<_> = out.prices.iter().map(|p| serde_json::json!({"snapshot_owner":OWNER,"map_v":MAP_VERSION,
        "article":metadata[&(p.material,p.variant.as_ref())].1,"page_change_date":snapshot.page_change_date,
        "page_change_precision":"calendar_day; UTC midnight is normalization, not source time",
        "date_semantics":"product/page modification evidence, not price publication"}).to_string()).collect();
    // Only the Store projection is normalized. Source Article, tiers and kg
    // thresholds above remain unchanged in both manifest and price metadata.
    // Reuse the standard kg<->t seam (also scales both bounds); don't change
    // its deliberately permissive Legacy/MSG contract globally.
    let normalized: Vec<_> = out
        .prices
        .iter()
        .map(|p| {
            let mid = metadata[&(p.material, p.variant.as_ref())].0;
            let result = crate::traders::normalize_unit(
                public,
                mid,
                p.price,
                p.price_min,
                p.price_max,
                p.unit,
            )?;
            if result.3 != catalog_units[&mid]
                || !result.0.is_finite()
                || result.1.is_some_and(|v| !v.is_finite())
                || result.2.is_some_and(|v| !v.is_finite())
            {
                return Err(fail(format!(
                    "unproven/nonfinite catalog normalization: {}",
                    p.material
                )));
            }
            Ok(result)
        })
        .collect::<Result<Vec<_>, IngestError>>()?;
    let prices: Vec<_> = out
        .prices
        .iter()
        .zip(&extras)
        .zip(&normalized)
        .map(|((p, extra), normalized)| NewPrice {
            trader_id: tid,
            material_id: metadata[&(p.material, p.variant.as_ref())].0,
            variant: p.variant.as_ref(),
            price: normalized.0,
            currency: p.currency,
            unit: &normalized.3,
            price_kind: p.price_kind,
            price_min: normalized.1,
            price_max: normalized.2,
            confidence: p.confidence,
            source_type: "haendler_angabe",
            published: true,
            source_url: URL,
            observed_at: &now_s,
            published_at: None,
            valid_from: None,
            valid_to: None,
            notes: &p.label,
            extra_json: extra,
            ingested_at: &now_s,
        })
        .collect();
    let copper_cable = public
        .find_material_id("kabel-kupfer")
        .map_err(|e| fail(e.to_string()))?;
    let berry = public
        .find_material_id("kupfer-berry")
        .map_err(|e| fail(e.to_string()))?;
    let legacy: Vec<_> = public
        .current_prices_from(tid, URL)
        .map_err(|e| fail(e.to_string()))?
        .iter()
        .filter(|p| {
            p.source_type == "haendler_angabe"
                && p.published
                && p.currency == "EUR"
                && p.unit == "EUR/kg"
                && p.price_kind == "upto"
                && p.price > 0.0
                && p.price.is_finite()
                && p.price_min.is_none()
                && p.price_max == Some(p.price)
                && p.confidence == Some(0.5)
                && p.valid_from.is_none()
                && p.valid_to.is_none()
                && ((Some(p.material_id) == copper_cable
                    && p.variant == "Stecker 30%"
                    && [
                        "Cu Steckerkabel (30%Cu)",
                        "Kupfer Steckerkabel (30% Kupfer)",
                    ]
                    .contains(&p.notes.as_str()))
                    || (Some(p.material_id) == berry
                        && p.variant == "Kerze"
                        && p.notes == "Kupfer Kerze")
                    || metadata.iter().any(|((_, variant), (mid, data))| {
                        *mid == p.material_id
                            && *variant == p.variant
                            && data["request_only"] == true
                            && data["label"] == p.notes
                            && data["article"]
                                .as_str()
                                .is_some_and(|id| PROVEN_LEGACY_ARTICLES.contains(&id))
                    }))
        })
        .map(|p| p.id)
        .collect();
    let acceptances: Vec<_> = accepted
        .into_iter()
        .map(|(mid, mut a)| {
            a.sort_by_key(|v| v["article"].as_str().unwrap_or("").to_owned());
            (mid, serde_json::to_string(&a).expect("JSON values"))
        })
        .collect();
    public
        .record_article_price_snapshot(&prices, &legacy, &manifest, &acceptances)
        .map_err(|e| fail(e.to_string()))?;
    Ok(RecordSummary {
        recorded: prices.len() as i64,
        accepted: request_count,
        skipped: vec![],
        canaries: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use schrott_mcp_store::{InternalDb, PriceRow};
    const HTML: &str = include_str!("../fixtures/mkm-overview-20261010.html");
    const IMP: &str = include_str!("../fixtures/mkm-impressum-20261010.html");
    fn fixture(html: &str) -> Snapshot {
        super::super::overview::parse(html, IMP, 200).unwrap()
    }
    fn setup(
        name: &str,
    ) -> (
        crate::test_support::TempDbDir,
        PublicDb,
        InternalDb,
        DateTime<Utc>,
    ) {
        let dir = crate::test_support::TempDbDir::new(name);
        let db = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&db).unwrap();
        let now = DateTime::parse_from_rfc3339("2026-10-11T01:00:00+00:00")
            .unwrap()
            .with_timezone(&Utc);
        (dir, db, internal, now)
    }
    async fn commit(
        db: &PublicDb,
        internal: &InternalDb,
        s: Snapshot,
        at: DateTime<Utc>,
    ) -> Result<RecordSummary, IngestError> {
        crate::traders::record_outcome(
            db,
            internal,
            SLUG,
            &crate::traders::RecordOutcome::MkmArticles(s),
            &at,
        )
        .await
    }
    fn current(db: &PublicDb, material: &str, variant: &str) -> Option<PriceRow> {
        db.current_price_for(
            db.find_trader_id(SLUG).unwrap().unwrap(),
            db.find_material_id(material).unwrap().unwrap(),
            variant,
        )
        .unwrap()
    }
    fn change_first_row(html: &str, request: bool, quantity: Option<u64>) -> String {
        change_row(html, "MET-KU-MILL", request, quantity)
    }
    fn change_row(html: &str, id: &str, request: bool, quantity: Option<u64>) -> String {
        let article = html
            .find(&format!("<td class=\"muted small\">{id}</td>"))
            .unwrap();
        let start = html[..article].rfind("<tr>").unwrap();
        let end = article + html[article..].find("</tr>").unwrap() + 5;
        let mut row = html[start..end].to_owned();
        if request {
            let td_start = row.find("<td class=\"num\">").unwrap();
            let td_end = td_start + row[td_start..].find("</td>").unwrap() + 5;
            row.replace_range(td_start..td_end, "<td class=\"num\">auf Anfrage</td>");
            let fourth = row.rfind("<td style=\"min-width:").unwrap();
            let fourth_end = fourth + row[fourth..].find("</td>").unwrap() + 5;
            row.replace_range(fourth..fourth_end, "<td></td>");
        } else if let Some(q) = quantity {
            row = row
                .replace("ab 250 kg</span>", &format!("ab {q} kg</span>"))
                .replace("&quot;ab&quot;:250", &format!("&quot;ab&quot;:{q}"));
        }
        format!("{}{}{}", &html[..start], row, &html[end..])
    }
    fn replay_readonly(db: &PublicDb) -> Vec<PriceRow> {
        let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!(
            "../fixtures/mkm-legacy-current-readonly-20261011.json"
        ))
        .unwrap();
        assert_eq!(rows.len(), 12);
        let tid = db.find_trader_id(SLUG).unwrap().unwrap();
        let acceptances: Vec<serde_json::Value> = serde_json::from_str(include_str!(
            "../fixtures/mkm-legacy-acceptances-readonly-20261011.json"
        ))
        .unwrap();
        assert_eq!(acceptances.len(), 5);
        for a in acceptances {
            let mid = db
                .find_material_id(a["material_slug"].as_str().unwrap())
                .unwrap()
                .unwrap();
            db.set_acceptance(
                tid,
                mid,
                a["accepts"].as_i64() == Some(1),
                a["conditions"].as_str().unwrap(),
                a["valid_from"].as_str(),
                a["valid_to"].as_str(),
                a["observed_at"].as_str().unwrap(),
                a["updated_at"].as_str().unwrap(),
            )
            .unwrap();
        }
        for row in rows {
            let slug = row["material_slug"].as_str().unwrap();
            let mid = db.find_material_id(slug).unwrap().unwrap();
            let s = |k: &str| row[k].as_str().unwrap();
            let optional = |k: &str| row[k].as_str();
            db.record_price(&NewPrice {
                trader_id: tid,
                material_id: mid,
                variant: s("variant"),
                price: row["price"].as_f64().unwrap(),
                currency: s("currency"),
                unit: s("unit"),
                price_kind: s("price_kind"),
                price_min: row["price_min"].as_f64(),
                price_max: row["price_max"].as_f64(),
                confidence: row["confidence"].as_f64(),
                source_type: s("source_type"),
                published: row["published"].as_i64() == Some(1),
                source_url: s("source_url"),
                observed_at: s("observed_at"),
                published_at: optional("published_at"),
                valid_from: optional("valid_from"),
                valid_to: optional("valid_to"),
                notes: s("notes"),
                extra_json: s("extra_json"),
                ingested_at: s("ingested_at"),
            })
            .unwrap();
        }
        db.current_prices_from(tid, URL).unwrap()
    }

    #[tokio::test]
    async fn readonly_legacy_alias_replay_preserves_history_and_matches_independent_oracle() {
        let (_dir, db, internal, now) = setup("mkm-alias-replay");
        let old = replay_readonly(&db);
        let oracle: serde_json::Value = serde_json::from_str(include_str!(
            "../fixtures/mkm-reviewed-mapping-oracle-20261011.json"
        ))
        .unwrap();
        let s = fixture(HTML);
        commit(&db, &internal, s, now).await.unwrap();
        assert!(current(&db, "kabel-kupfer", "Stecker 30%").is_none());
        assert!(current(&db, "kupfer-berry", "Kerze").is_none());
        assert_eq!(
            db.current_prices_from(old[0].trader_id, URL).unwrap().len(),
            55
        );
        for a in fixture(HTML).articles {
            let reviewed = &oracle["expected"][&a.article];
            assert_eq!(a.material, reviewed[0].as_str());
            assert_eq!(a.variant, reviewed[1].as_str().unwrap());
            let persisted = current(&db, a.material.unwrap(), &a.variant);
            if a.request_only {
                assert!(persisted.is_none());
            } else {
                let p = persisted.unwrap();
                let metadata: serde_json::Value = serde_json::from_str(&p.extra_json).unwrap();
                assert_eq!(metadata["article"]["article"], a.article);
                assert_eq!(metadata["article"]["url"], a.url);
                assert_eq!(p.published_at, None);
            }
        }
        for before in old {
            let history = db
                .price_history(before.trader_id, before.material_id, &before.variant, 10)
                .unwrap();
            let after = history.iter().find(|p| p.id == before.id).unwrap();
            assert_eq!(
                format!("{before:?}"),
                format!("{after:?}"),
                "all captured historical fields immutable"
            );
        }
    }

    #[tokio::test]
    async fn request_transition_and_metadata_changes_are_atomic_immutable_and_idempotent() {
        let (_dir, db, internal, now) = setup("mkm-request-metadata");
        commit(&db, &internal, fixture(HTML), now).await.unwrap();
        let before = current(&db, "kupfer-millberry", "").unwrap();
        let changed = change_first_row(HTML, false, Some(500));
        commit(
            &db,
            &internal,
            fixture(&changed),
            now + chrono::Duration::minutes(1),
        )
        .await
        .unwrap();
        let condition = current(&db, "kupfer-millberry", "").unwrap();
        assert_ne!(condition.id, before.id);
        assert_eq!(condition.price, before.price);
        let extra: serde_json::Value = serde_json::from_str(&condition.extra_json).unwrap();
        assert_eq!(extra["article"]["minimum_for_upper_bound_kg"], 500.0);
        commit(
            &db,
            &internal,
            fixture(&changed),
            now + chrono::Duration::minutes(2),
        )
        .await
        .unwrap();
        assert_eq!(
            current(&db, "kupfer-millberry", "").unwrap().id,
            condition.id
        );
        let tid = before.trader_id;
        let mid = before.material_id;
        assert_eq!(db.price_history(tid, mid, "", 10).unwrap().len(), 2);
        let requested = change_first_row(&changed, true, None);
        let summary = commit(
            &db,
            &internal,
            fixture(&requested),
            now + chrono::Duration::minutes(3),
        )
        .await
        .unwrap();
        assert_eq!((summary.recorded, summary.accepted), (54, 6));
        assert!(current(&db, "kupfer-millberry", "").is_none());
        assert!(db
            .existing_acceptance(tid, mid)
            .unwrap()
            .unwrap()
            .1
            .contains("auf Anfrage"));
        assert_eq!(db.price_history(tid, mid, "", 10).unwrap().len(), 2);
        assert!(
            commit(
                &db,
                &internal,
                fixture(&changed),
                now + chrono::Duration::minutes(2)
            )
            .await
            .is_err(),
            "late snapshot after request cannot resurrect price"
        );
        commit(
            &db,
            &internal,
            fixture(&requested),
            now + chrono::Duration::minutes(3),
        )
        .await
        .unwrap();
        assert!(
            commit(
                &db,
                &internal,
                fixture(HTML),
                now + chrono::Duration::minutes(3)
            )
            .await
            .is_err(),
            "same timestamp conflicting manifest"
        );
        let historical = db.price_history(tid, mid, "", 10).unwrap();
        assert_eq!(historical[1].extra_json, before.extra_json);
        assert_eq!(historical[1].observed_at, before.observed_at);
    }

    #[tokio::test]
    async fn complete_all_request_snapshot_retires_owned_quotes_without_phantom_prices() {
        let (_dir, db, internal, now) = setup("mkm-all-request");
        let original = fixture(HTML);
        commit(&db, &internal, fixture(HTML), now).await.unwrap();
        let mut html = HTML.to_owned();
        for article in original.articles.iter().filter(|a| !a.request_only) {
            html = change_row(&html, &article.article, true, None);
        }
        let requests = fixture(&html);
        assert!(
            requests.diagnostics.is_empty(),
            "{:?}",
            requests.diagnostics
        );
        assert_eq!(
            (
                requests.outcome.prices.len(),
                requests.outcome.acceptances.len()
            ),
            (0, 60)
        );
        let result = commit(&db, &internal, requests, now + chrono::Duration::minutes(1))
            .await
            .unwrap();
        assert_eq!((result.recorded, result.accepted), (0, 60));
        let tid = db.find_trader_id(SLUG).unwrap().unwrap();
        assert!(db.current_prices_from(tid, URL).unwrap().is_empty());
        for article in original.articles.iter().filter(|a| !a.request_only) {
            let mid = db
                .find_material_id(article.material.unwrap())
                .unwrap()
                .unwrap();
            assert_eq!(
                db.price_history(tid, mid, &article.variant, 10)
                    .unwrap()
                    .len(),
                1
            );
        }
        assert!(commit(&db, &internal, fixture(HTML), now).await.is_err());
    }

    #[tokio::test]
    async fn changed_product_and_page_dates_append_evidence_without_publication_invention() {
        let (_dir, db, internal, now) = setup("mkm-date-evidence");
        commit(&db, &internal, fixture(HTML), now).await.unwrap();
        let before = current(&db, "kupfer-millberry", "").unwrap();
        let html = HTML
            .replacen("datetime=\"2026-09-28\"", "datetime=\"2026-09-29\"", 1)
            .replace("zuletzt geändert 09.10.2026", "zuletzt geändert 10.10.2026");
        commit(
            &db,
            &internal,
            fixture(&html),
            now + chrono::Duration::minutes(1),
        )
        .await
        .unwrap();
        let after = current(&db, "kupfer-millberry", "").unwrap();
        assert_ne!(after.id, before.id);
        assert_eq!(after.price, before.price);
        let extra: serde_json::Value = serde_json::from_str(&after.extra_json).unwrap();
        assert_eq!(extra["article"]["product_change_date"], "2026-09-29");
        assert_eq!(extra["page_change_date"], "2026-10-10T00:00:00+00:00");
        assert!(extra["page_change_precision"]
            .as_str()
            .unwrap()
            .contains("not source time"));
        assert_eq!(after.published_at, None);
        let history = db
            .price_history(before.trader_id, before.material_id, "", 10)
            .unwrap();
        assert_eq!(history[1].extra_json, before.extra_json);
    }

    #[tokio::test]
    async fn aliases_preserve_foreign_source_producer_and_unknown_old_keys() {
        for (source, extra) in [
            ("https://foreign.test/", "{}"),
            (URL, "{\"snapshot_owner\":\"foreign-producer\"}"),
        ] {
            let (_dir, db, internal, now) = setup("mkm-foreign-alias");
            let captured = replay_readonly(&db);
            let alias = captured.iter().find(|p| p.variant == "Kerze").unwrap();
            let protected = db
                .record_price(&NewPrice {
                    trader_id: alias.trader_id,
                    material_id: alias.material_id,
                    variant: &alias.variant,
                    price: alias.price + 0.01,
                    currency: &alias.currency,
                    unit: &alias.unit,
                    price_kind: &alias.price_kind,
                    price_min: alias.price_min,
                    price_max: Some(alias.price + 0.01),
                    confidence: alias.confidence,
                    source_type: &alias.source_type,
                    published: alias.published,
                    source_url: source,
                    observed_at: "2026-10-11T00:00:00Z",
                    published_at: None,
                    valid_from: None,
                    valid_to: None,
                    notes: &alias.notes,
                    extra_json: extra,
                    ingested_at: "2026-10-11T00:00:00Z",
                })
                .unwrap();
            commit(&db, &internal, fixture(HTML), now).await.unwrap();
            assert_eq!(current(&db, "kupfer-berry", "Kerze").unwrap().id, protected);
            assert!(current(&db, "kupfer-stanzabfaelle", "Kerze").is_some());
            assert_eq!(
                db.price_history(alias.trader_id, alias.material_id, "Kerze", 10)
                    .unwrap()
                    .len(),
                2
            );
        }
    }

    #[tokio::test]
    async fn foreign_current_collision_rolls_back_every_article_and_alias() {
        let (_dir, db, internal, now) = setup("mkm-foreign-collision");
        let captured = replay_readonly(&db);
        let old = current(&db, "kupfer-millberry", "").unwrap();
        let foreign = db
            .record_price(&NewPrice {
                trader_id: old.trader_id,
                material_id: old.material_id,
                variant: "",
                price: 1.0,
                currency: "EUR",
                unit: "EUR/kg",
                price_kind: "upto",
                price_min: None,
                price_max: Some(1.0),
                confidence: Some(0.5),
                source_type: "haendler_angabe",
                published: true,
                source_url: "https://foreign.test/",
                observed_at: "2026-10-11T00:00:00Z",
                published_at: None,
                valid_from: None,
                valid_to: None,
                notes: "foreign",
                extra_json: "{}",
                ingested_at: "2026-10-11T00:00:00Z",
            })
            .unwrap();
        assert!(commit(&db, &internal, fixture(HTML), now).await.is_err());
        assert_eq!(current(&db, "kupfer-millberry", "").unwrap().id, foreign);
        assert!(current(&db, "kupfer-berry", "Kerze").is_some());
        assert!(current(&db, "kabel-kupfer", "Stecker 30%").is_some());
        assert_eq!(
            db.current_prices_from(old.trader_id, URL).unwrap().len(),
            captured.len() - 1
        );
        assert!(current(&db, "zinn-rein-formgemischt", "Zinn 99%").is_none());
    }

    #[tokio::test]
    async fn newer_or_foreign_acceptance_evidence_blocks_atomic_snapshot() {
        for owned in [false, true] {
            let (_dir, db, internal, now) = setup("mkm-acceptance-conflict");
            replay_readonly(&db);
            if owned {
                commit(&db, &internal, fixture(HTML), now).await.unwrap();
            }
            let tid = db.find_trader_id(SLUG).unwrap().unwrap();
            let mid = db.find_material_id("kupfer-millberry").unwrap().unwrap();
            let at = if owned {
                "2026-10-11T01:00:30.000000000+00:00"
            } else {
                "2026-10-12T01:00:00.000000000+00:00"
            };
            let conditions = if owned {
                "foreign acceptance writer"
            } else {
                ""
            };
            db.set_acceptance(tid, mid, true, conditions, None, None, at, at)
                .unwrap();
            let before = current(&db, "kupfer-millberry", "").unwrap();
            assert!(commit(
                &db,
                &internal,
                fixture(&change_first_row(HTML, false, Some(500))),
                now + chrono::Duration::minutes(1)
            )
            .await
            .is_err());
            assert_eq!(current(&db, "kupfer-millberry", "").unwrap().id, before.id);
            assert_eq!(
                db.existing_acceptance(tid, mid).unwrap().unwrap().1,
                conditions
            );
        }
    }

    #[tokio::test]
    async fn same_legacy_key_under_another_trader_is_untouched() {
        let (_dir, db, internal, now) = setup("mkm-foreign-trader");
        let rows = replay_readonly(&db);
        let alias = rows.iter().find(|p| p.variant == "Kerze").unwrap();
        let tid = db
            .find_trader_id(crate::traders::handlers::msg_metallrecycling_gotha::SLUG)
            .unwrap()
            .unwrap();
        let id = db
            .record_price(&NewPrice {
                trader_id: tid,
                material_id: alias.material_id,
                variant: &alias.variant,
                price: alias.price,
                currency: &alias.currency,
                unit: &alias.unit,
                price_kind: &alias.price_kind,
                price_min: alias.price_min,
                price_max: alias.price_max,
                confidence: alias.confidence,
                source_type: &alias.source_type,
                published: alias.published,
                source_url: URL,
                observed_at: &alias.observed_at,
                published_at: alias.published_at.as_deref(),
                valid_from: None,
                valid_to: None,
                notes: &alias.notes,
                extra_json: &alias.extra_json,
                ingested_at: &alias.ingested_at,
            })
            .unwrap();
        let before = db
            .current_price_for(tid, alias.material_id, "Kerze")
            .unwrap()
            .unwrap();
        commit(&db, &internal, fixture(HTML), now).await.unwrap();
        let after = db
            .current_price_for(tid, alias.material_id, "Kerze")
            .unwrap()
            .unwrap();
        assert_eq!(after.id, id);
        assert_eq!(format!("{before:?}"), format!("{after:?}"));
        assert!(current(&db, "kupfer-berry", "Kerze").is_none());
    }

    #[tokio::test]
    async fn review_f6_unsupported_catalog_unit_blocks_all_snapshot_writes() {
        for (slug, unit) in [
            ("kupfer-millberry", "EUR/Stk"),
            ("schrott-eisen-ne-gemischt", "USD/t"),
            ("mischschrott", ""),
        ] {
            let (_dir, db, internal, now) = setup("mkm-unsupported-unit");
            replay_readonly(&db);
            let m = db
                .list_materials()
                .unwrap()
                .into_iter()
                .find(|m| m.slug == slug)
                .unwrap();
            // Mutate only this isolated test DB, never the real catalog source.
            db.upsert_material(&schrott_mcp_store::NewMaterial {
                slug: &m.slug,
                name_de: &m.name_de,
                category: &m.category,
                unit,
                description: &m.description,
                updated_at: &m.updated_at,
            })
            .unwrap();
            let before: Vec<_> = [
                "prices",
                "current_prices",
                "trader_materials",
                "source_article_snapshots",
                "article_acceptance_owners",
            ]
            .iter()
            .map(|t| {
                db.query_sql(&format!("SELECT * FROM {t} ORDER BY rowid"))
                    .unwrap()
                    .rows
            })
            .collect();
            assert!(
                commit(&db, &internal, fixture(HTML), now).await.is_err(),
                "unsupported catalogunit {slug}/{unit} accepted"
            );
            let after: Vec<_> = [
                "prices",
                "current_prices",
                "trader_materials",
                "source_article_snapshots",
                "article_acceptance_owners",
            ]
            .iter()
            .map(|t| {
                db.query_sql(&format!("SELECT * FROM {t} ORDER BY rowid"))
                    .unwrap()
                    .rows
            })
            .collect();
            assert_eq!(
                after, before,
                "unsupported unit caused partial write/retirement"
            );
        }
    }

    #[tokio::test]
    async fn review_f6_future_steel_quotes_normalize_catalog_without_changing_source_contract() {
        let (_dir, db, internal, now) = setup("mkm-catalog-unit-future");
        commit(&db, &internal, fixture(HTML), now).await.unwrap();
        let mut html = HTML.to_owned();
        let source = fixture(HTML)
            .articles
            .into_iter()
            .find(|a| a.article == "MET-KU-MILL")
            .unwrap();
        for id in ["MKM-SCH-01", "MKM-SCH-02"] {
            html = numeric_request_row(&html, id, &source);
        }
        let snapshot = fixture(&html);
        assert!(snapshot.diagnostics.is_empty());
        assert_eq!(
            (
                snapshot.outcome.prices.len(),
                snapshot.outcome.acceptances.len()
            ),
            (57, 3)
        );
        commit(&db, &internal, snapshot, now + chrono::Duration::minutes(1))
            .await
            .unwrap();
        for id in ["MKM-SCH-01", "MKM-SCH-02"] {
            let a = fixture(&html)
                .articles
                .into_iter()
                .find(|a| a.article == id)
                .unwrap();
            let row = current(&db, a.material.unwrap(), &a.variant).unwrap();
            assert_eq!(
                db.material_unit(row.material_id).unwrap().as_deref(),
                Some("EUR/t")
            );
            assert_eq!(row.unit, "EUR/t");
            assert_eq!(row.price, 11130.0);
            assert_eq!(row.price_max, Some(11130.0));
            assert_eq!(row.price_min, None);
            let extra: serde_json::Value = serde_json::from_str(&row.extra_json).unwrap();
            assert_eq!(extra["article"]["price_eur_per_kg"], 11.13);
            assert_eq!(extra["article"]["minimum_for_upper_bound_kg"], 250.0);
            assert_eq!(
                extra["article"]["tiers"],
                serde_json::to_value(&source.tiers).unwrap()
            );
            assert_eq!(extra["article"]["price_cell"], source.price_cell);
            assert_eq!(row.published_at, None);
        }
        // Every currently mapped material is checked, not just the two changed
        // rows. The source Outcome remains €/kg while every stored quote uses
        // its actual catalog unit.
        for a in fixture(&html).articles {
            let mid = db.find_material_id(a.material.unwrap()).unwrap().unwrap();
            let unit = db.material_unit(mid).unwrap().unwrap();
            assert!(matches!(unit.as_str(), "EUR/kg" | "EUR/t"));
            let reviewed_unit =
                if ["mischschrott", "schrott-eisen-ne-gemischt"].contains(&a.material.unwrap()) {
                    "EUR/t"
                } else {
                    "EUR/kg"
                };
            assert_eq!(
                unit, reviewed_unit,
                "fixed reviewed catalog-unit expectation for {}",
                a.article
            );
            if !a.request_only {
                let row = current(&db, a.material.unwrap(), &a.variant).unwrap();
                assert_eq!(row.unit, unit);
                let factor = if unit == "EUR/t" { 1000.0 } else { 1.0 };
                assert_eq!(row.price, a.price_eur_per_kg.unwrap() * factor);
                assert_eq!(row.price_max, Some(a.price_eur_per_kg.unwrap() * factor));
            }
        }
    }

    #[test]
    fn review_f6_shared_conversion_scales_both_price_bounds() {
        let (_dir, db, _internal, _now) = setup("mkm-catalog-bound-conversion");
        let mid = db
            .find_material_id("schrott-eisen-ne-gemischt")
            .unwrap()
            .unwrap();
        // Today's upto contract rejects a source price_min. This proves the
        // reused seam also scales a lower bound if a future reviewed source
        // contract legitimately transports one; it does not activate ranges.
        let normalized =
            crate::traders::normalize_unit(&db, mid, 11.13, Some(10.6), Some(11.13), "EUR/kg")
                .unwrap();
        assert_eq!(
            normalized,
            (11130.0, Some(10600.0), Some(11130.0), "EUR/t".to_owned())
        );
    }

    fn numeric_request_row(
        html: &str,
        id: &str,
        source: &super::super::overview::Article,
    ) -> String {
        let pos = html
            .find(&format!("<td class=\"muted small\">{id}</td>"))
            .unwrap();
        let start = pos + html[pos..].find("</td>").unwrap() + 5;
        let end = start + html[start..].find("</tr>").unwrap();
        let tiers = serde_json::to_string(&source.tiers)
            .unwrap()
            .replace('"', "&quot;");
        let cells=format!("<td class=\"num\"><strong>11,13 €</strong> /kg ab 250 kg</td><td><time datetime=\"2026-09-28\">28.09.2026</time></td><td><input data-price=\"10.6\" data-staffel=\"{tiers}\"></td>");
        format!("{}{}{}", &html[..start], cells, &html[end..])
    }

    #[tokio::test]
    async fn review_f1_first_request_cutover_retires_all_proven_legacy_quotes() {
        let (_dir, db, internal, now) = setup("mkm-first-request-cutover");
        let old = replay_readonly(&db);
        let mut html = HTML.to_owned();
        for a in fixture(HTML).articles.iter().filter(|a| !a.request_only) {
            html = change_row(&html, &a.article, true, None);
        }
        let snapshot = fixture(&html);
        assert!(snapshot.diagnostics.is_empty());
        let result = commit(&db, &internal, snapshot, now).await.unwrap();
        assert_eq!((result.recorded, result.accepted), (0, 60));
        assert!(db.current_prices_from(old[0].trader_id,URL).unwrap().is_empty(),"first typed import must retire all 12 proven legacy keys, without prior typed numeric import");
        for before in old {
            let history = db
                .price_history(before.trader_id, before.material_id, &before.variant, 10)
                .unwrap();
            assert_eq!(
                format!("{:?}", history.iter().find(|p| p.id == before.id).unwrap()),
                format!("{before:?}")
            );
        }
    }

    #[tokio::test]
    async fn review_f2_real_typed_history_is_protected_from_public_legacy_writers() {
        let (_dir, db, internal, now) = setup("mkm-review-existing-owner");
        commit(&db, &internal, fixture(HTML), now).await.unwrap();
        let before = current(&db, "kupfer-millberry", "").unwrap();
        for extra in ["{}", "{\"snapshot_owner\":\"foreign-producer\"}"] {
            let p = NewPrice {
                trader_id: before.trader_id,
                material_id: before.material_id,
                variant: "",
                price: before.price,
                currency: &before.currency,
                unit: &before.unit,
                price_kind: &before.price_kind,
                price_min: before.price_min,
                price_max: before.price_max,
                confidence: before.confidence,
                source_type: &before.source_type,
                published: before.published,
                source_url: &before.source_url,
                observed_at: "2026-10-12T00:00:00+00:00",
                published_at: Some("2026-10-12T00:00:00+00:00"),
                valid_from: None,
                valid_to: None,
                notes: "foreign replacement label",
                extra_json: extra,
                ingested_at: "2026-10-12T00:00:00+00:00",
            };
            assert!(db.record_price(&p).is_err());
            assert!(db
                .record_price_snapshot(&[p], &[], "foreign-producer")
                .is_err());
            let after = current(&db, "kupfer-millberry", "").unwrap();
            assert_eq!(format!("{after:?}"), format!("{before:?}"));
            assert_eq!(
                db.price_history(before.trader_id, before.material_id, "", 10)
                    .unwrap()
                    .len(),
                1
            );
        }
    }

    #[tokio::test]
    async fn review_f1_first_request_cutover_preserves_foreign_and_unproven_keys() {
        for (source, extra, notes) in [
            ("https://foreign.test/", "{}", "Kupfer Millberry"),
            (
                URL,
                "{\"snapshot_owner\":\"foreign-producer\"}",
                "Kupfer Millberry",
            ),
            (URL, "{}", "unknown contract label"),
        ] {
            let (_dir, db, internal, now) = setup("mkm-first-request-protection");
            replay_readonly(&db);
            let old = current(&db, "kupfer-millberry", "").unwrap();
            db.record_price(&NewPrice {
                trader_id: old.trader_id,
                material_id: old.material_id,
                variant: "",
                price: 11.14,
                currency: "EUR",
                unit: "EUR/kg",
                price_kind: "upto",
                price_min: None,
                price_max: Some(11.14),
                confidence: Some(0.5),
                source_type: "haendler_angabe",
                published: true,
                source_url: source,
                observed_at: "2026-10-11T00:00:00+00:00",
                published_at: None,
                valid_from: None,
                valid_to: None,
                notes,
                extra_json: extra,
                ingested_at: "2026-10-11T00:00:00+00:00",
            })
            .unwrap();
            let protected = current(&db, "kupfer-millberry", "").unwrap();
            let mut html = HTML.to_owned();
            for a in fixture(HTML).articles.iter().filter(|a| !a.request_only) {
                html = change_row(&html, &a.article, true, None);
            }
            commit(&db, &internal, fixture(&html), now).await.unwrap();
            let after = current(&db, "kupfer-millberry", "").unwrap();
            assert_eq!(format!("{after:?}"), format!("{protected:?}"));
            assert_eq!(
                db.current_prices_from(old.trader_id, URL).unwrap().len(),
                usize::from(source == URL)
            );
        }
    }

    #[tokio::test]
    async fn review_f3_f4_f5_invalid_source_contract_cannot_mutate_any_evidence() {
        let (_dir, db, internal, now) = setup("mkm-review-invalid-contracts");
        replay_readonly(&db);
        let before: Vec<_> = [
            "prices",
            "current_prices",
            "trader_materials",
            "source_article_snapshots",
            "article_acceptance_owners",
        ]
        .iter()
        .map(|t| {
            db.query_sql(&format!("SELECT * FROM {t} ORDER BY rowid"))
                .unwrap()
                .rows
        })
        .collect();
        let mut cases = vec![];
        for hidden in [
            "hidden",
            "style=\"display:none\"",
            "style=\"DISPLAY: none !important\"",
        ] {
            cases.push(HTML.replacen(
                "Kupfer Millberry</a>",
                &format!("Kupfer <span {hidden}>Millberry</span></a>"),
                1,
            ));
            cases.push(HTML.replacen(
                "Kupfer Millberry</a>",
                &format!("Kupfer <span {hidden}><b>Millberry</b></span></a>"),
                1,
            ));
        }
        cases.push(HTML.replace("zuletzt geändert 09.10.2026", "zuletzt geändert 99.99.2026"));
        cases.push(HTML.replace("zuletzt geändert 09.10.2026", "zuletzt geändert unknown"));
        cases.push(HTML.replacen("<span class=\"eyebrow\">Preisliste · zuletzt geändert 09.10.2026</span>","<span class=\"eyebrow\">Preisliste · zuletzt geändert 09.10.2026</span><span class=\"eyebrow\">Preisliste · zuletzt geändert 10.10.2026</span>",1));
        for extra in [
            "&quot;only_customer&quot;:&quot;contract&quot;,",
            "&quot;condition&quot;:{&quot;only_customer&quot;:&quot;contract&quot;},",
        ] {
            cases.push(HTML.replacen("&quot;ab&quot;:40", &format!("{extra}&quot;ab&quot;:40"), 1));
        }
        // Strict nested schema: neither duplicate known fields nor changed
        // value shapes may become a silently simplified numeric contract.
        cases.push(HTML.replacen(
            "&quot;ab&quot;:40",
            "&quot;ab&quot;:40,&quot;ab&quot;:50",
            1,
        ));
        cases.push(HTML.replacen("&quot;ab&quot;:40", "&quot;ab&quot;:{&quot;kg&quot;:40}", 1));
        for (i, html) in cases.iter().enumerate() {
            assert_ne!(html, HTML, "mutation {i} actually applied");
            if let Ok(snapshot) = super::super::overview::parse(html, IMP, 200) {
                assert!(
                    commit(&db, &internal, snapshot, now).await.is_err(),
                    "review mutation {i} accepted"
                );
            }
            let after: Vec<_> = [
                "prices",
                "current_prices",
                "trader_materials",
                "source_article_snapshots",
                "article_acceptance_owners",
            ]
            .iter()
            .map(|t| {
                db.query_sql(&format!("SELECT * FROM {t} ORDER BY rowid"))
                    .unwrap()
                    .rows
            })
            .collect();
            assert_eq!(
                after, before,
                "review mutation {i} altered history/current/acceptances/ledger"
            );
        }
        // A genuinely absent page date remains honest None, not a parse defect.
        let absent = HTML.replace("Preisliste · zuletzt geändert 09.10.2026", "Preisliste");
        let snapshot = fixture(&absent);
        assert!(snapshot.page_change_date.is_none());
        assert!(snapshot.diagnostics.is_empty());
        commit(&db, &internal, snapshot, now).await.unwrap();
    }

    #[tokio::test]
    async fn plain_projection_and_wrong_trader_cannot_write() {
        let (_dir, db, internal, now) = setup("mkm-untyped-guard");
        let s = fixture(HTML);
        assert!(
            crate::traders::record(&db, &internal, SLUG, &s.outcome, &now)
                .await
                .is_err()
        );
        assert!(crate::traders::record_outcome(
            &db,
            &internal,
            "foreign-trader",
            &crate::traders::RecordOutcome::MkmArticles(s),
            &now
        )
        .await
        .is_err());
        assert!(db
            .current_prices_from(db.find_trader_id(SLUG).unwrap().unwrap(), URL)
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn incomplete_or_corrupt_typed_contract_cannot_write_or_retire() {
        let (_dir, db, internal, now) = setup("mkm-write-guard");
        replay_readonly(&db);
        let before = db.query_sql("SELECT COUNT(*) FROM prices").unwrap();
        let mut cases = vec![];
        let mut a = fixture(HTML);
        a.articles.remove(0);
        cases.push(a);
        let mut a = fixture(HTML);
        a.articles[0].article = "UNKNOWN".to_owned();
        cases.push(a);
        let mut a = fixture(HTML);
        a.articles[0].label.push_str(" changed");
        cases.push(a);
        let mut a = fixture(HTML);
        a.outcome.prices[0].unit = "EUR/t";
        cases.push(a);
        let mut a = fixture(HTML);
        a.outcome.prices[0].price = 999.0;
        cases.push(a);
        let mut a = fixture(HTML);
        a.outcome.fetch_url = "https://foreign.test/".to_owned();
        cases.push(a);
        let mut a = fixture(HTML);
        a.outcome.trader_info.street = "Industriestraße 40".to_owned();
        cases.push(a);
        let mut a = fixture(HTML);
        a.articles[0].tiers[0].preis = 0.0;
        cases.push(a);
        for s in cases {
            assert!(commit(&db, &internal, s, now).await.is_err());
            assert!(current(&db, "kupfer-berry", "Kerze").is_some());
            assert!(current(&db, "kabel-kupfer", "Stecker 30%").is_some());
            assert_eq!(
                db.query_sql("SELECT COUNT(*) FROM prices").unwrap().rows,
                before.rows
            );
        }
    }
}
