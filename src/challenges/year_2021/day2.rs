use std::str::FromStr;

use color_eyre::{Report, Result};

use crate::{runner::Messenger, Challenge, Dataset};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, amount) = s
            .split_once(' ')
            .ok_or_else(|| color_eyre::eyre::eyre!("Invalid format for command"))?;

        match command {
            "forward" => Ok(Command::Forward(
                amount.parse().map_err(color_eyre::Report::new)?,
            )),
            "down" => Ok(Command::Down(
                amount.parse().map_err(color_eyre::Report::new)?,
            )),
            "up" => Ok(Command::Up(
                amount.parse().map_err(color_eyre::Report::new)?,
            )),
            _ => Err(color_eyre::eyre::eyre!("Unknown command: {command}")),
        }
    }
}

pub struct Day2 {
    dataset: Dataset,
}

impl Day2 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day2 {
    fn name(&self) -> &'static str {
        "Dive!"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let (horizontal_position, depth) = data.iter().map(|s| Command::from_str(s)).try_fold(
            (0, 0),
            |(horizontal_position, depth), command| -> Result<(i32, i32)> {
                match command? {
                    Command::Forward(amount) => Ok((horizontal_position + amount, depth)),
                    Command::Down(amount) => Ok((horizontal_position, depth + amount)),
                    Command::Up(amount) => Ok((horizontal_position, depth - amount)),
                }
            },
        )?;

        Ok(format!("{}", horizontal_position * depth))
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        todo!()
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
