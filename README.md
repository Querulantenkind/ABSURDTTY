# ABSURDTTY

A cargo workspace containing two CLI tools that treat your terminal like a stage instead of a factory floor.

## What is this?

ABSURDTTY consists of two programs:

- **tty-mood** — A local-first mood reader that analyzes your shell history and system state to generate mood signatures
- **noise** — An unhelpful instrument that responds to commands but never solves anything

Together they transform your terminal into something between a bureaucratic office, an art installation, and a diagnostic theater.

---

## Status

**Version:** 0.1.0 (pre-incident)  
**Stability:** Deterministic chaos  
**Utility:** Questionable by design  
**License:** MIT (use at your own philosophical risk)

---

## Philosophy

### ABSURDTTY refuses instrumental thinking

The terminal is typically treated as a productivity machine—a place where commands execute, problems get solved, and efficiency is measured. ABSURDTTY rejects this frame.

Instead, it treats the terminal as:
- A stage for performance
- A lab for experimentation  
- A registry office for bureaucratic ritual
- Occasionally, a haunted waiting room

### Core Principles

**1. No Objective Utility**  
If this project becomes useful, it must be reclassified as an incident and documented in `docs/INCIDENT_LOG.md`.

**2. Internal Consistency**  
Absurd, not random. Weird, not sloppy. Every behavior follows its own internal logic.

**3. Deterministic Absurdity**  
When seeded with `--seed`, all chaos becomes reproducible. This enables:
- Screenshots for documentation
- Demos that work twice
- Polite chaos for shared environments

**4. Local-First, Always**  
- No telemetry
- No uploads
- No tracking
- No cloud services
- Your terminal activity stays on your machine

### Explicit Non-Features

ABSURDTTY will never include:
- Productivity metrics
- Life optimization dashboards
- AI coaching
- Motivational advice
- Correctness guarantees
- Actionable insights
- Growth mindset framing

---

## Installation

### Prerequisites

- Rust 1.75+ (edition 2021)
- A Unix-like shell (bash, zsh, fish)
- A sense of humor (optional but recommended)

### From Source
```bash
git clone https://github.com/yourusername/absurdtty.git
cd absurdtty
cargo build --release

# Install binaries
cargo install --path crates/noise
cargo install --path crates/tty-mood

# Optional: Install man pages
sudo cp man/*.1 /usr/local/share/man/man1/
```

### Using Cargo
```bash
cargo install absurdtty-noise
cargo install absurdtty-tty-mood
```

---

## Quick Start

### Step 1: Generate a Mood Signature
```bash
# Analyze your shell history (read-only, local)
tty-mood generate

# Output: ~/.local/share/absurdtty/mood.json
```

This reads your shell history (default: last 7 days) and generates a mood signature based on patterns it detects.

### Step 2: Use noise
```bash
# Check system status
noise status

# List directory contents (but weird)
noise ls

# Get a diagnosis
noise doctor

# File a form
noise form

# Read system patchnotes
noise patchnotes
```

Without a mood signature, `noise` works but responds in a neutral, boring tone.  
With a mood signature, it adapts its behavior to your detected state.

---

## How It Works

### The Mood Signature

`tty-mood` analyzes your shell history and generates a local JSON file containing:

- **Mood ID**: A qualitative state (e.g., "feral_productivity", "exhausted", "bureaucratic_zen")
- **Signals**: Pattern scores (command cadence, diversity, typo rate, temporal patterns)
- **Confidence**: How certain the detection is (0.0 - 1.0)
- **Notes**: Human-readable observations

Example `mood.json`:
```json
{
  "schema": "absurdtty.mood.v1",
  "case_id": "AB-20251212-001",
  "generated_at": "2025-12-12T11:32:00+01:00",
  "range": "7d",
  "source": {
    "shell": "zsh",
    "history_path": "/home/user/.zsh_history",
    "read_only": true
  },
  "mood": {
    "id": "feral_productivity",
    "label": "feral productivity",
    "confidence": 0.74
  },
  "signals": [
    { "id": "cadence_high", "score": 0.81 },
    { "id": "command_diversity_high", "score": 0.77 },
    { "id": "typo_rate_medium", "score": 0.53 },
    { "id": "late_night_orbit", "score": 0.62 }
  ],
  "notes": [
    "velocity exceeds reflection quota",
    "curiosity spike detected",
    "status: non-binding"
  ]
}
```

