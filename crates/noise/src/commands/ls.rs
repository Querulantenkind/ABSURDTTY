//! The `noise ls` command.
//!
//! Lists directory contents with mood-adapted presentation.

#![allow(clippy::explicit_auto_deref)]

use crate::mood_reader::MoodContext;
use absurd_core::format::{BoxBuilder, BoxStyle, truncate};
use absurd_core::Chaos;
use absurd_lexicon::moods::MoodId;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Execute the ls command.
pub fn cmd_ls(ctx: &MoodContext, chaos: &mut Chaos, path: Option<&Path>) -> Result<String> {
    let path = path.unwrap_or(Path::new("."));

    // Read directory entries
    let entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();

    if !ctx.has_mood {
        return Ok(boring_ls(&entries));
    }

    let output = match ctx.mood.id {
        MoodId::FeralProductivity => ls_feral(&entries, chaos),
        MoodId::Exhausted => ls_exhausted(&entries, chaos),
        MoodId::Methodical => ls_methodical(&entries, path),
        MoodId::ChaoticNeutral => ls_chaotic(&entries, chaos),
        MoodId::BureaucraticZen => ls_bureaucratic(&entries, path),
        MoodId::AmbientDrift => ls_drift(&entries, chaos),
        MoodId::RecursiveDoubt => ls_doubt(&entries, chaos),
        MoodId::EmergencyMode => ls_emergency(&entries, chaos),
        MoodId::Neutral => boring_ls(&entries),
    };

    Ok(output)
}

fn boring_ls(entries: &[fs::DirEntry]) -> String {
    entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join("  ")
        + "\n"
}

fn ls_feral(entries: &[fs::DirEntry], chaos: &mut Chaos) -> String {
    let mut output = String::new();
    let mut names: Vec<_> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    // Maybe shuffle a bit
    if chaos.chance(0.3) {
        chaos.shuffle(&mut names);
    }

    // Fast output, possibly incomplete
    let shown = if chaos.chance(0.4) {
        names.len().min(names.len() - 1).max(3)
    } else {
        names.len()
    };

    for name in names.iter().take(shown) {
        output.push_str(name);
        output.push_str("  ");
    }

    if shown < names.len() {
        output.push_str("[scanning...]  ");
    }

    output.push('\n');
    output.push_str("NOTE: Inventory ");
    output.push_str(*chaos.pick_unwrap(&[
        "incomplete. Operator moving too fast for census.",
        "approximate. Precision sacrificed for velocity.",
        "good enough. Moving on.",
    ]));
    output.push('\n');

    output
}

fn ls_exhausted(entries: &[fs::DirEntry], chaos: &mut Chaos) -> String {
    let mut output = String::new();
    let names: Vec<_> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    // Only show a few
    let show_count = chaos.range(2, 4) as usize;

    for (i, name) in names.iter().enumerate() {
        if i >= show_count {
            break;
        }
        output.push_str(&truncate(name, 15));
        output.push_str("  ");
    }

    if names.len() > show_count {
        output.push_str("...\n");
        output.push_str("[OUTPUT TRUNCATED: Energy budget exceeded]\n");
    } else {
        output.push('\n');
    }

    output
}

fn ls_methodical(entries: &[fs::DirEntry], path: &Path) -> String {
    let mut output = String::new();

    output.push_str(&format!("DIRECTORY CONTENTS ({}):\n", path.display()));
    output.push_str("─".repeat(40).as_str());
    output.push('\n');

    let mut names: Vec<_> = entries
        .iter()
        .map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
            (name, is_dir)
        })
        .collect();

    names.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    for (name, is_dir) in &names {
        let suffix = if *is_dir { "/" } else { "" };
        let kind = if *is_dir { "DIR " } else { "FILE" };
        output.push_str(&format!("  [{}] {}{}\n", kind, name, suffix));
    }

    output.push_str("─".repeat(40).as_str());
    output.push('\n');
    output.push_str(&format!("TOTAL: {} items catalogued\n", names.len()));

    output
}

fn ls_chaotic(entries: &[fs::DirEntry], chaos: &mut Chaos) -> String {
    let mut output = String::new();
    let mut names: Vec<_> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    // Definitely shuffle
    chaos.shuffle(&mut names);

    for name in &names {
        output.push_str(name);
        output.push_str(*chaos.pick_unwrap(&["  ", " ", "   ", "    "]));
    }

    output.push('\n');

    if chaos.chance(0.3) {
        output.push_str("(order: arbitrary)\n");
    }

    output
}

fn ls_bureaucratic(entries: &[fs::DirEntry], path: &Path) -> String {
    let mut output = String::new();

    output.push_str(&BoxBuilder::new()
        .style(BoxStyle::Double)
        .title("FORM LS-001: DIRECTORY CONTENTS DECLARATION")
        .line(format!("Location: {}", path.display()))
        .build());

    let mut names: Vec<_> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();

    output.push_str(&format!("Items found: {} ({})\n\n", names.len(), spell_number(names.len())));

    for (i, name) in names.iter().enumerate() {
        output.push_str(&format!("  {}. {} (STATUS: present)\n", i + 1, name));
    }

    output.push_str("\nDeclaration complete. No action required.\n");

    output
}

fn ls_drift(entries: &[fs::DirEntry], chaos: &mut Chaos) -> String {
    let mut output = String::new();
    let names: Vec<_> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    for name in &names {
        output.push_str(name);
        output.push('\n');
        if chaos.chance(0.1) {
            output.push_str("  ...\n");
        }
    }

    output
}

fn ls_doubt(entries: &[fs::DirEntry], chaos: &mut Chaos) -> String {
    let mut output = String::new();
    let names: Vec<_> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    for name in &names {
        output.push_str(name);
        if chaos.chance(0.2) {
            output.push_str(" (?)");
        }
        output.push('\n');
    }

    output.push_str(&format!("\n{} items. Probably. You should check.\n", names.len()));

    output
}

fn ls_emergency(entries: &[fs::DirEntry], _chaos: &mut Chaos) -> String {
    let mut output = String::new();
    let names: Vec<_> = entries
        .iter()
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    // Fast, minimal output
    output.push_str(&names.join(" "));
    output.push('\n');
    output.push_str(&format!("[{} items]\n", names.len()));

    output
}

fn spell_number(n: usize) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "many",
    }
}

