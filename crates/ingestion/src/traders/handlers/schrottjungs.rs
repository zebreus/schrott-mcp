//! One nationwide commercial reference feed, not Munich/Entel yard prices.
//! Every visible variant carries conditions and the unknown source-date basis.
//! Shared observed_price_change inference remains unchanged, never page-stated.

use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserInput, ParserState,
    RuleBodyItemParser, RuleBodyParser, Token,
};
use scraper::{ElementRef, Html, Selector};
use std::collections::{HashMap, HashSet};

use super::super::{parse_eur, Handler, HandlerOutcome, Schedule, ScrapedAcceptance, ScrapedPrice};
use crate::IngestError;

type CssVariables = HashMap<String, Option<String>>;

pub const SLUG: &str = "by-u-a-munchen-schrottjungs-mobil-bundesweit";
pub const URL: &str = "https://schrottjungs.de/gewerbe/";
const IMPRESSUM: &str = "https://schrottjungs.de/impressum-datenschutz/";
const CONDITIONS: &str = "Bundesweiter Gewerbe-/Großmengenrichtwert; unverbindlich bis zu; abhängig von Menge, Qualität, Sortierung und Marktlage; kleinere Mengen andere Konditionen; Tagesangebot auf Anfrage; kein Privat-/Entel-Tarif; Quelldatum unbekannt; ggf. Datum der Preisänderungsbeobachtung";
const ACCEPTANCE_SCOPE: &str =
    "Betreiberreferenz, keine Zusage einer Annahme durch einen lokalen Partnerhof";
const PAGE_CONTEXT: &str = include_str!("fixtures/schrottjungs-page-context-20261011.txt");
const PAGE_CONTEXT_ATTRIBUTES: &str =
    include_str!("fixtures/schrottjungs-page-context-attributes-20261011.txt");
#[cfg(test)]
const PAGE_FIXTURE: &str = include_str!("fixtures/schrottjungs-gewerbe-page-20261011.html");
const STATE_PSEUDOS: [&str; 11] = [
    "hover",
    "focus-visible",
    "focus-within",
    "focus",
    "active",
    "visited",
    "link",
    "target",
    "checked",
    "enabled",
    "disabled",
];

#[derive(Default)]
struct CssRules {
    hidden: Vec<Selector>,
    generated_text: Vec<Selector>,
    animated: Vec<Selector>,
    variables: CssVariables,
}

pub fn handler() -> Handler {
    Handler {
        slug: SLUG,
        url: URL,
        schedule: Schedule::every_6h(),
        scrape: |c| Box::pin(scrape(c)),
    }
}

fn error(detail: impl Into<String>) -> IngestError {
    IngestError::Parse {
        url: URL.to_owned(),
        detail: detail.into(),
    }
}

async fn owned_fetch(client: &reqwest::Client, url: &str) -> Result<String, IngestError> {
    let mut response = client
        .get(url)
        .send()
        .await
        .map_err(|source| IngestError::Fetch {
            url: url.to_owned(),
            source,
        })?;
    validate_response(url, response.url().as_str(), response.status().as_u16())?;
    const MAX_BYTES: usize = 4_000_000;
    if response
        .content_length()
        .is_some_and(|length| length > MAX_BYTES as u64)
    {
        return Err(error(format!("Quelltext zu groß: {url}")));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|source| IngestError::Body {
        url: url.to_owned(),
        source,
    })? {
        if bytes.len().saturating_add(chunk.len()) > MAX_BYTES {
            return Err(error(format!("Quelltext zu groß: {url}")));
        }
        bytes.extend_from_slice(&chunk);
    }
    String::from_utf8(bytes).map_err(|_| error(format!("Quelltext ist nicht UTF-8: {url}")))
}

fn source_client() -> Result<reqwest::Client, IngestError> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("schrott-mcp-ingestion/0.1")
        .build()
        .map_err(|source| IngestError::Fetch {
            url: URL.to_owned(),
            source,
        })
}

fn validate_response(expected: &str, actual: &str, status: u16) -> Result<(), IngestError> {
    if status != 200 || actual != expected {
        return Err(error(format!(
            "Quellidentität geändert: {actual} HTTP {status}"
        )));
    }
    Ok(())
}

async fn scrape(_shared_client: &reqwest::Client) -> Result<HandlerOutcome, IngestError> {
    // Every owned source request must retain its original URL. The shared
    // pipeline client follows redirects, so it is not safe for this handler.
    let client = source_client()?;
    let html = owned_fetch(&client, URL).await?;
    let imp = owned_fetch(&client, IMPRESSUM).await?;
    check_operator(&imp)?;
    parse_source(&html)
}

fn selector(s: &str) -> Selector {
    Selector::parse(s).expect("static selector")
}
fn text(el: ElementRef<'_>) -> String {
    el.text()
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn one<'a>(el: ElementRef<'a>, query: &str) -> Result<ElementRef<'a>, IngestError> {
    let sel = selector(query);
    let mut matches = el.select(&sel);
    let first = matches
        .next()
        .ok_or_else(|| error(format!("Anker fehlt: {query}")))?;
    if matches.next().is_some() {
        return Err(error(format!("Anker doppelt: {query}")));
    }
    Ok(first)
}

#[derive(Default, Clone, Copy)]
struct CssEffects {
    hidden: bool,
    generated_text: bool,
    animated: bool,
}

struct CssEffectParser<'a> {
    variables: &'a CssVariables,
}

impl<'a, 'i> DeclarationParser<'i> for CssEffectParser<'a> {
    type Declaration = CssEffects;
    type Error = ();

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        _declaration_start: &ParserState,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let start = input.position();
        let mut tokens = Vec::new();
        while let Ok(token) = input.next_including_whitespace().cloned() {
            if token.is_parse_error() {
                return Err(input.new_custom_error(()));
            }
            tokens.push(token);
        }
        let raw = input.slice(start..input.position());
        Ok(assess_declaration(&name, &tokens, raw, self.variables))
    }
}

impl<'i> AtRuleParser<'i> for CssEffectParser<'_> {
    type Prelude = ();
    type AtRule = CssEffects;
    type Error = ();
}

impl<'i> cssparser::QualifiedRuleParser<'i> for CssEffectParser<'_> {
    type Prelude = ();
    type QualifiedRule = CssEffects;
    type Error = ();
}

impl<'i> RuleBodyItemParser<'i, CssEffects, ()> for CssEffectParser<'_> {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

fn significant_tokens<'a, 'i>(tokens: &'a [Token<'i>]) -> Vec<&'a Token<'i>> {
    let mut significant: Vec<_> = tokens
        .iter()
        .filter(|token| !matches!(token, Token::WhiteSpace(_) | Token::Comment(_)))
        .collect();
    if significant.len() >= 2
        && matches!(significant[significant.len() - 2], Token::Delim('!'))
        && matches!(significant[significant.len() - 1], Token::Ident(name) if name.eq_ignore_ascii_case("important"))
    {
        significant.truncate(significant.len() - 2);
    }
    significant
}

fn single_ident<'a, 'i>(tokens: &'a [Token<'i>]) -> Option<&'a str> {
    match significant_tokens(tokens).as_slice() {
        [Token::Ident(value)] => Some(value.as_ref()),
        _ => None,
    }
}

fn single_numeric_value(tokens: &[Token<'_>]) -> Option<f32> {
    match significant_tokens(tokens).as_slice() {
        [Token::Number { value, .. }] => Some(*value),
        [Token::Percentage { unit_value, .. }] => Some(*unit_value),
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum CssAlpha {
    NoAlpha,
    Value(f32),
    Unknown,
}

fn parse_color_function_alpha<'i, 't>(
    parser: &mut Parser<'i, 't>,
    comma_alpha: bool,
) -> Result<CssAlpha, ParseError<'i, ()>> {
    parser.parse_nested_block(|nested| {
        let mut commas = 0;
        let mut alpha_started = false;
        let mut alpha = None;
        let mut unknown = false;
        let mut nested_function = false;
        while !nested.is_exhausted() {
            match nested.next_including_whitespace()? {
                Token::Comma => {
                    commas += 1;
                    if comma_alpha && commas == 3 {
                        alpha_started = true;
                    }
                }
                Token::Delim('/') => {
                    alpha_started = true;
                    alpha = None;
                    unknown = false;
                }
                Token::WhiteSpace(_) => {}
                Token::Number { value, .. } if alpha_started && alpha.is_none() => {
                    alpha = Some(*value);
                }
                Token::Percentage { unit_value, .. } if alpha_started && alpha.is_none() => {
                    alpha = Some(*unit_value);
                }
                Token::Function(_) => {
                    if alpha_started {
                        unknown = true;
                    } else {
                        nested_function = true;
                    }
                }
                _ if alpha_started => unknown = true,
                _ => {}
            }
        }

        Ok(if nested_function || unknown {
            CssAlpha::Unknown
        } else if alpha_started {
            alpha.map_or(CssAlpha::Unknown, CssAlpha::Value)
        } else {
            CssAlpha::NoAlpha
        })
    })
}

fn zero_alpha_hex(value: &str) -> bool {
    matches!(value.len(), 4 | 8) && value.ends_with('0')
}

/// Only explicit zero-alpha colors are treated as hidden. Inline custom
/// properties are resolved when unique; a variable sourced from an external
/// stylesheet is outside this HTML-only visibility check.
fn parse_var_name<'i, 't>(
    parser: &mut Parser<'i, 't>,
) -> Result<(String, bool), ParseError<'i, ()>> {
    parser.parse_nested_block(|nested| loop {
        match nested.next_including_whitespace()? {
            Token::WhiteSpace(_) => continue,
            Token::Ident(name) if name.starts_with("--") => {
                let name = name.to_string();
                let mut has_fallback = false;
                while !nested.is_exhausted() {
                    if matches!(nested.next_including_whitespace()?, Token::Comma) {
                        has_fallback = true;
                    }
                }
                return Ok((name, has_fallback));
            }
            _ => return Err(nested.new_custom_error(())),
        }
    })
}

fn color_may_hide(raw: &str, variables: &CssVariables, depth: usize) -> bool {
    if depth > 16 {
        return true;
    }
    const COLOR_FUNCTIONS: [&str; 10] = [
        "rgb", "rgba", "hsl", "hsla", "hwb", "lab", "lch", "oklab", "oklch", "color",
    ];
    let mut input = ParserInput::new(raw);
    let mut parser = Parser::new(&mut input);
    while !parser.is_exhausted() {
        let token = match parser.next_including_whitespace() {
            Ok(token) => token.clone(),
            Err(_) => return true,
        };
        match token {
            Token::Ident(name) if name.eq_ignore_ascii_case("transparent") => return true,
            Token::Hash(value) | Token::IDHash(value) if zero_alpha_hex(&value) => return true,
            Token::Function(name) => {
                let name = name.to_ascii_lowercase();
                if name == "var" {
                    let Ok((variable, has_fallback)) = parse_var_name(&mut parser) else {
                        return true;
                    };
                    // Definitions are collected globally, without proving that
                    // their selector applies here. An opaque unrelated value
                    // therefore cannot rule out this element's fallback.
                    if has_fallback {
                        return true;
                    }
                    match variables.get(&variable) {
                        Some(Some(value)) if color_may_hide(value, variables, depth + 1) => {
                            return true;
                        }
                        Some(None) => return true,
                        _ => {}
                    }
                    continue;
                }
                if !COLOR_FUNCTIONS.contains(&name.as_str()) {
                    return true;
                }
                let comma_alpha = matches!(name.as_str(), "rgb" | "rgba" | "hsl" | "hsla");
                match parse_color_function_alpha(&mut parser, comma_alpha) {
                    Ok(CssAlpha::NoAlpha) => {}
                    Ok(CssAlpha::Value(value)) if value > 0.0 => {}
                    Ok(CssAlpha::Value(_)) | Ok(CssAlpha::Unknown) | Err(_) => return true,
                }
            }
            _ => {}
        }
    }
    false
}

fn css_unescape(value: &str) -> String {
    let mut chars = value.chars().peekable();
    let mut decoded = String::with_capacity(value.len());
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            decoded.push(ch);
            continue;
        }
        let Some(next) = chars.next() else {
            decoded.push('\\');
            break;
        };
        if matches!(next, '\n' | '\r' | '\u{c}') {
            if next == '\r' && chars.peek() == Some(&'\n') {
                chars.next();
            }
            continue;
        }
        if !next.is_ascii_hexdigit() {
            decoded.push(next);
            continue;
        }
        let mut hex = String::from(next);
        while hex.len() < 6 && chars.peek().is_some_and(|ch| ch.is_ascii_hexdigit()) {
            hex.push(chars.next().expect("peeked hex digit"));
        }
        if chars
            .peek()
            .is_some_and(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{c}'))
        {
            let whitespace = chars.next().expect("peeked CSS whitespace");
            if whitespace == '\r' && chars.peek() == Some(&'\n') {
                chars.next();
            }
        }
        let codepoint = u32::from_str_radix(&hex, 16).unwrap_or(0);
        decoded.push(char::from_u32(codepoint).unwrap_or('\u{fffd}'));
    }
    decoded
}

