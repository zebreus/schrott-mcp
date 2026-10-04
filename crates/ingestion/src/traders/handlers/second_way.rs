//! 2nd way Recycling GmbH (Dresden): exact purchase prices
//! ("Ankaufspreise") in eight section tables (`<p><strong>` heading +
//! `<table>`), each price cell quoting its own unit ("EUR/t" for iron,
//! "EUR/kg" otherwise). The page date rides in "letzte Aktualisierung
//! 21.09.26 um 18:15 Uhr"; "Ankauf Hinweise" ends the block. Two
//! Schälkabel rows quote five Cu-share ladders (60–80%) in one cell and
//! are expanded into one price per share. "Hartmetall, sortiert" is an
//! "ab …" floor quote (approx). Zero rows, "auf Anfrage", paper,
//! shredder services and ambiguous iron grades have no clean mapping
//! and are skipped loudly.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_de_date, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "sn-dresden-2nd-way-recycling";
/// Bespoke, live-verified impressum URL (site nav "Impressum"). A move
/// fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://www.second-way.de/impressum";

pub const URL: &str = "https://www.second-way.de/ankaufspreise/";

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
    let (published_at, rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                let (price_kind, confidence) = kind_for(&label);
                prices.push(ScrapedPrice {
                    material,
                    variant,
                    price,
                    currency: "EUR",
                    unit,
                    price_kind,
                    price_min: None,
                    price_max: None,
                    confidence,
                    label,
                });
            }
            None => skipped_labels.push(label),
        }
    }
    // Impressum failure fails the whole step on purpose: a moved contact
    // page means the site changed and needs eyeballs before we trust
    // anything from it again.
    let (_, imp_html) = fetch_text(client, IMPRESSUM_URL).await?;
    let trader_info = extract_info(&imp_html)?;
    Ok(HandlerOutcome {
        prices,
        acceptances: vec![],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at,
    })
}

/// "ab 29,00 EUR/kg" (Hartmetall, sortiert) is a floor quote, not an
/// exact list price → approx/0.5. Everything else on the page is quoted
/// exact → exact/1.0.
fn kind_for(label: &str) -> (&'static str, Option<f64>) {
    if label.to_lowercase().contains("hartmetall") {
        ("approx", Some(0.5))
    } else {
        ("exact", Some(1.0))
    }
}

