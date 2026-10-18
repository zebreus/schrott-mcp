//! Goldtrans Edelmetallhandel (Hamburg-Wandsbek): exact per-gram purchase
//! prices in one clean HTML table ("Gold Legierung" / "Preis", €/g in
//! every row, page-stated "Stand: dd.mm.yyyy"). 13 gold alloys plus a
//! separate Zahngold row (live 27.09.2026: 14 rows). Quoted unit is
//! honestly EUR/g (catalog units match — no conversion anywhere).
//! Fineness rides in the variant ("Gold 999 24 Karat" → `gold`/`999`);
//! Zahngold maps to `zahngold` (dental alloy, never a gold alias).

use scraper::{Html, Selector};

use super::super::{
    Handler, HandlerOutcome, Schedule, ScrapedPrice, TraderInfo, fetch_text, parse_de_date,
    parse_eur,
};
use crate::IngestError;

pub const SLUG: &str = "hh-wandsbek-goldtrans-edelmetallhandel";
/// Bespoke, live-verified impressum URL. A move fails the step
/// loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.goldtrans.de/impressum.html";

pub const URL: &str =
    "https://www.goldtrans.de/goldankauf-preise-aktueller-goldpreis-ankauf-in-hamburg.html";

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
    let mut outcome = prices_from_html(status, &html)?;
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    outcome.trader_info = extract_info(&imp_html)?;
    Ok(outcome)
}

// Same price outcome as the live handler, without HTTP or contact enrichment.
// Kept separate so a captured response can reach the real recording boundary.
fn prices_from_html(status: u16, html: &str) -> Result<HandlerOutcome, IngestError> {
    let (published_at, rows, mut skipped_labels) = parse(html)?;
    // Exact per-gram list prices at full confidence. Fineness rides in
    // the variant; rows without metal word skip loudly with evidence.
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => prices.push(ScrapedPrice {
                material,
                variant: variant.into(),
                price,
                currency: "EUR",
                unit,
                price_kind: "exact",
                price_min: None,
                price_max: None,
                confidence: Some(1.0),
                label,
            }),
            None => skipped_labels.push(format!(
                "{label} (Sorte unverstaendlich: {price:.2} {unit})"
            )),
        }
    }
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
        prices,
        acceptances: vec![],
        trader_info: TraderInfo::default(),
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at,
    })
}

/// Explicit label → (material, variant) mapping. Fineness rides in the
/// variant ("Gold 999 24 Karat" → `gold`/`999`); Zahngold is dental
/// alloy, never a gold alias. Labels without metal word stay `None`.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    if l.contains("zahngold") {
        return Some(("zahngold", ""));
    }
    if l.contains("gold") {
        return Some(("gold", fineness(&l)));
    }
    if l.contains("silber") {
        return Some(("silber", fineness(&l)));
    }
    None
}

/// First fineness run in the label ("Gold 999 24 Karat" → "999").
// "Zahngold" carries no digits, so it is checked first above.
fn fineness(l: &str) -> &'static str {
    for fin in [
        "999", "986", "980", "965", "916", "900", "875", "833", "750", "585", "416", "375", "333",
    ] {
        if l.contains(fin) {
            return fin;
        }
    }
    ""
}

/// Bespoke unit rule for THIS table only: every price cell quotes "€/g"
/// (per gram). Anything else is a redesign → loud skip, never a default.
fn unit_of(price_cell: &str) -> Option<&'static str> {
    if price_cell.to_lowercase().contains("/g") {
        Some("EUR/g")
    } else {
        None
    }
}

