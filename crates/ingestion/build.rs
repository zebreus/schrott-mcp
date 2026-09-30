//! Compile-time seed codegen: `dossiers/<state>/<slug>.md` → `$OUT_DIR/seed_traders/<state>.json`.
//!
//! The dossiers are the single source of truth. This script runs on every
//! build of `schrott-mcp-ingestion` (and re-runs whenever any dossier
//! changes) and emits one JSON file per state into OUT_DIR, which
//! `seed_traders.rs` embeds via `include_str!`. Nothing is committed:
//! the generated files live under `target/` only.
//!
//! Format contract (must stay in sync with the dossier convention):
//! - Frontmatter: flat `key: value` scalars only (PyYAML-safe_dump shape).
//!   Long values may be folded across indented continuation lines (joined
//!   with a single space, like YAML folding). Single-quoted (`''`
//!   escape) and double-quoted (basic escapes) supported; anything else
//!   (block scalars, nested maps, duplicate or unknown keys) fails the
//!   build LOUDLY — a bad data file must never silently produce a bad
//!   binary. Verified against `yaml.safe_load` over the full corpus.
//! - `notes`: `- ` bullets under `## Timeline` (until the next `## `
//!   heading), wrapped continuation lines re-attached, whitespace
//!   collapsed, joined with ` | `, cut at 2000 chars.
//!
//! Zero third-party dependencies on purpose: the build must work offline
//! from a fresh clone with only the lockfile cache.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// States in dossier/build order (bw, by, be, bb, hb, hh, he, mv, ni, nw, rp, sl, sn, st, sh, th).
const STATES: &[&str] = &[
    "bw", "by", "be", "bb", "hb", "hh", "he", "mv", "ni", "nw", "rp", "sl", "sn", "st", "sh",
    "th",
];

/// Frontmatter keys (unknown keys = loud typo protection).
const KNOWN_KEYS: &[&str] = &[
    "slug",
    "name",
    "trader_type",
    "state",
    "city",
    "street",
    "postcode",
    "phone",
    "email",
    "opening_hours",
    "website",
    "website_status",
    "status",
    "description",
    "dropoff_json",
    "pickup_json",
    "provenance_seed_file",
    "provenance_section",
    "provenance_ankauf_raw",
    "provenance_origin",
];

fn fail(ctx: &str, msg: &str) -> ! {
    panic!("seed codegen: {ctx}: {msg}");
}

/// A `key:` line starts a new entry (same shape PyYAML accepts as mapping key).
fn is_key_line(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
        i += 1;
    }
    i > 0 && bytes.get(i) == Some(&b':') && matches!(bytes.get(i + 1), None | Some(b' ') | Some(b'\t'))
}

/// Parse one logical `key: value` frontmatter line. Returns (key, scalar string).
fn parse_kv(path: &str, lineno: usize, line: &str) -> (String, String) {
    let ctx = format!("{path}:{lineno}");
    let colon = line.find(':').unwrap_or_else(|| fail(&ctx, &format!("kein ':': {line:?}")));
    let key = line[..colon].to_string();
    if !KNOWN_KEYS.contains(&key.as_str()) {
        fail(&ctx, &format!("unbekannter Key {key:?} (Tippfehler?)"));
    }
    let rest = &line[colon + 1..];
    let raw = match rest.strip_prefix([' ', '\t']) {
        Some(v) => v.trim_start_matches([' ', '\t']),
        None if rest.is_empty() => "",
        None => fail(&ctx, &format!("erwarte Leerzeichen nach ':' bei Key {key:?}")),
    };
    (key, parse_scalar(&ctx, raw))
}

