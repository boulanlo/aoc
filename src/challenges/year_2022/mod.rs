use std::sync::Arc;

use color_eyre::Result;

use crate::DataConfiguration;

use super::Year;

pub struct Year2022;

mod day1;

impl Year for Year2022 {
    fn challenges(
        data_config: DataConfiguration,
    ) -> Result<[Option<Arc<dyn crate::Challenge + Send + Sync>>; 25]> {
        Ok([
            Some(Arc::new(day1::Day1::new(
                data_config.get_dataset("2022", "1")?,
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
            None,
        ])
    }

    fn name() -> &'static str {
        "Advent of Code 2022"
    }
}
