use std::path::PathBuf;

pub mod utils;
pub mod cli;
pub mod error;

fn get_program_path(binary_name: &str) -> Option<PathBuf> {
    match which::which(binary_name) {
        Ok(path) => Some(path),
        Err(_) => None
    }
}