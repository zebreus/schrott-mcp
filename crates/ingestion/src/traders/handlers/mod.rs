//! The trader handlers. One file per trader on purpose:
//! price pages are all shaped differently, so parsing is never shared —
//! only HTTP (`fetch_text`) and number/date helpers from `super`.
//! Each handler owns its URL, selectors and label→material mapping;
//! unknown labels are reported via `skipped_labels`, never guessed.

pub mod a_z_recycling_muenster;
pub mod albus_leipzig;
pub mod allgaeu_zinn;
pub mod altmetalle_kraft;
pub mod amr_schrottplatz;
pub mod antikart;
pub mod asn_norderstedt;
pub mod bio_goldankauf;
pub mod boehner_altmetalle;
pub mod bruno_welz;
pub mod buntmetallhandel_altmittweida;
pub mod cederbaum;
pub mod db_recycling;
pub mod degussa_frankfurt;
pub mod degussa_hamburg;
pub mod doering;
pub mod dsh;
pub mod eas_recycling_herdecke;
pub mod easygold24_fellbach;
pub mod edelcat;
pub mod esh;
pub mod fairkat;
pub mod frisch_recycling;
pub mod geld_fuer_gold;
pub mod gold_richtig;
pub mod goldankauf_boerse;
pub mod goldankauf_boerse_leipzig;
pub mod goldhandelshaus_bremen;
pub mod goldhaus_brb;
pub mod goldschanze;
pub mod goldtrans;
pub mod gouchev;
pub mod gerwischer;
pub mod gutzmann;
pub mod hafen_schrott;
pub mod hammer_leipzig;
pub mod hansa_goldankauf;
pub mod sommer_hanau;
pub mod hanusa_vechelde;
pub mod harbi_kats;
pub mod hein_schrotthandel;
pub mod hendrichs_krefeld;
pub mod henken_friesoythe;
pub mod hensel_recycling;
pub mod hofmann_metall;
pub mod huth_aschaffenburg;
pub mod kalkmann;
pub mod katalysator_hai;
pub mod kiro;
pub mod klix_recycling_pattensen;
pub mod koppe_strausberg;
pub mod kulisch_co;
pub mod kupferhelden;
pub mod lausitz;
pub mod lungwitz;
pub mod madi;
pub mod mc_schrott;
pub mod metallankauf24;
pub mod metallhandel_jacob;
pub mod metallorum;
pub mod metcera;
pub mod mis_buntmetallhandel;
pub mod mk_wertstoffhandel;
pub mod mkm_metals;
pub mod mkr_rothenbuecher;
pub mod mobschrott;
pub mod moelter_kronach;
pub mod moroder_scheideanstalt;
pub mod ms_recycling_frankfurt;
pub mod msg_metallrecycling_gotha;
pub mod ne_metalle;
pub mod nes_scheideanstalt;
pub mod nfh_zickura_saterland;
pub mod neuwert;
pub mod nordkat;
pub mod oder_metalle;
pub mod on_schrott;
pub mod ophirum;
pub mod oschatz_recycling;
pub mod papierfritze;
pub mod philoro_berlin_leipziger;
pub mod philoro_berlin_stresemann;
pub mod philoro_bremen;
pub mod philoro_frankfurt;
pub mod philoro_hamburg;
pub mod plum;
pub mod pur_umwelt_bonn;
pub mod quell;
pub mod rheinische_berlin_kudamm;
pub mod rheinische_berlin_mitte;
pub mod rheinische_bremen;
pub mod rheinische_hamburg_frankfurt_wiesbaden;
pub mod rheinische_kaiserslautern;
pub mod rheinische_paderborn;
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
pub mod schrottmobil_oelsnitz;
pub mod schrottplatz_roetgesbuettel;
pub mod second_way;
pub mod smr;
pub mod springer_sohn_oldenburg;
pub mod suitner;
pub mod tappe;
pub mod vana;
pub mod vedder;
pub mod vedder_delmenhorst;
pub mod vhm_hartmetall;
pub mod wahl_co_zella_mehlis;
pub mod wertstoff_bauer;
pub mod westarp;
pub mod wirkaufendeingold;
pub mod wkr_koln;

use super::Handler;

