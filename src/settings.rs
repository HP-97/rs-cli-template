use serde::Deserialize;

use config::Config;

// Required to use shortened 'Result'
use crate::{error::AppError, prelude::Result};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    /// Determines output logging level
    pub log_level: usize,
    /// If true, do not do any actual work
    pub dry_run: bool,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let s = Config::builder()
            .set_default("log_level", 3)?
            .set_default("dry_run", false)?;

        match s.build()?.try_deserialize::<Settings>() {
            Ok(v) => Ok(v),
            Err(e) => return Err(AppError::Config(e)),
        }
    }
}
