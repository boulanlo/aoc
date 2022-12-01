use color_eyre::Result;
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

pub struct Day1 {
    dataset: Dataset,
}

impl Day1 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day1 {
    fn name(&self) -> &'static str {
        "Sonar Sweep"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .tuple_windows()
            .fold(0u32, |result, (previous, current)| {
                if previous < current {
                    result + 1
                } else {
                    result
                }
            });

        Ok(format!("{result}"))
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .fold(0u32, |result, (previous, current)| {
                if previous < current {
                    result + 1
                } else {
                    result
                }
            });

        Ok(format!("{result}"))
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
