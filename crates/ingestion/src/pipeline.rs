//! The pipeline: seed the material catalog, then fan out to scrapers.
//!
//! Right now only the static material catalog is seeded — the
//! Händler-scrapers that will fill traders/prices come later. The run
//! bookkeeping (runs/steps in the internal database) already works, so the
//! dashboard and scheduler behave the same before and after.

use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use schrott_mcp_store::{InternalDb, PublicDb};
use tokio::task::JoinHandle;

use super::seed_traders::seed_traders;

/// The static price catalog: (slug, German name, category, unit, description).
/// Seeded into `materials` on every run so the catalog exists even before
/// the first scraper lands a price.
const MATERIAL_CATALOG: &[(&str, &str, &str, &str, &str)] = &[
    (
        "stahlschrott-sorte-1",
        "Stahlschrott Sorte 1 (Neuschrott)",
        "eisen",
        "EUR/t",
        "Sauberer Neu- und Stanzschrott aus der Verarbeitung.",
    ),
    (
        "stahlschrott-scheren",
        "Stahlschrott Scherenschrott (Altschrott)",
        "eisen",
        "EUR/t",
        "Zerkleinerter Altschrott, scherengerecht aufbereitet.",
    ),
    (
        "stahlschrott-shredder",
        "Stahlschrott Shreddervormaterial",
        "eisen",
        "EUR/t",
        "Geschredderter Stahlschrott aus Altautos und Geräten.",
    ),
    (
        "eisenschrott-gussbruch",
        "Eisenschrott / Gussbruch",
        "eisen",
        "EUR/t",
        "Gussschrott aus Maschinen- und Ofenbruch.",
    ),
    (
        "mischschrott",
        "Mischschrott",
        "eisen",
        "EUR/t",
        "Gemischter Eisen- und Stahlschrott ohne Aufbereitung.",
    ),
    (
        "kupfer-millberry",
        "Kupfer Millberry (blank)",
        "nichteisen",
        "EUR/kg",
        "Blanker, unbeschichteter Kupferdraht ab 1 mm.",
    ),
    (
        "kupfer-gemischt",
        "Kupfer gemischt",
        "nichteisen",
        "EUR/kg",
        "Gemischte Kupferschrotte ohne Sortenreinheit (Raff, schwer, Nr. 2 …).",
    ),
    (
        "kupfer-berry",
        "Kupfer Berry (beschichtet)",
        "nichteisen",
        "EUR/kg",
        "Beschichteter oder lackierter Kupferdraht.",
    ),
    (
        "messing",
        "Messing",
        "nichteisen",
        "EUR/kg",
        "Messing aus Armaturen, Schrauben und Drehspänen.",
    ),
    (
        "bronze-rotguss",
        "Bronze / Rotguss",
        "nichteisen",
        "EUR/kg",
        "Bronze und Rotguss aus Lagern und Armaturen.",
    ),
    (
        "aluminium-profile",
        "Aluminium Profile (blank)",
        "nichteisen",
        "EUR/kg",
        "Blanke Aluminiumprofile ohne Anhaftungen.",
    ),
    (
        "aluminium-gemischt",
        "Aluminium gemischt",
        "nichteisen",
        "EUR/kg",
        "Gemischte Aluminiumschrotte ohne Sortenreinheit.",
    ),
    (
        "aluminium-guss",
        "Aluminium Guss",
        "nichteisen",
        "EUR/kg",
        "Aluminiumguss aus Motoren und Gehäusen.",
    ),
    (
        "aluminium-blech",
        "Aluminium Blech",
        "nichteisen",
        "EUR/kg",
        "Alublech und -folien, ggf. lackiert.",
    ),
    (
        "zink",
        "Zink",
        "nichteisen",
        "EUR/kg",
        "Zinkblech, Dachrinnen und Titanzink-Verschnitt.",
    ),
    (
        "blei",
        "Blei",
        "nichteisen",
        "EUR/kg",
        "Weichblei aus Rohren, Blechen und Auswuchtgewichten.",
    ),
    (
        "zinn",
        "Zinn / Lötzinn",
        "nichteisen",
        "EUR/kg",
        "Reinzinn und Lötzinn aus Elektronik und Handwerk.",
    ),
    (
        "edelstahl-v2a",
        "Edelstahl V2A (1.4301)",
        "edelstahl",
        "EUR/kg",
        "Nickelhaltiger Edelstahlschrott, magnetisch prüfbar.",
    ),
    (
        "edelstahl-gemischt",
        "Edelstahl gemischt",
        "edelstahl",
        "EUR/kg",
        "Gemischte Edelstahlschrotte ohne Sortenreinheit.",
    ),
    (
        "edelstahl-v4a",
        "Edelstahl V4A (1.4401/1.4571)",
        "edelstahl",
        "EUR/kg",
        "Molybdänhaltiger Edelstahlschrott aus Chemie und Meerestechnik.",
    ),
    (
        "kabel-kupfer",
        "Kabelschrott Kupfer (isoliert)",
        "kabel",
        "EUR/kg",
        "Isolierte Kupferkabel, Preis nach Kupferanteil.",
    ),
    (
        "kabel-alu",
        "Kabelschrott Aluminium (isoliert)",
        "kabel",
        "EUR/kg",
        "Isolierte Aluminiumkabel, Preis nach Aluanteil.",
    ),
    (
        "elektromotoren",
        "Elektromotoren",
        "elektronik",
        "EUR/kg",
        "Ausgebaute E-Motoren aus Geräten und Anlagen.",
    ),
    (
        "platinen",
        "Platinen / Leiterplatten",
        "elektronik",
        "EUR/kg",
        "Bestückte und unbestückte Leiterplatten.",
    ),
    (
        "katalysatoren",
        "Katalysatoren (Keramik)",
        "elektronik",
        "EUR/Stk",
        "Keramik-Katalysatoren aus dem Kfz-Bereich, Preis je Stück.",
    ),
    (
        "hartmetall",
        "Hartmetall / VHM / Widia",
        "nichteisen",
        "EUR/kg",
        "Hartmetallschrott aus Werkzeugen, Wendeschneidplatten und Fräsern.",
    ),
    (
        "gold",
        "Goldlegierungen",
        "edelmetall",
        "EUR/g",
        "Goldlegierungen nach Feingehalt (Variante = Feingehalt), Ankauf je Gramm.",
    ),
    (
        "zahngold",
        "Zahngold / Dentalgold",
        "edelmetall",
        "EUR/g",
        "Dentalgold aus Praxen und Laboren, Ankauf je Gramm.",
    ),
    (
        "silber",
        "Silber",
        "edelmetall",
        "EUR/g",
        "Silberlegierungen und Silberwaren, Ankauf je Gramm.",
    ),
    (
        "platin",
        "Platin",
        "edelmetall",
        "EUR/g",
        "Platinlegierungen, Ankauf je Gramm.",
    ),
    (
        "palladium",
        "Palladium",
        "edelmetall",
        "EUR/g",
        "Palladiumlegierungen, Ankauf je Gramm.",
    ),
];

