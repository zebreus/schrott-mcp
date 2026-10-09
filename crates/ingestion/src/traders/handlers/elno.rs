//! ELNO's private-customer PDF, rediscovered on every scrape.
use super::super::{fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice};
use crate::IngestError;
use scraper::{Html, Selector};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
    let text = extract(&bytes, &url).await?;
    let mut out = parse_text(&text, &url)?;
    out.status_code = status;
    out.byte_len = len;
    out.website_alive = true;
    Ok(out)
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

async fn extract(bytes: &[u8], url: &str) -> Result<String, IngestError> {
    // OS limits bound decompression memory/CPU; dropping the future kills the
    // same process (prlimit execs pdftotext), unlike a detached blocking task.
    let mut child = tokio::process::Command::new("/usr/bin/prlimit")
        .args([
            "--as=268435456",
            "--cpu=15",
            "--",
            "/usr/bin/pdftotext",
            "-raw",
            "-",
            "-",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| error(url, format!("PDF tools unavailable: {e}")))?;
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();
    let result = tokio::time::timeout(std::time::Duration::from_secs(20), async {
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
    let (_, output, stderr, status) = result;
    if !status.success() {
        return Err(error(
            url,
            format!("pdftotext failed: {}", String::from_utf8_lossy(&stderr)),
        ));
    }
    String::from_utf8(output).map_err(|e| error(url, format!("PDF text: {e}")))
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
            let (label, amount) = before
                .trim()
                .rsplit_once(char::is_whitespace)
                .unwrap_or(("", before.trim()));
            let mut label = label.split_whitespace().collect::<Vec<_>>().join(" ");
            if label.is_empty() && index >= 2 {
                label = format!("{} {}", lines[index - 2].trim(), lines[index - 1].trim());
            }
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
            if !valid_amount(amount) {
                return Err(error(url, format!("not an exact positive price: {line}")));
            }
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
    if out.prices.len() != 36 {
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

fn valid_amount(amount: &str) -> bool {
    let (whole, decimal) = amount.split_once(',').unwrap_or((amount, ""));
    let groups: Vec<_> = whole.split('.').collect();
    !whole.is_empty()
        && groups
            .iter()
            .all(|group| !group.is_empty() && group.chars().all(|c| c.is_ascii_digit()))
        && (groups.len() == 1
            || (groups[0].len() <= 3 && groups[1..].iter().all(|group| group.len() == 3)))
        && (!amount.contains(',')
            || (!decimal.is_empty() && decimal.chars().all(|c| c.is_ascii_digit())))
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

    #[tokio::test]
    async fn changed_quote_tokens_and_units_are_not_silently_accepted() {
        let original = extract(include_bytes!("fixtures/elno-20261008.pdf"), URL)
            .await
            .unwrap();
        for replacement in [
            "-10,30 €/Kg",
            "10,30-11,00 €/Kg",
            "10,30 €/t",
            "10,30 €/kg",
            "Auf Anfrage",
        ] {
            let changed = original.replacen("10,30 €/Kg", replacement, 1);
            assert_ne!(changed, original, "fixture mutation did not reach quote");
            assert!(
                parse_text(&changed, URL).is_err(),
                "accepted changed quote: {replacement}"
            );
        }
    }

    #[tokio::test]
    async fn actual_two_page_pdf_preserves_grades_units_and_excludes_fees() {
        let text = extract(include_bytes!("fixtures/elno-20261008.pdf"), URL)
            .await
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
