//! Shared vocabulary for ABSURDTTY.
//!
//! This crate defines the language of absurdity:
//! - Mood states and their detection criteria
//! - Tone transformations per mood
//! - Bureaucratic phrases and templates

pub mod moods;
pub mod tone;

pub use moods::{Mood, MoodId};
pub use tone::Tone;

