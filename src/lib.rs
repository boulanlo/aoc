use color_eyre::Result as EResult;
use runner::{Selection, Task};
use tui::backend::Backend;

use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

mod challenges;
pub use challenges::year_2022::Year2022;
use challenges::Year;

mod ui;
use ui::UI;

mod runner;
pub use runner::{messenger, Messenger, MessengerReceiver};

pub trait Challenge {
    fn name(&self) -> &'static str;
    fn part_1(&self, data: &[String], messenger: &mut Messenger) -> EResult<String>;
    fn part_2(&self, data: &[String], messenger: &mut Messenger) -> EResult<String>;
    fn dataset(&self) -> &Dataset;

    fn part_1_verified(&self, messenger: &mut Messenger) -> EResult<String> {
        let dataset = self.dataset();
        self.part_1(&dataset.example_data, messenger)
            .and_then(|example_result| {
                if example_result == *dataset.example_results[0].as_ref().unwrap() {
                    self.part_1(dataset.real_data.as_deref().unwrap(), messenger).and_then(|real_result| {
                        let result = format!("{real_result} (example: {example_result})");
                        if let Some(real_expected_result) = dataset.real_results[0].as_ref() {
                            if *real_expected_result == real_result {
                                Ok(result)
                            } else {
                                Err(color_eyre::eyre::eyre!(
                                    "Part 1: Real expected result (\"{real_expected_result}\") different from computed one (\"{real_result}\")",
                                ))
                            }
                        } else {
                            Ok(result)
                        }
                    })
                } else {
                    Err(color_eyre::eyre::eyre!(
                        "Part 1: Example result (\"{}\") different from computed one (\"{example_result}\")",
                        dataset.example_results[0].as_ref().unwrap()
                    ))
                }
            })
    }

    fn part_2_verified(&self, messenger: &mut Messenger) -> EResult<String> {
        let dataset = self.dataset();
        self.part_2(&dataset.example_data, messenger)
            .and_then(|example_result| {
                if example_result == *dataset.example_results[1].as_ref().unwrap() {
                    self.part_2(dataset.real_data.as_deref().unwrap(), messenger).and_then(|real_result| {
                        let result = format!("{real_result} (example: {example_result})");
                        if let Some(real_expected_result) = dataset.real_results[1].as_ref() {
                            if *real_expected_result == real_result {
                                Ok(result)
                            } else {
                                Err(color_eyre::eyre::eyre!(
                                    "Part 2: Real expected result (\"{real_expected_result}\") different from computed one (\"{real_result}\")",
                                ))
                            }
                        } else {
                            Ok(result)
                        }
                    })
                } else {
                    Err(color_eyre::eyre::eyre!(
                        "Part 2: Example result (\"{}\") different from computed one (\"{example_result}\")",
                        dataset.example_results[1].as_ref().unwrap()
                    ))
                }
            })
    }
}

#[derive(Debug, Clone)]
pub struct DataConfiguration {
    root_dir: PathBuf,
}

impl DataConfiguration {
    pub fn new<P>(root_dir: P) -> EResult<Self>
    where
        P: AsRef<Path>,
    {
        let root_dir = root_dir.as_ref();
        if root_dir.exists() && root_dir.is_dir() {
            Ok(Self {
                root_dir: root_dir.to_owned(),
            })
        } else {
            Err(color_eyre::eyre::eyre!(
                "The `root_dir` parameter \"{}\" must exist and be a directory",
                root_dir.to_string_lossy()
            ))
        }
    }

