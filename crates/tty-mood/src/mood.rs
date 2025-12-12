//! Mood detection from signals.
//!
//! Maps signal patterns to mood states using weighted scoring.

use crate::signals::SignalCollection;
use absurd_lexicon::moods::{Mood, MoodId};

/// Detect mood from analyzed signals.
pub fn detect_mood(signals: &SignalCollection) -> Mood {
    // Score each mood based on signal presence and strength
    let scores: Vec<(MoodId, f64)> = vec![
        (MoodId::FeralProductivity, score_feral_productivity(signals)),
        (MoodId::Exhausted, score_exhausted(signals)),
        (MoodId::Methodical, score_methodical(signals)),
        (MoodId::ChaoticNeutral, score_chaotic_neutral(signals)),
        (MoodId::BureaucraticZen, score_bureaucratic_zen(signals)),
        (MoodId::AmbientDrift, score_ambient_drift(signals)),
        (MoodId::RecursiveDoubt, score_recursive_doubt(signals)),
        (MoodId::EmergencyMode, score_emergency_mode(signals)),
    ];

    // Find highest scoring mood
    let (best_mood, best_score) = scores
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap_or((MoodId::Neutral, 0.0));

    // Require minimum confidence
    if best_score < 0.3 {
        return Mood::neutral().with_note("Insufficient signal strength for classification");
    }

    let mut mood = Mood::new(best_mood, best_score);

    // Add relevant notes based on strong signals
    for signal in signals.strong_signals() {
        if let Some(note) = &signal.note {
            mood = mood.with_note(note.clone());
        }
    }

    mood
}

/// Score for feral_productivity mood.
///
/// High cadence + high diversity + late night activity
fn score_feral_productivity(signals: &SignalCollection) -> f64 {
    let cadence_high = signals.score("cadence_high");
    let diversity_high = signals.score("command_diversity_high");
    let late_night = signals.score("late_night_orbit");
    let burst = signals.score("burst_pattern");

    // Weighted combination
    let base = cadence_high * 0.35 + diversity_high * 0.25 + late_night * 0.2 + burst * 0.2;

    // Penalty for too many errors (feral but not sloppy)
    let typo_penalty = signals.score("typo_rate_high") * 0.3;

    (base - typo_penalty).max(0.0)
}

/// Score for exhausted mood.
///
/// Low cadence + high typo rate + repeated commands
fn score_exhausted(signals: &SignalCollection) -> f64 {
    let cadence_low = signals.score("cadence_low");
    let typo_high = signals.score("typo_rate_high");
    let typo_medium = signals.score("typo_rate_medium");
    let repeats = signals.score("repeat_commands");
    let late_night = signals.score("late_night_orbit");

    // Exhaustion indicators
    let typo_score = typo_high * 0.8 + typo_medium * 0.4;

    cadence_low * 0.3 + typo_score * 0.3 + repeats * 0.2 + late_night * 0.2
}

/// Score for methodical mood.
///
/// Steady rhythm + low error rate + systematic patterns
fn score_methodical(signals: &SignalCollection) -> f64 {
    let rhythm = signals.score("steady_rhythm");
    let typo_low = signals.score("typo_rate_low");
    let build_cycle = signals.score("build_cycle");
    let git_heavy = signals.score("git_heavy");

    // Bonus for consistent tool usage
    let workflow_bonus = (build_cycle + git_heavy) * 0.5;

    // Penalty for chaos indicators
    let chaos_penalty = signals.score("burst_pattern") * 0.2
        + signals.score("context_switching") * 0.2;

    (rhythm * 0.4 + typo_low * 0.2 + workflow_bonus * 0.2 - chaos_penalty).max(0.0)
}

/// Score for chaotic_neutral mood.
///
/// High diversity + burst patterns + unpredictable timing
fn score_chaotic_neutral(signals: &SignalCollection) -> f64 {
    let diversity = signals.score("command_diversity_high");
    let burst = signals.score("burst_pattern");
    let context_switch = signals.score("context_switching");
    let time_spread = signals.score("time_spread");

    // Chaos without complete disaster
    let base = diversity * 0.3 + burst * 0.25 + context_switch * 0.25 + time_spread * 0.2;

    // Penalty for too orderly
    let order_penalty = signals.score("steady_rhythm") * 0.3;

    (base - order_penalty).max(0.0)
}

