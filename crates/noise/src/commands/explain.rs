//! The `noise explain` command.
//!
//! Explains commands in deeply unhelpful ways.

use crate::mood_reader::MoodContext;
use absurd_core::Chaos;
use anyhow::Result;

/// Execute the explain command.
pub fn cmd_explain(ctx: &MoodContext, chaos: &mut Chaos, command: &[String]) -> Result<String> {
    if command.is_empty() {
        return Ok("EXPLANATION: You asked me to explain nothing.\nThis is the most honest request I've received.\n".to_string());
    }

    let cmd_str = command.join(" ");
    let cmd_name = command[0].as_str();

    let output = match cmd_name {
        "rm" => explain_rm(chaos, &cmd_str),
        "sudo" => explain_sudo(chaos),
        "git" => explain_git(chaos, command),
        "cd" => explain_cd(chaos),
        "ls" => explain_ls(chaos),
        "cat" => explain_cat(chaos),
        "grep" => explain_grep(chaos),
        "find" => explain_find(chaos),
        "curl" => explain_curl(chaos),
        "chmod" => explain_chmod(chaos),
        "kill" => explain_kill(chaos),
        "man" => explain_man(chaos),
        "exit" | "logout" => explain_exit(chaos),
        _ => explain_generic(chaos, cmd_name, ctx),
    };

    Ok(output)
}

fn explain_rm(chaos: &mut Chaos, cmd_str: &str) -> String {
    let mut output = String::new();

    if cmd_str.contains("-rf") || cmd_str.contains("-fr") {
        output.push_str("EXPLANATION: The irreversible gesture.\n\n");
        output.push_str("MECHANISM: Asks no questions. Provides no confirmations.\n");
        output.push_str("           Trusts you completely. This is its weakness.\n\n");
        output.push_str("HISTORICAL NOTE: Has ended more careers than any other 8 characters.\n\n");
        output.push_str("RECOMMENDED USE: Never, unless absolutely certain.\n");
        output.push_str("                 Then reconsider.\n\n");
        output.push_str("SAFETY: None. That is the point.\n");
    } else {
        let metaphor = chaos.pick_unwrap(&[
            "Digital cremation.",
            "The delete key, but committed.",
            "Entropy's friend.",
            "Marie Kondo for filesystems.",
        ]);
        output.push_str(&format!("EXPLANATION: {}\n\n", metaphor));
        output.push_str("WHAT IT DOES: Removes things. Permanently.\n");
        output.push_str("WHAT IT DOESN'T DO: Ask twice. Forgive.\n");
    }

    output
}

fn explain_sudo(chaos: &mut Chaos) -> String {
    let mut output = String::new();

    output.push_str("EXPLANATION: The magic word for adults.\n\n");

    let observation = chaos.pick_unwrap(&[
        "Temporarily grants you the powers of someone more trusted.",
        "The system says 'are you sure?' This says 'I am sure.'",
        "Power without accountability. Use wisely.",
        "Your password is the key. The system is the lock. sudo picks it.",
    ]);

    output.push_str(&format!("OBSERVATION: {}\n\n", observation));
    output.push_str("COMMON USAGE: When permission is denied but confidence is not.\n");
    output.push_str("WARNING: With great power comes great opportunity for mistakes.\n");

    output
}

fn explain_git(chaos: &mut Chaos, command: &[String]) -> String {
    let mut output = String::new();

    let subcommand = command.get(1).map(|s| s.as_str());

    match subcommand {
        Some("push") => {
            output.push_str("EXPLANATION: Sharing your problems with others.\n\n");
            output.push_str("MECHANISM: Transmits your local confusion to a remote location.\n");
            output.push_str("CONSEQUENCE: Now it's everyone's problem.\n");
        }
        Some("pull") => {
            output.push_str("EXPLANATION: Importing other people's problems.\n\n");
            output.push_str("MECHANISM: Downloads confusion from elsewhere.\n");
            output.push_str("HOPE: That their confusion is compatible with yours.\n");
        }
        Some("commit") => {
            output.push_str("EXPLANATION: A promise. To yourself. That you'll explain later.\n\n");
            let message = chaos.pick_unwrap(&[
                "The message is a lie you tell your future self.",
                "'-m \"fix\"' - the most honest message.",
                "Future you will not understand. Current you barely does.",
            ]);
            output.push_str(&format!("NOTE: {}\n", message));
        }
        Some("status") => {
            output.push_str("EXPLANATION: Asking git if you've messed up yet.\n\n");
            output.push_str("FREQUENCY OF USE: Directly proportional to anxiety.\n");
            output.push_str("USEFUL INFORMATION PROVIDED: Sometimes.\n");
            output.push_str("PEACE OF MIND PROVIDED: Rarely.\n");
        }
        Some("rebase") => {
            output.push_str("EXPLANATION: Lying about history.\n\n");
            output.push_str("MECHANISM: Pretends your commits happened differently.\n");
            output.push_str("ETHICS: Debatable.\n");
            output.push_str("CONSEQUENCES: Often surprising.\n");
        }
        _ => {
            let observation = chaos.pick_unwrap(&[
                "A distributed system for managing regret.",
                "Version control: the idea that past mistakes should be preserved.",
                "Time travel for code. Side effects: merge conflicts.",
                "The answer to 'what did I change?' (Usually.)",
            ]);
            output.push_str(&format!("EXPLANATION: {}\n", observation));
        }
    }

    output
}

