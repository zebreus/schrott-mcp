//! Metallhandel Jens Jacob (Leipzig): accepted sorts from the PDF price list.
//!
//! The former offer page (`dienstleistungen.html`) is gone — HTTP 404 since
//! 15.11.2026 over http *and* https. The homepage now links the price list
//! as PDF; Poppler `pdftotext` is required (the ELNO/Trapper pattern).
//!
//! **The prices are deliberately NOT recorded.** The metal rows of the "Aushang
//! Preisliste" carry **negative** values ("Kupfer Draht blank (Milberry)"
//! -9,20 EUR/kg) while the service rows carry positive ones ("Lohnverwiegung"
//! 10,00 EUR/St, "Schutt/Beton*" 250,00 EUR/t). Neither the price list nor
//! the conditions PDF (`konditionen-containerdienstleitstungen_neu.pdf`)
//! states what the sign means — a deduction from the daily quotation, a
//! surcharge, or a book-entry convention. Without that explanation a
//! negative number is not an absolute purchase price, so the handler records
//! the **accepted sorts** (same list) and leaves the prices out. The gap is
//! documented in `fixtures/jacob-preisliste-20261112.md`.
//!
//! Every PDF change that touches the accepted sorts must fail closed: the
//! identity anchors (headline, price column, purchase terms, minimum
//! quantity) are asserted before a single row is read, and a row whose
//! designation is not in the reviewed mapping table is skipped, never
//! guessed.

use scraper::{Html, Selector};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::super::{fetch_text, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, TraderInfo};
use crate::IngestError;

pub const SLUG: &str = "sn-leipzig-metallhandel-jens-jacob";
/// Bespoke, live-verified impressum URL (site nav's own "Impressum"
/// link). A move fails the step loudly (fix the URL) — never guessed,
/// never shared.
pub const IMPRESSUM_URL: &str = "https://www.metallhandel-jacob.de/impressum.html";
/// The homepage — the price list is only reachable from here (the old
/// `dienstleistungen.html` is a 404 since 15.11.2026).
pub const URL: &str = "https://www.metallhandel-jacob.de/";

/// A price cell of the list: `-9,20` / `10,00`. The comma is the German
/// decimal separator and always has two digits — that also keeps numbers
/// inside a designation ("Sorte 3 (bis 1,5 m Länge)") out of the price
/// column detection.
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
    let pdf_url = discover(&html)?;
    let bytes = fetch_pdf(client, &pdf_url).await?;
    let text = extract(&bytes, &pdf_url).await?;
    let labels = parse_labels(&text)?;
    let mut acceptances = Vec::with_capacity(labels.len());
    let mut skipped_labels = Vec::new();
    for label in labels {
        match grade_for(&label) {
            Some(material) => acceptances.push(ScrapedAcceptance {
                material,
                // The list states a price per unit, not an acceptance
                // condition; the terms live in the general Einkaufbedingungen.
                conditions: String::new(),
                label: label.clone(),
            }),
            None => skipped_labels.push(label),
        }
    }
    if acceptances.is_empty() {
        return Err(IngestError::Parse {
            url: pdf_url,
            detail: "keine Annahmesorte aus der Preisliste lesbar".to_owned(),
        });
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        skip_price_acceptances: false,
        // See the module doc: the sign of the metal rows is undocumented.
        prices: vec![],
        acceptances,
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: pdf_url,
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Exactly one price-list PDF on the homepage. The anchor text decides
/// ("PREISLISTE"); the conditions PDF ("Transportbehälter") is not a price
/// list. Zero or several hits fail closed.
fn discover(html: &str) -> Result<String, IngestError> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("a[href]").expect("valid selector");
    let base = reqwest::Url::parse(URL).expect("valid URL");
    let mut links: Vec<String> = Vec::new();
    for a in doc.select(&selector) {
        let label: String = a.text().collect::<String>().to_lowercase();
        if !label.contains("preisliste") {
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
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("expected one Preisliste PDF, found {}", links.len()),
        });
    }
    Ok(links.remove(0))
}

