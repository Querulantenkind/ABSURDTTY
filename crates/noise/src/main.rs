//! noise - An unhelpful instrument.
//!
//! Responds to commands but never solves anything.
//! Reads mood signatures from tty-mood to adapt its tone.

mod cli;
mod commands;
mod mood_reader;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use mood_reader::MoodContext;
use absurd_core::Chaos;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load mood context
    let ctx = MoodContext::load(cli.mood_file.as_deref());

    // Create chaos source
    let mut chaos = Chaos::from_optional_seed(cli.seed);

    // Execute command
    let output = match &cli.command {
        Commands::Status => commands::cmd_status(&ctx, &mut chaos)?,

        Commands::Ls { path } => {
            commands::cmd_ls(&ctx, &mut chaos, path.as_deref())?
        }

        Commands::Doctor { verbose } => {
            commands::cmd_doctor(&ctx, &mut chaos, *verbose)?
        }

        Commands::Uptime => commands::cmd_uptime(&ctx, &mut chaos)?,

        Commands::Explain { command } => {
            commands::cmd_explain(&ctx, &mut chaos, command)?
        }

        Commands::Form { template } => {
            commands::cmd_form(&ctx, &mut chaos, template)?
        }

        Commands::Patchnotes { since } => {
            commands::cmd_patchnotes(&ctx, &mut chaos, since.as_deref())?
        }
    };

    // Handle output format
    match cli.format {
        OutputFormat::Text => print!("{}", output),
        OutputFormat::Json => {
            let json = serde_json::json!({
                "output": output.trim(),
                "mood": if ctx.has_mood { ctx.mood.label() } else { "none" },
                "case_id": ctx.case_id,
            });
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
    }

    Ok(())
}