/// Outcome of one pipeline run.
#[derive(Debug, Clone, Default)]
pub struct IngestSummary {
    /// Total items written across all scrapers.
    pub upserted: i64,
    /// Items whose write failed (should stay zero; surfaced, not swallowed).
    pub write_failures: i64,
    /// Scrapers that failed, with reasons.
    pub failed: Vec<String>,
    /// Canary trips (data recorded, human should check the scraper).
    pub warnings: Vec<String>,
}

/// Guard so the scheduler and manual triggers never run concurrently.
static RUNNING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Make sure every catalog material exists in the public database.
pub fn seed_metadata(public: &PublicDb) -> Result<(), super::IngestError> {
    use super::IngestError;
    use schrott_mcp_store::NewMaterial;
    let now = Utc::now().to_rfc3339();
    for (slug, name_de, category, unit, description) in MATERIAL_CATALOG {
        public
            .upsert_material(&NewMaterial {
                slug,
                name_de,
                category,
                unit,
                description,
                updated_at: &now,
            })
            .map_err(|source| IngestError::Catalog {
                what: "material",
                name: (*slug).to_owned(),
                source,
            })?;
    }
    // Trader seed (embedded JSON, idempotent via payload hashes).
    let wrote = seed_traders(public, &now)?;
    tracing::info!("ingestion: trader seed up to date ({wrote} rows written)");
    Ok(())
}

