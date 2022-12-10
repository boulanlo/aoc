use std::str::FromStr;

use color_eyre::{Report, Result};
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let operand = split
            .next()
            .ok_or_else(|| color_eyre::eyre::eyre!("Missing opcode"))?;

        match operand {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let operand = split
                    .next()
                    .ok_or_else(|| color_eyre::eyre::eyre!("Missing opcode"))
                    .and_then(|s| s.parse::<i32>().map_err(Report::new))?;

                Ok(Instruction::Addx(operand))
            }
            _ => Err(color_eyre::eyre::eyre!("Unknown opcode {operand}")),
        }
    }
}

struct Cpu {
    program: Vec<Instruction>,
    register_x: i32,
}

impl Cpu {
    pub fn x_over_time(&self) -> impl Iterator<Item = i32> + '_ {
        let mut x = self.register_x;
        self.program.iter().flat_map(move |i| match i {
            Instruction::Noop => {
                vec![x]
            }
            Instruction::Addx(add) => {
                let res = vec![x, x];
                x += add;
                res
            }
        })
    }

    pub fn signal_strength(&self) -> i32 {
        const START: usize = 20;
        const STEP: usize = 40;

        self.x_over_time()
            .enumerate()
            .map(|(i, x)| (i + 1, x))
            .skip(START - 1)
            .step_by(STEP)
            .map(|(i, x)| (i as i32) * x)
            .sum::<i32>()
    }

    pub fn draw_crt(&self) -> String {
        self.x_over_time()
            .chunks(40)
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if (x - 1..=x + 1).contains(&(i as i32)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<S> FromIterator<S> for Cpu
where
    S: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let program = iter
            .into_iter()
            .map(|s| s.as_ref().parse::<Instruction>())
            .collect::<Result<Vec<_>>>()
            .unwrap();

        Self {
            program,
            register_x: 1,
        }
    }
}

pub struct Day10 {
    dataset: Dataset,
}

impl Day10 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day10 {
    fn name(&self) -> &'static str {
        "Cathode-Ray Tube"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let program = data.iter().collect::<Cpu>();

        Ok(program.signal_strength().to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let program = data.iter().collect::<Cpu>();

        Ok(program.draw_crt())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