/// Explicit label → (material, variant) mapping. Anything unlisted is
/// skipped. Notes on judgement calls:
/// - "Sorte 3" and "Trägerschrott über 1,50 m" are trader grades with no
///   provable catalog equivalent → None (proposals, not crammed).
/// - Paper, Aktenvernichtung fees and the ambiguous iron grades
///   ("Sorte 3", "Trägerschrott") have no catalog material → None.
/// - Specific-before-generic: bare "guss" only matches after the Alu
///   arms; "getriebe"-style clashes don't exist on this page.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // No-catalog guards first (proposal material, never crammed).
    if l.contains("träger") || l.contains("traeger") {
        return None;
    }
    if l.contains("sorte 3") {
        return None;
    }
    if l.contains("altpapier")
        || l.contains("bücher")
        || l.contains("buecher")
        || l.contains("kartonagen")
        || l.contains("aktenvernichtung")
        || l.contains("ordner")
        || l.contains("behälter")
        || l.contains("behaelter")
        || l.contains("zuzahlung")
    {
        return None;
    }
    if l.trim()
        .trim_start_matches(['–', '-', ' '])
        .starts_with("lose")
    {
        return None;
    }
    if l.contains("auf anfrage") {
        return None;
    }
    // Schälkabel ladders (parse() expands these to one label per share).
    if l.contains("schälkabel") || l.contains("schalkabel") {
        if l.contains("starr") {
            if l.contains("60%") {
                return Some(("kabel-kupfer", "starr 60%"));
            } else if l.contains("65%") {
                return Some(("kabel-kupfer", "starr 65%"));
            } else if l.contains("70%") {
                return Some(("kabel-kupfer", "starr 70%"));
            } else if l.contains("75%") {
                return Some(("kabel-kupfer", "starr 75%"));
            } else if l.contains("80%") {
                return Some(("kabel-kupfer", "starr 80%"));
            }
            return None;
        } else if l.contains("litze") {
            if l.contains("60%") {
                return Some(("kabel-kupfer", "Litze 60%"));
            } else if l.contains("65%") {
                return Some(("kabel-kupfer", "Litze 65%"));
            } else if l.contains("70%") {
                return Some(("kabel-kupfer", "Litze 70%"));
            } else if l.contains("75%") {
                return Some(("kabel-kupfer", "Litze 75%"));
            } else if l.contains("80%") {
                return Some(("kabel-kupfer", "Litze 80%"));
            }
            return None;
        } else if l.contains("alu") {
            return Some(("kabel-alu", "60%"));
        }
        return None;
    }
    if l.contains("cu kabel") && l.contains("38") {
        Some(("kabel-kupfer", "38%"))
    } else if l.contains("granulier") {
        Some(("kabel-kupfer", "Granulier"))
    } else if l.contains("cu") && l.contains("papier") {
        Some(("kabel-blei", "Cu-Pb Papier"))
    } else if l.contains("millberry") {
        Some(("kupfer-millberry", ""))
    } else if l.contains("berry") {
        Some(("kupfer-berry", ""))
    } else if l.contains("kupfer raff") {
        Some(("kupfer-gemischt", "Raff"))
    } else if l.contains("alt, sauber") {
        Some(("kupfer-gemischt", "alt sauber"))
    } else if l.contains("kupfer neu") {
        Some(("kupfer-gemischt", "neu"))
    } else if l.contains("kupfer") && l.contains("leitsch") {
        Some(("kupfer-gemischt", "Leitschienen blank"))
    } else if l.contains("kupfer") {
        Some(("kupfer-gemischt", ""))
    } else if l.contains("messing schwer") {
        Some(("messing", "schwer"))
    } else if l.contains("erodierdraht") {
        Some(("messing", "Erodierdraht"))
    } else if l.contains("rotguss raff") {
        Some(("bronze-rotguss", "Raff"))
    } else if l.contains("rotguss") {
        Some(("bronze-rotguss", "sauber"))
    } else if l.contains("blech max. 4") || l.contains("blech max 4") {
        Some(("aluminium-blech", "max. 4% Anhaftung"))
    } else if l.contains("blech alt/neu") {
        Some(("aluminium-blech", "alt/neu sauber"))
    } else if l.contains("guss max. 4") || l.contains("guss max 4") {
        Some(("aluminium-guss", "max. 4% Anhaftung"))
    } else if l.contains("guss sauber") {
        Some(("aluminium-guss", "sauber"))
    } else if l.contains("felgen") {
        Some(("aluminium-guss", "Felgen"))
    } else if l.contains("profil lackiert") {
        Some(("aluminium-profile", "lackiert kurz"))
    } else if l.contains("profil blank") {
        Some(("aluminium-profile", "blank kurz"))
    } else if l.contains("alu") && l.contains("leitschienen") {
        Some(("aluminium-gemischt", "Leitschienen blank"))
    } else if l.contains("draht") {
        Some(("aluminium-gemischt", "Draht blank"))
    } else if l.contains("v2a") {
        Some(("edelstahl-v2a", "max. 1,5x0,5x0,5m"))
    } else if l.contains("v4a") {
        Some(("edelstahl-v4a", "max. 1,5x0,5x0,5m"))
    } else if l.contains("e-motore") || l.contains("emotore") {
        Some(("elektromotoren", ""))
    } else if l.contains("zink") {
        Some(("zink", ""))
    } else if l.contains("auswuchtblei") {
        Some(("blei-auswucht", "Auswuchtblei gemischt"))
    } else if l.contains("altblei") {
        Some(("blei", ""))
    } else if l.contains("hartmetall") {
        Some(("hartmetall", ""))
    } else if l.contains("mischschrott") {
        Some(("mischschrott", ""))
    } else if l.contains("schreddervormaterial") || l.contains("shreddervormaterial") {
        Some(("stahlschrott-shredder", ""))
    } else if l.contains("bremsscheiben") {
        Some(("eisenschrott-gussbruch", "Bremsscheiben"))
    } else if l.contains("guss") || l.contains("guß") {
        Some(("eisenschrott-gussbruch", ""))
    } else {
        None
    }
}

