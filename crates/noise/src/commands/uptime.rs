//! The `noise uptime` command.
//!
//! Reports system uptime with philosophical commentary.

use crate::mood_reader::MoodContext;
use absurd_core::Chaos;
use absurd_lexicon::moods::MoodId;
use anyhow::Result;
use std::process::Command;

/// Execute the uptime command.
pub fn cmd_uptime(ctx: &MoodContext, chaos: &mut Chaos) -> Result<String> {
    let system_uptime = get_system_uptime();

    if !ctx.has_mood {
        return Ok(format!("System uptime: {}\n", system_uptime));
    }

    let output = match ctx.mood.id {
        MoodId::FeralProductivity => uptime_feral(&system_uptime, chaos),
        MoodId::Exhausted => uptime_exhausted(&system_uptime, chaos),
        MoodId::Methodical => uptime_methodical(&system_uptime),
        MoodId::ChaoticNeutral => uptime_chaotic(&system_uptime, chaos),
        MoodId::BureaucraticZen => uptime_bureaucratic(&system_uptime),
        MoodId::AmbientDrift => uptime_drift(&system_uptime),
        MoodId::RecursiveDoubt => uptime_doubt(&system_uptime, chaos),
        MoodId::EmergencyMode => uptime_emergency(&system_uptime),
        MoodId::Neutral => format!("System uptime: {}\n", system_uptime),
    };

    Ok(output)
}

fn get_system_uptime() -> String {
    // Try to get actual uptime from /proc/uptime
    if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
        if let Some(secs_str) = content.split_whitespace().next() {
            if let Ok(secs) = secs_str.parse::<f64>() {
                return format_uptime(secs as u64);
            }
        }
    }

    // Fallback to uptime command
    if let Ok(output) = Command::new("uptime").arg("-p").output() {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }

    "unknown".to_string()
}

fn format_uptime(total_secs: u64) -> String {
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;

    if days > 0 {
        format!("{} days, {} hours, {} minutes", days, hours, mins)
    } else if hours > 0 {
        format!("{} hours, {} minutes", hours, mins)
    } else {
        format!("{} minutes", mins)
    }
}

fn uptime_feral(uptime: &str, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("SYSTEM UPTIME: {}\n", uptime));
    output.push_str("USER UPTIME: [REDACTED - likely exceeds safe parameters]\n\n");

    let observation = chaos.pick_unwrap(&[
        "Machine and operator racing toward their respective limits.",
        "System stable. Operator stability: unverified.",
        "Both running hot. Only one will need to reboot.",
        "The machine rests when you tell it to. Consider the implications.",
    ]);

    output.push_str(&format!("OBSERVATION: {}\n", observation));

    output
}

fn uptime_exhausted(uptime: &str, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("SYSTEM UPTIME: {}\n", uptime));
    output.push_str("USER UPTIME: [CONCERNING]\n\n");

    let discrepancy = chaos.pick_unwrap(&[
        "Machine outlasting operator.",
        "System: functional. User: questionable.",
        "Hardware: stable. Wetware: degrading.",
        "The computer doesn't need sleep. You do.",
    ]);

    output.push_str(&format!("DISCREPANCY: {}\n", discrepancy));
    output.push_str("RECOMMENDATION: Role reversal advised.\n");

    output
}

fn uptime_methodical(uptime: &str) -> String {
    let mut output = String::new();

    output.push_str("SYSTEM UPTIME REPORT\n");
    output.push_str("─".repeat(30).as_str());
    output.push('\n');
    output.push_str(&format!("Current uptime: {}\n", uptime));
    output.push_str("Status: NOMINAL\n");
    output.push_str("Last reboot: LOGGED\n");
    output.push_str("─".repeat(30).as_str());
    output.push('\n');
    output.push_str("Report filed.\n");

    output
}

fn uptime_chaotic(uptime: &str, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("SYSTEM UPTIME: {} (allegedly)\n", uptime));

    let comment = chaos.pick_unwrap(&[
        "Time is a construct. Uptime doubly so.",
        "The system claims this. We have no reason to trust it.",
        "Numbers. They could mean anything.",
        "Running since... whenever. Does it matter?",
    ]);

    output.push_str(&format!("\n{}\n", comment));

    output
}

fn uptime_bureaucratic(uptime: &str) -> String {
    let mut output = String::new();

    output.push_str("FORM U-001: UPTIME DECLARATION\n");
    output.push_str("══════════════════════════════\n\n");
    output.push_str(&format!("System Uptime: {}\n", uptime));
    output.push_str("Verification: AUTOMATIC\n");
    output.push_str("Filing Status: RECORDED\n\n");
    output.push_str("This uptime is hereby declared for the record.\n");
    output.push_str("No action required.\n");

    output
}

fn uptime_drift(uptime: &str) -> String {
    format!("uptime: {}\ntime passes...\n", uptime)
}

fn uptime_doubt(uptime: &str, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("SYSTEM UPTIME: {} (?)\n", uptime));
    output.push_str("(Is this accurate? Should we check again?)\n\n");

    let question = chaos.pick_unwrap(&[
        "Has it really been that long?",
        "Should we verify this independently?",
        "The system could be lying.",
        "Maybe reboot just to be sure?",
    ]);

    output.push_str(&format!("{}\n", question));

    output
}

fn uptime_emergency(uptime: &str) -> String {
    format!("UPTIME: {}\n[NOTED - MOVING ON]\n", uptime)
}

