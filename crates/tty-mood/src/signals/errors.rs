//! Error and typo signal detection.
//!
//! Analyzes patterns that suggest mistakes, corrections, and uncertainty.

use super::{Signal, SignalCollection};
use crate::history::HistoryEntry;
use std::collections::HashMap;

/// Error signal analyzer.
pub struct ErrorSignals;

impl ErrorSignals {
    /// Analyze error patterns in history entries.
    pub fn analyze(entries: &[HistoryEntry]) -> SignalCollection {
        let mut signals = SignalCollection::new();

        if entries.is_empty() {
            return signals;
        }

        // Typo rate analysis
        let typo_count = entries.iter().filter(|e| e.looks_like_typo()).count();
        let typo_rate = typo_count as f64 / entries.len() as f64;

        if typo_rate > 0.1 {
            signals.add(
                Signal::new("typo_rate_high", (typo_rate * 5.0).min(1.0))
                    .with_note(format!("{}% possible typos", (typo_rate * 100.0) as u32)),
            );
        } else if typo_rate > 0.03 {
            signals.add(Signal::new("typo_rate_medium", typo_rate * 10.0));
        } else if entries.len() > 50 {
            signals.add(Signal::new("typo_rate_low", 1.0 - typo_rate * 10.0));
        }

        // Repeated commands analysis
        let repeat_score = Self::detect_repeats(entries);
        if repeat_score > 0.2 {
            signals.add(
                Signal::new("repeat_commands", repeat_score)
                    .with_note("Same commands executed multiple times"),
            );
        }

        // Correction pattern (command followed by similar command)
        let correction_score = Self::detect_corrections(entries);
        if correction_score > 0.1 {
            signals.add(Signal::new("correction_pattern", correction_score));
        }

        // Status check repetition (git status, ls, etc.)
        let status_check_score = Self::detect_status_checks(entries);
        if status_check_score > 0.3 {
            signals.add(
                Signal::new("status_check_loop", status_check_score)
                    .with_note("Frequent status verification"),
            );
        }

        signals
    }

    /// Detect repeated consecutive commands.
    fn detect_repeats(entries: &[HistoryEntry]) -> f64 {
        if entries.len() < 2 {
            return 0.0;
        }

        let mut repeat_count = 0;

        for window in entries.windows(2) {
            if window[0].command == window[1].command {
                repeat_count += 1;
            }
        }

        repeat_count as f64 / (entries.len() - 1) as f64
    }

    /// Detect correction patterns (typo followed by correct command).
    fn detect_corrections(entries: &[HistoryEntry]) -> f64 {
        if entries.len() < 2 {
            return 0.0;
        }

        let mut correction_count = 0;

        for window in entries.windows(2) {
            let cmd1 = &window[0].command;
            let cmd2 = &window[1].command;

            // Check if commands are similar (likely correction)
            if Self::is_likely_correction(cmd1, cmd2) {
                correction_count += 1;
            }
        }

        correction_count as f64 / (entries.len() - 1) as f64
    }

    /// Check if cmd2 looks like a correction of cmd1.
    fn is_likely_correction(cmd1: &str, cmd2: &str) -> bool {
        // Same length, differ by 1-2 chars
        let len_diff = (cmd1.len() as i32 - cmd2.len() as i32).abs();
        if len_diff > 2 {
            return false;
        }

        // Calculate simple edit distance
        let distance = Self::simple_distance(cmd1, cmd2);

        // Correction if very similar but not identical
        distance > 0 && distance <= 2
    }

    /// Simple character-based distance (not full Levenshtein).
    fn simple_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        if a_chars.len() != b_chars.len() {
            return a_chars.len().abs_diff(b_chars.len());
        }

        a_chars
            .iter()
            .zip(b_chars.iter())
            .filter(|(a, b)| a != b)
            .count()
    }

    /// Detect status-checking behavior (repeated ls, git status, etc.)
    fn detect_status_checks(entries: &[HistoryEntry]) -> f64 {
        let status_commands = ["ls", "git", "pwd", "cat", "head", "tail", "stat", "file"];

        let mut status_count = 0;
        let mut command_counts: HashMap<&str, usize> = HashMap::new();

        for entry in entries {
            let cmd = entry.command_name();
            if status_commands.contains(&cmd) {
                status_count += 1;
                *command_counts.entry(cmd).or_insert(0) += 1;
            }
        }

        // High ratio of status commands suggests uncertainty
        let ratio = status_count as f64 / entries.len() as f64;

        // Bonus for many repetitions of same status command
        let max_single = command_counts.values().max().copied().unwrap_or(0);
        let repetition_bonus = if max_single > 5 {
            (max_single as f64 / entries.len() as f64) * 0.5
        } else {
            0.0
        };

        (ratio + repetition_bonus).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typo_detection() {
        let entries: Vec<_> = ["gti", "sl", "cta", "git", "ls", "cat"]
            .iter()
            .enumerate()
            .map(|(i, cmd)| HistoryEntry::new(cmd.to_string(), None, i + 1))
            .collect();

        let signals = ErrorSignals::analyze(&entries);
        // 3 typos out of 6 = 50%
        assert!(signals.score("typo_rate_high") > 0.5);
    }

    #[test]
    fn repeat_detection() {
        let entries: Vec<_> = ["ls", "ls", "ls", "cd", "ls", "ls"]
            .iter()
            .enumerate()
            .map(|(i, cmd)| HistoryEntry::new(cmd.to_string(), None, i + 1))
            .collect();

        let signals = ErrorSignals::analyze(&entries);
        assert!(signals.score("repeat_commands") > 0.5);
    }

    #[test]
    fn status_check_loop() {
        let entries: Vec<_> = ["git", "ls", "git", "ls", "git", "pwd", "git"]
            .iter()
            .enumerate()
            .map(|(i, cmd)| HistoryEntry::new(cmd.to_string(), None, i + 1))
            .collect();

        let signals = ErrorSignals::analyze(&entries);
        assert!(signals.score("status_check_loop") > 0.5);
    }
}

