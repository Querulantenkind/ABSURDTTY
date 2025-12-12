//! Shell history parsing for tty-mood.
//!
//! This module provides parsers for different shell history formats:
//! - ZSH (extended format with timestamps)
//! - Bash (simple format, timestamps via HISTTIMEFORMAT)
//!
//! All parsing is read-only. We never modify history files.

pub mod entry;
pub mod zsh;

pub use entry::HistoryEntry;
pub use zsh::ZshHistoryParser;

use anyhow::Result;
use std::path::Path;
use time::OffsetDateTime;

/// Trait for shell history parsers.
pub trait HistoryParser {
    /// Parse history from a file path.
    fn parse_file(&self, path: &Path) -> Result<Vec<HistoryEntry>>;

    /// Parse history from a string (for testing).
    #[allow(dead_code)]
    fn parse_str(&self, content: &str) -> Result<Vec<HistoryEntry>>;
}

/// Filter entries by time range.
pub fn filter_by_range(
    entries: Vec<HistoryEntry>,
    since: Option<OffsetDateTime>,
    until: Option<OffsetDateTime>,
) -> Vec<HistoryEntry> {
    entries
        .into_iter()
        .filter(|e| {
            let Some(ts) = e.timestamp else {
                // Keep entries without timestamps (can't filter them)
                return true;
            };
            let after_since = since.map(|s| ts >= s).unwrap_or(true);
            let before_until = until.map(|u| ts <= u).unwrap_or(true);
            after_since && before_until
        })
        .collect()
}

/// Calculate time range for "last N days".
pub fn last_n_days(days: u32) -> (OffsetDateTime, OffsetDateTime) {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let duration = time::Duration::days(days as i64);
    let since = now - duration;
    (since, now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_keeps_entries_without_timestamps() {
        let entries = vec![
            HistoryEntry::new("cmd1".into(), None, 1),
            HistoryEntry::new("cmd2".into(), None, 2),
        ];

        let filtered = filter_by_range(entries, Some(OffsetDateTime::now_utc()), None);
        assert_eq!(filtered.len(), 2);
    }
}