### Communication Flow
```
tty-mood ──(mood.json)──▶ noise
```

1. `tty-mood` observes your shell history
2. Generates a mood signature (stored at `~/.local/share/absurdtty/mood.json`)
3. `noise` reads this file when invoked
4. Adapts its tone and output based on the detected mood

---

## Mood States

ABSURDTTY recognizes the following moods:

### feral_productivity
High command cadence, high diversity, late-night activity. Operator moving faster than reflection allows.

### exhausted  
Low cadence, high typo rate, repeated commands. System functional, operator questionable.

### methodical
Steady rhythm, low error rate, systematic patterns. Everything catalogued and verified.

### chaotic_neutral
High diversity, burst patterns, unpredictable timing. Entropy rising but controlled.

### bureaucratic_zen
Steady patterns, form-like command sequences, perfect adherence to ritual without attachment to outcome.

### ambient_drift
Many small commands, no clear direction. Present but unfocused.

### recursive_doubt
Repeated status checks, validation commands, uncertainty loops.

### emergency_mode
Fast bursts, high error rate, correction patterns. Crisis management in progress.

---

## Commands Reference

### noise status

Shows current mood and case information.
```bash
noise status
```

**Output with mood "exhausted":**
```
CASE: AB-20251212-001
MOOD: exhausted (confidence: regrettably high)
NOTES: velocity has collapsed
       reflection quota: exceeded by default
STATUS: operational but barely
```

**Output without mood:**
```
System operational.
```

---

### noise ls [path]

Lists directory contents with mood-influenced presentation.
```bash
noise ls /home
noise ls          # current directory
```

**Behavior by mood:**

**feral_productivity:**
```
usr  documetns  downloads  [scanning...]  uh  stuff  more_stuff
NOTE: Inventory incomplete. Operator moving too fast for census.
```

**exhausted:**
```
user  documents  do...
[OUTPUT TRUNCATED: Energy budget exceeded]
```

**methodical:**
```
DIRECTORY CONTENTS (alpha-sorted, verified):
  - documents/ (last accessed: 2025-12-11)
  - downloads/ (recommended cleanup: overdue)
  - user/ (classification: primary)
TOTAL: 3 items catalogued
```

**bureaucratic_zen:**
```
FORM LS-001: DIRECTORY CONTENTS DECLARATION
Filed: 2025-12-12 11:45:23
Items found: 3 (three)
  1. user (STATUS: present)
  2. documents (STATUS: present)
  3. downloads (STATUS: present)
Declaration complete. No action required.
```

---

### noise uptime

Reports system uptime with philosophical commentary.
```bash
noise uptime
```

**Output with mood "exhausted":**
```
SYSTEM UPTIME: 14 days, 7 hours, 23 minutes
USER UPTIME: [REDACTED]
DISCREPANCY: Concerning. Machine outlasting operator.
RECOMMENDATION: Role reversal advised.
```

---

### noise doctor [--verbose]

Provides diagnostic assessment of user/system state.
```bash
noise doctor
noise doctor --verbose
```

**Output with mood "feral_productivity":**
```
PATIENT FILE: AB-20251212-001
SYMPTOMS OBSERVED:
  - Elevated command cadence (81% above baseline)
  - Late-night orbital pattern detected
  - Typo rate within feral parameters
  - Curiosity spike: confirmed

DIAGNOSIS: Acute productivity mania
PRESCRIPTION: None. This is not a medical facility.
              However, consider: sleep exists.
PROGNOSIS: Continued operation until collapse or insight.
           Whichever arrives first.
NOTES: Patient unlikely to follow recommendations.
       Recommendations therefore not binding.
```

---

### noise explain <command>

Explains any command in deeply unhelpful ways.
```bash
noise explain git push
noise explain rm -rf
noise explain cd
```

**Example:**
```bash
$ noise explain rm -rf

EXPLANATION: The irreversible gesture.
MECHANISM: Asks no questions. Provides no confirmations.
           Trusts you completely. This is its weakness.
HISTORICAL NOTE: Has ended more careers than any other 8 characters.
RECOMMENDED USE: Never, unless absolutely certain.
                 Then reconsider.
SAFETY: None. That is the point.
```

---

### noise form [--template <name>]

Generates bureaucratic forms for self-reporting and documentation.

**Templates:**
- `declaration` — Self-reporting form
- `incident` — Log an incident  
- `requisition` — Request something (never approved)
- `appeal` — Appeal a decision (no decision exists)
```bash
noise form
noise form --template incident
```