/// Parse a flat YAML scalar (PyYAML-safe_dump shape): plain, 'single' or
/// "double"-quoted. Empty / null-ish / comment → "".
fn parse_scalar(ctx: &str, raw: &str) -> String {
    if raw.is_empty() {
        return String::new();
    }
    if let Some(inner) = raw.strip_prefix('\'') {
        let body = inner.strip_suffix('\'').unwrap_or_else(|| {
            fail(ctx, &format!("unbalanced single quotes: {raw:?}"))
        });
        return body.replace("''", "'");
    }
    if let Some(inner) = raw.strip_prefix('"') {
        let body = inner.strip_suffix('"').unwrap_or_else(|| {
            fail(ctx, &format!("unbalanced double quotes: {raw:?}"))
        });
        let mut out = String::with_capacity(body.len());
        let mut it = body.chars();
        while let Some(c) = it.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            match it.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some('"') => out.push('"'),
                Some('0') => out.push('\0'),
                Some(other) => fail(ctx, &format!("unbekannter Escape \\{other} in {raw:?}")),
                None => fail(ctx, &format!("trailing backslash in {raw:?}")),
            }
        }
        return out;
    }
    if raw.starts_with(['|', '>', '*', '&', '!', '[', '{', '@', '`']) {
        fail(ctx, &format!("nur flache Skalare erlaubt: {raw:?}"));
    }
    if raw.starts_with('#') {
        return String::new(); // comment
    }
    if raw.contains(" #") || raw.contains("\t#") {
        fail(ctx, &format!("Wert mit Kommentar braucht Quotes: {raw:?}"));
    }
    match raw {
        "null" | "Null" | "NULL" | "~" => String::new(),
        _ => raw.to_string(),
    }
}

