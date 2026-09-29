//! Frisch Recycling GmbH (Schäftlarn): exact per-kg list prices in Elementor
//! `li.elementor-price-list-item` cards (`span.elementor-price-list-title`
//! + `span.elementor-price-list-price`), grouped under eleven `h4` section
//! headings. Live 28.09.2026: 135 cards, one without a price
//! ("Echtschmuck — Wird zur Analyse eingereicht").
//!
//! Deliberate exclusions, all loud:
//! - The hidden legacy `table.pricetable` (section `elementor-hidden-*` on
//!   every viewport, stale values: Hartmetall 30 € vs. 55 € visible,
//!   Eisenschrott 0,08 € vs. 0,10 € visible) is NEVER parsed — only the
//!   visible price-list cards. A regression test pins this.
//! - CPUs (Keramik/Kunststoff/SUN), RAM ("Speicherkarten mit Goldkante"),
//!   whole devices (Smartphones/Handys/Computer/Netzteile/Laptops/Lüfter/
//!   Rauchmelder/Laufwerke/Festplatten), Nickel, Batterien, Trafos,
//!   Cu-Fe-Ankerschrott, Leitschienen, mixed coolers/targets (CU-MS,
//!   Al-Cu), Alu-Baugruppen, Pumpen, "Stecker blank", silver-plated ware,
//!   CD/DVD have no catalog material and skip loudly (koppe/lungwitz/
//!   vedder precedent: no `cpu`/`ram`/`e-schrott-geraete`/`nickel`
//!   materials — proposals in `grade_for`).
//! - "Alle angegebenen Preise sind Kilogramm-Preise (€/kg)" is the
//!   documented page default; an explicitly foreign unit still skips.
//! - No price-validity date anywhere (29.09.2026 is a closure notice), so
//!   `published_at` stays `None`.
//! - Mindestmengen sentence proves two acceptances (`mischschrott` ab
//!   50 kg, `platinen` ab 1 kg); "Elektroschrott ab 30 kg" names no
//!   catalog material and stays a comment.

use scraper::{Html, Selector};

use super::super::{
    fetch_text, parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice,
    TraderInfo,
};
use crate::IngestError;

pub const SLUG: &str = "by-schaftlarn-frisch-recycling";
/// Bespoke, live-verified impressum URL (site footer "Impressum" link).
/// A move fails the step loudly (fix the URL) — never guessed, never shared.
pub const IMPRESSUM_URL: &str = "https://frisch-recycling.de/impressum/";

pub const URL: &str = "https://frisch-recycling.de/ankauf/";

/// Verbatim Mindestmengen sentence on the price page (acceptance source).
const MINDEST_SENTENCE: &str =
    "Unsere Annahme gilt ab definierten Mindestmengen – z. B. Eisenschrott ab 50 kg, \
     Elektroschrott ab 30 kg, Leiterplatten und Chips ab 1 kg.";

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
    let (rows, mut skipped_labels) = parse(&html)?;
    let mut prices = Vec::with_capacity(rows.len());
    let mut seen = std::collections::HashSet::new();
    for (label, price, unit) in rows {
        match grade_for(&label) {
            Some((material, variant)) => {
                // Doppelblock-Guard: same (material, variant, price) twice
                // means the page repeats a section — keep the first.
                if seen.insert((material, variant, price.to_bits())) {
                    prices.push(ScrapedPrice {
                        material,
                        variant,
                        price,
                        currency: "EUR",
                        unit,
                        price_kind: "exact",
                        price_min: None,
                        price_max: None,
                        confidence: Some(1.0),
                        label,
                    });
                }
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
        acceptances: vec![
            ScrapedAcceptance {
                material: "mischschrott",
                conditions: "Annahme ab 50 kg".to_owned(),
                label: MINDEST_SENTENCE.to_owned(),
            },
            ScrapedAcceptance {
                material: "platinen",
                conditions: "Annahme ab 1 kg".to_owned(),
                label: MINDEST_SENTENCE.to_owned(),
            },
        ],
        trader_info,
        website_alive: true,
        skipped_labels,
        fetch_url: URL.to_owned(),
        status_code: status,
        byte_len: html.len(),
        published_at: None,
    })
}