fn generated_content_is_semantic(tokens: &[Token<'_>]) -> bool {
    for token in significant_tokens(tokens) {
        match token {
            Token::QuotedString(value) => {
                if value.chars().any(char::is_alphanumeric) {
                    return true;
                }
            }
            Token::Function(name)
                if !name.eq_ignore_ascii_case("url")
                    || ["var", "attr", "counter", "counters"]
                        .iter()
                        .any(|function| name.eq_ignore_ascii_case(function)) =>
            {
                return true;
            }
            Token::Ident(name)
                if ![
                    "none",
                    "normal",
                    "open-quote",
                    "close-quote",
                    "no-open-quote",
                    "no-close-quote",
                ]
                .iter()
                .any(|keyword| name.eq_ignore_ascii_case(keyword)) =>
            {
                return true;
            }
            _ => {}
        }
    }
    false
}

fn assess_declaration(
    property: &str,
    tokens: &[Token<'_>],
    raw: &str,
    variables: &CssVariables,
) -> CssEffects {
    let property = property.to_ascii_lowercase();
    let ident = single_ident(tokens).map(str::to_ascii_lowercase);
    // Whitespace separates numeric tokens (scale(1 0) is not scale(10)).
    let compact = css_unescape(raw).to_ascii_lowercase();
    let has_variable = compact.contains("var(")
        || tokens.iter().any(
            |token| matches!(token, Token::Function(name) if name.eq_ignore_ascii_case("var")),
        );
    let hidden = match property.as_str() {
        "display" => has_variable || ident.as_deref() == Some("none"),
        "visibility" => has_variable || ident.as_deref() != Some("visible"),
        "content-visibility" => has_variable || ident.as_deref() != Some("visible"),
        "opacity" => has_variable || single_numeric_value(tokens).is_none_or(|value| value <= 0.0),
        "clip-path" => has_variable || ident.as_deref() != Some("none"),
        "clip" => has_variable || ident.as_deref() != Some("auto"),
        "transform" => has_variable || zero_area_transform(&compact),
        "filter" => has_variable || zero_filter_opacity(&compact),
        "color" | "-webkit-text-fill-color" => color_may_hide(raw, variables, 0),
        _ => false,
    };
    let generated_text = property == "content" && generated_content_is_semantic(tokens);
    let animated = matches!(property.as_str(), "animation" | "animation-name")
        && ident.as_deref() != Some("none");
    CssEffects {
        hidden,
        generated_text,
        animated,
    }
}

fn declaration_effects(css: &str, variables: &CssVariables) -> Result<CssEffects, ()> {
    let mut input = ParserInput::new(css);
    let mut parser = Parser::new(&mut input);
    let mut declaration_parser = CssEffectParser { variables };
    let mut effects = CssEffects::default();
    for declaration in RuleBodyParser::new(&mut parser, &mut declaration_parser) {
        let declaration = declaration.map_err(|_| ())?;
        effects.hidden |= declaration.hidden;
        effects.generated_text |= declaration.generated_text;
        effects.animated |= declaration.animated;
    }
    Ok(effects)
}

fn is_keyframe_step(selector: &str) -> bool {
    let selector = selector.trim();
    selector == "from"
        || selector == "to"
        || selector.split(',').all(|step| {
            let step = step.trim().trim_end_matches('%');
            !step.is_empty() && step.parse::<f64>().is_ok()
        })
}

fn css_number_is_zero(value: &str) -> bool {
    value
        .trim()
        .trim_end_matches('%')
        .parse::<f64>()
        .is_ok_and(|number| number == 0.0)
}

fn zero_area_transform(value: &str) -> bool {
    value.split(')').any(|function| {
        if let Some((_, args)) = function.rsplit_once("scale(") {
            let axes: Vec<_> = args
                .split(|ch: char| ch == ',' || ch.is_whitespace())
                .filter(|axis| !axis.is_empty())
                .collect();
            return axes.iter().take(2).any(|axis| css_number_is_zero(axis));
        }
        if let Some((_, args)) = function.rsplit_once("scalex(") {
            return css_number_is_zero(args);
        }
        if let Some((_, args)) = function.rsplit_once("scaley(") {
            return css_number_is_zero(args);
        }
        if let Some((_, args)) = function.rsplit_once("matrix(") {
            let values: Vec<_> = args.split(',').collect();
            if values.len() == 6 {
                let parsed: Option<Vec<_>> = values
                    .iter()
                    .take(4)
                    .map(|value| value.trim().parse::<f64>().ok())
                    .collect();
                if let Some(v) = parsed {
                    return v[0] * v[3] - v[1] * v[2] == 0.0;
                }
            }
        }
        false
    })
}

fn zero_filter_opacity(value: &str) -> bool {
    value.split(')').any(|function| {
        function
            .rsplit_once("opacity(")
            .is_some_and(|(_, args)| css_number_is_zero(args))
    })
}

fn embedded_hidden_selectors(doc: &Html) -> Result<CssRules, IngestError> {
    let raw = doc
        .select(&selector("style"))
        .map(|style| style.text().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    css_visibility_selectors(&raw)
}

fn strip_css_comments(raw: &str) -> Result<String, IngestError> {
    fn copy_escape(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, output: &mut String) {
        let Some(next) = chars.next() else {
            return;
        };
        output.push(next);
        if !next.is_ascii_hexdigit() {
            return;
        }
        let mut digits = 1;
        while digits < 6 && chars.peek().is_some_and(|ch| ch.is_ascii_hexdigit()) {
            output.push(chars.next().expect("peeked CSS hex digit"));
            digits += 1;
        }
        if chars
            .peek()
            .is_some_and(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{c}'))
        {
            let whitespace = chars.next().expect("peeked CSS whitespace");
            output.push(whitespace);
            if whitespace == '\r' && chars.peek() == Some(&'\n') {
                output.push(chars.next().expect("peeked LF after CR"));
            }
        }
    }

    let mut chars = raw.chars().peekable();
    let mut output = String::with_capacity(raw.len());
    let mut quote = None;
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            output.push(ch);
            copy_escape(&mut chars, &mut output);
            continue;
        }
        if let Some(delimiter) = quote {
            output.push(ch);
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            output.push(ch);
            continue;
        }
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            let mut previous_star = false;
            let mut closed = false;
            for comment_ch in chars.by_ref() {
                if previous_star && comment_ch == '/' {
                    closed = true;
                    break;
                }
                previous_star = comment_ch == '*';
            }
            if !closed {
                return Err(error("Unvollständiger CSS-Kommentar"));
            }
            output.push(' ');
            continue;
        }
        output.push(ch);
    }
    Ok(output)
}

fn next_css_boundary(css: &str, start: usize) -> Option<(usize, char)> {
    let mut quote = None;
    let mut escaped = false;
    let mut parentheses = 0usize;
    let mut brackets = 0usize;
    for (relative, ch) in css[start..].char_indices() {
        let index = start + relative;
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '(' => parentheses = parentheses.saturating_add(1),
            ')' => parentheses = parentheses.saturating_sub(1),
            '[' => brackets = brackets.saturating_add(1),
            ']' => brackets = brackets.saturating_sub(1),
            ';' | '{' if parentheses == 0 && brackets == 0 => return Some((index, ch)),
            _ => {}
        }
    }
    None
}

fn css_block_end(css: &str, open: usize) -> Option<usize> {
    let mut depth = 1usize;
    let mut quote = None;
    let mut escaped = false;
    let mut parentheses = 0usize;
    let mut brackets = 0usize;
    for (relative, ch) in css[open + 1..].char_indices() {
        let index = open + 1 + relative;
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '(' => parentheses = parentheses.saturating_add(1),
            ')' => parentheses = parentheses.saturating_sub(1),
            '[' => brackets = brackets.saturating_add(1),
            ']' => brackets = brackets.saturating_sub(1),
            '{' if parentheses == 0 && brackets == 0 => depth += 1,
            '}' if parentheses == 0 && brackets == 0 => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn replace_nesting_selector(
    parent: &str,
    child: &str,
    budget: usize,
) -> Result<String, IngestError> {
    // Preflight before allocating or inserting any parent text. Multiple &
    // require :is(parent-list) cross combinations, which we do not implement.
    let mut count = 0usize;
    let mut quote = None;
    let mut escaped = false;
    let mut brackets = 0usize;
    for ch in child.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '[' => brackets += 1,
            ']' => brackets = brackets.saturating_sub(1),
            '&' if brackets == 0 => {
                count += 1;
                if count > 1 {
                    return Err(error("Mehrfache CSS-Nesting-Referenz nicht unterstützt"));
                }
            }
            _ => {}
        }
    }
    let size = if count == 1 {
        parent.len().checked_add(child.len() - 1)
    } else {
        parent
            .len()
            .checked_add(child.trim().len())
            .and_then(|size| size.checked_add(1))
    }
    .filter(|size| *size <= budget)
    .ok_or_else(|| error("CSS-Selektoren nach Nesting-Auflösung zu groß"))?;
    let mut output = String::with_capacity(size);
    if count == 0 {
        output.push_str(parent);
        output.push(' ');
        output.push_str(child.trim());
        return Ok(output);
    }
    let mut quote = None;
    let mut escaped = false;
    let mut brackets = 0usize;
    let mut replaced = false;
    for ch in child.chars() {
        if escaped {
            output.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            output.push(ch);
            escaped = true;
            continue;
        }
        if let Some(delimiter) = quote {
            output.push(ch);
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            output.push(ch);
        } else if ch == '[' {
            brackets += 1;
            output.push(ch);
        } else if ch == ']' {
            brackets = brackets.saturating_sub(1);
            output.push(ch);
        } else if ch == '&' && brackets == 0 {
            output.push_str(parent);
            replaced = true;
        } else {
            output.push(ch);
        }
    }
    if replaced {
        Ok(output)
    } else {
        output.clear();
        output.push_str(parent);
        output.push(' ');
        output.push_str(child.trim());
        Ok(output)
    }
}

fn emit_css_declarations(
    selectors: &[String],
    declarations: &str,
    output: &mut String,
) -> Result<(), IngestError> {
    const MAX_FLATTENED_BYTES: usize = 8_000_000;
    if selectors.is_empty() || declarations.trim().is_empty() {
        return Ok(());
    }
    let selector_bytes = selectors.iter().try_fold(0usize, |size, selector| {
        size.checked_add(selector.len())
            .and_then(|size| size.checked_add(2))
    });
    let emitted_bytes = selector_bytes
        .and_then(|size| size.checked_add(declarations.len()))
        .and_then(|size| size.checked_add(3))
        .ok_or_else(|| error("CSS-Regeln nach Nesting-Auflösung zu groß"))?;
    if output.len().saturating_add(emitted_bytes) > MAX_FLATTENED_BYTES {
        return Err(error("CSS-Regeln nach Nesting-Auflösung zu groß"));
    }
    for (index, selector) in selectors.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        output.push_str(selector);
    }
    output.push('{');
    output.push_str(declarations);
    if !declarations.trim_end().ends_with(';') {
        output.push(';');
    }
    output.push('}');
    Ok(())
}

