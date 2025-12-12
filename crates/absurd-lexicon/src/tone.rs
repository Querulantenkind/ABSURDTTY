//! Tone transformations for mood-adapted output.
//!
//! Each mood has an associated tone that affects how `noise` communicates.
//! Tones modify vocabulary, verbosity, formality, and emotional register.

use crate::moods::MoodId;

/// Tone configuration for output generation.
#[derive(Debug, Clone)]
pub struct Tone {
    /// Verbosity level (0.0 = terse, 1.0 = verbose)
    pub verbosity: f64,
    /// Formality level (0.0 = casual, 1.0 = bureaucratic)
    pub formality: f64,
    /// Chaos level (0.0 = orderly, 1.0 = unpredictable)
    pub chaos: f64,
    /// Energy level (0.0 = lethargic, 1.0 = manic)
    pub energy: f64,
    /// Confidence in assertions (0.0 = uncertain, 1.0 = absolute)
    pub certainty: f64,
}

impl Tone {
    /// Create a new tone with all parameters.
    pub fn new(verbosity: f64, formality: f64, chaos: f64, energy: f64, certainty: f64) -> Self {
        Self {
            verbosity: verbosity.clamp(0.0, 1.0),
            formality: formality.clamp(0.0, 1.0),
            chaos: chaos.clamp(0.0, 1.0),
            energy: energy.clamp(0.0, 1.0),
            certainty: certainty.clamp(0.0, 1.0),
        }
    }

    /// Get the default tone for a mood.
    pub fn for_mood(mood: MoodId) -> Self {
        match mood {
            MoodId::FeralProductivity => Self::new(0.6, 0.3, 0.5, 0.9, 0.4),
            MoodId::Exhausted => Self::new(0.2, 0.5, 0.2, 0.1, 0.3),
            MoodId::Methodical => Self::new(0.8, 0.8, 0.1, 0.5, 0.9),
            MoodId::ChaoticNeutral => Self::new(0.5, 0.3, 0.9, 0.6, 0.3),
            MoodId::BureaucraticZen => Self::new(0.7, 1.0, 0.1, 0.4, 0.5),
            MoodId::AmbientDrift => Self::new(0.4, 0.4, 0.3, 0.3, 0.2),
            MoodId::RecursiveDoubt => Self::new(0.6, 0.5, 0.4, 0.4, 0.1),
            MoodId::EmergencyMode => Self::new(0.3, 0.2, 0.7, 0.8, 0.2),
            MoodId::Neutral => Self::default(),
        }
    }

    /// Check if output should be truncated due to low energy.
    pub fn should_truncate(&self) -> bool {
        self.energy < 0.3
    }

    /// Check if output should include extra detail.
    pub fn should_elaborate(&self) -> bool {
        self.verbosity > 0.7
    }

    /// Check if output should use formal/bureaucratic language.
    pub fn should_be_formal(&self) -> bool {
        self.formality > 0.6
    }

    /// Check if chaos effects should be applied.
    pub fn should_inject_chaos(&self) -> bool {
        self.chaos > 0.5
    }
}

impl Default for Tone {
    fn default() -> Self {
        // Neutral tone: middle of everything
        Self::new(0.5, 0.5, 0.2, 0.5, 0.5)
    }
}

/// Phrases and templates that vary by tone.
pub struct Phrases;

impl Phrases {
    /// Get a status prefix based on tone.
    pub fn status_prefix(tone: &Tone) -> &'static str {
        if tone.formality > 0.8 {
            "OFFICIAL STATUS REPORT:"
        } else if tone.energy < 0.3 {
            "Status:"
        } else if tone.chaos > 0.6 {
            "STATUS (probably):"
        } else {
            "STATUS:"
        }
    }

    /// Get a confidence qualifier.
    pub fn confidence_qualifier(tone: &Tone, confidence: f64) -> &'static str {
        if confidence > 0.9 {
            if tone.certainty > 0.7 {
                "confirmed"
            } else {
                "allegedly confirmed"
            }
        } else if confidence > 0.7 {
            if tone.formality > 0.6 {
                "assessed with reasonable certainty"
            } else {
                "probably"
            }
        } else if confidence > 0.5 {
            if tone.chaos > 0.5 {
                "maybe? who knows"
            } else {
                "tentatively"
            }
        } else if tone.certainty < 0.3 {
            "speculatively at best"
        } else {
            "uncertain"
        }
    }

    /// Get an ending phrase.
    pub fn ending(tone: &Tone) -> &'static str {
        if tone.formality > 0.8 {
            "End of report. No action required."
        } else if tone.energy < 0.3 {
            "[transmission ends]"
        } else if tone.chaos > 0.7 {
            "...or does it?"
        } else {
            ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_for_all_moods() {
        for mood in MoodId::all() {
            let tone = Tone::for_mood(*mood);
            assert!(tone.verbosity >= 0.0 && tone.verbosity <= 1.0);
            assert!(tone.formality >= 0.0 && tone.formality <= 1.0);
        }
    }

    #[test]
    fn exhausted_tone_should_truncate() {
        let tone = Tone::for_mood(MoodId::Exhausted);
        assert!(tone.should_truncate());
    }

    #[test]
    fn methodical_tone_should_elaborate() {
        let tone = Tone::for_mood(MoodId::Methodical);
        assert!(tone.should_elaborate());
    }

    #[test]
    fn bureaucratic_zen_is_formal() {
        let tone = Tone::for_mood(MoodId::BureaucraticZen);
        assert!(tone.should_be_formal());
    }
}

