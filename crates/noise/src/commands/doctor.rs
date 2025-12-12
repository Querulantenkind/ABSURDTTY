//! The `noise doctor` command.
//!
//! Provides diagnostic assessment of user/system state.

#![allow(clippy::explicit_auto_deref)]

use crate::mood_reader::MoodContext;
use absurd_core::format::{BoxBuilder, BoxStyle, Stamp, Table};
use absurd_core::Chaos;
use absurd_lexicon::moods::MoodId;
use anyhow::Result;

/// Execute the doctor command.
pub fn cmd_doctor(ctx: &MoodContext, chaos: &mut Chaos, verbose: bool) -> Result<String> {
    if !ctx.has_mood {
        return Ok(boring_doctor());
    }

    let output = match ctx.mood.id {
        MoodId::FeralProductivity => doctor_feral(ctx, chaos, verbose),
        MoodId::Exhausted => doctor_exhausted(ctx, chaos, verbose),
        MoodId::Methodical => doctor_methodical(ctx, verbose),
        MoodId::ChaoticNeutral => doctor_chaotic(ctx, chaos, verbose),
        MoodId::BureaucraticZen => doctor_bureaucratic(ctx, verbose),
        MoodId::AmbientDrift => doctor_drift(ctx, chaos),
        MoodId::RecursiveDoubt => doctor_doubt(ctx, chaos),
        MoodId::EmergencyMode => doctor_emergency(ctx, chaos),
        MoodId::Neutral => boring_doctor(),
    };

    Ok(output)
}

fn boring_doctor() -> String {
    let mut output = String::new();
    output.push_str("DIAGNOSIS: No mood signature found.\n");
    output.push_str("PRESCRIPTION: Run 'tty-mood generate' first.\n");
    output.push_str("PROGNOSIS: Boring until remedied.\n");
    output
}

fn doctor_feral(ctx: &MoodContext, chaos: &mut Chaos, verbose: bool) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Single)
        .title("PATIENT FILE")
        .line(format!("Case: {}", ctx.case_id))
        .build());

    output.push_str("SYMPTOMS OBSERVED:\n");
    output.push_str("  - Elevated command cadence\n");
    output.push_str("  - Late-night orbital pattern detected\n");
    output.push_str("  - Curiosity spike: confirmed\n");

    if verbose {
        output.push_str("  - Sleep schedule: non-existent\n");
        output.push_str("  - Focus: intense but narrow\n");
        output.push_str("  - Hydration status: unknown\n");
    }

    output.push_str("\nDIAGNOSIS: Acute productivity mania\n");

    let prescription = chaos.pick_unwrap(&[
        "None. This is not a medical facility.",
        "Consider: sleep exists.",
        "Perhaps consume food at some point.",
        "The rabbit hole has no bottom. Plan accordingly.",
    ]);
    output.push_str(&format!("PRESCRIPTION: {}\n", prescription));

    output.push_str("PROGNOSIS: Continued operation until collapse or insight.\n");
    output.push_str("           Whichever arrives first.\n");
    output.push_str("\nNOTES: Patient unlikely to follow recommendations.\n");
    output.push_str("       Recommendations therefore not binding.\n");

    output
}

fn doctor_exhausted(ctx: &MoodContext, chaos: &mut Chaos, verbose: bool) -> String {
    let mut output = String::new();

    output.push_str(&format!("PATIENT FILE: {}\n\n", ctx.case_id));

    output.push_str("SYMPTOMS OBSERVED:\n");
    output.push_str("  - Command velocity: collapsed\n");
    output.push_str("  - Error rate: elevated\n");
    output.push_str("  - Response time: delayed\n");

    if verbose {
        output.push_str("  - Typos: frequent\n");
        output.push_str("  - Repeat commands: yes\n");
        output.push_str("  - Caffeine dependency: probable\n");
    }

    output.push_str("\nDIAGNOSIS: Terminal exhaustion\n");

    let prescription = chaos.pick_unwrap(&[
        "Horizontal position. Immediately.",
        "Close the laptop. It will still be there tomorrow.",
        "The bugs can wait. You cannot.",
        "Sleep: mandatory, not optional.",
    ]);
    output.push_str(&format!("PRESCRIPTION: {}\n", prescription));
    output.push_str("PROGNOSIS: Recovery possible with intervention.\n");

    output
}

fn doctor_methodical(ctx: &MoodContext, verbose: bool) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("DIAGNOSTIC REPORT")
        .line(format!("Patient Reference: {}", ctx.case_id))
        .line(format!("Classification: {}", ctx.mood.label()))
        .build());

    output.push_str("\nSYSTEMATIC ASSESSMENT:\n\n");
    output.push_str(&Table::new()
        .row("Command Rhythm", "STEADY")
        .row("Error Rate", "LOW")
        .row("Pattern Consistency", "HIGH")
        .row("Documentation Status", "CURRENT")
        .build());

    if verbose {
        output.push_str("\nDETAILED METRICS:\n");
        output.push_str(&Table::new()
            .row("Workflow Adherence", "98.2%")
            .row("Process Compliance", "SATISFACTORY")
            .row("Deviation Events", "0")
            .build());
    }

    output.push_str("\nDIAGNOSIS: Optimal operational state\n");
    output.push_str("PRESCRIPTION: Continue current protocol\n");
    output.push_str("PROGNOSIS: Stable\n");
    output.push_str("\nNo intervention required.\n");

    output
}

