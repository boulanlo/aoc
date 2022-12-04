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

fn to_range(s: &str) -> Result<(usize, usize)> {
    let (start, end) = s
        .split_once('-')
        .ok_or_else(|| color_eyre::eyre::eyre!("Invalid range format"))?;

    Ok((start.parse::<usize>()?, end.parse::<usize>()?))
}

impl Challenge for Day4 {
    fn name(&self) -> &'static str {
        "Camp Cleanup"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|s| {
                let (left, right) = s
                    .split_once(',')
                    .ok_or_else(|| color_eyre::eyre::eyre!("Invalid pair format"))?;
                let ((l0, l1), (r0, r1)) = (to_range(left)?, to_range(right)?);

                Ok(usize::from(
                    (l0 <= r0 && l1 >= r1) || (r0 <= l0 && r1 >= l1),
                ))
            })
            .sum::<Result<usize>>()?;

        Ok(result.to_string())
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        let result = data
            .iter()
            .map(|s| {
                let (left, right) = s
                    .split_once(',')
                    .ok_or_else(|| color_eyre::eyre::eyre!("Invalid pair format"))?;
                let ((l0, l1), (r0, r1)) = (to_range(left)?, to_range(right)?);

                Ok(usize::from(l1 >= r0 && r1 >= l0))
            })
            .sum::<Result<usize>>()?;

        Ok(result.to_string())
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
