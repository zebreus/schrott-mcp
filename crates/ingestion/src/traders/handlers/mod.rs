//! The trader handlers. One file per trader on purpose:
//! price pages are all shaped differently, so parsing is never shared —
//! only HTTP (`fetch_text`) and number/date helpers from `super`.
//! Each handler owns its URL, selectors and label→material mapping;
//! unknown labels are reported via `skipped_labels`, never guessed.

pub mod albus_leipzig;
pub mod amr_schrottplatz;
pub mod antikart;
pub mod asn_norderstedt;
pub mod boehner_altmetalle;
pub mod bruno_welz;
pub mod buntmetallhandel_altmittweida;
pub mod db_recycling;
pub mod degussa_frankfurt;
pub mod degussa_hamburg;
pub mod doering;
pub mod dsh;
pub mod edelcat;
pub mod esh;
pub mod fairkat;
pub mod geld_fuer_gold;
pub mod gold_richtig;
pub mod goldankauf_boerse;
pub mod goldankauf_boerse_leipzig;
pub mod goldhaus_brb;
pub mod goldschanze;
pub mod goldtrans;
pub mod gouchev;
pub mod gutzmann;
pub mod hafen_schrott;
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
pub mod lungwitz;
pub mod madi;
pub mod mc_schrott;
pub mod metallankauf24;
pub mod metallhandel_jacob;
pub mod metcera;
pub mod mis_buntmetallhandel;
pub mod mk_wertstoffhandel;
pub mod mkm_metals;
pub mod mkr_rothenbuecher;
pub mod mobschrott;
pub mod moroder_scheideanstalt;
pub mod ms_recycling_frankfurt;
pub mod ne_metalle;
pub mod nes_scheideanstalt;
pub mod neuwert;
pub mod nordkat;
pub mod oder_metalle;
pub mod on_schrott;
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
pub mod scheideanstalt_ka;
pub mod schiefer_co;
pub mod schrott_anton;
pub mod schrott_frankfurt;
pub mod schrott_recycle_meikel;
pub mod schrott_triebsch;
pub mod schrottabholung_top;
pub mod schrottabholung_zentrale;
pub mod second_way;
pub mod smr;
pub mod suitner;
pub mod tappe;
pub mod vana;
pub mod vedder;
pub mod vhm_hartmetall;
pub mod wertstoff_bauer;

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
        albus_leipzig::handler(),
        amr_schrottplatz::handler(),
        antikart::handler(),
        asn_norderstedt::handler(),
        boehner_altmetalle::handler(),
        bruno_welz::handler(),
        buntmetallhandel_altmittweida::handler(),
        db_recycling::handler(),
        degussa_frankfurt::handler(),
        degussa_hamburg::handler(),
        doering::handler(),
        dsh::handler(),
        edelcat::handler(),
        fairkat::handler(),
        geld_fuer_gold::handler(),
        gold_richtig::handler(),
        goldankauf_boerse::handler(),
        goldankauf_boerse_leipzig::handler(),
        goldhaus_brb::handler(),
        goldschanze::handler(),
        goldtrans::handler(),
        gouchev::handler(),
        gutzmann::handler(),
        hafen_schrott::handler(),
        hansa_goldankauf::handler(),
        harbi_kats::handler(),
        hein_schrotthandel::handler(),
        hensel_recycling::handler(),
        kalkmann::handler(),
        katalysator_hai::handler(),
        kiro::handler(),
        koppe_strausberg::handler(),
        lungwitz::handler(),
        madi::handler(),
        mc_schrott::handler(),
        metallhandel_jacob::handler(),
        metcera::handler(),
        mis_buntmetallhandel::handler(),
        mk_wertstoffhandel::handler(),
        mkm_metals::handler(),
        mkr_rothenbuecher::handler(),
        mobschrott::handler(),
        moroder_scheideanstalt::handler(),
        ms_recycling_frankfurt::handler(),
        ne_metalle::handler(),
        nes_scheideanstalt::handler(),
        neuwert::handler(),
        nordkat::handler(),
        oder_metalle::handler(),
        on_schrott::handler(),
        papierfritze::handler(),
        philoro_berlin_leipziger::handler(),
        philoro_berlin_stresemann::handler(),
        philoro_bremen::handler(),
        philoro_frankfurt::handler(),
        philoro_hamburg::handler(),
        plum::handler(),
        rheinische_berlin_kudamm::handler(),
        rheinische_berlin_mitte::handler(),
        rheinische_bremen::handler(),
        rheinische_kaiserslautern::handler(),
        rheinische_saarbruecken::handler(),
        rheinische_trier::handler(),
        scheideanstalt_ka::handler(),
        schiefer_co::handler(),
        schrott_anton::handler(),
        schrott_frankfurt::handler(),
        schrott_recycle_meikel::handler(),
        schrott_triebsch::handler(),
        schrottabholung_top::handler(),
        schrottabholung_zentrale::handler(),
        second_way::handler(),
        smr::handler(),
        suitner::handler(),
        vana::handler(),
        vhm_hartmetall::handler(),
        wertstoff_bauer::handler(),

    ]
}
