//! The `noise status` command.
//!
//! Shows current mood and case information with mood-adapted commentary.

#![allow(clippy::explicit_auto_deref)]

use crate::mood_reader::MoodContext;
use absurd_core::format::{BoxBuilder, BoxStyle, Stamp, Table};
use absurd_core::Chaos;
use absurd_lexicon::moods::MoodId;
use anyhow::Result;

/// Execute the status command.
pub fn cmd_status(ctx: &MoodContext, chaos: &mut Chaos) -> Result<String> {
    if !ctx.has_mood {
        return Ok(boring_status());
    }

    let output = match ctx.mood.id {
        MoodId::FeralProductivity => status_feral(ctx, chaos),
        MoodId::Exhausted => status_exhausted(ctx, chaos),
        MoodId::Methodical => status_methodical(ctx, chaos),
        MoodId::ChaoticNeutral => status_chaotic(ctx, chaos),
        MoodId::BureaucraticZen => status_bureaucratic(ctx, chaos),
        MoodId::AmbientDrift => status_drift(ctx, chaos),
        MoodId::RecursiveDoubt => status_doubt(ctx, chaos),
        MoodId::EmergencyMode => status_emergency(ctx, chaos),
        MoodId::Neutral => boring_status(),
    };

    Ok(output)
}

fn boring_status() -> String {
    "System operational.\n".to_string()
}

fn status_feral(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let velocity = chaos.pick_unwrap(&[
        "exceeding parameters",
        "approaching critical",
        "beyond measurement",
        "suspiciously high",
    ]);

    let observation = chaos.pick_unwrap(&[
        "Curiosity spike confirmed.",
        "Sleep debt: accumulating.",
        "Ideas per minute: concerning.",
        "Keyboard temperature: elevated.",
    ]);

    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Single)
        .title("STATUS REPORT")
        .line(format!("CASE: {}", ctx.case_id))
        .line(format!("MOOD: {} (velocity: {})", ctx.mood.label(), velocity))
        .build());

    output.push_str(&Table::new()
        .row("CONFIDENCE", format!("{:.0}%", ctx.mood.confidence * 100.0))
        .row("TRAJECTORY", "upward (unsustainable)")
        .row("OBSERVATION", *observation)
        .build());

    output.push('\n');

    if !ctx.mood.notes.is_empty() {
        output.push_str("NOTES:\n");
        for note in &ctx.mood.notes {
            output.push_str(&format!("  - {}\n", note));
        }
        output.push('\n');
    }

    output.push_str(&Stamp::Certified.inline());
    output.push('\n');

    output
}

fn status_exhausted(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let state = chaos.pick_unwrap(&[
        "operational but barely",
        "functional (allegedly)",
        "running on residual momentum",
        "technically online",
    ]);

    let recommendation = chaos.pick_unwrap(&[
        "Consider: horizontal position.",
        "Perhaps the machine should run itself.",
        "Caffeine levels: insufficient.",
        "Energy budget: overdrawn.",
    ]);

    let mut output = String::new();

    output.push_str("CASE: ");
    output.push_str(&ctx.case_id);
    output.push('\n');
    output.push_str("MOOD: ");
    output.push_str(ctx.mood.label());
    output.push_str(" (confidence: regrettably high)\n");
    output.push_str("STATUS: ");
    output.push_str(state);
    output.push_str("\n\n");
    output.push_str(recommendation);
    output.push_str("\n\n");

    // Truncated output for exhausted mood
    if ctx.tone.should_truncate() {
        output.push_str("[OUTPUT TRUNCATED: Energy budget exceeded]\n");
    }

    output
}

fn status_methodical(ctx: &MoodContext, _chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("SYSTEM STATUS REPORT")
        .line(format!("Case Reference: {}", ctx.case_id))
        .line(format!("Classification: {}", ctx.mood.label()))
        .line(format!("Confidence Level: {:.1}%", ctx.mood.confidence * 100.0))
        .build());

    output.push_str("\nSYSTEM ASSESSMENT:\n");
    output.push_str("  Status: NOMINAL\n");
    output.push_str("  Deviation from baseline: WITHIN PARAMETERS\n");
    output.push_str("  Verification: COMPLETE\n");
    output.push_str("  Documentation: CURRENT\n\n");

    if !ctx.mood.notes.is_empty() {
        output.push_str("RECORDED OBSERVATIONS:\n");
        for (i, note) in ctx.mood.notes.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", i + 1, note));
        }
        output.push('\n');
    }

    output.push_str("No action required. All systems catalogued.\n");

    output
}

