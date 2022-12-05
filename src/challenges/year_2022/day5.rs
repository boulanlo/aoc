use color_eyre::Result;
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

pub struct Day5 {
    dataset: Dataset,
}

#[derive(Debug)]
struct Cargo {
    stacks: Vec<Vec<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

impl Cargo {
    pub fn execute(&mut self) {
        for (amount, from, to) in self.instructions.drain(..) {
            for _ in 0..amount {
                let c = self.stacks[from - 1].pop().unwrap();
                self.stacks[to - 1].push(c);
            }
        }
    }

    pub fn execute_move_multiple(&mut self) {
        for (amount, from, to) in self.instructions.drain(..) {
            let len = self.stacks[from - 1].len();
            let crates = self.stacks[from - 1]
                .drain(len - amount..)
                .collect::<Vec<_>>();
            self.stacks[to - 1].extend(crates);
        }
    }

    pub fn top(&self) -> Vec<char> {
        self.stacks
            .iter()
            .map(|v| v.last().copied().unwrap())
            .collect()
    }
}

impl FromIterator<String> for Cargo {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut stacks = Vec::new();
        let mut instructions = Vec::new();
        let mut finished_stacks = false;

        for line in iter.into_iter() {
            if line.trim().is_empty() {
                finished_stacks = true
            } else if finished_stacks {
                let instruction = {
                    let mut s = line
                        .split(' ')
                        .filter(|s| !["move", "from", "to"].contains(s))
                        .map(|s| s.parse().unwrap());
                    (s.next().unwrap(), s.next().unwrap(), s.next().unwrap())
                };
                instructions.push(instruction);
            } else {
                if stacks.is_empty() {
                    stacks.extend(
                        std::iter::repeat_with(Vec::new).take((line.chars().count() / 4) + 1),
                    )
                }
                for (i, maybe_crate) in line.chars().chunks(4).into_iter().enumerate() {
                    let maybe_crate = maybe_crate.collect::<Vec<char>>();
                    if maybe_crate[1].is_ascii_alphabetic() {
                        stacks[i].insert(0, maybe_crate[1]);
                    }
                }
            }
        }

        Self {
            stacks,
            instructions,
        }
    }
}

impl Day5 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day5 {
    fn name(&self) -> &'static str {
        "Supply Stacks"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let mut cargo = data.iter().cloned().collect::<Cargo>();
        cargo.execute();
        Ok(cargo.top().into_iter().join(""))
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let mut cargo = data.iter().cloned().collect::<Cargo>();
        cargo.execute_move_multiple();
        Ok(cargo.top().into_iter().join(""))
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
