//! The `noise patchnotes` command.
//!
//! Displays fake changelog/patchnotes for your terminal system.

#![allow(clippy::explicit_auto_deref)]

use crate::mood_reader::MoodContext;
use absurd_core::format::BoxBuilder;
use absurd_core::Chaos;
use anyhow::Result;

/// Execute the patchnotes command.
pub fn cmd_patchnotes(ctx: &MoodContext, chaos: &mut Chaos, _since: Option<&str>) -> Result<String> {
    let mut output = String::new();

    let version = format!("v2025.{}.{}", chaos.range(1, 12), chaos.range(1, 31));

    output.push_str(&BoxBuilder::new()
        .title(format!("TERMINAL SYSTEM PATCHNOTES {}", version))
        .build());

    output.push('\n');

    // Mood engine section
    output.push_str("MOOD ENGINE:\n");
    let mood_changes = generate_mood_changes(ctx, chaos);
    for change in mood_changes {
        output.push_str(&format!("  {}\n", change));
    }
    output.push('\n');

    // Signal detection section
    output.push_str("SIGNAL DETECTION:\n");
    let signal_changes = generate_signal_changes(chaos);
    for change in signal_changes {
        output.push_str(&format!("  {}\n", change));
    }
    output.push('\n');

    // Output formatting section
    output.push_str("OUTPUT FORMATTING:\n");
    let format_changes = generate_format_changes(chaos);
    for change in format_changes {
        output.push_str(&format!("  {}\n", change));
    }
    output.push('\n');

    // Known issues
    output.push_str("KNOWN ISSUES:\n");
    let issues = generate_known_issues(chaos);
    for issue in issues {
        output.push_str(&format!("  - {}\n", issue));
    }
    output.push('\n');

    // Breaking changes
    output.push_str("BREAKING CHANGES:\n");
    output.push_str(*chaos.pick_unwrap(&[
        "  - None. This was always broken in this specific way.\n",
        "  - Everything. But it was already broken.\n",
        "  - The concept of 'breaking' implies something worked before.\n",
        "  - Compatibility with reality: reduced.\n",
    ]));
    output.push('\n');

    // Deprecation notice
    output.push_str("DEPRECATION NOTICE:\n");
    output.push_str(*chaos.pick_unwrap(&[
        "  - Meaning (scheduled removal: TBD)\n",
        "  - Hope (status: experimental)\n",
        "  - Sleep schedules (migration path: unclear)\n",
        "  - Productivity expectations (no replacement planned)\n",
    ]));

    Ok(output)
}

fn generate_mood_changes(ctx: &MoodContext, chaos: &mut Chaos) -> Vec<String> {
    let mut changes = Vec::new();

    // Always include current mood
    changes.push(format!("+ Added: '{}' mood state detected", ctx.mood.label()));

    let additions = [
        "+ Added: 'feral_productivity' mood state",
        "+ Added: 'bureaucratic_zen' detection",
        "+ Added: 'ambient_drift' recognition",
        "+ Added: mood persistence across sessions",
    ];

    let removals = [
        "- Removed: 'calm_confidence' (user never achieved it)",
        "- Removed: 'well_rested' (insufficient data)",
        "- Removed: 'organized' (detection failed)",
        "- Removed: 'certainty' (deprecated)",
    ];

    let fixes = [
        "* Fixed: Occasional honesty in status reports",
        "* Fixed: Mood detection being too accurate",
        "* Fixed: False positives for 'happiness'",
        "* Changed: Default mood now accurately pessimistic",
    ];

    // Pick random items
    for _ in 0..chaos.range(1, 3) {
        changes.push(chaos.pick_unwrap(&additions).to_string());
    }
    changes.push(chaos.pick_unwrap(&removals).to_string());
    changes.push(chaos.pick_unwrap(&fixes).to_string());

    changes
}

fn generate_signal_changes(chaos: &mut Chaos) -> Vec<String> {
    let mut changes = Vec::new();

    let items = [
        "+ Added: 'late_night_orbit' pattern recognition",
        "+ Added: 'recursive_doubt' detection",
        "* Improved: Typo rate now 23% more judgmental",
        "* Improved: Command cadence analysis accuracy",
        "* Changed: Error detection sensitivity increased",
        "- Deprecated: 'productivity' signal (never reliable)",
    ];

    for _ in 0..chaos.range(2, 4) {
        changes.push(chaos.pick_unwrap(&items).to_string());
    }

    changes
}

fn generate_format_changes(chaos: &mut Chaos) -> Vec<String> {
    let mut changes = Vec::new();

    let items = [
        "- Fixed: Excessive clarity in error messages",
        "+ Added: More ambiguity in success confirmations",
        "* Changed: Timestamps now occasionally philosophical",
        "+ Added: Box-drawing character support",
        "* Improved: Stamp authenticity",
        "- Removed: Helpful suggestions (by request)",
    ];

    for _ in 0..chaos.range(2, 4) {
        changes.push(chaos.pick_unwrap(&items).to_string());
    }

    changes
}

fn generate_known_issues(chaos: &mut Chaos) -> Vec<String> {
    let mut issues = Vec::new();

    let all_issues = [
        "User expectations still calibrated incorrectly",
        "Hope persists despite contrary evidence",
        "Reality checks fail more often than they should",
        "Sleep debt accumulator has no upper bound",
        "Coffee dependency not properly tracked",
        "Time perception increasingly unreliable",
        "Documentation remains unread",
        "Backlog grows faster than capacity",
    ];

    for _ in 0..chaos.range(3, 5) {
        issues.push(chaos.pick_unwrap(&all_issues).to_string());
    }

    issues
}