async fn fetch_pdf(client: &reqwest::Client, url: &str) -> Result<Vec<u8>, IngestError> {
    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.to_owned(),
            source,
        })?;
    let status = response.status().as_u16();
    if !response.status().is_success() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: format!("HTTP {status}"),
        });
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|source| IngestError::Body {
        url: url.to_owned(),
        source,
    })? {
        if bytes.len() + chunk.len() > 5_000_000 {
            return Err(IngestError::Parse {
                url: url.to_owned(),
                detail: "PDF exceeds 5 MB".to_owned(),
            });
        }
        bytes.extend_from_slice(&chunk);
    }
    if !bytes.starts_with(b"%PDF-") {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: "not a PDF".to_owned(),
        });
    }
    Ok(bytes)
}

/// `pdftotext -layout` under a memory/CPU limit, with a hard timeout —
/// the ELNO/Trapper pattern.
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
        .map_err(|e| IngestError::Parse {
            url: url.to_owned(),
            detail: format!("pdftotext unavailable: {e}"),
        })?;
    let mut stdin = child.stdin.take().expect("piped stdin");
    let stdout = child.stdout.take().expect("piped stdout");
    let stderr = child.stderr.take().expect("piped stderr");
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
        .map_err(|_| IngestError::Parse {
            url: url.to_owned(),
            detail: "PDF extraction timeout".to_owned(),
        })?
        .map_err(|e| IngestError::Parse {
            url: url.to_owned(),
            detail: format!("PDF extraction: {e}"),
        })?;
    if !status.success() {
        return Err(IngestError::Parse {
            url: url.to_owned(),
            detail: format!("pdftotext: {}", String::from_utf8_lossy(&stderr)),
        });
    }
    String::from_utf8(stdout).map_err(|e| IngestError::Parse {
        url: url.to_owned(),
        detail: format!("PDF text: {e}"),
    })
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

/// The designations of the list, in the operator's own wording.
///
/// The list is two columns; `pdftotext -layout` keeps them side by side. The
/// price cells are right-aligned, so their **end offsets** cluster into
/// exactly two positions — that split is what separates the columns (and it
/// survives a changed designation length, unlike a fixed column index).
fn parse_labels(text: &str) -> Result<Vec<String>, IngestError> {
    // Identity and terms: a different document must fail closed instead of
    // being read as this operator's list.
    for anchor in [
        "Aushang Preisliste",
        "Preis (€/Einheit)",
        "Reserve-Charge-Verfahren",
        "Mindestmenge von einem Kilogramm",
    ] {
        if !text.contains(anchor) {
            return Err(IngestError::Parse {
                url: URL.to_owned(),
                detail: format!("Preisliste-Anker fehlt: {anchor}"),
            });
        }
    }
    // End offsets of the price cells. The list is set in a proportional
    // font, so the **character** offset of a right-aligned column is the
    // stable one — a byte offset drifts by two per umlaut in front of it.
    let mut ends: Vec<(usize, usize)> = Vec::new();
    for line in text.lines() {
        ends.extend(price_cells(line));
    }
    ends.sort_unstable();
    // Cluster the ends (right-aligned cells differ by at most one character).
    let mut clusters: Vec<(usize, usize, usize)> = Vec::new();
    for (zeichen, byte) in ends {
        if let Some(last) = clusters.last_mut() {
            if zeichen.abs_diff(last.0) <= 1 {
                last.0 = zeichen;
                last.1 = byte;
                last.2 += 1;
                continue;
            }
        }
        clusters.push((zeichen, byte, 1));
    }
    let solid: Vec<(usize, usize, usize)> =
        clusters.into_iter().filter(|(_, _, n)| *n >= 3).collect();
    if solid.len() != 2 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: format!("expected two price columns, found {}", solid.len()),
        });
    }
    let split = solid[0].1;
    let mut labels = Vec::new();
    for line in text.lines() {
        let (left, right) = split_at(line, split);
        for part in [left, right] {
            if let Some((label, _unit, _price)) = split_row(part) {
                labels.push(label);
            }
        }
    }
    if labels.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preislistenzeilen leer".to_owned(),
        });
    }
    Ok(labels)
}

