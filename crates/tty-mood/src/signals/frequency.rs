//! Frequency-based signal detection.
//!
//! Analyzes command cadence and patterns over time.

use super::{Signal, SignalCollection};
use crate::history::HistoryEntry;

/// Frequency signal analyzer.
pub struct FrequencySignals;

impl FrequencySignals {
    /// Analyze frequency patterns in history entries.
    pub fn analyze(entries: &[HistoryEntry]) -> SignalCollection {
        let mut signals = SignalCollection::new();

        if entries.is_empty() {
            return signals;
        }

        // Calculate commands per hour
        let cph = Self::commands_per_hour(entries);

        // High cadence: > 30 commands/hour on average
        if cph > 30.0 {
            let score = ((cph - 30.0) / 50.0).clamp(0.0, 1.0);
            signals.add(
                Signal::new("cadence_high", score)
                    .with_note(format!("{:.1} commands/hour", cph)),
            );
        }

        // Low cadence: < 5 commands/hour
        if cph < 5.0 && cph > 0.0 {
            let score = 1.0 - (cph / 5.0);
            signals.add(
                Signal::new("cadence_low", score)
                    .with_note(format!("{:.1} commands/hour", cph)),
            );
        }

        // Detect burst patterns (many commands in short time spans)
        let burst_score = Self::detect_bursts(entries);
        if burst_score > 0.3 {
            signals.add(Signal::new("burst_pattern", burst_score));
        }

        // Detect steady rhythm (consistent intervals)
        let rhythm_score = Self::detect_steady_rhythm(entries);
        if rhythm_score > 0.5 {
            signals.add(Signal::new("steady_rhythm", rhythm_score));
        }

        signals
    }

    /// Calculate average commands per hour.
    fn commands_per_hour(entries: &[HistoryEntry]) -> f64 {
        let timestamps: Vec<_> = entries.iter().filter_map(|e| e.timestamp).collect();

        if timestamps.len() < 2 {
            return 0.0;
        }

        let first = timestamps.iter().min().unwrap();
        let last = timestamps.iter().max().unwrap();

        let duration_hours = (*last - *first).as_seconds_f64() / 3600.0;

        if duration_hours < 0.1 {
            return 0.0;
        }

        timestamps.len() as f64 / duration_hours
    }

    /// Detect burst patterns (clusters of rapid commands).
    fn detect_bursts(entries: &[HistoryEntry]) -> f64 {
        let mut timestamps: Vec<_> = entries.iter().filter_map(|e| e.timestamp).collect();
        timestamps.sort();

        if timestamps.len() < 3 {
            return 0.0;
        }

        // Count intervals < 10 seconds (burst indicator)
        let mut burst_count = 0;
        let mut total_intervals = 0;

        for window in timestamps.windows(2) {
            let interval = (window[1] - window[0]).as_seconds_f64();
            total_intervals += 1;

            if interval < 10.0 {
                burst_count += 1;
            }
        }

        if total_intervals == 0 {
            return 0.0;
        }

        burst_count as f64 / total_intervals as f64
    }

    /// Detect steady rhythm (consistent intervals between commands).
    fn detect_steady_rhythm(entries: &[HistoryEntry]) -> f64 {
        let mut timestamps: Vec<_> = entries.iter().filter_map(|e| e.timestamp).collect();
        timestamps.sort();

        if timestamps.len() < 5 {
            return 0.0;
        }

        // Calculate intervals
        let intervals: Vec<f64> = timestamps
            .windows(2)
            .map(|w| (w[1] - w[0]).as_seconds_f64())
            .filter(|&i| i > 0.0 && i < 3600.0) // Ignore gaps > 1 hour
            .collect();

        if intervals.len() < 3 {
            return 0.0;
        }

        // Calculate coefficient of variation (lower = more steady)
        let mean = intervals.iter().sum::<f64>() / intervals.len() as f64;
        let variance = intervals.iter().map(|i| (i - mean).powi(2)).sum::<f64>() / intervals.len() as f64;
        let std_dev = variance.sqrt();
        let cv = std_dev / mean;

        // Convert CV to score (low CV = high rhythm score)
        // CV of 0.5 = medium rhythm, CV of 0.2 = high rhythm
        (1.0 - cv.min(1.0)).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Duration, OffsetDateTime};

    fn make_entries_with_interval(count: usize, interval_secs: i64) -> Vec<HistoryEntry> {
        let start = OffsetDateTime::now_utc();
        (0..count)
            .map(|i| {
                let ts = start + Duration::seconds(interval_secs * i as i64);
                HistoryEntry::new(format!("cmd{}", i), Some(ts), i + 1)
            })
            .collect()
    }

    #[test]
    fn high_cadence_detected() {
        // 100 commands in ~30 minutes = ~200/hour
        let entries = make_entries_with_interval(100, 18);
        let signals = FrequencySignals::analyze(&entries);

        assert!(signals.score("cadence_high") > 0.5);
    }

    #[test]
    fn low_cadence_detected() {
        // 5 commands in 2 hours = 2.5/hour
        let entries = make_entries_with_interval(5, 1440); // 24 min apart
        let signals = FrequencySignals::analyze(&entries);

        assert!(signals.score("cadence_low") > 0.3);
    }

    #[test]
    fn steady_rhythm_detected() {
        // Commands every 30 seconds (very steady)
        let entries = make_entries_with_interval(20, 30);
        let signals = FrequencySignals::analyze(&entries);

        assert!(signals.score("steady_rhythm") > 0.7);
    }
}

