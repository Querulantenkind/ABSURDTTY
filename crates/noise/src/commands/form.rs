//! The `noise form` command.
//!
//! Generates bureaucratic forms for self-reporting and documentation.

#![allow(clippy::explicit_auto_deref)]

use crate::mood_reader::MoodContext;
use absurd_core::format::{BoxBuilder, BoxStyle, Stamp};
use absurd_core::Chaos;
use anyhow::Result;

/// Execute the form command.
pub fn cmd_form(ctx: &MoodContext, chaos: &mut Chaos, template: &str) -> Result<String> {
    let output = match template.to_lowercase().as_str() {
        "declaration" | "default" => form_declaration(ctx, chaos),
        "incident" => form_incident(ctx, chaos),
        "requisition" => form_requisition(chaos),
        "appeal" => form_appeal(chaos),
        _ => {
            format!(
                "Unknown form template: '{}'\n\nAvailable templates:\n  - declaration\n  - incident\n  - requisition\n  - appeal\n",
                template
            )
        }
    };

    Ok(output)
}

fn form_declaration(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("FORM 27-B: TERMINAL ACTIVITY DECLARATION")
        .build());

    output.push_str("Filed by: [REDACTED per privacy protocol]\n");
    output.push_str(&format!("Case ID: {}\n", ctx.case_id));
    output.push_str(&format!("Current mood: {}\n", ctx.mood.label()));
    output.push('\n');

    output.push_str("Purpose of filing: ");
    output.push_str(*chaos.pick_unwrap(&[
        "Mandatory self-reporting",
        "Compliance with nonexistent policy",
        "Because the form exists",
        "Bureaucratic ritual observance",
    ]));
    output.push('\n');

    output.push_str("Activity summary: ");
    output.push_str(*chaos.pick_unwrap(&[
        "Extensive. Perhaps excessive.",
        "Within tolerable parameters.",
        "Notable but unremarkable.",
        "Recorded for posterity.",
    ]));
    output.push('\n');

    output.push_str("Justification: ");
    output.push_str(*chaos.pick_unwrap(&[
        "Unclear. Form is self-justifying.",
        "Not required. Form is compulsory.",
        "The justification is the filing itself.",
        "See attached (nothing attached).",
    ]));
    output.push('\n');

    output.push_str("Reviewer: None assigned. None required.\n");
    output.push_str("Status: Filed. Nothing will happen.\n\n");

    output.push_str(&Stamp::NullBureau.inline());
    output.push('\n');

    output
}

fn form_incident(ctx: &MoodContext, chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("FORM I-001: INCIDENT REPORT")
        .build());

    output.push_str(&format!("Case Reference: {}\n", ctx.case_id));
    output.push_str("Date of Incident: [AUTO-POPULATED]\n");
    output.push_str("Severity: ");
    output.push_str(*chaos.pick_unwrap(&[
        "UNDEFINED",
        "THEORETICALLY CONCERNING",
        "NOTABLE BUT CONTAINED",
        "BUREAUCRATICALLY SIGNIFICANT",
    ]));
    output.push('\n');
    output.push('\n');

    output.push_str("INCIDENT DESCRIPTION:\n");
    output.push_str("─".repeat(40).as_str());
    output.push('\n');

    let incident = chaos.pick_unwrap(&[
        "Something useful was accidentally accomplished.",
        "Productivity was briefly detected before containment.",
        "An insight occurred. Impact assessment ongoing.",
        "A feature worked on the first try. Investigation pending.",
    ]);
    output.push_str(&format!("{}\n", incident));

    output.push_str("─".repeat(40).as_str());
    output.push('\n');
    output.push('\n');

    output.push_str("IMMEDIATE ACTIONS TAKEN:\n");
    output.push_str("  [X] Incident documented\n");
    output.push_str("  [ ] Root cause analysis (pending)\n");
    output.push_str("  [ ] Preventive measures (unclear)\n");
    output.push('\n');

    output.push_str("FOLLOW-UP REQUIRED: No. This form is the follow-up.\n\n");

    output.push_str(&Stamp::Filed.inline());
    output.push('\n');

    output
}

fn form_requisition(chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("FORM R-404: RESOURCE REQUISITION")
        .build());

    output.push_str("Requestor: [CURRENT USER]\n");
    output.push_str("Department: Terminal Operations\n");
    output.push('\n');

    output.push_str("ITEMS REQUESTED:\n");

    let items = [
        "More time",
        "Fewer meetings",
        "Working code",
        "Documentation that makes sense",
        "A debugger that reads minds",
        "Coffee (infinite)",
        "Understanding",
    ];

    for (i, item) in items.iter().take(chaos.range(3, 5) as usize).enumerate() {
        output.push_str(&format!("  {}. {}\n", i + 1, item));
    }

    output.push('\n');
    output.push_str("JUSTIFICATION: It would help.\n\n");

    output.push_str("APPROVAL STATUS: ");
    output.push_str(*chaos.pick_unwrap(&[
        "PENDING (indefinitely)",
        "UNDER REVIEW (no reviewer assigned)",
        "FORWARDED (destination unknown)",
        "ACKNOWLEDGED (no action implied)",
    ]));
    output.push_str("\n\n");

    output.push_str("NOTE: This requisition will not be fulfilled.\n");
    output.push_str("      Filing it was the point.\n\n");

    output.push_str(&Stamp::Pending.inline());
    output.push('\n');

    output
}

fn form_appeal(chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("FORM A-000: APPEAL AGAINST DECISION")
        .build());

    output.push_str("Appellant: [CURRENT USER]\n");
    output.push_str("Decision Being Appealed: [NONE ON RECORD]\n");
    output.push('\n');

    output.push_str("GROUNDS FOR APPEAL:\n");
    output.push_str("─".repeat(40).as_str());
    output.push('\n');

    let grounds = chaos.pick_unwrap(&[
        "The decision was never made, yet its effects are felt.",
        "No one decided, but here we are.",
        "The outcome was predetermined by bureaucratic inertia.",
        "I disagree with the absence of a decision.",
    ]);
    output.push_str(&format!("{}\n", grounds));

    output.push_str("─".repeat(40).as_str());
    output.push('\n');
    output.push('\n');

    output.push_str("DESIRED OUTCOME:\n");
    output.push_str("  [ ] Reversal of non-decision\n");
    output.push_str("  [ ] Acknowledgment of appeal\n");
    output.push_str("  [X] Filing of appeal (accomplished)\n");
    output.push('\n');

    output.push_str("APPEAL STATUS: ");
    output.push_str(*chaos.pick_unwrap(&[
        "RECEIVED (not read)",
        "FILED (no appeals committee exists)",
        "ACKNOWLEDGED (acknowledgment non-binding)",
        "PENDING (permanently)",
    ]));
    output.push_str("\n\n");

    output.push_str("NEXT STEPS: None. The appeal is complete.\n");
    output.push_str("            Its completion is the resolution.\n\n");

    output.push_str(&Stamp::Denied.inline());
    output.push('\n');

    output
}

