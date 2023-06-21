use std::path::PathBuf;

use crate::prelude::*;

pub mod prelude;
pub mod cli;
pub mod error;
pub mod utils;

fn get_program_path(binary_name: &str) -> Option<PathBuf> {
    match which::which(binary_name) {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}
