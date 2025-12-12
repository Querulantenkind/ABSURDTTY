//! Temporal pattern signal detection.
//!
//! Analyzes when commands are executed (time of day, day of week).

use super::{Signal, SignalCollection};
use crate::history::HistoryEntry;

/// Temporal signal analyzer.
pub struct TemporalSignals;

impl TemporalSignals {
    /// Analyze temporal patterns in history entries.
    pub fn analyze(entries: &[HistoryEntry]) -> SignalCollection {
        let mut signals = SignalCollection::new();

        if entries.is_empty() {
            return signals;
        }

        let with_timestamps: Vec<_> = entries.iter().filter(|e| e.timestamp.is_some()).collect();

        if with_timestamps.is_empty() {
            return signals;
        }

        // Late night orbit (22:00 - 04:00)
        let late_night_count = with_timestamps.iter().filter(|e| e.is_late_night()).count();
        let late_night_ratio = late_night_count as f64 / with_timestamps.len() as f64;

        if late_night_ratio > 0.1 {
            signals.add(
                Signal::new("late_night_orbit", late_night_ratio.min(1.0))
                    .with_note(format!("{}% of commands after 22:00", (late_night_ratio * 100.0) as u32)),
            );
        }

        // Early morning surge (05:00 - 07:00)
        let early_count = with_timestamps.iter().filter(|e| e.is_early_morning()).count();
        let early_ratio = early_count as f64 / with_timestamps.len() as f64;

        if early_ratio > 0.1 {
            signals.add(
                Signal::new("early_morning_surge", early_ratio.min(1.0))
                    .with_note(format!("{}% of commands before 07:00", (early_ratio * 100.0) as u32)),
            );
        }

        // Lunch void (12:00 - 13:00 absence)
        let lunch_count = with_timestamps.iter().filter(|e| e.is_lunch_time()).count();
        let lunch_ratio = lunch_count as f64 / with_timestamps.len() as f64;

        // Low lunch activity suggests taking breaks (healthy!)
        // High lunch activity suggests no breaks
        if lunch_ratio < 0.02 && with_timestamps.len() > 20 {
            signals.add(Signal::new("lunch_void", 0.8));
        }

        // Weekend anomaly
        let weekend_count = with_timestamps.iter().filter(|e| e.is_weekend()).count();
        let weekend_ratio = weekend_count as f64 / with_timestamps.len() as f64;

        // Expected weekend ratio is ~28% (2/7 days)
        // Higher = weekend warrior, Lower = weekday only
        if weekend_ratio > 0.4 {
            signals.add(
                Signal::new("weekend_warrior", (weekend_ratio - 0.28) * 2.0)
                    .with_note("Above-average weekend activity"),
            );
        } else if weekend_ratio < 0.15 && with_timestamps.len() > 50 {
            signals.add(
                Signal::new("weekday_bound", (0.28 - weekend_ratio) * 2.0)
                    .with_note("Below-average weekend activity"),
            );
        }

        // Hour distribution analysis
        let hour_signals = Self::analyze_hour_distribution(&with_timestamps);
        for signal in hour_signals {
            signals.add(signal);
        }

        signals
    }

    /// Analyze distribution of commands across hours.
    fn analyze_hour_distribution(entries: &[&HistoryEntry]) -> Vec<Signal> {
        let mut signals = Vec::new();
        let mut hour_counts = [0u32; 24];

        for entry in entries {
            if let Some(hour) = entry.hour() {
                hour_counts[hour as usize] += 1;
            }
        }

        let total: u32 = hour_counts.iter().sum();
        if total == 0 {
            return signals;
        }

        // Find peak hours
        let max_count = *hour_counts.iter().max().unwrap_or(&0);
        let peak_hours: Vec<usize> = hour_counts
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == max_count && c > 0)
            .map(|(h, _)| h)
            .collect();

        if !peak_hours.is_empty() && max_count as f64 / total as f64 > 0.15 {
            let peak_desc = peak_hours
                .iter()
                .map(|h| format!("{:02}:00", h))
                .collect::<Vec<_>>()
                .join(", ");
            signals.push(
                Signal::new("peak_hours", max_count as f64 / total as f64)
                    .with_note(format!("Most active: {}", peak_desc)),
            );
        }

        // Check for spread vs concentrated activity
        let active_hours = hour_counts.iter().filter(|&&c| c > 0).count();
        let concentration = 1.0 - (active_hours as f64 / 24.0);

        if concentration > 0.6 {
            signals.push(Signal::new("time_concentrated", concentration));
        } else if concentration < 0.3 {
            signals.push(Signal::new("time_spread", 1.0 - concentration));
        }

        signals
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{UtcOffset, Date, Time, Month};

    fn entry_at_hour(hour: u8) -> HistoryEntry {
        let date = Date::from_calendar_date(2025, Month::January, 15).unwrap();
        let time = Time::from_hms(hour, 30, 0).unwrap();
        let dt = date.with_time(time).assume_offset(UtcOffset::UTC);
        HistoryEntry::new("test".into(), Some(dt), 1)
    }

    #[test]
    fn late_night_orbit_detected() {
        let entries: Vec<_> = (0..10)
            .map(|_| entry_at_hour(23))
            .chain((0..5).map(|_| entry_at_hour(14)))
            .collect();

        let signals = TemporalSignals::analyze(&entries);
        assert!(signals.score("late_night_orbit") > 0.5);
    }

    #[test]
    fn no_late_night_when_daytime() {
        let entries: Vec<_> = (0..20).map(|_| entry_at_hour(14)).collect();
        let signals = TemporalSignals::analyze(&entries);
        assert_eq!(signals.score("late_night_orbit"), 0.0);
    }
}

