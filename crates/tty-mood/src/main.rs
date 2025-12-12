//! tty-mood - A mood reader for terminal behavior.
//!
//! Analyzes shell history to generate mood signatures.

mod cli;
mod history;
mod mood;
mod report;
mod signals;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use history::HistoryParser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate {
            range,
            history,
            out,
            dry_run,
        } => cmd_generate(&cli, range, history.clone(), out.clone(), *dry_run),

        Commands::Show { mood_file } => cmd_show(&cli, mood_file.clone()),

        Commands::Signals {
            range,
            history,
            all,
        } => cmd_signals(&cli, range, history.clone(), *all),
    }
}

fn cmd_generate(
    cli: &Cli,
    range: &str,
    history_path: Option<std::path::PathBuf>,
    out_path: Option<std::path::PathBuf>,
    dry_run: bool,
) -> Result<()> {
    // Resolve history file
    let history_path = history_path
        .or_else(absurd_core::Paths::shell_history)
        .context("Could not detect shell history. Use --history to specify path.")?;

    eprintln!("Reading history from: {:?}", history_path);

    // Parse history
    let parser = history::ZshHistoryParser::new();
    let entries = parser.parse_file(&history_path)?;

    eprintln!("Found {} history entries", entries.len());

    // Filter by time range
    let days = cli::parse_range(range)?;
    let (since, until) = history::last_n_days(days);
    let entries = history::filter_by_range(entries, Some(since), Some(until));

    eprintln!("Analyzing {} entries from last {} days", entries.len(), days);

    if entries.is_empty() {
        anyhow::bail!("No history entries found in the specified time range");
    }

    // Analyze signals
    let signals = signals::analyze(&entries);

    eprintln!("Detected {} signals", signals.signals.len());

    // Detect mood
    let detected_mood = mood::detect_mood(&signals);

    eprintln!(
        "Detected mood: {} (confidence: {:.0}%)",
        detected_mood.label(),
        detected_mood.confidence * 100.0
    );

    // Create report
    let source = report::SourceInfo {
        shell: detect_shell(),
        history_path: history_path.to_string_lossy().to_string(),
        read_only: true,
        entries_analyzed: entries.len(),
    };

    let signature = report::MoodSignature::new(&detected_mood, &signals, source, range, cli.seed);

    // Output
    if dry_run {
        match cli.format {
            OutputFormat::Json => println!("{}", signature.to_json()?),
            OutputFormat::Text => println!("{}", signature.to_summary()),
        }
    } else {
        let out_path = out_path
            .or_else(|| absurd_core::Paths::mood_file().ok())
            .context("Could not determine output path")?;

        signature.write_to_file(&out_path)?;
        eprintln!("Wrote mood signature to: {:?}", out_path);

        match cli.format {
            OutputFormat::Json => println!("{}", signature.to_json()?),
            OutputFormat::Text => println!("{}", signature.to_summary()),
        }
    }

    Ok(())
}

fn cmd_show(cli: &Cli, mood_file: Option<std::path::PathBuf>) -> Result<()> {
    let mood_file = mood_file
        .or_else(|| absurd_core::Paths::mood_file().ok())
        .context("Could not determine mood file path")?;

    if !mood_file.exists() {
        anyhow::bail!(
            "No mood signature found at {:?}. Run 'tty-mood generate' first.",
            mood_file
        );
    }

    let content = std::fs::read_to_string(&mood_file)
        .with_context(|| format!("Failed to read mood file: {:?}", mood_file))?;

    let signature: report::MoodSignature = serde_json::from_str(&content)
        .with_context(|| "Failed to parse mood signature")?;

    match cli.format {
        OutputFormat::Json => println!("{}", signature.to_json()?),
        OutputFormat::Text => println!("{}", signature.to_summary()),
    }

    Ok(())
}

fn cmd_signals(
    cli: &Cli,
    range: &str,
    history_path: Option<std::path::PathBuf>,
    show_all: bool,
) -> Result<()> {
    // Resolve history file
    let history_path = history_path
        .or_else(absurd_core::Paths::shell_history)
        .context("Could not detect shell history")?;

    // Parse and filter history
    let parser = history::ZshHistoryParser::new();
    let entries = parser.parse_file(&history_path)?;

    let days = cli::parse_range(range)?;
    let (since, until) = history::last_n_days(days);
    let entries = history::filter_by_range(entries, Some(since), Some(until));

    if entries.is_empty() {
        println!("No history entries found in the specified time range.");
        return Ok(());
    }

    // Analyze
    let signals = signals::analyze(&entries);

    // Filter and sort
    let mut signals_vec: Vec<_> = signals
        .signals
        .iter()
        .filter(|s| show_all || s.score >= 0.3)
        .collect();

    signals_vec.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    match cli.format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&signals_vec)?;
            println!("{}", json);
        }
        OutputFormat::Text => {
            println!("DETECTED SIGNALS (last {} days, {} entries):\n", days, entries.len());

            if signals_vec.is_empty() {
                println!("  No significant signals detected.");
            } else {
                for signal in signals_vec {
                    let bar_len = (signal.score * 20.0) as usize;
                    let bar = "#".repeat(bar_len);
                    let empty = "-".repeat(20 - bar_len);

                    print!(
                        "  {:24} [{}{}] {:5.1}%",
                        signal.id,
                        bar,
                        empty,
                        signal.score * 100.0
                    );

                    if let Some(note) = &signal.note {
                        print!("  {}", note);
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

/// Detect current shell from environment.
fn detect_shell() -> String {
    std::env::var("SHELL")
        .ok()
        .and_then(|s| s.rsplit('/').next().map(String::from))
        .unwrap_or_else(|| "unknown".to_string())
}