/// The price cells of one line: (character end, byte end) of every token that
/// looks like a price.
fn price_cells(line: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut rest = line;
    let mut byte_pos = 0usize;
    let mut char_pos = 0usize;
    loop {
        let trimmed = rest.trim_start();
        let skip_bytes = rest.len() - trimmed.len();
        byte_pos += skip_bytes;
        char_pos += rest[..skip_bytes].chars().count();
        if trimmed.is_empty() {
            break;
        }
        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        let token = &trimmed[..end];
        let token_chars = token.chars().count();
        if is_price(token) {
            out.push((char_pos + token_chars, byte_pos + end));
        }
        byte_pos += end;
        char_pos += token_chars;
        rest = &trimmed[end..];
    }
    out
}

/// Split a line at a byte offset that falls on a character boundary.
fn split_at(line: &str, at: usize) -> (&str, &str) {
    if at == 0 || at >= line.len() {
        return (line, "");
    }
    if !line.is_char_boundary(at) {
        return (line, "");
    }
    (&line[..at], &line[at..])
}

fn is_price(token: &str) -> bool {
    let mut chars = token.chars();
    let mut digits = 0;
    let mut comma = false;
    let mut after = 0;
    for c in chars.by_ref() {
        match c {
            '-' if digits == 0 => {}
            d if d.is_ascii_digit() => {
                digits += 1;
                if comma {
                    after += 1;
                }
            }
            ',' if !comma && digits > 0 => comma = true,
            _ => return false,
        }
    }
    comma && digits > 0 && after == 2 && digits <= 5
}

/// One table row: designation, unit, price — from the right (price), then
/// the unit, then the designation.
fn split_row(part: &str) -> Option<(String, String, String)> {
    let part = part.trim();
    if part.is_empty() {
        return None;
    }
    let (head, price) = part.rsplit_once(' ')?;
    let price = price.trim();
    if !is_price(price) {
        return None;
    }
    let (label, unit) = head.trim_end().rsplit_once(' ')?;
    let unit = unit.trim();
    if !matches!(unit, "kg" | "t" | "St") {
        return None;
    }
    let label = label.trim();
    if label.is_empty() || label.len() > 120 {
        return None;
    }
    Some((label.to_owned(), unit.to_owned(), price.to_owned()))
}

