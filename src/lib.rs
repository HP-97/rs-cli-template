use std::path::PathBuf;

pub mod cli;
pub mod error;
pub mod prelude;
pub mod utils;

fn _get_program_path(binary_name: &str) -> Option<PathBuf> {
    match which::which(binary_name) {
        Ok(path) => Some(path),
        Err(_) => None,
    }
}
