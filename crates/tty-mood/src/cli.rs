//! CLI definition for tty-mood.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tty-mood")]
#[command(version, about = "A local-first mood reader that analyzes shell history")]
#[command(long_about = "
tty-mood observes your shell history and generates a mood signature.
This signature is then used by noise to adapt its responses.

All analysis is local and read-only. Your history never leaves your machine.
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Seed for reproducible analysis
    #[arg(long, global = true)]
    pub seed: Option<u64>,

    /// Output format: text, json
    #[arg(long, global = true, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a mood signature from shell history
    Generate {
        /// Time range to analyze (e.g., 7d, 14d, 30d)
        #[arg(long, default_value = "7d")]
        range: String,

        /// Path to shell history file (auto-detected if not specified)
        #[arg(long)]
        history: Option<PathBuf>,

        /// Output file path (default: ~/.local/share/absurdtty/mood.json)
        #[arg(long, short)]
        out: Option<PathBuf>,

        /// Print output instead of writing to file
        #[arg(long)]
        dry_run: bool,
    },

    /// Show current mood signature
    Show {
        /// Path to mood file (default: ~/.local/share/absurdtty/mood.json)
        #[arg(long)]
        mood_file: Option<PathBuf>,
    },

    /// List all detected signals from current analysis
    Signals {
        /// Time range to analyze
        #[arg(long, default_value = "7d")]
        range: String,

        /// Path to shell history file
        #[arg(long)]
        history: Option<PathBuf>,

        /// Show all signals (including weak ones)
        #[arg(long)]
        all: bool,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Unknown format: {}. Use 'text' or 'json'.", s)),
        }
    }
}

/// Parse time range string into days.
pub fn parse_range(range: &str) -> anyhow::Result<u32> {
    let range = range.trim().to_lowercase();

    if let Some(days) = range.strip_suffix('d') {
        days.parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Invalid range: {}", range))
    } else if let Some(weeks) = range.strip_suffix('w') {
        let w = weeks
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Invalid range: {}", range))?;
        Ok(w * 7)
    } else {
        // Try parsing as plain number (days)
        range
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Invalid range: {}. Use format like '7d' or '2w'.", range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_range_days() {
        assert_eq!(parse_range("7d").unwrap(), 7);
        assert_eq!(parse_range("14d").unwrap(), 14);
        assert_eq!(parse_range("30d").unwrap(), 30);
    }

    #[test]
    fn parse_range_weeks() {
        assert_eq!(parse_range("1w").unwrap(), 7);
        assert_eq!(parse_range("2w").unwrap(), 14);
    }

    #[test]
    fn parse_range_plain_number() {
        assert_eq!(parse_range("7").unwrap(), 7);
    }

    #[test]
    fn parse_range_invalid() {
        assert!(parse_range("abc").is_err());
        assert!(parse_range("7x").is_err());
    }
}

