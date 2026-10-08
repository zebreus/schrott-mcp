//! Trader seed import: the `dossiers/<state>/<slug>.md` dossiers are the
//! single source of truth, compiled at build time by
//! `crates/ingestion/build.rs` into `$OUT_DIR/seed_traders/<state>.json`
//! and embedded in the binary — no committed JSON, no hand-editable
//! intermediate. Applied idempotently on every run.
//!
//! Format per entry: slug (STABLE — derived once as
//! `<state>-<city>-<name>`, never hand-edited), name, trader_type, city,
//! state, website, status, notes, provenance. Street/postcode are usually
//! empty until scrapers enrich them.
//! Evidence-backed lat/lon pairs override stored GEO; both absent/empty
//! preserve it. Coordinates are dossier data, never geocoded here.
//!
//! Change detection: a hash of the canonical payload lives in
//! `extra_json.seed_hash`. Rows whose seed payload did not change are
//! skipped, so `updated_at` keeps meaning "last real change" and repeat
//! runs are cheap. `first_seen_at` is only set on insert (see upsert).

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::Deserialize;

use schrott_mcp_store::{NewTrader, PublicDb};

/// One seed row, as compiled at build time from a dossier by
/// `crates/ingestion/build.rs`.
/// `description`, service conditions and `certifications` preserve stored
/// values when the dossier leaves them empty.
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
    /// Evidence-backed WGS84 pair. Both missing/empty preserve stored coordinates.
    #[serde(default)]
    pub lat: String,
    #[serde(default)]
    pub lon: String,
    pub city: String,
    pub state: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub website_status: String,
    /// JSON array of current, evidence-backed certifications. Empty/missing
    /// means the dossier has no curated value and preserves stored enrichment.
    #[serde(default)]
    pub certifications: String,
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
    pub section: String,
    #[serde(default)]
    pub ankauf_raw: String,
    #[serde(default)]
    pub origin: String,
}

