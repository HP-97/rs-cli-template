use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};


// Get the current executable's directory
//
// Returns an error if the directory cannot be retrieved
pub fn get_exe_parent_path() -> Result<PathBuf> {
    match env::current_exe() {
        Ok(mut exe_path) => {
            exe_path.pop();
            Ok(exe_path)
        }
        Err(e) => Err(anyhow!("failed to get current exe path: {e}")),
    }
}