/// Bespoke contact extraction for THIS impressum only: the
/// "Anbieterkennzeichnung nach TMG §5" heading is followed by a firm /
/// street / PLZ-city `<p>` (`<br>` lines) and a `Tel.: … E-Mail: …`
/// `<p>`. Missing anchors mean the page changed shape → loud error.
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let doc = Html::parse_document(imp);
    let strong = Selector::parse("strong").expect("valid selector");
    let p = Selector::parse("p").expect("valid selector");
    let anchor = doc.select(&strong).find(|s| {
        s.text()
            .collect::<String>()
            .contains("Anbieterkennzeichnung nach TMG")
    });
    let Some(_) = anchor else {
        return Err(IngestError::Parse {
            url: IMPRESSUM_URL.to_owned(),
            detail: "Anbieterkennzeichnung fehlt".to_owned(),
        });
    };
    // Address: first <p> after the anchor whose <br> rows hold a PLZ.
    let paras: Vec<_> = doc.select(&p).collect();
    let mut addr_rows: Vec<String> = Vec::new();
    let mut contact = String::new();
    let mut in_block = false;
    for e in &paras {
        let text: String = e
            .text()
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !in_block {
            if text.contains("Anbieterkennzeichnung nach TMG") {
                in_block = true;
            }
            continue;
        }
        if addr_rows.is_empty() {
            for part in e.inner_html().split("<br") {
                let t = strip_tags(part);
                if !t.is_empty() {
                    addr_rows.push(t);
                }
            }
            continue;
        }
        if contact.is_empty() {
            contact = text;
            break;
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for row in &addr_rows {
        if row.contains("GmbH")
            || (row.contains("Recycling") && !row.chars().any(|c| c.is_ascii_digit()))
        {
            continue;
        }
        let mut it = row.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5 && pc.chars().all(|c| c.is_ascii_digit()) {
                postcode = pc.to_owned();
                city = it.fold(ci.to_owned(), |a, b| a + " " + b);
                continue;
            }
        }
        if street.is_empty() {
            street = row.clone();
        }
    }
    let phone = {
        // Cut at the next label first: scraper text() glues "5963719"+
        // "Internet:" into one token and the digit filter would stop early.
        let after = after_marker(&contact, "Tel.:");
        let end = ["Internet:", "E-Mail:", "Mobil:", "Fax:", "Telefax:"]
            .iter()
            .filter_map(|m| after.find(m))
            .min()
            .unwrap_or(after.len());
        after[..end]
            .split_whitespace()
            .take_while(|t| {
                t.chars()
                    .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    // E-mail needs its own rule: the phone-style filter stops at the
    // first letter, so take the @ token instead.
    let email = contact
        .split_whitespace()
        .find(|t| t.contains('@'))
        .unwrap_or_default()
        .trim_matches([',', ';', '.'])
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

fn after_marker<'a>(text: &'a str, marker: &str) -> &'a str {
    text.find(marker)
        .map(|i| &text[i + marker.len()..])
        .unwrap_or("")
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
    // Window, never the whole page: the footer reuses headings and
    // links that must not leak into the price block.
    let start = html
        .find("<h1>Ankaufspreise</h1>")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankaufspreise-Block fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail
        .find("<h2>Ankauf Hinweise</h2>")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Ankauf-Hinweise-Terminator fehlt".to_owned(),
        })?;
    let window = &tail[..end];
    let published_at = find_update_date(window);
    // Section per table: the last <strong> heading before each <table>.
    let mut heads: Vec<(usize, String)> = Vec::new();
    let mut pos = 0;
    while let Some(i) = window[pos..].find("<strong>") {
        let a = pos + i + "<strong>".len();
        if let Some(e) = window[a..].find("</strong>") {
            // Inner markup (<br/>) never belongs to the name.
            let raw = &window[a..a + e];
            let text = raw
                .split('<')
                .next()
                .unwrap_or(raw)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if !text.is_empty() {
                heads.push((pos + i, text));
            }
            pos = a + e + "</strong>".len();
        } else {
            break;
        }
    }
    let table_sel = Selector::parse("table").expect("valid selector");
    let row_sel = Selector::parse("tr").expect("valid selector");
    let cell_sel = Selector::parse("td").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips: Vec<String> = Vec::new();
    let mut search = 0;
    let mut table_count = 0;
    while let Some(i) = window[search..].find("<table") {
        let a = search + i;
        let Some(e) = window[a..].find("</table>") else {
            break;
        };
        let block = &window[a..a + e + "</table>".len()];
        search = a + e + "</table>".len();
        table_count += 1;
        let section = heads
            .iter()
            .rev()
            .find(|(p, _)| *p < a)
            .map(|(_, s)| s.clone())
            .unwrap_or_default();
        let frag = Html::parse_fragment(block);
        let Some(t) = frag.select(&table_sel).next() else {
            continue;
        };
        for tr in t.select(&row_sel) {
            let cells: Vec<String> = tr.select(&cell_sel).map(|c| c.text().collect()).collect();
            if cells.len() < 2 {
                continue;
            }
            let label = cells[0].replace(['\u{a0}', '\u{feff}'], " ");
            let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
            if label.is_empty() {
                continue;
            }
            let price_raw = cells[1].replace('\u{a0}', " ");
            let price_trim = price_raw.split_whitespace().collect::<Vec<_>>().join(" ");
            if price_trim.is_empty() {
                skips.push(format!("{label} (kein Preis [{section}])"));
                continue;
            }
            if price_trim.to_lowercase().contains("auf anfrage") {
                skips.push(format!("{label} (Preis auf Anfrage)"));
                continue;
            }
            // Cu-share ladders: "CU Schälkabel starr, 60 / 65 / 70 / 75 /
            // 80%" with "5,09 / 5,63 / … EUR/kg" — one price per share.
            if (label.contains("Schälkabel") || label.contains("Schalkabel"))
                && price_trim.contains('/')
            {
                match expand_ladder(&label, &price_trim) {
                    Some(expanded) => {
                        for (lab, price, unit) in expanded {
                            if price == 0.0 {
                                skips.push(format!("{lab} (0,00 — kein Ankauf)"));
                            } else {
                                rows.push((lab, price, unit));
                            }
                        }
                    }
                    None => skips.push(format!(
                        "{label} (Staffelpreise unverständlich: {price_trim})"
                    )),
                }
                continue;
            }
            let Some(price) = parse_eur(&price_trim) else {
                skips.push(format!("{label} (kein Preis: {price_trim} [{section}])"));
                continue;
            };
            if price == 0.0 {
                skips.push(format!("{label} (0,00 — kein Ankauf)"));
                continue;
            }
            // An unparseable unit is a loud skip, never a silent
            // default: a per-tonne price recorded as per-kg would be a
            // 1000x error.
            let Some(unit) = unit_of(&price_trim) else {
                skips.push(format!("{label} (Einheit unverständlich: {price_trim})"));
                continue;
            };
            rows.push((label, price, unit));
        }
    }
    if table_count == 0 {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "keine Preistabellen".to_owned(),
        });
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preistabellen leer".to_owned(),
        });
    }
    Ok((published_at, rows, skips))
}