/// Exact designation → catalog material. The operator's own wording is the
/// variant (it is kept in the label), so two grades never collapse into one
/// material row. A designation that is not listed here is skipped, never
/// guessed — a wrong acceptance is worse than a logged gap.
fn grade_for(label: &str) -> Option<&'static str> {
    Some(match label {
        // --- Kupfer ---
        "Kupfer Kanal" => "kupfer-gemischt",
        "Kupfer Draht blank (Milberry)" => "kupfer-millberry",
        "Kupfer Raff" => "kupfer-leicht",
        "Kupfer Blech neu (Kerze)" => "kupfer-gemischt",
        "Kupfer Oberleitungsdraht" => "kupfer-gemischt",
        "Kupfer Schwer" => "kupfer-schwer",
        "Kupfer Leitschiene (E-Schiene)" => "kupfer-gemischt",
        // --- Kupferkabel ---
        "Kupferkabel dünn (Kupfershredderkabel)" => "kabel-kupfer",
        "Kupferkabel mittel" => "kabel-kupfer",
        "Kupferkabel dick" => "kabel-kupfer",
        "Kupfer Telekomkabel dick" => "kabel-kupfer",
        "Kupfer Postkabel dünn" => "kabel-kupfer",
        "Kupferkabel Gummilitze" => "kabel-kupfer",
        "Kupferkabel dünn mit Anhaftung" => "kabel-kupfer",
        "Kupfer Wirrkabel" => "kabel-kupfer",
        "Kupfer-Blei Kabel dick" => "kabel-blei",
        "Kupfer-Blei Kabel dünn" => "kabel-blei",
        "Kupfer-Blei Kabel mittel" => "kabel-blei",
        "Kupfer-Blei Kabel mit Eisen" => "kabel-blei",
        // --- Aluminium ---
        "Aluminium Blech ohne Anhaftung" => "aluminium-blech",
        "Aluminium Blech neu" => "aluminium-blech",
        "Aluminium Blech mit 2 % Eisen (Geschirr)" => "aluminium-blech",
        "Aluminium Blech mit 5% Eisen (Geschirr)" => "aluminium-blech",
        "Aluminium Kühler" => "aluminium-gemischt",
        "Aluminium Draht blank" => "aluminium-profile",
        "Aluminium Profil blank" => "aluminium-profile",
        "Aluminium Profil eloxiert" => "aluminium-profile",
        "Aluminium Leitschienen (E-Schiene)" => "aluminium-profile",
        "Aluminium Guss ohne Eisen" => "aluminium-guss",
        "Aluminium Guss mit 5% Eisen" => "aluminium-guss",
        "Aluminium Guss mit 35% Eisen" => "aluminium-guss",
        "Aluminium Späne" => "aluminium-spaene",
        "Aluminiumkabel dick" => "kabel-alu",
        "Aluminiumkabel dünn" => "kabel-alu",
        "Aluminium Wirrkabel" => "kabel-alu",
        // --- Messing / Blei / Zink / Zinn / Nickel ---
        "Messing leicht/schwer" => "messing-leicht",
        "Messing mit Anhaftung" => "messing",
        "Messing 58" => "messing",
        "Messing Späne" => "messing",
        "Blei ohne Anhaftung" => "blei",
        "Blei mit Anhaftung" => "blei",
        "Bleibatterien Starter" => "batterien-blei",
        "Bleibatterien Stapler" => "batterien-blei",
        "Blei Auswuchtgewichte" => "blei-auswucht",
        "Zinkblech ohne Anhaftung" => "zink",
        "Zinkblech mit Anhaftung" => "zink",
        "Zinn 90%-100%" => "zinn",
        "Nickel 90%-100%" => "nickel",
        "Hartmetall" => "hartmetall",
        // --- Kühler ---
        "Kupfer-Messing Kühler" => "cu-ms-kuehler",
        "Kupfer-Messing-Eisen Kühler" => "cu-ms-kuehler",
        "Kupfer-Alu Kühler" => "alu-cu-kuehler",
        // --- Edelstahl / Eisen ---
        "V2A ohne Anhaftung" => "edelstahl-v2a",
        "V2A Späne" => "edelstahl-v2a",
        "V2A mit Anhaftung" => "edelstahl-v2a",
        "Elektromotoren" => "elektromotoren",
        "Verbrennungsmotoren/Getriebe" => "mischschrott",
        "Sorte 3 (bis 1,5 m Länge)" => "mischschrott",
        "Träger/S3 lang (ab 1,5 m - 6 m Länge)" => "stahlschrott-traeger",
        "Scherenschrott schwer" => "stahlschrott-scheren",
        "Scherenschrott leicht/Leichte Schere" => "stahlschrott-scheren",
        "Shredderschrott/Kondiratorvormaterial" => "stahlschrott-shredder",
        "Shredderschrott Anhaft." => "stahlschrott-shredder",
        "Baustahlschrott" => "mischschrott",
        "Handelsguss" => "eisenschrott-gussbruch",
        "Bremsscheibenschrott" => "eisenschrott-gussbruch",
        "Maschinenguss" => "eisenschrott-gussbruch",
        // --- Katalysatoren ---
        "Katalysatoren Metall S" => "katalysatoren-monolith",
        "Katalysatoren Keramik S" => "katalysatoren",
        // --- Platinen ---
        "Platinen Klasse 1A" => "platinen",
        "Platinen Klasse 1B" => "platinen",
        "Platinen Klasse 2" => "platinen",
        "Platinen Klasse 3" => "platinen",
        "Platinen Klasse 4" => "platinen",
        // --- Elektronik ---
        "Prozessoren x86/Goldcaps" => "prozessoren",
        "Prozessoren Keramik gemischt" => "prozessoren",
        "Prozessoren Kunststoff mit Kupferkern" => "prozessoren",
        "Prozessoren Kunststoff" => "prozessoren",
        "Prozessoren Slot" => "prozessoren",
        "Arbeitsspeicher mit Goldkante" => "ram",
        "Arbeitsspeicher mit Silberkante" => "ram",
        "Netzteile (Computer)" => "netzteile",
        "Laufwerke" => "laufwerke",
        "Festplatten" => "festplatten",
        "Integrierte Schaltkreise (ICs)" => "integrierte-schaltungen",
        "Smartphone mit Akku" => "handys",
        "Handys ohne Akku" => "handys",
        "Computer beraubt max.1 Teil" => "computer",
        "Notebooks/Laptops ohne Akku" => "notebooks",
        "Server unberaubt (auf Anfrage)" => "computer",
        "Elektronikschrott gemischt" => "elektrobauteile",
        "Stecker Flachband" => "elektronikstecker",
        "Stecker EDV" => "elektronikstecker",
        _ => return None,
    })
}

