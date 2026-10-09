//! Dynamically linked Trapper Anlieferung PDF; Poppler pdftotext required.
//! Non-binding Richtwerte, per-row units and grade variants are preserved.
use crate::traders::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
};
use crate::IngestError;
use scraper::{Html, Selector};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const URL: &str = "https://www.trapper-kulmbach.de/service/preise";

pub fn handler() -> Handler {
    Handler {
        slug: "by-kulmbach-trapper",
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |client| Box::pin(scrape(client)),
    }
}
fn error(url: &str, detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: url.to_owned(),
        detail: detail.into(),
    }
}
fn discover(html: &str) -> Result<String, IngestError> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("main a[href]").expect("selector");
    let base = reqwest::Url::parse(URL).expect("URL");
    let mut links = Vec::new();
    for a in doc.select(&selector) {
        let label = a.text().collect::<String>().to_lowercase();
        if !label.contains("preisliste") || !label.contains("schrott") {
            continue;
        }
        let Ok(url) = base.join(a.value().attr("href").unwrap_or_default()) else {
            continue;
        };
        if url.scheme() == "https"
            && url.host_str() == base.host_str()
            && url.path().to_lowercase().ends_with(".pdf")
        {
            let url = url.to_string();
            if !links.contains(&url) {
                links.push(url);
            }
        }
    }
    if links.len() != 1 {
        return Err(error(
            URL,
            format!("expected one Schrott preisliste PDF, found {}", links.len()),
        ));
    }
    Ok(links.remove(0))
}
async fn extract(bytes: &[u8], url: &str) -> Result<String, IngestError> {
    let mut child = tokio::process::Command::new("/usr/bin/prlimit")
        .args([
            "--as=268435456",
            "--cpu=15",
            "--",
            "/usr/bin/pdftotext",
            "-layout",
            "-",
            "-",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| error(url, format!("pdftotext unavailable: {e}")))?;
    let mut stdin = child.stdin.take().expect("piped stdin");
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let (_, stdout, stderr, status) =
        tokio::time::timeout(std::time::Duration::from_secs(20), async {
            tokio::try_join!(
                async {
                    stdin.write_all(bytes).await?;
                    drop(stdin);
                    Ok::<_, std::io::Error>(())
                },
                read_output(stdout, 1_000_000),
                read_output(stderr, 65_536),
                child.wait()
            )
        })
        .await
        .map_err(|_| error(url, "PDF extraction timeout"))?
        .map_err(|e| error(url, format!("PDF extraction: {e}")))?;
    if !status.success() {
        return Err(error(
            url,
            format!("pdftotext: {}", String::from_utf8_lossy(&stderr)),
        ));
    }
    String::from_utf8(stdout).map_err(|e| error(url, format!("PDF text: {e}")))
}
async fn read_output<R: tokio::io::AsyncRead + Unpin>(
    reader: R,
    limit: usize,
) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .await?;
    if bytes.len() > limit {
        return Err(std::io::Error::other(
            "PDF extraction output limit exceeded",
        ));
    }
    Ok(bytes)
}
async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (_, html) = fetch_text(client, URL).await?;
    let url = discover(&html)?;
    let mut response = client
        .get(&url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.clone(),
            source,
        })?;
    let status = response.status().as_u16();
    if !response.status().is_success() {
        return Err(error(&url, format!("HTTP {status}")));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|source| IngestError::Body {
        url: url.clone(),
        source,
    })? {
        if bytes.len() + chunk.len() > 5_000_000 {
            return Err(error(&url, "PDF exceeds 5 MB"));
        }
        bytes.extend_from_slice(&chunk);
    }
    if !bytes.starts_with(b"%PDF-") {
        return Err(error(&url, "not a PDF or PDF exceeds 5 MB"));
    }
    let text = extract(&bytes, &url).await?;
    let mut out = parse(&text, &url)?;
    out.byte_len = bytes.len();
    out.status_code = status;
    out.website_alive = true;
    Ok(out)
}

