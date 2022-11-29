use aoc::{AdventOfCode, DataConfiguration, Year2021};
use color_eyre::Result;

fn main() -> Result<()> {
    setup()?;

    let data_config = DataConfiguration::new("./data")?;
    let aoc = AdventOfCode::of_year::<Year2021>(data_config)?.with_ui();

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
