//! Heraeus Goldstore Hanau: 24 visible EUR/g Ankauf-Richtwerte.
//! Shop operator (HMG, HRA 93163) and buyer (AHD, HRB 505728) are
//! separate entities. Reviewed contract PDFs are pinned, not equated.
use scraper::{ElementRef, Html, Selector};
use sha2::{Digest, Sha256};

use super::super::{parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice};
use crate::IngestError;

pub const SLUG: &str = "he-hanau-heraeus-goldstore-hanau-argor-heraeus-de";
pub const URL: &str = "https://store.heraeus-gold.com/ankauf/";
pub const IMPRESSUM_URL: &str = "https://store.heraeus-gold.com/impressum/";
const AGB_URL: &str = "https://azweust1ieh606.blob.core.windows.net/azweusc1wjo540/media/43/a1/51/1737043797/AGB-Edelmetallankauf-Goldstore-Hanau.pdf";
const FORM_URL: &str = "https://azweust1ieh606.blob.core.windows.net/azweusc1wjo540/media/d4/4d/97/1737626990/Einreichungsformular-Edelmetalle-Goldstore-Hanau.pdf";
const AGB_SHA256: &str = "8f89a7bfe591addb9ccc2b42a1bc6dc59466fa6aa1195540adaa8dd6af2faa40";
const FORM_SHA256: &str = "3a791f306c7e6051f2423df6a7660b25f5d59d17578539e1b9ddf7208d1170a7";
const QUALIFIED_RATES: &str = include_str!("fixtures/heraeus-goldstore-hanau-rates.html");
const QUALIFIED_IMPRESSUM: &str = include_str!("fixtures/heraeus-goldstore-hanau-impressum.html");
const CONDITIONS: &str = "Unverbindlicher Ankauf-Richtwert; mindestens 10 g Edelmetall; endgültiges Angebot nach Analyse zum Angebotstagskurs. Ankauf eingeschränkt, Bearbeitung bis zu drei Wochen. Nur volljährige Verbraucher mit EU-/UK-Staatsangehörigkeit; persönliche Abgabe im Heraeus Goldstore Hanau, Rodenbacher Weg 2. Barren ausschließlich LBMA-zertifizierte Edelmetallbarren; keine Barren von Agr, Elemetal Refining, Emeko, Goldas Switzerland, Interbee, Kaloti, Republic Metals Corporation oder Tony Goetz. Nicht angenommen: Silberkorpusware und Silbermesser; versilberte oder vergoldete Ware; Dentalgaumenplatten; Halbzeuge (Metallplatten, Drähte), Feilungen (Metallspäne, Abrieb), Gussreste und Scheidgut (gemischte/verunreinigte Edelmetallabfälle). Wenn Analyse unmöglich oder bei Ablehnung des Angebots: 59,50 EUR inkl. MwSt. Analyse-/Rücktransportgebühr (kein Ankaufpreis). Vertragspartner: Argor-Heraeus Deutschland GmbH, Wurmberg, HRB 505728 Mannheim; Shopbetreiber: Heraeus Metals Germany GmbH & Co. KG, HRA 93163 Hanau.";

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |client| Box::pin(scrape(client)),
    }
}

fn error(url: &str, detail: &str) -> IngestError {
    IngestError::Parse {
        url: url.into(),
        detail: detail.into(),
    }
}

/// Private type ensures fetch cannot accidentally receive the redirect-following
/// scheduler client. Only this handler-local constructor creates it.
struct SourceClient(reqwest::Client);

fn source_client() -> Result<SourceClient, IngestError> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("schrott-mcp-ingestion/0.1")
        .build()
        .map(SourceClient)
        .map_err(|source| IngestError::Fetch {
            url: URL.into(),
            source,
        })
}

/// Inspect explicit HTML visibility only. No browser/external stylesheet
/// computation: unfamiliar CSS rules need renewed source review. A legitimate
/// Bootstrap FAQ accordion is expandable evidence, not a withdrawn price.
fn visible(el: ElementRef<'_>, faq: bool, url: &str) -> Result<(), IngestError> {
    for ancestor in std::iter::once(el).chain(el.ancestors().filter_map(ElementRef::wrap)) {
        let value = ancestor.value();
        let classes: Vec<_> = value.classes().collect();
        let style = value.attr("style").unwrap_or("").to_ascii_lowercase();
        let hidden_style = style
            .split(';')
            .filter_map(|declaration| declaration.split_once(':'))
            .any(|(key, val)| {
                let key = key.trim();
                let val = val.split('!').next().unwrap_or("").trim();
                matches!(
                    (key, val),
                    ("display", "none")
                        | ("visibility", "hidden" | "collapse")
                        | ("content-visibility", "hidden")
                ) || (key == "opacity" && val.parse::<f64>().ok() == Some(0.0))
            });
        if value.attr("hidden").is_some()
            || value
                .attr("aria-hidden")
                .is_some_and(|v| v.trim().eq_ignore_ascii_case("true"))
            || classes.iter().any(|class| {
                matches!(
                    *class,
                    "hidden" | "visually-hidden" | "sr-only" | "invisible"
                ) || (*class == "d-none" || (class.starts_with("d-") && class.ends_with("-none")))
            })
            || (!faq && classes.contains(&"collapse") && !classes.contains(&"show"))
            || hidden_style
        {
            return Err(error(url, "Required evidence explicitly hidden"));
        }
    }
    Ok(())
}

fn visible_block(el: ElementRef<'_>, faq: bool, url: &str) -> Result<(), IngestError> {
    visible(el, faq, url)?;
    for node in el.descendants() {
        if node.value().as_text().is_some_and(|t| !t.trim().is_empty()) {
            if let Some(parent) = node.parent().and_then(ElementRef::wrap) {
                visible(parent, faq, url)?;
            }
        }
    }
    Ok(())
}

