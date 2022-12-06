use color_eyre::Result;
use itertools::Itertools;

use crate::{runner::Messenger, Challenge, Dataset};

fn find_header<const SIZE: usize>(s: &str) -> Result<usize> {
    s.chars()
        .enumerate()
        .try_fold((Some(0), [None; SIZE]), |(next_empty, mut arr), (i, c)| {
            if let Some(next_empty) = next_empty {
                arr[next_empty] = Some(c);
                Ok((
                    if next_empty == SIZE - 1 {
                        None
                    } else {
                        Some(next_empty + 1)
                    },
                    arr,
                ))
            } else {
                arr.rotate_left(1);
                arr[SIZE - 1] = Some(c);

                if arr.iter().unique().count() == SIZE {
                    Err(i + 1)
                } else {
                    Ok((next_empty, arr))
                }
            }
        })
        .err()
        .ok_or_else(|| color_eyre::eyre::eyre!("Header not found!"))
}

pub struct Day6 {
    dataset: Dataset,
}

impl Day6 {
    pub fn new(dataset: Dataset) -> Self {
        Self { dataset }
    }
}

impl Challenge for Day6 {
    fn name(&self) -> &'static str {
        "Tuning Trouble"
    }

    fn part_1(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        const HEADER_LEN: usize = 4;

        if data.len() != 1 {
            Err(color_eyre::eyre::eyre!("Expected only one string"))
        } else {
            find_header::<HEADER_LEN>(&data[0]).map(|r| r.to_string())
        }
    }

    fn part_2(&self, data: &[String], _: &mut Messenger) -> Result<String> {
        const MESSAGE_HEADER_LEN: usize = 14;

        if data.len() != 1 {
            Err(color_eyre::eyre::eyre!("Expected only one string"))
        } else {
            find_header::<MESSAGE_HEADER_LEN>(&data[0]).map(|r| r.to_string())
        }
    }

    fn dataset(&self) -> &Dataset {
        &self.dataset
    }
}