/// Collapse all whitespace runs to single spaces (mirrors Python `\s+` → " ").
fn collapse(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A `## `-level heading (like regex `^## \S`) ends the Timeline section.
fn is_h2(line: &str) -> bool {
    line.starts_with("## ") && line[3..].starts_with(|c: char| !c.is_whitespace())
}

/// Extract `notes` from the `## Timeline` section: `- ` bullets (wrapped
/// continuation lines re-attached), joined with ` | `, cut at 2000 chars.
fn timeline_notes(body: &str) -> String {
    let mut in_tl = false;
    let mut bullets: Vec<String> = Vec::new();
    for line in body.lines() {
        if !in_tl {
            if line.trim_end() == "## Timeline" {
                in_tl = true;
            }
            continue;
        }
        if is_h2(line) {
            break;
        }
        let s = line.trim();
        if let Some(b) = s.strip_prefix("- ") {
            bullets.push(b.trim().to_string());
        } else if s.starts_with('#') || s.is_empty() {
            continue;
        } else if let Some(last) = bullets.last_mut() {
            last.push(' ');
            last.push_str(s);
        }
        // else: stray prose before the first bullet — ignore (old compiler did too).
    }
    let joined = bullets
        .iter()
        .map(|b| collapse(b))
        .collect::<Vec<_>>()
        .join(" | ");
    joined.chars().take(2000).collect()
}

/// Parse one dossier into an ordered (key → value) row for JSON emission.
fn parse_dossier(path: &Path, text: &str, dir_state: &str) -> Vec<(String, String)> {
    let name = path.display().to_string();
    let body_start = text.strip_prefix("---\n").unwrap_or_else(|| {
        fail(&name, "muss mit Frontmatter '---' beginnen")
    });
    let end = body_start.find("\n---\n").unwrap_or_else(|| {
        fail(&name, "Frontmatter-Ende ('---') fehlt")
    });
    let (front, rest) = body_start.split_at(end);
    let body = &rest["\n---\n".len()..];

    // YAML folding: indented continuation lines join the pending entry.
    let mut logical: Vec<(usize, String)> = Vec::new();
    for (i, raw_line) in front.lines().enumerate() {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with([' ', '\t']) {
            let cur = logical.last_mut().unwrap_or_else(|| {
                fail(&name, "Fortsetzungszeile ohne Key")
            });
            cur.1.push(' ');
            cur.1.push_str(line.trim());
        } else if is_key_line(line) {
            logical.push((i + 1, line.trim_end().to_string()));
        } else {
            fail(&name, &format!("kein Key und keine Fortsetzung: {line:?}"));
        }
    }

    let mut fm: BTreeMap<String, String> = BTreeMap::new();
    for (lineno, line) in &logical {
        let (k, v) = parse_kv(&name, *lineno, line);
        if fm.insert(k.clone(), v).is_some() {
            fail(&name, &format!("doppelter Key {k:?}"));
        }
    }
    let get = |k: &str| fm.get(k).cloned().unwrap_or_default();
    let slug = get("slug");
    if slug.is_empty() {
        fail(&name, "slug fehlt/leer");
    }
    let state = {
        let s = get("state");
        if s.is_empty() {
            dir_state.to_uppercase()
        } else {
            s
        }
    };
    let notes = timeline_notes(body);

    // Emission order = old seed key order (provenance nested).
    let prov = |suffix: &str| get(&format!("provenance_{suffix}"));
    let mut row = Vec::with_capacity(18);
    for k in [
        "slug", "name", "trader_type", "description", "street", "postcode", "phone", "email",
        "opening_hours", "city",
    ] {
        row.push((k.to_string(), get(k)));
    }
    row.push(("state".to_string(), state));
    for k in [
        "website",
        "website_status",
        "dropoff_json",
        "pickup_json",
        "status",
    ] {
        row.push((k.to_string(), get(k)));
    }
    row.push(("notes".to_string(), notes));
    row.push((
        "provenance".to_string(),
        format!(
            "{{\"seed_file\":{},\"section\":{},\"ankauf_raw\":{},\"origin\":{}}}",
            json_str(&prov("seed_file")),
            json_str(&prov("section")),
            json_str(&prov("ankauf_raw")),
            json_str(&prov("origin")),
        ),
    ));
    row
}

/// Minimal JSON string escaper (inputs are plain strings only; non-ASCII raw).
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main() {
    let pkg = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let dossiers = pkg.join("../../dossiers");
    if !dossiers.is_dir() {
        panic!(
            "seed codegen: dossiers-Verzeichnis fehlt: {} (Build braucht das Repo)",
            dossiers.display()
        );
    }
    // Re-run when any dossier is added/removed/modified.
    println!("cargo:rerun-if-changed={}", dossiers.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("seed_traders");
    fs::create_dir_all(&out_dir).expect("OUT_DIR/seed_traders anlegen");

    for st in STATES {
        let dir = dossiers.join(st);
        let mut files: Vec<PathBuf> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("seed codegen: dossiers/{st} unreadable: {e}"))
            .filter_map(|e| {
                let p = e.expect("dir entry").path();
                (p.extension().and_then(|x| x.to_str()) == Some("md")).then_some(p)
            })
            .collect();
        if files.is_empty() {
            panic!("seed codegen: keine Dossiers in dossiers/{st}");
        }
        files.sort();
        // Per-file watch (directory mtime alone misses content edits on some setups).
        for f in &files {
            println!("cargo:rerun-if-changed={}", f.display());
        }

        let mut out = String::from("[\n");
        for (i, f) in files.iter().enumerate() {
            let text = fs::read_to_string(f)
                .unwrap_or_else(|e| panic!("seed codegen: {} unreadable: {e}", f.display()));
            let row = parse_dossier(f, &text, st);
            out.push_str(" {\n");
            for (j, (k, v)) in row.iter().enumerate() {
                let comma = if j + 1 == row.len() { "" } else { "," };
                if k == "provenance" {
                    out.push_str(&format!("  {k:?}: {v}{comma}\n"));
                } else {
                    out.push_str(&format!("  {k:?}: {}{comma}\n", json_str(v)));
                }
            }
            out.push_str(if i + 1 == files.len() { " }\n" } else { " },\n" });
        }
        out.push_str("]\n");
        fs::write(out_dir.join(format!("{st}.json")), out).expect("seed json schreiben");
    }
    println!("cargo:warning=seed codegen ok (dossiers -> $OUT_DIR/seed_traders)");
}
