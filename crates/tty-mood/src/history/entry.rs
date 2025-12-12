//! History entry representation.
//!
//! A single command from shell history with metadata.

use time::OffsetDateTime;

/// A single entry from shell history.
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// The command that was executed (without arguments for privacy)
    pub command: String,
    /// Full command line (if available, for internal analysis only)
    pub full_line: String,
    /// When the command was executed
    pub timestamp: Option<OffsetDateTime>,
    /// Duration in seconds (if available)
    pub duration: Option<u64>,
    /// Line number in history file (for debugging)
    #[allow(dead_code)]
    pub line_number: usize,
}

impl HistoryEntry {
    /// Create a new history entry.
    pub fn new(command: String, timestamp: Option<OffsetDateTime>, line_number: usize) -> Self {
        Self {
            command,
            full_line: String::new(),
            timestamp,
            duration: None,
            line_number,
        }
    }

    /// Create an entry with full command line preserved.
    pub fn with_full_line(mut self, full_line: String) -> Self {
        self.full_line = full_line;
        self
    }

    /// Extract just the command name (first word).
    pub fn command_name(&self) -> &str {
        self.command
            .split_whitespace()
            .next()
            .unwrap_or(&self.command)
    }

    /// Check if this looks like a typo (very short, uncommon command).
    pub fn looks_like_typo(&self) -> bool {
        let cmd = self.command_name();
        // Common typos are short and don't match known commands
        cmd.len() <= 3 && !Self::is_known_short_command(cmd)
    }

    /// Check if command is a known short command.
    fn is_known_short_command(cmd: &str) -> bool {
        matches!(
            cmd,
            "ls" | "cd" | "cp" | "mv" | "rm" | "cat" | "man" | "top"
                | "ps" | "df" | "du" | "ln" | "vi" | "fg" | "bg" | "id"
                | "wc" | "nl" | "od" | "tr" | "xd" | "bc" | "dc" | "go"
                | "oc" | "gh"
        )
    }

    /// Get hour of day (0-23) if timestamp available.
    pub fn hour(&self) -> Option<u8> {
        self.timestamp.map(|ts| ts.hour())
    }

    /// Check if this was executed during late night (22:00 - 04:00).
    pub fn is_late_night(&self) -> bool {
        self.hour().map(|h| !(4..22).contains(&h)).unwrap_or(false)
    }

    /// Check if this was executed during early morning (05:00 - 07:00).
    pub fn is_early_morning(&self) -> bool {
        self.hour().map(|h| (5..=7).contains(&h)).unwrap_or(false)
    }

    /// Check if this was executed during lunch time (12:00 - 13:00).
    pub fn is_lunch_time(&self) -> bool {
        self.hour().map(|h| (12..=13).contains(&h)).unwrap_or(false)
    }

    /// Get day of week (1=Monday, 7=Sunday) if timestamp available.
    pub fn weekday(&self) -> Option<u8> {
        self.timestamp.map(|ts| ts.weekday().number_from_monday())
    }

    /// Check if this was on a weekend.
    pub fn is_weekend(&self) -> bool {
        self.weekday().map(|d| d >= 6).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_name_extracts_first_word() {
        let entry = HistoryEntry::new("git status".into(), None, 1);
        assert_eq!(entry.command_name(), "git");
    }

    #[test]
    fn known_short_commands_not_typos() {
        let entry = HistoryEntry::new("ls".into(), None, 1);
        assert!(!entry.looks_like_typo());
    }

    #[test]
    fn unknown_short_commands_are_typos() {
        let entry = HistoryEntry::new("gti".into(), None, 1);
        assert!(entry.looks_like_typo());
    }
}