fn explain_cd(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Movement without progress.",
        "The illusion of getting somewhere.",
        "Directory tourism.",
        "Changing location. Rarely changing outcome.",
    ]);

    format!("EXPLANATION: {}\n\nYou are now somewhere else.\nThe problems followed.\n", observation)
}

fn explain_ls(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Looking at what you have. Deciding what to do: delayed.",
        "The first step of any plan that leads nowhere.",
        "Proof that files exist. Action: pending.",
        "Reconnaissance without mission.",
    ]);

    format!("EXPLANATION: {}\n", observation)
}

fn explain_cat(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Reading without understanding.",
        "The file's contents, raw and unforgiving.",
        "Looking at text. Comprehension: optional.",
        "Named after an animal that doesn't come when called. Fitting.",
    ]);

    format!("EXPLANATION: {}\n", observation)
}

fn explain_grep(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Finding needles in haystacks. The haystack: your codebase.",
        "Proof that the thing you're looking for exists. Somewhere.",
        "Search. Hopefully find. Context: minimal.",
        "Regular expressions: the language of mild insanity.",
    ]);

    format!("EXPLANATION: {}\n\nTIP: If the regex works first try, be suspicious.\n", observation)
}

fn explain_find(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Searching for files. Finding more than expected. Always.",
        "Like grep, but for existence rather than content.",
        "The -exec flag: where things get interesting. Or dangerous.",
    ]);

    format!("EXPLANATION: {}\n\nNOTE: The thing you're looking for is in the last place you look.\n      Because you stop looking after that.\n", observation)
}

fn explain_curl(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Asking the internet for things. Politely.",
        "HTTP: the language of machines talking to machines.",
        "What could go wrong? (Timeouts. Timeouts could go wrong.)",
    ]);

    format!("EXPLANATION: {}\n\nWARNING: curl | bash is a trust exercise.\n         Do you trust the internet?\n", observation)
}

fn explain_chmod(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Permission management. The numbers mean things.",
        "777: The 'I give up on security' option.",
        "Deciding who can do what to whom.",
    ]);

    format!("EXPLANATION: {}\n\nCOMMON USAGE: chmod +x thing.sh, then wondering why it doesn't work.\n", observation)
}

fn explain_kill(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Asking a process to stop. Firmly.",
        "Signal delivery. Some signals are suggestions. -9 is not.",
        "The nuclear option for misbehaving software.",
    ]);

    format!("EXPLANATION: {}\n\nNOTE: kill -9: When you really mean it.\n      The process doesn't get to argue.\n", observation)
}

fn explain_man(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "The manual. Which you're reading instead of experimenting.",
        "Documentation written by people who already understand.",
        "RTFM incarnate.",
    ]);

    format!("EXPLANATION: {}\n\nIRONY: You're using noise explain instead.\n", observation)
}

fn explain_exit(chaos: &mut Chaos) -> String {
    let observation = chaos.pick_unwrap(&[
        "Leaving. The terminal will miss you.",
        "The correct response to most situations.",
        "Acknowledgment that you're done. (Are you?)",
    ]);

    format!("EXPLANATION: {}\n\nNOTE: There is no undo. But you can always come back.\n", observation)
}

fn explain_generic(chaos: &mut Chaos, cmd: &str, _ctx: &MoodContext) -> String {
    let observations = [
        format!("'{}': A command. It does things.", cmd),
        format!("'{}': Presumably useful. To someone.", cmd),
        format!("'{}': The documentation exists. Somewhere.", cmd),
        format!("'{}': Type it and find out.", cmd),
        format!("'{}': I'm sure you know what you're doing.", cmd),
    ];

    let mut v = observations.to_vec();
    chaos.shuffle(&mut v);

    format!("EXPLANATION: {}\n\nFor actual help, try 'man {}' or '{} --help'.\n", v[0], cmd, cmd)
}

