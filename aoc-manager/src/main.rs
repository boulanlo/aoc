use std::{
    collections::HashSet,
    fmt,
    fs::ReadDir,
    path::{Path, PathBuf},
    process::Child,
    str::FromStr,
};

use chrono::{DateTime, Datelike, Local, ParseError};
use clap::{Parser, Subcommand};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use thiserror::Error;
use toml::{Table, Value};

#[derive(Debug, Error)]
enum RuntimeError {
    #[error("chrono parse error")]
    DateParse(#[from] ParseError),
    #[error("Integer parse error")]
    IntegerParse(#[from] std::num::ParseIntError),
    #[error("day value outside of valid range (1..=25)")]
    DayOutsideRange,
    #[error("current date outside of AoC day range and no day specified (use the -d flag!)")]
    NoDay,
    #[error("i/o error")]
    Io(#[from] std::io::Error),
    #[error("toml parse error")]
    TomlParse(#[from] toml::de::Error),
    #[error("the Cargo.toml file in the current directory is missing the [workspace] table (are you in the root directory?)")]
    MissingWorkspace,
    #[error("day {} is already present in the workspace", .0)]
    DayAlreadyPresent(Challenge),
    #[error("network error")]
    NetworkError(#[from] Box<ureq::Error>),
    #[error(
        "AoC session key not found in AOC_SESSION environment variable or in .aoc-token file."
    )]
    MissingSessionKey,
    #[error("notify error")]
    WatchError(#[from] notify::Error),
    #[error("the year '{}' is earlier than 2015, the earliest Advent of Code event.", .0)]
    YearOutsideRange(i32),
    #[error("year specified without day. specify either both, neither or just the day.")]
    YearWithoutDay,
    #[error("\"{}\" is not a valid AoC crate name.", .0)]
    InvalidCrateFormat(String),
}

/// Returns the current day of the challenge. This requires the current date to
/// be between December 1st and December 26th of the current year, exclusive.
fn get_current_aoc_day() -> Result<Option<Day>, RuntimeError> {
    let now: DateTime<Local> = Local::now();
    let year = now.year();
    // 1st December at midnight
    let start = DateTime::parse_from_rfc3339(&format!("{year}-12-01T00:00:00+00:00"))?
        .with_timezone(&Local);
    // 26th December at midnight
    let end = DateTime::parse_from_rfc3339(&format!("{year}-12-26T00:00:00+00:00"))?
        .with_timezone(&Local);

    if (start..end).contains(&now) {
        Ok(Some(Day::new(now.day())?))
    } else {
        Ok(None)
    }
}

fn copy_dir_recursively<S, D>(src: S, dst: D) -> Result<(), RuntimeError>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    fn rec(dir: ReadDir, root: PathBuf, dst: PathBuf) -> Result<(), RuntimeError> {
        for entry in dir {
            let entry = entry?;
            let meta = entry.metadata()?;
            let path = entry.path();
            let relative_path = path
                .strip_prefix(root.clone())
                .expect("we shouldn't end up with a different prefix here");

            if meta.is_file() {
                std::fs::copy(&path, dst.join(relative_path))?;
            } else if meta.is_dir() {
                std::fs::create_dir(dst.join(relative_path))?;
                rec(std::fs::read_dir(path)?, root.clone(), dst.clone())?;
            }
        }

        Ok(())
    }

    std::fs::create_dir(&dst)?;
    rec(
        std::fs::read_dir(&src)?,
        src.as_ref().into(),
        dst.as_ref().into(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct Day(u32);

impl Day {
    fn new(value: u32) -> Result<Self, RuntimeError> {
        if (1..26).contains(&value) {
            Ok(Self(value))
        } else {
            Err(RuntimeError::DayOutsideRange)
        }
    }
}

impl FromStr for Day {
    type Err = RuntimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u32 = s.parse()?;

        Self::new(value)
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct Year(i32);

impl Year {
    fn new(value: i32) -> Result<Self, RuntimeError> {
        if (2015..).contains(&value) {
            Ok(Self(value))
        } else {
            Err(RuntimeError::YearOutsideRange(value))
        }
    }

    fn current() -> Self {
        Self(Local::now().year())
    }
}

impl FromStr for Year {
    type Err = RuntimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: i32 = s.parse()?;

        Self::new(value)
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Challenge {
    day: Day,
    year: Year,
}

impl Challenge {
    fn new(day: Day, year: Year) -> Self {
        Self { day, year }
    }

    fn crate_name(&self) -> String {
        format!("day_{}_{}", self.year, self.day)
    }

    fn input_name(&self) -> String {
        format!("{}_{}.txt", self.year, self.day)
    }
}

impl FromStr for Challenge {
    type Err = RuntimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split('_');
        if tokens.next() != Some("day") {
            return Err(RuntimeError::InvalidCrateFormat(s.to_string()));
        }

        let year = tokens
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .map(|v| Year(v))
            .ok_or_else(|| RuntimeError::InvalidCrateFormat(s.to_string()))?;

        let day = tokens
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .map(|v| Day(v))
            .ok_or_else(|| RuntimeError::InvalidCrateFormat(s.to_string()))?;

        if tokens.next().is_some() {
            return Err(RuntimeError::InvalidCrateFormat(s.to_string()));
        }

        Ok(Challenge { day, year })
    }
}

impl fmt::Display for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.day, self.year)
    }
}

#[derive(Debug)]
#[repr(transparent)]
struct Workspace(Table);

impl Workspace {
    fn from_current_directory() -> Result<Self, RuntimeError> {
        let path = "./Cargo.toml";
        let table = std::fs::read_to_string(path)?.parse::<Table>()?;

        if table.get("workspace").is_some() {
            Ok(Self(table))
        } else {
            Err(RuntimeError::MissingWorkspace)
        }
    }

    fn members_mut(&mut self) -> &mut Vec<Value> {
        let workspace = self
            .0
            .get_mut("workspace")
            .expect("a constructed Workspace struct should always have a [workspace] table")
            .as_table_mut()
            .expect("the `workspace` value should always be a table");

        workspace
            .get_mut("members")
            .expect("the `members` value is missing from [workspace]")
            .as_array_mut()
            .expect("the `members` value should be an array")
    }

    fn members(&self) -> &[Value] {
        let workspace = self
            .0
            .get("workspace")
            .expect("a constructed Workspace struct should always have a [workspace] table")
            .as_table()
            .expect("the `workspace` value should always be a table");

        workspace
            .get("members")
            .expect("the `members` value is missing from [workspace]")
            .as_array()
            .expect("the `members` value should be an array")
    }

    fn get_days(&self) -> Vec<Challenge> {
        self.members()
            .iter()
            .filter_map(|v| v.as_str().and_then(|s| s.parse().ok()))
            .collect()
    }

    fn add_day(&mut self, challenge: Challenge) -> Result<(), RuntimeError> {
        let members = self.members_mut();

        let value = Value::String(challenge.crate_name());
        if members.contains(&value) {
            Err(RuntimeError::DayAlreadyPresent(challenge))
        } else {
            members.push(Value::String(challenge.crate_name()));
            Ok(())
        }
    }

    fn write(self) -> Result<(), RuntimeError> {
        std::fs::write("./Cargo.toml", self.0.to_string())?;
        Ok(())
    }
}

#[derive(Debug)]
struct DayCrate {
    table: Table,
    challenge: Challenge,
}

impl DayCrate {
    pub fn new(challenge: Challenge) -> Result<Self, RuntimeError> {
        let crate_name = challenge.crate_name();
        let table =
            std::fs::read_to_string(format!("{crate_name}/Cargo.toml"))?.parse::<Table>()?;

        Ok(Self { table, challenge })
    }

    pub fn set_name(mut self) -> Result<(), RuntimeError> {
        let package = self
            .table
            .get_mut("package")
            .expect("a constructed DayCrate struct should always have a [package] table")
            .as_table_mut()
            .expect("the `package` value should always be a table");

        *package
            .get_mut("name")
            .expect("`name` attribute missing from [package] table") =
            Value::String(self.challenge.crate_name());

        std::fs::write(
            format!("{}/Cargo.toml", self.challenge.crate_name()),
            self.table.to_string(),
        )?;

        Ok(())
    }
}

struct InputCache;

impl InputCache {
    const CACHE_PATH: &str = ".input-cache";
    const SESSION_FILE_PATH: &str = ".aoc-token";

    fn get_session() -> Result<String, RuntimeError> {
        if let Ok(x) = std::env::var("AOC_SESSION") {
            Ok(x)
        } else if let Ok(x) = std::fs::read_to_string(Self::SESSION_FILE_PATH) {
            Ok(x)
        } else {
            Err(RuntimeError::MissingSessionKey)
        }
    }

    fn get_present() -> Result<Vec<Challenge>, RuntimeError> {
        std::fs::read_dir(Self::CACHE_PATH)?
            .map(|entry| {
                let entry = entry?;
                let meta = entry.metadata()?;

                Ok(meta.is_file().then(|| entry.path()).and_then(|path| {
                    path.file_name()
                        .and_then(|s| s.to_string_lossy().to_string().parse().ok())
                }))
            })
            .filter_map(|r| match r {
                Err(e) => Some(Err(e)),
                Ok(None) => None,
                Ok(Some(v)) => Some(Ok(v)),
            })
            .collect()
    }

    fn fetch(challenge: &Challenge) -> Result<String, RuntimeError> {
        std::fs::create_dir_all(Self::CACHE_PATH)?;

        if let Some(res) = std::fs::read_dir(Self::CACHE_PATH)?.find_map(|entry| {
            match entry.and_then(|entry| {
                entry.metadata().map(|meta| {
                    if meta.is_file()
                        && entry
                            .path()
                            .file_name()
                            .map(|s| s.to_string_lossy().to_string())
                            == Some(challenge.input_name())
                    {
                        Some(std::fs::read_to_string(entry.path()))
                    } else {
                        None
                    }
                })
            }) {
                Ok(Some(r)) => Some(r),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            }
        }) {
            res.map_err(Into::into)
        } else {
            let input = ureq::get(&format!(
                "https://adventofcode.com/{}/day/{}/input",
                challenge.year, challenge.day.0,
            ))
            .set("Cookie", &Self::get_session()?)
            .call()
            .map_err(Box::new)?
            .into_string()?;

            std::fs::write(
                format!("{}/{}", Self::CACHE_PATH, challenge.input_name()),
                &input,
            )?;

            Ok(input)
        }
    }
}

fn spawn_compiler(challenge: Challenge) -> Result<Child, RuntimeError> {
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(challenge.crate_name())
        .spawn()
        .map_err(Into::into)
}

fn watch(challenge: Challenge) -> Result<(), RuntimeError> {
    let path = challenge.crate_name();
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(&PathBuf::from(&path), RecursiveMode::Recursive)?;

    let mut child: Option<Child> = None;

    for res in rx {
        match res {
            Ok(_) => match child.as_mut().map(|x| x.try_wait()) {
                Some(Ok(None)) => {}
                Some(Err(e)) => return Err(e.into()),
                Some(Ok(Some(_))) | None => child = Some(spawn_compiler(challenge)?),
            },
            Err(error) => return Err(error.into()),
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Add a new day to the repository.
    Add {
        /// Specify the specific day to add. When omitted, it uses the current
        /// day if and only if the current date is during AoC 2023 (i.e. between
        /// 2023-12-01T00:00:00 and 2023-12-26T00:00:00 exclusive). Accepted values
        /// are integers in the range 1..26.
        #[arg(short, long)]
        day: Option<Day>,
        /// Specify the specific year considered for the challenge. When omitted,
        /// it uses the current year.
        #[arg(short, long)]
        year: Option<Year>,
        /// Only create the directory and don't watch for changes afterwards.
        #[arg(long)]
        no_watch: bool,
    },
    /// Watch the crate for a day and run it when the code changes.
    Watch {
        /// Specify the specific day to add. When omitted, it uses the current
        /// day if and only if the current date is during AoC 2023 (i.e. between
        /// 2023-12-01T00:00:00 and 2023-12-26T00:00:00 exclusive). Accepted values
        /// are integers in the range 1..26.
        #[arg(short, long)]
        day: Option<Day>,
        /// Specify the specific year considered for the challenge. When omitted,
        /// it uses the current year.
        #[arg(short, long)]
        year: Option<Year>,
    },
    /// Fetches input for all crates missing it
    Fetch,
}

fn main() -> Result<(), RuntimeError> {
    let args = Args::parse();

    match args.command {
        Command::Add {
            day,
            year,
            no_watch,
        } => {
            if year.is_some() && day.is_none() {
                Err(RuntimeError::YearWithoutDay)
            } else {
                let year = year.unwrap_or_else(Year::current);

                match Ok(day).or_else(|_: ()| get_current_aoc_day()) {
                    Ok(Some(day)) => {
                        // Add the new crate to the workspace
                        let mut workspace = Workspace::from_current_directory()?;
                        let challenge = Challenge::new(day, year);

                        workspace.add_day(challenge)?;

                        // Copy template crate to the new crate
                        copy_dir_recursively("template", challenge.crate_name())?;

                        // Set name of new crate
                        DayCrate::new(challenge)?.set_name()?;

                        // Add input to new crate
                        std::fs::write(
                            format!("{}/src/input.txt", challenge.crate_name()),
                            InputCache::fetch(&challenge)?,
                        )?;

                        // Commit workspace changes
                        workspace.write()?;

                        if !no_watch {
                            watch(challenge)
                        } else {
                            Ok(())
                        }
                    }
                    Ok(None) => Err(RuntimeError::NoDay),
                    Err(e) => Err(e),
                }
            }
        }
        Command::Watch { day, year } => {
            if year.is_some() && day.is_none() {
                Err(RuntimeError::YearWithoutDay)
            } else {
                let year = year.unwrap_or_else(Year::current);

                match Ok(day).or_else(|_: ()| get_current_aoc_day()) {
                    Ok(Some(day)) => watch(Challenge::new(day, year)),
                    Ok(None) => Err(RuntimeError::NoDay),
                    Err(e) => Err(e),
                }
            }
        }
        Command::Fetch => {
            let crates = Workspace::from_current_directory()?
                .get_days()
                .into_iter()
                .collect::<HashSet<_>>();

            let inputs = InputCache::get_present()?
                .into_iter()
                .collect::<HashSet<_>>();

            for challenge in crates.difference(&inputs) {
                std::fs::write(
                    format!("{}/src/input.txt", challenge.crate_name()),
                    InputCache::fetch(challenge)?,
                )?;
            }

            Ok(())
        }
    }
}
