//! Core utilities for ABSURDTTY.
//!
//! This crate provides shared functionality used by both `tty-mood` and `noise`:
//!
//! - [`seed`] - Deterministic randomness for reproducible chaos
//! - [`format`] - Box-drawing, stamps, and bureaucratic formatting
//! - [`fs_safety`] - Safe file operations with XDG compliance
//!
//! # Philosophy
//!
//! All utilities here embody ABSURDTTY's core principles:
//! - Local-first (no network, no telemetry)
//! - Deterministic when seeded
//! - Aesthetically bureaucratic

pub mod seed;
pub mod format;
pub mod fs_safety;

// Re-export commonly used types
pub use seed::Chaos;
pub use format::{BoxBuilder, BoxStyle, Stamp, Table};
pub use fs_safety::Paths;