/// Bespoke contact extraction for THIS impressum only: `<h1>Impressum</h1>`
/// followed by the address `<p>` ("Dortmunder Str.12<br>04357 Leipzig<br>
/// Telefon: …<br>Email: mailto-link<br>Internet: …"). The street line
/// glues the house number ("Str.12") — deglued to "Str. 12" with a test.
/// Missing anchors → loud error, never a guessed fallback.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let h1 = Selector::parse("h1").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let a = Selector::parse("a").expect("valid selector");
    if !doc
        .select(&h1)
        .any(|h| h.text().collect::<String>().trim() == "Impressum")
    {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Impressum-Block fehlt".to_owned(),
        });
    }
    let addr_p = doc.select(&p).find(|el| {
        let t: String = el.text().collect();
        t.contains("Dortmunder Str.")
    });
    let Some(addr_p) = addr_p else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Adress-Block fehlt".to_owned(),
        });
    };
    let addr_lines: Vec<String> = addr_p
        .inner_html()
        .split("<br")
        .map(strip_fragment)
        .filter(|s| !s.is_empty())
        .collect();
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in addr_lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| format!("{a} {b}"));
                if k > 0 {
                    street = deglue_number(&addr_lines[k - 1]);
                }
                break;
            }
        }
    }
    let mut phone = String::new();
    for part in addr_p.inner_html().split("<br") {
        let t = strip_fragment(part);
        if let Some(v) = t.strip_prefix("Telefon:") {
            phone = v.trim().to_owned();
            break;
        }
    }
    // E-mail per mailto-href, never per token split (neighbour nodes glue
    // together without whitespace in scraper text output).
    let email = addr_p
        .select(&a)
        .filter_map(|el| el.value().attr("href"))
        .find_map(|h| h.strip_prefix("mailto:"))
        .unwrap_or_default()
        .trim()
        .to_owned();
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

/// "Dortmunder Str.12" → "Dortmunder Str. 12": split a house number glued
/// onto the street token ("Str.12" → "Str." + "12"). Anything else passes
/// through untouched.
fn deglue_number(s: &str) -> String {
    let mut toks: Vec<String> = s.split_whitespace().map(str::to_owned).collect();
    if let Some(last) = toks.last() {
        let cut = last
            .char_indices()
            .rev()
            .take_while(|(_, c)| c.is_ascii_digit() || *c == '-')
            .last()
            .map(|(i, _)| i);
        if let Some(i) = cut {
            let head_ends = last[..i].ends_with(' ') || last[..i].ends_with('-');
            if i > 0 && !head_ends {
                let head = last[..i].to_owned();
                let tail = last[i..].to_owned();
                if head.ends_with('.') || head.len() > 2 {
                    toks.pop();
                    toks.push(head);
                    toks.push(tail);
                }
            }
        }
    }
    toks.join(" ")
}