**Example output:**
```
╔════════════════════════════════════════╗
║  FORM 27-B: TERMINAL ACTIVITY DECLARATION  ║
╚════════════════════════════════════════╝
Filed by: [REDACTED per privacy protocol]
Case ID: AB-20251212-001
Current mood: feral productivity
Purpose of filing: Mandatory self-reporting
Activity summary: Extensive. Perhaps excessive.
Justification: Unclear. Form is self-justifying.
Reviewer: None assigned. None required.
Status: Filed. Nothing will happen.

[STAMP: NULL BUREAU - FORM RECEIVED BUT NOT READ]
```

---

### noise patchnotes [--since <date>]

Displays fake changelog/patchnotes for your terminal system.
```bash
noise patchnotes
noise patchnotes --since 2025-12-01
```

**Example output:**
```
╔════════════════════════════════════════╗
║  TERMINAL SYSTEM PATCHNOTES v2025.12.12  ║
╚════════════════════════════════════════╝

MOOD ENGINE:
  + Added: 'feral_productivity' mood state
  - Removed: 'calm_confidence' (user never achieved it)
  * Fixed: Occasional honesty in status reports

SIGNAL DETECTION:
  + Added: 'late_night_orbit' pattern recognition
  * Improved: Typo rate now 23% more judgmental

OUTPUT FORMATTING:
  - Fixed: Excessive clarity in error messages
  + Added: More ambiguity in success confirmations
  * Changed: Timestamps now occasionally philosophical

KNOWN ISSUES:
  - User expectations still calibrated incorrectly
  - Hope persists despite contrary evidence
  - Reality checks fail more often than they should

BREAKING CHANGES:
  - None. This was always broken in this specific way.

DEPRECATION NOTICE:
  - Meaning (scheduled removal: TBD)
```

---

## tty-mood Usage

### Generate Mood Signature
```bash
# Default: last 7 days, output to ~/.local/share/absurdtty/mood.json
tty-mood generate

# Specify range
tty-mood generate --range 14d

# Custom output path
tty-mood generate --out /tmp/mood.json

# With seed for reproducibility
tty-mood generate --seed 42
```

### Show Current Mood
```bash
tty-mood show
```

Displays the current mood signature in human-readable format.

### List Detected Signals
```bash
tty-mood signals
```

Shows all detected pattern signals with scores.

---

## Configuration

### Mood File Location

Default: `~/.local/share/absurdtty/mood.json`

Override with:
```bash
export ABSURDTTY_MOOD_PATH="/custom/path/mood.json"
```

Or specify per-command:
```bash
noise status --mood-file /tmp/mood.json
```

### Shell History Paths

`tty-mood` auto-detects:
- bash: `~/.bash_history`
- zsh: `~/.zsh_history`
- fish: `~/.local/share/fish/fish_history`

Override with:
```bash
tty-mood generate --history ~/.custom_history
```

---

## Architecture

### Workspace Structure
```
absurdtty/
├─ crates/
│  ├─ absurd-core/      # Shared utilities (seed, format, fs_safety)
│  ├─ absurd-lexicon/   # Shared language (moods, tones)
│  ├─ noise/            # CLI tool: responds to commands
│  └─ tty-mood/         # CLI tool: generates mood signatures
├─ docs/
│  ├─ MANIFESTO.md
│  ├─ CLI_SPEC.md
│  └─ INCIDENT_LOG.md   # When usefulness occurs
└─ man/                 # Man pages for both tools
```

### Crate Responsibilities

**absurd-core**
- `seed.rs` — Deterministic randomness for reproducible chaos
- `format.rs` — Shared output formatting (stamps, boxes, tables)
- `fs_safety.rs` — Safe local file operations

**absurd-lexicon**
- `moods.rs` — Mood state definitions and matching
- `tone.rs` — Tone transformation rules per mood

**noise**
- `main.rs` — CLI entry point
- `commands/` — Individual command implementations

**tty-mood**
- `history/` — Shell history parsing and pattern detection
- `report/` — Mood signature generation and rendering

---

## Signal Detection

`tty-mood` analyzes shell history for these signal types:

### Frequency Patterns
- **cadence_high/low** — Commands per hour
- **burst_pattern** — Activity spikes
- **steady_rhythm** — Consistent distribution