/// Explicit label → (material, variant) mapping, specific-before-generic.
/// Anything unlisted returns None (loud skip at the call site).
///
/// Catalog-gap proposals (do NOT cram):
/// - "Keramik/Kunststoff CPU …", "SUN Keramik" → new `cpu` material
///   (ceramic/goldcap/plastic are own grades).
/// - "Speicherkarten mit Gold/Silberkante" → new `ram` material.
/// - "Smartphones/Handys/Computer/Netzteile/Laptops/Lüfter/Rauchmelder/
///   Laufwerke/Festplatten" → new `e-schrott-geraete` material.
/// - "Nickel …" → new `nickel` material.
/// - "Batterien" → new `batterien` material.
/// - "Trafo …", "Cu-Fe Ankerschrott", "Kupfer Leitschienen ECU" →
///   unproven grades, no catalog material.
/// - "CU-MS/Al-Cu Kühler …", "Alu Cu Targets" → mixed-alloy, no catalog.
/// - "Alu Baugruppen", "Pumpen", "Stecker blank" → ambiguous assemblies.
/// - "… versilbert …" (Geschirr/Messer/Besteck) → silver-plated ware is
///   not `silber` (vedder precedent).
/// - "Echtschmuck", "CD / DVD" → no price / no material.
fn grade_for(label: &str) -> Option<(&'static str, &'static str)> {
    let l = label.to_lowercase();
    let l = l.as_str();
    // No-catalog families first: their keywords must never reach a
    // generic arm ("handy" is inside "handyplatinen", "festplatten"
    // inside "festplatten platinen").
    if l.contains("cpu") || l.contains("prozessor") || l.contains("sun keramik") {
        return None;
    }
    if l.contains("speicherkarte") {
        return None;
    }
    if l.contains("handyplatinen") {
        return Some(("platinen", "Handy"));
    }
    if l.contains("smartphone") || (l.contains("handy") && !l.contains("platine")) {
        return None;
    }
    if l.contains("festplatten platinen") {
        return Some(("platinen", "Festplatte"));
    }
    if l.contains("festplatte") && !l.contains("leiterplatte") {
        // Bare "Festplatten" (whole devices, no catalog material). The
        // guard keeps "Leiterplatten aus Festplatten" for the platinen
        // arm below ("festplatten" contains "festplatte").
        return None;
    }
    if l.contains("laufwerk-platine") {
        return Some(("platinen", "Laufwerk"));
    }
    if l.contains("computerschrott")
        || l.contains("netzteil")
        || (l.contains("laptop") && !l.contains("leiterplatte"))
        || (l.contains("notebook") && !l.contains("leiterplatte"))
        || l.contains("lüfter")
        || l.contains("luefter")
        || l.contains("rauchmelder")
        || l.contains("laufwerk")
    {
        // Guards keep "Laptop-Leiterplatten" for the platinen arm below.
        return None;
    }
    if l.contains("leiterplatte") {
        if l.contains("güteklasse 2 gemischt") || l.contains("gueteklasse 2 gemischt") {
            return Some(("platinen", "Güteklasse 2 gemischt"));
        } else if l.contains("güteklasse 1") || l.contains("gueteklasse 1") {
            return Some(("platinen", "Güteklasse 1"));
        } else if l.contains("güteklasse 2") || l.contains("gueteklasse 2") {
            return Some(("platinen", "Güteklasse 2"));
        } else if l.contains("güteklasse 3") || l.contains("gueteklasse 3") {
            return Some(("platinen", "Güteklasse 3"));
        } else if l.contains("festplatte") {
            return Some(("platinen", "aus Festplatten"));
        } else if l.contains("laptop") {
            return Some(("platinen", "Laptop"));
        } else if l.contains("kühler") || l.contains("kuehler") {
            return Some(("platinen", "mit Kühler"));
        } else if l.contains("unbestückt") || l.contains("unbestueckt") {
            return Some(("platinen", "unbestückt"));
        } else if l.contains("server") {
            return Some(("platinen", "von Server"));
        } else if l.contains("gedeckelt") {
            return Some(("platinen", "gedeckelt"));
        } else if l.contains("rückwänd") || l.contains("rueckwaend") || l.contains("rückwand") {
            return Some(("platinen", "Rückwände"));
        } else {
            return Some(("platinen", ""));
        }
    }
    if l.contains("steckkarte") {
        if l.contains("ohne metallblende") {
            return Some(("platinen", "Steckkarte ohne Blende"));
        } else {
            return Some(("platinen", "Steckkarte mit Blende"));
        }
    }
    if l.contains("stecker") && !l.contains("kabel") {
        // Bare "Stecker blank" (ambiguous pins, no catalog). The guard
        // keeps "Kabel mit/ohne Stecker" for the kabel arm below.
        return None;
    }
    if l.contains("keramikplatte") {
        return Some(("platinen", "Keramikplatte"));
    }
    // Copper: millberri before berri ("millberri" contains "berri").
    if l.contains("millberri") {
        return Some(("kupfer-millberry", ""));
    } else if l.contains("berri") {
        return Some(("kupfer-berry", ""));
    } else if l.contains("kupfer") {
        if l.contains("leitschiene") {
            return None;
        } else if l.contains("granulat") {
            return Some(("kupfer-gemischt", "Granulat"));
        } else if l.contains("pellet") {
            return Some(("kupfer-gemischt", "Pellets"));
        } else if l.contains("späne") || l.contains("spaene") {
            return Some(("kupfer-gemischt", "Späne"));
        } else if l.contains("target") {
            return Some(("kupfer-gemischt", "Targets"));
        } else if l.contains("teer") {
            return Some(("kupfer-gemischt", "mit Teeranhaftung"));
        } else if l.contains("leicht") {
            return Some(("kupfer-gemischt", "leicht"));
        } else if l.contains("schwer") {
            return Some(("kupfer-gemischt", "schwer"));
        } else if l.contains("neu") {
            return Some(("kupfer-gemischt", "neu"));
        } else {
            return None;
        }
    }
    if l.contains("e-motor") || l.contains("emotor") {
        // "ohne Getriebe" contains "getriebe" — negation first.
        if l.contains("ohne getriebe") {
            return Some(("elektromotoren", ""));
        } else if l.contains("getriebe") {
            return Some(("elektromotoren", "mit Getriebe"));
        } else {
            return Some(("elektromotoren", ""));
        }
    }
    if l.contains("anker") || l.contains("cu-fe") {
        return None;
    }
    if l.contains("trafo") {
        return None;
    }
    if l.contains("nickel") {
        return None;
    }
    // Stainless: V2A/V4A before bare chromstahl (generic, never a grade).
    if l.contains("v2a") {
        if l.contains("unrein") {
            return Some(("edelstahl-v2a", "unrein"));
        } else if l.contains("kühler") || l.contains("kuehler") {
            return Some(("edelstahl-v2a", "Kühler"));
        } else if l.contains("späne") || l.contains("spaene") {
            return Some(("edelstahl-v2a", "Späne"));
        } else {
            return Some(("edelstahl-v2a", ""));
        }
    }
    if l.contains("v4a") {
        return Some(("edelstahl-v4a", ""));
    }
    if l.contains("chromstahl") {
        return Some(("edelstahl-gemischt", "Chromstahl"));
    }
    // Brass/bronze: silver-plated and mixed coolers have no catalog.
    if l.contains("versilbert") {
        return None;
    }
    if l.contains("cu-ms") || l.contains("cu—ms") {
        return None;
    }
    if l.contains("rot-gu") || l.contains("rotgu") {
        return Some(("bronze-rotguss", ""));
    }
    if l.contains("friedhof") {
        return Some(("bronze-rotguss", "Friedhof"));
    }
    if l.contains("messing") {
        if l.contains("58") {
            return Some(("messing", "58"));
        } else if l.contains("späne rein") || l.contains("spaene rein") {
            return Some(("messing", "Späne rein"));
        } else if l.contains("späne unrein") || l.contains("spaene unrein") {
            return Some(("messing", "Späne unrein"));
        } else if l.contains("patronenhüls") || l.contains("patronenhuls") {
            return Some(("messing", "Patronenhülsen"));
        } else if l.contains("draht") {
            return Some(("messing", "Draht grau"));
        } else if l.contains("wasserzähler") || l.contains("wasserzaehler") {
            return Some(("messing", "Wasserzähler"));
        } else if l.contains("unrein") {
            return Some(("messing", "unrein"));
        } else {
            return Some(("messing", ""));
        }
    }
    // Cable: alu cable before bare "kabel" (never a guessed kupfer).
    if l.contains("alu-kabel") || l.contains("alu kabel") {
        return Some(("kabel-alu", ""));
    } else if l.contains("kabel") {
        if l.contains("mit stecker") {
            return Some(("kabel-kupfer", "mit Stecker"));
        } else if l.contains("ohne stecker") {
            return Some(("kabel-kupfer", "ohne Stecker"));
        } else if l.contains("schlitz") || l.contains("60") {
            return Some(("kabel-kupfer", "60%"));
        } else if l.contains("telefon") {
            return Some(("kabel-kupfer", "Telefon"));
        } else if l.contains("litzen") {
            return Some(("kabel-kupfer", "Litze"));
        } else if l.contains("erdkabel") {
            return Some(("kabel-kupfer", "Erd"));
        } else if l.contains("daten") {
            return Some(("kabel-kupfer", "Daten"));
        } else {
            return Some(("kabel-kupfer", ""));
        }
    }
    // Aluminium: mixed coolers/targets and assemblies have no catalog.
    if l.contains("alu") {
        if l.contains("cu kühler")
            || l.contains("cu kuehler")
            || l.contains("alu - cu")
            || l.contains("alu-cu")
            || l.contains("al-cu")
            || l.contains("cu target")
        {
            return None;
        }
        if l.contains("baugruppe") {
            return None;
        }
        if l.contains("felge") {
            if l.contains("unrein") {
                return Some(("aluminium-guss", "Felgen unrein"));
            } else {
                return Some(("aluminium-guss", "Felgen rein"));
            }
        } else if l.contains("guß") || l.contains("guss") {
            if l.contains("alt") {
                return Some(("aluminium-guss", "alt"));
            } else {
                return Some(("aluminium-guss", "neu"));
            }
        } else if l.contains("profil") {
            if l.contains("lang") {
                return Some(("aluminium-profile", "lang"));
            } else if l.contains("kurz") {
                return Some(("aluminium-profile", "kurz"));
            } else {
                return Some(("aluminium-profile", "lackiert"));
            }
        } else if l.contains("offset") {
            return Some(("aluminium-blech", "Offset"));
        } else if l.contains("draht") {
            return Some(("aluminium-gemischt", "Draht"));
        } else if l.contains("späne") || l.contains("spaene") {
            return Some(("aluminium-gemischt", "Späne"));
        } else if l.contains("amg") {
            return Some(("aluminium-gemischt", "AMG"));
        } else if l.contains("kontruktal") {
            return Some(("aluminium-gemischt", "Kontruktal"));
        } else if l.contains("neu") {
            return Some(("aluminium-gemischt", "neu"));
        } else if l.contains("alt") {
            return Some(("aluminium-gemischt", "alt"));
        } else {
            return Some(("aluminium-gemischt", ""));
        }
    }
    if l.contains("batterie") {
        return None;
    }
    if l.contains("wuchtblei") {
        return Some(("blei", "Wucht"));
    } else if l.contains("schuß") || l.contains("schuss") {
        return Some(("blei", "Schuss"));
    } else if l.contains("weichblei") {
        return Some(("blei", ""));
    }
    if l.contains("zink") {
        if l.contains("guß") || l.contains("guss") {
            return Some(("zink", "Guss"));
        } else if l.contains("blech neu") {
            return Some(("zink", "Blech neu"));
        } else if l.contains("blech alt") {
            return Some(("zink", "Blech alt"));
        } else {
            return Some(("zink", ""));
        }
    }
    if l.contains("zinn") {
        if l.contains("60/40") {
            return Some(("zinn", "60/40"));
        } else if l.contains("geschirr") {
            return Some(("zinn", "Geschirr"));
        } else {
            return Some(("zinn", ""));
        }
    }
    // Iron: generic eisenschrott is the mischschrott bucket (esh/madi
    // precedent); cast grades ride as gussbruch variants.
    if l.contains("eisenschrott") {
        if l.contains("leicht") {
            return Some(("mischschrott", "leicht"));
        } else if l.contains("schwer") {
            return Some(("mischschrott", "schwer"));
        } else if l.contains("anhaftung") {
            return Some(("mischschrott", "mit Anhaftung"));
        } else {
            return Some(("mischschrott", ""));
        }
    }
    if l.contains("bremsscheibe") {
        return Some(("eisenschrott-gussbruch", "Bremsscheiben"));
    }
    if l.contains("baugu") {
        return Some(("eisenschrott-gussbruch", ""));
    }
    if l.contains("pumpe") {
        return None;
    }
    if l.contains("hartmetall") || l.contains("wendeplättchen") || l.contains("wendeplaettchen") {
        if l.contains("wende") {
            return Some(("hartmetall", "Wendeplättchen"));
        } else {
            return Some(("hartmetall", "Bohrer"));
        }
    }
    if l.contains("stahlspäne") || l.contains("stahlspaene") {
        return Some(("mischschrott", "Stahlspäne"));
    }
    if l.contains("echt") || l.contains("schmuck") {
        return None;
    }
    if l.contains("cd / dvd") || l.contains("cd/dvd") {
        return None;
    }
    None
}

