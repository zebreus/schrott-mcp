//! The first five trader handlers. One file per trader on purpose:
//! price pages are all shaped differently, so parsing is never shared —
//! only HTTP (`fetch_text`) and number/unit/date helpers from `super`.
//! Each handler owns its URL, selectors and label→material mapping;
//! unknown labels are reported via `skipped_labels`, never guessed.

pub mod esh;
pub mod kupferhelden;
pub mod lausitz;
pub mod metallankauf24;
pub mod quell;
pub mod tappe;
pub mod vedder;

use super::Handler;

/// All live handlers. Adding trader #8..#500 means adding one file here
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
    ]
}
