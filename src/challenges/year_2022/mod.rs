use std::sync::Arc;

use color_eyre::Result;

use crate::DataConfiguration;

use super::Year;

pub struct Year2022;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

impl Year for Year2022 {
    fn challenges(
        data_config: DataConfiguration,
    ) -> Result<[Option<Arc<dyn crate::Challenge + Send + Sync>>; 25]> {
        Ok([
            Some(Arc::new(day1::Day1::new(
                data_config.get_dataset("2022", "1", false)?,
            ))),
            Some(Arc::new(day2::Day2::new(
                data_config.get_dataset("2022", "2", false)?,
            ))),
            Some(Arc::new(day3::Day3::new(
                data_config.get_dataset("2022", "3", false)?,
            ))),
            Some(Arc::new(day4::Day4::new(
                data_config.get_dataset("2022", "4", false)?,
            ))),
            Some(Arc::new(day5::Day5::new(
                data_config.get_dataset("2022", "5", false)?,
            ))),
            Some(Arc::new(day6::Day6::new(
                data_config.get_dataset("2022", "6", false)?,
            ))),
            Some(Arc::new(day7::Day7::new(
                data_config.get_dataset("2022", "7", false)?,
            ))),
            Some(Arc::new(day8::Day8::new(
                data_config.get_dataset("2022", "8", false)?,
            ))),
            Some(Arc::new(day9::Day9::new(
                data_config.get_dataset("2022", "9", false)?,
            ))),
            Some(Arc::new(day10::Day10::new(
                data_config.get_dataset("2022", "10", true)?,
            ))),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ])
    }

    fn name() -> &'static str {
        "Advent of Code 2022"
    }
}