// This site's Gold/Zahngold table publishes exact purchase quotes, never
// free acceptance. Validate the full German amount rather than using the
// permissive shared parser, which can discard signs or malformed separators.
fn positive_gold_quote(label: &str, price_cell: &str) -> Result<f64, IngestError> {
    let amount = price_cell
        .trim()
        .strip_suffix("€/g")
        .or_else(|| price_cell.trim().strip_suffix("EUR/g"))
        .map(str::trim);
    amount
        .filter(|s| {
            let digits = |v: &str| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit());
            let mut parts = s.split(',');
            let integer = parts.next().unwrap_or_default();
            let fraction_valid = parts.next().is_none_or(digits);
            let mut groups = integer.split('.');
            let first = groups.next().unwrap_or_default();
            let integer_valid = digits(first)
                && (!integer.contains('.') || first.len() <= 3)
                && groups.all(|g| g.len() == 3 && digits(g));
            integer_valid && fraction_valid && parts.next().is_none()
        })
        .and_then(|s| s.replace('.', "").replace(',', ".").parse::<f64>().ok())
        .filter(|price| price.is_finite() && *price > 0.0)
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("{label}: positiver endlicher Ankaufspreis in EUR/g erforderlich; gesamter Snapshot verworfen"),
        })
}

fn parse(
    html: &str,
) -> Result<
    (
        Option<String>,
        Vec<(String, f64, &'static str)>,
        Vec<String>,
    ),
    IngestError,
> {
    // Window: the price block between its intro line and the "Bitte
    // Beachten" notes. The footer maps <table> sits outside this window
    // and is excluded twice over by the header selection below.
    let start = html
        .find("Hier aktuelle Ankaufspreise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisblock fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Bitte Beachten").unwrap_or(tail.len());
    let window = &tail[..end];
    // Kopf-Selektion: the table whose header reads "Gold Legierung" —
    // never the first table on the page. The <style> block inside the
    // window is never read as text (only table cells are selected).
    let doc = Html::parse_fragment(window);
    let table_sel = Selector::parse("table").expect("valid selector");
    let th_sel = Selector::parse("th").expect("valid selector");
    let tr_sel = Selector::parse("tr").expect("valid selector");
    let td_sel = Selector::parse("td").expect("valid selector");
    let table = doc
        .select(&table_sel)
        .find(|t| {
            t.select(&th_sel)
                .any(|h| h.text().collect::<String>().contains("Gold Legierung"))
        })
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabelle (Gold Legierung) fehlt".to_owned(),
        })?;
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for tr in table.select(&tr_sel) {
        let cells: Vec<String> = tr
            .select(&td_sel)
            .map(|c| c.text().collect::<String>())
            .map(|t| t.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect();
        if cells.is_empty() || cells[0].is_empty() {
            continue;
        }
        let label = cells[0].clone();
        let price_raw = cells.get(1).map(String::as_str).unwrap_or_default();
        if matches!(grade_for(&label), Some(("gold" | "zahngold", _))) {
            // Fail the complete parse, including earlier valid rows, before
            // any HandlerOutcome can reach recording or contact enrichment.
            let price = positive_gold_quote(&label, price_raw)?;
            rows.push((label, price, "EUR/g"));
            continue;
        }
        if cells.len() < 2 {
            continue;
        }
        let Some(price) = parse_eur(price_raw) else {
            skips.push(format!("{label} (Preis unverstaendlich: {price_raw})"));
            continue;
        };
        let Some(unit) = unit_of(price_raw) else {
            skips.push(format!("{label} (Einheit unverstaendlich: {price_raw})"));
            continue;
        };
        rows.push((label, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preiszeilen".to_owned(),
        });
    }
    // Page-stated validity ("Stand : 27.09.2026"); absent → None
    // (observed_at = age). Anchored on "Stand" so stray dates elsewhere
    // cannot leak in.
    let published_at = window.find("Stand").and_then(|i| date_in(&window[i..]));
    Ok((published_at, rows, skips))
}

/// dd.mm.yyyy → RFC 3339 UTC midnight (tappe-style byte scan; this
/// handler's own copy, no shared helper).
fn date_in(window: &str) -> Option<String> {
    let bytes = window.as_bytes();
    let mut i = 0;
    while i + 10 <= bytes.len() {
        if bytes[i].is_ascii_digit()
            && bytes[i + 2] == b'.'
            && bytes[i + 5] == b'.'
            && bytes[i + 6..].iter().take(4).all(|c| c.is_ascii_digit())
        {
            let (d, m, y) = (
                &window[i..i + 2],
                &window[i + 3..i + 5],
                &window[i + 6..i + 10],
            );
            if let Some(rfc) = parse_de_date(d, m, y) {
                return Some(rfc);
            }
        }
        i += 1;
    }
    None
}