fn flatten_css_rule_list(
    css: &str,
    inherited_selectors: &[String],
    output: &mut String,
    depth: usize,
) -> Result<(), IngestError> {
    const MAX_DEPTH: usize = 64;
    const MAX_FLATTENED_BYTES: usize = 8_000_000;
    if depth > MAX_DEPTH {
        return Err(error("CSS-Nesting zu tief"));
    }

    let mut cursor = 0;
    let mut statement_start = 0;
    let mut declarations = String::new();
    while cursor < css.len() {
        match next_css_boundary(css, cursor) {
            Some((index, ';')) => {
                if !inherited_selectors.is_empty() {
                    declarations.push_str(&css[statement_start..=index]);
                }
                cursor = index + 1;
                statement_start = cursor;
            }
            Some((index, '{')) => {
                emit_css_declarations(inherited_selectors, &declarations, output)?;
                declarations.clear();
                let prelude = css[statement_start..index].trim();
                let close =
                    css_block_end(css, index).ok_or_else(|| error("Unvollständiger CSS-Block"))?;
                if !prelude.is_empty() {
                    flatten_css_block(
                        prelude,
                        &css[index + 1..close],
                        inherited_selectors,
                        output,
                        depth + 1,
                    )?;
                }
                cursor = close + 1;
                statement_start = cursor;
            }
            Some((_, '}')) => break,
            Some((_, _)) => unreachable!("CSS scanner only reports statement boundaries"),
            None => {
                if !inherited_selectors.is_empty() {
                    declarations.push_str(&css[statement_start..]);
                }
                cursor = css.len();
            }
        }
        if output.len() > MAX_FLATTENED_BYTES {
            return Err(error("CSS-Regeln nach Nesting-Auflösung zu groß"));
        }
    }
    emit_css_declarations(inherited_selectors, &declarations, output)?;
    Ok(())
}

fn flatten_css_block(
    prelude: &str,
    body: &str,
    inherited_selectors: &[String],
    output: &mut String,
    depth: usize,
) -> Result<(), IngestError> {
    if prelude.starts_with('@') {
        let at_rule = prelude.to_ascii_lowercase();
        if [
            "@keyframes",
            "@-webkit-keyframes",
            "@font-face",
            "@font-palette-values",
            "@property",
            "@counter-style",
            "@page",
        ]
        .iter()
        .any(|rule| at_rule.starts_with(rule))
        {
            return Ok(());
        }
        return flatten_css_rule_list(body, inherited_selectors, output, depth);
    }

    let child_selectors =
        split_selector_list(prelude).map_err(|_| error("Unvollständige CSS-Selektorliste"))?;
    let mut selectors = Vec::new();
    if inherited_selectors.is_empty() {
        selectors.extend(child_selectors.into_iter().map(str::to_owned));
    } else {
        const MAX_SELECTOR_EXPANSION: usize = 512;
        const MAX_SELECTOR_BYTES: usize = 8_000_000;
        if inherited_selectors
            .len()
            .saturating_mul(child_selectors.len())
            > MAX_SELECTOR_EXPANSION
        {
            return Err(error("CSS-Selektorliste nach Nesting-Auflösung zu groß"));
        }
        let mut expanded_bytes = 0usize;
        for parent in inherited_selectors {
            for child in &child_selectors {
                let remaining = MAX_SELECTOR_BYTES
                    .checked_sub(output.len())
                    .and_then(|size| size.checked_sub(expanded_bytes))
                    .and_then(|size| size.checked_sub(2))
                    .ok_or_else(|| error("CSS-Selektoren nach Nesting-Auflösung zu groß"))?;
                let selector = replace_nesting_selector(parent, child, remaining)?;
                expanded_bytes = expanded_bytes
                    .saturating_add(selector.len())
                    .saturating_add(2);
                if expanded_bytes > MAX_SELECTOR_BYTES {
                    return Err(error("CSS-Selektoren nach Nesting-Auflösung zu groß"));
                }
                selectors.push(selector);
            }
        }
    }
    flatten_css_rule_list(body, &selectors, output, depth)
}

fn flatten_nested_css(css: &str) -> Result<String, IngestError> {
    let mut flattened = String::new();
    flatten_css_rule_list(css, &[], &mut flattened, 0)?;
    Ok(flattened)
}

fn css_block_segments(css: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut start = 0;
    let mut quote = None;
    let mut escaped = false;
    let mut parentheses = 0usize;
    let mut brackets = 0usize;
    for (index, ch) in css.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '(' => parentheses = parentheses.saturating_add(1),
            ')' => parentheses = parentheses.saturating_sub(1),
            '[' => brackets = brackets.saturating_add(1),
            ']' => brackets = brackets.saturating_sub(1),
            '}' if parentheses == 0 && brackets == 0 => {
                segments.push(&css[start..index]);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    segments.push(&css[start..]);
    segments
}

fn css_variable_definitions(css: &str) -> CssVariables {
    let mut variables: CssVariables = HashMap::new();
    for block in css_block_segments(css) {
        let Some((_, declarations)) = block.rsplit_once('{') else {
            continue;
        };
        let mut start = 0;
        loop {
            let boundary = next_css_boundary(declarations, start);
            let end = boundary.map_or(declarations.len(), |(index, _)| index);
            if let Some((property, value)) = declarations[start..end].split_once(':') {
                // Match cssparser's decoded var() identifiers, including CSS
                // hexadecimal escapes and their terminating whitespace.
                let property = css_unescape(property.trim());
                if property.starts_with("--") {
                    let value = value.trim().to_owned();
                    if let Some(existing) = variables.get_mut(&property) {
                        if existing.as_deref() != Some(value.as_str()) {
                            *existing = None;
                        }
                    } else {
                        variables.insert(property.to_owned(), Some(value));
                    }
                }
            }
            match boundary {
                Some((index, ';')) => start = index + 1,
                Some((_, '{')) | Some((_, '}')) => break,
                Some((_, _)) => unreachable!("CSS scanner only reports statement boundaries"),
                None => break,
            }
        }
    }
    variables
}

fn css_escape_end(input: &str, slash: usize) -> usize {
    let bytes = input.as_bytes();
    let mut end = slash + 1;
    if end >= bytes.len() {
        return end;
    }
    if matches!(bytes[end], b'\n' | b'\r' | 0x0c) {
        end += 1;
        if bytes[end - 1] == b'\r' && bytes.get(end) == Some(&b'\n') {
            end += 1;
        }
        return end;
    }
    if bytes[end].is_ascii_hexdigit() {
        let mut digits = 0;
        while end < bytes.len() && digits < 6 && bytes[end].is_ascii_hexdigit() {
            end += 1;
            digits += 1;
        }
        if bytes
            .get(end)
            .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0c))
        {
            let is_cr = bytes[end] == b'\r';
            end += 1;
            if is_cr && bytes.get(end) == Some(&b'\n') {
                end += 1;
            }
        }
        return end;
    }
    end += input[end..]
        .chars()
        .next()
        .expect("valid UTF-8 boundary")
        .len_utf8();
    end
}

fn has_state_pseudo(selector: &str) -> bool {
    let bytes = selector.as_bytes();
    let mut index = 0;
    let mut quote = None;
    let mut brackets = 0usize;
    while index < bytes.len() {
        let ch = selector[index..]
            .chars()
            .next()
            .expect("valid UTF-8 boundary");
        if ch == '\\' {
            index = css_escape_end(selector, index);
            continue;
        }
        if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
            index += ch.len_utf8();
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            index += ch.len_utf8();
            continue;
        }
        if ch == '[' {
            brackets += 1;
            index += 1;
            continue;
        }
        if ch == ']' {
            brackets = brackets.saturating_sub(1);
            index += 1;
            continue;
        }
        if brackets == 0 && ch == ':' && bytes.get(index + 1) != Some(&b':') {
            let mut end = index + 1;
            while bytes
                .get(end)
                .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
            {
                end += 1;
            }
            let name = &selector[index + 1..end];
            if STATE_PSEUDOS
                .iter()
                .any(|pseudo| name.eq_ignore_ascii_case(pseudo))
            {
                return true;
            }
            index = end.max(index + 1);
            continue;
        }
        index += ch.len_utf8();
    }
    false
}

fn matching_parenthesis(input: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    for (relative, ch) in input[open..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '(' => depth += 1,
            ')' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + relative);
                }
            }
            _ => {}
        }
    }
    None
}

fn broaden_state_pseudos(selector: &str) -> String {
    let bytes = selector.as_bytes();
    let mut output = String::with_capacity(selector.len());
    let mut index = 0;
    let mut quote = None;
    let mut brackets = 0usize;
    while index < bytes.len() {
        let ch = selector[index..]
            .chars()
            .next()
            .expect("valid UTF-8 boundary");
        if ch == '\\' {
            let end = css_escape_end(selector, index);
            output.push_str(&selector[index..end]);
            index = end;
            continue;
        }
        if let Some(delimiter) = quote {
            output.push(ch);
            if ch == delimiter {
                quote = None;
            }
            index += ch.len_utf8();
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            output.push(ch);
            index += ch.len_utf8();
            continue;
        }
        if ch == '[' {
            brackets += 1;
        } else if ch == ']' {
            brackets = brackets.saturating_sub(1);
        }
        if brackets == 0 && bytes[index] == b':' && bytes.get(index + 1) != Some(&b':') {
            let mut name_end = index + 1;
            while bytes
                .get(name_end)
                .is_some_and(|ch| ch.is_ascii_alphanumeric() || *ch == b'-')
            {
                name_end += 1;
            }
            if name_end > index + 1 {
                let name = &selector[index + 1..name_end];
                if STATE_PSEUDOS
                    .iter()
                    .any(|pseudo| name.eq_ignore_ascii_case(pseudo))
                {
                    index = name_end;
                    continue;
                }
                if bytes.get(name_end) == Some(&b'(') {
                    if let Some(close) = matching_parenthesis(selector, name_end) {
                        if has_state_pseudo(&selector[name_end + 1..close]) {
                            index = close + 1;
                            continue;
                        }
                    }
                }
            }
        }
        output.push(ch);
        index += ch.len_utf8();
    }
    output
}

fn css_visibility_selectors(raw: &str) -> Result<CssRules, IngestError> {
    let css = strip_css_comments(raw)?;
    if css.to_ascii_lowercase().contains("@import") {
        return Err(error("Nicht ausgewerteter CSS-@import"));
    }
    let css = flatten_nested_css(&css)?;
    let variables = css_variable_definitions(&css);

    let mut rules = CssRules {
        variables,
        ..CssRules::default()
    };
    for block in css_block_segments(&css) {
        let Some((prefix, declarations)) = block.rsplit_once('{') else {
            continue;
        };
        let effects = declaration_effects(declarations, &rules.variables)
            .map_err(|_| error("Nicht auswertbare CSS-Deklaration"))?;
        let hides = effects.hidden;
        let generates_text = effects.generated_text;
        let animates = effects.animated;
        if !hides && !generates_text && !animates {
            continue;
        }
        // Nested @media/@supports are evaluated conservatively across conditions.
        let selectors = prefix.rsplit('{').next().unwrap_or(prefix).trim();
        let selector_list = split_selector_list(selectors)
            .map_err(|_| error("Unvollständige CSS-Selektorliste"))?;
        for raw_selector in selector_list {
            if is_keyframe_step(raw_selector) {
                continue;
            }
            let selector = raw_selector
                .trim()
                .split("::")
                .next()
                .unwrap_or(raw_selector)
                .trim()
                .to_owned();
            // State-dependent rules are broadened to their base selector, so
            // all possible states are checked without a browser interaction.
            let selector = broaden_state_pseudos(&selector);
            let selector = [
                "::before",
                "::after",
                "::first-line",
                "::first-letter",
                "::marker",
                ":before",
                ":after",
                ":first-line",
                ":first-letter",
            ]
            .iter()
            .filter_map(|pseudo| selector.find(pseudo))
            .min()
            .map_or(selector.as_str(), |end| &selector[..end])
            .trim();
            let parsed = Selector::parse(selector).map_err(|_| {
                error(format!(
                    "Nicht auswertbarer CSS-Visibility-Selektor: {selector}"
                ))
            })?;
            if hides {
                rules.hidden.push(parsed.clone());
            }
            if generates_text {
                rules.generated_text.push(parsed.clone());
            }
            if animates {
                rules.animated.push(parsed);
            }
        }
    }
    Ok(rules)
}

