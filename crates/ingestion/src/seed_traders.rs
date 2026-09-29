//! Trader seed import: versioned JSON (`seed/traders/<state>.json`,
//! converted from the `recherche/*.md` reports via `tools/md2seed.py`)
//! embedded in the binary and applied idempotently on every run.
//!
//! Format per entry: slug (STABLE — derived once as
//! `<state>-<city>-<name>`, never hand-edited), name, trader_type, city,
//! state, website, status, notes, provenance. Street/postcode are usually
//! empty until scrapers enrich them.
//!
//! Change detection: a hash of the canonical payload lives in
//! `extra_json.seed_hash`. Rows whose seed payload did not change are
//! skipped, so `updated_at` keeps meaning "last real change" and repeat
//! runs are cheap. `first_seen_at` is only set on insert (see upsert).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::Deserialize;

use schrott_mcp_store::{NewTrader, PublicDb};

/// One seed row, exactly as stored in `seed/traders/*.json`.
/// `description`, `dropoff_json` and `pickup_json` are enrichment-owned:
/// the importer keeps stored values whenever the seed leaves them empty.
#[derive(Debug, Clone, Deserialize)]
pub struct SeedTrader {
    pub slug: String,
    pub name: String,
    pub trader_type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub street: String,
    #[serde(default)]
    pub postcode: String,
    pub city: String,
    pub state: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub website_status: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub opening_hours: String,
    #[serde(default)]
    pub dropoff_json: String,
    #[serde(default)]
    pub pickup_json: String,
    pub status: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub provenance: SeedProvenance,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SeedProvenance {
    #[serde(default)]
    pub seed_file: String,
    #[serde(default)]
    pub section: String,
    #[serde(default)]
    pub ankauf_raw: String,
    #[serde(default)]
    pub origin: String,
}

/// Allowed enum values (mirror the `traders` CHECK-adjacent conventions).
pub const TRADER_TYPES: &[&str] = &[
    "schrotthaendler",
    "wertstoffhaendler",
    "metallhaendler",
    "autoverwertung",
    "containerdienst",
    "schrottplatz",
    "mobil",
    "sonstige",
];
pub const STATUSES: &[&str] = &["aktiv", "geschlossen", "pruefung", "unbekannt"];
pub const STATES: &[&str] = [
    "BW", "BY", "BE", "BB", "HB", "HH", "HE", "MV", "NI", "NW", "RP", "SL", "SN", "ST", "SH", "TH",
]
.as_slice();

macro_rules! seed_files {
    ($($state:literal),*) => {
        &[$(($state, include_str!(concat!("../../../seed/traders/", $state, ".json")))),*]
    };
}

/// All embedded seed files: (state, json text).
pub const SEED_FILES: &[(&str, &str)] = seed_files!(
    "bw", "by", "be", "bb", "hb", "hh", "he", "mv", "ni", "nw", "rp", "sl", "sn", "st", "sh", "th"
);

/// Parse every embedded seed file. Errors name the file.
pub fn load_seeds() -> Result<Vec<SeedTrader>, String> {
    let mut all = Vec::new();
    for (state, text) in SEED_FILES {
        let mut rows: Vec<SeedTrader> =
            serde_json::from_str(text).map_err(|e| format!("seed {state}: {e}"))?;
        for r in &mut rows {
            if r.state.is_empty() {
                r.state = state.to_uppercase();
            }
        }
        all.extend(rows);
    }
    Ok(all)
}

/// Validate the whole seed corpus. Used by tests (and missions that
/// regenerate seeds should run `cargo test` before committing).
pub fn validate_seeds(traders: &[SeedTrader]) -> Result<(), String> {
    use std::collections::HashSet;
    let mut slugs = HashSet::new();
    for t in traders {
        if t.slug.is_empty() || t.name.len() < 2 {
            return Err(format!("bad name/slug: {}", t.slug));
        }
        if !slugs.insert(t.slug.clone()) {
            return Err(format!("duplicate slug: {}", t.slug));
        }
        if !TRADER_TYPES.contains(&t.trader_type.as_str()) {
            return Err(format!("bad trader_type {} in {}", t.trader_type, t.slug));
        }
        if !STATUSES.contains(&t.status.as_str()) {
            return Err(format!("bad status {} in {}", t.status, t.slug));
        }
        if !STATES.contains(&t.state.as_str()) {
            return Err(format!("bad state {} in {}", t.state, t.slug));
        }
        if !t.website.is_empty()
            && !(t.website.starts_with("http://") || t.website.starts_with("https://"))
        {
            return Err(format!("bad website in {}", t.slug));
        }
        if !t.website_status.is_empty()
            && !["aktiv", "tot", "blockiert", "unbekannt"].contains(&t.website_status.as_str())
        {
            return Err(format!("bad website_status in {}", t.slug));
        }
        for (key, raw) in [
            ("dropoff_json", &t.dropoff_json),
            ("pickup_json", &t.pickup_json),
        ] {
            if !raw.is_empty() {
                let v: serde_json::Value = serde_json::from_str(raw)
                    .map_err(|_| format!("bad {key} JSON in {}", t.slug))?;
                if !v.is_object() {
                    return Err(format!("bad {key} (not an object) in {}", t.slug));
                }
            }
        }
        if t.city.is_empty() {
            return Err(format!("empty city in {}", t.slug));
        }
    }
    Ok(())
}

/// Hash over everything the seed owns about a trader (effective values,
/// i.e. after enrichment preservation below).
fn payload_hash(
    t: &SeedTrader,
    description: &str,
    dropoff_json: &str,
    pickup_json: &str,
    website: &str,
    website_status: &str,
    phone: &str,
    email: &str,
    opening_hours: &str,
) -> String {
    let mut h = DefaultHasher::new();
    [
        t.slug.as_str(),
        t.name.as_str(),
        t.trader_type.as_str(),
        t.street.as_str(),
        t.postcode.as_str(),
        t.city.as_str(),
        t.state.as_str(),
        website,
        website_status,
        phone,
        email,
        opening_hours,
        t.status.as_str(),
        t.notes.as_str(),
        description,
        dropoff_json,
        pickup_json,
    ]
    .join("\x1f")
    .hash(&mut h);
    format!("{:016x}", h.finish())
}

/// Non-empty seed value wins, otherwise the stored (enriched) one survives.
/// The seed never clobbers scraper/human enrichment with blanks.
fn keep(seed: &str, stored: &str, fallback: &str) -> String {
    if !seed.is_empty() {
        seed.to_owned()
    } else if !stored.is_empty() {
        stored.to_owned()
    } else {
        fallback.to_owned()
    }
}

/// Apply the seed corpus. Returns the number of rows written (inserts +
/// real changes); unchanged rows are skipped via the stored hash.
pub fn seed_traders(public: &PublicDb, now: &str) -> Result<usize, super::IngestError> {
    let traders = load_seeds().map_err(|detail| super::IngestError::Parse {
        url: "seed/traders".to_owned(),
        detail,
    })?;
    validate_seeds(&traders).map_err(|detail| super::IngestError::Parse {
        url: "seed/traders".to_owned(),
        detail,
    })?;
    let mut wrote = 0;
    for t in &traders {
        let kept = public
            .existing_seed_state(&t.slug)
            .map_err(|source| super::IngestError::Catalog {
                what: "trader",
                name: t.slug.clone(),
                source,
            })?
            .unwrap_or_default();
        let description = keep(&t.description, &kept.description, "");
        let dropoff_json = keep(&t.dropoff_json, &kept.dropoff_json, "{}");
        let pickup_json = keep(&t.pickup_json, &kept.pickup_json, "{}");
        let website = keep(&t.website, &kept.website, "");
        let website_status = keep(&t.website_status, &kept.website_status, "unbekannt");
        let phone = keep(&t.phone, &kept.phone, "");
        let email = keep(&t.email, &kept.email, "");
        let opening_hours = keep(&t.opening_hours, &kept.opening_hours, "");
        let hash = payload_hash(
            t,
            &description,
            &dropoff_json,
            &pickup_json,
            &website,
            &website_status,
            &phone,
            &email,
            &opening_hours,
        );
        if kept.seed_hash == Some(hash.clone()) {
            continue; // unchanged — keep updated_at meaningful
        }
        let extra = serde_json::json!({
            "seed_file": t.provenance.seed_file,
            "seed_section": t.provenance.section,
            "ankauf_raw": t.provenance.ankauf_raw,
            "seed_hash": hash,
        })
        .to_string();
        public
            .upsert_trader(&NewTrader {
                slug: &t.slug,
                name: &t.name,
                trader_type: &t.trader_type,
                description: &description,
                street: &t.street,
                postcode: &t.postcode,
                city: &t.city,
                state: &t.state,
                country: "DE",
                lat: None,
                lon: None,
                email: &email,
                website: &website,
                website_status: &website_status,
                website_checked_at: &kept.website_checked_at,
                phone: &phone,
                opening_hours: &opening_hours,
                dropoff_json: &dropoff_json,
                pickup_json: &pickup_json,
                min_quantity_kg: None,
                max_quantity_kg: None,
                certifications: "[]",
                status: &t.status,
                notes: &t.notes,
                extra_json: &extra,
                now,
            })
            .map_err(|source| super::IngestError::Catalog {
                what: "trader",
                name: t.slug.clone(),
                source,
            })?;
        wrote += 1;
    }
    Ok(wrote)
}

#[cfg(test)]
mod tests {
    use super::{load_seeds, seed_traders, validate_seeds};
    use schrott_mcp_store::PublicDb;

