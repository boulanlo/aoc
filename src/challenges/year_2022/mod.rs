use std::sync::Arc;

use color_eyre::Result;

use crate::DataConfiguration;

use super::Year;

pub struct Year2022;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

impl Year for Year2022 {
    fn challenges(
        data_config: DataConfiguration,
    ) -> Result<[Option<Arc<dyn crate::Challenge + Send + Sync>>; 25]> {
        Ok([
            Some(Arc::new(day1::Day1::new(
                data_config.get_dataset("2022", "1")?,
            ))),
            Some(Arc::new(day2::Day2::new(
                data_config.get_dataset("2022", "2")?,
            ))),
            Some(Arc::new(day3::Day3::new(
                data_config.get_dataset("2022", "3")?,
            ))),
            Some(Arc::new(day4::Day4::new(
                data_config.get_dataset("2022", "4")?,
            ))),
            Some(Arc::new(day5::Day5::new(
                data_config.get_dataset("2022", "5")?,
            ))),
            Some(Arc::new(day6::Day6::new(
                data_config.get_dataset("2022", "6")?,
            ))),
            Some(Arc::new(day7::Day7::new(
                data_config.get_dataset("2022", "7")?,
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
            None,
            None,
            None,
        ])
    }

    fn name() -> &'static str {
        "Advent of Code 2022"
    }
}
