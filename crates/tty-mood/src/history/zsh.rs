//! ZSH history parser.
//!
//! Parses ZSH extended history format:
//! `: TIMESTAMP:DURATION;COMMAND`
//!
//! Where:
//! - TIMESTAMP is Unix epoch seconds
//! - DURATION is execution time (often 0)
//! - COMMAND is the actual command (may span multiple lines with backslash)
//!
//! Example:
//! ```text
//! : 1702400000:0;git status
//! : 1702400100:0;cd ~/projects
//! : 1702400200:0;echo "hello\
//! world"
//! ```

use super::{entry::HistoryEntry, HistoryParser};
use anyhow::{Context, Result};
use std::path::Path;
use time::OffsetDateTime;

/// Parser for ZSH extended history format.
#[derive(Debug, Default)]
pub struct ZshHistoryParser {
    /// Whether to preserve full command lines (for internal analysis)
    preserve_full_line: bool,
}

impl ZshHistoryParser {
    /// Create a new ZSH history parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure whether to preserve full command lines.
    #[allow(dead_code)]
    pub fn preserve_full_lines(mut self, preserve: bool) -> Self {
        self.preserve_full_line = preserve;
        self
    }

    /// Parse a single history line in extended format.
    ///
    /// Returns None if the line is not a valid extended format entry.
    fn parse_line(&self, line: &str, line_number: usize) -> Option<HistoryEntry> {
        // Extended format: `: TIMESTAMP:DURATION;COMMAND`
        if !line.starts_with(": ") {
            // Simple format (no timestamp) - just the command
            if !line.is_empty() && !line.starts_with('#') {
                let command = extract_command_name(line);
                return Some(HistoryEntry::new(command, None, line_number));
            }
            return None;
        }

        // Parse extended format
        let rest = &line[2..]; // Skip ": "

        // Find the semicolon that separates metadata from command
        let semicolon_pos = rest.find(';')?;
        let metadata = &rest[..semicolon_pos];
        let full_command = &rest[semicolon_pos + 1..];

        // Parse timestamp and optional duration
        let mut parts = metadata.split(':');
        let timestamp_str = parts.next()?;
        let duration_str = parts.next();

        let timestamp = timestamp_str
            .parse::<i64>()
            .ok()
            .and_then(|ts| OffsetDateTime::from_unix_timestamp(ts).ok());

        let duration = duration_str.and_then(|d| d.parse::<u64>().ok());

        let command = extract_command_name(full_command);

        let mut entry = HistoryEntry::new(command, timestamp, line_number);
        entry.duration = duration;

        if self.preserve_full_line {
            entry = entry.with_full_line(full_command.to_string());
        }

        Some(entry)
    }

    /// Parse content handling multi-line commands.
    fn parse_content(&self, content: &str) -> Vec<HistoryEntry> {
        let mut entries = Vec::new();
        let mut current_line = String::new();
        let mut start_line_number = 1;
        let mut line_number = 0;

        for line in content.lines() {
            line_number += 1;

            if current_line.is_empty() {
                start_line_number = line_number;
            }

            // Check if line continues (ends with backslash)
            if let Some(stripped) = line.strip_suffix('\\') {
                // Remove trailing backslash and add to current
                current_line.push_str(stripped);
                current_line.push('\n');
                continue;
            }

            // Complete the line
            current_line.push_str(line);

            // Try to parse
            if let Some(entry) = self.parse_line(&current_line, start_line_number) {
                entries.push(entry);
            }

            current_line.clear();
        }

        // Handle any remaining content
        if !current_line.is_empty() {
            if let Some(entry) = self.parse_line(&current_line, start_line_number) {
                entries.push(entry);
            }
        }

        entries
    }
}

impl HistoryParser for ZshHistoryParser {
    fn parse_file(&self, path: &Path) -> Result<Vec<HistoryEntry>> {
        // Read as bytes first, then convert with lossy UTF-8 handling
        // ZSH history can contain non-UTF-8 bytes in some cases
        let bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read history file: {:?}", path))?;
        let content = String::from_utf8_lossy(&bytes);
        Ok(self.parse_content(&content))
    }

    fn parse_str(&self, content: &str) -> Result<Vec<HistoryEntry>> {
        Ok(self.parse_content(content))
    }
}

/// Extract just the command name from a full command line.
///
/// This extracts the first word, handling:
/// - Leading whitespace
/// - Environment variable assignments (FOO=bar cmd)
/// - Sudo/command prefixes
fn extract_command_name(line: &str) -> String {
    let line = line.trim();

    // Skip empty or comment lines
    if line.is_empty() || line.starts_with('#') {
        return line.to_string();
    }

    // Handle env var assignments at start: FOO=bar BAZ=qux command
    let mut words = line.split_whitespace().peekable();
    while let Some(word) = words.peek() {
        if word.contains('=') && !word.starts_with('-') {
            words.next();
        } else {
            break;
        }
    }

    // Get the actual command
    let cmd = words.next().unwrap_or("");

    // Check for prefix commands that take another command as argument
    let prefix_commands = ["sudo", "command", "builtin", "exec", "env", "nice", "nohup", "time"];
    if prefix_commands.contains(&cmd) {
        // Return the next word (the actual command)
        words.next().unwrap_or(cmd).to_string()
    } else {
        cmd.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_extended_format() {
        let parser = ZshHistoryParser::new();
        let entries = parser.parse_content(": 1702400000:0;git status\n");

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, "git");
        assert!(entries[0].timestamp.is_some());
    }

    #[test]
    fn parse_multiline_command() {
        let parser = ZshHistoryParser::new();
        let content = ": 1702400000:0;echo hello\\\nworld\n";
        let entries = parser.parse_content(content);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].command, "echo");
    }

    #[test]
    fn parse_simple_format() {
        let parser = ZshHistoryParser::new();
        let entries = parser.parse_content("ls -la\ncd ..\n");

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].command, "ls");
        assert_eq!(entries[1].command, "cd");
        assert!(entries[0].timestamp.is_none());
    }

    #[test]
    fn extract_command_skips_env_vars() {
        assert_eq!(extract_command_name("FOO=bar mycommand"), "mycommand");
        assert_eq!(extract_command_name("A=1 B=2 gcc main.c"), "gcc");
    }

    #[test]
    fn extract_command_handles_sudo() {
        assert_eq!(extract_command_name("sudo pacman -S vim"), "pacman");
    }

    #[test]
    fn timestamp_parsing() {
        let parser = ZshHistoryParser::new();
        let entries = parser.parse_content(": 1702400000:0;test\n");

        let ts = entries[0].timestamp.unwrap();
        // 1702400000 = 2023-12-12 roughly
        assert!(ts.year() >= 2023);
    }

    #[test]
    fn handles_real_world_format() {
        let content = r#": 1765024056:0;ls -la ~/.zshrc
: 1765024162:0;source ~/.zshrc
: 1765024284:0;micro ~/.zshrc
: 1765034660:0;# Nerd Fonts Paket installieren\
sudo pacman -S ttf-jetbrains-mono-nerd\
\
# Font Cache aktualisieren\
fc-cache -fv
"#;
        let parser = ZshHistoryParser::new();
        let entries = parser.parse_content(content);

        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0].command, "ls");
        assert_eq!(entries[1].command, "source");
        assert_eq!(entries[2].command, "micro");
        // Multi-line command starting with comment
        assert!(entries[3].command.starts_with('#'));
    }
}