    pub fn get_dataset<T, U>(&self, year: T, day: U, multiline_result: bool) -> EResult<Dataset>
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        fn read_file<P: AsRef<Path> + Clone>(path: P) -> EResult<Vec<String>> {
            let path = path.as_ref();
            if path.exists() {
                File::open(path).and_then(|file| {
                    BufReader::new(file)
                        .lines()
                        .collect::<Result<Vec<String>, _>>()
                })
            } else {
                std::fs::create_dir_all(path.parent().unwrap())?;
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path)
                    .and_then(|mut file| {
                        file.write_all(b"TODO: fill this file!")?;
                        file.flush()?;
                        file.seek(std::io::SeekFrom::Start(0))?;
                        Ok(file)
                    })
                    .and_then(|file| {
                        BufReader::new(file)
                            .lines()
                            .collect::<Result<Vec<String>, _>>()
                    })
            }
            .map_err(|e| {
                color_eyre::Report::new(e).wrap_err(format!(
                    "Could not load dataset file {}",
                    path.to_string_lossy()
                ))
            })
        }

        fn read_file_optional<P: AsRef<Path> + Clone>(path: P) -> EResult<Option<Vec<String>>> {
            if path.as_ref().exists() {
                read_file(path).map(Some)
            } else {
                Ok(None)
            }
        }

        let mut day_directory = self.root_dir.clone();
        day_directory.push(year.as_ref());
        day_directory.push(day.as_ref());

        Ok(Dataset {
            example_data: read_file({
                let mut p = day_directory.clone();
                p.push("example_data.txt");
                p
            })?,
            example_results: [
                Some(read_file({
                    let mut p = day_directory.clone();
                    p.push("example_expected_1.txt");
                    p
                })?)
                .map(|mut f| {
                    if multiline_result {
                        f.join("\n")
                    } else {
                        f.pop().unwrap()
                    }
                }),
                read_file_optional({
                    let mut p = day_directory.clone();
                    p.push("example_expected_2.txt");
                    p
                })?
                .map(|mut f| f.pop().unwrap()),
            ],
            real_data: Some(read_file({
                let mut p = day_directory.clone();
                p.push("real_data.txt");
                p
            })?),
            real_results: [
                read_file_optional({
                    let mut p = day_directory.clone();
                    p.push("real_results_1.txt");
                    p
                })?
                .map(|mut v| {
                    if multiline_result {
                        v.join("\n")
                    } else {
                        v.pop().unwrap()
                    }
                }),
                read_file_optional({
                    let mut p = day_directory.clone();
                    p.push("real_results_2.txt");
                    p
                })?
                .map(|mut v| {
                    if multiline_result {
                        v.join("\n")
                    } else {
                        v.pop().unwrap()
                    }
                }),
            ],
        })
    }
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub example_data: Vec<String>,
    pub example_results: [Option<String>; 2],
    pub real_data: Option<Vec<String>>,
    pub real_results: [Option<String>; 2],
}

pub struct AdventOfCode {
    pub(crate) challenges: [Option<Arc<dyn Challenge + Send + Sync>>; 25],
    pub(crate) name: &'static str,
}

impl AdventOfCode {
    pub fn all(data_config: DataConfiguration) -> EResult<HashMap<String, Self>> {
        Ok([
            (
                String::from("2021"),
                Self::of_year::<challenges::year_2021::Year2021>(data_config.clone())?,
            ),
            (
                String::from("2022"),
                Self::of_year::<challenges::year_2022::Year2022>(data_config)?,
            ),
        ]
        .into_iter()
        .collect())
    }

    pub fn of_year_string<S>(data_config: DataConfiguration, year: S) -> EResult<Self>
    where
        S: AsRef<str>,
    {
        Self::all(data_config)?
            .remove(year.as_ref())
            .ok_or_else(|| color_eyre::eyre::eyre!("Unsupported AoC year: {}", year.as_ref()))
    }

    pub fn of_year<Y>(data_config: DataConfiguration) -> EResult<Self>
    where
        Y: Year,
    {
        Ok(Self {
            challenges: Y::challenges(data_config)?,
            name: Y::name(),
        })
    }

    pub fn with_ui<B: Backend>(self) -> UI<B> {
        UI::new(self)
    }

    pub fn available_challenges(&self) -> Vec<usize> {
        self.challenges
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.as_ref().map(|_| i + 1))
            .collect()
    }

    pub fn challenge(&self, day: usize) -> Option<Arc<dyn Challenge + Send + Sync>> {
        self.challenges[day - 1].clone()
    }

    pub fn task_for(&self, selection: Selection) -> Option<Task> {
        self.challenges[selection.day - 1].as_ref().and_then(|c| {
            let dataset = c.dataset();
            if selection.part == 2 && dataset.example_results[1].is_none() {
                None
            } else {
                Some(Task {
                    selection,
                    challenge: c.clone(),
                })
            }
        })
    }
}