/// Bespoke contact extraction for THIS impressum only, anchored on
/// `div.legal-section` with the `h5 "Frisch Recycling GmbH"` heading and
/// the `p "Geschäftsführer: Josef Frisch<br/>Kloster Schäftlarn 3 a<br/>
/// 82067 Schäftlarn"` block. Missing anchors mean the page changed shape
/// → loud error, never a guessed fallback. Phone from the raw window
/// after "Tel.:" (the number lives inside `<a href="tel:…">` in the same
/// `<br>`-part as the marker, so `<br>`-split line parsing would drop the
/// marker with the tag remnant), e-mail from the `mailto:` href (scraper
/// `text()` glues neighbours, so hrefs beat token splits).
fn extract_info(imp: &str) -> Result<TraderInfo, IngestError> {
    let err = |detail: &str| IngestError::Parse {
        url: IMPRESSUM_URL.to_owned(),
        detail: detail.to_owned(),
    };
    let i = imp
        .find("legal-section")
        .ok_or_else(|| err("Impressum-Block fehlt"))?;
    let tail = &imp[i..];
    let end = tail.find("Haftungsausschluss").unwrap_or(tail.len());
    let window = &tail[..end];
    if !window.contains("Frisch Recycling GmbH") {
        return Err(err("Impressum-Block fehlt"));
    }
    if !window.contains("Geschäftsführer: Josef Frisch") {
        return Err(err("Geschäftsführer-Block fehlt"));
    }
    let frag = Html::parse_fragment(&format!("<div>{window}</div>"));
    let p_sel = Selector::parse("p").expect("valid selector");
    let mut lines: Vec<String> = Vec::new();
    for el in frag.select(&p_sel) {
        for part in el.inner_html().split("<br") {
            let t = strip_fragment(part);
            if !t.is_empty() {
                lines.push(t);
            }
        }
    }
    let (mut street, mut postcode, mut city) = (String::new(), String::new(), String::new());
    for (k, line) in lines.iter().enumerate() {
        let mut it = line.split_whitespace();
        if let (Some(pc), Some(ci)) = (it.next(), it.next()) {
            if pc.len() == 5
                && pc.chars().all(|c| c.is_ascii_digit())
                && ci.chars().next().is_some_and(|c| c.is_uppercase())
            {
                postcode = pc.to_owned();
                city = ci.trim_matches(',').to_owned();
                if k > 0 {
                    street = lines[k - 1].clone();
                }
                break;
            }
        }
    }
    let phone = phone_from_window(window);
    let email = mailto(window).unwrap_or_else(|| email_token(&lines.join(" ")));
    if street.is_empty() && phone.is_empty() && email.is_empty() {
        return Err(err("keine Kontaktdaten gefunden"));
    }
    Ok(TraderInfo {
        street,
        postcode,
        city,
        phone,
        email,
    })
}

