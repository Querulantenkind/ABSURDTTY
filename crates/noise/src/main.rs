//! noise - An unhelpful instrument.
//!
//! Responds to commands but never solves anything.

use clap::Parser;

#[derive(Parser)]
#[command(name = "noise")]
#[command(version, about = "An unhelpful instrument that responds to commands but never solves anything")]
struct Cli {
    /// Seed for reproducible chaos
    #[arg(long)]
    seed: Option<u64>,
}

fn main() {
    let _cli = Cli::parse();

    println!("noise: Not yet implemented.");
    println!("The void awaits.");
}