/// All live handlers. Adding another trader means adding one file here
/// plus one line below — nothing else changes.
pub fn all() -> Vec<Handler> {
    vec![
        a_z_recycling_muenster::handler(),
vedder::handler(),
        lausitz::handler(),
        tappe::handler(),
        kupferhelden::handler(),
        metallankauf24::handler(),
        esh::handler(),
        quell::handler(),
        albus_leipzig::handler(),
        allgaeu_zinn::handler(),
        altmetalle_kraft::handler(),
        amr_schrottplatz::handler(),
        antikart::handler(),
        asn_norderstedt::handler(),
        bio_goldankauf::handler(),
        boehner_altmetalle::handler(),
        bruno_welz::handler(),
        buntmetallhandel_altmittweida::handler(),
        cederbaum::handler(),
        db_recycling::handler(),
        degussa_frankfurt::handler(),
        degussa_hamburg::handler(),
        doering::handler(),
        dsh::handler(),
        eas_recycling_herdecke::handler(),
        easygold24_fellbach::handler(),
        edelcat::handler(),
        fairkat::handler(),
        frisch_recycling::handler(),
        geld_fuer_gold::handler(),
        gold_richtig::handler(),
        goldankauf_boerse::handler(),
        goldankauf_boerse_leipzig::handler(),
        goldhandelshaus_bremen::handler(),
        goldhaus_brb::handler(),
        goldschanze::handler(),
        goldtrans::handler(),
        gouchev::handler(),
        gerwischer::handler(),
        gutzmann::handler(),
        hafen_schrott::handler(),
        hammer_leipzig::handler(),
        hansa_goldankauf::handler(),
        sommer_hanau::handler(),
        hanusa_vechelde::handler(), // TEMP-VERIFY
        harbi_kats::handler(),
        hein_schrotthandel::handler(),
        hendrichs_krefeld::handler(),
        henken_friesoythe::handler(),
        hensel_recycling::handler(),
        hofmann_metall::handler_chemnitz(),
        hofmann_metall::handler_zwickau(),
        huth_aschaffenburg::handler(),
        kalkmann::handler(),
        katalysator_hai::handler(),
        kiro::handler(),
        klix_recycling_pattensen::handler(),
        koppe_strausberg::handler(),
        kulisch_co::handler(),
        lungwitz::handler(),
        madi::handler(),
        madi::handler_rosengarten(),
        mc_schrott::handler(),
        metallhandel_jacob::handler(),
        metallorum::handler(),
        metcera::handler(),
        mis_buntmetallhandel::handler(),
        mk_wertstoffhandel::handler(),
        mkm_metals::handler(),
        mkr_rothenbuecher::handler(),
        mobschrott::handler(),
        moelter_kronach::handler(),
        moroder_scheideanstalt::handler(),
        ms_recycling_frankfurt::handler(),
        msg_metallrecycling_gotha::handler(),
        ne_metalle::handler(),
        nes_scheideanstalt::handler(),
        nfh_zickura_saterland::handler(),
        neuwert::handler(),
        nordkat::handler(),
        oder_metalle::handler(),
        on_schrott::handler(),
        ophirum::handler_frankfurt(),
        ophirum::handler_bremen(),
        ophirum::handler_hanau(),
        oschatz_recycling::handler(),
        papierfritze::handler(),
        philoro_berlin_leipziger::handler(),
        philoro_berlin_stresemann::handler(),
        philoro_bremen::handler(),
        philoro_frankfurt::handler(),
        philoro_hamburg::handler(),
        plum::handler(),
        pur_umwelt_bonn::handler(),
        rheinische_berlin_kudamm::handler(),
        rheinische_berlin_mitte::handler(),
        rheinische_bremen::handler(),
        rheinische_hamburg_frankfurt_wiesbaden::handler_hamburg(),
        rheinische_hamburg_frankfurt_wiesbaden::handler_frankfurt(),
        rheinische_hamburg_frankfurt_wiesbaden::handler_wiesbaden(),
        rheinische_kaiserslautern::handler(),
        rheinische_paderborn::handler(),
        rheinische_saarbruecken::handler(),
        rheinische_trier::handler(),
        scheideanstalt_ka::handler(),
        schiefer_co::handler(),
        schrott_anton::handler(),
        schrott_frankfurt::handler(),
        schrott_recycle_meikel::handler(),
        schrott_triebsch::handler(),
        schrottmobil_oelsnitz::handler(),
        schrottabholung_top::handler(),
        schrottabholung_zentrale::handler(),
        schrottplatz_roetgesbuettel::handler(),
        second_way::handler(),
        smr::handler(),
        springer_sohn_oldenburg::handler(),
        suitner::handler(),
        vana::handler(),
        vedder_delmenhorst::handler(),
        vhm_hartmetall::handler(),
        wahl_co_zella_mehlis::handler(),
        wertstoff_bauer::handler(),
        westarp::handler(),
        wirkaufendeingold::handler(),
        wkr_koln::handler(),
    ]
}