/// Strip tags from a `<br>`-split fragment. Fragments start with a tag
/// remnant — drop everything up to the first '>' first, or attributes
/// parse as text.
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
    use std::io::Write;

    use super::{grade_for, is_price, parse_labels, split_row};

    const FIXTURE: &[u8] = include_bytes!("fixtures/jacob-preisliste-20261112.pdf");

    /// The same extraction the handler runs, but synchronous so the offline
    /// fixture can be read in a test (the ELNO pattern).
    fn extract_bytes_for_test(bytes: &[u8]) -> String {
        let mut child = std::process::Command::new("/usr/bin/pdftotext")
            .args(["-layout", "-", "-"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("pdftotext");
        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(bytes)
            .expect("writes");
        let out = child.wait_with_output().expect("pdftotext output");
        assert!(out.status.success(), "pdftotext failed");
        String::from_utf8(out.stdout).expect("utf-8")
    }

    #[test]
    fn price_tokens_need_two_decimals() {
        assert!(is_price("-9,20"));
        assert!(is_price("10,00"));
        assert!(is_price("250,00"));
        // Numbers inside a designation must not be mistaken for prices.
        assert!(!is_price("1,5"));
        assert!(!is_price("3"));
        assert!(!is_price("1A"));
        assert!(!is_price("2,5"));
    }

    #[test]
    fn row_splits_from_the_right() {
        let (label, unit, price) = split_row("Kupfer Raff kg -8,70").expect("row");
        assert_eq!(label, "Kupfer Raff");
        assert_eq!(unit, "kg");
        assert_eq!(price, "-8,70");
        // A designation with spaces and parentheses survives.
        let (label, unit, price) =
            split_row("Kupferkabel dünn (Kupfershredderkabel) kg -3,05").expect("row");
        assert_eq!(label, "Kupferkabel dünn (Kupfershredderkabel)");
        assert_eq!(unit, "kg");
        assert_eq!(price, "-3,05");
        // No unit → not a price row.
        assert!(split_row("Kupfer Raff -8,70").is_none());
    }

    #[test]
    fn actual_pdf_yields_both_columns_and_the_expected_sorts() {
        let text = extract_bytes_for_test(FIXTURE);
        let labels = parse_labels(&text).expect("parses");
        // Both columns are read: a designation from the left one and one from
        // the right one.
        assert!(labels.iter().any(|l| l == "Kupfer Raff"));
        assert!(labels
            .iter()
            .any(|l| l == "Kupferkabel dünn (Kupfershredderkabel)"));
        // Service rows are read as designations but never mapped: a
        // weighing fee is not an accepted sort.
        assert!(labels.iter().any(|l| l == "Lohnverwiegung"));
        assert_eq!(grade_for("Lohnverwiegung"), None);
        // The footnote and the terms carry no price cell — no designations.
        assert!(!labels.iter().any(|l| l.contains("Nur bei Anhaftung")));
        assert!(!labels.iter().any(|l| l.contains("Reserve-Charge")));
        assert!(!labels.iter().any(|l| l.contains("Mindestmenge")));
        // Every accepted sort is mapped; unknown ones are skipped, never
        // guessed.
        assert_eq!(grade_for("Kupfer Raff"), Some("kupfer-leicht"));
        assert_eq!(
            grade_for("Kupferkabel dünn (Kupfershredderkabel)"),
            Some("kabel-kupfer")
        );
        assert_eq!(grade_for("Katalysatoren Keramik S"), Some("katalysatoren"));
        assert_eq!(
            grade_for("Katalysatoren Metall S"),
            Some("katalysatoren-monolith")
        );
        assert_eq!(grade_for("Papier"), None);
        assert_eq!(grade_for("Lohnverwiegung"), None);
    }

    #[test]
    fn a_foreign_document_fails_closed() {
        assert!(parse_labels("Preisliste\nEisen und Stahl\n").is_err());
    }
}
