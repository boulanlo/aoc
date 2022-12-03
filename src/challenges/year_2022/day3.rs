use std::collections::HashSet;

use color_eyre::Result;
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

pub struct Day3 {
    dataset: Dataset,
}

impl Day3 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

fn value(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => ((c as u32) - 64) + 26,
        _ => unreachable!(),
    }
}

impl Challenge for Day3 {
    fn name(&self) -> &'static str {
        "Rucksack Reorganization"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|s| {
                let len = s.chars().count();
                let (left, right) = s.split_at(len / 2);
                let c = *left
                    .chars()
                    .collect::<HashSet<char>>()
                    .intersection(&right.chars().collect())
                    .next()
                    .unwrap();
                value(c)
            })
            .sum::<u32>();

        Ok(format!("{result}"))
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .tuples()
            .map(|(first, second, third)| {
                let common = (first.chars().collect::<HashSet<char>>())
                    .intersection(&second.chars().collect())
                    .copied()
                    .collect::<HashSet<char>>();
                let common = *common
                    .intersection(&third.chars().collect())
                    .next()
                    .unwrap();

                value(common)
            })
            .sum::<u32>();

        Ok(format!("{result}"))
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