/// First `mailto:` href in the raw window (glued text defeats splits).
fn mailto(window: &str) -> Option<String> {
    let i = window.find("mailto:")?;
    let rest = &window[i + "mailto:".len()..];
    let end = rest
        .find(|c| c == '"' || c == '\'' || c == '>' || c == '?')
        .unwrap_or(rest.len());
    let mail = rest[..end].trim().to_owned();
    if mail.contains('@') {
        Some(mail)
    } else {
        None
    }
}

/// Phone from the raw impressum window: tag-stripped text after "Tel.:"
/// (live: `Tel.: <a href="tel:08178955003">08178 / 9550-03</a>` — marker
/// and number share one `<br>`-part, so line parsing drops the marker).
fn phone_from_window(window: &str) -> String {
    let Some(i) = window.find("Tel.:") else {
        return String::new();
    };
    let after = &window[i + "Tel.:".len()..];
    let mut text = String::new();
    let mut in_tag = false;
    for c in after.chars() {
        match c {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                text.push(' ');
            }
            _ if !in_tag => text.push(c),
            _ => {}
        }
    }
    phone_after(&text, "")
}

/// Phone-ish token run after a marker ("Tel.: 08178 / 9550-03"). An empty
/// marker starts at the (already marker-stripped) text head.
fn phone_after(text: &str, marker: &str) -> String {
    text.find(marker).map_or_else(String::new, |i| {
        text[i + marker.len()..]
            .split_whitespace()
            .take_while(|t| {
                t.chars()
                    .all(|c| c.is_ascii_digit() || "+/().-".contains(c))
            })
            .collect::<Vec<_>>()
            .join(" ")
    })
}

/// Strip tags from a `<br`-split fragment. Fragments start with a tag
/// remnant (`/>`) — drop everything up to the first '>' first, or the
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