/// Score for bureaucratic_zen mood.
///
/// Steady patterns + form-like sequences + git operations
fn score_bureaucratic_zen(signals: &SignalCollection) -> f64 {
    let rhythm = signals.score("steady_rhythm");
    let git_heavy = signals.score("git_heavy");
    let typo_low = signals.score("typo_rate_low");
    let weekday = signals.score("weekday_bound");

    // Bureaucratic: orderly, git-focused, working hours
    let base = rhythm * 0.3 + git_heavy * 0.3 + typo_low * 0.2 + weekday * 0.2;

    // Penalty for chaos
    let chaos_penalty = signals.score("late_night_orbit") * 0.2;

    (base - chaos_penalty).max(0.0)
}

/// Score for ambient_drift mood.
///
/// Low diversity + low cadence + no clear direction
fn score_ambient_drift(signals: &SignalCollection) -> f64 {
    let diversity_low = signals.score("command_diversity_low");
    let cadence_low = signals.score("cadence_low");
    let status_checks = signals.score("status_check_loop");

    // Drift: present but unfocused
    diversity_low * 0.35 + cadence_low * 0.35 + status_checks * 0.3
}

/// Score for recursive_doubt mood.
///
/// Repeated status checks + validation commands + uncertainty loops
fn score_recursive_doubt(signals: &SignalCollection) -> f64 {
    let status_loop = signals.score("status_check_loop");
    let repeats = signals.score("repeat_commands");
    let corrections = signals.score("correction_pattern");
    let git_heavy = signals.score("git_heavy");

    // Doubt: checking, rechecking, uncertainty
    let base = status_loop * 0.35 + repeats * 0.25 + corrections * 0.2;

    // Git status spam is a strong indicator
    let git_bonus = if status_loop > 0.5 && git_heavy > 0.5 {
        0.2
    } else {
        0.0
    };

    (base + git_bonus + git_heavy * 0.1).min(1.0)
}

/// Score for emergency_mode mood.
///
/// Fast bursts + high error rate + crisis patterns
fn score_emergency_mode(signals: &SignalCollection) -> f64 {
    let burst = signals.score("burst_pattern");
    let typo_high = signals.score("typo_rate_high");
    let corrections = signals.score("correction_pattern");
    let cadence_high = signals.score("cadence_high");

    // Emergency: fast, frantic, error-prone
    let base = burst * 0.3 + typo_high * 0.3 + corrections * 0.2 + cadence_high * 0.2;

    // Must have multiple indicators to count as emergency
    let indicator_count = [burst > 0.3, typo_high > 0.3, corrections > 0.2, cadence_high > 0.5]
        .iter()
        .filter(|&&x| x)
        .count();

    if indicator_count < 2 {
        0.0
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signals::Signal;

    fn signals_with(pairs: &[(&str, f64)]) -> SignalCollection {
        let mut signals = SignalCollection::new();
        for (id, score) in pairs {
            signals.add(Signal::new(*id, *score));
        }
        signals
    }

    #[test]
    fn feral_productivity_detected() {
        let signals = signals_with(&[
            ("cadence_high", 0.8),
            ("command_diversity_high", 0.7),
            ("late_night_orbit", 0.6),
        ]);

        let mood = detect_mood(&signals);
        assert_eq!(mood.id, MoodId::FeralProductivity);
        assert!(mood.confidence > 0.5);
    }

    #[test]
    fn exhausted_detected() {
        let signals = signals_with(&[
            ("cadence_low", 0.8),
            ("typo_rate_high", 0.7),
            ("repeat_commands", 0.5),
        ]);

        let mood = detect_mood(&signals);
        assert_eq!(mood.id, MoodId::Exhausted);
    }

    #[test]
    fn neutral_when_no_signals() {
        let signals = SignalCollection::new();
        let mood = detect_mood(&signals);
        assert_eq!(mood.id, MoodId::Neutral);
    }

    #[test]
    fn recursive_doubt_from_status_checks() {
        let signals = signals_with(&[
            ("status_check_loop", 0.8),
            ("repeat_commands", 0.6),
            ("git_heavy", 0.7),
        ]);

        let mood = detect_mood(&signals);
        assert_eq!(mood.id, MoodId::RecursiveDoubt);
    }
}

