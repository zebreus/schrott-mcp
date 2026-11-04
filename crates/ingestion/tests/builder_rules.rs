//! Die Testfunktionen des Seed-Builders als **eigenes Testziel**.
//!
//! Cargo fuehrt Build-Skripte nicht als Testziel aus: die `#[cfg(test)]`-Tests
//! in `build.rs` wurden deshalb nie ausgefuehrt - der Seed-Build und die
//! uebrigen Tests haben das nie bemerkt. Mit diesem Target laufen sie.
//!
//! `build.rs` wird als Modul eingebunden; sein `main()` (der den Seed
//! schreibt) wird nicht aufgerufen.

// Der Build-Codegen wird im Test nicht gebraucht - die Warnungen dazu
// (unbenutzte Konstanten, das `main`) sind hier erwartet.
#[allow(dead_code)]
#[path = "../build.rs"]
mod build;
