//! # Alnico
//!
//! A competitor to JUCE for Rust: everything needed to build and ship an audio
//! plugin — DSP, device I/O, plugin formats, and a GUI toolkit — under a
//! permissive licence, with realtime safety enforced by the type system.
//!
//! **This release is a name reservation.** Nothing is implemented yet.
//! Development is tracked at <https://github.com/NotTheDj/alnico-audio>.
//!
//! When it lands, this crate will be a facade re-exporting the workspace behind
//! feature flags (`dsp`, `gui`, `clap`, `vst3`, `au`, …), so the common case is
//! one dependency line and the specialist case is a handful of crates.

#![doc(html_root_url = "https://docs.rs/alnico/0.0.0")]