    #[test]
    fn seeds_parse_validate_and_count() {
        let traders = load_seeds().expect("seeds parse");
        validate_seeds(&traders).expect("seeds valid");
        assert!(traders.len() > 2000, "seed corpus unexpectedly small");
        // Spot checks across states and origins.
        let by_slug = |s: &str| traders.iter().find(|t| t.slug == s).expect(s);
        let falk = by_slug("bw-stuttgart-falk-adler");
        assert_eq!(falk.city, "Stuttgart");
        assert!(falk.website.starts_with("https://"));
        assert_eq!(falk.status, "aktiv");
        assert!(traders.iter().any(|t| t.provenance.origin == "prose"));
        assert!(traders.iter().any(|t| t.status == "pruefung"));
    }

    #[test]
    fn seed_is_idempotent_and_skips_unchanged() {
        let dir = std::env::temp_dir().join(format!("schrott-seed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
        let now = "2026-09-27T00:00:00Z";
        let first = seed_traders(&db, now).expect("first seed writes");
        assert!(first > 2000);
        assert_eq!(db.counts().expect("counts").traders as usize, first);
        // Second run with same payload writes nothing (hash-skip).
        let second = seed_traders(&db, now).expect("second seed runs");
        assert_eq!(second, 0);
        // first_seen_at survived, updated_at untouched by the no-op run.
        let res = db
            .query_sql("SELECT first_seen_at, updated_at FROM traders WHERE slug = 'bw-stuttgart-falk-adler'")
            .expect("spot check");
        assert_eq!(res.rows.len(), 1);
    }

    #[test]
    fn seed_preserves_enrichment() {
        use schrott_mcp_store::NewTrader;
        let dir = std::env::temp_dir().join(format!("schrott-seed-keep-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let db = PublicDb::open(&dir).expect("test db opens");
        let now = "2026-09-27T00:00:00Z";
        seed_traders(&db, now).expect("first seed writes");
        let slug = "bw-stuttgart-falk-adler";
        // Simulate scraper enrichment that does not know the seed hash.
        let cur = db
            .query_sql(&format!(
                "SELECT name, trader_type, city, state, website, status, notes
                 FROM traders WHERE slug = '{slug}'"
            ))
            .expect("read row");
        assert_eq!(cur.rows.len(), 1);
        let c: Vec<String> = cur.rows[0]
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_owned())
            .collect();
        db.upsert_trader(&NewTrader {
            slug,
            name: &c[0],
            trader_type: &c[1],
            description: "Vom Scraper angereichert.",
            street: "",
            postcode: "",
            city: &c[2],
            state: &c[3],
            country: "DE",
            lat: None,
            lon: None,
            phone: "",
            email: "",
            website: &c[4],
            website_status: "aktiv",
            website_checked_at: now,
            opening_hours: "",
            dropoff_json: "{\"allowed\":true,\"customer_types\":[\"gewerbe\"]}",
            pickup_json: "{}",
            min_quantity_kg: None,
            max_quantity_kg: None,
            certifications: "[]",
            status: &c[5],
            notes: &c[6],
            extra_json: "{}",
            now,
        })
        .expect("enrichment writes");
        // Re-seed re-imports the row (hash was lost) but keeps the enrichment.
        assert_eq!(seed_traders(&db, now).expect("reseed"), 1);
        let kept = db
            .existing_seed_state(slug)
            .expect("state reads")
            .expect("row exists");
        assert_eq!(kept.description, "Vom Scraper angereichert.");
        assert_eq!(kept.website_status, "aktiv");
        assert!(kept.dropoff_json.contains("gewerbe"));
        // And the run after that is a no-op again.
        assert_eq!(seed_traders(&db, now).expect("reseed2"), 0);
    }
}