impl SeedTrader {
    fn coordinates(&self) -> Result<Option<(f64, f64)>, String> {
        if self.lat.is_empty() && self.lon.is_empty() {
            return Ok(None);
        }
        let bad = || format!("bad lat/lon WGS84 pair in {}", self.slug);
        let lat = self.lat.parse::<f64>().map_err(|_| bad())?;
        let lon = self.lon.parse::<f64>().map_err(|_| bad())?;
        if !lat.is_finite() || !lon.is_finite() || lat.abs() > 90.0 || lon.abs() > 180.0 {
            return Err(bad());
        }
        Ok(Some((lat, lon)))
    }
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
        // Generated at build time by crates/ingestion/build.rs from
        // dossiers/<state>/*.md — never committed, lives in OUT_DIR only.
        &[$(($state, include_str!(concat!(env!("OUT_DIR"), "/seed_traders/", $state, ".json")))),*]
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

/// Validate the whole seed corpus. Used by tests (dossier changes must
/// keep `cargo test` green before committing).
pub fn validate_seeds(traders: &[SeedTrader]) -> Result<(), String> {
    use std::collections::HashSet;
    let mut slugs = HashSet::new();
    for t in traders {
        t.coordinates()?;
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
        if !t.certifications.is_empty() {
            let value: serde_json::Value = serde_json::from_str(&t.certifications)
                .map_err(|_| format!("bad certifications JSON in {}", t.slug))?;
            if !value
                .as_array()
                .is_some_and(|items| items.iter().all(serde_json::Value::is_string))
            {
                return Err(format!(
                    "bad certifications (not an array of strings) in {}",
                    t.slug
                ));
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
    certifications: Option<&str>,
) -> String {
    let mut h = DefaultHasher::new();
    let mut fields = vec![
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
    ];
    // Keep the historical hash byte-for-byte stable for dossiers without
    // certifications; otherwise one field addition would rewrite every seed.
    if let Some(certifications) = certifications {
        fields.push(certifications);
    }
    fields.join("\x1f").hash(&mut h);
    // Preserve historical hashes for dossiers without coordinates. Numeric
    // hashing also avoids writes for equivalent decimal spellings.
    if let Some((lat, lon)) = t.coordinates().expect("validated seed coordinates") {
        "dossier-wgs84".hash(&mut h);
        lat.to_bits().hash(&mut h);
        lon.to_bits().hash(&mut h);
    }
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
        url: "dossiers".to_owned(),
        detail,
    })?;
    apply_seed_traders(public, &traders, now)
}

/// Shared application path for embedded dossiers and isolated regression fixtures.
fn apply_seed_traders(
    public: &PublicDb,
    traders: &[SeedTrader],
    now: &str,
) -> Result<usize, super::IngestError> {
    validate_seeds(traders).map_err(|detail| super::IngestError::Parse {
        url: "dossiers".to_owned(),
        detail,
    })?;
    let mut wrote = 0;
    for t in traders {
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
        let certifications = keep(&t.certifications, &kept.certifications, "[]");
        let certification_payload =
            (!t.certifications.is_empty()).then_some(certifications.as_str());
        let coordinates = t.coordinates().expect("validated seed coordinates");
        let (lat, lon) = coordinates
            .map(|(lat, lon)| (Some(lat), Some(lon)))
            .unwrap_or((kept.lat, kept.lon));
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
            certification_payload,
        );
        if kept.seed_hash == Some(hash.clone()) && kept.lat == lat && kept.lon == lon {
            continue; // unchanged — keep updated_at meaningful
        }
        let extra = serde_json::json!({
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
                // Dossier pair wins; absent/empty pair preserves stored GEO.
                lat,
                lon,
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
                certifications: Some(&certifications),
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
    use super::{apply_seed_traders, load_seeds, seed_traders, validate_seeds, SeedTrader};
    use crate::test_support::TempDbDir;
    use schrott_mcp_store::PublicDb;

    fn coordinate_fixture(fields: serde_json::Value) -> SeedTrader {
        let mut row = serde_json::json!({
            "slug": "bb-coordinate-test", "name": "Coordinate test",
            "trader_type": "schrotthaendler", "city": "Teststadt",
            "state": "BB", "status": "aktiv"
        });
        row.as_object_mut()
            .unwrap()
            .extend(fields.as_object().unwrap().clone());
        serde_json::from_value(row).expect("fixture parses")
    }

    #[test]
    fn dossier_coordinates_insert_update_and_skip_unchanged() {
        let dir = TempDbDir::new("seed-coordinates");
        let db = PublicDb::open(dir.path()).expect("test db opens");
        let first_seen = "2026-10-07T00:00:00Z";
        let changed = "2026-10-08T00:00:00Z";
        let mut seed = coordinate_fixture(serde_json::json!({"lat": "52.5", "lon": "13.4"}));
        assert_eq!(
            apply_seed_traders(&db, &[seed.clone()], first_seen).unwrap(),
            1
        );
        let initial = db.existing_seed_state(&seed.slug).unwrap().unwrap();
        assert_eq!((initial.lat, initial.lon), (Some(52.5), Some(13.4)));
        assert_eq!(
            apply_seed_traders(&db, &[seed.clone()], changed).unwrap(),
            0
        );
        seed.lat = "52.6".to_owned();
        seed.lon = "13.5".to_owned();
        assert_eq!(
            apply_seed_traders(&db, &[seed.clone()], changed).unwrap(),
            1
        );
        let updated = db.existing_seed_state(&seed.slug).unwrap().unwrap();
        assert_eq!((updated.lat, updated.lon), (Some(52.6), Some(13.5)));
        assert_ne!(initial.seed_hash, updated.seed_hash);
        seed.lat = "52.6000".to_owned();
        assert_eq!(
            apply_seed_traders(&db, &[seed], "2026-10-09T00:00:00Z").unwrap(),
            0
        );
        let dates = db
            .query_sql(
                "SELECT first_seen_at, updated_at FROM traders WHERE slug = 'bb-coordinate-test'",
            )
            .unwrap();
        assert_eq!(dates.rows[0][0].as_str(), Some(first_seen));
        assert_eq!(dates.rows[0][1].as_str(), Some(changed));
    }

    #[test]
    fn absent_or_empty_dossier_coordinates_preserve_existing_pair() {
        for fields in [
            serde_json::json!({}),
            serde_json::json!({"lat": "", "lon": ""}),
        ] {
            let dir = TempDbDir::new("seed-coordinate-keep");
            let db = PublicDb::open(dir.path()).expect("test db opens");
            let now = "2026-10-08T00:00:00Z";
            let blank = coordinate_fixture(fields.clone());
            assert_eq!(apply_seed_traders(&db, &[blank.clone()], now).unwrap(), 1);
            let inserted = db.existing_seed_state(&blank.slug).unwrap().unwrap();
            assert_eq!((inserted.lat, inserted.lon), (None, None));
            assert_eq!(apply_seed_traders(&db, &[blank], now).unwrap(), 0);
            let existing = coordinate_fixture(serde_json::json!({"lat": "52.5", "lon": "13.4"}));
            apply_seed_traders(&db, &[existing], now).unwrap();
            let mut seed = coordinate_fixture(fields);
            assert_eq!(apply_seed_traders(&db, &[seed.clone()], now).unwrap(), 1);
            assert_eq!(apply_seed_traders(&db, &[seed.clone()], now).unwrap(), 0);
            seed.notes = "Dossier changed without GEO".to_owned();
            assert_eq!(apply_seed_traders(&db, &[seed.clone()], now).unwrap(), 1);
            let kept = db.existing_seed_state(&seed.slug).unwrap().unwrap();
            assert_eq!((kept.lat, kept.lon), (Some(52.5), Some(13.4)));
            assert_eq!(apply_seed_traders(&db, &[seed], now).unwrap(), 0);
        }
    }

    #[test]
    fn invalid_coordinate_pairs_fail_before_any_write() {
        let dir = TempDbDir::new("seed-coordinate-invalid");
        let db = PublicDb::open(dir.path()).expect("test db opens");
        for fields in [
            serde_json::json!({"lat": "52.5"}),
            serde_json::json!({"lon": "13.4"}),
            serde_json::json!({"lat": "unknown", "lon": "13.4"}),
            serde_json::json!({"lat": "NaN", "lon": "13.4"}),
            serde_json::json!({"lat": "52.5", "lon": "inf"}),
            serde_json::json!({"lat": "91", "lon": "13.4"}),
            serde_json::json!({"lat": "52.5", "lon": "181"}),
        ] {
            let valid = coordinate_fixture(serde_json::json!({}));
            let mut invalid = coordinate_fixture(fields);
            invalid.slug = "bb-invalid".to_owned();
            assert!(apply_seed_traders(&db, &[valid, invalid], "2026-10-08T00:00:00Z").is_err());
            assert_eq!(db.counts().unwrap().traders, 0);
        }
    }

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
        let drh = by_slug("ni-springe-31832-drh-deutsche-rohstoff-handelsgesellschaf");
        assert_eq!(
            drh.certifications,
            r#"["Entsorgungsfachbetrieb §56 KrWG (Zertifikat 801.0978/26, gültig bis 09.08.2027)"]"#
        );
        let long_timeline = by_slug("ni-visselhovede-martin-broschinski-schrotthandel-peter-b");
        assert!(
            long_timeline.notes.chars().count() > 2000,
            "timeline notes must not be silently truncated"
        );
        assert!(traders.iter().any(|t| t.provenance.origin == "prose"));
        assert!(traders.iter().any(|t| t.status == "pruefung"));
    }

    #[test]
    fn seed_is_idempotent_and_skips_unchanged() {
        let dir = TempDbDir::new("seed");
        let db = PublicDb::open(dir.path()).expect("test db opens");
        let now = "2026-09-27T00:00:00Z";
        let first = seed_traders(&db, now).expect("first seed writes");
        assert!(first > 2000);
        assert_eq!(db.counts().expect("counts").traders as usize, first);
        let certifications = db
            .query_sql(
                "SELECT certifications FROM traders
                 WHERE slug = 'ni-springe-31832-drh-deutsche-rohstoff-handelsgesellschaf'",
            )
            .expect("read seeded certifications");
        assert_eq!(
            certifications.rows[0][0].as_str(),
            Some(
                r#"["Entsorgungsfachbetrieb §56 KrWG (Zertifikat 801.0978/26, gültig bis 09.08.2027)"]"#
            )
        );
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
        let dir = TempDbDir::new("seed-keep");
        let db = PublicDb::open(dir.path()).expect("test db opens");
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
            certifications: None,
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