fn status_chaotic(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    let maybe = if chaos.chance(0.5) { "probably" } else { "allegedly" };

    output.push_str(&format!("CASE: {} ({})\n", ctx.case_id, maybe));
    output.push_str(&format!("MOOD: {} (or is it?)\n", ctx.mood.label()));
    output.push_str(&format!("CONFIDENCE: {:.0}%", ctx.mood.confidence * 100.0));

    if chaos.chance(0.3) {
        output.push_str(" (give or take)");
    }
    output.push('\n');

    output.push_str("\nSTATUS: ");
    output.push_str(*chaos.pick_unwrap(&[
        "Entropy: rising but controlled.",
        "Direction: multiple, simultaneously.",
        "Pattern: emerging (maybe).",
        "Coherence: optional.",
    ]));
    output.push_str("\n\n");

    // Sometimes add extra chaos
    if chaos.chance(0.4) {
        output.push_str("NOTE: This status report may or may not reflect reality.\n");
    }

    output
}

fn status_bureaucratic(ctx: &MoodContext, _chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("FORM S-001: STATUS DECLARATION")
        .line(format!("Filed: {}", chrono_now()))
        .line(format!("Case Number: {}", ctx.case_id))
        .build());

    output.push_str("\nDECLARATION OF CURRENT STATE:\n\n");
    output.push_str(&Table::new()
        .row("Mood Classification", ctx.mood.label())
        .row("Confidence Rating", format!("{:.1}%", ctx.mood.confidence * 100.0))
        .row("Form Compliance", "SATISFACTORY")
        .row("Ritual Adherence", "OBSERVED")
        .build());

    output.push('\n');
    output.push_str("This declaration is filed for the record.\n");
    output.push_str("No response is expected. No action is required.\n");
    output.push_str("The form is its own purpose.\n\n");

    output.push_str(&Stamp::NullBureau.inline());
    output.push('\n');

    output
}

fn status_drift(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("case: {}\n", ctx.case_id.to_lowercase()));
    output.push_str(&format!("mood: {}\n", ctx.mood.label()));
    output.push_str(&format!("confidence: {:.0}%\n", ctx.mood.confidence * 100.0));
    output.push('\n');

    let observation = chaos.pick_unwrap(&[
        "present, unfocused",
        "here, sort of",
        "online, drifting",
        "connected, disconnected",
    ]);
    output.push_str(&format!("status: {}\n", observation));
    output.push('\n');

    if chaos.chance(0.5) {
        output.push_str("...\n");
    }

    output
}

fn status_doubt(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("CASE: {} (?)\n", ctx.case_id));
    output.push_str(&format!("MOOD: {} (probably)\n", ctx.mood.label()));
    output.push_str(&format!("CONFIDENCE: {:.0}%", ctx.mood.confidence * 100.0));
    output.push_str(" (is that right?)\n\n");

    output.push_str("STATUS CHECK:\n");
    output.push_str("  - System: operational (verify?)\n");
    output.push_str("  - Files: present (check again?)\n");
    output.push_str("  - State: uncertain (always)\n\n");

    let suggestion = chaos.pick_unwrap(&[
        "Consider running status again. Just to be sure.",
        "Did you save? You should check.",
        "Maybe run git status. One more time.",
        "Are you sure that worked?",
    ]);
    output.push_str(&format!("SUGGESTION: {}\n", suggestion));

    output
}

fn status_emergency(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    let urgency = chaos.pick_unwrap(&["HIGH", "ELEVATED", "CRITICAL", "CONCERNING"]);

    output.push_str(&format!("!! CASE: {} !!\n", ctx.case_id));
    output.push_str(&format!("MOOD: {} [URGENCY: {}]\n", ctx.mood.label(), urgency));
    output.push_str(&format!("CONFIDENCE: {:.0}%\n\n", ctx.mood.confidence * 100.0));

    output.push_str("SITUATION ASSESSMENT:\n");
    output.push_str("  - Error rate: elevated\n");
    output.push_str("  - Command velocity: erratic\n");
    output.push_str("  - Correction patterns: detected\n");
    output.push_str("  - Calm: not detected\n\n");

    let advice = chaos.pick_unwrap(&[
        "Breathe. The terminal will wait.",
        "Step back. Assess. Then proceed.",
        "The bug is not going anywhere.",
        "Consider: is this truly urgent?",
    ]);
    output.push_str(&format!("ADVICE: {}\n", advice));

    output
}

/// Get current date/time as string.
fn chrono_now() -> String {
    // Simple implementation without chrono crate
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Just return the unix timestamp for now
    format!("T{}", now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boring_status_works() {
        let output = boring_status();
        assert!(output.contains("operational"));
    }

    #[test]
    fn status_with_mood() {
        use absurd_lexicon::moods::Mood;

        let ctx = MoodContext {
            mood: Mood::new(MoodId::FeralProductivity, 0.8),
            case_id: "TEST-001".to_string(),
            tone: absurd_lexicon::tone::Tone::for_mood(MoodId::FeralProductivity),
            has_mood: true,
        };

        let mut chaos = Chaos::seeded(42);
        let output = cmd_status(&ctx, &mut chaos).unwrap();

        assert!(output.contains("TEST-001"));
        assert!(output.contains("feral"));
    }
}