/// Run the whole pipeline once: seed the catalog, run due trader handlers.
///
/// `force` runs every handler regardless of schedule (manual trigger);
/// otherwise only due handlers run (background loop).
pub async fn run_once(
    internal: &InternalDb,
    public: &PublicDb,
    client: &reqwest::Client,
    force: bool,
) -> IngestSummary {
    use std::sync::atomic::Ordering;
    if RUNNING.swap(true, Ordering::SeqCst) {
        tracing::warn!("ingestion: previous run still active, skipping overlap");
        return IngestSummary {
            failed: vec!["previous run still active".to_owned()],
            ..IngestSummary::default()
        };
    }
    let summary = run_once_inner(internal, public, client, force).await;
    RUNNING.store(false, Ordering::SeqCst);
    summary
}

async fn run_once_inner(
    internal: &InternalDb,
    public: &PublicDb,
    client: &reqwest::Client,
    force: bool,
) -> IngestSummary {
    let run_started = std::time::Instant::now();
    let started = Utc::now().to_rfc3339();
    let run_id = match internal.create_run(&started) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("ingestion: cannot open run: {e}");
            return IngestSummary::default();
        }
    };
    if let Err(e) = seed_metadata(public) {
        tracing::warn!("ingestion: seeding catalog failed: {e}");
    }

    let mut summary = IngestSummary::default();
    let handlers = super::traders::handlers::all();
    let (recorded, failed, warnings) =
        super::traders::scheduler::run_due_with(&handlers, internal, public, client, run_id, force)
            .await;
    summary.upserted = recorded;
    summary.failed = failed;
    summary.warnings = warnings;

    let status = if summary.write_failures > 0 || !summary.failed.is_empty() {
        "partial"
    } else if !summary.warnings.is_empty() {
        "warning"
    } else {
        "ok"
    };
    let mut detail = format!(
        "Katalog aktuell, {} Preise übernommen, {} Händler-Fehler",
        summary.upserted,
        summary.failed.len()
    );
    if !summary.warnings.is_empty() {
        detail.push_str(&format!(", {} Warnungen", summary.warnings.len()));
    }
    if let Err(e) = internal.finish_run(run_id, status, &detail, &Utc::now().to_rfc3339()) {
        tracing::warn!("ingestion: cannot close run {run_id}: {e}");
    }
    tracing::info!(
        "ingestion run {run_id} finished in {}ms: {detail}",
        run_started.elapsed().as_millis()
    );
    summary
}

/// Background scheduler: ticks often, runs what's due.
/// Trader handlers stagger themselves across their 6 h cadence, so the
/// tick only needs to be finer than the smallest schedule (15 min).
/// Runs inside the same process as the web + MCP server.
pub fn spawn_scheduler(
    internal: Arc<InternalDb>,
    public: Arc<PublicDb>,
    interval_secs: u64,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let client = reqwest::Client::builder()
            .user_agent("schrott-mcp-ingestion/0.1")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("ingestion http client builds");
        // Let the server finish booting before the first run.
        tokio::time::sleep(Duration::from_secs(15)).await;
        loop {
            run_once(&internal, &public, &client, false).await;
            tokio::time::sleep(Duration::from_secs(interval_secs)).await;
        }
    })
}