/// Exact normalized non-listing main text, not a bag of keyword fragments.
/// Includes complete conditions/FAQ, headings, branch and notices even outside
/// expected classes. Only the variable product-listing subtree and technical
/// CSS/JS are excluded; wrapper siblings remain pinned. Conservative: even
/// unrelated main copy edits require requalification.
fn excluded_semantic_ancestor(el: ElementRef<'_>, allow_listing: bool) -> bool {
    matches!(el.value().name(), "script" | "style")
        || (allow_listing
            && el
                .value()
                .classes()
                .any(|c| c == "cms-element-product-listing"))
}

fn semantic_text(main: ElementRef<'_>, allow_listing: bool) -> String {
    main.descendants()
        .filter_map(|node| {
            let text = node.value().as_text()?;
            let technical = node
                .ancestors()
                .filter_map(ElementRef::wrap)
                .any(|el| excluded_semantic_ancestor(el, allow_listing));
            (!technical).then_some(text.as_ref())
        })
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Validate FAQ content only when it retains the qualified accordion structure.
/// A class name alone must not let arbitrary condition text claim the exception
/// for a hidden Bootstrap collapse. Match each exact heading/body pair against
/// the reviewed source and require each body to belong to one FAQ card.
fn faq_signatures(main: ElementRef<'_>, url: &str) -> Result<Vec<(String, String)>, IngestError> {
    let body_selector = Selector::parse(".netzp-powerpack6-collapse-body").unwrap();
    let body_count = main.select(&body_selector).count();
    if body_count == 0 {
        return Ok(Vec::new());
    }

    let blocks: Vec<_> = main
        .select(&Selector::parse(".cms-block-netzp-powerpack6-collapse").unwrap())
        .collect();
    if blocks.len() != 1 {
        return Err(error(url, "Reviewed FAQ structure missing or ambiguous"));
    }
    let elements: Vec<_> = blocks[0]
        .select(&Selector::parse(".netzp-powerpack6-collapse-element").unwrap())
        .collect();
    if elements.len() != 1 {
        return Err(error(url, "Reviewed FAQ accordion missing or ambiguous"));
    }

    let cards: Vec<_> = elements[0]
        .select(&Selector::parse(".card").unwrap())
        .collect();
    if cards.is_empty() || cards.len() != body_count {
        return Err(error(url, "FAQ body is not contained in a reviewed card"));
    }

    let mut signatures = Vec::with_capacity(cards.len());
    for card in cards {
        let headers: Vec<_> = card
            .select(&Selector::parse(".card-header").unwrap())
            .collect();
        let collapses: Vec<_> = card
            .select(&Selector::parse(".collapse").unwrap())
            .collect();
        let all_bodies = card.select(&body_selector).count();
        let collapsed_bodies: Vec<_> = card
            .select(&Selector::parse(".collapse .netzp-powerpack6-collapse-body").unwrap())
            .collect();
        if headers.len() != 1
            || collapses.len() != 1
            || all_bodies != 1
            || collapsed_bodies.len() != 1
        {
            return Err(error(url, "FAQ card structure changed"));
        }
        let heading = text(headers[0]);
        let body = text(collapsed_bodies[0]);
        if heading.is_empty() || body.is_empty() {
            return Err(error(url, "FAQ question or answer is empty"));
        }
        signatures.push((heading, body));
    }
    Ok(signatures)
}

/// Check visibility for all pinned semantic text, not just nodes that still
/// carry their expected CMS classes. Otherwise changing a condition block's
/// class can remove it from the narrower selector checks while preserving its
/// text and allowing stale conditions to be published. Only the variable
/// product-listing subtree and script/style are skipped; wrapper siblings are
/// semantic evidence and stay visibility-checked.
fn visible_semantic_text(
    main: ElementRef<'_>,
    url: &str,
    allow_listing: bool,
) -> Result<(), IngestError> {
    for node in main.descendants() {
        if !node
            .value()
            .as_text()
            .is_some_and(|text| !text.trim().is_empty())
        {
            continue;
        }

        let ancestors: Vec<_> = node.ancestors().filter_map(ElementRef::wrap).collect();
        let technical = ancestors
            .iter()
            .any(|el| excluded_semantic_ancestor(*el, allow_listing));
        if technical {
            continue;
        }

        let faq = ancestors.iter().any(|el| {
            el.value()
                .classes()
                .any(|class| class == "netzp-powerpack6-collapse-body")
        });
        if let Some(parent) = node.parent().and_then(ElementRef::wrap) {
            visible(parent, faq, url)?;
        }
    }
    Ok(())
}

fn qualify_main(
    main: ElementRef<'_>,
    qualified: &str,
    url: &str,
    allow_listing: bool,
) -> Result<(), IngestError> {
    let reference = Html::parse_document(qualified);
    let expected = one(reference.root_element(), "main#content-main")?;
    if semantic_text(main, allow_listing) != semantic_text(expected, allow_listing) {
        return Err(error(
            url,
            "Complete current source text changed; requalify conditions/identity/direction",
        ));
    }
    if faq_signatures(main, url)? != faq_signatures(expected, url)? {
        return Err(error(url, "Reviewed FAQ questions/answers changed"));
    }
    visible_semantic_text(main, url, allow_listing)?;
    visible(main, false, url)
}

fn text(el: ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn one<'a>(el: ElementRef<'a>, selector: &str) -> Result<ElementRef<'a>, IngestError> {
    let selector = Selector::parse(selector).expect("selector");
    let mut elements = el.select(&selector);
    let first = elements
        .next()
        .ok_or_else(|| error(URL, "Required source element missing"))?;
    if elements.next().is_some() {
        return Err(error(URL, "Ambiguous source element"));
    }
    Ok(first)
}

