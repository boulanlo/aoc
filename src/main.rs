use std::sync::Arc;

use aoc::{messenger, AdventOfCode, Challenge, DataConfiguration, MessengerReceiver};
use clap::{Parser, Subcommand};
use color_eyre::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The year to display
    #[arg()]
    year: u32,
    /// Specific action to take
    #[command(subcommand)]
    command: Option<Command>,
    /// The directory in which to look for (and potentially download) dataset input
    #[arg(short, value_name = "DATA_PATH", default_value_t = String::from("./data"))]
    data_dir: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        /// The day to run
        #[arg()]
        day: u8,
        /// The specific part to run
        #[arg(short, long)]
        part: Option<u8>,
    },
}

fn display_challenge_results(part: u8, result: String, rx: &mut MessengerReceiver) -> Result<()> {
    println!("Part {part} report:\nStdout:");
    for line in rx.receive_stdout()? {
        println!("{line}");
    }
    println!("\nStderr:");
    for line in rx.receive_stderr()? {
        println!("{line}");
    }
    println!("\nResult: {result}\n");

    Ok(())
}

fn run_challenge(challenge: Arc<dyn Challenge + Send + Sync>, part: Option<u8>) -> Result<()> {
    let (mut messenger, mut messenger_rx) = messenger();

    if let Some(part) = part {
        match part {
            1 => {
                let result = challenge.part_1_verified(&mut messenger)?;
                display_challenge_results(1, result, &mut messenger_rx)?;
            }
            2 => {
                let result = challenge.part_2_verified(&mut messenger)?;
                display_challenge_results(2, result, &mut messenger_rx)?;
            }
            _ => {
                return Err(color_eyre::eyre::eyre!(
                    "Invalid part number: {part} (must be 1 or 2)",
                ))
            }
        }
    } else {
        let result = challenge.part_1_verified(&mut messenger)?;
        display_challenge_results(1, result, &mut messenger_rx)?;

        let result = challenge.part_2_verified(&mut messenger)?;
        display_challenge_results(2, result, &mut messenger_rx)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    setup()?;
    let args = Args::parse();

    let data_config = DataConfiguration::new(&args.data_dir)?;

    let aoc = AdventOfCode::of_year_string(data_config, args.year.to_string())?;

    if let Some(command) = args.command {
        match command {
            Command::Run { day, part } => {
                if let Some(challenge) = aoc.challenge(day as usize) {
                    run_challenge(challenge, part)?;
                } else {
                    return Err(color_eyre::eyre::eyre!("Day {day} not available"));
                }
            }
        }
    } else {
        aoc.with_ui().run()?;
    }

    Ok(())
}

fn setup() -> Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    Ok(())
}
