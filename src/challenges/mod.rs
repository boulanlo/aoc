use std::sync::Arc;

use color_eyre::Result;

use crate::{Challenge, DataConfiguration};

pub mod year_2021;
pub mod year_2022;

pub trait Year {
    fn challenges(
        data_config: DataConfiguration,
    ) -> Result<[Option<Arc<dyn Challenge + Send + Sync>>; 25]>;
    fn name() -> &'static str;
}
