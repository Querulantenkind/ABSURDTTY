//! Signal detection for mood analysis.
//!
//! Signals are patterns detected in shell history that contribute
//! to mood classification. Each signal has an ID and a score (0.0 - 1.0).

pub mod frequency;
pub mod temporal;
pub mod errors;
pub mod diversity;

pub use frequency::FrequencySignals;
pub use temporal::TemporalSignals;
pub use errors::ErrorSignals;
pub use diversity::DiversitySignals;

use crate::history::HistoryEntry;
use serde::{Deserialize, Serialize};

/// A detected signal with score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// Signal identifier (e.g., "cadence_high", "late_night_orbit")
    pub id: String,
    /// Signal score (0.0 - 1.0)
    pub score: f64,
    /// Optional human-readable note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Signal {
    /// Create a new signal.
    pub fn new(id: impl Into<String>, score: f64) -> Self {
        Self {
            id: id.into(),
            score: score.clamp(0.0, 1.0),
            note: None,
        }
    }

    /// Add a note to the signal.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Check if this is a strong signal (>= 0.7).
    pub fn is_strong(&self) -> bool {
        self.score >= 0.7
    }

    /// Check if this is a weak signal (< 0.3).
    #[allow(dead_code)]
    pub fn is_weak(&self) -> bool {
        self.score < 0.3
    }
}

/// Collection of all detected signals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalCollection {
    pub signals: Vec<Signal>,
}

impl SignalCollection {
    /// Create a new empty collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a signal to the collection.
    pub fn add(&mut self, signal: Signal) {
        self.signals.push(signal);
    }

    /// Add a signal only if score is above threshold.
    #[allow(dead_code)]
    pub fn add_if_significant(&mut self, signal: Signal, threshold: f64) {
        if signal.score >= threshold {
            self.signals.push(signal);
        }
    }

    /// Get a signal by ID.
    pub fn get(&self, id: &str) -> Option<&Signal> {
        self.signals.iter().find(|s| s.id == id)
    }

    /// Get score for a signal, defaulting to 0.0 if not found.
    pub fn score(&self, id: &str) -> f64 {
        self.get(id).map(|s| s.score).unwrap_or(0.0)
    }

    /// Get all strong signals.
    pub fn strong_signals(&self) -> Vec<&Signal> {
        self.signals.iter().filter(|s| s.is_strong()).collect()
    }

    /// Merge another collection into this one.
    pub fn merge(&mut self, other: SignalCollection) {
        self.signals.extend(other.signals);
    }
}

/// Analyze history entries and detect all signals.
pub fn analyze(entries: &[HistoryEntry]) -> SignalCollection {
    let mut signals = SignalCollection::new();

    if entries.is_empty() {
        return signals;
    }

    // Frequency signals
    signals.merge(FrequencySignals::analyze(entries));

    // Temporal signals
    signals.merge(TemporalSignals::analyze(entries));

    // Error signals
    signals.merge(ErrorSignals::analyze(entries));

    // Diversity signals
    signals.merge(DiversitySignals::analyze(entries));

    signals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_score_clamped() {
        let signal = Signal::new("test", 1.5);
        assert_eq!(signal.score, 1.0);

        let signal = Signal::new("test", -0.5);
        assert_eq!(signal.score, 0.0);
    }

    #[test]
    fn collection_get_returns_score() {
        let mut collection = SignalCollection::new();
        collection.add(Signal::new("test", 0.8));

        assert_eq!(collection.score("test"), 0.8);
        assert_eq!(collection.score("missing"), 0.0);
    }
}