/// Fail closed on redirects and oversized bodies, including identity PDFs.
async fn fetch(client: &SourceClient, url: &str) -> Result<(u16, Vec<u8>), IngestError> {
    let mut res = client
        .0
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.into(),
            source,
        })?;
    let status = res.status().as_u16();
    if !res.status().is_success() || res.url().as_str() != url {
        return Err(error(url, "HTTP failure or unreviewed redirect"));
    }
    const MAX: usize = 2_000_000;
    if res.content_length().is_some_and(|len| len > MAX as u64) {
        return Err(error(url, "Source body too large"));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = res.chunk().await.map_err(|source| IngestError::Body {
        url: url.into(),
        source,
    })? {
        if bytes.len().saturating_add(chunk.len()) > MAX {
            return Err(error(url, "Source body too large"));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok((status, bytes))
}

fn verify_contract(bytes: &[u8], url: &str, hash: &str) -> Result<(), IngestError> {
    if !bytes.starts_with(b"%PDF-") || format!("{:x}", Sha256::digest(bytes)) != hash {
        return Err(error(
            url,
            "Ankauf contract changed; buyer identity must be requalified",
        ));
    }
    Ok(())
}

async fn scrape(_client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    // Handler-local policy; do not mutate the scheduler's shared client.
    let client = source_client()?;
    let (status, bytes) = fetch(&client, URL).await?;
    let html = std::str::from_utf8(&bytes).map_err(|_| error(URL, "Non-UTF8 price source"))?;
    let mut out = parse(html)?;
    let (_, impressum) = fetch(&client, IMPRESSUM_URL).await?;
    verify_operator(
        std::str::from_utf8(&impressum).map_err(|_| error(IMPRESSUM_URL, "Non-UTF8 Impressum"))?,
    )?;
    // The linked contract explicitly names AHD as buyer at this HMG-operated
    // Goldstore. Every byte change requires renewed primary-source review.
    for (url, hash) in [(AGB_URL, AGB_SHA256), (FORM_URL, FORM_SHA256)] {
        let (_, pdf) = fetch(&client, url).await?;
        verify_contract(&pdf, url, hash)?;
    }
    out.status_code = status;
    out.byte_len = bytes.len();
    out.website_alive = true;
    Ok(out)
}

fn verify_operator(html: &str) -> Result<(), IngestError> {
    let doc = Html::parse_document(html);
    let main = one(doc.root_element(), "main#content-main")?;
    // The Impressum has no separately validated variable-price listing: all
    // nontechnical identity text must participate in qualification.
    qualify_main(main, QUALIFIED_IMPRESSUM, IMPRESSUM_URL, false)?;
    let blocks = Selector::parse("main#content-main .cms-element-text").unwrap();
    let owners: Vec<_> = doc
        .select(&blocks)
        .filter(|el| {
            el.select(&Selector::parse("h4").unwrap())
                .any(|h| text(h) == "Verantwortlich für den Shop")
        })
        .collect();
    if owners.len() != 1 {
        return Err(error(
            IMPRESSUM_URL,
            "Reviewed shop operator missing or changed",
        ));
    }
    visible_block(owners[0], false, IMPRESSUM_URL)?;
    // No enrichment: the shop's legal seat is not the drop-off address.
    Ok(())
}

fn grade(label: &str) -> Option<(&'static str, &'static str)> {
    Some(match label {
        "Gold (333)" => ("gold", "333"),
        "Gold (375)" => ("gold", "375"),
        "Gold (585)" => ("gold", "585"),
        "Gold (750)" => ("gold", "750"),
        "Gold (900)" => ("gold", "900"),
        "Gold (916)" => ("gold", "916"),
        "Gold (986)" => ("gold", "986"),
        "Gold (999)" => ("gold", "999"),
        "Gold (999,9)" => ("gold", "999,9"),
        "Palladium (500)" => ("palladium", "500"),
        "Palladium (950)" => ("palladium", "950"),
        "Palladium (999)" => ("palladium", "999"),
        "Platin (600)" => ("platin", "600"),
        "Platin (750)" => ("platin", "750"),
        "Platin (950)" => ("platin", "950"),
        "Platin (999)" => ("platin", "999"),
        "Silber (625)" => ("silber", "625"),
        "Silber (800)" => ("silber", "800"),
        "Silber (835)" => ("silber", "835"),
        "Silber (900)" => ("silber", "900"),
        "Silber (925)" => ("silber", "925"),
        "Silber (999)" => ("silber", "999"),
        "Zahngold (500)" => ("zahngold", "500"),
        "Zahngold (700)" => ("zahngold", "700"),
        _ => return None,
    })
}

fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let main = one(doc.root_element(), "main#content-main")?;
    // Only this page validates the unique excluded listing completely below.
    qualify_main(main, QUALIFIED_RATES, URL, true)?;
    // Qualify visibility of the complete relevant blocks, including nested
    // facts and titles. FAQ .collapse is allowed only for reviewed FAQ bodies.
    for block in main
        .select(&Selector::parse(".cms-element-text, .netzp-powerpack6-collapse-body, h1").unwrap())
    {
        let faq = block
            .value()
            .classes()
            .any(|c| c == "netzp-powerpack6-collapse-body");
        visible_block(block, faq, URL)?;
    }
    for expected in [AGB_URL, FORM_URL] {
        let links: std::collections::HashSet<_> = main
            .select(&Selector::parse("a[href]").unwrap())
            .filter_map(|a| a.value().attr("href"))
            .filter(|href| href.split('?').next() == Some(expected))
            .collect();
        if links.is_empty() {
            return Err(error(URL, "Reviewed contract link missing"));
        }
        for link in main
            .select(&Selector::parse("a[href]").unwrap())
            .filter(|a| {
                a.value()
                    .attr("href")
                    .is_some_and(|href| href.split('?').next() == Some(expected))
            })
        {
            visible(
                link,
                link.ancestors().filter_map(ElementRef::wrap).any(|el| {
                    el.value()
                        .classes()
                        .any(|c| c == "netzp-powerpack6-collapse-body")
                }),
                URL,
            )?;
        }
    }
    let listing = one(main, ".cms-element-product-listing")?;
    visible(listing, false, URL)?;
    // Non-card notices inside the otherwise price-varying listing must not
    // bypass the non-listing snapshot qualification. Stop at the listing root
    // so its classes cannot claim the exemption; only matching descendant
    // cards have their complete text separately checked below.
    let listing_prose = listing
        .descendants()
        .filter_map(|node| {
            let data = node.value().as_text()?;
            let excluded = node
                .ancestors()
                .filter_map(ElementRef::wrap)
                .take_while(|el| {
                    !el.value()
                        .classes()
                        .any(|class| class == "cms-element-product-listing")
                })
                .any(|el| {
                    matches!(el.value().name(), "script" | "style")
                        || (el.value().classes().any(|c| c == "card")
                            && el.value().classes().any(|c| c == "product-box"))
                });
            (!excluded).then_some(data.as_ref())
        })
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if !matches!(
        listing_prose.as_str(),
        "" | "Sortierung: Name A-Z Preis aufsteigend Preis absteigend Topseller"
    ) {
        return Err(error(URL, "Unreviewed notice inside Ankauf listing"));
    }
    let cards: Vec<_> = listing
        .select(&Selector::parse(".card.product-box").unwrap())
        .collect();
    if cards.len() != 24 {
        return Err(error(URL, "Expected complete 24-card Ankauf snapshot"));
    }
    let mut prices = Vec::new();
    for card in cards {
        visible(card, false, URL)?;
        let name = one(card, ".product-name")?;
        visible_block(name, false, URL)?;
        let label = text(name);
        let (material, variant) =
            grade(&label).ok_or_else(|| error(URL, "Unreviewed Ankauf grade"))?;
        let purchase = one(card, ".e-purchase-listing-product-box-price")?;
        visible(purchase, false, URL)?;
        let unit = one(purchase, ".e-component-product-box-price-note")?;
        visible_block(unit, false, URL)?;
        if text(unit) != "pro Gramm" {
            return Err(error(URL, "Ankauf unit is not per gram"));
        }
        let price_element = one(purchase, ".product-price")?;
        visible_block(price_element, false, URL)?;
        let raw = text(price_element);
        // Only known shop boilerplate can accompany the Ankauf quote. Do not
        // silently ignore an added withdrawal, exclusion or sale-direction
        // notice inside a card merely because its label/price still exist.
        let card_text = text(card);
        let known = [
            format!("{label} {raw} pro Gramm"),
            format!("{label} Regulärer Preis: {raw} pro Gramm"),
        ];
        let suffixes = [
            "",
            " * Preise inkl. MwSt. zzgl. Versandkosten",
            " * Preise inkl. MwSt. zzgl. Versandkosten Anlagegold gemäß § 25c UStG ohne MwSt.",
        ];
        if !known.iter().any(|prefix| {
            suffixes
                .iter()
                .any(|suffix| card_text == format!("{prefix}{suffix}"))
        }) {
            return Err(error(URL, "Unreviewed notice inside Ankauf card"));
        }
        let number = raw
            .strip_suffix('€')
            .map(str::trim)
            .ok_or_else(|| error(URL, "Price is not EUR"))?;
        if !number.split_once(',').is_some_and(|(whole, cents)| {
            !whole.is_empty()
                && whole.chars().all(|c| c.is_ascii_digit())
                && cents.len() == 2
                && cents.chars().all(|c| c.is_ascii_digit())
        }) {
            return Err(error(URL, "Malformed visible Ankauf price"));
        }
        let price = parse_eur(number)
            .filter(|p| p.is_finite() && *p > 0.0)
            .ok_or_else(|| error(URL, "Nonpositive Ankauf price"))?;
        if prices
            .iter()
            .any(|p: &ScrapedPrice| p.material == material && p.variant == variant)
        {
            return Err(error(URL, "Duplicate Ankauf grade"));
        }
        prices.push(ScrapedPrice {
            material,
            variant: variant.into(),
            price,
            currency: "EUR",
            unit: "EUR/g",
            price_kind: "approx",
            price_min: None,
            price_max: None,
            confidence: Some(0.8),
            label: format!("{label}; {CONDITIONS}"),
        });
    }
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
        prices,
        acceptances: ["gold", "silber", "platin", "palladium", "zahngold"]
            .into_iter()
            .map(|material| ScrapedAcceptance {
                material,
                conditions: CONDITIONS.into(),
                label: "Edelmetallankauf Preise / Annahmebedingungen".into(),
            })
            .collect(),
        fetch_url: URL.into(),
        // No page-stated price date. Media URLs, PDF metadata and HTTP dates
        // are not evidence of rate publication; record() keeps repo inference.
        published_at: None,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const RATES: &str = include_str!("fixtures/heraeus-goldstore-hanau-rates.html");
    const IMPRESSUM: &str = include_str!("fixtures/heraeus-goldstore-hanau-impressum.html");

    #[test]
    fn complete_visible_rates_have_precise_catalog_grades_and_conditions() {
        let out = parse(RATES).unwrap();
        let expected = [
            34.73, 39.24, 61.81, 79.55, 97.83, 99.57, 110.81, 113.97, 114.08, 13.97, 23.89, 25.12,
            24.78, 29.27, 35.31, 37.15, 0.86, 1.10, 1.15, 1.23, 1.30, 1.40, 49.74, 70.18,
        ];
        assert_eq!(out.prices.len(), 24);
        assert_eq!(
            out.prices.iter().map(|p| p.price).collect::<Vec<_>>(),
            expected
        );
        assert_eq!(
            (out.prices[22].material, out.prices[22].variant.as_ref()),
            ("zahngold", "500")
        );
        assert_eq!(
            (out.prices[23].material, out.prices[23].variant.as_ref()),
            ("zahngold", "700")
        );
        assert_eq!(out.prices[8].variant, "999,9");
        assert!(out.prices.iter().all(|p| p.currency == "EUR"
            && p.unit == "EUR/g"
            && p.price_kind == "approx"
            && p.price_min.is_none()
            && p.price_max.is_none()
            && p.label.contains(CONDITIONS)));
        assert!(out.skipped_labels.is_empty());
        assert_eq!(out.published_at, None);
        assert_eq!(out.fetch_url, URL);
        assert_eq!(out.acceptances.len(), 5);
        assert!(out.trader_info.is_empty());
    }

    #[test]
    fn malformed_partial_duplicate_unknown_or_shop_snapshots_fail_closed() {
        for html in [
            "".to_owned(),
            RATES.replace("Gold  (333)", "Gold (375)"),
            RATES.replace("Gold  (333)", "Gold (334)"),
            RATES.replace("pro Gramm", "pro Kilogramm"),
            RATES.replace("€", "USD"),
            RATES.replace("e-purchase-listing-product-box-price", "shop-price"),
            RATES.replace("Edelmetallankauf Preise", "Shopverkauf"),
            RATES.replace("unverbindliche Richtwerte", "Festpreis"),
            RATES.replace("Rodenbacher Weg 2", "Andere Filiale"),
            RATES.replace(FORM_URL, "https://example.org/form.pdf"),
            RATES.replace(
                "Argor-Heraeus Deutschland GmbH",
                "Heraeus Metals Germany GmbH &amp; Co. KG",
            ),
            RATES.replace("card product-box", "card other-box"),
        ] {
            assert!(parse(&html).is_err());
        }
        for bad in [
            "-34,73",
            "0,00",
            "34.73",
            "bis zu 34,73",
            "34,73-40,00",
            "34,73,5",
            "auf Anfrage",
        ] {
            assert!(parse(&RATES.replace("34,73", bad)).is_err(), "{bad}");
        }
        let extra =
            "<div class='card product-box'><div class='product-name'>Gold (999)</div></div>";
        assert!(parse(&RATES.replacen(
            "<div class=\"card product-box box-standard\"",
            &format!("{extra}<div class=\"card product-box box-standard\""),
            1
        ))
        .is_err());
        assert!(
            parse(&RATES.replace("<span class=\"product-price\">34,73&nbsp;€</span>", "")).is_err()
        );
    }

    #[test]
    fn separate_operator_and_buyer_identity_are_not_equated() {
        verify_operator(IMPRESSUM).unwrap();
        assert!(verify_operator(&IMPRESSUM.replace("HRA 93163", "HRB 505728")).is_err());
        assert!(verify_operator(
            &IMPRESSUM.replace("Heraeus Metals Germany", "Argor-Heraeus Deutschland")
        )
        .is_err());
        assert!(
            verify_operator(&IMPRESSUM.replace("Verantwortlich für den Shop", "Footer")).is_err()
        );
        assert!(verify_contract(b"%PDF-unreviewed", FORM_URL, FORM_SHA256).is_err());
        assert!(verify_contract(b"not a pdf", AGB_URL, AGB_SHA256).is_err());
        let reviewed = b"%PDF-fixture";
        let hash = format!("{:x}", Sha256::digest(reviewed));
        verify_contract(reviewed, FORM_URL, &hash).unwrap();
        assert!(verify_contract(b"%PDF-fixture-changed", FORM_URL, &hash).is_err());
    }

    #[test]
    fn impressum_cannot_hide_identity_changes_in_a_product_listing() {
        verify_operator(IMPRESSUM).unwrap();
        for attributes in ["", " hidden", " style=\"display:none\""] {
            let changed = IMPRESSUM.replace(
                "</main>",
                &format!("<div class=\"cms-element-product-listing\"{attributes}>Neuer Betreiber: Beispiel GmbH. Heraeus Metals Germany ist nicht mehr Shopbetreiber.</div></main>"),
            );
            assert!(
                verify_operator(&changed).is_err(),
                "unchecked Impressum listing accepted"
            );
        }
    }

    #[test]
    fn not_registered_while_operator_says_buying_is_stopped() {
        let handlers = super::super::all();
        assert!(handlers.iter().all(|h| h.slug != SLUG));
    }

    #[test]
    fn b1_changed_complete_conditions_cannot_publish_old_acceptances() {
        parse(RATES).unwrap();
        for (old, new) in [
            ("59,50 €", "159,50 €"),
            ("Das kaufen wir nicht an", "Das kaufen wir jetzt an"),
            ("Silber Korpusware und Silbermesser", "Silber nur ab 1000 Gramm"),
            ("Das kaufen wir an", "Gold kaufen wir nicht mehr an. Das kaufen wir an"),
            ("<li>Dental Gaumenplatten</li>", "<li>Dental Gaumenplatten</li><li>Gold unter 100 g</li>"),
            ("Edelmetall mit einem Gewicht unter 10g", "Kein Ausschluss: Edelmetall mit einem Gewicht unter 10g"),
            ("Der Ankauf von Altgold ist derzeit nur eingeschränkt möglich.", "Der Ankauf ist ausgesetzt. Historisch: Der Ankauf von Altgold ist derzeit nur eingeschränkt möglich."),
            ("Tony Goetz", "Tony Goetz, weiterer ausgeschlossener Hersteller"),
            ("<main id=\"content-main\">", "<main id=\"content-main\"><p>Gold kaufen wir nicht mehr an.</p>"),
            ("<div class=\"cms-element-product-listing\">", "<div class=\"cms-element-product-listing\"><p>Ankauf ausgesetzt; alle Werte Verkaufspreise.</p>"),
            ("<div class=\"product-name\">Gold  (333)</div>", "<div class=\"product-name\">Gold  (333)</div><p>Gold kaufen wir nicht mehr an.</p>"),
        ] {
            assert!(RATES.contains(old), "fixture missing {old}");
            assert!(parse(&RATES.replace(old, new)).is_err(), "B1 accepted {new}");
        }
    }

    #[test]
    fn b1_listing_wrapper_notices_are_pinned() {
        let listing = "<div class=\"cms-element-product-listing\"";
        let index = RATES.find(listing).expect("fixture listing");
        let mut mutated = RATES.to_owned();
        mutated.insert_str(
            index,
            "<p>ANKAUF AUSGESETZT: Alle folgenden Werte sind Verkaufspreise.</p>",
        );
        assert!(
            parse(&mutated).is_err(),
            "B1 accepted a contradictory notice outside the listing but inside its wrapper"
        );
    }

    #[test]
    fn b1_only_reviewed_product_cards_may_hide_listing_text() {
        let listing_row = "<div class=\"row cms-listing-row js-listing-wrapper\"";
        let index = RATES.find(listing_row).expect("fixture listing row");
        let mut mutated = RATES.to_owned();
        mutated.insert_str(
            index,
            "<span class=\"product-box\" style=\"display:inline !important\">ANKAUF AUSGESETZT: Alle folgenden Werte sind Verkaufspreise.</span>",
        );
        assert!(
            parse(&mutated).is_err(),
            "B1 accepted unreviewed listing prose disguised with product-box class"
        );
    }

    #[test]
    fn b1_listing_root_cannot_claim_the_reviewed_card_exemption() {
        let root = "<div class=\"cms-element-product-listing\"";
        let marked_root = RATES.replacen(
            root,
            "<div class=\"cms-element-product-listing card product-box\"",
            1,
        );
        let root_start = marked_root
            .find("<div class=\"cms-element-product-listing card product-box\"")
            .expect("product listing root");
        let listing_content = root_start
            + marked_root[root_start..]
                .find('>')
                .expect("product listing opening")
            + 1;
        let mut injected = marked_root;
        injected.insert_str(
            listing_content,
            "<p>ANKAUF AUSGESETZT: Alle folgenden Werte sind Verkaufspreise.</p>",
        );

        assert!(
            parse(&injected).is_err(),
            "B1 accepted listing prose after the listing root claimed the card exemption"
        );
    }

    #[test]
    fn b2_non_faq_condition_cannot_claim_the_faq_collapse_exception() {
        let old = "<div class=\"cms-element-text\"><p>Das kaufen wir nicht an";
        let new = "<div class=\"cms-element-text collapse netzp-powerpack6-collapse-body\"><p>Das kaufen wir nicht an";
        assert!(RATES.contains(old), "fixture condition block");
        assert!(
            parse(&RATES.replace(old, new)).is_err(),
            "B2 accepted a condition block disguised as a collapsed FAQ body"
        );
    }

    #[test]
    fn b2_hidden_prices_identity_and_ancestors_fail_but_faq_collapse_is_valid() {
        parse(RATES).unwrap(); // Legitimate .collapse FAQs stay available.
        for (old, new) in [
            (
                "class=\"e-purchase-listing-product-box-price\"",
                "class=\"e-purchase-listing-product-box-price\" hidden",
            ),
            ("class=\"product-price\"", "class=\"product-price\" hidden"),
            (
                "class=\"cms-element-product-listing\"",
                "class=\"cms-element-product-listing\" hidden",
            ),
            (
                "class=\"cms-element-product-listing-wrapper\"",
                "class=\"cms-element-product-listing-wrapper\" style=\"display: none !important\"",
            ),
            (
                "class=\"cms-element-product-listing-wrapper\"",
                "class=\"cms-element-product-listing-wrapper d-none\"",
            ),
            (
                "<main id=\"content-main\">",
                "<main id=\"content-main\" style=\"visibility: hidden\">",
            ),
            ("<h1>Altgold Ankauf", "<h1 hidden>Altgold Ankauf"),
            (
                "class=\"rhweb-heading-cta\"",
                "class=\"rhweb-heading-cta visually-hidden\"",
            ),
            (
                "<li>Silber Korpusware",
                "<li aria-hidden=\"true\">Silber Korpusware",
            ),
            (
                "<div class=\"cms-element-text\"><p>Das kaufen wir nicht an",
                "<div class=\"hidden\"><p>Das kaufen wir nicht an",
            ),
        ] {
            assert!(RATES.contains(old));
            assert!(
                parse(&RATES.replace(old, new)).is_err(),
                "B2 accepted {new}"
            );
        }
        for (old, new) in [
            (
                "<main id=\"content-main\">",
                "<main id=\"content-main\" hidden>",
            ),
            (
                "<div class=\"cms-element-text\">\n<h4>",
                "<div class=\"cms-element-text\" hidden>\n<h4>",
            ),
            (
                "<p>Heraeus Metals Germany",
                "<p style=\"opacity:0\">Heraeus Metals Germany",
            ),
        ] {
            assert!(IMPRESSUM.contains(old));
            assert!(
                verify_operator(&IMPRESSUM.replace(old, new)).is_err(),
                "B2 accepted operator {new}"
            );
        }
    }

    #[test]
    fn b3_current_identity_branch_and_purchase_direction_not_historical_fragments() {
        for (old, new) in [
            (
                "<p>Heraeus Metals Germany",
                "<p>Neuer Betreiber: Beispiel GmbH. Früher: Heraeus Metals Germany",
            ),
            (
                "<h4>Verantwortlich für den Shop</h4>",
                "<h4>Verantwortlich für den Shop</h4><p>Neuer Betreiber: Beispiel GmbH</p>",
            ),
        ] {
            assert!(
                verify_operator(&IMPRESSUM.replace(old, new)).is_err(),
                "B3 accepted {new}"
            );
        }
        for (old, new) in [
            ("im Rodenbacher Weg 2 in Hanau", "Nicht mehr Rodenbacher Weg 2: Abgabe jetzt in Frankfurt"),
            ("Edelmetallankauf Preise</h1>", "Edelmetallankauf Preise</h1><p>Kein Ankauf mehr: alle folgenden Werte sind Verkaufspreise.</p>"),
            ("Altgold Ankauf in Hanau: Gold und andere Edelmetalle verkaufen", "Früher: Altgold Ankauf in Hanau: Gold und andere Edelmetalle verkaufen"),
        ] {
            assert!(RATES.contains(old));
            assert!(parse(&RATES.replace(old, new)).is_err(), "B3 accepted {new}");
        }
    }

    #[test]
    fn b4_material_customer_and_both_fee_triggers_are_explicitly_preserved() {
        let out = parse(RATES).unwrap();
        for required in [
            "Silberkorpusware",
            "Silbermesser",
            "versilberte",
            "vergoldete",
            "Dentalgaumenplatten",
            "Halbzeuge",
            "Feilungen",
            "Gussreste",
            "Scheidgut",
            "LBMA-zertifizierte",
            "Agr",
            "Elemetal Refining",
            "Emeko",
            "Goldas Switzerland",
            "Interbee",
            "Kaloti",
            "Republic Metals Corporation",
            "Tony Goetz",
            "volljährige Verbraucher",
            "EU-/UK-Staatsangehörigkeit",
            "persönliche Abgabe",
            "Analyse unmöglich",
            "Ablehnung",
            "59,50 EUR",
            "mindestens 10 g",
        ] {
            assert!(
                out.prices.iter().all(|p| p.label.contains(required)),
                "B4 missing price-note {required}"
            );
            assert!(
                out.acceptances
                    .iter()
                    .all(|a| a.conditions.contains(required)),
                "B4 missing acceptance {required}"
            );
        }
        for source_fact in [
            "Das kaufen wir nicht an",
            "Ausschließlich LBMA zertifizierte Edelmetallbarren",
            "Silber Korpusware und Silbermesser",
            "Halbzeuge",
            "Feilungen",
            "Gussreste",
            "Scheidgut",
            "59,50 €",
            "Folgende Barren werden",
            "Tony Goetz",
        ] {
            assert!(
                RATES.contains(source_fact),
                "expanded source omits {source_fact}"
            );
        }
        let contract = include_str!("fixtures/heraeus-goldstore-hanau-contract-excerpts.txt");
        for source_fact in [
            "ausschließlich Verbraucher",
            "volljährig",
            "Europäischen Union oder UK",
            "persönlich",
            "Unmöglichkeit der Analyse",
            "Ablehnung",
            "59,50 €",
            "Rücktransport",
            "HRB 505728",
        ] {
            assert!(
                contract.contains(source_fact),
                "contract source omits {source_fact}"
            );
        }
    }

    #[tokio::test]
    async fn b5_actual_redirect_chain_back_to_original_is_rejected_before_hop() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let requests = Arc::new(AtomicUsize::new(0));
        let seen = requests.clone();
        let server = tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut buf = [0; 4096];
                let n = stream.read(&mut buf).await.unwrap();
                let request = String::from_utf8_lossy(&buf[..n]);
                let count = seen.fetch_add(1, Ordering::SeqCst);
                let response = if request.starts_with("GET /hop ") {
                    "HTTP/1.1 302 Found\r\nLocation: /start\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                } else if count == 0 {
                    "HTTP/1.1 302 Found\r\nLocation: /hop\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                } else {
                    "HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: close\r\n\r\nok!"
                };
                stream.write_all(response.as_bytes()).await.unwrap();
            }
        });
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            fetch(&source_client().unwrap(), &format!("http://{addr}/start")),
        )
        .await
        .unwrap();
        server.abort();
        assert!(result.is_err(), "B5 accepted original -> hop -> original");
        assert_eq!(
            requests.load(Ordering::SeqCst),
            1,
            "unreviewed hop contacted"
        );
    }

    #[test]
    fn expanded_fixture_or_actual_source_rejects_adversarial_changes() {
        // Optional fresh source snapshots let the same tests exercise full
        // actual HTML without adding network/DB dependencies to the test suite.
        let (rates, impressum) = if let Ok(dir) = std::env::var("HERAEUS_TEST_SOURCE_DIR") {
            let dir = std::path::Path::new(&dir);
            (
                std::fs::read_to_string(dir.join("rates.html")).unwrap(),
                std::fs::read_to_string(dir.join("impressum.html")).unwrap(),
            )
        } else {
            (RATES.to_owned(), IMPRESSUM.to_owned())
        };
        assert_eq!(parse(&rates).unwrap().prices.len(), 24);
        verify_operator(&impressum).unwrap();
        for (old, new) in [
            ("59,50", "159,50"),
            ("Das kaufen wir nicht an", "Das kaufen wir jetzt an"),
            (
                "Das kaufen wir an",
                "Gold kaufen wir nicht mehr an. Das kaufen wir an",
            ),
            (
                "Silber Korpusware und Silbermesser",
                "Silber nur ab 1000 Gramm",
            ),
            (
                "Edelmetall mit einem Gewicht unter 10g",
                "Kein Ausschluss: Edelmetall mit einem Gewicht unter 10g",
            ),
            (
                "eingeschränkt möglich",
                "eingeschränkt möglich war der Ankauf früher, heute ausgesetzt",
            ),
            (
                "Edelmetallankauf Preise",
                "Edelmetallankauf Preise. Kein Ankauf mehr: alle Werte Verkaufspreise",
            ),
            (
                "im Rodenbacher Weg 2 in Hanau",
                "Nicht mehr Rodenbacher Weg 2: Abgabe jetzt in Frankfurt",
            ),
            (
                "class=\"e-purchase-listing-product-box-price\"",
                "class=\"e-purchase-listing-product-box-price\" hidden",
            ),
            (
                "class=\"product-price\"",
                "class=\"product-price\" style=\"display:none\"",
            ),
            (
                "class=\"product-price-wrapper\"",
                "class=\"product-price-wrapper\" hidden",
            ),
            (
                "class=\"cms-element-product-listing\"",
                "class=\"cms-element-product-listing\" hidden",
            ),
            ("<main ", "<main style=\"visibility: hidden\" "),
        ] {
            assert!(rates.contains(old), "real source mutation missing {old}");
            assert!(
                parse(&rates.replace(old, new)).is_err(),
                "real source accepted {new}"
            );
        }
        let listing = "<div class=\"cms-element-product-listing\"";
        let listing_at = rates.find(listing).expect("real listing wrapper");
        let mut wrapper_notice = rates.clone();
        wrapper_notice.insert_str(
            listing_at,
            "<p>ANKAUF AUSGESETZT: Alle folgenden Werte sind Verkaufspreise.</p>",
        );
        assert!(
            parse(&wrapper_notice).is_err(),
            "real source accepted a contradictory sibling notice in the listing wrapper"
        );

        let listing_start = rates
            .find("<div class=\"cms-element-product-listing\"")
            .expect("real product listing");
        let listing_content = listing_start
            + rates[listing_start..]
                .find('>')
                .expect("real product listing opening")
            + 1;
        let mut fake_product_box = rates.clone();
        fake_product_box.insert_str(
            listing_content,
            "<span class=\"product-box\" style=\"display:inline !important\">ANKAUF AUSGESETZT: Alle folgenden Werte sind Verkaufspreise.</span>",
        );
        assert!(
            parse(&fake_product_box).is_err(),
            "real source accepted unreviewed prose disguised with product-box class"
        );

        let product_box_root = rates.replacen(
            "<div class=\"cms-element-product-listing\"",
            "<div class=\"cms-element-product-listing card product-box\"",
            1,
        );
        let root_start = product_box_root
            .find("<div class=\"cms-element-product-listing card product-box\"")
            .expect("real product listing root");
        let root_content = root_start
            + product_box_root[root_start..]
                .find('>')
                .expect("real product listing opening")
            + 1;
        let mut root_notice = product_box_root;
        root_notice.insert_str(
            root_content,
            "<p>ANKAUF AUSGESETZT: Alle folgenden Werte sind Verkaufspreise.</p>",
        );
        assert!(
            parse(&root_notice).is_err(),
            "real source accepted a notice after the listing root claimed the card exemption"
        );

        let no_purchase_heading = "Das kaufen wir nicht an";
        let heading_at = rates
            .find(no_purchase_heading)
            .expect("real conditions block");
        let block_at = rates[..heading_at]
            .rfind("<div class=\"cms-element-text\"")
            .expect("real conditions block opening");
        let mut fake_faq = rates.clone();
        fake_faq.insert_str(
            block_at + "<div class=\"cms-element-text".len(),
            " collapse netzp-powerpack6-collapse-body",
        );
        assert!(
            parse(&fake_faq).is_err(),
            "real source accepted a condition block disguised as an FAQ collapse body"
        );
        for (old, new) in [
            (
                "Verantwortlich für den Shop",
                "Verantwortlich für den Shop: Neuer Betreiber Beispiel GmbH. Früher:",
            ),
            ("<main ", "<main hidden "),
            (
                "<p>Heraeus Metals Germany",
                "<p hidden>Heraeus Metals Germany",
            ),
        ] {
            assert!(impressum.contains(old));
            assert!(
                verify_operator(&impressum.replace(old, new)).is_err(),
                "real operator accepted {new}"
            );
        }
        // Entity encoding/whitespace isn't a substantive conditions change.
        assert_eq!(
            parse(&rates.replace("59,50 €", "59,50&nbsp;€"))
                .unwrap()
                .prices
                .len(),
            24
        );
    }

    #[tokio::test]
    async fn transport_non_success_and_declared_or_streamed_oversize_are_rejected() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        for response in [
            b"HTTP/1.1 503 Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_vec(),
            b"HTTP/1.1 200 OK\r\nContent-Length: 2000001\r\nConnection: close\r\n\r\n".to_vec(),
            [
                b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n".as_slice(),
                &vec![b'x'; 2_000_001],
            ]
            .concat(),
        ] {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = [0; 4096];
                let n = stream.read(&mut request).await.unwrap();
                assert!(String::from_utf8_lossy(&request[..n])
                    .to_ascii_lowercase()
                    .contains("user-agent: schrott-mcp-ingestion/0.1"));
                // Early rejection can close before this oversized body finishes.
                let _ = stream.write_all(&response).await;
            });
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                fetch(&source_client().unwrap(), &format!("http://{addr}/source")),
            )
            .await
            .unwrap();
            server.await.unwrap();
            assert!(result.is_err());
        }
    }

    #[tokio::test]
    async fn real_catalog_records_all_grades_conditions_and_honest_date_history() {
        use schrott_mcp_store::{InternalDb, PublicDb};
        let dir = crate::test_support::TempDbDir::new("heraeus");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let mut out = parse(RATES).unwrap();
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-11T10:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let first = super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        assert_eq!((first.recorded, first.accepted), (24, 5));
        assert!(first.skipped.is_empty() && first.canaries.is_empty());
        let trader = public.find_trader_id(SLUG).unwrap().unwrap();
        for p in &out.prices {
            let material = public.find_material_id(p.material).unwrap().unwrap();
            assert_eq!(
                public.material_unit(material).unwrap().as_deref(),
                Some("EUR/g")
            );
            let current = public
                .current_price_for(trader, material, p.variant.as_ref())
                .unwrap()
                .unwrap();
            assert_eq!(current.published_at, None);
            assert_eq!(current.source_url, URL);
            assert_eq!(current.price_kind, "approx");
            assert_eq!(
                public.existing_acceptance(trader, material).unwrap(),
                Some((true, CONDITIONS.into()))
            );
        }
        let same = now + chrono::Duration::hours(6);
        super::super::super::record(&public, &internal, SLUG, &out, &same)
            .await
            .unwrap();
        out.prices[0].price += 0.01;
        let later = same + chrono::Duration::hours(6);
        super::super::super::record(&public, &internal, SLUG, &out, &later)
            .await
            .unwrap();
        let material = public.find_material_id("gold").unwrap().unwrap();
        let changed = public
            .current_price_for(trader, material, "333")
            .unwrap()
            .unwrap();
        assert_eq!(
            changed.published_at.as_deref(),
            Some("2026-10-12T00:00:00+00:00")
        );
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&changed.extra_json).unwrap()
                ["published_at_basis"],
            "observed_price_change"
        );
        assert_eq!(
            public
                .price_history(trader, material, "333", 10)
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            public
                .query_sql(&format!(
                    "SELECT count(*) FROM current_prices WHERE trader_id={trader}"
                ))
                .unwrap()
                .rows[0][0],
            24
        );
    }
}
