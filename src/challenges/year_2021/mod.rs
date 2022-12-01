use std::sync::Arc;

use color_eyre::Result;

use crate::DataConfiguration;

use super::Year;

mod day1;
mod day2;

pub struct Year2021;

impl Year for Year2021 {
    fn challenges(
        data_config: DataConfiguration,
    ) -> Result<[Option<Arc<dyn crate::Challenge + Send + Sync>>; 25]> {
        Ok([
            Some(Arc::new(day1::Day1::new(
                data_config.get_dataset("2021", "1")?,
            ))),
            Some(Arc::new(day2::Day2::new(
                data_config.get_dataset("2021", "2")?,
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
            None,
            None,
            None,
            None,
            None,
        ])
    }

    fn name() -> &'static str {
        "Advent of Code 2021"
    }
}