fn doctor_chaotic(ctx: &MoodContext, chaos: &mut Chaos, verbose: bool) -> String {
    let mut output = String::new();

    output.push_str(&format!("PATIENT FILE: {} (probably)\n\n", ctx.case_id));

    output.push_str("SYMPTOMS OBSERVED:\n");
    let symptoms = [
        "Unpredictable command patterns",
        "Entropy levels: rising",
        "Direction: multiple",
        "Focus: variable",
        "Method: unclear",
    ];

    let mut shown: Vec<_> = symptoms.to_vec();
    chaos.shuffle(&mut shown);

    for symptom in shown.iter().take(if verbose { 5 } else { 3 }) {
        output.push_str(&format!("  - {}\n", symptom));
    }

    output.push_str("\nDIAGNOSIS: ");
    output.push_str(*chaos.pick_unwrap(&[
        "Chaotic but stable (paradoxically)",
        "Entropy within tolerable limits",
        "Creative disorder in progress",
        "Organized chaos (emphasis on chaos)",
    ]));
    output.push('\n');

    output.push_str("PRESCRIPTION: None. You wouldn't follow it anyway.\n");
    output.push_str("PROGNOSIS: Uncertain. By design.\n");

    output
}

fn doctor_bureaucratic(ctx: &MoodContext, verbose: bool) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("FORM D-001: DIAGNOSTIC DECLARATION")
        .line(format!("Case Number: {}", ctx.case_id))
        .build());

    output.push_str("\nPATIENT ASSESSMENT:\n\n");

    output.push_str("The patient demonstrates adherence to proper form and procedure.\n");
    output.push_str("All rituals observed. All checkboxes checked.\n\n");

    if verbose {
        output.push_str("DETAILED COMPLIANCE RECORD:\n");
        output.push_str("  [X] Command history reviewed\n");
        output.push_str("  [X] Patterns catalogued\n");
        output.push_str("  [X] Forms filed\n");
        output.push_str("  [X] Bureaucratic requirements met\n\n");
    }

    output.push_str("DIAGNOSIS: Bureaucratic wellness achieved\n");
    output.push_str("PRESCRIPTION: Continue filing forms\n");
    output.push_str("PROGNOSIS: Compliant\n\n");

    output.push_str(&Stamp::Filed.inline());
    output.push('\n');

    output
}

fn doctor_drift(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("case: {}\n\n", ctx.case_id.to_lowercase()));

    output.push_str("symptoms:\n");
    output.push_str("  - present, unfocused\n");
    output.push_str("  - commands without direction\n");
    output.push_str("  - movement without destination\n\n");

    let diagnosis = chaos.pick_unwrap(&[
        "ambient drift",
        "terminal wandering",
        "purposeful purposelessness",
    ]);
    output.push_str(&format!("diagnosis: {}\n", diagnosis));
    output.push_str("prescription: none required\n");
    output.push_str("prognosis: floating\n");

    output
}

fn doctor_doubt(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&format!("PATIENT FILE: {} (?)\n\n", ctx.case_id));

    output.push_str("SYMPTOMS OBSERVED (probably):\n");
    output.push_str("  - Repeated status checks\n");
    output.push_str("  - Validation loops\n");
    output.push_str("  - Uncertainty spirals\n");
    output.push_str("  - Second-guessing patterns\n\n");

    output.push_str("DIAGNOSIS: Recursive doubt\n");
    output.push_str("          (Are we sure about this diagnosis?)\n\n");

    let prescription = chaos.pick_unwrap(&[
        "Trust the process. Or don't. It's unclear.",
        "Maybe run the tests again? Just in case.",
        "git status. One more time.",
        "The answer is probably fine. Probably.",
    ]);
    output.push_str(&format!("PRESCRIPTION: {}\n", prescription));
    output.push_str("PROGNOSIS: Uncertain (naturally)\n");

    output
}

fn doctor_emergency(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str("!! EMERGENCY DIAGNOSTIC !!\n\n");
    output.push_str(&format!("CASE: {}\n\n", ctx.case_id));

    output.push_str("SYMPTOMS:\n");
    output.push_str("  - Rapid command bursts\n");
    output.push_str("  - Elevated error rate\n");
    output.push_str("  - Correction patterns: frantic\n");
    output.push_str("  - Stress indicators: high\n\n");

    output.push_str("DIAGNOSIS: Crisis mode active\n\n");

    let prescription = chaos.pick_unwrap(&[
        "Breathe. Then type.",
        "The production server can wait 30 seconds.",
        "Panic is not a debugging strategy.",
        "Step away. Return with clarity.",
    ]);
    output.push_str(&format!("PRESCRIPTION: {}\n", prescription));
    output.push_str("PROGNOSIS: This too shall pass.\n");

    output
}

