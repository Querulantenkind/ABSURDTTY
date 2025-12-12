//! Mood state definitions for ABSURDTTY.
//!
//! A mood is a qualitative assessment of the operator's terminal behavior.
//! Moods are detected by `tty-mood` and consumed by `noise` to adapt output.

use serde::{Deserialize, Serialize};

/// Unique identifier for a mood state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MoodId {
    /// High cadence, high diversity, late-night activity
    FeralProductivity,
    /// Low cadence, high typo rate, repeated commands
    Exhausted,
    /// Steady rhythm, low error rate, systematic patterns
    Methodical,
    /// High diversity, burst patterns, unpredictable timing
    ChaoticNeutral,
    /// Steady patterns, form-like sequences, ritual adherence
    BureaucraticZen,
    /// Many small commands, no clear direction
    AmbientDrift,
    /// Repeated status checks, validation loops
    RecursiveDoubt,
    /// Fast bursts, high error rate, crisis management
    EmergencyMode,
    /// No mood detected or insufficient data
    #[default]
    Neutral,
}

impl MoodId {
    /// Get the human-readable label for this mood.
    pub fn label(&self) -> &'static str {
        match self {
            MoodId::FeralProductivity => "feral productivity",
            MoodId::Exhausted => "exhausted",
            MoodId::Methodical => "methodical",
            MoodId::ChaoticNeutral => "chaotic neutral",
            MoodId::BureaucraticZen => "bureaucratic zen",
            MoodId::AmbientDrift => "ambient drift",
            MoodId::RecursiveDoubt => "recursive doubt",
            MoodId::EmergencyMode => "emergency mode",
            MoodId::Neutral => "neutral",
        }
    }

    /// Get a brief description of this mood.
    pub fn description(&self) -> &'static str {
        match self {
            MoodId::FeralProductivity => {
                "Operator moving faster than reflection allows"
            }
            MoodId::Exhausted => {
                "System functional, operator questionable"
            }
            MoodId::Methodical => {
                "Everything catalogued and verified"
            }
            MoodId::ChaoticNeutral => {
                "Entropy rising but controlled"
            }
            MoodId::BureaucraticZen => {
                "Perfect adherence to ritual without attachment to outcome"
            }
            MoodId::AmbientDrift => {
                "Present but unfocused"
            }
            MoodId::RecursiveDoubt => {
                "Uncertainty loops detected"
            }
            MoodId::EmergencyMode => {
                "Crisis management in progress"
            }
            MoodId::Neutral => {
                "Insufficient data for classification"
            }
        }
    }

    /// Get all mood IDs (excluding Neutral).
    pub fn all() -> &'static [MoodId] {
        &[
            MoodId::FeralProductivity,
            MoodId::Exhausted,
            MoodId::Methodical,
            MoodId::ChaoticNeutral,
            MoodId::BureaucraticZen,
            MoodId::AmbientDrift,
            MoodId::RecursiveDoubt,
            MoodId::EmergencyMode,
        ]
    }
}

impl std::fmt::Display for MoodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}


/// A complete mood assessment with confidence and notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mood {
    /// The detected mood state
    pub id: MoodId,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// Human-readable notes about the detection
    #[serde(default)]
    pub notes: Vec<String>,
}

impl Mood {
    /// Create a new mood with the given ID and confidence.
    pub fn new(id: MoodId, confidence: f64) -> Self {
        Self {
            id,
            confidence: confidence.clamp(0.0, 1.0),
            notes: Vec::new(),
        }
    }

    /// Create a neutral mood (no detection).
    pub fn neutral() -> Self {
        Self::new(MoodId::Neutral, 0.0)
    }

    /// Add a note to the mood.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add multiple notes to the mood.
    pub fn with_notes<I, S>(mut self, notes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.notes.extend(notes.into_iter().map(Into::into));
        self
    }

    /// Check if this is a high-confidence detection.
    pub fn is_confident(&self) -> bool {
        self.confidence >= 0.7
    }

    /// Check if this is a neutral/undetected mood.
    pub fn is_neutral(&self) -> bool {
        self.id == MoodId::Neutral
    }

    /// Get the label for display.
    pub fn label(&self) -> &'static str {
        self.id.label()
    }
}

impl Default for Mood {
    fn default() -> Self {
        Self::neutral()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mood_id_has_label() {
        for mood in MoodId::all() {
            assert!(!mood.label().is_empty());
            assert!(!mood.description().is_empty());
        }
    }

    #[test]
    fn mood_confidence_clamped() {
        let mood = Mood::new(MoodId::Exhausted, 1.5);
        assert_eq!(mood.confidence, 1.0);

        let mood = Mood::new(MoodId::Exhausted, -0.5);
        assert_eq!(mood.confidence, 0.0);
    }

    #[test]
    fn mood_serialization() {
        let mood = Mood::new(MoodId::FeralProductivity, 0.8)
            .with_note("test note");

        let json = serde_json::to_string(&mood).unwrap();
        assert!(json.contains("feral_productivity"));

        let deserialized: Mood = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, MoodId::FeralProductivity);
    }
}

