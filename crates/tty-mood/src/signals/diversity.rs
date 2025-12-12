//! Command diversity signal detection.
//!
//! Analyzes the variety of commands and tools being used.

use super::{Signal, SignalCollection};
use crate::history::HistoryEntry;
use std::collections::{HashMap, HashSet};

/// Diversity signal analyzer.
pub struct DiversitySignals;

impl DiversitySignals {
    /// Analyze command diversity in history entries.
    pub fn analyze(entries: &[HistoryEntry]) -> SignalCollection {
        let mut signals = SignalCollection::new();

        if entries.is_empty() {
            return signals;
        }

        // Count unique commands
        let unique_commands: HashSet<_> = entries.iter().map(|e| e.command_name()).collect();
        let diversity_ratio = unique_commands.len() as f64 / entries.len() as f64;

        // High diversity: many different commands
        if diversity_ratio > 0.5 {
            signals.add(
                Signal::new("command_diversity_high", diversity_ratio)
                    .with_note(format!("{} unique commands", unique_commands.len())),
            );
        } else if diversity_ratio < 0.2 && entries.len() > 20 {
            signals.add(
                Signal::new("command_diversity_low", 1.0 - diversity_ratio)
                    .with_note("Limited command variety"),
            );
        }

        // Tool fixation (one tool dominates)
        let fixation = Self::detect_tool_fixation(entries);
        if let Some((tool, score)) = fixation {
            signals.add(
                Signal::new("tool_fixation", score)
                    .with_note(format!("Focused on: {}", tool)),
            );
        }

        // Context switching (different project directories)
        let context_score = Self::detect_context_switching(entries);
        if context_score > 0.3 {
            signals.add(Signal::new("context_switching", context_score));
        }

        // Tool categories
        let category_signals = Self::analyze_tool_categories(entries);
        for signal in category_signals {
            signals.add(signal);
        }

        signals
    }

    /// Detect if one tool dominates the session.
    fn detect_tool_fixation(entries: &[HistoryEntry]) -> Option<(String, f64)> {
        let mut counts: HashMap<&str, usize> = HashMap::new();

        for entry in entries {
            *counts.entry(entry.command_name()).or_insert(0) += 1;
        }

        let (top_cmd, top_count) = counts.iter().max_by_key(|(_, c)| *c)?;

        let ratio = *top_count as f64 / entries.len() as f64;

        // Fixation if > 40% of commands are the same tool
        if ratio > 0.4 {
            Some((top_cmd.to_string(), ratio))
        } else {
            None
        }
    }

    /// Detect context switching (cd commands to different paths).
    fn detect_context_switching(entries: &[HistoryEntry]) -> f64 {
        let cd_count = entries
            .iter()
            .filter(|e| e.command_name() == "cd")
            .count();

        if entries.len() < 10 {
            return 0.0;
        }

        // High cd ratio suggests context switching
        let cd_ratio = cd_count as f64 / entries.len() as f64;

        (cd_ratio * 5.0).min(1.0)
    }

    /// Analyze which categories of tools are being used.
    fn analyze_tool_categories(entries: &[HistoryEntry]) -> Vec<Signal> {
        let mut signals = Vec::new();

        let git_commands = ["git", "gh", "hub", "tig", "lazygit"];
        let editor_commands = ["vim", "nvim", "nano", "micro", "code", "emacs", "hx"];
        let build_commands = ["cargo", "make", "npm", "yarn", "pnpm", "go", "rustc", "gcc", "cmake"];
        let system_commands = ["systemctl", "journalctl", "dmesg", "htop", "top", "ps", "kill"];
        let package_commands = ["pacman", "apt", "yay", "brew", "dnf", "pip", "cargo"];

        let total = entries.len() as f64;

        // Git usage
        let git_count = entries
            .iter()
            .filter(|e| git_commands.contains(&e.command_name()))
            .count();
        if git_count as f64 / total > 0.15 {
            signals.push(
                Signal::new("git_heavy", (git_count as f64 / total * 3.0).min(1.0))
                    .with_note(format!("{} git operations", git_count)),
            );
        }

        // Editor usage
        let editor_count = entries
            .iter()
            .filter(|e| editor_commands.contains(&e.command_name()))
            .count();
        if editor_count as f64 / total > 0.1 {
            signals.push(Signal::new("editor_focused", (editor_count as f64 / total * 4.0).min(1.0)));
        }

        // Build tools
        let build_count = entries
            .iter()
            .filter(|e| build_commands.contains(&e.command_name()))
            .count();
        if build_count as f64 / total > 0.1 {
            signals.push(Signal::new("build_cycle", (build_count as f64 / total * 4.0).min(1.0)));
        }

        // System administration
        let system_count = entries
            .iter()
            .filter(|e| system_commands.contains(&e.command_name()))
            .count();
        if system_count as f64 / total > 0.1 {
            signals.push(Signal::new("system_admin", (system_count as f64 / total * 4.0).min(1.0)));
        }

        // Package management
        let package_count = entries
            .iter()
            .filter(|e| package_commands.contains(&e.command_name()))
            .count();
        if package_count as f64 / total > 0.1 {
            signals.push(Signal::new("package_operations", (package_count as f64 / total * 4.0).min(1.0)));
        }

        signals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_diversity_detected() {
        let entries: Vec<_> = ["git", "ls", "cd", "cat", "vim", "cargo", "npm", "python"]
            .iter()
            .enumerate()
            .map(|(i, cmd)| HistoryEntry::new(cmd.to_string(), None, i + 1))
            .collect();

        let signals = DiversitySignals::analyze(&entries);
        assert!(signals.score("command_diversity_high") > 0.8);
    }

    #[test]
    fn tool_fixation_detected() {
        let entries: Vec<_> = ["git", "git", "git", "git", "git", "ls", "cd"]
            .iter()
            .enumerate()
            .map(|(i, cmd)| HistoryEntry::new(cmd.to_string(), None, i + 1))
            .collect();

        let signals = DiversitySignals::analyze(&entries);
        assert!(signals.score("tool_fixation") > 0.5);
    }

    #[test]
    fn git_heavy_detected() {
        let entries: Vec<_> = ["git", "git", "git", "ls", "git", "cd", "git"]
            .iter()
            .enumerate()
            .map(|(i, cmd)| HistoryEntry::new(cmd.to_string(), None, i + 1))
            .collect();

        let signals = DiversitySignals::analyze(&entries);
        assert!(signals.score("git_heavy") > 0.5);
    }
}

