use std::str::FromStr;

use color_eyre::{Report, Result};

use crate::{runner::Messenger, Challenge, Dataset};

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn score(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    pub fn win_against(&self) -> Self {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }

    pub fn lose_against(&self) -> Self {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }

    pub fn draw_against(&self) -> Self {
        *self
    }

    pub fn compare_with(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissors, Play::Scissors) => Outcome::Draw,
            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissors)
            | (Play::Scissors, Play::Rock) => Outcome::Win,
            (Play::Rock, Play::Scissors)
            | (Play::Paper, Play::Rock)
            | (Play::Scissors, Play::Paper) => Outcome::Lose,
        }
    }

    pub fn match_against(&self, other: &Self) -> u32 {
        other.compare_with(self).score() + self.score()
    }

    pub fn rig_match(&self, outcome: &Outcome) -> u32 {
        match outcome {
            Outcome::Win => self.win_against(),
            Outcome::Draw => self.draw_against(),
            Outcome::Lose => self.lose_against(),
        }
        .score()
            + outcome.score()
    }
}

impl FromStr for Play {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Invalid Play string: {s}")),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

impl FromStr for Outcome {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(color_eyre::eyre::eyre!("Invalid Outcome string: {s}")),
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
        "Rock Paper Scissors"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|s| {
                s.split_once(' ')
                    .ok_or_else(|| color_eyre::eyre::eyre!("Invalid data format"))
                    .and_then(|(other, me)| {
                        other
                            .parse::<Play>()
                            .and_then(|other| me.parse::<Play>().map(|me| (other, me)))
                            .map(|(other, me)| me.match_against(&other))
                    })
            })
            .sum::<Result<u32>>()?;

        Ok(result.to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|s| {
                s.split_once(' ')
                    .ok_or_else(|| color_eyre::eyre::eyre!("Invalid data format"))
                    .and_then(|(other, outcome)| {
                        other
                            .parse::<Play>()
                            .and_then(|other| {
                                outcome.parse::<Outcome>().map(|outcome| (other, outcome))
                            })
                            .map(|(other, outcome)| other.rig_match(&outcome))
                    })
            })
            .sum::<Result<u32>>()?;

        Ok(result.to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