### Temporal Patterns  
- **late_night_orbit** — 22:00 - 04:00 activity
- **early_morning_surge** — 05:00 - 07:00 activity
- **weekend_anomaly** — Weekend vs weekday patterns
- **lunch_void** — 12:00 - 13:00 absence

### Error Patterns
- **typo_rate_high/medium/low** — Misspelled commands
- **repeat_commands** — Same command multiple times
- **correction_pattern** — Command followed by corrected version

### Diversity Patterns
- **command_diversity_high** — Many different tools used
- **tool_fixation** — Single tool dominates
- **context_switching** — Frequent directory/project changes

---

## Reproducibility

### Using Seeds

For reproducible output (demos, screenshots, testing):
```bash
tty-mood generate --seed 42
noise status --seed 42
noise ls --seed 42
```

Same seed + same mood signature + same system state = same output.

---

## Privacy & Data

### What Gets Read
- Your shell history (read-only)
- Timestamps of commands
- Command names (not arguments)

### What Gets Stored
- A single local JSON file with pattern analysis
- No command arguments
- No file paths
- No personal identifiable information

### What Gets Sent
- Nothing. Ever. No network requests.

---

## Development

### Build
```bash
cargo build
```

### Test
```bash
cargo test --workspace
```

### Format
```bash
cargo fmt --all
```

### Lint
```bash
cargo clippy --workspace -- -D warnings
```

### Using Just
```bash
just build
just test
just lint
just install
```

See `justfile` for all available commands.

---

## Contributing

### Before You Submit

1. Read `docs/MANIFESTO.md` — understand the philosophy
2. Check `docs/INCIDENT_LOG.md` — has someone accidentally made this useful?
3. Run `just lint` — maintain code quality
4. Add tests if adding signal detection or mood logic
5. Update man pages if changing command interfaces

### What We Accept

- New mood states (with clear detection logic)
- New signal patterns (with tests)
- New commands for `noise` (if sufficiently absurd)
- Bug fixes (but verify it's actually a bug, not a feature)
- Documentation improvements

### What We Reject

- Features that optimize productivity
- Metrics dashboards
- Cloud integrations
- Telemetry of any kind
- Anything that makes this "useful" in the conventional sense

---

## Troubleshooting

### "No mood signature found"

Run `tty-mood generate` first. `noise` works without it but is boring.

### "Shell history not found"

Specify manually:
```bash
tty-mood generate --history ~/.your_history_file
```

### "Permission denied reading history"

Ensure your history file is readable:
```bash
chmod 644 ~/.zsh_history
```

### "Output is too normal"

This is the worst bug. Please file an issue immediately in `docs/INCIDENT_LOG.md`.

---

## Documentation

- **Manifesto**: `docs/MANIFESTO.md` — Full philosophical framework
- **CLI Spec**: `docs/CLI_SPEC.md` — Detailed command specifications  
- **Incident Log**: `docs/INCIDENT_LOG.md` — When things become useful
- **Man Pages**: `man noise.1` and `man tty-mood.1`

---

## License

MIT License — use at your own philosophical risk.

See `LICENSE` for full text.

---

## Acknowledgments

This project stands on the shoulders of:
- Every terminal tool that took itself too seriously
- Unix philosophy (which we respect by deliberately misunderstanding)
- The bureaucratic form as an art medium
- Everyone who has ever typed `git status` 47 times in a row

---

## Questions That May Arise

**Q: Is this useful?**  
A: If you find it useful, please document this incident in `docs/INCIDENT_LOG.md`.

**Q: Should I use this in production?**  
A: Define "production." Define "use." Consider whether you should reconsider the question.

**Q: Can I integrate this with my CI/CD pipeline?**  
A: You can. Whether you should is between you and your conscience.

**Q: Does this violate my company's acceptable use policy?**  
A: Probably. But so does reading this README on company time.

**Q: Will you add feature X?**  
A: Only if it makes things worse in an interesting way.

**Q: Is this art or software?**  
A: Yes.

---

## Final Note

ABSURDTTY is an experiment in treating computation as performance rather than problem-solving. It asks: what happens when we build tools that respond instead of resolve? That observe instead of optimize? That file forms no one will read?

The answer, we've found, is surprisingly compelling chaos.

Your terminal is a stage. We're just providing the props.

---

**Version:** 0.1.0  
**Status:** Pre-incident  
**Maintained by:** The Null Bureau  
**Stamp:** Filed but not read