fn ensure_visible(el: ElementRef<'_>, rules: &CssRules) -> Result<(), IngestError> {
    for node in el
        .ancestors()
        .filter_map(ElementRef::wrap)
        .chain(std::iter::once(el))
        .chain(el.select(&selector("*")))
    {
        let value = node.value();
        let style = value.attr("style").unwrap_or("");
        let style_effects = declaration_effects(style, &rules.variables).unwrap_or(CssEffects {
            hidden: true,
            ..CssEffects::default()
        });
        let matching_rule = rules
            .hidden
            .iter()
            .chain(&rules.generated_text)
            .chain(&rules.animated)
            .find(|rule| rule.matches(&node));
        if value.attr("hidden").is_some()
            || value.attr("aria-hidden") == Some("true")
            || style_effects.hidden
            || style_effects.animated
            || value.classes().any(|class| {
                matches!(
                    class,
                    "hidden" | "d-none" | "invisible" | "visually-hidden" | "sr-only"
                )
            })
            || matching_rule.is_some()
        {
            return Err(error("Versteckte Preisposition oder Kondition"));
        }
    }
    Ok(())
}

/// Source-window attributes are a contract too: arbitrary custom attributes
/// can encode quote conditions without changing the text we consume. Keep a
/// narrow allowlist for structure/classes and known decorative media geometry.
fn reviewed_source_attribute(el: ElementRef<'_>, section: ElementRef<'_>, key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    let key = key.as_str();
    if key == "class" {
        return true;
    }
    match el.value().name() {
        "section" => {
            el.id() == section.id() && key == "id" && el.value().attr("id") == Some("sj-preise")
        }
        "img" => matches!(key, "src" | "alt" | "width" | "height" | "loading"),
        "svg" => matches!(
            key,
            "xmlns" | "viewbox" | "width" | "height" | "fill" | "stroke" | "stroke-width"
        ),
        "circle" => matches!(key, "cx" | "cy" | "r" | "fill" | "stroke" | "stroke-width"),
        "line" => matches!(key, "x1" | "y1" | "x2" | "y2" | "stroke" | "stroke-width"),
        _ => false,
    }
}

