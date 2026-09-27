//! The trader handlers. One file per trader on purpose:
//! price pages are all shaped differently, so parsing is never shared —
//! only HTTP (`fetch_text`) and number/date helpers from `super`.
//! Each handler owns its URL, selectors and label→material mapping;
//! unknown labels are reported via `skipped_labels`, never guessed.

pub mod antikart;
pub mod asn_norderstedt;
pub mod bruno_welz;
pub mod degussa_frankfurt;
pub mod degussa_hamburg;
pub mod dsh;
pub mod edelcat;
pub mod esh;
pub mod fairkat;
pub mod gold_richtig;
pub mod goldankauf_boerse;
pub mod goldhaus_brb;
pub mod goldschanze;
pub mod goldtrans;
pub mod gutzmann;
pub mod hansa_goldankauf;
pub mod hensel_recycling;
pub mod kalkmann;
pub mod katalysator_hai;
pub mod kupferhelden;
pub mod lausitz;
pub mod metallankauf24;
pub mod mkr_rothenbuecher;
pub mod moroder_scheideanstalt;
pub mod nordkat;
pub mod philoro_berlin_leipziger;
pub mod philoro_berlin_stresemann;
pub mod philoro_bremen;
pub mod philoro_frankfurt;
pub mod philoro_hamburg;
pub mod plum;
pub mod quell;
pub mod rheinische_berlin_kudamm;
pub mod rheinische_berlin_mitte;
pub mod rheinische_bremen;
pub mod rheinische_kaiserslautern;
pub mod rheinische_saarbruecken;
pub mod rheinische_trier;
pub mod schrottabholung_top;
pub mod schrottabholung_zentrale;
pub mod smr;
pub mod suitner;
pub mod tappe;
pub mod vedder;
pub mod vhm_hartmetall;

use super::Handler;

/// All live handlers. Adding another trader means adding one file here
/// plus one line below — nothing else changes.
pub fn all() -> Vec<Handler> {
    vec![
        vedder::handler(),
        lausitz::handler(),
        tappe::handler(),
        kupferhelden::handler(),
        metallankauf24::handler(),
        esh::handler(),
        quell::handler(),
        kalkmann::handler(),
        dsh::handler(),
        plum::handler(),
        nordkat::handler(),
        edelcat::handler(),
        fairkat::handler(),
        katalysator_hai::handler(),
        moroder_scheideanstalt::handler(),
        mkr_rothenbuecher::handler(),
        smr::handler(),
        suitner::handler(),
        gutzmann::handler(),
        antikart::handler(),
        asn_norderstedt::handler(),
        bruno_welz::handler(),
        hansa_goldankauf::handler(),
        hensel_recycling::handler(),
        goldankauf_boerse::handler(),
        gold_richtig::handler(),
        goldhaus_brb::handler(),
        goldschanze::handler(),
        goldtrans::handler(),
        schrottabholung_top::handler(),
        schrottabholung_zentrale::handler(),
        vhm_hartmetall::handler(),
        degussa_frankfurt::handler(),
        degussa_hamburg::handler(),
        rheinische_saarbruecken::handler(),
        rheinische_berlin_mitte::handler(),
        rheinische_berlin_kudamm::handler(),
        rheinische_bremen::handler(),
        rheinische_trier::handler(),
        rheinische_kaiserslautern::handler(),
        philoro_bremen::handler(),
        philoro_berlin_stresemann::handler(),
        philoro_berlin_leipziger::handler(),
        philoro_frankfurt::handler(),
        philoro_hamburg::handler(),
    ]
}
