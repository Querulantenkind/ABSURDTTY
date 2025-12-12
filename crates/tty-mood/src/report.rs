//! Mood signature report generation.
//!
//! Creates the JSON report that noise consumes.

use crate::signals::SignalCollection;
use absurd_core::Chaos;
use absurd_lexicon::moods::Mood;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use time::OffsetDateTime;

/// The complete mood signature report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodSignature {
    /// Schema identifier for versioning
    pub schema: String,
    /// Unique case identifier
    pub case_id: String,
    /// When this report was generated
    #[serde(with = "time::serde::rfc3339")]
    pub generated_at: OffsetDateTime,
    /// Time range analyzed
    pub range: String,
    /// Source information
    pub source: SourceInfo,
    /// Detected mood
    pub mood: MoodInfo,
    /// Detected signals
    pub signals: Vec<SignalInfo>,
    /// Human-readable notes
    pub notes: Vec<String>,
}

/// Information about the data source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    /// Shell type (zsh, bash, etc.)
    pub shell: String,
    /// Path to history file
    pub history_path: String,
    /// Whether read was read-only
    pub read_only: bool,
    /// Number of entries analyzed
    pub entries_analyzed: usize,
}

/// Mood information for the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodInfo {
    /// Mood identifier
    pub id: String,
    /// Human-readable label
    pub label: String,
    /// Confidence score
    pub confidence: f64,
}

/// Signal information for the report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalInfo {
    /// Signal identifier
    pub id: String,
    /// Signal score
    pub score: f64,
}

impl MoodSignature {
    /// Create a new mood signature report.
    pub fn new(
        mood: &Mood,
        signals: &SignalCollection,
        source: SourceInfo,
        range: &str,
        seed: Option<u64>,
    ) -> Self {
        let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());

        // Generate case ID
        let mut chaos = Chaos::from_optional_seed(seed);
        let date = now.date();
        let date_str = format!(
            "{}{:02}{:02}",
            date.year(),
            date.month() as u8,
            date.day()
        );
        let case_id = chaos.case_id(&date_str);

        // Convert signals
        let signal_infos: Vec<SignalInfo> = signals
            .signals
            .iter()
            .filter(|s| s.score >= 0.3) // Only include significant signals
            .map(|s| SignalInfo {
                id: s.id.clone(),
                score: (s.score * 100.0).round() / 100.0, // Round to 2 decimals
            })
            .collect();

        // Collect notes
        let mut notes: Vec<String> = mood.notes.clone();

        // Add mood description
        notes.push(format!("status: {}", mood.id.description()));

        Self {
            schema: "absurdtty.mood.v1".to_string(),
            case_id,
            generated_at: now,
            range: range.to_string(),
            source,
            mood: MoodInfo {
                id: format!("{:?}", mood.id).to_lowercase(),
                label: mood.label().to_string(),
                confidence: (mood.confidence * 100.0).round() / 100.0,
            },
            signals: signal_infos,
            notes,
        }
    }

    /// Write the signature to a file.
    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        absurd_core::fs_safety::write_json_atomic(path, self)
    }

    /// Render as formatted JSON string.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Render as human-readable summary.
    pub fn to_summary(&self) -> String {
        use absurd_core::format::{BoxBuilder, BoxStyle, Stamp, Table};

        let mut output = String::new();

        // Header box
        output.push_str(&BoxBuilder::new()
            .style(BoxStyle::Double)
            .title("MOOD SIGNATURE REPORT")
            .line(format!("Case: {}", self.case_id))
            .line(format!("Generated: {}", self.generated_at.date()))
            .build());

        output.push('\n');

        // Mood info
        output.push_str(&Table::new()
            .row("MOOD", &self.mood.label)
            .row("CONFIDENCE", format!("{:.0}%", self.mood.confidence * 100.0))
            .row("RANGE", &self.range)
            .row("ENTRIES", self.source.entries_analyzed.to_string())
            .build());

        output.push('\n');

        // Signals
        if !self.signals.is_empty() {
            output.push_str("SIGNALS DETECTED:\n");
            for signal in &self.signals {
                let bar_len = (signal.score * 20.0) as usize;
                let bar = "#".repeat(bar_len);
                let empty = "-".repeat(20 - bar_len);
                output.push_str(&format!(
                    "  {:24} [{}{}] {:.0}%\n",
                    signal.id,
                    bar,
                    empty,
                    signal.score * 100.0
                ));
            }
            output.push('\n');
        }

        // Notes
        if !self.notes.is_empty() {
            output.push_str("NOTES:\n");
            for note in &self.notes {
                output.push_str(&format!("  - {}\n", note));
            }
            output.push('\n');
        }

        // Stamp
        output.push_str(&Stamp::Certified.inline());
        output.push('\n');

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signals::Signal;
    use absurd_lexicon::moods::MoodId;

    #[test]
    fn signature_serializes() {
        let mood = Mood::new(MoodId::FeralProductivity, 0.8);
        let mut signals = SignalCollection::new();
        signals.add(Signal::new("cadence_high", 0.9));

        let source = SourceInfo {
            shell: "zsh".to_string(),
            history_path: "/home/user/.zsh_history".to_string(),
            read_only: true,
            entries_analyzed: 100,
        };

        let sig = MoodSignature::new(&mood, &signals, source, "7d", Some(42));
        let json = sig.to_json().unwrap();

        assert!(json.contains("absurdtty.mood.v1"));
        assert!(json.contains("feral"));
        assert!(json.contains("cadence_high"));
    }

    #[test]
    fn summary_renders() {
        let mood = Mood::new(MoodId::Exhausted, 0.7);
        let signals = SignalCollection::new();

        let source = SourceInfo {
            shell: "zsh".to_string(),
            history_path: "/test".to_string(),
            read_only: true,
            entries_analyzed: 50,
        };

        let sig = MoodSignature::new(&mood, &signals, source, "7d", None);
        let summary = sig.to_summary();

        assert!(summary.contains("MOOD SIGNATURE REPORT"));
        assert!(summary.contains("exhausted"));
    }
}