fn split_selector_list(selectors: &str) -> Result<Vec<&str>, ()> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut parentheses = 0usize;
    let mut brackets = 0usize;
    let mut quote = None;
    let mut escaped = false;

    for (index, ch) in selectors.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(delimiter) = quote {
            if ch == delimiter {
                quote = None;
            }
            continue;
        }
        match ch {
            '\'' | '"' => quote = Some(ch),
            '(' => parentheses += 1,
            ')' => parentheses = parentheses.checked_sub(1).ok_or(())?,
            '[' => brackets += 1,
            ']' => brackets = brackets.checked_sub(1).ok_or(())?,
            ',' if parentheses == 0 && brackets == 0 => {
                let part = selectors[start..index].trim();
                if part.is_empty() {
                    return Err(());
                }
                parts.push(part);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    if escaped || quote.is_some() || parentheses != 0 || brackets != 0 {
        return Err(());
    }
    let part = selectors[start..].trim();
    if part.is_empty() {
        return Err(());
    }
    parts.push(part);
    Ok(parts)
}

/// The two owned source sections are a closed quote/condition window. Decorative
/// images/SVG have no quote text; every other non-whitespace text node must belong
/// to an explicitly consumed anchor/field, not merely sit beside a known card.
fn consume_source_window(
    section: ElementRef<'_>,
    fields: &[ElementRef<'_>],
) -> Result<(), IngestError> {
    for node in section.descendants() {
        if let Some(t) = node.value().as_text() {
            if !t.trim().is_empty()
                && !node
                    .ancestors()
                    .any(|ancestor| fields.iter().any(|field| field.id() == ancestor.id()))
            {
                return Err(error(format!(
                    "Unverbrauchtes Quellfeld im Preisabschnitt: {}",
                    t.trim()
                )));
            }
        }
        if let Some(el) = ElementRef::wrap(node) {
            if !(el.id() == section.id()
                || matches!(
                    el.value().name(),
                    "div" | "span" | "h2" | "p" | "img" | "strong" | "svg" | "circle" | "line"
                ))
            {
                return Err(error(format!(
                    "Unverbrauchtes Element im Preisabschnitt: {}",
                    el.value().name()
                )));
            }
            if let Some((key, _)) = el
                .value()
                .attrs()
                .find(|(key, _)| !reviewed_source_attribute(el, section, key))
            {
                return Err(error(format!(
                    "Unverbrauchtes Attribut im Preisabschnitt: {}@{key}",
                    el.value().name()
                )));
            }
        }
    }
    Ok(())
}

fn excluded_from_page_context(el: ElementRef<'_>, price_sections: &[ElementRef<'_>]) -> bool {
    matches!(el.value().name(), "script" | "style")
        || price_sections.iter().any(|section| section.id() == el.id())
}

fn normalize_context(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn technical_context_element(el: ElementRef<'_>) -> bool {
    matches!(
        el.value().name(),
        "script" | "style" | "template" | "noscript"
    )
}

fn validate_page_context(doc: &Html, css_rules: &CssRules) -> Result<(), IngestError> {
    let main = one(doc.root_element(), "main#brx-content")?;
    let price_sections = [
        one(main, "section.sj-preise-section-fe")?,
        one(main, "section.sj-preise-section-ne")?,
    ];
    let mut text_nodes = Vec::new();
    for node in main.descendants() {
        if let Some(value) = node.value().as_text() {
            let excluded = node
                .ancestors()
                .filter_map(ElementRef::wrap)
                .any(|ancestor| excluded_from_page_context(ancestor, &price_sections));
            if !excluded {
                text_nodes.push(value.as_ref());
            }
        }
    }
    let page_text = normalize_context(&text_nodes.join(" "));
    let expected_text = normalize_context(PAGE_CONTEXT);
    if page_text != expected_text {
        return Err(error("Ungeprüfter sichtbarer Gewerbe-Seitenkontext"));
    }

    let mut attributes = Vec::new();
    for el in main.select(&selector("*")) {
        let excluded = el
            .ancestors()
            .filter_map(ElementRef::wrap)
            .chain(std::iter::once(el))
            .any(|node| excluded_from_page_context(node, &price_sections));
        if excluded {
            continue;
        }
        for key in ["alt", "title", "aria-label", "placeholder"] {
            if let Some(value) = el.value().attr(key) {
                attributes.push(format!(
                    "{}@{key}={}",
                    el.value().name(),
                    normalize_context(value)
                ));
            }
        }
    }
    let expected_attributes: Vec<_> = PAGE_CONTEXT_ATTRIBUTES
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    if attributes
        .iter()
        .map(String::as_str)
        .ne(expected_attributes.iter().copied())
    {
        return Err(error("Ungeprüfte Textattribute im Gewerbe-Seitenkontext"));
    }

    let all_elements = selector("*");
    let page_nodes = main
        .ancestors()
        .filter_map(ElementRef::wrap)
        .chain(std::iter::once(main))
        .chain(main.select(&all_elements));
    for node in page_nodes {
        if css_rules
            .generated_text
            .iter()
            .any(|rule| rule.matches(&node))
        {
            return Err(error("Ungeprüfter CSS-generierter Seitenkontext"));
        }
    }
    Ok(())
}

fn check_operator(html: &str) -> Result<(), IngestError> {
    let doc = Html::parse_document(html);
    let main = one(doc.root_element(), "main#brx-content")?;
    let legal = one(doc.root_element(), "section.sj-contact")?;
    if !legal.ancestors().any(|node| node.id() == main.id()) {
        return Err(error("Betreiberblock außerhalb des Impressum-Hauptinhalts"));
    }
    if legal
        .ancestors()
        .filter_map(ElementRef::wrap)
        .any(technical_context_element)
    {
        return Err(error(
            "Betreiberblock innerhalb nicht sichtbarer technischer Daten",
        ));
    }
    let rules = embedded_hidden_selectors(&doc)?;
    ensure_visible(legal, &rules)?;
    // DOM text pins do not include pseudo-element text. Check the entire
    // Impressum main and its ancestors, not only the legal section, while
    // excluding technical subtrees that cannot establish visible identity.
    for node in main
        .ancestors()
        .filter_map(ElementRef::wrap)
        .chain(std::iter::once(main))
        .chain(main.select(&selector("*")))
    {
        let technical = node
            .ancestors()
            .filter_map(ElementRef::wrap)
            .chain(std::iter::once(node))
            .any(technical_context_element);
        if !technical && rules.generated_text.iter().any(|rule| rule.matches(&node)) {
            return Err(error("Ungeprüfter CSS-generierter Impressum-Kontext"));
        }
    }
    if legal
        .select(&selector("script, style, template, noscript"))
        .next()
        .is_some()
    {
        return Err(error("Nicht sichtbarer Inhalt im Betreiberblock"));
    }
    let expected = Html::parse_document(include_str!(
        "fixtures/schrottjungs-impressum-operator-20261013.html"
    ));
    let expected = one(expected.root_element(), "section.sj-contact")?;
    // Pin the complete qualified current legal block, not a bag of names that
    // could equally describe a historical, negated or different operator.
    if text(legal) != text(expected) {
        return Err(error("Ungeprüfter aktueller Impressum-Betreiberblock"));
    }
    // The only reviewed sibling is the full privacy notice from the fresh
    // 2026-10-13 source. Pin its normalized text too: a new legal identity must
    // not coexist with the old, otherwise still perfectly matching block.
    let outside = main
        .descendants()
        .filter_map(|node| {
            if node
                .ancestors()
                .filter_map(ElementRef::wrap)
                .any(|ancestor| ancestor.id() == legal.id() || technical_context_element(ancestor))
            {
                return None;
            }
            node.value().as_text().map(|value| value.to_string())
        })
        .collect::<Vec<_>>()
        .join(" ");
    let outside = normalize_context(&outside);
    if !outside.is_empty() {
        use sha2::{Digest, Sha256};
        let digest = format!("{:x}", Sha256::digest(outside.as_bytes()));
        if digest != "29a47948be56f0588d50057b3661a8b90349a8be26f17f31c0da288c9a959364" {
            return Err(error(
                "Ungeprüfter Impressum-Kontext außerhalb des Betreiberblocks",
            ));
        }
    }
    Ok(())
}

// No guessed conductor metal, tin purity, prepared steel or tungsten carbide.
fn grade(label: &str) -> Result<&'static str, &'static str> {
    match label {
        "Mischschrott leicht" | "Mischschrott schwer" => Ok("mischschrott"),
        "Trägerschrott" => Ok("stahlschrott-traeger"),
        "Bremsscheiben" => Ok("eisenschrott-gussbruch"),
        "Kupfer Raff" => Ok("kupfer-gemischt"),
        "Kupfer Millberry" => Ok("kupfer-millberry"),
        "Messing gemischt" => Ok("messing"),
        "Rotguss" => Ok("bronze-rotguss"),
        "Aluminium (div.)" => Ok("aluminium-gemischt"),
        "Edelstahl V2A" => Ok("edelstahl-v2a"),
        "Zink" => Ok("zink"),
        "Scherenvormaterial" => Err("Vormaterial ist nicht belegter aufbereiteter Scherenschrott"),
        "Sorte 3" => Err("keine exakte Sorte-3-Katalogposition"),
        "Zerlegematerial" => Err("Materialzusammensetzung unbekannt"),
        "Kupferschiene" => Err("keine exakte Kupferschienen-Katalogposition; Qualität unbekannt"),
        "Kabel (diverse)" => Err("Leitermetall, Stecker und Gehalt unbekannt"),
        "Blei" => Err("Weichblei-/Auswucht-/Formzuordnung unbekannt"),
        "Zinn" => Err("keine belegte Reinzinnqualität oder Warenform"),
        "Hartmetall / Wolfram" => {
            Err("Hartmetall und metallisches Wolfram sind getrennte Katalogmaterialien")
        }
        _ => Err("unbekannte Originalposition; keine Kataloginferenz"),
    }
}

fn ceiling(raw: &str, suffix: &str) -> Result<f64, IngestError> {
    let number = raw
        .strip_prefix("bis zu ")
        .and_then(|s| s.strip_suffix(suffix))
        .ok_or_else(|| {
            error(format!(
                "Keine ausdrückliche Obergrenze/Originaleinheit: {raw}"
            ))
        })?;
    let parts: Vec<_> = number.split(',').collect();
    if !(parts.len() == 1 || parts.len() == 2 && parts[1].len() == 2)
        || parts
            .iter()
            .any(|p| p.is_empty() || !p.chars().all(|c| c.is_ascii_digit()))
    {
        return Err(error(format!("Ungültiger Preis: {raw}")));
    }
    parse_eur(number)
        .filter(|p| p.is_finite() && *p > 0.0)
        .ok_or_else(|| error(format!("Ungültiger Preis: {raw}")))
}

#[cfg(test)]
fn parse(html: &str) -> Result<HandlerOutcome, IngestError> {
    parse_document(html, false)
}

fn parse_source(html: &str) -> Result<HandlerOutcome, IngestError> {
    parse_document(html, true)
}

fn parse_document(html: &str, require_page_context: bool) -> Result<HandlerOutcome, IngestError> {
    let doc = Html::parse_document(html);
    let css_rules = embedded_hidden_selectors(&doc)?;
    if require_page_context {
        validate_page_context(&doc, &css_rules)?;
    }
    let mut out = HandlerOutcome {
        skip_price_acceptances: true,
        fetch_url: URL.to_owned(),
        status_code: 200,
        byte_len: html.len(),
        website_alive: true,
        ..Default::default()
    };
    let mut labels = HashSet::new();
    for (section, heading, count, unit, suffix, sub) in [
        ("section.sj-preise-section-fe", "FE-Schrott / Eisenschrott", 7, "EUR/t", " €/Tonne", "Metallpreise für Buntmetalle - Richtpreise pro Tonne · Gültig ab Großmengen · Tagespreise auf Anfrage"),
        ("section.sj-preise-section-ne", "Buntmetalle / NE-Metalle", 12, "EUR/kg", " €/kg", "Metallpreise für Buntmetalle - Richtpreise pro Kilogramm · Mengenabhängig · Detailpreise auf Anfrage"),
    ] {
        let section = one(doc.root_element(), section)?;
        let header = one(section, ".sj-preise-header h2")?;
        let subtitle = one(section, ".sj-preise-sub")?;
        let mut fields = vec![header, subtitle];
        if heading.starts_with("FE-") { fields.push(one(section, ".sj-overline")?); }
        let icons: Vec<_> = section.select(&selector(".sj-preise-icon")).collect();
        if icons.len() > 1 || icons.iter().any(|icon| text(*icon) != if unit == "EUR/t" { "Fe" } else { "NE" }) { return Err(error("Dekorativer Abschnittsanker geändert")); }
        fields.extend(icons);
        if unit == "EUR/kg" { fields.push(one(section, ".sj-preis-banner p")?); }
        one(section, ".sj-preis-grid")?;
        if text(one(section, ".sj-preise-header h2")?) != heading || text(one(section, ".sj-preise-sub")?) != sub {
            return Err(error("Preisrichtung oder Abschnittskonditionen geändert"));
        }
        if heading.starts_with("FE-") && text(one(section, ".sj-overline")?) != "Ankaufspreise" { return Err(error("Ankaufanker geändert")); }
        let cards: Vec<_> = section.select(&selector(".sj-preis-grid .sj-preis-item")).collect();
        if section.select(&selector(".sj-preis-item")).count() != cards.len() { return Err(error("Zusätzliche eigene Preisposition außerhalb des Grids")); }
        if cards.len() != count { return Err(error(format!("Unvollständige/geänderte {heading}-Liste: {} statt {count}", cards.len()))); }
        let mut consumed_images = 0;
        for card in cards {
            // Hidden CMS cards/sections cannot silently become active quotes.
            ensure_visible(card, &css_rules)?;
            let name = one(card, ".sj-preis-name")?;
            let tag = one(card, ".sj-preis-tag")?;
            fields.extend([name, tag]);
            if name.value().attr("hidden").is_some() || tag.value().attr("hidden").is_some() { return Err(error("Versteckte Preisfelder")); }
            let label = text(name);
            if label.is_empty() || !labels.insert(label.clone()) { return Err(error("Leere/doppelte Originalposition")); }
            let images: Vec<_> = card.select(&selector("img")).collect();
            consumed_images += images.len();
            let expected_alt = match label.as_str() {
                "Kabel (diverse)" => "Kabel diverse Sorten",
                "Aluminium (div.)" => "Aluminium",
                "Hartmetall / Wolfram" => "Hartmetall Wolfram",
                _ => label.as_str(),
            };
            if images.len() > 1 || images.iter().any(|img| img.value().attr("alt") != Some(expected_alt)) {
                return Err(error("Nicht konsumierte/abweichende Bildbeschreibung im Preisfeld"));
            }
            let raw = text(tag);
            let price = ceiling(&raw, suffix)?;
            if let Ok(material) = grade(&label) {
                let expected_unit = if matches!(material, "mischschrott" | "stahlschrott-traeger" | "eisenschrott-gussbruch") { "EUR/t" } else { "EUR/kg" };
                if unit != expected_unit { return Err(error(format!("Originalsorte in falschem Einheitenabschnitt: {label}"))); }
            }
            match grade(&label) {
                Ok(material) => out.prices.push(ScrapedPrice {
                    material, variant: format!("{label} — {CONDITIONS}").into(), price, currency: "EUR", unit,
                    price_kind: "upto", price_min: None, price_max: Some(price), confidence: Some(0.5),
                    label: format!("{label}: {raw}; {CONDITIONS}"),
                }),
                Err(reason) => out.skipped_labels.push(format!("{label}: {raw} (nicht gemappt: {reason}); {CONDITIONS}")),
            }
        }
        if section.select(&selector("img")).count() != consumed_images { return Err(error("Nicht konsumierte Bildbeschreibung außerhalb der Preiskarten")); }
        for field in &fields { ensure_visible(*field, &css_rules)?; }
        consume_source_window(section, &fields)?;
    }
    let banner_element = one(
        doc.root_element(),
        ".sj-preise-section-ne .sj-preis-banner p",
    )?;
    ensure_visible(banner_element, &css_rules)?;
    let banner = text(banner_element);
    if banner != "Alle genannten Preise sind unverbindliche Richtpreise für Metallankauf in Großmengen. Tagesaktuelle Metallpreise erhalten Sie auf Anfrage. Die tatsächlichen Metallankaufspreise können je nach Menge, Qualität, Sortierung und aktueller Marktlage variieren. Für kleinere Mengen gelten andere Konditionen. Wir beraten Sie gerne persönlich." {
        return Err(error("Globale Großmengen-/Qualitätsbedingungen geändert"));
    }
    let mut accepted = HashSet::new();
    for p in &out.prices {
        if accepted.insert(p.material) {
            let original_grades = out
                .prices
                .iter()
                .filter(|quote| quote.material == p.material)
                .map(|quote| {
                    quote
                        .variant
                        .split(" — ")
                        .next()
                        .expect("constructed original label")
                })
                .collect::<Vec<_>>()
                .join("; ");
            out.acceptances.push(ScrapedAcceptance {
                material: p.material,
                conditions: format!(
                    "{CONDITIONS}; {ACCEPTANCE_SCOPE}; Originalsorten: {original_grades}"
                ),
                label: p.label.clone(),
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("fixtures/schrottjungs-gewerbe-20261011.html");
    const OPERATOR_FIXTURE: &str =
        include_str!("fixtures/schrottjungs-impressum-operator-20261013.html");

    #[test]
    fn rereview_color_fallback_is_not_proven_by_unrelated_definition() {
        assert!(parse_source(PAGE_FIXTURE).is_ok());
        let css =
            ".unrelated{--review-color:black}.sj-preis-tag{color:var(--review-color,transparent)}";
        assert!(parse_source(&format!("<style>{css}</style>{PAGE_FIXTURE}")).is_err());
    }

    #[test]
    fn rereview_operator_rejects_generated_identity_anywhere_in_main() {
        for css in [
            ".review-provider::after{content:'Aktueller Betreiber: Andere GmbH'}",
            "main::before{content:'Aktueller Betreiber: Andere GmbH'}",
            "body::after{content:'Aktueller Betreiber: Andere GmbH'}",
        ] {
            let html = format!(
                "<style>{css}</style>{}",
                OPERATOR_FIXTURE.replace("</main>", "<span class='review-provider'></span></main>")
            );
            assert!(
                check_operator(&html).is_err(),
                "generated legal context: {css}"
            );
        }
        let decorative = format!(
            "<style>.review-provider::after{{content:'›'}}</style>{}",
            OPERATOR_FIXTURE.replace("</main>", "<span class='review-provider'></span></main>")
        );
        assert!(check_operator(&decorative).is_ok());
    }

    #[test]
    fn rereview_operator_technical_data_is_not_identity_or_context() {
        for technical in [
            "<script type='application/ld+json'>{\"operator\":\"Aktueller Betreiber: Andere GmbH\"}</script>",
            "<style>.unrelated{--note:'Aktueller Betreiber: Andere GmbH'}</style>",
            "<template><p>Aktueller Betreiber: Andere GmbH</p></template>",
            "<noscript><p>Aktueller Betreiber: Andere GmbH</p></noscript>",
        ] {
            let html = OPERATOR_FIXTURE.replace("</main>", &format!("{technical}</main>"));
            assert!(check_operator(&html).is_ok(), "uninvolved technical data: {technical}");
        }
        for tag in ["script", "style", "template", "noscript"] {
            assert!(
                check_operator(&format!("<{tag}>{OPERATOR_FIXTURE}</{tag}>")).is_err(),
                "technical-only identity: {tag}"
            );
        }
        let stale_script = OPERATOR_FIXTURE
            .replace("Schrottjungs UG", "Andere GmbH")
            .replace("</main>", "<script>Schrottjungs UG</script></main>");
        assert!(check_operator(&stale_script).is_err());
    }

    #[test]
    fn review_source_rejects_whitespace_separated_zero_scale_axis() {
        for value in ["scale(1 0)", "scale(0 1)", "scale(1, 0)"] {
            let html =
                format!("<style>.sj-preis-tag {{ transform: {value}; }}</style>{PAGE_FIXTURE}");
            assert!(parse_source(&html).is_err(), "zero scale axis: {value}");
        }
    }

    #[test]
    fn review_source_rejects_rgb_and_hsl_comma_zero_alpha_aliases() {
        for value in ["rgb(0,0,0,0)", "hsl(0,0%,0%,0)"] {
            let html = format!("<style>.sj-preis-tag {{ color: {value}; }}</style>{PAGE_FIXTURE}");
            assert!(parse_source(&html).is_err(), "zero alpha alias: {value}");
        }
    }

    #[test]
    fn review_source_decodes_escaped_custom_property_definitions() {
        let html = format!(
            r"<style>:root {{ --h\69 de:transparent }} .sj-preis-tag {{ color:var(--hide); }}</style>{PAGE_FIXTURE}"
        );
        assert!(parse_source(&html).is_err());
    }

    #[test]
    fn review_source_rejects_repeated_nesting_with_cross_parent_semantics() {
        for css in [
            ".sj-preis-name,.sj-preis-tag { & + & {color:transparent;} }",
            ".sj-preis-item { & & & & & & & & {color:transparent;} }",
        ] {
            assert!(
                parse_source(&format!("<style>{css}</style>{PAGE_FIXTURE}")).is_err(),
                "unsupported repeated nesting: {css}"
            );
        }
    }

    #[test]
    #[ignore = "requires SCHROTTJUNGS_EVIDENCE_DIR with freshly fetched source HTML"]
    fn review_fresh_source_evidence() {
        let dir = std::env::var("SCHROTTJUNGS_EVIDENCE_DIR").unwrap();
        let impressum = std::fs::read_to_string(format!("{dir}/impressum.html")).unwrap();
        let opener = "<section id=\"brxe-xepois\" class=\"brxe-section sj-contact\">";
        let excerpt = |html: &str| {
            let start = html.find(opener).unwrap();
            let end = start + html[start..].find("</section>").unwrap() + "</section>".len();
            html[start..end].to_owned()
        };
        assert_eq!(
            excerpt(OPERATOR_FIXTURE),
            excerpt(&impressum),
            "operator fixture must retain the actual raw section verbatim"
        );
        check_operator(&impressum).unwrap();
        for technical in [
            "<script type='application/ld+json'>{\"operator\":\"Andere GmbH\"}</script>",
            "<style>.unrelated{--operator:'Andere GmbH'}</style>",
            "<template><p>Aktueller Betreiber: Andere GmbH</p></template>",
            "<noscript><p>Aktueller Betreiber: Andere GmbH</p></noscript>",
        ] {
            check_operator(&impressum.replace("</main>", &format!("{technical}</main>"))).unwrap();
        }
        let out =
            parse_source(&std::fs::read_to_string(format!("{dir}/gewerbe.html")).unwrap()).unwrap();
        assert_eq!(
            (
                out.prices.len(),
                out.skipped_labels.len(),
                out.acceptances.len()
            ),
            (11, 8, 10)
        );
        assert!(out.skip_price_acceptances);
        assert!(out.trader_info.is_empty());
        for price in &out.prices {
            assert_eq!(price.price_kind, "upto");
            assert_eq!(price.price_max, Some(price.price));
            assert!(price.variant.ends_with(CONDITIONS));
        }
        for acceptance in &out.acceptances {
            assert!(acceptance.conditions.starts_with(CONDITIONS));
            assert!(acceptance.conditions.contains(ACCEPTANCE_SCOPE));
        }
        println!("fresh actual sources: 11 prices, 8 skips, 10 conditioned acceptances; no trader_info/GEO; automatic price acceptances disabled");
    }

    #[test]
    fn review_repeated_nesting_does_not_lose_cross_parent_matches() {
        // A price follows a name in the real DOM: :is(name, tag) +
        // :is(name, tag) matches it, unlike name+name or tag+tag.
        let css = ".sj-preis-name, .sj-preis-tag { & + & { color:transparent } }";
        assert!(parse_source(&format!("<style>{css}</style>{PAGE_FIXTURE}")).is_err());
        // Bounded adversarial input: reject before constructing expansions.
        let css = format!(
            ".{} {{ {} {{ color:transparent }} }}",
            "a".repeat(2048),
            "& + ".repeat(1024) + "&"
        );
        assert!(parse_source(&format!("<style>{css}</style>{PAGE_FIXTURE}")).is_err());
    }

    #[test]
    fn review_single_nesting_expansion_has_aggregate_byte_budget() {
        // Small source, at most 8 MB of bounded expansions: no giant & input.
        let children = (0..512)
            .map(|index| format!("& .child{index}"))
            .collect::<Vec<_>>()
            .join(",");
        let css = format!(
            ".{} {{ {children} {{ color:transparent }} }}",
            "a".repeat(16_384)
        );
        assert!(parse_source(&format!("<style>{css}</style>{PAGE_FIXTURE}")).is_err());
    }

    #[test]
    fn review_escaped_custom_property_definitions_fail_closed() {
        for css in [
            r":root{--h\69 de:transparent} .sj-preis-tag{color:var(--hide)}",
            r":root{--hide:transparent} .sj-preis-tag{color:var(--h\69 de)}",
            r":root{--hide:black;--h\69 de:transparent} .sj-preis-tag{color:var(--hide)}",
        ] {
            assert!(
                parse_source(&format!("<style>{css}</style>{PAGE_FIXTURE}")).is_err(),
                "{css}"
            );
        }
    }

    #[test]
    fn review_zero_alpha_color_aliases_fail_closed() {
        for value in ["rgb(0,0,0,0)", "hsl(0,0%,0%,0)"] {
            assert!(
                parse_source(&format!(
                    "<style>.sj-preis-tag {{ color:{value} }}</style>{PAGE_FIXTURE}"
                ))
                .is_err(),
                "{value}"
            );
        }
        assert!(parse_source(&format!(
            "<style>.sj-preis-tag {{ color:rgb(0,0,0,1) }}</style>{PAGE_FIXTURE}"
        ))
        .is_ok());
    }

    #[test]
    fn review_space_separated_scale_zero_fails_closed() {
        for value in ["scale(1 0)", "scale(0 1)", "scale(1, 0)"] {
            let html =
                format!("<style>.sj-preis-tag {{ transform:{value} }}</style>{PAGE_FIXTURE}");
            assert!(parse_source(&html).is_err(), "{value}");
        }
        assert!(parse_source(&format!(
            "<style>.sj-preis-tag {{ transform:scale(1 1) }}</style>{PAGE_FIXTURE}"
        ))
        .is_ok());
    }

    #[test]
    fn review_operator_requires_unique_current_visible_qualified_legal_block() {
        check_operator(OPERATOR_FIXTURE).unwrap();
        for html in [
            format!("<div hidden>{OPERATOR_FIXTURE}</div>"),
            format!("<script type='application/ld+json'>{OPERATOR_FIXTURE}</script>"),
            format!("<style>{OPERATOR_FIXTURE}</style>"),
            OPERATOR_FIXTURE.replace("<strong>", "<strong hidden>"),
            OPERATOR_FIXTURE.replace("Schrottjungs UG", "Nicht mehr Schrottjungs UG"),
            OPERATOR_FIXTURE.replace(
                "Geschäftsführer: Magnus Ditz",
                "Früher Geschäftsführer: Magnus Ditz",
            ),
            OPERATOR_FIXTURE.replace(
                "<dl ",
                "<p>Aktueller Betreiber: Andere GmbH, HRB 999999</p><dl ",
            ),
            OPERATOR_FIXTURE.replace(
                "</section>",
                "</section><section><p>Aktueller Betreiber: Andere GmbH, HRB 999999</p></section>",
            ),
            format!("{OPERATOR_FIXTURE}{OPERATOR_FIXTURE}"),
            format!("<style>.sj-contact {{ display:none }}</style>{OPERATOR_FIXTURE}"),
        ] {
            assert!(
                check_operator(&html).is_err(),
                "operator drift was accepted: {html}"
            );
        }
    }

    #[test]
    fn all_nineteen_partitioned_bounded_and_visible_conditions() {
        let out = parse(FIXTURE).unwrap();
        assert_eq!(out.prices.len(), 11);
        assert_eq!(out.skipped_labels.len(), 8);
        assert_eq!(out.published_at, None);
        assert!(out.trader_info.is_empty());
        assert_eq!(out.acceptances.len(), 10);
        assert!(out.skip_price_acceptances);
        for p in &out.prices {
            assert_eq!(p.price_kind, "upto");
            assert_eq!(p.price_max, Some(p.price));
            assert_eq!(p.price_min, None);
            assert_eq!(p.confidence, Some(0.5));
            assert!(p.variant.ends_with(CONDITIONS));
            for required in [
                "Bundesweiter Gewerbe-/Großmengenrichtwert",
                "unverbindlich bis zu",
                "Menge, Qualität, Sortierung und Marktlage",
                "kleinere Mengen andere Konditionen",
                "Tagesangebot auf Anfrage",
                "kein Privat-/Entel-Tarif",
                "Quelldatum unbekannt; ggf. Datum der Preisänderungsbeobachtung",
            ] {
                assert!(p.variant.contains(required));
            }
            assert!(p.label.ends_with(CONDITIONS));
            assert!(p.label.contains(if p.unit == "EUR/t" {
                "€/Tonne"
            } else {
                "€/kg"
            }));
        }
        for (label, amount, unit) in [
            ("Mischschrott leicht", 170.0, "EUR/t"),
            ("Mischschrott schwer", 210.0, "EUR/t"),
            ("Kupfer Millberry", 9.5, "EUR/kg"),
            ("Edelstahl V2A", 1.1, "EUR/kg"),
            ("Trägerschrott", 230.0, "EUR/t"),
            ("Bremsscheiben", 240.0, "EUR/t"),
            ("Kupfer Raff", 8.5, "EUR/kg"),
            ("Messing gemischt", 4.5, "EUR/kg"),
            ("Rotguss", 8.0, "EUR/kg"),
            ("Aluminium (div.)", 2.5, "EUR/kg"),
            ("Zink", 1.9, "EUR/kg"),
        ] {
            let p = out
                .prices
                .iter()
                .find(|p| p.variant.starts_with(label))
                .unwrap();
            assert_eq!((p.price, p.unit), (amount, unit));
        }
        for label in [
            "Scherenvormaterial",
            "Sorte 3",
            "Zerlegematerial",
            "Kupferschiene",
            "Kabel (diverse)",
            "Blei",
            "Zinn",
            "Hartmetall / Wolfram",
        ] {
            assert!(out.skipped_labels.iter().any(|s| s.starts_with(label)
                && s.contains("bis zu")
                && s.contains("nicht gemappt")));
        }
    }

    #[test]
    fn semantic_drift_and_malformed_prices_fail_closed() {
        for html in [
            FIXTURE.replace("bis zu 170", "170"),
            FIXTURE.replace("170 €/Tonne", "170 €/kg"),
            FIXTURE.replace("170 €/Tonne", "170-190 €/Tonne"),
            FIXTURE.replace("170 €/Tonne", "170,5 €/Tonne"),
            FIXTURE.replace("170 €/Tonne", "1.170 €/Tonne"),
            FIXTURE.replace("170 €/Tonne", "0 €/Tonne"),
            FIXTURE.replace("Großmengen", "Kleinmengen"),
            FIXTURE.replace("Ankaufspreise", "Verkaufspreise"),
            FIXTURE.replace("Mischschrott schwer", "Mischschrott leicht"),
            FIXTURE.replacen(
                "class=\"sj-preis-item\"",
                "hidden class=\"sj-preis-item\"",
                1,
            ),
            FIXTURE.replace("sj-preise-section-fe", "missing"),
            FIXTURE.replacen(
                "class=\"sj-preis-tag\"",
                "style=\"display: none\" class=\"sj-preis-tag\"",
                1,
            ),
            FIXTURE.replacen(
                "class=\"sj-preis-item\"",
                "style=\"display:var(--visibility-state)\" class=\"sj-preis-item\"",
                1,
            ),
            FIXTURE.replace(
                "class=\"sj-preis-banner\"",
                "hidden class=\"sj-preis-banner\"",
            ),
            FIXTURE
                .replace("Mischschrott leicht", "SWAP")
                .replace("Kupfer Raff", "Mischschrott leicht")
                .replace("SWAP", "Kupfer Raff"),
        ] {
            assert!(parse(&html).is_err(), "{html}");
        }
        let escaped_variable = FIXTURE.replacen(
            "class=\"sj-preis-item\"",
            r#"style="display: v\61r(--visibility-state)" class="sj-preis-item""#,
            1,
        );
        assert!(parse(&escaped_variable).is_err());
        let mut changed = parse(&FIXTURE.replace("170 €/Tonne", "180 €/Tonne")).unwrap();
        assert_eq!(changed.prices.remove(0).price_max, Some(180.0));
        let unknown_grade = FIXTURE
            .replace("Zink</p>", "Unbekannt</p>")
            .replace("alt=\"Zink\"", "alt=\"Unbekannt\"");
        let out = parse(&unknown_grade).unwrap();
        assert_eq!((out.prices.len(), out.skipped_labels.len()), (10, 9));
        assert!(out
            .skipped_labels
            .iter()
            .any(|s| s.starts_with("Unbekannt")));
        let decoy = format!("{FIXTURE}<div class='sj-preis-item'><p class='sj-preis-name'>Zinn</p><p class='sj-preis-tag'>bis zu 999 €/kg</p></div><p>Gebühren 40-80 Euro; Stand 11.10.2026; USt 19%</p>");
        assert_eq!(parse(&decoy).unwrap().prices.len(), 11);
        assert_eq!(parse(&decoy).unwrap().published_at, None);
    }

    #[test]
    fn b2_unconsumed_source_conditions_and_outside_grid_quote_fail_closed() {
        for html in [
            format!("<style>.sj-preis-tag::after {{ content: 'Nur ab 10 Tonnen, netto'; }}</style>{FIXTURE}"),
            FIXTURE.replace("bis zu 170 €/Tonne</p>", "bis zu 170 €/Tonne</p><p class='sj-preis-condition'>Nur ab 10 Tonnen, netto, keine Abholung</p>"),
            FIXTURE.replacen("<div class=\"sj-preis-grid\">", "<p>Alle folgenden Preise ausschließlich für Privatkunden</p><div class=\"sj-preis-grid\">", 1),
            FIXTURE.replacen("</section>", "<div class='sj-preis-item'><p class='sj-preis-name'>Extra</p><p class='sj-preis-tag'>bis zu 999 €/Tonne</p></div></section>", 1),
            FIXTURE.replacen("<div class=\"sj-preis-grid\">", "<span>Nur netto und nach Sortierung</span><div class=\"sj-preis-grid\">", 1),
            FIXTURE.replacen("class=\"sj-preis-item\"", "data-min-quantity='10t' class=\"sj-preis-item\"", 1),
            FIXTURE.replacen("class=\"sj-preis-item\"", "min-quantity='10t' class=\"sj-preis-item\"", 1),
            FIXTURE.replacen("class=\"sj-preise-section-fe\"", "data-customer-type='privat' class=\"sj-preise-section-fe\"", 1),
            FIXTURE.replacen("bis zu 170 €/Tonne</p>", "bis zu 170 €/Tonne</p><img alt='Nur ab 10 Tonnen netto'>", 1),
            FIXTURE.replacen("</section>", "<img alt='Nur Privatkunden'></section>", 1),
        ] { assert!(parse(&html).is_err(), "unconsumed conditions/quote must not produce an unchanged qualified feed"); }
    }

    #[test]
    fn full_page_context_and_generated_sibling_conditions_are_pinned() {
        let page = PAGE_FIXTURE;
        assert!(parse_source(page).is_ok(), "{:?}", parse_source(page));

        let sibling_notice = page.replacen("</p>", "Nur ab 10 Tonnen, netto</p>", 1);
        assert!(
            parse_source(&sibling_notice).is_err(),
            "new page-level conditions outside the two price sections must block ingestion"
        );

        let colliding_class_sibling = page.replacen(
            "</main>",
            "<div class=\"sj-preise-section-fe\">Nur ab 10 Tonnen, netto</div></main>",
            1,
        );
        assert!(
            parse_source(&colliding_class_sibling).is_err(),
            "a sibling that reuses a section class is still page-level context"
        );

        let sibling_marker = page.replacen(
            "</main>",
            "<style>.page-qualifier::after { content: 'Nur ab 10 Tonnen, netto'; }</style><span class=\"page-qualifier\"></span></main>",
            1,
        );
        assert!(
            parse_source(&sibling_marker).is_err(),
            "CSS-generated sibling conditions in the main page must block ingestion"
        );
        let empty_sibling_marker = page.replacen(
            "</main>",
            "<style>.page-qualifier::after { content: ''; }</style><span class=\"page-qualifier\"></span></main>",
            1,
        );
        assert!(parse_source(&empty_sibling_marker).is_ok());
    }

    #[test]
    fn b3_hidden_semantic_anchors_and_embedded_css_classes_fail_closed() {
        for marker in [
            "class=\"sj-preise-header\"",
            "class=\"sj-preise-sub\"",
            "class=\"sj-overline\"",
        ] {
            // First AND last occurrence independently: both FE and NE anchors.
            for at in [
                FIXTURE.find(marker).unwrap(),
                FIXTURE.rfind(marker).unwrap(),
            ] {
                let mut html = FIXTURE.to_owned();
                html.insert_str(at, "hidden ");
                assert!(
                    parse(&html).is_err(),
                    "hidden individual semantic anchor: {marker} at {at}"
                );
            }
        }
        for html in [
            FIXTURE.replacen("<h2>", "<h2 hidden>", 1),
            FIXTURE.replacen("class=\"sj-preise-header\"", "aria-hidden='true' class=\"sj-preise-header\"", 1),
            FIXTURE.replacen("class=\"sj-preise-sub\"", "hidden class=\"sj-preise-sub\"", 1),
            FIXTURE.replace("class=\"sj-overline\"", "hidden class=\"sj-overline\""),
            format!("<style>.review-hidden {{ display:none }}</style>{}", FIXTURE.replacen("class=\"sj-preis-item\"", "class=\"sj-preis-item review-hidden\"", 1)),
            format!("<style>@media print {{ .review-hidden {{ visibility: hidden !important }} }}</style>{}", FIXTURE.replacen("class=\"sj-preise-sub\"", "class=\"sj-preise-sub review-hidden\"", 1)),
            format!("<style>.review-hidden {{ opacity: 0.00 }}</style>{}", FIXTURE.replacen("class=\"sj-preis-item\"", "class=\"sj-preis-item review-hidden\"", 1)),
            format!("<style>.review-hidden {{ transform: scale(0) }}</style>{}", FIXTURE.replacen("class=\"sj-preis-item\"", "class=\"sj-preis-item review-hidden\"", 1)),
            format!("<style>.review-hidden {{ clip-path: inset(50%) }}</style>{}", FIXTURE.replacen("class=\"sj-preis-item\"", "class=\"sj-preis-item review-hidden\"", 1)),
            format!("<style>.review-hidden {{ filter: opacity(0) }}</style>{}", FIXTURE.replacen("class=\"sj-preis-item\"", "class=\"sj-preis-item review-hidden\"", 1)),
        ] { assert!(parse(&html).is_err(), "hidden semantic anchor or matching stylesheet must not authorize ingestion"); }
        // Unrelated navigation hiding is not a price-window condition.
        assert!(parse(&format!("<style>.foreign-nav {{ display:none }}</style><nav class='foreign-nav'>Navigation</nav>{FIXTURE}")).is_ok());
    }

    #[test]
    fn embedded_stylesheet_selectors_and_quote_css_are_checked() {
        assert_eq!(
            broaden_state_pseudos(".sr-only:not(:focus, :active)"),
            ".sr-only"
        );
        assert_eq!(broaden_state_pseudos(".button:hover"), ".button");
        assert_eq!(
            broaden_state_pseudos(".item:not(.disabled)"),
            ".item:not(.disabled)"
        );
        assert_eq!(
            broaden_state_pseudos(r#".item[data-label=":focus"]"#),
            r#".item[data-label=":focus"]"#
        );

        let nested_state_rule = ".sj-preis-item:not(:focus, :active) { display:none }";
        let rules = css_visibility_selectors(nested_state_rule).unwrap();
        let doc = Html::parse_document(FIXTURE);
        let card = doc.select(&selector(".sj-preis-item")).next().unwrap();
        assert!(rules.hidden.iter().any(|rule| rule.matches(&card)));
        assert!(parse(&format!("<style>{nested_state_rule}</style>{FIXTURE}")).is_err());
        assert_eq!(
            css_visibility_selectors("h1, .review-hidden { display:none }")
                .unwrap()
                .hidden
                .len(),
            2
        );

        for css in [
            ".sj-preis-tag::after { content: 'Nur ab 10 Tonnen, netto'; }",
            ".review-hidden { transform: scale(0) }",
            ".review-hidden { clip-path: inset(50%) }",
            ".review-hidden { clip: rect(0, 0, 0, 0) }",
            ".review-hidden { filter: opacity(0) }",
            ".review-hidden { color: rgb(0 0 0 / 0) }",
            ".sj-preis-item { color: var(--missing-color, transparent) }",
            ":root { --dynamic-color: #111; } @media print { :root { --dynamic-color: transparent; } } .sj-preis-item { color: var(--dynamic-color); }",
            ".sj-preis-item { & .sj-preis-tag { color: rgb(0 0 0 / 0); } }",
            ".review-hidden { color: rgba(0, 0, 0, 0) }",
            ".review-hidden { -webkit-text-fill-color: #00000000 }",
            ":root { --hidden: none; } .sj-preis-item { display: var(--hidden); }",
            ":root { --hidden: none; } .sj-preis-item { display: v\\61r(--hidden); }",
            ".sj-preis-item { d\\69splay: none; }",
            ".sj-preis-item { opacity: var(--possibly-hidden, 1); }",
            ".sj-preis-item { animation: fade-out 2s forwards; } @keyframes fade-out { from { opacity: 1; } to { opacity: 0; } }",
        ] {
            let html = FIXTURE.replacen(
                "class=\"sj-preis-item\"",
                "class=\"sj-preis-item review-hidden\"",
                1,
            );
            assert!(
                parse(&format!("<style>{css}</style>{html}")).is_err(),
                "embedded CSS must not hide or qualify a parsed source window: {css}"
            );
        }
        let nested_hidden = ".sj-preis-item { & .sj-preis-tag { color: rgb(0 0 0 / 0); } }";
        assert!(parse(&format!("<style>{nested_hidden}</style>{FIXTURE}")).is_err());
        let unrelated_nested = ".popup-sj__form { & .options-wrapper { & li { display:none; } } }";
        assert!(parse(&format!("<style>{unrelated_nested}</style>{FIXTURE}")).is_ok());
        let inline_transparent_variable =
            ":root { --hidden-color: rgb(0 0 0 / 0); } .sj-preis-item { color: var(--hidden-color); }";
        assert!(parse(&format!(
            "<style>{inline_transparent_variable}</style>{FIXTURE}"
        ))
        .is_err());
        let inline_transparent_text = FIXTURE.replacen(
            "class=\"sj-preis-item\"",
            "style=\"color:var(--hidden-color)\" class=\"sj-preis-item\"",
            1,
        );
        assert!(parse(&format!(
            "<style>:root {{ --hidden-color: rgb(0 0 0 / 0); }}</style>{inline_transparent_text}"
        ))
        .is_err());
        let unresolved_external_variable =
            ".sj-preis-item { color: var(--externally-defined-color); }";
        assert!(parse(&format!(
            "<style>{unresolved_external_variable}</style>{FIXTURE}"
        ))
        .is_ok());
        assert!(parse(&format!(
            "<style>.sj-preis-tag::after {{ content: \"/*Nur ab 10 Tonnen, netto*/\"; }}</style>{FIXTURE}"
        ))
        .is_err());
        for style in [
            "color: rgb(0 0 0 / 0)",
            "color: rgba(0, 0, 0, 0)",
            "-webkit-text-fill-color: #00000000",
        ] {
            let html = FIXTURE.replacen(
                "class=\"sj-preis-item\"",
                &format!("style=\"{style}\" class=\"sj-preis-item\""),
                1,
            );
            assert!(
                parse(&html).is_err(),
                "transparent inline price style: {style}"
            );
        }
        for css in [
            ".sj-preis-tag::after { content: ''; }",
            ".sj-preis-tag::after { content: \"›\"; }",
            ".sj-preis-tag::after { content: \"\\f105\"; }",
        ] {
            assert!(
                parse(&format!("<style>{css}</style>{FIXTURE}")).is_ok(),
                "empty or decorative pseudo-element content must not block scraping: {css}"
            );
        }
        assert!(parse(&format!(
            "<style>@import url('/other.css');</style>{FIXTURE}"
        ))
        .is_err());
    }

    #[test]
    fn operator_and_single_canonical_registration() {
        let imp = OPERATOR_FIXTURE;
        check_operator(imp).unwrap();
        assert!(check_operator(&imp.replace("196586", "1965860")).is_err());
        for url in [URL, IMPRESSUM] {
            validate_response(url, url, 200).unwrap();
            assert!(validate_response(url, "https://foreign.test/gewerbe/", 200).is_err());
            assert!(validate_response(url, url, 201).is_err());
            assert!(validate_response(url, url, 404).is_err());
        }
        for marker in [
            "Schrottjungs UG",
            "HRB 196586",
            "Magnus Ditz",
            "20537 Hamburg",
        ] {
            assert!(check_operator(&imp.replace(marker, "Fremd")).is_err());
        }
        let handlers = super::super::all();
        assert_eq!(handlers.iter().filter(|h| h.slug == SLUG).count(), 1);
        assert_eq!(handlers.iter().filter(|h| h.url == URL).count(), 1);
    }

    #[tokio::test]
    async fn owned_fetch_rejects_redirect_without_contacting_private_target() {
        use std::io::Write;
        use std::net::TcpListener;
        use std::sync::mpsc;
        use std::time::{Duration, Instant};

        let target = TcpListener::bind("127.0.0.1:0").unwrap();
        target.set_nonblocking(true).unwrap();
        let target_url = format!("http://{}/private", target.local_addr().unwrap());
        let (contact_tx, contact_rx) = mpsc::channel();
        let target_thread = std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_millis(750);
            loop {
                match target.accept() {
                    Ok((_, _)) => {
                        let _ = contact_tx.send(true);
                        return;
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline {
                            let _ = contact_tx.send(false);
                            return;
                        }
                        std::thread::sleep(Duration::from_millis(5));
                    }
                    Err(error) => panic!("target listener failed: {error}"),
                }
            }
        });

        let redirect = TcpListener::bind("127.0.0.1:0").unwrap();
        let redirect_url = format!("http://{}/allowed", redirect.local_addr().unwrap());
        let redirect_thread = std::thread::spawn(move || {
            let (mut stream, _) = redirect.accept().unwrap();
            write!(
                stream,
                "HTTP/1.1 302 Found\r\nLocation: {target_url}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            )
            .unwrap_or_else(|error| {
                // A no-redirect client may close as soon as it has the 302
                // headers, before write! finishes its remaining fragments.
                assert!(
                    matches!(error.kind(), std::io::ErrorKind::BrokenPipe | std::io::ErrorKind::ConnectionReset),
                    "redirect response write failed unexpectedly: {error}"
                );
            });
        });

        let result = owned_fetch(&source_client().unwrap(), &redirect_url).await;
        assert!(result.is_err(), "redirect response must fail closed");
        assert!(!contact_rx.recv_timeout(Duration::from_secs(2)).unwrap());
        redirect_thread.join().unwrap();
        target_thread.join().unwrap();
    }

    #[tokio::test]
    async fn b1_conditioned_acceptances_preserve_foreign_baseline_without_new_fallbacks() {
        use schrott_mcp_store::{InternalDb, PublicDb};
        let dir = crate::test_support::TempDbDir::new("schrottjungs-b1");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let trader = public.find_trader_id(SLUG).unwrap().unwrap();
        let baseline_at = "2026-10-10T00:00:00+00:00";
        for (slug, accepts, conditions) in [
            (
                "stahlschrott-scheren",
                false,
                "Fremdbeleg: keine Scherenannahme",
            ),
            ("edelstahl-gemischt", true, "Fremdbeleg: nur nach Analyse"),
            ("kabel-alu", true, "Fremdbeleg: separate Alukabelannahme"),
        ] {
            public
                .set_acceptance(
                    trader,
                    public.find_material_id(slug).unwrap().unwrap(),
                    accepts,
                    conditions,
                    None,
                    None,
                    baseline_at,
                    baseline_at,
                )
                .unwrap();
        }
        let foreign_query = format!("SELECT tm.* FROM trader_materials tm JOIN materials m ON m.id=tm.material_id WHERE tm.trader_id={trader} AND m.slug IN ('stahlschrott-scheren','edelstahl-gemischt','kabel-alu') ORDER BY tm.material_id");
        let before = public.query_sql(&foreign_query).unwrap().rows;
        assert_eq!(before.len(), 3);
        let out = parse(FIXTURE).unwrap();
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-11T20:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let mixed = out
            .acceptances
            .iter()
            .find(|a| a.material == "mischschrott")
            .unwrap();
        assert!(mixed
            .conditions
            .contains("Originalsorten: Mischschrott leicht; Mischschrott schwer"));
        let copper = out
            .acceptances
            .iter()
            .find(|a| a.material == "kupfer-gemischt")
            .unwrap();
        assert!(copper.conditions.contains("Originalsorten: Kupfer Raff"));
        let result = super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        assert_eq!(public.query_sql(&foreign_query).unwrap().rows, before, "foreign pre-existing evidence must not be overwritten/purged by price-derived direct or fallback writes");
        let matrix = public.query_sql(&format!("SELECT m.slug,tm.accepts,tm.conditions FROM trader_materials tm JOIN materials m ON m.id=tm.material_id WHERE tm.trader_id={trader} AND m.slug NOT IN ('stahlschrott-scheren','edelstahl-gemischt','kabel-alu') ORDER BY m.slug")).unwrap();
        assert_eq!(matrix.rows.len(), 10, "ten new distinct mapped material acceptances, not eleven variants or twelve direct+fallback claims");
        assert_eq!(
            matrix
                .rows
                .iter()
                .map(|r| r[0].as_str().unwrap())
                .collect::<Vec<_>>(),
            [
                "aluminium-gemischt",
                "bronze-rotguss",
                "edelstahl-v2a",
                "eisenschrott-gussbruch",
                "kupfer-gemischt",
                "kupfer-millberry",
                "messing",
                "mischschrott",
                "stahlschrott-traeger",
                "zink"
            ]
        );
        for row in &matrix.rows {
            assert_eq!(row[1], 1);
            let conditions = row[2].as_str().unwrap();
            assert!(
                conditions.contains("keine Zusage einer Annahme durch einen lokalen Partnerhof")
            );
            for required in [
                "Gewerbe",
                "Großmengen",
                "Menge, Qualität, Sortierung und Marktlage",
                "Tagesangebot auf Anfrage",
                "kein Privat-/Entel-Tarif",
            ] {
                assert!(conditions.contains(required), "{row:?}");
            }
        }
        assert_eq!(result.accepted, 10);
        assert_eq!(result.recorded, 11);
        assert_eq!(result.skipped.len(), 8);
        let total = public
            .query_sql(&format!(
                "SELECT count(*) FROM trader_materials WHERE trader_id={trader}"
            ))
            .unwrap();
        assert_eq!(
            total.rows[0][0], 13,
            "three preserved baseline rows plus ten new writes; no baseline purge"
        );
        let all_query =
            format!("SELECT * FROM trader_materials WHERE trader_id={trader} ORDER BY material_id");
        let before_repeat = public.query_sql(&all_query).unwrap().rows;
        let repeated = super::super::super::record(
            &public,
            &internal,
            SLUG,
            &out,
            &(now + chrono::Duration::hours(1)),
        )
        .await
        .unwrap();
        assert_eq!(
            repeated.accepted, 10,
            "accepted counts resolved explicit entries, not changed writes"
        );
        assert_eq!(
            public.query_sql(&all_query).unwrap().rows,
            before_repeat,
            "unchanged explicit acceptances have zero write delta, including timestamps"
        );
        let view = public.query_sql(&format!("SELECT variant,price_kind,price,price_max,price_min FROM v_current_prices WHERE trader_slug='{SLUG}'")).unwrap();
        assert_eq!(view.rows.len(), 11);
        for row in &view.rows {
            assert!(row[0].as_str().unwrap().ends_with(CONDITIONS));
            assert_eq!(row[1], "upto");
            assert_eq!(row[2], row[3]);
            assert!(row[4].is_null());
        }
    }

    #[tokio::test]
    async fn b1_legacy_default_outcomes_keep_direct_fallback_and_explicit_acceptances() {
        use schrott_mcp_store::{InternalDb, PublicDb};
        let dir = crate::test_support::TempDbDir::new("legacy-price-acceptance");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-11T20:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        // Synthetic quotes at the shared recorder seam, NOT claimed live Beller/
        // Vedder prices. Both seeded legacy handler identities use the default.
        for handler in [
            super::super::beller::handler(),
            super::super::vedder::handler(),
        ] {
            let mut out = HandlerOutcome {
                fetch_url: handler.url.to_owned(),
                ..Default::default()
            };
            assert!(!out.skip_price_acceptances);
            for (material, unit) in [
                ("eisenschrott-gussbruch", "EUR/t"),
                ("edelstahl-v2a", "EUR/kg"),
            ] {
                out.prices.push(ScrapedPrice {
                    material,
                    variant: "recorder-regression".into(),
                    price: 1.0,
                    currency: "EUR",
                    unit,
                    price_kind: "exact",
                    price_min: None,
                    price_max: None,
                    confidence: Some(1.0),
                    label: "synthetic recorder regression".to_owned(),
                });
            }
            out.acceptances.push(ScrapedAcceptance {
                material: "aluminium-gemischt",
                conditions: "nur nach bestätigtem Angebot".to_owned(),
                label: "explicit regression".to_owned(),
            });
            super::super::super::record(&public, &internal, handler.slug, &out, &now)
                .await
                .unwrap();
            let trader = public.find_trader_id(handler.slug).unwrap().unwrap();
            for (material, condition) in [
                ("eisenschrott-gussbruch", ""),
                ("edelstahl-v2a", ""),
                (
                    "stahlschrott-scheren",
                    "Annahme via eisenschrott-gussbruch-Ankauf",
                ),
                ("edelstahl-gemischt", "Annahme via edelstahl-v2a-Ankauf"),
                ("aluminium-gemischt", "nur nach bestätigtem Angebot"),
            ] {
                let id = public.find_material_id(material).unwrap().unwrap();
                assert_eq!(
                    public.existing_acceptance(trader, id).unwrap(),
                    Some((true, condition.to_owned()))
                );
            }
        }
    }

    #[tokio::test]
    async fn real_catalog_record_and_change_date_reach_standard_view_honestly() {
        use schrott_mcp_store::{InternalDb, PublicDb};
        let dir = crate::test_support::TempDbDir::new("schrottjungs");
        let public = PublicDb::open(dir.path()).unwrap();
        let internal = InternalDb::open(dir.path()).unwrap();
        crate::seed_metadata(&public).unwrap();
        let now = chrono::DateTime::parse_from_rfc3339("2026-10-11T20:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let out = parse(FIXTURE).unwrap();
        let recorded = super::super::super::record(&public, &internal, SLUG, &out, &now)
            .await
            .unwrap();
        assert_eq!((recorded.recorded, recorded.skipped.len()), (11, 8));
        assert_eq!(recorded.accepted, 10);
        let matrix = public.query_sql(&format!("SELECT m.slug,tm.conditions FROM trader_materials tm JOIN materials m ON m.id=tm.material_id JOIN traders t ON t.id=tm.trader_id WHERE t.slug='{SLUG}' ORDER BY m.slug")).unwrap();
        assert_eq!(
            matrix.rows.len(),
            10,
            "fresh DB receives no unproven broader fallback rows"
        );
        for row in &matrix.rows {
            assert!(row[1].as_str().unwrap().starts_with(&format!(
                "{CONDITIONS}; {ACCEPTANCE_SCOPE}; Originalsorten: "
            )));
            assert!(!matches!(
                row[0].as_str().unwrap(),
                "stahlschrott-scheren" | "edelstahl-gemischt"
            ));
        }
        let rows = public.query_sql(&format!("SELECT variant,price,unit,price_kind,price_min,price_max,published_at,source_url FROM v_current_prices WHERE trader_slug='{SLUG}' ORDER BY variant")).unwrap();
        assert_eq!(rows.rows.len(), 11);
        for row in &rows.rows {
            assert!(row[0].as_str().unwrap().ends_with(CONDITIONS));
            assert_eq!(row[3], "upto");
            assert!(row[4].is_null());
            assert_eq!(row[1], row[5]);
            assert!(row[6].is_null());
            assert_eq!(row[7], URL);
            let source = out
                .prices
                .iter()
                .find(|p| p.variant.as_ref() == row[0].as_str().unwrap())
                .unwrap();
            assert_eq!(row[1], source.price);
            assert_eq!(row[2], source.unit);
        }
        let changed = parse(&FIXTURE.replace("170 €/Tonne", "180 €/Tonne")).unwrap();
        let later = now + chrono::Duration::hours(6);
        super::super::super::record(&public, &internal, SLUG, &changed, &later)
            .await
            .unwrap();
        let rows = public.query_sql(&format!("SELECT variant,price,price_min,price_max,published_at FROM v_current_prices WHERE trader_slug='{SLUG}' AND variant LIKE 'Mischschrott leicht%'")).unwrap();
        assert_eq!(rows.rows.len(), 1);
        let row = &rows.rows[0];
        assert!(row[0].as_str().unwrap().ends_with(CONDITIONS));
        assert_eq!(row[1], 180.0);
        assert!(row[2].is_null());
        assert_eq!(row[3], 180.0);
        assert_eq!(row[4], "2026-10-12T00:00:00+00:00");
        let trader = public.find_trader_id(SLUG).unwrap().unwrap();
        let material = public.find_material_id("mischschrott").unwrap().unwrap();
        let current = public
            .current_price_for(trader, material, changed.prices[0].variant.as_ref())
            .unwrap()
            .unwrap();
        let extra: serde_json::Value = serde_json::from_str(&current.extra_json).unwrap();
        assert_eq!(extra["published_at_basis"], "observed_price_change");
        assert!(current.notes.contains("180 €/Tonne"));
        assert_eq!(changed.published_at, None);
        let sibling = public.query_sql(&format!("SELECT count(*) FROM v_current_prices WHERE source_url='{URL}' AND trader_slug!='{SLUG}'")).unwrap();
        assert_eq!(sibling.rows[0][0], 0);
    }
}