/// First email address in the text: expand from the '@' over email
/// characters (glued neighbours defeat token splitting). Cut at the
/// domain end so trailing prose never sticks.
fn email_token(r: &str) -> String {
    let Some(at) = r.find('@') else {
        return String::new();
    };
    let b = r.as_bytes();
    let is_email = |c: u8| c.is_ascii_alphanumeric() || b".-_+@".contains(&c);
    let mut s = at;
    while s > 0 && is_email(b[s - 1]) {
        s -= 1;
    }
    let mut e = at + 1;
    while e < b.len() && is_email(b[e]) {
        e += 1;
    }
    let cand = &r[s..e];
    for suffix in [".de", ".com", ".net", ".org", ".eu", ".info", ".biz"] {
        if let Some(p) = cand.rfind(suffix) {
            let cut = cand[..p + suffix.len()].to_owned();
            if cut.contains('@') && !cut.starts_with('@') {
                return cut;
            }
        }
    }
    String::new()
}

/// Visible price-list cards only: `.elementor-price-list-item` between
/// the "Unsere Preise" heading and the "Jetzt Schrott verkaufen" block.
/// The class usually sits on the `li`, but live at least one card
/// ("Datenkabel") wraps it in `<a class="elementor-price-list-item">` —
/// so select by class, never by `li`. The hidden legacy
/// `table.pricetable` is excluded BY CONSTRUCTION (it uses `tr`/`td`,
/// never price-list items) — a stale duplicate with deviating values,
/// never a second source.
fn parse(html: &str) -> Result<(Vec<(String, f64, &'static str)>, Vec<String>), IngestError> {
    let start = html
        .find("Unsere Preise")
        .ok_or_else(|| IngestError::Parse {
            url: URL.to_owned(),
            detail: "Unsere-Preise-Block fehlt".to_owned(),
        })?;
    let tail = &html[start..];
    let end = tail.find("Jetzt Schrott verkaufen").unwrap_or(tail.len());
    let window = &tail[..end];
    if !window.contains("Kilogramm-Preise") {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "kg-Einheitsanker fehlt".to_owned(),
        });
    }
    let doc = Html::parse_fragment(&format!("<div>{window}</div>"));
    let item_sel = Selector::parse(".elementor-price-list-item").expect("valid selector");
    let title_sel = Selector::parse("span.elementor-price-list-title").expect("valid selector");
    let price_sel = Selector::parse("span.elementor-price-list-price").expect("valid selector");
    let mut rows = Vec::new();
    let mut skips = Vec::new();
    for item in doc.select(&item_sel) {
        let title = item
            .select(&title_sel)
            .next()
            .map(|el| {
                el.text()
                    .collect::<String>()
                    .replace(['\u{a0}'], " ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();
        let price_raw = item
            .select(&price_sel)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        if title.is_empty() || title.len() > 120 {
            continue; // Prosa, nie ein Label.
        }
        let Some(price) = parse_eur(&price_raw) else {
            skips.push(format!("{title} (kein Preis: {})", price_raw.trim()));
            continue;
        };
        if price == 0.0 {
            skips.push(format!("{title} (0,00 €)"));
            continue;
        }
        let Some(unit) = unit_of(&price_raw) else {
            skips.push(format!(
                "{title} (Einheit unverständlich: {})",
                price_raw.trim()
            ));
            continue;
        };
        rows.push((title, price, unit));
    }
    if rows.is_empty() {
        return Err(IngestError::Parse {
            url: URL.to_owned(),
            detail: "Preisliste leer".to_owned(),
        });
    }
    Ok((rows, skips))
}

/// Bespoke unit rule for THIS page: the "Unsere Preise" box states "Alle
/// angegebenen Preise sind Kilogramm-Preise (€/kg)", so EUR/kg is the
/// documented page default. An explicitly foreign unit still skips loudly —
/// a per-tonne price recorded as per-kg would be a 1000x error.
fn unit_of(price_raw: &str) -> Option<&'static str> {
    let lower = price_raw.to_lowercase();
    if lower.contains("pro tonne")
        || lower.contains("pro to")
        || lower.contains("/t")
        || lower.contains("pro sack")
        || lower.contains("stk")
        || lower.contains("stück")
        || lower.contains("stueck")
        || lower.contains("pauschal")
    {
        None
    } else {
        Some("EUR/kg")
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_info, grade_for, parse, unit_of};

    // Real live-HTML excerpts (only whitespace shortened): Elementor
    // price-list cards incl. the "Wird zur Analyse eingereicht" card, the
    // kg-unit anchor, the end anchor, and one hidden-table row with a
    // deviating stale price (must be ignored).
    const FIXTURE: &str = concat!(
        "<h3>Unsere Preise</h3>",
        "<span>Alle angegebenen Preise sind Kilogramm-Preise (€/kg)</span>",
        "<ul class=\"elementor-price-list\">",
        "<li class=\"elementor-price-list-item\"><div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\"> Keramik CPU braun </span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">70 €</span>",
        "</div></div></li>",
        "<li class=\"elementor-price-list-item\"><div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\">Leiterplatten Güteklasse 1</span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">10 €</span>",
        "</div></div></li>",
        "<li class=\"elementor-price-list-item\"><div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\">MillBerri</span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">10,90 €</span>",
        "</div></div></li>",
        "<li class=\"elementor-price-list-item\"><div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\">Kupfer Draht Berri</span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">9,50 €</span>",
        "</div></div></li>",
        "<li class=\"elementor-price-list-item\"><div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\">Alu-Kabel</span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">0,90 €</span>",
        "</div></div></li>",
        "<li class=\"elementor-price-list-item\"><div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\">V2A rein</span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">0,80 €</span>",
        "</div></div></li>",
        "<li class=\"elementor-price-list-item\"><div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\">Echtschmuck</span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">Wird zur Analyse eingereicht</span>",
        "</div></div></li>",
        // Live quirk: one card wraps the class in a link (`<a>` instead
        // of `<li>`) — the class selector must catch it too.
        "<li><a class=\"elementor-price-list-item\" href=\"#\">",
        "<div class=\"elementor-price-list-text\">",
        "<div class=\"elementor-price-list-header\">",
        "<span class=\"elementor-price-list-title\">Datenkabel</span>",
        "<span class=\"elementor-price-list-separator\"></span>",
        "<span class=\"elementor-price-list-price\">1,80 €</span>",
        "</div></div></a></li>",
        "</ul>",
        "<table class=\"pricetable\"><tbody>",
        "<tr><td>Hartmetall-Bohrer</td><td>kg</td><td>30,00 €</td></tr>",
        "</tbody></table>",
        "<h3>Jetzt Schrott verkaufen</h3>",
    );

    #[test]
    fn cards_parse_hidden_table_ignored() {
        let (rows, skips) = parse(FIXTURE).expect("parses");
        assert_eq!(rows.len(), 7, "{rows:?}");
        let get = |l: &str| {
            rows.iter()
                .find(|(x, _, _)| x == l)
                .map(|(_, p, u)| (*p, *u))
        };
        assert_eq!(get("Keramik CPU braun"), Some((70.0, "EUR/kg")));
        assert_eq!(get("Leiterplatten Güteklasse 1"), Some((10.0, "EUR/kg")));
        assert_eq!(get("MillBerri"), Some((10.9, "EUR/kg")));
        assert_eq!(get("Kupfer Draht Berri"), Some((9.5, "EUR/kg")));
        assert_eq!(get("Alu-Kabel"), Some((0.9, "EUR/kg")));
        assert_eq!(get("Datenkabel"), Some((1.8, "EUR/kg")));
        assert_eq!(get("V2A rein"), Some((0.8, "EUR/kg")));
        // Stale hidden-table row never becomes a card …
        assert!(get("Hartmetall-Bohrer").is_none());
        // … and the no-price card skips loudly instead of failing.
        assert_eq!(skips.len(), 1);
        assert!(skips[0].contains("Echtschmuck") && skips[0].contains("kein Preis"));
    }

    #[test]
    fn anchors_fail_loudly() {
        assert!(parse("<p>Neu hier</p>").is_err());
        let no_unit = FIXTURE.replace("Kilogramm-Preise", "Tagespreise");
        assert!(parse(&no_unit).is_err());
        let no_cards = concat!(
            "<h3>Unsere Preise</h3>",
            "<span>Alle angegebenen Preise sind Kilogramm-Preise (€/kg)</span>",
            "<h3>Jetzt Schrott verkaufen</h3>",
        );
        assert!(parse(no_cards).is_err());
    }

    #[test]
    fn foreign_unit_skips() {
        assert_eq!(unit_of("70 €"), Some("EUR/kg"));
        assert_eq!(unit_of("10,90 €"), Some("EUR/kg"));
        assert_eq!(unit_of("220 €/t"), None);
        assert_eq!(unit_of("5 € pro Sack"), None);
        assert_eq!(unit_of("18 €/Stk"), None);
    }

    #[test]
    fn impressum_legal_section() {
        let imp = "<div class=\"legal-section\"><h5>Frisch Recycling GmbH</h5>\
            <p>Geschäftsführer: Josef Frisch<br />Kloster Schäftlarn 3 a<br />82067 Schäftlarn</p>\
            <p>Tel.: <a href=\"tel:08178955003\">08178 / 9550-03</a><br />\
            E-Mail: <a href=\"mailto:frisch-recycling@t-online.de\">frisch-recycling@t-online.de</a></p>\
            <h5>Haftungsausschluss (Disclaimer)</h5></div>";
        let info = extract_info(imp).expect("parses");
        assert_eq!(info.street, "Kloster Schäftlarn 3 a");
        assert_eq!(info.postcode, "82067");
        assert_eq!(info.city, "Schäftlarn");
        assert_eq!(info.phone, "08178 / 9550-03");
        assert_eq!(info.email, "frisch-recycling@t-online.de");
        assert!(extract_info("<div>Neu hier</div>").is_err());
        assert!(extract_info("<div class=\"legal-section\"><h5>Anderer Name</h5></div>").is_err());
    }

    #[test]
    fn mapping_covers_every_live_label() {
        // Mapped (live 28.09.2026): platinen grades …
        assert_eq!(
            grade_for("Leiterplatten Güteklasse 1"),
            Some(("platinen", "Güteklasse 1"))
        );
        assert_eq!(
            grade_for("Leiterplatten Güteklasse 2"),
            Some(("platinen", "Güteklasse 2"))
        );
        assert_eq!(
            grade_for("Leiterplatten Güteklasse 3"),
            Some(("platinen", "Güteklasse 3"))
        );
        assert_eq!(
            grade_for("Leiterplatten Güteklasse 2 gemischt"),
            Some(("platinen", "Güteklasse 2 gemischt"))
        );
        assert_eq!(
            grade_for("Leiterplatten aus Festplatten"),
            Some(("platinen", "aus Festplatten"))
        );
        assert_eq!(
            grade_for("Laptop-Leiterplatten"),
            Some(("platinen", "Laptop"))
        );
        assert_eq!(
            grade_for("Leiterplatten mit Kühler"),
            Some(("platinen", "mit Kühler"))
        );
        assert_eq!(
            grade_for("Leiterplatten unbestückt"),
            Some(("platinen", "unbestückt"))
        );
        assert_eq!(
            grade_for("Leiterplatten von Server"),
            Some(("platinen", "von Server"))
        );
        assert_eq!(
            grade_for("Leiterplatten gedeckelt"),
            Some(("platinen", "gedeckelt"))
        );
        assert_eq!(
            grade_for("Laufwerk-Platinen"),
            Some(("platinen", "Laufwerk"))
        );
        assert_eq!(
            grade_for("Leiterplatten Rückwände"),
            Some(("platinen", "Rückwände"))
        );
        assert_eq!(
            grade_for("Steckkarten ohne Metallblende"),
            Some(("platinen", "Steckkarte ohne Blende"))
        );
        assert_eq!(
            grade_for("Steckkarten mit Metallblende"),
            Some(("platinen", "Steckkarte mit Blende"))
        );
        assert_eq!(grade_for("Handyplatinen"), Some(("platinen", "Handy")));
        assert_eq!(
            grade_for("Festplatten Platinen"),
            Some(("platinen", "Festplatte"))
        );
        assert_eq!(
            grade_for("Keramikplatten weiß / braun"),
            Some(("platinen", "Keramikplatte"))
        );
        // … copper (millberri before berri) …
        assert_eq!(grade_for("MillBerri"), Some(("kupfer-millberry", "")));
        assert_eq!(grade_for("Kupfer Draht Berri"), Some(("kupfer-berry", "")));
        assert_eq!(
            grade_for("Kupfer leicht"),
            Some(("kupfer-gemischt", "leicht"))
        );
        assert_eq!(
            grade_for("Kupfer schwer"),
            Some(("kupfer-gemischt", "schwer"))
        );
        assert_eq!(grade_for("Kupfer neu"), Some(("kupfer-gemischt", "neu")));
        assert_eq!(
            grade_for("Kupfer Targets"),
            Some(("kupfer-gemischt", "Targets"))
        );
        assert_eq!(
            grade_for("Kupfer Späne"),
            Some(("kupfer-gemischt", "Späne"))
        );
        assert_eq!(
            grade_for("Kupfer m. Teeranhaftung"),
            Some(("kupfer-gemischt", "mit Teeranhaftung"))
        );
        assert_eq!(
            grade_for("Kupfer Granulat"),
            Some(("kupfer-gemischt", "Granulat"))
        );
        assert_eq!(
            grade_for("Kupfer Pellets"),
            Some(("kupfer-gemischt", "Pellets"))
        );
        // … motors, stainless, brass/bronze …
        assert_eq!(
            grade_for("E-Motore ohne Getriebe"),
            Some(("elektromotoren", ""))
        );
        assert_eq!(
            grade_for("E-Motore mit Getriebe"),
            Some(("elektromotoren", "mit Getriebe"))
        );
        assert_eq!(grade_for("V2A rein"), Some(("edelstahl-v2a", "")));
        assert_eq!(grade_for("V2A unrein"), Some(("edelstahl-v2a", "unrein")));
        assert_eq!(grade_for("V2A Kühler"), Some(("edelstahl-v2a", "Kühler")));
        assert_eq!(grade_for("V2A Späne"), Some(("edelstahl-v2a", "Späne")));
        assert_eq!(grade_for("V4A"), Some(("edelstahl-v4a", "")));
        assert_eq!(
            grade_for("Chromstahl"),
            Some(("edelstahl-gemischt", "Chromstahl"))
        );
        assert_eq!(grade_for("Messing"), Some(("messing", "")));
        assert_eq!(grade_for("Messing 58"), Some(("messing", "58")));
        assert_eq!(
            grade_for("Messing Späne rein"),
            Some(("messing", "Späne rein"))
        );
        assert_eq!(
            grade_for("Messing Späne unrein"),
            Some(("messing", "Späne unrein"))
        );
        assert_eq!(
            grade_for("Messing Patronenhülsen abgeschlossen"),
            Some(("messing", "Patronenhülsen"))
        );
        assert_eq!(
            grade_for("Messing-Draht grau"),
            Some(("messing", "Draht grau"))
        );
        assert_eq!(
            grade_for("Messing Wasserzähler"),
            Some(("messing", "Wasserzähler"))
        );
        assert_eq!(grade_for("Messing unrein"), Some(("messing", "unrein")));
        assert_eq!(grade_for("Rot-Guß"), Some(("bronze-rotguss", "")));
        assert_eq!(
            grade_for("Friedhofs-Bronze"),
            Some(("bronze-rotguss", "Friedhof"))
        );
        // … cable (alu before bare kabel) …
        assert_eq!(
            grade_for("Kabel mit Stecker"),
            Some(("kabel-kupfer", "mit Stecker"))
        );
        assert_eq!(
            grade_for("Kabel ohne Stecker"),
            Some(("kabel-kupfer", "ohne Stecker"))
        );
        assert_eq!(
            grade_for("Schlitzkabel 60 %ig"),
            Some(("kabel-kupfer", "60%"))
        );
        assert_eq!(grade_for("Telefonkabel"), Some(("kabel-kupfer", "Telefon")));
        assert_eq!(grade_for("Litzenkabel"), Some(("kabel-kupfer", "Litze")));
        assert_eq!(grade_for("Erdkabel"), Some(("kabel-kupfer", "Erd")));
        assert_eq!(grade_for("Alu-Kabel"), Some(("kabel-alu", "")));
        assert_eq!(grade_for("Datenkabel"), Some(("kabel-kupfer", "Daten")));
        // … aluminium …
        assert_eq!(grade_for("Misch-Alu"), Some(("aluminium-gemischt", "")));
        assert_eq!(grade_for("Alu neu"), Some(("aluminium-gemischt", "neu")));
        assert_eq!(grade_for("Alu alt"), Some(("aluminium-gemischt", "alt")));
        assert_eq!(
            grade_for("Alu Felgen rein"),
            Some(("aluminium-guss", "Felgen rein"))
        );
        assert_eq!(
            grade_for("Alu Felgen unrein"),
            Some(("aluminium-guss", "Felgen unrein"))
        );
        assert_eq!(
            grade_for("Alu Draht rein"),
            Some(("aluminium-gemischt", "Draht"))
        );
        assert_eq!(
            grade_for("Alu Späne"),
            Some(("aluminium-gemischt", "Späne"))
        );
        assert_eq!(grade_for("Alu Guß alt"), Some(("aluminium-guss", "alt")));
        assert_eq!(grade_for("Alu Guß neu"), Some(("aluminium-guss", "neu")));
        assert_eq!(
            grade_for("Alu Profile lang"),
            Some(("aluminium-profile", "lang"))
        );
        assert_eq!(
            grade_for("Alu Profile kurz"),
            Some(("aluminium-profile", "kurz"))
        );
        assert_eq!(grade_for("Alu Offset"), Some(("aluminium-blech", "Offset")));
        assert_eq!(
            grade_for("Alu Profile Lack"),
            Some(("aluminium-profile", "lackiert"))
        );
        assert_eq!(grade_for("ALU AMG"), Some(("aluminium-gemischt", "AMG")));
        assert_eq!(
            grade_for("Alu Kontruktal"),
            Some(("aluminium-gemischt", "Kontruktal"))
        );
        // … lead/zinc/tin, iron …
        assert_eq!(grade_for("Wuchtblei"), Some(("blei", "Wucht")));
        assert_eq!(grade_for("Schußblei"), Some(("blei", "Schuss")));
        assert_eq!(grade_for("Weichblei"), Some(("blei", "")));
        assert_eq!(grade_for("Zink-Guß"), Some(("zink", "Guss")));
        assert_eq!(grade_for("Zink-Blech neu"), Some(("zink", "Blech neu")));
        assert_eq!(grade_for("Zink-Blech alt"), Some(("zink", "Blech alt")));
        assert_eq!(grade_for("Zinn 60/40"), Some(("zinn", "60/40")));
        assert_eq!(grade_for("Zinn Geschirr"), Some(("zinn", "Geschirr")));
        assert_eq!(
            grade_for("Eisenschrott leicht"),
            Some(("mischschrott", "leicht"))
        );
        assert_eq!(
            grade_for("Eisenschrott schwer"),
            Some(("mischschrott", "schwer"))
        );
        assert_eq!(
            grade_for("Eisenschrott mit Anhaftung"),
            Some(("mischschrott", "mit Anhaftung"))
        );
        assert_eq!(
            grade_for("Bremsscheiben"),
            Some(("eisenschrott-gussbruch", "Bremsscheiben"))
        );
        assert_eq!(grade_for("Bauguß"), Some(("eisenschrott-gussbruch", "")));
        assert_eq!(
            grade_for("Hartmetall Bohrer"),
            Some(("hartmetall", "Bohrer"))
        );
        assert_eq!(
            grade_for("Hartmetall-Wendeplättchen"),
            Some(("hartmetall", "Wendeplättchen"))
        );
        assert_eq!(
            grade_for("Stahlspäne"),
            Some(("mischschrott", "Stahlspäne"))
        );
        // … and loudly skipped (no catalog material, proposals above).
        for l in [
            "Keramik CPU mit Kühlkörper",
            "Keramik CPU braun",
            "Keramik CPU mit Aludeckel",
            "Keramik CPU",
            "Keramikprozessoren Goldcap 1 seitig",
            "Keramikprozessoren Goldcap 2 seitig",
            "Keramik CPU Goldcap (286/386/486) von Intel",
            "Keramik CPU Intel / AMD Prozessoren",
            "Kunststoff CPU grün mit Kupferkern",
            "Kunststoff CPU grün ohne Kupferkern",
            "Kunststoff CPU braun / grün",
            "Kunststoff CPU schwarz",
            "SUN Keramik",
            "Stecker blank",
            "Speicherkarten mit Goldkante",
            "Speicherkarten mit Goldkante mit Alurahmen",
            "Speicherkarten mit Silberkante",
            "Festplatten",
            "Smartphones ohne Akku",
            "Smartphones mit Akku",
            "Handys ohne Akku",
            "Handys mit Akku",
            "Computerschrott",
            "Netzteile mit Kabel",
            "Netzteile ohne Kabel",
            "Laptop / Notebook ohne Akku",
            "Laptop / Notebook mit Akku",
            "Lüfter",
            "Rauchmelder",
            "Laufwerke",
            "Kupfer Leitschienen ECU",
            "Cu-Fe Ankerschrott",
            "Trafo",
            "Trafo in Gießharz",
            "Nickel 60%",
            "Nickel 99%",
            "Nickelpellets",
            "Nickelspäne",
            "Messing, versilbert Geschirr",
            "CU-MS Kühler rein",
            "CU-MS Kühler unrein",
            "Alu - Cu Kühler rein",
            "Alu - Cu Kühler unrein",
            "Alu Cu Targets",
            "Alu Baugruppen",
            "Batterien",
            "Pumpen",
            "Messer, versilbert",
            "Besteck, versilbert",
            "Echtschmuck",
            "CD / DVD",
        ] {
            assert_eq!(grade_for(l), None, "{l}");
        }
    }
}
