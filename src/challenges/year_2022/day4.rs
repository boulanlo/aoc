use std::{collections::HashSet, ops::RangeBounds};

use color_eyre::Result;

use crate::{runner::Messenger, Challenge, Dataset};

pub struct Day4 {
    dataset: Dataset,
}

impl Day4 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day4 {
    fn name(&self) -> &'static str {
        "Camp Cleanup"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        fn to_range(s: &str) -> HashSet<usize> {
            let (start, end) = s.split_once('-').unwrap();
            (start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
                .into_iter()
                .collect()
        }

        let result = data
            .iter()
            .map(|s| {
                let (left, right) = s.split_once(',').unwrap();
                let (left, right) = (to_range(left), to_range(right));

                usize::from(left.is_superset(&right) || right.is_superset(&left))
            })
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        fn to_range(s: &str) -> HashSet<usize> {
            let (start, end) = s.split_once('-').unwrap();
            (start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap())
                .into_iter()
                .collect()
        }

        let result = data
            .iter()
            .map(|s| {
                let (left, right) = s.split_once(',').unwrap();
                let (left, right) = (to_range(left), to_range(right));

                usize::from(left.intersection(&right).next().is_some())
            })
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