/// "letzte Aktualisierung 21.09.26 um 18:15 Uhr" — day.month.2-digit-year
/// right after the marker; the clock time is ignored.
fn find_update_date(window: &str) -> Option<String> {
    let i = window.find("letzte Aktualisierung")?;
    let after = &window[i + "letzte Aktualisierung".len()..];
    let tok = after
        .split_whitespace()
        .find(|t| t.chars().filter(|c| *c == '.').count() == 2)?;
    let parts: Vec<&str> = tok.trim_matches(',').split('.').collect();
    if parts.len() == 3 && parts[2].len() == 2 {
        parse_de_date(parts[0], parts[1], &format!("20{}", parts[2]))
    } else {
        None
    }
}

/// Expand a Cu-share ladder into one (label, price, unit) per share.
/// Labels become "CU Schälkabel starr 60%" etc. so grade_for() can map
/// each share explicitly. Count mismatch → None (loud skip).
fn expand_ladder(label: &str, price_cell: &str) -> Option<Vec<(String, f64, &'static str)>> {
    let base = label.split(',').next().unwrap_or(label).trim();
    let shares: Vec<String> = label
        .split(|c: char| !c.is_ascii_digit())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_owned())
        .collect();
    let mut prices = Vec::new();
    // Cut the trailing unit first: "EUR/kg" holds a '/' that would fake
    // a sixth price segment ("5,09 / … / 7,25 EUR/kg" → 6 parts).
    let body = price_cell.split("EUR").next().unwrap_or(price_cell);
    let body = body.split('€').next().unwrap_or(body);
    for part in body.split('/') {
        let Some(v) = parse_eur(part) else {
            return None;
        };
        prices.push(v);
    }
    if shares.len() != prices.len() || shares.is_empty() {
        return None;
    }
    let unit = unit_of(price_cell)?;
    Some(
        shares
            .into_iter()
            .zip(prices)
            .map(|(s, v)| (format!("{base} {s}%"), v, unit))
            .collect(),
    )
}