// Exact operator labels, not item-number-only mappings: reused numbers must
// not silently become another grade. Distinct grades never collapse.
fn grade(label: &str) -> Option<(&'static str, &'static str)> {
    Some(match label {
        "Mischschrott" => ("mischschrott", "ab 100 kg Vergütung"),
        "Shreddervormaterial" => ("stahlschrott-shredder", "ab 100 kg Vergütung"),
        "Trägerschrott" => ("stahlschrott-traeger", "ab 100 kg Vergütung"),
        "Gemischter Guss" => ("eisenschrott-gussbruch", "gemischt; ab 100 kg Vergütung"),
        "PKW-Bremsscheiben (ohne Anhaftung)" => (
            "eisenschrott-gussbruch",
            "PKW-Bremsscheiben ohne Anhaftung; ab 100 kg Vergütung",
        ),
        "Cu-Millberry" => ("kupfer-millberry", ""),
        "Cu-Blech neu" => ("kupfer-gemischt", "Cu-Blech neu"),
        "Cu-Raff. 95 %" => ("kupfer-gemischt", "Cu-Raff. 95 %"),
        "Cu-Kabel ohne Stecker" => ("kabel-kupfer", "ohne Stecker"),
        "Cu-Rohre isol." => ("kupfer-wicu", "Cu-Rohre isol."),
        "Messing-Abfälle" => ("messing", ""),
        "Alu-Blech alt 3%Fe" => ("aluminium-blech", "alt 3%Fe"),
        "Alu-Blech alt Fe-frei" => ("aluminium-blech", "alt Fe-frei"),
        "VA-Abfälle" => ("edelstahl-gemischt", "VA-Abfälle"),
        "Zinkblech neu" => ("zink", "Blech neu"),
        "Zinkblech alt" => ("zink", "Blech alt"),
        "Zinn" => ("zinn", ""),
        "Weichblei" => ("blei", "Weichblei"),
        "Elektromotoren" => ("elektromotoren", ""),
        "Starterbatterien" => ("batterien-blei", "Starterbatterien"),
        "Leiterplatten-/Platinen (fest)" => ("platinen", "fest"),
        _ => return None,
    })
}

