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
        "Calorie Counting"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|line| {
                if line.trim().is_empty() {
                    None
                } else {
                    Some(line.parse::<u32>())
                }
            })
            .try_fold(Vec::new(), |mut acc, maybe_cals| -> Result<Vec<u32>> {
                if acc.is_empty() {
                    acc.push(0u32)
                }
                if let Some(cals) = maybe_cals {
                    let cals = cals?;
                    if let Some(x) = acc.last_mut() {
                        *x += cals
                    };
                } else {
                    acc.push(0u32);
                }
                Ok(acc)
            })?
            .into_iter()
            .max()
            .unwrap();

        Ok(format!("{result}"))
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let mut vec = data
            .iter()
            .map(|line| {
                if line.trim().is_empty() {
                    None
                } else {
                    Some(line.parse::<u32>())
                }
            })
            .try_fold(Vec::new(), |mut acc, maybe_cals| -> Result<Vec<u32>> {
                if acc.is_empty() {
                    acc.push(0u32)
                }
                if let Some(cals) = maybe_cals {
                    let cals = cals?;
                    if let Some(x) = acc.last_mut() {
                        *x += cals
                    };
                } else {
                    acc.push(0u32);
                }
                Ok(acc)
            })?;
        vec.sort();
        let result: (u32, u32, u32) = vec.into_iter().tuple_windows().last().unwrap();

        Ok(format!("{}", result.0 + result.1 + result.2))
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