/// Bespoke contact extraction for THIS impressum only: the `<p>` holding
/// "GOLDTRANS Edelmetallhandel e.K." carries `<br/>`-separated lines
/// (firm, street + PLZ city on one line, "Tel.: … / Fax.: …", "E-Mail:
/// …"). Missing anchor → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let p = Selector::parse("p").expect("valid selector");
    let block = doc
        .select(&p)
        .find(|el| {
            el.text()
                .collect::<String>()
                .contains("GOLDTRANS Edelmetallhandel e.K.")
        })
        .ok_or_else(|| IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        })?;
    let lines: Vec<String> = block
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    // "Ahrensburger Strasse 69 22041 Hamburg": street is everything
    // before the 5-digit PLZ token, city the token after it.
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for line in lines.iter() {
        let toks: Vec<&str> = line.split_whitespace().collect();
        for (j, t) in toks.iter().enumerate() {
            if t.len() == 5 && t.chars().all(|c| c.is_ascii_digit()) {
                postcode = (*t).to_owned();
                if let Some(ci) = toks.get(j + 1) {
                    city = (*ci).to_owned();
                }
                street = toks[..j].join(" ");
                break;
            }
        }
        if !postcode.is_empty() {
            break;
        }
    }
    let mut phone = String::new();
    let mut email = String::new();
    for line in &lines {
        if let Some(v) = line.strip_prefix("Tel.:") {
            phone = v.split('/').next().unwrap_or_default().trim().to_owned();
        } else if let Some(v) = line.strip_prefix("E-Mail:") {
            email = v.trim().to_owned();
        }
    }
    if street.is_empty() && phone.is_empty() && email.is_empty() {
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

/// Strip tags from a fragment (html5ever already decoded entities).
/// Fragments from splitting on "<br" start with a tag remnant
/// (` class="…"`) — drop everything up to the first '>' first, or the
/// attributes parse as text.
fn strip_fragment(s: &str) -> String {
    let s = match s.find('>') {
        Some(i) => &s[i + 1..],
        None => s,
    };
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

#[cfg(test)]
mod tests {
    use super::{date_in, grade_for, parse, unit_of};
    use schrott_mcp_store::{InternalDb, PublicDb};

    const ZERO_CAPTURE: &str = include_str!("fixtures/goldtrans-zero-20261018.html");

    // Synthetic positive control, NOT another live capture. Keep all 14 real
    // labels/cells; substitute the independently observed 12:30 History values.
    fn positive_snapshot() -> String {
        assert_eq!(ZERO_CAPTURE.matches("0,00").count(), 14);
        let mut html = ZERO_CAPTURE.to_owned();
        for value in [
            "110,70", "109,15", "108,49", "106,83", "101,40", "99,63", "96,86", "92,21", "83,03",
            "64,76", "46,05", "41,51", "36,86", "68,63",
        ] {
            html = html.replacen("0,00", value, 1);
        }
        html
    }

    fn isolated_catalog(
        label: &str,
    ) -> (
        crate::test_support::TempDbDir,
        PublicDb,
        InternalDb,
        chrono::DateTime<chrono::Utc>,
    ) {
        let dir = crate::test_support::TempDbDir::new(label);
        let public = PublicDb::open(dir.path()).expect("isolated public DB");
        let internal = InternalDb::open(dir.path()).expect("isolated internal DB");
        crate::seed_metadata(&public).expect("existing catalog setup helper");
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-18T12:30:09+00:00")
            .unwrap()
            .with_timezone(&chrono::Utc);
        (dir, public, internal, now)
    }

    fn database_state(public: &PublicDb) -> serde_json::Value {
        serde_json::json!({
            "current": public.query_sql("SELECT * FROM current_prices ORDER BY trader_id,material_id,variant").unwrap().rows,
            "history": public.query_sql("SELECT * FROM prices ORDER BY id").unwrap().rows,
            "acceptances": public.query_sql("SELECT * FROM trader_materials ORDER BY trader_id,material_id").unwrap().rows,
            "trader": public.query_sql("SELECT * FROM traders WHERE slug='hh-wandsbek-goldtrans-edelmetallhandel'").unwrap().rows,
            "sequences": public.query_sql("SELECT * FROM sqlite_sequence ORDER BY name").unwrap().rows,
        })
    }

    // Replay the exact price-outcome path used by scrape(), then the real
    // recording entry point. No test-only price filtering/validation or SQL
    // writes: an accepted bad snapshot really reaches the isolated recorder.
    async fn replay_capture(
        public: &PublicDb,
        internal: &InternalDb,
        html: &str,
        now: &chrono::DateTime<chrono::Utc>,
    ) -> Result<(), crate::IngestError> {
        let outcome = super::prices_from_html(200, html)?;
        crate::traders::record(public, internal, super::SLUG, &outcome, now)
            .await
            .map(|_| ())
    }

    async fn assert_snapshot_rejected_without_writes(html: &str, label: &str) {
        let (_dir, public, internal, now) = isolated_catalog(label);
        replay_capture(&public, &internal, &positive_snapshot(), &now)
            .await
            .expect("positive baseline must record all 14 keys");
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices")
                .unwrap()
                .rows
                .len(),
            14
        );
        assert_replay_rejected_without_writes(&public, &internal, html, &now, label).await;
    }

    async fn assert_replay_rejected_without_writes(
        public: &PublicDb,
        internal: &InternalDb,
        html: &str,
        now: &chrono::DateTime<chrono::Utc>,
        label: &str,
    ) {
        let before = database_state(public);
        let result =
            replay_capture(public, internal, html, &(*now + chrono::Duration::hours(6))).await;
        assert_eq!(
            database_state(public),
            before,
            "{label}: the whole snapshot must be rejected before any History, Current, acceptance or trader write"
        );
        assert!(
            matches!(&result, Err(crate::IngestError::Parse { url, .. }) if url == super::URL),
            "{label}: require a Goldtrans parse error, not successful skips/empty output or an unrelated recording failure: {result:?}"
        );
    }

    #[tokio::test]
    async fn captured_zero_snapshot_rejected_before_recording() {
        use sha2::Digest;

        assert_eq!(
            format!("{:x}", sha2::Sha256::digest(ZERO_CAPTURE.as_bytes())),
            "b029d1ec12c2b0dcf1496df6dc28fab985f83e62b2bc66df797f02f011372682",
            "sanitized capture must stay pinned to the reviewed source excerpt"
        );
        assert_snapshot_rejected_without_writes(ZERO_CAPTURE, "goldtrans-zero-capture").await;
    }

    #[tokio::test]
    async fn partial_zero_snapshot_rejected_atomically_before_recording() {
        // A changed valid first row must not get written before the bad late
        // gold or Zahngold row is found; dropping only that bad row is not OK.
        for (value, label) in [
            ("36,86", "goldtrans-partial-zero-gold"),
            ("68,63", "goldtrans-partial-zero-zahngold"),
        ] {
            let positive = positive_snapshot();
            assert_eq!(positive.matches(value).count(), 1);
            let damaged = positive
                .replacen("110,70", "111,70", 1)
                .replacen(value, "0,00", 1);
            assert_ne!(damaged, positive, "mutation must reach the real capture");
            assert_eq!(damaged.matches("0,00").count(), 1);
            assert_snapshot_rejected_without_writes(&damaged, label).await;
        }
    }

    #[tokio::test]
    async fn negative_nonfinite_and_unparseable_prices_reject_before_recording() {
        let (_dir, public, internal, now) = isolated_catalog("goldtrans-invalid-amounts");
        let positive = positive_snapshot();
        replay_capture(&public, &internal, &positive, &now)
            .await
            .unwrap();
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices")
                .unwrap()
                .rows
                .len(),
            14
        );
        // Share one baseline across the matrix: every rejection must leave it
        // intact. No repeated catalog seeding and no test-only guard/filter.
        let overflow = "9".repeat(400);
        for amount in [
            "-1,00",
            "\u{2212}1,00",
            "-0,00",
            "NaN",
            "inf",
            "-inf",
            "Infinity",
            "1e309",
            "1,,",
            "1.2.3,45",
            ",12",
            "12,",
            "1.23,45",
            "1,2,3",
            "1.25",
            overflow.as_str(),
            "auf Anfrage",
            "",
        ] {
            for old in ["36,86", "68,63"] {
                assert_eq!(positive.matches(old).count(), 1);
                let damaged = positive
                    .replacen("110,70", "111,70", 1)
                    .replacen(old, amount, 1);
                assert_ne!(damaged, positive);
                assert_replay_rejected_without_writes(
                    &public,
                    &internal,
                    &damaged,
                    &now,
                    &format!("invalid late gold/Zahngold amount replacing {old}: {amount}"),
                )
                .await;
            }
        }
    }

    #[tokio::test]
    async fn missing_gold_price_cell_rejects_before_recording() {
        let positive = positive_snapshot();
        let start = positive.find("<td>Zahngold").unwrap();
        let tail = &positive[start..];
        let price_start = start + tail.find("<td>68,63").unwrap();
        let price_end = price_start + positive[price_start..].find("</td>").unwrap() + 5;
        let mut damaged = positive.replacen("110,70", "111,70", 1);
        // The early edit preserves byte length, so the real late-cell offsets
        // above still identify the entire Zahngold price cell.
        assert_eq!(damaged.len(), positive.len());
        damaged.replace_range(price_start..price_end, "");
        assert_snapshot_rejected_without_writes(&damaged, "goldtrans-missing-price").await;
    }

    #[tokio::test]
    async fn positive_snapshots_preserve_prices_variants_and_recording() {
        let (_dir, public, internal, now) = isolated_catalog("goldtrans-positive");
        // Existing positive excerpt must remain accepted, not just our mutation.
        replay_capture(&public, &internal, FIXTURE, &now)
            .await
            .unwrap();
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices")
                .unwrap()
                .rows
                .len(),
            3
        );
        let positive = positive_snapshot();
        let out = super::prices_from_html(200, &positive).expect("positive price outcome");
        assert!(out.skipped_labels.is_empty());
        assert_eq!(out.prices.len(), 14);
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-10-18T00:00:00+00:00")
        );
        for (p, (material, variant, price)) in out.prices.iter().zip([
            ("gold", "999", 110.70),
            ("gold", "986", 109.15),
            ("gold", "980", 108.49),
            ("gold", "965", 106.83),
            ("gold", "916", 101.40),
            ("gold", "900", 99.63),
            ("gold", "875", 96.86),
            ("gold", "833", 92.21),
            ("gold", "750", 83.03),
            ("gold", "585", 64.76),
            ("gold", "416", 46.05),
            ("gold", "375", 41.51),
            ("gold", "333", 36.86),
            ("zahngold", "", 68.63),
        ]) {
            assert_eq!(
                (p.material, p.variant.as_ref(), p.price),
                (material, variant, price)
            );
            assert_eq!(p.unit, "EUR/g");
            assert_eq!(p.price_kind, "exact");
            assert_eq!(p.confidence, Some(1.0));
        }
        replay_capture(
            &public,
            &internal,
            &positive,
            &(now + chrono::Duration::hours(1)),
        )
        .await
        .unwrap();
        assert_eq!(
            public
                .query_sql("SELECT * FROM current_prices")
                .unwrap()
                .rows
                .len(),
            14
        );
        assert_eq!(
            public.query_sql("SELECT * FROM prices WHERE price <= 0 OR unit != 'EUR/g' OR price_kind != 'exact' OR published != 1").unwrap().rows.len(),
            0
        );
    }

    #[test]
    fn positive_german_number_grammar() {
        for (cell, expected) in [
            ("1.234,56 €/g", 1234.56),
            ("1.234 EUR/g", 1234.0),
            ("1234,56 €/g", 1234.56),
            ("1,2 €/g", 1.2),
            ("1 €/g", 1.0),
        ] {
            assert_eq!(
                super::positive_gold_quote("Gold 999", cell).unwrap(),
                expected
            );
        }
    }

    // Real live excerpt (27.09.2026): intro + <style> + table head +
    // three rows (incl. the Zahngold row with entity) + Stand line.
    const FIXTURE: &str = "<p>Hier aktuelle Ankaufspreise f&uuml;r Altgold in Hamburg f&uuml;r unseren Goldankauf<style type=\"text/css\">table { border-collapse: collapse; }</style>\
        <table><tr><th width=\"266\"><div align=\"left\">Gold Legierung </div></th><th width=\"270\"><div align=\"left\">Preis</div></th></tr>\
        <tr><td>Gold 999 24 Karat</td><td>111,33 &euro;/g</td></tr>\
        <tr><td>Gold 750 18 Karat</td><td>83,50 &euro;/g </td></tr>\
        <tr><td>Zahngold (Ohne Z&auml;hne, Gelb) </td><td>69,02 &euro;/g </td></tr>\
        </table><p><span class=\"style2\">Stand : 27.09.2026</span> Hamburg</p>";

    #[test]
    fn table_rows_and_stand_date() {
        let (published, rows, skips) = parse(FIXTURE).expect("parses");
        assert!(skips.is_empty());
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].0, "Gold 999 24 Karat");
        assert_eq!(rows[0].1, 111.33);
        assert_eq!(rows[0].2, "EUR/g");
        assert_eq!(rows[2].0, "Zahngold (Ohne Z\u{00e4}hne, Gelb)");
        assert_eq!(rows[2].1, 69.02);
        assert!(
            published
                .as_deref()
                .unwrap_or_default()
                .starts_with("2026-09-27")
        );
        assert_eq!(unit_of("111,33 \u{20ac}/g"), Some("EUR/g"));
        assert_eq!(unit_of("auf Anfrage"), None);
        assert!(
            parse("<p>Hier aktuelle Ankaufspreise X</p>").is_err(),
            "leere Tabelle = Err"
        );
        assert!(
            parse("<p>anderer Inhalt ohne Block</p>").is_err(),
            "missing anchor = Err"
        );
        assert!(date_in("Stand : 27.09.2026 Hamburg").is_some());
        assert_eq!(date_in("kein Datum"), None);
    }

    #[test]
    fn fineness_rides_in_variant() {
        // Every live label (27.09.2026: 13 alloys + Zahngold) maps now
        // that the catalog holds EUR/g precious metals.
        for (label, material, variant) in [
            ("Gold 999 24 Karat", "gold", "999"),
            ("Gold 986 23,6 Karat", "gold", "986"),
            ("Gold 980 23,5 Karat", "gold", "980"),
            ("Gold 965 23 Karat", "gold", "965"),
            ("Gold 916 22 Karat", "gold", "916"),
            ("Gold 900 21.6 Karat", "gold", "900"),
            ("Gold 875 21 Karat", "gold", "875"),
            ("Gold 833 20 Karat", "gold", "833"),
            ("Gold 750 18 Karat", "gold", "750"),
            ("Gold 585 14 Karat", "gold", "585"),
            ("Gold 416 10 Karat", "gold", "416"),
            ("Gold 375 9 Karat", "gold", "375"),
            ("Gold 333 8 Karat", "gold", "333"),
            ("Zahngold (Ohne Z\u{00e4}hne, Gelb)", "zahngold", ""),
        ] {
            assert_eq!(grade_for(label), Some((material, variant)), "{label}");
        }
        assert_eq!(grade_for("Ankaufsbedingungen"), None);
    }

    #[test]
    fn impressum_block() {
        let imp = "<div class=\"entry-content\"><p>GOLDTRANS Edelmetallhandel e.K.<br />Inh. Tarkan Yilmaz<br />Ahrensburger Strasse 69 22041 Hamburg<br />Tel.: 040 970 79 580 / Fax.: 040 970 79 581<br />E-Mail: info@goldtrans.de<br />St.-Nr.:51/669/00559</p></div>";
        let info = super::extract_info(imp).expect("parses");
        assert_eq!(info.street, "Ahrensburger Strasse 69");
        assert_eq!(info.postcode, "22041");
        assert_eq!(info.city, "Hamburg");
        assert_eq!(info.phone, "040 970 79 580");
        assert_eq!(info.email, "info@goldtrans.de");
        assert!(super::extract_info("<p>Neu hier</p>").is_err());
    }
}