/// Bespoke unit matcher for THIS page: units ride inline in the price
/// cell (live: "120,00 EUR/t", "9,45 EUR/kg", "9,50 EUR/m³",
/// "45,00 EUR/Stk."). Only kg/t exist as purchase units — m³/Stk.
/// service lines skip loudly at the call site.
fn unit_of(cell: &str) -> Option<&'static str> {
    let lower = cell.to_lowercase();
    if lower.contains("eur/kg") || lower.contains("€/kg") {
        Some("EUR/kg")
    } else if lower.contains("eur/t") || lower.contains("€/t") {
        Some("EUR/t")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{expand_ladder, extract_info, grade_for, kind_for, parse};

    // Real page shape, condensed: h1 + update line, strong-headed
    // tables, a share ladder, ab/zero/Anfrage rows, paper table, h2 end.
    const FIXTURE: &str = "<h1>Ankaufspreise</h1>\
        <p>letzte Aktualisierung 21.09.26 um 18:15 Uhr</p>\
        <p><strong>Eisenschrott</strong></p>\
        <table><tbody>\
        <tr><td>Schreddervormaterial</td><td>0,00 EUR/t</td></tr>\
        <tr><td>Mischschrott</td><td>120,00 EUR/t</td></tr>\
        <tr><td>Sorte 3</td><td>180,00 EUR/t</td></tr>\
        <tr><td>Bremsscheiben</td><td>200,00 EUR/t</td></tr>\
        </tbody></table>\
        <p><strong>Kabel (Kupfer)</strong></p>\
        <table><tbody>\
        <tr><td>CU Kabel min. 38%</td><td>3,35 EUR/kg</td></tr>\
        <tr><td>CU Schälkabel starr, 60 / 65 / 70 / 75 / 80%</td>\
        <td>5,09 / 5,63 / 6,17 / 6,71 / 7,25 EUR/kg</td></tr>\
        </tbody></table>\
        <p><strong>Kupfer</strong></p>\
        <table><tbody>\
        <tr><td>Kupfer Millberry</td><td>10,25 EUR/kg</td></tr>\
        </tbody></table>\
        <p><strong>Edelstahl / Zink / Blei</strong></p>\
        <table><tbody>\
        <tr><td>Hartmetall, sortiert</td><td>ab 29,00 EUR/kg</td></tr>\
        <tr><td>V2A max.1,5x0,5x0,5m</td><td>0,50 EUR/kg</td></tr>\
        </tbody></table>\
        <p><strong>Kabel (Aluminium)</strong></p>\
        <table><tbody>\
        <tr><td>Alu Kabel ab 100kg</td><td>auf Anfrage</td></tr>\
        </tbody></table>\
        <p><strong>Papier / Kartonagen / Aktenvernichtung</strong></p>\
        <table><tbody>\
        <tr><td>Altpapier</td><td>0,08 EUR/kg</td></tr>\
        <tr><td>Kartonagen – Zuzahlung</td><td>9,50 EUR/m³ zzgl. 19% MwSt.</td></tr>\
        </tbody></table>\
        <h2>Ankauf Hinweise</h2>";

    #[test]
    fn tables_sections_and_date_parse() {
        let (published_at, rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(published_at.as_deref(), Some("2026-09-21T00:00:00+00:00"));
        // 3 Fe (Sorte 3 parses — grade skips it later) + 1 Kabel +
        // 5 ladder + 1 Cu + 2 Edel + 1 Papier = 13 rows.
        assert_eq!(rows.len(), 13, "{rows:?}");
        // Zero row, Anfrage row, m³ row = 3 loud skips.
        assert_eq!(skips.len(), 3, "{skips:?}");
        assert!(skips
            .iter()
            .any(|s| s.contains("Schreddervormaterial") && s.contains("kein Ankauf")));
        assert!(skips.iter().any(|s| s.contains("auf Anfrage")));
        assert!(skips.iter().any(|s| s.contains("Einheit unverständlich")));
        let fe = rows.iter().find(|r| r.0 == "Mischschrott").expect("fe");
        assert_eq!((fe.1, fe.2), (120.0, "EUR/t"));
        let ladder: Vec<_> = rows
            .iter()
            .filter(|r| r.0.contains("Schälkabel starr"))
            .collect();
        assert_eq!(ladder.len(), 5);
        assert_eq!(ladder[0].0, "CU Schälkabel starr 60%");
        assert_eq!((ladder[0].1, ladder[0].2), (5.09, "EUR/kg"));
        assert_eq!(ladder[4].0, "CU Schälkabel starr 80%");
        assert_eq!(ladder[4].1, 7.25);
    }

    #[test]
    fn window_and_ladder_fail_loudly() {
        assert!(parse("<html><body>keine Preise</body></html>").is_err());
        let no_end = FIXTURE.replace("<h2>Ankauf Hinweise</h2>", "<h2>Anderes</h2>");
        assert!(parse(&no_end).is_err());
        assert!(expand_ladder("CU Schälkabel starr, 60 / 70%", "5,09 EUR/kg").is_none());
        let html = FIXTURE
            .replace("EUR/kg", "pro Sack")
            .replace("EUR/t", "pro Sack");
        let err = parse(&html).expect_err("empty tables error");
        assert!(err.to_string().contains("leer"));
    }

    #[test]
    fn copper_and_aluminium_rails_keep_separate_materials() {
        let html = "<h1>Ankaufspreise</h1><table>\
            <tr><td>Kupfer Leitsch. blank</td><td>10,15 EUR/kg</td></tr>\
            <tr><td>Alu Leitschienen, blank</td><td>1,90 EUR/kg</td></tr>\
            </table><h2>Ankauf Hinweise</h2>";
        let (_, rows, _) = parse(html).expect("parses both rail grades");
        assert_eq!(rows.len(), 2);
        assert_eq!(
            grade_for(&rows[0].0),
            Some(("kupfer-gemischt", "Leitschienen blank"))
        );
        assert_eq!(
            grade_for(&rows[1].0),
            Some(("aluminium-gemischt", "Leitschienen blank"))
        );
        assert_eq!((rows[0].1, rows[0].2), (10.15, "EUR/kg"));
        assert_eq!((rows[1].1, rows[1].2), (1.90, "EUR/kg"));
    }

    #[test]
    fn mapping_and_kind_cover_fixture_labels() {
        assert_eq!(grade_for("Mischschrott"), Some(("mischschrott", "")));
        assert_eq!(grade_for("Sorte 3"), None);
        assert_eq!(grade_for("Trägerschrott über 1,50 m"), None);
        assert_eq!(
            grade_for("CU Kabel min. 38%"),
            Some(("kabel-kupfer", "38%"))
        );
        assert_eq!(
            grade_for("CU Schälkabel starr 60%"),
            Some(("kabel-kupfer", "starr 60%"))
        );
        assert_eq!(
            grade_for("CU Schälkabel Litze 80%"),
            Some(("kabel-kupfer", "Litze 80%"))
        );
        assert_eq!(
            grade_for("CU Granulierkabel"),
            Some(("kabel-kupfer", "Granulier"))
        );
        assert_eq!(
            grade_for("Kupfer Millberry"),
            Some(("kupfer-millberry", ""))
        );
        assert_eq!(grade_for("Kupfer Berry"), Some(("kupfer-berry", "")));
        assert_eq!(
            grade_for("Rotguss, sauber"),
            Some(("bronze-rotguss", "sauber"))
        );
        assert_eq!(
            grade_for("V2A max.1,5x0,5x0,5m"),
            Some(("edelstahl-v2a", "max. 1,5x0,5x0,5m"))
        );
        assert_eq!(grade_for("Hartmetall, sortiert"), Some(("hartmetall", "")));
        assert_eq!(kind_for("Hartmetall, sortiert"), ("approx", Some(0.5)));
        assert_eq!(kind_for("Kupfer Millberry"), ("exact", Some(1.0)));
        assert_eq!(grade_for("Altpapier"), None);
        assert_eq!(grade_for("Alu Kabel ab 100kg"), None);
    }

    #[test]
    fn copper_and_aluminium_busbars_keep_separate_materials() {
        assert_eq!(
            grade_for("Alu Leitschienen, blank"),
            Some(("aluminium-gemischt", "Leitschienen blank"))
        );
        assert_eq!(
            grade_for("Kupfer Leitsch. blank"),
            Some(("kupfer-gemischt", "Leitschienen blank"))
        );
        assert_eq!(grade_for("Leitschienen blank"), None);
    }

    #[test]
    fn impressum_extracts_contact() {
        let imp = "<div class=\"et_pb_text_inner\">\
            <p><strong>Anbieterkennzeichnung nach TMG §5</strong></p>\
            <p>2nd way Recycling GmbH<br />Fritz-Reuter-Str.41<br />01097 Dresden</p>\
            <p>Tel.: 0173 5963719<br />Internet: www.second-way.de<br />E-Mail: info@second-way.de</p>\
            <p>Geschäftsführer: Stefan Lepke</p></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Fritz-Reuter-Str.41");
        assert_eq!(info.postcode, "01097");
        assert_eq!(info.city, "Dresden");
        assert_eq!(info.phone, "0173 5963719");
        assert_eq!(info.email, "info@second-way.de");
        assert!(extract_info("<html><body><p>Neu hier</p></body></html>").is_err());
    }
}
