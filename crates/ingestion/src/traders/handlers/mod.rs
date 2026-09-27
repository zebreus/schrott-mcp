//! The trader handlers. One file per trader on purpose:
//! price pages are all shaped differently, so parsing is never shared —
//! only HTTP (`fetch_text`) and number/date helpers from `super`.
//! Each handler owns its URL, selectors and label→material mapping;
//! unknown labels are reported via `skipped_labels`, never guessed.

pub mod amr_schrottplatz;
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
pub mod gouchev;
pub mod gutzmann;
pub mod hansa_goldankauf;
pub mod harbi_kats;
pub mod hein_schrotthandel;
pub mod hensel_recycling;
pub mod kalkmann;
pub mod katalysator_hai;
pub mod kiro;
pub mod koppe_strausberg;
pub mod kupferhelden;
pub mod lausitz;
pub mod madi;
pub mod mc_schrott;
pub mod metallankauf24;
pub mod mkr_rothenbuecher;
pub mod mk_wertstoffhandel;
pub mod mobschrott;
pub mod moroder_scheideanstalt;
pub mod ne_metalle;
pub mod nes_scheideanstalt;
pub mod neuwert;
pub mod nordkat;
pub mod oder_metalle;
pub mod papierfritze;
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
pub mod schiefer_co;
pub mod scheideanstalt_ka;
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
        koppe_strausberg::handler(),
        moroder_scheideanstalt::handler(),
        mkr_rothenbuecher::handler(),
        amr_schrottplatz::handler(),
        harbi_kats::handler(),
        hein_schrotthandel::handler(),
        kiro::handler(),
        madi::handler(),
        mc_schrott::handler(),
        mk_wertstoffhandel::handler(),
        mobschrott::handler(),
        ne_metalle::handler(),
        nes_scheideanstalt::handler(),
        neuwert::handler(),
        oder_metalle::handler(),
        papierfritze::handler(),
        scheideanstalt_ka::handler(),
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
        gouchev::handler(),
        schrottabholung_top::handler(),
        schrottabholung_zentrale::handler(),
        schiefer_co::handler(),
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
