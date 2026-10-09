//! ELNO's private-customer PDF, rediscovered on every scrape.
use super::super::{fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice};
use crate::IngestError;
use scraper::{Html, Selector};

const URL: &str = "https://www.elno-container.de/";
const IMPRESSUM_URL: &str = "https://www.elno-container.de/impressum/";

pub fn handler() -> Handler {
    Handler {
        slug: "be-spandau-elno-container-und-dienstleistungs",
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

fn verify_operator(html: &str) -> Result<(), IngestError> {
    let doc = Html::parse_document(html);
    let body = doc.root_element().text().collect::<String>();
    if [
        "ELNO Container- und Dienstleistungs GmbH",
        "HRB 141261 B",
        "Tiefwerderweg 13",
        "13597 Berlin",
    ]
    .iter()
    .all(|fact| body.contains(fact))
    {
        Ok(())
    } else {
        Err(error(
            IMPRESSUM_URL,
            "Betreiberidentität oder Standort hat sich geändert",
        ))
    }
}

fn pdf_url(html: &str) -> Result<String, IngestError> {
    let doc = Html::parse_document(html);
    let base = reqwest::Url::parse(URL).unwrap();
    let mut urls = std::collections::BTreeSet::new();
    for anchor in doc.select(&Selector::parse("a[href]").unwrap()) {
        let text = anchor.text().collect::<Vec<_>>().join(" ").to_lowercase();
        if !text.contains("altmetall") || !text.contains("preise") {
            continue;
        }
        let target = base
            .join(anchor.value().attr("href").unwrap())
            .map_err(|e| error(URL, format!("invalid price link: {e}")))?;
        if target.scheme() != "https"
            || target.host_str() != base.host_str()
            || !target.path().to_lowercase().ends_with(".pdf")
        {
            return Err(error(
                URL,
                "Altmetall price link is not an operator HTTPS PDF",
            ));
        }
        urls.insert(target.to_string());
    }
    if urls.len() != 1 {
        return Err(error(URL, "missing or ambiguous Altmetall price PDF"));
    }
    Ok(urls.into_iter().next().unwrap())
}

async fn scrape(client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    let (_, html) = fetch_text(client, URL).await?;
    let url = pdf_url(&html)?;
    let (_, impressum) = fetch_text(client, IMPRESSUM_URL).await?;
    verify_operator(&impressum)?;
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
    const MAX_PDF_BYTES: usize = 5_000_000;
    if response
        .content_length()
        .is_some_and(|len| len > MAX_PDF_BYTES as u64)
    {
        return Err(error(&url, "oversized price PDF"));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|source| IngestError::Body {
        url: url.clone(),
        source,
    })? {
        if bytes.len().saturating_add(chunk.len()) > MAX_PDF_BYTES {
            return Err(error(&url, "oversized price PDF"));
        }
        bytes.extend_from_slice(&chunk);
    }
    if !bytes.starts_with(b"%PDF-") {
        return Err(error(&url, "invalid or oversized price PDF"));
    }
    let len = bytes.len();
    let text = tokio::task::spawn_blocking(move || pdf_extract::extract_text_from_mem(&bytes))
        .await
        .map_err(|e| error(&url, format!("PDF task: {e}")))?
        .map_err(|e| error(&url, format!("PDF extraction: {e}")))?;
    let mut out = parse_text(&text, &url)?;
    out.status_code = status;
    out.byte_len = len;
    out.website_alive = true;
    Ok(out)
}

fn grade(label: &str) -> Option<(&'static str, &'static str)> {
    Some(match label {
        "Cu-Raff 95%" => ("kupfer-gemischt", "Cu-Raff 95%"),
        "Cu-Millberry" => ("kupfer-millberry", ""),
        "Cu.Haardraht" => ("kupfer-berry", "Cu.Haardraht"),
        "u-Kerze, sauber/ neu" => ("kupfer-berry", "Kerze, sauber/neu"),
        "Cu-Schienen, sauber" => ("kupfer-schwer", "Schienen, sauber"),
        "Cu Kabel ca. 38%" => ("kabel-kupfer", "ca. 38%"),
        "Ms-Milbe, 1% Anhaftung" => ("messing", "Milbe, 1% Anhaftung"),
        "Ms-Raff (Wasseruhren etc.)" => ("messing-leicht", "Raff (Wasseruhren etc.)"),
        "Ms-58" => ("messing", "Ms-58"),
        "V2A" => ("edelstahl-v2a", ""),
        "V2A Verhüttung" => ("edelstahl-v2a", "Verhüttung"),
        "V4A" => ("edelstahl-v4a", ""),
        "Mischschrott" => ("mischschrott", ""),
        "Shreddervormaterial" => ("stahlschrott-shredder", ""),
        "Leichte Schere" => ("stahlschrott-scheren", "Leichte Schere"),
        "Schwere Schere" => ("stahlschrott-scheren", "Schwere Schere"),
        "Guss" => ("eisenschrott-gussbruch", ""),
        "Bremsscheiben" => ("eisenschrott-gussbruch", "Bremsscheiben"),
        "Alt-Zink" => ("zink", "Alt-Zink"),
        "Neu-Zink" => ("zink", "Neu-Zink"),
        "Alt-Blei" => ("blei", ""),
        "Al-Profile, gemischt" => ("aluminium-profile", "gemischt"),
        "Al-Profile, Länge über 1m" => ("aluminium-profile", "Länge über 1m"),
        "Al-Blech ohne Anhaftung" => ("aluminium-blech", "ohne Anhaftung"),
        "Al-Geschirr, max. 5% FE" => ("aluminium-gemischt", "Geschirr, max. 5% FE"),
        "Al-Verhüttung" => ("aluminium-gemischt", "Verhüttung"),
        "Al-Kabel, dick" => ("kabel-alu", "dick"),
        "Al-Guss ohne Anhaftung" => ("aluminium-guss", "ohne Anhaftung"),
        "Al-Guss, 5% FE" => ("aluminium-guss", "5% FE"),
        "Al-Felgen mit Anhaftung" => ("aluminium-felgen", "mit Anhaftung"),
        "Al-Felgen ohne Anhaftung" => ("aluminium-felgen", "ohne Anhaftung"),
        "Al-Cu Kühler mit Anhaftung" => ("alu-cu-kuehler", "mit Anhaftung"),
        "Al-Cu Kühler ohne Anhaftung" => ("alu-cu-kuehler", "ohne Anhaftung"),
        "Al-Draht, ohne Anhaftungen" => ("aluminium-gemischt", "Draht ohne Anhaftungen"),
        "Akku-Blei, z.B. Autobatterien" => ("batterien-blei", "Autobatterien"),
        "Sammelzinn" => ("zinn", "Sammelzinn"),
        _ => return None,
    })
}

fn parse_text(text: &str, url: &str) -> Result<HandlerOutcome, IngestError> {
    if !text.contains("PRIVATKUNDEN") || !text.contains("elno-container.de") {
        return Err(error(url, "not the ELNO private-customer price list"));
    }
    let mut out = HandlerOutcome {
        fetch_url: url.to_owned(),
        ..HandlerOutcome::default()
    };
    let mut keys = std::collections::HashSet::new();
    let lines: Vec<_> = text.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        if let Some((before, _)) = line.split_once(" €/Kg") {
            let Some((label, amount)) = before.trim().rsplit_once(char::is_whitespace) else {
                continue;
            };
            let mut label = label.split_whitespace().collect::<Vec<_>>().join(" ");
            if matches!(
                label.as_str(),
                "Anhaftungen" | "Anhaftung" | "Autobatterien"
            ) {
                if let Some(prefix) = index
                    .checked_sub(1)
                    .and_then(|previous| lines.get(previous))
                    .map(|line| line.trim())
                    .filter(|prefix| !prefix.is_empty())
                {
                    label = format!("{prefix} {label}");
                }
            }
            let Some((material, variant)) = grade(&label) else {
                out.skipped_labels
                    .push(format!("{label} (no exact catalog mapping)"));
                continue;
            };
            let price = parse_eur(amount)
                .filter(|price| price.is_finite() && *price > 0.0)
                .ok_or_else(|| error(url, format!("invalid/non-positive price: {line}")))?;
            if !keys.insert((material, variant)) {
                return Err(error(url, format!("duplicate grade: {label}")));
            }
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
                label: format!(
                    "{label}; Privatkunden; freibleibend/tagesabhängig; Einstufung vor Ort"
                ),
            });
        }
    }
    if out.prices.len() < 33 {
        return Err(error(
            url,
            format!(
                "incomplete ELNO price list: only {} mapped prices",
                out.prices.len()
            ),
        ));
    }
    if text.contains("Elektronikschrott") && text.contains("Auf Anfrage") {
        out.skipped_labels
            .push("Elektronikschrott (Platinen, Netzteile, Rechner etc.): auf Anfrage".to_owned());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actual_two_page_pdf_preserves_grades_units_and_excludes_fees() {
        let text = pdf_extract::extract_text_from_mem(include_bytes!("fixtures/elno-20261008.pdf"))
            .unwrap();
        let out = parse_text(&text, "https://www.elno-container.de/current.pdf").unwrap();
        assert_eq!(out.prices.len(), 36, "{:#?}\n{text}", out);
        assert_eq!(out.skipped_labels.len(), 3, "{:?}", out.skipped_labels);
        assert!(out.published_at.is_none());
        assert!(out.prices.iter().all(|p| p.unit == "EUR/kg"));
        assert!(out.prices.iter().all(|p| p.label.contains("Privatkunden")));
        let find = |material, variant| {
            out.prices
                .iter()
                .find(|p| p.material == material && p.variant == variant)
                .unwrap()
                .price
        };
        assert_eq!(find("kupfer-millberry", ""), 10.30);
        assert_eq!(find("mischschrott", ""), 0.130);
        assert_eq!(find("zink", "Alt-Zink"), 1.30);
        assert_eq!(find("zink", "Neu-Zink"), 1.40);
        assert_eq!(find("edelstahl-v2a", "Verhüttung"), 0.20);
        assert_eq!(find("alu-cu-kuehler", "mit Anhaftung"), 3.50);
        assert_eq!(find("alu-cu-kuehler", "ohne Anhaftung"), 3.70);
        assert_eq!(find("batterien-blei", "Autobatterien"), 0.275);
        assert_eq!(find("zinn", "Sammelzinn"), 5.0);
        assert_eq!(find("aluminium-gemischt", "Draht ohne Anhaftungen"), 1.95);
        let keys: std::collections::HashSet<_> =
            out.prices.iter().map(|p| (p.material, p.variant)).collect();
        assert_eq!(keys.len(), out.prices.len());
        assert!(!out
            .prices
            .iter()
            .any(|p| p.price == 89.0 || p.price == 290.0 || p.price == 280.0));
        assert!(out
            .skipped_labels
            .contains(&"Sorte 3 (no exact catalog mapping)".to_owned()));
        assert!(out
            .skipped_labels
            .contains(&"Cu-MS Kühler (no exact catalog mapping)".to_owned()));
        assert!(out
            .skipped_labels
            .iter()
            .any(|label| label.contains("auf Anfrage")));
        assert!(parse_text(
            "PRIVATKUNDEN\nelno-container.de\nMischschrott 0,00 €/Kg",
            "https://www.elno-container.de/current.pdf"
        )
        .is_err());
        assert!(parse_text(
            "PRIVATKUNDEN\nelno-container.de\nMischschrott 0,13 €/Kg",
            "https://www.elno-container.de/current.pdf"
        )
        .is_err());
    }

    #[test]
    fn operator_identity_is_checked_separately_from_pdf_text() {
        let impressum =
            "ELNO Container- und Dienstleistungs GmbH HRB 141261 B Tiefwerderweg 13 13597 Berlin";
        assert!(verify_operator(impressum).is_ok());
        assert!(verify_operator(&impressum.replace("HRB 141261 B", "HRB 999999 B")).is_err());
    }

    #[test]
    fn discovers_replaced_pdf_not_certificate_or_attachment_page() {
        let html = r#"<a href="/certificate.pdf">EfB Zertifikat</a>
            <a href="/preisliste_11-09/">Zur Preisliste</a>
            <a href="/wp-content/uploads/2027/01/new.pdf"><span>Unsere Altmetall-Preise</span> entdecken</a>
            <a href="/wp-content/uploads/2027/01/new.pdf">Unsere Altmetall-Preise</a>"#;
        assert_eq!(
            pdf_url(html).unwrap(),
            "https://www.elno-container.de/wp-content/uploads/2027/01/new.pdf"
        );
        assert!(pdf_url("<a href='/certificate.pdf'>EfB Zertifikat</a>").is_err());
        assert!(
            pdf_url("<a href='https://foreign.test/a.pdf'>Unsere Altmetall-Preise</a>").is_err()
        );
        assert!(pdf_url("<a href='/a.pdf'>Unsere Altmetall-Preise</a><a href='/b.pdf'>Unsere Altmetall-Preise</a>").is_err());
    }
}