fn parse(text: &str, url: &str) -> Result<HandlerOutcome, IngestError> {
    if !text.contains("Preisliste Anlieferung:")
        || !text.contains("Trapper GmbH")
        || !text.contains("Kulmbach")
    {
        return Err(error(url, "not Trapper's Anlieferung price list"));
    }
    // The threshold is part of our first five variants, so fail closed if it
    // changes instead of stamping an obsolete condition onto new prices.
    if !text.contains("Mindestmenge 100kg für Vergütung")
        || !text.contains("Stahlschrott*")
        || !text.contains("Gussbruch*")
    {
        return Err(error(url, "steel/cast iron payment threshold changed"));
    }
    let date = text
        .split("gültig ab dem ")
        .nth(1)
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| {
            let p: Vec<_> = s.split('.').collect();
            if p.len() == 3 {
                parse_de_date(p[0], p[1], p[2])
            } else {
                None
            }
        });
    let Some(date) = date else {
        return Err(error(url, "missing/invalid PDF validity date"));
    };
    let mut out = HandlerOutcome {
        fetch_url: url.to_owned(),
        published_at: Some(date),
        ..HandlerOutcome::default()
    };
    let mut keys = std::collections::HashSet::new();
    for line in text.lines() {
        let tokens: Vec<_> = line.split_whitespace().collect();
        if !tokens
            .first()
            .is_some_and(|s| s.len() == 3 && s.chars().all(|c| c.is_ascii_digit()))
        {
            continue;
        }
        let n = tokens.len();
        if n < 5 || tokens[n - 2] != "€" {
            return Err(error(url, format!("malformed price row: {line}")));
        }
        let unit = match tokens[n - 1] {
            "kg" => "EUR/kg",
            "to" => "EUR/t",
            _ => return Err(error(url, format!("unknown row unit: {line}"))),
        };
        if !tokens[n - 3]
            .chars()
            .all(|c| c.is_ascii_digit() || c == ',' || c == '.')
        {
            return Err(error(url, format!("invalid price: {line}")));
        }
        let Some(price) = parse_eur(tokens[n - 3]).filter(|v| v.is_finite() && *v >= 0.0) else {
            return Err(error(url, format!("invalid price: {line}")));
        };
        let label = tokens[1..n - 3].join(" ");
        let Some((material, variant)) = grade(&label) else {
            out.skipped_labels.push(label);
            continue;
        };
        if !keys.insert((material, variant)) {
            return Err(error(url, format!("duplicate grade: {label}")));
        }
        out.prices.push(ScrapedPrice {
            material,
            variant,
            price,
            currency: "EUR",
            unit,
            price_kind: "approx",
            price_min: None,
            price_max: None,
            confidence: Some(0.8),
            label: format!(
                "{label}; freibleibender Richtwert, abhängig von Menge/Zusammensetzung; {variant}"
            ),
        });
    }
    if out.prices.is_empty() {
        return Err(error(url, "no mapped PDF price rows"));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn extraction_rejects_invalid_input_and_bounds_output() {
        assert!(extract(b"not a PDF", URL).await.is_err());
        assert_eq!(read_output(&b"1234"[..], 4).await.unwrap(), b"1234");
        assert!(read_output(&b"12345"[..], 4).await.is_err());
    }

    #[test]
    fn source_richtwerte_are_structured_as_indicative() {
        let out = parse(include_str!("fixtures/trapper-20260804.txt"), URL).unwrap();
        assert_eq!(out.prices.len(), 21);
        assert!(out
            .prices
            .iter()
            .all(|price| price.price_kind == "approx" && price.confidence == Some(0.8)));
    }
    const FIXTURE: &str = include_str!("fixtures/trapper-20260804.txt");
    #[test]
    fn discovers_current_link_not_a_fixed_month() {
        let html = "<main><a href='/upload/new-october.pdf'>Preisliste Schrott&amp;Metalle KW41</a><a href='/upload/fees.pdf'>Gebühren</a></main>";
        assert_eq!(
            discover(html).unwrap(),
            "https://www.trapper-kulmbach.de/upload/new-october.pdf"
        );
        assert!(discover(
            &html.replace("/upload/new-october.pdf", "https://other.test/prices.pdf")
        )
        .is_err());
        assert!(discover("<main></main>").is_err());
        assert!(discover(&html.replace(
            "</main>",
            "<a href='/other.pdf'>Preisliste Schrott</a></main>"
        ))
        .is_err());
    }
    #[test]
    fn all_grades_dates_units_and_conditions() {
        let out = parse(FIXTURE, URL).unwrap();
        assert_eq!(out.prices.len(), 21);
        assert!(out.skipped_labels.is_empty());
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-08-04T00:00:00+00:00")
        );
        assert_eq!(out.prices.iter().filter(|p| p.unit == "EUR/t").count(), 8);
        assert_eq!((out.prices[0].price, out.prices[5].price), (120.0, 8.5));
        assert!(out.prices[..5]
            .iter()
            .all(|p| p.variant.contains("ab 100 kg Vergütung")));
        assert!(out.prices[5..]
            .iter()
            .all(|p| !p.variant.contains("100 kg")));
        assert_ne!(out.prices[6].variant, out.prices[7].variant);
        assert_ne!(out.prices[11].variant, out.prices[12].variant);
        assert_ne!(out.prices[14].variant, out.prices[15].variant);
        assert_eq!(out.prices[13].material, "edelstahl-gemischt");
        assert_eq!(out.prices[9].material, "kupfer-wicu");
    }
    #[test]
    fn new_pdf_date_and_prices_are_not_frozen_to_fixture() {
        let updated = FIXTURE
            .replace("04.08.2026", "09.10.2026")
            .replace("8,50 €", "9,15 €");
        let out = parse(
            &updated,
            "https://www.trapper-kulmbach.de/upload/october.pdf",
        )
        .unwrap();
        assert_eq!(
            out.published_at.as_deref(),
            Some("2026-10-09T00:00:00+00:00")
        );
        assert_eq!(out.prices[5].price, 9.15);
        assert!(out.fetch_url.ends_with("october.pdf"));
    }
    #[test]
    fn fails_closed_on_changed_contract_and_reports_unknown_grades() {
        for changed in [
            FIXTURE.replace("Anlieferung:", "Verkauf:"),
            FIXTURE.replace("04.08.2026", "31.02.2026"),
            FIXTURE.replace("100kg", "200kg"),
            FIXTURE.replace("Gussbruch*", "Gussbruch"),
            FIXTURE.replace("8,50 € kg", "8,50 € Stk"),
            FIXTURE.replace("8,50 €", "-8,50 €"),
        ] {
            assert!(parse(&changed, URL).is_err());
        }
        let out = parse(&FIXTURE.replace("Cu-Millberry", "Cu unbekannt"), URL).unwrap();
        assert_eq!(out.prices.len(), 20);
        assert_eq!(out.skipped_labels, ["Cu unbekannt"]);
        assert!(parse(&format!("{FIXTURE}\n310 Cu-Millberry 8,50 € kg"), URL).is_err());
    }
    #[tokio::test]
    async fn records_catalog_prices_with_normalization_and_pdf_provenance() {
        let dir = crate::test_support::TempDbDir::new("trapper");
        let public = schrott_mcp_store::PublicDb::open(dir.path()).unwrap();
        let internal = schrott_mcp_store::InternalDb::open(dir.path()).unwrap();
        crate::pipeline::seed_metadata(&public).unwrap();
        let pdf = "https://www.trapper-kulmbach.de/upload/new.pdf";
        let out = parse(FIXTURE, pdf).unwrap();
        let recorded = crate::traders::record(
            &public,
            &internal,
            handler().slug,
            &out,
            &chrono::Utc::now(),
        )
        .await
        .unwrap();
        assert_eq!(recorded.recorded, 21);
        assert!(recorded.skipped.is_empty());
        assert!(recorded.canaries.is_empty());
        let trader = public.find_trader_id(handler().slug).unwrap().unwrap();
        for (material, variant, price) in [
            ("elektromotoren", "", 0.4),
            ("batterien-blei", "Starterbatterien", 0.3),
            ("platinen", "fest", 0.3),
        ] {
            let mid = public.find_material_id(material).unwrap().unwrap();
            let p = public
                .current_price_for(trader, mid, variant)
                .unwrap()
                .unwrap();
            assert_eq!(p.price, price);
            assert_eq!(p.unit, "EUR/kg");
            assert_eq!(p.source_url, pdf);
            assert_eq!(p.published_at.as_deref(), Some("2026-08-04T00:00:00+00:00"));
        }
    }
}
