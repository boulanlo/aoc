use aoc::{AdventOfCode, DataConfiguration, Year2022};
use clap::{Parser, Subcommand};
use color_eyre::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specific action to take
    #[command(subcommand)]
    command: Option<Command>,
    /// Download the available data for the given token
    #[arg(long, value_name = "AOC_TOKEN")]
    download_data: Option<String>,
    data_dir: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Run {
        year: u32,
        day: u8,
        part: Option<u8>,
    },
}

fn main() -> Result<()> {
    setup()?;
    let args = Args::parse();

    let data_config = DataConfiguration::new("./data")?;
    let aoc = AdventOfCode::of_year::<Year2022>(data_config)?.with_ui();

    aoc.run()?;

    Ok(())
}

fn setup() -> Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    Ok(())
}
